# Slint toast component — Python integration example.
#
# Demonstrates:
#   - Driving toast state via ToastGlobals (the correct pattern for Python hosts)
#   - Auto-dismiss via threading.Timer
#   - Reacting to toast-closed and toast-action via ToastGlobals callbacks
#
# Why ToastGlobals instead of show_toast / hide_toast?
# Slint's code generator only exposes the root Window's own public functions on
# the generated object. Named child sub-element functions (toast-host.show / hide)
# are not accessible from Python via the generated AppWindow API. ToastGlobals
# is the canonical bridge — see docs/slint-toast-api.md §7.
#
# Run:
#   pip install -r requirements.txt
#   python main.py

import os
import threading

import slint

_dir = os.path.dirname(os.path.abspath(__file__))
ui_module = slint.load_file(os.path.join(_dir, "ui", "app.slint"))

window = ui_module.AppWindow()
g = window.ToastGlobals

# Track the active dismiss timer so it can be cancelled if the user closes
# the toast manually before it expires.
_dismiss_timer: threading.Timer | None = None


def _cancel_timer() -> None:
    global _dismiss_timer
    if _dismiss_timer is not None:
        _dismiss_timer.cancel()
        _dismiss_timer = None


# Wire callbacks once at startup via ToastGlobals.
def _on_toast_closed() -> None:
    print("Toast closed by user.")
    _cancel_timer()
    g.active = False


def _on_toast_action() -> None:
    print("Toast action triggered.")


g.toast_closed = _on_toast_closed
g.toast_action = _on_toast_action

# Show a welcome toast on startup that auto-dismisses after 4 seconds.
g.active_text = "Welcome! This toast will auto-dismiss in 4 seconds."
g.active_kind = ui_module.ToastKind.Info
g.active = True

_dismiss_timer = threading.Timer(4.0, lambda: setattr(g, "active", False))
_dismiss_timer.start()

window.run()
