// Quick Access: painel Tauri sobre o rbw. Segredos nunca são devolvidos ao JS.
mod macos;
mod pinentry;

use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{ActivationPolicy, AppHandle, Emitter, Manager, State, WebviewWindow, WindowEvent};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

pub(crate) const RBW: &str = "/opt/homebrew/bin/rbw";

#[derive(Clone, Serialize, Deserialize)]
struct Item {
    id: String,
    name: String,
    user: Option<String>,
    folder: Option<String>,
    uris: Option<Vec<String>>,
    #[serde(rename = "type")]
    kind: Option<String>,
}

#[derive(Default)]
struct App {
    items: Mutex<Vec<Item>>,
    unlocking: AtomicBool, // enquanto o pinentry está aberto, o blur não fecha o painel
}

// Corre o rbw sem shell; devolve stdout sem espaços nas pontas.
fn rbw(args: &[&str]) -> Option<String> {
    let o = Command::new(RBW).args(args).output().ok()?;
    o.status.success().then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn find(app: &App, id: &str) -> Result<Item, String> {
    let items = app.items.lock().map_err(|_| "Estado inválido")?;
    items.iter().find(|i| i.id == id).cloned().ok_or_else(|| "Item não encontrado".into())
}

#[tauri::command]
async fn list_items(app: State<'_, App>) -> Result<Vec<Item>, String> {
    if rbw(&["unlocked"]).is_none() {
        app.unlocking.store(true, Ordering::SeqCst);
        let _ = Command::new(RBW).arg("unlock").output();
        // o foco volta ao painel um instante depois do pinentry fechar
        std::thread::sleep(std::time::Duration::from_millis(400));
        app.unlocking.store(false, Ordering::SeqCst);
    }
    let raw = rbw(&["list", "--raw"]).ok_or("Não foi possível ler o cofre")?;
    let items: Vec<Item> = serde_json::from_str(&raw).map_err(|_| "Resposta do rbw inválida")?;
    *app.items.lock().map_err(|_| "Estado inválido")? = items.clone();
    Ok(items)
}

#[tauri::command]
async fn copy(app: State<'_, App>, id: String, field: String) -> Result<(), String> {
    let item = find(&app, &id)?;
    let value = match field.as_str() {
        "user" => item.user.filter(|u| !u.is_empty()).ok_or("Este item não tem nome de usuário")?,
        "password" => rbw(&["get", &id]).filter(|v| !v.is_empty()).ok_or("Não foi possível obter a senha")?,
        "totp" => rbw(&["code", &id]).filter(|v| !v.is_empty()).ok_or("Este item não tem código de uso único")?,
        _ => return Err("Campo inválido".into()),
    };
    macos::copy_concealed(&value);
    Ok(())
}

#[tauri::command]
fn open_url(app: State<'_, App>, id: String) -> Result<(), String> {
    let item = find(&app, &id)?;
    let url = item
        .uris
        .unwrap_or_default()
        .into_iter()
        .find(|u| u.starts_with("http://") || u.starts_with("https://"))
        .ok_or("Este item não tem URL http(s)")?;
    Command::new("open").arg(url).spawn().map_err(|_| "Não foi possível abrir o navegador")?;
    Ok(())
}

#[tauri::command]
fn open_bitwarden() -> Result<(), String> {
    Command::new("open").args(["-b", "com.bitwarden.desktop"]).spawn().map_err(|_| "Não foi possível abrir o Bitwarden")?;
    Ok(())
}

pub(crate) fn hide_panel(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
    let _ = app.hide(); // devolve o foco à app anterior
}

#[tauri::command]
fn hide(app: AppHandle) {
    hide_panel(&app);
}

fn show_panel(app: &AppHandle, w: &WebviewWindow) {
    let _ = w.center();
    let _ = app.show();
    let _ = w.show();
    let _ = w.set_focus();
    let _ = app.emit("show", ());
}

fn main() {
    // O mesmo binário faz de pinentry do rbw (ver pinentry.rs).
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--pinentry") => {
            let _ = pinentry::run(&args[1..]);
            return;
        }
        Some("--enroll") => {
            if let Err(e) = pinentry::enroll() {
                eprintln!("{e}");
                std::process::exit(1);
            }
            println!("guardada; o rbw unlock passa a pedir Touch ID");
            return;
        }
        _ => {}
    }
    let toggle = Shortcut::new(Some(Modifiers::SHIFT | Modifiers::SUPER), Code::Space);
    tauri::Builder::default()
        .manage(App::default())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(toggle)
                .expect("atalho inválido")
                .with_handler(move |app, _, ev| {
                    if ev.state() != ShortcutState::Pressed {
                        return;
                    }
                    if let Some(w) = app.get_webview_window("main") {
                        if w.is_visible().unwrap_or(false) {
                            hide_panel(app);
                        } else {
                            show_panel(app, &w);
                        }
                    }
                })
                .build(),
        )
        .invoke_handler(tauri::generate_handler![list_items, copy, open_url, open_bitwarden, hide])
        .on_window_event(|win, ev| {
            if let WindowEvent::Focused(false) = ev {
                if !win.app_handle().state::<App>().unlocking.load(Ordering::SeqCst) {
                    hide_panel(win.app_handle());
                }
            }
        })
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            macos::install_lock_observers(app.handle().clone());
            if let Some(w) = app.get_webview_window("main") {
                macos::float_over_fullscreen(&w);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("erro a arrancar o Quick Access");
}
