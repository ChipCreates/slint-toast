// Slint toast component — JavaScript integration example.
//
// Demonstrates:
//   - Calling show() / hide() from JavaScript
//   - Auto-dismiss via setTimeout
//   - Reacting to toast-closed and toast-action callbacks
//
// Run:
//   npm install
//   npm start

import * as slint from "slint-ui";
import { fileURLToPath } from "url";
import path from "path";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Load the Slint UI file.
// slint-ui compiles the .slint file on first load.
const ui = slint.loadFile(path.join(__dirname, "ui/app.slint"));
const window = new ui.AppWindow();

// Show a welcome toast on startup that auto-dismisses after 4 seconds.
// Slint converts kebab-case function names to snake_case for JavaScript.
window.show_toast("Welcome! This toast will auto-dismiss in 4 seconds.", { Info: {} });

const dismissTimer = setTimeout(() => {
    window.hide_toast();
}, 4000);

// React to the user manually closing the toast.
// In a real app: clearTimeout(dismissTimer) here to cancel the pending auto-dismiss.
window.toast_closed = () => {
    console.log("Toast closed by user.");
    clearTimeout(dismissTimer);
};

// React to the action button.
window.toast_action = () => {
    console.log("Toast action triggered.");
};

// Show the window and start the event loop.
window.run();
