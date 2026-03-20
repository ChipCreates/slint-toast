// Slint toast component — Rust integration example.
//
// Demonstrates:
//   - Calling show() / hide() from Rust
//   - Auto-dismiss via slint::Timer::single_shot
//   - Reacting to toast-closed and toast-action callbacks

use std::time::Duration;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Show a welcome toast on startup that auto-dismisses after 4 seconds.
    ui.invoke_show_toast(
        "Welcome! This toast will auto-dismiss in 4 seconds.".into(),
        ToastKind::Info,
    );

    slint::Timer::single_shot(Duration::from_millis(4000), {
        let ui = ui.as_weak();
        move || {
            if let Some(ui) = ui.upgrade() {
                ui.invoke_hide_toast();
            }
        }
    });

    // React to the user manually closing the toast.
    // In a real app: cancel any pending dismiss timer here.
    ui.on_toast_closed(move || {
        eprintln!("Toast closed by user.");
    });

    // React to the action button.
    ui.on_toast_action(move || {
        eprintln!("Toast action triggered.");
    });

    ui.run()
}
