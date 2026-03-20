// Slint toast component — JavaScript integration example.
//
// Demonstrates:
//   - Driving toast state via ToastGlobals (the correct pattern for JS hosts)
//   - Auto-dismiss via setTimeout
//   - Reacting to toast-closed and toast-action via ToastGlobals callbacks
//
// Why ToastGlobals instead of show_toast / hide_toast?
// Slint's code generator only exposes the root Window's own public functions on
// the generated object. Named child sub-element functions (toast-host.show / hide)
// are not accessible from JavaScript via the generated AppWindow API. ToastGlobals
// is the canonical bridge — see docs/slint-toast-api.md §7.
//
// Run:
//   npm install
//   npm start

import * as slint from "slint-ui";
import { fileURLToPath } from "url";
import path from "path";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const ui = slint.loadFile(path.join(__dirname, "ui/app.slint"));
const window = new ui.AppWindow();
const globals = window.ToastGlobals;

// Wire callbacks once at startup via ToastGlobals.
// In a real app: clearTimeout(dismissTimer) inside toast_closed.
globals.toast_closed = () => {
    console.log("Toast closed by user.");
    clearTimeout(dismissTimer);
    globals.active = false;
};

globals.toast_action = () => {
    console.log("Toast action triggered.");
};

// Show a welcome toast on startup that auto-dismisses after 4 seconds.
globals.active_text = "Welcome! This toast will auto-dismiss in 4 seconds.";
globals.active_kind = { Info: {} };
globals.active = true;

const dismissTimer = setTimeout(() => {
    globals.active = false;
}, 4000);

window.run();
