//! rbw pinentry with Touch ID.
//!
//! rbw-agent asks a pinentry program for the master password (Assuan protocol, see rbw's
//! `src/pinentry.rs`). This mode stores it in the Keychain and answers `GETPIN` after
//! Touch ID. Not enrolled, Touch ID cancelled, or any request that isn't the master
//! password → everything is passed to `pinentry-mac`.
//!
//! Known limit: without a Developer ID there is no `biometryCurrentSet` item (the protected
//! Keychain returns -34018), so the item lives in the login Keychain, with its ACL tied to
//! this binary's signature, and it is the `LAContext` here that enforces Touch ID, not the Keychain.

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

/// Asks pinentry-mac for the password (twice, `SETREPEAT`) and stores it. Needs no
/// terminal. Doesn't validate it: if wrong, rbw sends `SETERROR` and falls back to pinentry-mac.
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
                b"SETTITLE Quick Access\nSETDESC Bitwarden master password, to unlock with Touch ID\nSETPROMPT Master Password\nSETREPEAT Repeat\nSETREPEATERROR Passwords do not match\nGETPIN\n",
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
    let mut pin = pin.filter(|p| !p.is_empty()).ok_or("cancelled")?;
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

/// Biometrics only: no fallback to the Mac passcode. If Touch ID fails, the master
/// password in pinentry-mac decides.
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

/// `None` if not enrolled or Touch ID fails.
fn read_password() -> Option<Vec<u8>> {
    let pin = get_generic_password(SERVICE, ACCOUNT).ok()?;
    if touch_id("unlock the vault") {
        Some(pin)
    } else {
        let mut pin = pin;
        pin.fill(0);
        None
    }
}

/// Assuan escapes `%`, CR and LF in `D` lines; rbw decodes them.
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

    // Commands already acknowledged, to replay to pinentry-mac if needed.
    let mut seen: Vec<String> = Vec::new();
    // Master password only, and never after an error: SETERROR means the stored
    // password failed, and retrying it would loop.
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

/// Replays the commands to pinentry-mac and forwards the GETPIN response. Its `OK`s to the
/// replayed commands (plus the greeting) were already sent by us, so they are skipped.
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
    } // closes stdin: pinentry-mac exits after answering

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
