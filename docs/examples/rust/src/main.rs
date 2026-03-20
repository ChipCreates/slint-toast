// Slint toast component — Rust integration example.
//
// Demonstrates:
//   - Driving toast state via ToastGlobals (the correct pattern for Rust hosts)
//   - Auto-dismiss via slint::Timer
//   - Reacting to toast-closed and toast-action via ToastGlobals callbacks
//
// Why ToastGlobals instead of invoke_show_toast / invoke_hide_toast?
// Slint's code generator only exposes the root Window's own public functions on
// the generated struct. Named child sub-element functions (toast-host.show / hide)
// are not accessible from Rust via the generated AppWindow API. ToastGlobals is
// the canonical bridge — see docs/slint-toast-api.md §7.

use std::time::Duration;

use slint::ComponentHandle as _;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Wire callbacks once at startup via ToastGlobals.
    // In a real app: cancel any pending dismiss timer inside on_toast_closed.
    ui.global::<ToastGlobals>().on_toast_closed({
        let ui = ui.as_weak();
        move || {
            eprintln!("Toast closed by user.");
            if let Some(ui) = ui.upgrade() {
                ui.global::<ToastGlobals>().set_active(false);
            }
        }
    });

    ui.global::<ToastGlobals>().on_toast_action(|| {
        eprintln!("Toast action triggered.");
    });

    // Show a welcome toast on startup that auto-dismisses after 4 seconds.
    {
        let g = ui.global::<ToastGlobals>();
        g.set_active_text("Welcome! This toast will auto-dismiss in 4 seconds.".into());
        g.set_active_kind(ToastKind::Info);
        g.set_active(true);
    }

    slint::Timer::single_shot(Duration::from_millis(4000), {
        let ui = ui.as_weak();
        move || {
            if let Some(ui) = ui.upgrade() {
                ui.global::<ToastGlobals>().set_active(false);
            }
        }
    });

    ui.run()
}
