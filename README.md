# Slint Toast Component

![Slint Toast Component demo screenshot](docs/screenshots/demo-screen-1.png)

A pure-UI, language-agnostic toast/snackbar notification component for the [Slint](https://slint.dev) UI toolkit.
Designed for upstream inclusion in the Slint component library.

- No timers
- No business logic
- No backend assumptions
- No Rust/C++/JS/Python dependencies

All behavioral orchestration (timers, queueing, sequencing, screen reader announcements) is the responsibility of the host application.

---

## Features

- Four semantic toast kinds: `Info`, `Success`, `Warning`, `Error`
- Six anchor positions (top/bottom × left/center/right)
- Optional action button
- Optional icon
- Configurable close button
- Fully themeable via `ToastStyle`
- Palette-aware defaults with accessible fallback colors (WCAG AA)
- Smooth fade + slide animations (opacity-only — no layout reflow)
- Accessibility roles and labels built in

---

## Repository Structure

```
slint-toast/
├── ui/
│   ├── toast-types.slint       # Enums and ToastStyle struct — no visuals
│   ├── toast.slint             # Toast — the visual atom
│   └── toast-host.slint        # ToastHost — positioning container
├── demo/
│   └── toast-demo.slint        # Self-contained demo (slint-viewer compatible)
└── README.md
```

Only `.slint` files. No backend code required or provided.

---

## Quick Start

### 1. Import the components

```slint
import { ToastHost, ToastKind } from "ui/toast-host.slint";
```

### 2. Add `ToastHost` as the **last direct child** of your root `Window`

```slint
export component AppWindow inherits Window {
    // ... all your UI content ...

    // ToastHost MUST be last — renders on top of all other content
    toast-host := ToastHost {
        anchor: ToastAnchor.BottomRight;
    }
}
```

### 3. Show a toast (from your host language or Slint Button)

```slint
// Slint
toast-host.show("File saved.", ToastKind.Success);
```

```rust
// Rust
window.invoke_show("File saved.".into(), ToastKind::Success);
```

### 4. Hide it (on timer expiry or user action)

```slint
toast-host.hide();
```

---

## Public API

### `ToastKind` enum

```slint
export enum ToastKind {
    Info,      // zero-value default
    Success,
    Warning,
    Error,
}
```

### `ToastAnchor` enum

```slint
export enum ToastAnchor {
    BottomRight,   // default
    BottomCenter,
    BottomLeft,
    TopRight,
    TopCenter,
    TopLeft,
}
```

### `ToastStyle` struct

```slint
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
}
```

> **Zero-value contract:** Slint struct fields have no defaults. Zero values (`0px`, `0ms`, transparent brushes) are treated as *unset* and replaced with component defaults at render time. A host cannot use `ToastStyle` to request zero padding or instant animations — this is a known Slint language constraint.

### `Toast` component

The visual atom. No positioning, no timers, no queueing.

**Properties**

| Property | Type | Default | Notes |
|---|---|---|---|
| `text` | `string` | `""` | Notification message |
| `kind` | `ToastKind` | `Info` | Controls color resolution |
| `visible` | `bool` | `false` | Drives show/hide and animations |
| `enabled` | `bool` | `true` | When false, buttons are non-interactive |
| `show-close` | `bool` | `true` | Whether the close button is rendered |
| `action-label` | `string` | `""` | Empty = no action button rendered |
| `icon` | `image` | — | Optional. Detected via `icon.width > 0` |
| `style` | `ToastStyle` | — | Visual override. Zero fields use defaults |

**Callbacks**

| Callback | Fired when |
|---|---|
| `closed()` | User clicks the close button |
| `action()` | User clicks the action button |

### `ToastHost` component

The overlay container. Owns a single `Toast` instance. Command-driven — no public `visible` property.

**Properties**

| Property | Type | Default | Notes |
|---|---|---|---|
| `text` | `string` | `""` | Forwarded to `Toast.text` |
| `kind` | `ToastKind` | `Info` | Forwarded |
| `enabled` | `bool` | `true` | Forwarded |
| `show-close` | `bool` | `true` | Forwarded |
| `action-label` | `string` | `""` | Forwarded |
| `icon` | `image` | — | Forwarded |
| `anchor` | `ToastAnchor` | `BottomRight` | Overlay position |
| `style` | `ToastStyle` | — | Forwarded |

> There is intentionally **no public `visible` property** on `ToastHost`. Visibility is controlled exclusively by `show()` and `hide()`.

**Functions**

```slint
public function show(text: string, kind: ToastKind)
public function hide()
```

**Callbacks**

| Callback | Fired when |
|---|---|
| `toast-closed()` | Internal `Toast.closed` fires |
| `toast-action()` | Internal `Toast.action` fires |

---

## Theming

### Palette integration (hybrid strategy)

| Kind | Background | Foreground |
|---|---|---|
| `Info` | `Palette.accent-background` | `Palette.accent-foreground` |
| `Success` | light `#2e7d32` / dark `#388e3c` | `#ffffff` |
| `Warning` | light `#e65100` / dark `#f57c00` | `#000000` |
| `Error` | light `#c62828` / dark `#ef5350` | `#ffffff` |

`Info` automatically adapts to Fluent, Cosmic, and Material styles via `Palette`. All colors are WCAG AA compliant.

### Animation defaults

| Field | Default |
|---|---|
| `fade-in-duration` | `180ms` |
| `fade-out-duration` | `220ms` |
| `slide-duration` | `200ms` |

### Shape defaults

| Field | Default |
|---|---|
| `border-radius` | `6px` |
| `padding` | `14px` |

### Custom style example

```slint
toast-host := ToastHost {
    anchor: ToastAnchor.BottomRight;
    style: {
        background-error:  #7f1d1d,
        foreground-error:  #fef2f2,
        border-radius:     12px,
        fade-out-duration: 300ms,
    };
}
```

---

## Z-Order and Layout Rules

> These rules are **mandatory** for correct overlay behavior.

### Rule 1: `ToastHost` must be the **last direct child** of the root `Window`

Slint renders children in declaration order. The last child renders on top.

```slint
export component AppWindow inherits Window {
    // All other content — layouts, panels, etc.
    VerticalLayout { ... }

    // ToastHost MUST be declared last
    toast-host := ToastHost { }
}
```

### Rule 2: `ToastHost` must **not** be nested inside any layout

Layout elements (`HorizontalLayout`, `VerticalLayout`, `GridLayout`) constrain children to their bounds. The toast overlay must float above all content.

```slint
// ✓ Correct
export component AppWindow inherits Window {
    toast-host := ToastHost { }
}

// ✗ Incorrect — ToastHost inside a layout
VerticalLayout {
    ToastHost { }   // will not overlay correctly
}
```

---

## Accessibility

The component provides internally:

| Element | `accessible-role` | `accessible-label` |
|---|---|---|
| Message text | `text` | bound to `text` property |
| Close button | `button` | `"Close"` (static) |
| Action button | `button` | bound to `action-label` |

When `enabled = false`, close and action buttons are removed from keyboard focus traversal.

### Known limitations

- **Screen reader announcement:** Slint does not provide an `alert` accessible role. This component cannot guarantee proactive screen reader announcement when a toast appears. Applications requiring guaranteed announcement must implement supplemental logic at the application level.
- **Close button i18n:** The close button `accessible-label` is the static English string `"Close"`. Localisation is out of scope and deferred to a future enhancement.

---

## Host Responsibilities

To use this component correctly, the host application must:

1. **Place `ToastHost` as the last direct child** of the root `Window` (not nested in any layout)
2. **Own a timer** — on expiry, call `toast-host.hide()`
3. **Own a queue** — on `toast-closed()` or timer expiry, pop the next entry and call `toast-host.show(text, kind)`
4. **Set properties** (`action-label`, `icon`, `style`) before calling `show()`
5. **React to `toast-action()`** with application-specific logic
6. **Implement supplemental screen reader announcement** if the application requires guaranteed assistive technology notification

A minimal host implementation requires approximately 20–30 lines in any supported language.

---

## Demo

```
slint-viewer demo/toast-demo.slint
```

Covers: all four kinds, interactive dismiss, action button, `show-close: false`, `enabled: false`, anchor selector across all six positions, and custom `ToastStyle` override.

---

## Known Limitations

| Limitation | Detail |
|---|---|
| Screen reader announcement | No `alert` role in Slint — host must supplement |
| Close button i18n | Static English label `"Close"` |
| `ToastHost` placement | Must be last child of `Window`, not in any layout |
| `ToastStyle` zero values | Cannot request zero padding or instant animations via style |
| Single toast | `ToastHost` owns one `Toast` — queuing is host responsibility |

---

## Contributing

This component is intended for upstream submission to the Slint project.

Before opening a PR:

1. Open a **GitHub Discussion** in the Slint repository to validate maintainer interest
2. Wait for at least one maintainer response before writing code
3. Sign the **Contributor License Agreement (CLA)** when prompted during the PR process
4. Run `slint-fmt` on all `.slint` files — required before any PR is accepted
5. Verify `slint-viewer demo/toast-demo.slint` launches with no errors

All contributions are licensed under **MIT No Attribution**.

---

## License

MIT — see [LICENSE](LICENSE) for details.
