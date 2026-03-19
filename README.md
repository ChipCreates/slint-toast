Slint Toast Component

A pure‑UI, language‑agnostic toast/snackbar notification component for the Slint UI toolkit.  
This component provides a reusable visual primitive for transient notifications, designed for upstream inclusion in the Slint component library.

- No timers  
- No business logic  
- No backend assumptions  
- No dynamic children  
- No Rust/C++/JS/Python dependencies  

All behavioral orchestration (timers, queueing, sequencing, accessibility announcements) is the responsibility of the host application.

---

Features

- Four semantic toast kinds: Info, Success, Warning, Error
- Six anchor positions (top/bottom × left/center/right)
- Optional action button
- Optional icon
- Close button (configurable)
- Fully themeable via ToastStyle
- Palette‑aware defaults with accessible fallback colors
- Smooth fade + slide animations
- Accessibility roles and labels included
- Works in all Slint host languages

---

Repository Structure

`
slint-toast/
├── ui/
│   ├── toast-types.slint       # Enums and style struct — no visuals
│   ├── toast.slint             # Toast — visual atom
│   └── toast-host.slint        # ToastHost — positioning container
├── demo/
│   └── toast-demo.slint        # Self-contained demo for slint-viewer
└── README.md
`

Only .slint files are included. No backend code is required or provided.

---

Quick Start

1. Import the components

`slint
import { ToastHost, ToastKind } from "ui/toast-host.slint";
`

2. Add ToastHost as the last direct child of your root Window

`slint
export component AppWindow inherits Window {
    // ... your UI ...

    toast-host := ToastHost {
        anchor: BottomRight;
    }
}
`

3. Show a toast from your host language

`rust
ui.global::<ToastHost>().invoke_show("File saved.", ToastKind.Success);
`

4. Hide it later (timer or user action)

`rust
ui.global::<ToastHost>().invoke_hide();
`

---

Public API

1. ToastKind

`slint
export enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}
`

2. ToastAnchor

`slint
export enum ToastAnchor {
    BottomRight, BottomCenter, BottomLeft,
    TopRight,    TopCenter,    TopLeft,
}
`

3. ToastStyle

`slint
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
`

Zero‑value contract
Slint struct fields cannot have defaults.  
Zero values (0px, 0ms, transparent brushes) are treated as unset and replaced with component defaults.

---

4. Toast Component

The visual atom. No positioning, no timers, no queueing.

Properties

| Name | Type | Default | Notes |
|------|------|---------|-------|
| text | string | "" | Message text |
| kind | ToastKind | Info | Controls colors |
| visible | bool | false | Drives animations |
| enabled | bool | true | Disables buttons |
| show-close | bool | true | Renders close button |
| action-label | string | "" | Empty = no action button |
| icon | image | — | Optional |
| style | ToastStyle | — | Theme override |

Callbacks

| Callback | Fired when |
|----------|------------|
| closed() | Close button clicked |
| action() | Action button clicked |

---

5. ToastHost Component

The overlay container. Owns a single Toast instance and exposes a command‑driven interface.

Properties

| Name | Type | Default | Notes |
|------|------|---------|-------|
| text | string | "" | Forwarded to Toast |
| kind | ToastKind | Info | Forwarded |
| enabled | bool | true | Forwarded |
| show-close | bool | true | Forwarded |
| action-label | string | "" | Forwarded |
| icon | image | — | Forwarded |
| anchor | ToastAnchor | BottomRight | Overlay position |
| style | ToastStyle | — | Forwarded |

Functions (command input)

`slint
public function show(text: string, kind: ToastKind)
public function hide()
`

Callbacks (outbound)

`slint
callback toast-closed()
callback toast-action()
`

Important
ToastHost has no public visible property.  
Visibility is controlled exclusively by show() and hide().

---

Theming

Palette Integration
- Info uses Palette.accent-background and Palette.accent-foreground
- Other kinds use accessible fallback colors (WCAG AA compliant)
- All values can be overridden via ToastStyle

Default Animation Durations
| Field | Default |
|-------|---------|
| fade-in-duration | 180ms |
| fade-out-duration | 220ms |
| slide-duration | 200ms |

Default Shape
| Field | Default |
|-------|---------|
| border-radius | 6px |
| padding | 14px |

---

Z‑Order & Layout Rules

These rules are mandatory:

1. ToastHost must be the last direct child of the root Window.

Slint renders children in declaration order.  
Last child = topmost overlay.

2. ToastHost must NOT be placed inside a layout.

Layouts clip and constrain children.  
Toast overlays must float above all content.

Correct:

`slint
export component AppWindow inherits Window {
    // content...

    ToastHost { }
}
`

Incorrect:

`slint
VerticalLayout {
    ToastHost { }   // ❌ will not overlay correctly
}
`

---

Accessibility

Provided by the component:
- Message text: accessible-role: text
- Close button: accessible-role: button, label "Close"
- Action button: accessible-role: button, label bound to action-label
- Disabled state removes buttons from keyboard navigation

Limitations:
- Slint does not yet provide an alert role → screen readers may not announce toasts automatically  
- Close button label is not localized (static English "Close")

Applications requiring guaranteed screen reader announcement must implement supplemental logic.

---

Host Responsibilities

The host application must:

1. Place ToastHost as the last child of the root Window
2. Own a timer for auto‑dismiss
3. Own a queue for sequencing multiple toasts
4. Call show() and hide() at appropriate times
5. React to toast-closed() and toast-action()
6. Provide screen reader announcements if required

The component is intentionally pure UI.

---

Demo

A complete demo is included in:

`
demo/toast-demo.slint
`

Run it with:

`
slint-viewer demo/toast-demo.slint
`

The demo includes:
- All four kinds  
- Action button  
- Disabled state  
- No-close-button mode  
- Anchor selector  
- Custom style override  
- Interactive show/hide  

---

Contributing

This component is intended for upstream submission to the Slint project.

Before opening a PR:

1. Start a GitHub Discussion in the Slint repository  
2. Ensure the API matches maintainer expectations  
3. Sign the Contributor License Agreement (CLA) when prompted  
4. Run slint-fmt on all .slint files  
5. Ensure the demo runs in slint-viewer with no errors  

All contributions are licensed under MIT No Attribution.

---

License

MIT No Attribution  
See LICENSE for details.
