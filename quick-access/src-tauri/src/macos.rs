// Partes que só o AppKit faz: área de transferência oculta e bloqueio no sleep/ecrã.
use block2::RcBlock;
use objc2_app_kit::{
    NSPasteboard, NSStatusWindowLevel, NSWindow, NSWindowCollectionBehavior, NSPasteboardTypeString, NSWorkspace, NSWorkspaceScreensDidSleepNotification,
    NSWorkspaceWillSleepNotification,
};
use objc2_foundation::{NSDistributedNotificationCenter, NSNotification, NSString};
use std::process::Command;
use std::ptr::NonNull;
use std::time::Duration;

const CLEAR_AFTER: Duration = Duration::from_secs(45);

/// Marca `org.nspasteboard.ConcealedType` (gestores de histórico decentes não guardam) e
/// limpa aos 45 s, só se ninguém copiou nada entretanto: o `changeCount` só muda com uma
/// cópia nova, mais fiável do que comparar o texto.
pub fn copy_concealed(v: &str) {
    let pb = NSPasteboard::generalPasteboard();
    pb.clearContents();
    pb.setString_forType(&NSString::from_str(""), &NSString::from_str("org.nspasteboard.ConcealedType"));
    pb.setString_forType(&NSString::from_str(v), unsafe { NSPasteboardTypeString });
    let ours = pb.changeCount();
    std::thread::spawn(move || {
        std::thread::sleep(CLEAR_AFTER);
        let pb = NSPasteboard::generalPasteboard();
        if pb.changeCount() == ours {
            pb.clearContents();
        }
    });
}

/// Além do `lock_timeout` do rbw: bloqueia ao adormecer, ao apagar o ecrã e ao bloquear o ecrã.
pub fn install_lock_observers(app: tauri::AppHandle) {
    let lock = RcBlock::new(move |_: NonNull<NSNotification>| {
        let _ = Command::new(crate::RBW).arg("lock").status();
        crate::hide_panel(&app);
    });
    unsafe {
        let ws = NSWorkspace::sharedWorkspace().notificationCenter();
        for name in [NSWorkspaceWillSleepNotification, NSWorkspaceScreensDidSleepNotification] {
            // O centro guarda o observador; o token só serviria para o remover, e nunca removemos.
            std::mem::forget(ws.addObserverForName_object_queue_usingBlock(Some(name), None, None, &lock));
        }
        std::mem::forget(
            NSDistributedNotificationCenter::defaultCenter().addObserverForName_object_queue_usingBlock(
                Some(&NSString::from_str("com.apple.screenIsLocked")),
                None,
                None,
                &lock,
            ),
        );
    }
}

/// Sem `FullScreenAuxiliary` o macOS não deixa a janela aparecer sobre uma app em ecrã
/// inteiro; o `visibleOnAllWorkspaces` do Tauri só põe `CanJoinAllSpaces`.
pub fn float_over_fullscreen(w: &tauri::WebviewWindow) {
    let Ok(ptr) = w.ns_window() else { return };
    let win = unsafe { &*(ptr as *const NSWindow) };
    win.setCollectionBehavior(
        win.collectionBehavior()
            | NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::FullScreenAuxiliary,
    );
    win.setLevel(NSStatusWindowLevel);
}
