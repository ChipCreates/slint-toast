# Slint Toast Component — Public API Specification

> Upstream contribution proposal for the Slint component library.
> Pure UI. Language-agnostic. No timers. No business logic.

---

## 1. `ToastKind` enum

Semantic category of the notification. Controls which default `Palette`-derived
colors are applied when no `ToastStyle` override is provided.

```slint
export enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}
```

`Info` is first and therefore the zero-value default when `kind` is not set
by the host. This is the correct default for a notification system.

---

## 2. `ToastAnchor` enum

Anchor position of the `ToastHost` overlay within its parent window.

```slint
export enum ToastAnchor {
    BottomRight,
    BottomCenter,
    BottomLeft,
    TopRight,
    TopCenter,
    TopLeft,
}
```

---

## 3. `ToastStyle` struct

Complete visual configuration for the toast system. All fields are optional
in the sense that a host may pass `ToastStyle {}` (all zero values) and the
component will substitute sensible Palette-derived defaults at render time.

```slint
export struct ToastStyle {
    // Per-kind background colors
    background-info:    brush,
    background-success: brush,
    background-warning: brush,
    background-error:   brush,

    // Per-kind foreground colors
    foreground-info:    brush,
    foreground-success: brush,
    foreground-warning: brush,
    foreground-error:   brush,

    // Shape and spacing
    border-radius: length,
    padding:       length,

    // Animation durations
    fade-in-duration:  duration,
    fade-out-duration: duration,
    slide-duration:    duration,
}
```

### Zero-value contract

Slint does not support struct field default values. All fields initialise to
their type's zero value (`brush` → fully transparent, `length` → `0px`,
`duration` → `0ms`). **Zero values are treated by the component as "unset"
and are replaced at render time by the defaults documented in the Theming
Contract.** A host that intentionally wants zero padding or instant animations
cannot express that through `ToastStyle` — this is a known constraint of the
current Slint language and is documented here explicitly.

### Palette integration

When brush fields are unset (transparent), the component resolves defaults
from `Palette` where a direct semantic mapping exists, and from a defined
set of accessible fallback colors where `Palette` has no equivalent. The full
default color table is defined in the Architecture document.

Integrators targeting a custom theme should populate the relevant `ToastStyle`
fields explicitly rather than relying on defaults.

---

## 4. `Toast` component

The visual atom. Renders a single toast notification. Has no knowledge of
positioning, stacking, timers, or the host application.

### Properties

| Property | Type | Direction | Default | Notes |
|---|---|---|---|---|
| `text` | `string` | `in` | `""` | The notification message |
| `kind` | `ToastKind` | `in` | `Info` | Controls color resolution |
| `visible` | `bool` | `in` | `false` | Drives show/hide state and animations |
| `enabled` | `bool` | `in` | `true` | When false, close and action buttons are non-interactive |
| `show-close` | `bool` | `in` | `true` | Whether the close button is rendered |
| `action-label` | `string` | `in` | `""` | Empty string = no action button rendered |
| `icon` | `image` | `in` | — | Optional. Detected via `icon.width > 0` (see note) |
| `style` | `ToastStyle` | `in` | — | Visual override. Zero fields use defaults |

> **Icon detection note:** Slint's `image` type has no null/none sentinel.
> The component detects a populated icon via `icon.width > 0 && icon.height > 0`.
> This is the idiomatic Slint pattern for optional images and is a platform
> constraint, not a design choice.

### Callbacks

| Callback | Direction | Fired when |
|---|---|---|
| `closed()` | out | User clicks the close button |
| `action()` | out | User clicks the action button |

### Accessibility

The `Toast` component declares the following accessible properties internally.
These are not part of the public property API but are required for upstream
inclusion.

| Element | `accessible-role` | `accessible-label` |
|---|---|---|
| Message text | `text` | bound to `text` property |
| Close button | `button` | `"Close"` (static English string — see i18n note) |
| Action button | `button` | bound to `action-label` property |

When `enabled = false`, the close button and action button are non-interactive
and must not be reachable via keyboard navigation.

> **Screen reader announcement:** Because Slint's `AccessibleRole` enum does
> not include an `alert` role, screen reader announcement of toast notifications
> depends on the host application and platform. This component cannot guarantee
> proactive announcement. This limitation should be noted in host application
> documentation.

> **i18n note:** The close button `accessible-label` is the static English
> string `"Close"`. Localisation is out of scope for this component and is
> deferred to a future enhancement.

### Animation

Driven by `visible` via Slint `states` and `transitions`. The component uses
an `opacity` property for fading rather than toggling `visible` directly, so
the element remains present in the render tree during the exit animation.
Specifically:

```
element visible = (root.visible || display-opacity > 0.0)
```

This guarantees the fade-out animation completes fully before the element
is removed from layout. No `Timer` is used anywhere in this component.

---

## 5. `ToastHost` component

The positioning container. Anchors the toast overlay within the window and
owns the single internal `Toast` instance.

`ToastHost` is **command-driven**. There is no public `visible` property —
visibility is controlled exclusively through `ToastGlobals.active` (written by
`show()` and `hide()` for Slint-to-Slint callers, or directly by generated-binding
hosts). This eliminates the dual-source-of-truth problem.

Per-toast state (`text`, `kind`, `action-label`) lives in `ToastGlobals` so it
is accessible to all host languages. Per-instance configuration (`enabled`,
`show-close`, `anchor`, `style`, `icon`) remains as `in` properties on the element,
since these are stable across show/hide cycles and set once in the host window.

### Per-instance properties

| Property | Type | Direction | Default | Notes |
|---|---|---|---|---|
| `enabled` | `bool` | `in` | `true` | Forwarded to internal `Toast.enabled` |
| `show-close` | `bool` | `in` | `true` | Forwarded to internal `Toast.show-close` |
| `icon` | `image` | `in` | — | Forwarded to internal `Toast.icon` |
| `anchor` | `ToastAnchor` | `in` | `BottomRight` | Controls overlay position |
| `style` | `ToastStyle` | `in` | — | Forwarded to internal `Toast.style` |

### Per-toast state — `ToastGlobals`

| Global property | Type | Direction | Default | Notes |
|---|---|---|---|---|
| `ToastGlobals.active` | `bool` | `in-out` | `false` | Sole source of truth for visibility |
| `ToastGlobals.active-text` | `string` | `in-out` | `""` | The notification message |
| `ToastGlobals.active-kind` | `ToastKind` | `in-out` | `Info` | Controls color resolution |
| `ToastGlobals.action-label` | `string` | `in-out` | `""` | Non-empty = action button rendered |

### Functions (Slint-to-Slint convenience)

These write to `ToastGlobals` internally and are unchanged from the caller's perspective.
Generated-binding hosts cannot call these on the parent Window's generated API — see §7.

| Function | Signature | Behaviour |
|---|---|---|
| `show` | `(text: string, kind: ToastKind)` | Writes `ToastGlobals.active-text/kind`, sets `ToastGlobals.active = true` |
| `hide` | `()` | Sets `ToastGlobals.active = false` |

### Callbacks — `ToastGlobals`

| Global callback | Direction | Fired when |
|---|---|---|
| `ToastGlobals.toast-closed()` | out | Internal `Toast.closed` fires |
| `ToastGlobals.toast-action()` | out | Internal `Toast.action` fires |

### Z-order and overlay placement

`ToastHost` must be placed as the **last child** of the root `Window` element.
Slint renders children in declaration order; last child renders on top. The
host is responsible for correct placement — `ToastHost` does not manipulate
z-order itself, and it must not be nested inside any layout element.

```slint
export component AppWindow inherits Window {
    // ... all other UI content ...

    // ToastHost MUST be last — renders on top of all other content
    toast-host := ToastHost {
        anchor: BottomRight;
    }
}
```

---

## 6. Complete API Summary

```slint
export enum ToastKind {
    Info, Success, Warning, Error,
}

export enum ToastAnchor {
    BottomRight, BottomCenter, BottomLeft,
    TopRight,    TopCenter,    TopLeft,
}

export struct ToastStyle {
    background-info:    brush,
    background-success: brush,
    background-warning: brush,
    background-error:   brush,
    foreground-info:    brush,
    foreground-success: brush,
    foreground-warning: brush,
    foreground-error:   brush,
    border-radius:      length,
    padding:            length,
    fade-in-duration:   duration,
    fade-out-duration:  duration,
    slide-duration:     duration,
    // Zero values are treated as unset — see Zero-value contract
}

component Toast {
    in property <string>     text;
    in property <ToastKind>  kind;
    in property <bool>       visible;
    in property <bool>       enabled;       // default: true
    in property <bool>       show-close;    // default: true
    in property <string>     action-label;  // empty = no button rendered
    in property <image>      icon;          // detected via icon.width > 0
    in property <ToastStyle> style;         // zero fields use defaults

    callback closed();
    callback action();

    // Internal accessibility (not public properties):
    // accessible-role: text        on message element
    // accessible-role: button      on close and action buttons
    // accessible-label: text       on message element
    // accessible-label: "Close"    on close button
    // accessible-label: action-label on action button
}

// Per-instance configuration — stable across show/hide cycles
component ToastHost {
    in property <bool>        enabled;       // default: true
    in property <bool>        show-close;    // default: true
    in property <image>       icon;
    in property <ToastAnchor> anchor;        // default: BottomRight
    in property <ToastStyle>  style;

    // Slint-to-Slint callers: show()/hide() write to ToastGlobals internally.
    // Generated-binding hosts: write to ToastGlobals directly (see §7).
    public function show(text: string, kind: ToastKind) { ... }
    public function hide() { ... }
}

// Per-toast state and callbacks — the bridge for generated-binding hosts
global ToastGlobals {
    in-out property <bool>      active;        // default: false
    in-out property <string>    active-text;   // default: ""
    in-out property <ToastKind> active-kind;   // default: Info
    in-out property <string>    action-label;  // default: "" (empty = no button)

    callback toast-closed();
    callback toast-action();
}
```

---

## 7. Host-language integration note — generated bindings (Rust, C++, JavaScript)

### The limitation

When `ToastHost` is used as a named child of a root `Window` in an application with
generated language bindings, the host application's generated `Window` struct (e.g.
Rust's `MainWindow`) **does not expose `show()` or `hide()` as callable methods**.

Slint's code generator only surfaces the root `Window`'s own `public function`,
`callback`, and `in-out property` declarations in the generated API. Named child
sub-element functions — including `toast-host.show()` and `toast-host.hide()` —
are internal to the Slint tree and are not accessible from outside via the
generated bindings.

This is a Slint code-generation constraint, not a design issue with this component.
It affects **all** host languages that use generated bindings: Rust, C++, JavaScript.

> **Slint-to-Slint callers** are not affected. Within a `.slint` file,
> `toast-host.show(text, kind)` works as documented.

### The solution — `ToastGlobals` pattern

The idiomatic workaround is to introduce a **Slint global singleton** that
`ToastHost` binds to, replacing the private state that `show()`/`hide()` previously
drove. The host application then drives toast state through the global, which is
accessible via `ui.global::<ToastGlobals>()` (Rust), `ui->global<ToastGlobals>()`
(C++), etc.

```slint
// toast_globals.slint — add to your application
import { ToastKind } from "toast_types.slint";

export global ToastGlobals {
    in-out property <bool>      active: false;
    in-out property <string>    active-text: "";
    in-out property <ToastKind> active-kind: ToastKind.Info;
    in-out property <string>    action-label: "";

    callback toast-closed();
    callback toast-action();
}
```

`ToastHost` is then modified to bind its internal state to `ToastGlobals` instead
of using private properties driven by `show()`/`hide()`. The `Toast` child reads
`ToastGlobals.active`, `ToastGlobals.active-text`, etc. directly, and its callbacks
forward into `ToastGlobals.toast-closed()` and `ToastGlobals.toast-action()`.

```slint
// Modified toast_host.slint for generated-binding hosts
import { ToastGlobals } from "toast_globals.slint";

// Inside the Toast child:
text:         ToastGlobals.active-text;
kind:         ToastGlobals.active-kind;
show:         ToastGlobals.active;
action-label: ToastGlobals.action-label;
closed => { ToastGlobals.toast-closed(); }
action => { ToastGlobals.toast-action(); }
```

The host application then controls toast state entirely through the global:

```rust
// Rust example
let g = ui.global::<ToastGlobals>();
g.set_active_text("Saved".into());
g.set_active_kind(ToastKind::Success);
g.set_active(true);

// Register callbacks once at startup
g.on_toast_closed({
    let weak = ui.as_weak();
    move || {
        if let Some(win) = weak.upgrade() {
            win.global::<ToastGlobals>().set_active(false);
        }
    }
});
```

### API summary with `ToastGlobals`

| Concern | Original `show()`/`hide()` API | `ToastGlobals` pattern |
|---|---|---|
| Show a toast | `toast-host.show(text, kind)` | `g.set_active_text(…); g.set_active_kind(…); g.set_active(true)` |
| Hide a toast | `toast-host.hide()` | `g.set_active(false)` |
| React to close | `on_toast_host_toast_closed` | `g.on_toast_closed(…)` |
| React to action | `on_toast_host_toast_action` | `g.on_toast_action(…)` |
| Slint-to-Slint | ✅ Works as documented | ✅ Works via global |
| Rust/C++/JS | ❌ Not in generated API | ✅ Accessible via `ui.global::<ToastGlobals>()` |

### Responsibility boundary unchanged

The `ToastGlobals` pattern does not change the component's responsibility boundary.
The host application still owns the dismiss timer, queue policy, and action handler.
`ToastGlobals` is the bridge — it is not business logic.

---

## 8. Revision History

### v4 (this revision) — ToastGlobals integration

| Item | Change |
|---|---|
| `toast_globals.slint` | **Added.** `ToastGlobals` singleton bridges generated-binding hosts (Rust, C++, JS) to `ToastHost` state. |
| `ToastHost` internal state | **Moved to `ToastGlobals`.** `active-visible`, `active-text`, `active-kind` replaced by `ToastGlobals.active/active-text/active-kind`. |
| `ToastHost.show()/hide()` | **Preserved as Slint-to-Slint convenience.** Now write to `ToastGlobals` internally instead of private properties. Caller API unchanged. |
| `ToastHost` properties | `text`, `kind`, `action-label` removed from per-instance `in` properties — superseded by `ToastGlobals`. |
| §7 host-language note | **Added.** Documents the Slint code-generation constraint and `ToastGlobals` as the canonical workaround. |

### v3 — post peer review

| Item | Change |
|---|---|
| `ToastHost.visible` property | **Removed.** `ToastHost` is now command-driven only via `show()`/`hide()`. Eliminates dual-source-of-truth. |
| Animation persistence | **Clarified.** Exit animation is guaranteed complete via `visible = (root.visible \|\| opacity > 0)` binding — no `is-showing` lifecycle needed. |
| `ToastStyle` zero-value contract | **Explicit.** Zero = unset is now a named, documented contract, not an implicit behaviour. |
| Palette fallback strategy | **Formalised.** Hybrid approach named and defined — Palette where available, defined accessible fallbacks where not. Full table in Architecture document. |
| Screen reader announcement | **Added.** Explicit limitation note — no `alert` role means no guaranteed proactive announcement. |
| Icon detection | **Documented.** `icon.width > 0` pattern named as idiomatic Slint constraint. |
| Width/height collapse | **Removed.** Layout stability preserved via opacity-based animation only. |

### v2 — post gap analysis

Added: `enabled`, accessibility spec, `Palette` integration, `ToastAnchor`, `ToastStyle`, z-order guidance, CLA notes.

### v1 — initial

Original API surface from maintainer-approved design.
