// Quickwarden: Tauri panel on top of rbw. Secrets are never returned to JS.
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
    unlocking: AtomicBool, // while the pinentry is open, blur does not close the panel
}

// Runs rbw without a shell; returns stdout trimmed.
fn rbw(args: &[&str]) -> Option<String> {
    let o = Command::new(RBW).args(args).output().ok()?;
    o.status.success().then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn find(app: &App, id: &str) -> Result<Item, String> {
    let items = app.items.lock().map_err(|_| "Invalid state")?;
    items.iter().find(|i| i.id == id).cloned().ok_or_else(|| "Item not found".into())
}

#[tauri::command]
async fn list_items(app: State<'_, App>) -> Result<Vec<Item>, String> {
    if rbw(&["unlocked"]).is_none() {
        app.unlocking.store(true, Ordering::SeqCst);
        let _ = Command::new(RBW).arg("unlock").output();
        // focus returns to the panel a moment after the pinentry closes
        std::thread::sleep(std::time::Duration::from_millis(400));
        app.unlocking.store(false, Ordering::SeqCst);
    }
    let raw = rbw(&["list", "--raw"]).ok_or("Could not read the vault")?;
    let items: Vec<Item> = serde_json::from_str(&raw).map_err(|_| "Invalid response from rbw")?;
    *app.items.lock().map_err(|_| "Invalid state")? = items.clone();
    Ok(items)
}

#[tauri::command]
async fn copy(app: State<'_, App>, id: String, field: String) -> Result<(), String> {
    let item = find(&app, &id)?;
    let value = match field.as_str() {
        "user" => item.user.filter(|u| !u.is_empty()).ok_or("This item has no username")?,
        "password" => rbw(&["get", &id]).filter(|v| !v.is_empty()).ok_or("Could not get the password")?,
        "totp" => rbw(&["code", &id]).filter(|v| !v.is_empty()).ok_or("This item has no one-time code")?,
        _ => return Err("Invalid field".into()),
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
        .ok_or("This item has no http(s) URL")?;
    Command::new("open").arg(url).spawn().map_err(|_| "Could not open the browser")?;
    Ok(())
}

#[tauri::command]
fn open_bitwarden() -> Result<(), String> {
    Command::new("open").args(["-b", "com.bitwarden.desktop"]).spawn().map_err(|_| "Could not open Bitwarden")?;
    Ok(())
}

pub(crate) fn hide_panel(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.hide();
    }
    let _ = app.hide(); // returns focus to the previous app
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
    // launchd's PATH lacks Homebrew; rbw spawns rbw-agent via PATH.
    let path = std::env::var("PATH").unwrap_or_default();
    std::env::set_var("PATH", format!("/opt/homebrew/bin:{path}"));
    // The same binary doubles as rbw's pinentry (see pinentry.rs).
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
            println!("saved; rbw unlock will now ask for Touch ID");
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
                .expect("invalid shortcut")
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
        .expect("error starting Quickwarden");
}
