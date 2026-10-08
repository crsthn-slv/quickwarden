// Parts only AppKit can do: concealed clipboard and locking on sleep/screen events.
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

/// Marks `org.nspasteboard.ConcealedType` (decent clipboard managers skip it) and clears
/// after 45 s, only if nothing else was copied meanwhile: `changeCount` only changes on a
/// new copy, more reliable than comparing the text.
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

/// On top of rbw's `lock_timeout`: lock on sleep, on display sleep and on screen lock.
pub fn install_lock_observers(app: tauri::AppHandle) {
    let lock = RcBlock::new(move |_: NonNull<NSNotification>| {
        let _ = Command::new(crate::RBW).arg("lock").status();
        crate::hide_panel(&app);
    });
    unsafe {
        let ws = NSWorkspace::sharedWorkspace().notificationCenter();
        for name in [NSWorkspaceWillSleepNotification, NSWorkspaceScreensDidSleepNotification] {
            // The center keeps the observer; the token would only be used to remove it, and we never do.
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

/// Without `FullScreenAuxiliary` macOS won't show the window over a fullscreen app;
/// Tauri's `visibleOnAllWorkspaces` only sets `CanJoinAllSpaces`.
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
