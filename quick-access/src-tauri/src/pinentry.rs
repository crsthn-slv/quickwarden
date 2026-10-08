//! Pinentry do rbw com Touch ID.
//!
//! O rbw-agent pede a palavra-passe mestra a um programa pinentry (protocolo Assuan, ver
//! `src/pinentry.rs` do rbw). Este modo guarda-a no Keychain e responde ao `GETPIN` depois do
//! Touch ID. Sem inscrição, Touch ID cancelado, ou qualquer pedido que não seja a palavra-passe
//! mestra → passa tudo ao `pinentry-mac`.
//!
//! Limite conhecido: sem Developer ID não há item com `biometryCurrentSet` (o Keychain
//! protegido devolve -34018), por isso o item vive no Keychain de sessão, com a ACL presa à
//! assinatura deste binário, e quem exige o Touch ID é o `LAContext` aqui — não o Keychain.

use block2::RcBlock;
use objc2::runtime::Bool;
use objc2_foundation::{NSError, NSString};
use objc2_local_authentication::{LAContext, LAPolicy};
use security_framework::passwords::{get_generic_password, set_generic_password};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

const SERVICE: &str = "local.quick-access";
const ACCOUNT: &str = "rbw-master-password";
const FALLBACK: &str = "/opt/homebrew/bin/pinentry-mac";

/// Pede a palavra-passe ao pinentry-mac (duas vezes, `SETREPEAT`) e guarda-a. Não precisa de
/// terminal. Não a valida: se estiver errada, o rbw manda `SETERROR` e cai no pinentry-mac.
pub fn enroll() -> Result<(), String> {
    let mut child = Command::new(FALLBACK)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("pinentry-mac: {e}"))?;
    {
        let mut stdin = child.stdin.take().expect("stdin piped");
        stdin
            .write_all(
                b"SETTITLE Quick Access\nSETDESC Palavra-passe mestra do Bitwarden, para desbloquear com Touch ID\nSETPROMPT Master Password\nSETREPEAT Outra vez\nSETREPEATERROR N%C3%A3o coincidem\nGETPIN\n",
            )
            .map_err(|e| e.to_string())?;
    }
    let mut pin = None;
    for line in BufReader::new(child.stdout.take().expect("stdout piped")).split(b'\n') {
        let line = line.map_err(|e| e.to_string())?;
        if let Some(d) = line.strip_prefix(b"D ") {
            pin = Some(unescape(d));
        } else if line.starts_with(b"ERR") {
            break;
        }
    }
    let _ = child.wait();
    let mut pin = pin.filter(|p| !p.is_empty()).ok_or("cancelado")?;
    let r = set_generic_password(SERVICE, ACCOUNT, &pin).map_err(|e| format!("Keychain: {e}"));
    pin.fill(0);
    r
}

fn unescape(d: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(d.len());
    let mut i = 0;
    while i < d.len() {
        let hex = d.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(std::str::from_utf8(h).ok()?, 16).ok());
        match (d[i], hex) {
            (b'%', Some(b)) => {
                out.push(b);
                i += 3;
            }
            (b, _) => {
                out.push(b);
                i += 1;
            }
        }
    }
    out
}

/// Só biometria: sem recurso ao código do Mac. Se o Touch ID falhar, quem decide é a
/// palavra-passe mestra no pinentry-mac.
fn touch_id(reason: &str) -> bool {
    let (tx, rx) = std::sync::mpsc::channel();
    let reply = RcBlock::new(move |ok: Bool, _: *mut NSError| {
        let _ = tx.send(ok.as_bool());
    });
    unsafe {
        LAContext::new().evaluatePolicy_localizedReason_reply(
            LAPolicy::DeviceOwnerAuthenticationWithBiometrics,
            &NSString::from_str(reason),
            &reply,
        );
    }
    rx.recv().unwrap_or(false)
}

/// `None` se não houver inscrição ou se o Touch ID não passar.
fn read_password() -> Option<Vec<u8>> {
    let pin = get_generic_password(SERVICE, ACCOUNT).ok()?;
    if touch_id("desbloquear o cofre") {
        Some(pin)
    } else {
        let mut pin = pin;
        pin.fill(0);
        None
    }
}

/// Assuan escapa `%`, CR e LF nas linhas `D`; o rbw descodifica.
fn escape(pin: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pin.len());
    for &b in pin {
        match b {
            b'%' => out.extend_from_slice(b"%25"),
            b'\r' => out.extend_from_slice(b"%0D"),
            b'\n' => out.extend_from_slice(b"%0A"),
            _ => out.push(b),
        }
    }
    out
}

pub fn run(args: &[String]) -> io::Result<()> {
    let mut out = io::stdout().lock();
    out.write_all(b"OK quick-access\n")?;
    out.flush()?;

    // Comandos já confirmados, para repetir ao pinentry-mac se for preciso.
    let mut seen: Vec<String> = Vec::new();
    // Só a palavra-passe mestra, e nunca depois de um erro: SETERROR significa que a
    // palavra-passe guardada falhou, e repeti-la seria um ciclo.
    let mut biometric = true;

    for line in io::stdin().lock().lines() {
        let line = line?;
        let cmd = line.split(' ').next().unwrap_or("");
        match cmd {
            "GETPIN" => {
                if biometric {
                    if let Some(mut pin) = read_password() {
                        let mut d = b"D ".to_vec();
                        d.extend(escape(&pin));
                        d.extend_from_slice(b"\nOK\n");
                        out.write_all(&d)?;
                        out.flush()?;
                        pin.fill(0);
                        d.fill(0);
                        continue;
                    }
                }
                return fallback(args, &seen, &mut out);
            }
            "BYE" => {
                out.write_all(b"OK\n")?;
                break;
            }
            _ => {
                if cmd == "SETERROR" || (cmd == "SETPROMPT" && line != "SETPROMPT Master Password") {
                    biometric = false;
                }
                seen.push(line);
                out.write_all(b"OK\n")?;
                out.flush()?;
            }
        }
    }
    Ok(())
}

/// Repete os comandos ao pinentry-mac e reencaminha a resposta ao GETPIN. Os `OK` dele aos
/// comandos repetidos (mais a saudação) já foram dados por nós, por isso saltam-se.
fn fallback(args: &[String], seen: &[String], out: &mut impl Write) -> io::Result<()> {
    let mut child = Command::new(FALLBACK)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let mut stdin = child.stdin.take().expect("stdin piped");
        for line in seen {
            writeln!(stdin, "{line}")?;
        }
        stdin.write_all(b"GETPIN\n")?;
    } // fecha o stdin: o pinentry-mac sai depois de responder

    let mut skip = seen.len() + 1;
    for line in BufReader::new(child.stdout.take().expect("stdout piped")).split(b'\n') {
        let mut line = line?;
        if skip > 0 && line.starts_with(b"OK") {
            skip -= 1;
            continue;
        }
        line.push(b'\n');
        out.write_all(&line)?;
        line.fill(0);
    }
    out.flush()?;
    child.wait()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{escape, unescape};

    #[test]
    fn escapes_like_rbw_decodes() {
        assert_eq!(escape(b"a%b\nc\rd"), b"a%25b%0Ac%0Dd");
        assert_eq!(escape(b"plain"), b"plain");
        assert_eq!(unescape(&escape(b"a%b\nc\rd%")), b"a%b\nc\rd%");
        assert_eq!(unescape(b"100%"), b"100%");
    }
}
