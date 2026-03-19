# Slint Toast Component — Full Implementation Plan

> A complete, sequenced plan for writing and submitting this component to the
> Slint component library. Each phase builds on the last. Nothing is skipped.

---

## Overview

The component is three `.slint` files plus a demo and a README. The
implementation is pure Slint — no Rust, no C++, no build scripts. The work
divides into six phases:

```
Phase 1 — Community Validation       (pre-code)
Phase 2 — Repository Setup           (pre-code)
Phase 3 — toast-types.slint          (data layer)
Phase 4 — toast.slint                (visual atom)
Phase 5 — toast-host.slint           (container)
Phase 6 — Demo + README + Submission (delivery)
```

Total estimated scope: 3–5 focused sessions.

---

## Phase 1 — Community Validation

**Do this before writing a single line of `.slint`.**

### 1.1 — Open a GitHub Discussion

Navigate to: https://github.com/slint-ui/slint/discussions

Open a new Discussion with category "Ideas" or "Show and Tell". Title it:

> **[Proposal] Toast / Snackbar notification component for std-widgets**

The body should include:

- One-paragraph problem statement (no built-in toast, real gap in the
  ecosystem)
- The API summary block from the API specification document (v3)
- The three-file architecture overview
- Explicit statement: pure UI, no timers, language-agnostic, command-driven
  host interface
- A direct question: "Is this the right shape for upstream inclusion? Are
  there API or structural concerns before I begin implementation?"

Attach or link the API specification document. Do not frame this as the
foundation for a broader notification pattern system — present it as exactly
what it is: a focused, bounded primitive.

### 1.2 — Wait for maintainer feedback before proceeding

Do not begin Phase 3 until at least one Slint maintainer has responded. Key
questions to resolve in the Discussion:

- Is `accessible-role: text` acceptable for the message element, or do they
  want the `alert` role added to `AccessibleRole` as part of this PR?
- Is the `public function show()/hide()` pattern accepted, or do maintainers
  prefer a pure property-driven interface?
- Where in the repository does this land? (Third-party listing vs. std-widgets
  vs. a new components examples directory?)
- Any concerns about the `ToastStyle` zero-value contract given that struct
  field defaults are not yet supported in Slint?

Maintainer answers may reshape the API. This is the entire point of Phase 1.

---

## Phase 2 — Repository Setup

### 2.1 — Create the repository structure

```
slint-toast/
├── ui/
│   ├── toast-types.slint
│   ├── toast.slint
│   └── toast-host.slint
├── demo/
│   └── toast-demo.slint
└── README.md
```

### 2.2 — Install tooling

Ensure you have:

- `slint-viewer` for live-previewing `.slint` files without a host app:
  `cargo install slint-viewer`
- The Slint VS Code extension with LSP enabled for inline preview
- `slint-fmt` available (ships with the Slint toolchain — required before
  any PR is accepted)

### 2.3 — Verify `slint-viewer` works

Create a throwaway `test.slint` with a basic `Window` and verify it renders:

```
slint-viewer test.slint
```

Confirm your toolchain before investing in the real files.

### 2.4 — Audit the Palette API before writing any color code

Before writing theming logic, read the current Slint docs for:

- `Palette` — enumerate all available properties. Specifically confirm whether
  `Palette.accent-background`, `Palette.accent-foreground`, `Palette.foreground`,
  and `Palette.color-scheme` exist in the current release.
- `StyleMetrics` — check for spacing/sizing tokens that could inform defaults.

The default color table in the Architecture document assumes these Palette
properties exist. Verify each name against the running version before writing
the fallback logic in Phase 4.2. Do not assume — check.

---

## Phase 3 — `toast-types.slint`

Pure data declarations. No elements, no layout, no visuals. Write this first
because both `toast.slint` and `toast-host.slint` import it.

### 3.1 — Write the enums

```slint
// SPDX-License-Identifier: MIT

export enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

export enum ToastAnchor {
    BottomRight,
    BottomCenter,
    BottomLeft,
    TopRight,
    TopCenter,
    TopLeft,
}
```

Notes:
- `Info` must be first — it is the zero-value default when `kind` is not set.
- Do not add `Custom` or extensible variants. Maintainers will reject them.
- PascalCase for enum values — kebab-case does not apply here.

### 3.2 — Write the `ToastStyle` struct

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

**Do not attempt default values on struct fields.** The syntax
`border-radius: length = 6px` is not supported in current Slint. All fields
initialise to zero. The fallback logic lives in `toast.slint` as private
computed properties. This file is data-only.

### 3.3 — Run `slint-fmt`

```
slint-fmt ui/toast-types.slint
```

Commit only after formatting. Establish this as a non-negotiable habit.

---

## Phase 4 — `toast.slint`

The most complex file. Work through sub-phases in order. Do not proceed to
Phase 5 until this file is visually verified in `slint-viewer`.

### 4.1 — File header and imports

```slint
// SPDX-License-Identifier: MIT
// Toast — visual atom. No timers. No positioning. No business logic.

import { ToastKind, ToastStyle } from "toast-types.slint";
import { Palette } from "std-widgets.slint";
```

### 4.2 — Private Palette fallback properties

This is the solution to the zero-value struct problem. For every brush field
in `ToastStyle`, define a private computed property that reads the incoming
style field and substitutes a default when the brush is transparent (unset).

The detection pattern for brushes uses `.is-transparent()`. For numeric
fields (`length`, `duration`), compare to zero.

**Background resolution pattern:**

```slint
private property <brush> resolved-background: {
    if (root.kind == ToastKind.Info) :
        root.style.background-info.is-transparent()
            ? Palette.accent-background
            : root.style.background-info
    else if (root.kind == ToastKind.Success) :
        root.style.background-success.is-transparent()
            ? (Palette.color-scheme == ColorScheme.Dark ? #388e3c : #2e7d32)
            : root.style.background-success
    else if (root.kind == ToastKind.Warning) :
        root.style.background-warning.is-transparent()
            ? (Palette.color-scheme == ColorScheme.Dark ? #f57c00 : #e65100)
            : root.style.background-warning
    else :
        root.style.background-error.is-transparent()
            ? (Palette.color-scheme == ColorScheme.Dark ? #ef5350 : #c62828)
            : root.style.background-error
};
```

**Repeat this pattern for foreground resolution.** Info uses
`Palette.accent-foreground`. Success/Error use `#ffffff`. Warning uses
`#000000` (dark foreground for WCAG AA contrast on amber).

**Numeric fallbacks:**

```slint
private property <length>   resolved-border-radius:
    root.style.border-radius == 0px  ? 6px   : root.style.border-radius;

private property <length>   resolved-padding:
    root.style.padding == 0px        ? 14px  : root.style.padding;

private property <duration> resolved-fade-in:
    root.style.fade-in-duration == 0ms  ? 180ms : root.style.fade-in-duration;

private property <duration> resolved-fade-out:
    root.style.fade-out-duration == 0ms ? 220ms : root.style.fade-out-duration;

private property <duration> resolved-slide:
    root.style.slide-duration == 0ms    ? 200ms : root.style.slide-duration;
```

Verify `Palette.accent-background`, `Palette.accent-foreground`, and
`Palette.color-scheme` against the live Slint docs before finalising. If any
name is wrong the component will fail to compile.

### 4.3 — Animation state management

**Do not use an `is-showing` boolean lifecycle.** This approach requires
detecting when an animation completes, which Slint does not support cleanly.

Instead, use a composite `visible` binding that keeps the element in the
render tree during the exit animation:

```slint
private property <float> display-opacity: 0.0;
private property <length> slide-offset: 40px;
```

The element's Slint `visible` property is bound as:

```slint
visible: root.visible || root.display-opacity > 0.0;
```

This means:
- When `root.visible` becomes `true` → element appears, animations play in
- When `root.visible` becomes `false` → `display-opacity` animates to `0.0`;
  element stays present until `display-opacity` reaches zero, at which point
  the binding resolves to `false` and the element is removed
- No lifecycle management required
- No width/height mutation — do not set `width: 0` or `height: 0` to suppress
  layout. Opacity handles visibility; layout stability is preserved.

**States and transitions:**

```slint
states [
    shown when root.visible : {
        display-opacity: 1.0;
        slide-offset: 0px;
        in {
            animate display-opacity {
                duration: root.resolved-fade-in;
                easing: ease-out;
            }
            animate slide-offset {
                duration: root.resolved-slide;
                easing: ease-out;
            }
        }
    }
    hidden when !root.visible : {
        display-opacity: 0.0;
        slide-offset: 40px;
        out {
            animate display-opacity {
                duration: root.resolved-fade-out;
                easing: ease-in;
            }
            animate slide-offset {
                duration: root.resolved-slide;
                easing: ease-in;
            }
        }
    }
]
```

### 4.4 — Root element and layout

`Toast` inherits `Rectangle`. Size is content-driven — use preferred sizing.

```slint
export component Toast inherits Rectangle {
    // === Public API ===
    in property <string>     text;
    in property <ToastKind>  kind;
    in property <bool>       visible: false;
    in property <bool>       enabled: true;
    in property <bool>       show-close: true;
    in property <string>     action-label;
    in property <image>      icon;
    in property <ToastStyle> style;

    callback closed();
    callback action();

    // === Private resolved properties (Phases 4.2 and 4.3) ===
    // ... all private properties here ...

    // === Root visual bindings ===
    background:    root.resolved-background;
    border-radius: root.resolved-border-radius;
    opacity:       root.display-opacity;

    // Composite visible — keeps element in tree during exit animation
    visible: root.visible || root.display-opacity > 0.0;

    // y-offset for slide animation — direction depends on anchor context;
    // default slides up from below
    y: root.slide-offset;
```

### 4.5 — Interior layout

```slint
    HorizontalLayout {
        padding: root.resolved-padding;
        spacing: 8px;
        alignment: center;

        // Icon — Slint has no null image sentinel; detect via dimensions
        // This is the idiomatic Slint pattern for optional images.
        if root.icon.width > 0 && root.icon.height > 0 : Image {
            source: root.icon;
            width: 20px;
            height: 20px;
            vertical-alignment: center;
        }

        // Message text
        Text {
            text: root.text;
            color: root.resolved-foreground;
            vertical-alignment: center;
            horizontal-stretch: 1;
            wrap: word-wrap;

            accessible-role: text;
            accessible-label: root.text;
        }

        // Action button — only when action-label is non-empty
        if root.action-label != "" : TouchArea {
            enabled: root.enabled;
            clicked => { root.action(); }

            accessible-role: button;
            accessible-label: root.action-label;

            Text {
                text: root.action-label;
                color: root.resolved-foreground;
                vertical-alignment: center;
            }
        }

        // Close button — only when show-close is true
        if root.show-close : TouchArea {
            enabled: root.enabled;
            clicked => { root.closed(); }

            accessible-role: button;
            accessible-label: "Close";

            Text {
                text: "✕";
                color: root.resolved-foreground;
                vertical-alignment: center;
            }
        }
    }
}
```

### 4.6 — Verify in `slint-viewer`

Create a temporary `test-toast.slint`:

```slint
import { Toast, ToastKind } from "ui/toast.slint";

export component TestToast inherits Window {
    width: 600px; height: 200px;
    Toast {
        kind: ToastKind.Success;
        text: "File saved successfully.";
        visible: true;
        show-close: true;
        action-label: "Undo";
    }
}
```

```
slint-viewer test-toast.slint
```

Verify before proceeding:
- Toast renders with correct background for `Success`
- Text is visible with correct foreground color
- Action button and close button are present
- All four `kind` values produce distinct colors
- `visible: false` hides the toast with animation (not an abrupt disappearance)
- An empty `ToastStyle {}` does not produce an invisible toast

### 4.7 — Run `slint-fmt`

```
slint-fmt ui/toast.slint
```

---

## Phase 5 — `toast-host.slint`

### 5.1 — File header and imports

```slint
// SPDX-License-Identifier: MIT
// ToastHost — positioning container. Command-driven. No public visible property.
// Must be the last direct child of the root Window in the host application.

import { ToastKind, ToastAnchor, ToastStyle } from "ui/toast-types.slint";
import { Toast } from "ui/toast.slint";
```

### 5.2 — Component declaration

`ToastHost` inherits `Rectangle` and fills the parent window entirely. It is
transparent — it exists only to position the internal `Toast` at the correct
anchor point. It must be a direct child of `Window`, not nested in any layout.

```slint
export component ToastHost inherits Rectangle {
    width: 100%;
    height: 100%;
    background: transparent;

    // === Public API — no visible property ===
    in property <string>      text: "";
    in property <ToastKind>   kind: ToastKind.Info;
    in property <bool>        enabled: true;
    in property <bool>        show-close: true;
    in property <string>      action-label: "";
    in property <image>       icon;
    in property <ToastAnchor> anchor: ToastAnchor.BottomRight;
    in property <ToastStyle>  style;

    callback toast-closed();
    callback toast-action();

    // === Internal state — sole source of truth for visibility ===
    private property <bool>      active-visible: false;
    private property <string>    active-text: "";
    private property <ToastKind> active-kind: ToastKind.Info;

    public function show(t: string, k: ToastKind) {
        active-text    = t;
        active-kind    = k;
        active-visible = true;
    }

    public function hide() {
        active-visible = false;
    }
```

### 5.3 — Anchor positioning and internal Toast

Property forwarding uses one-way bindings (`toast.text: root.active-text`),
not two-way bindings (`<=>`). Two-way bindings across component boundaries
can trigger alias optimisation issues in the Slint compiler. One-way is
safer and is the idiomatic pattern for pass-through properties.

```slint
    private property <length> edge-margin: 16px;

    toast := Toast {
        // Forward all state via one-way bindings
        text:         root.active-text;
        kind:         root.active-kind;
        visible:      root.active-visible;
        enabled:      root.enabled;
        show-close:   root.show-close;
        action-label: root.action-label;
        icon:         root.icon;
        style:        root.style;

        // Forward callbacks outward
        closed => { root.toast-closed(); }
        action => { root.toast-action(); }

        // Anchor x position
        x: root.anchor == ToastAnchor.BottomLeft || root.anchor == ToastAnchor.TopLeft
            ? root.edge-margin
            : root.anchor == ToastAnchor.BottomCenter || root.anchor == ToastAnchor.TopCenter
                ? (root.width - self.width) / 2
                : root.width - self.width - root.edge-margin;

        // Anchor y position
        y: root.anchor == ToastAnchor.TopLeft
            || root.anchor == ToastAnchor.TopCenter
            || root.anchor == ToastAnchor.TopRight
            ? root.edge-margin
            : root.height - self.height - root.edge-margin;
    }
}
```

### 5.4 — Verify in `slint-viewer`

Create `test-host.slint`:

```slint
import { ToastHost, ToastKind, ToastAnchor } from "ui/toast-host.slint";

export component TestHost inherits Window {
    width: 800px; height: 600px;
    background: #1e1e2e;

    toast-host := ToastHost {
        anchor: ToastAnchor.BottomRight;
    }

    init => {
        toast-host.show("Export complete.", ToastKind.Success);
    }
}
```

Test all six anchor positions. Verify:
- Toast appears at the correct corner/edge in each case
- `show()` and `hide()` work
- Animations play correctly
- No other elements are displaced (transparency confirmed)
- Toast is not constrained by the host's geometry (fills window)

### 5.5 — Run `slint-fmt`

```
slint-fmt ui/toast-host.slint
```

---

## Phase 6 — Demo, README, and Submission

### 6.1 — Write `demo/toast-demo.slint`

Must be fully self-contained and runnable with no backend:

```
slint-viewer demo/toast-demo.slint
```

All interactivity via Slint `Button`, `ComboBox`, and internal state.
Structure as a scrollable vertical layout with labeled sections:

**Section 1 — All four kinds**
Four `Toast` instances with `visible: true`, each a different `kind`.
Primary visual reference for reviewers.

**Section 2 — Interactive dismiss**
A `ToastHost` with a `Button` that calls `show()`. Clicking the toast's
close button animates it out. A second button re-triggers it.

**Section 3 — Action button**
`action-label: "Undo"`. A `Text` label below reads "Action triggered"
when the callback fires, driven by a private `bool` state property.

**Section 4 — No close button**
`show-close: false`. Toast visible with no dismiss control.

**Section 5 — Disabled state**
`enabled: false`. Buttons visible but non-interactive.

**Section 6 — Anchor selector**
A `ComboBox` listing all six `ToastAnchor` values. A `ToastHost` whose
`anchor` is bound to the selected value. A `Button` to trigger a toast.

**Section 7 — Custom style override**
A hand-constructed `ToastStyle` with unusual colors demonstrating the
override system end to end.

Window root: `title: "Slint Toast Demo"`, approximately `900px × 700px`,
neutral background.

### 6.2 — Write `README.md`

All sections are required. None are optional.

**Header** — component name, one-sentence description, Slint version.

**Installation / Import** — import path (adjust to wherever maintainers place
the file after the Discussion).

**Quick Start** — minimal working example: `ToastHost` as last child of
`Window`, one `Button` calling `toast-host.show()`.

**API Reference** — full property, function, and callback tables for both
`Toast` and `ToastHost`. Taken directly from the API specification document.

**Theming** — how to construct a `ToastStyle`. Full field table. Include
the zero-value contract explicitly: *"Zero values for `ToastStyle` fields are
treated as unset and will be replaced by component defaults."*

**Z-Order Rule** — prominent warning with code example. State both the
last-child rule and the layout nesting prohibition.

**Host Responsibilities** — bulleted list covering: timer, queue, calling
`show()`/`hide()`, reacting to callbacks, and screen reader announcement if
required.

**Accessibility** — state that `accessible-role` and `accessible-label` are
provided internally. Include both limitations explicitly:
- *"The close button accessible label is the static English string 'Close'.
  Localisation is not supported."*
- *"Because Slint does not provide an `alert` accessible role, this component
  cannot guarantee proactive screen reader announcement. Applications requiring
  guaranteed announcement should implement supplemental logic at the
  application level."*

**Known Limitations** — close button i18n, screen reader announcement,
last-child placement, layout nesting prohibition, zero-value contract edge case
(cannot request zero padding or instant animations via `ToastStyle`).

**Contributing** — CLA requirement, GitHub Discussion before PR, `slint-fmt`
mandatory.

**License** — MIT.

### 6.3 — Final `slint-fmt` pass

```
slint-fmt ui/toast-types.slint
slint-fmt ui/toast.slint
slint-fmt ui/toast-host.slint
slint-fmt demo/toast-demo.slint
```

Commit only after all files produce no diff. A formatting failure in CI is an
avoidable distraction during review.

### 6.4 — Pre-submission checklist

Every item must pass before the PR is opened.

**Functionality**
- [ ] All four `ToastKind` variants render with distinct, accessible colors
- [ ] `visible: true` plays fade-in + slide-in animation
- [ ] `visible: false` plays fade-out + slide-out fully before element is removed
- [ ] `show-close: false` hides close button; `hide()` still works
- [ ] `action-label: ""` renders no action button
- [ ] `action-label: "Undo"` renders action button; `action()` fires on click
- [ ] `enabled: false` renders buttons non-interactive; no click events fire
- [ ] `icon` with a valid image renders; unset `icon` renders nothing
- [ ] All six `ToastAnchor` positions place the toast correctly
- [ ] `ToastStyle {}` (all zeroes) does not produce an invisible toast
- [ ] `ToastStyle` with all fields set overrides all visual defaults
- [ ] `show()` and `hide()` are the only control surface on `ToastHost`
- [ ] There is no public `visible` property on `ToastHost`

**Animation**
- [ ] Exit animation completes fully before element disappears
- [ ] No width/height mutation used — opacity-only approach confirmed
- [ ] No `is-showing` boolean or lifecycle tracking used
- [ ] No `Timer` used anywhere in any `.slint` file

**Accessibility**
- [ ] Message text has `accessible-role: text` and `accessible-label` bound
      to `text`
- [ ] Close button has `accessible-role: button` and `accessible-label: "Close"`
- [ ] Action button has `accessible-role: button` and `accessible-label`
      bound to `action-label`
- [ ] `enabled: false` removes buttons from keyboard focus traversal

**Theming**
- [ ] Component renders correctly with Fluent style
- [ ] Component renders correctly with Cosmic style
- [ ] Component renders correctly with Material style (if available)
- [ ] Palette light mode: all four kinds readable
- [ ] Palette dark mode: all four kinds readable
- [ ] WCAG AA contrast confirmed for Warning kind (dark text on amber)
- [ ] `Info` kind uses `Palette.accent-background` / `Palette.accent-foreground`

**Code quality**
- [ ] No `Timer` used anywhere
- [ ] No business logic anywhere
- [ ] No Rust/C++/JS-specific types anywhere
- [ ] All files pass `slint-fmt` with no diff
- [ ] All public properties use correct direction (`in`)
- [ ] All callbacks are outbound only
- [ ] Property forwarding uses one-way bindings throughout, not `<=>`
- [ ] SPDX license header on every `.slint` file
- [ ] No hardcoded magic numbers — all sizing through `resolved-*` properties

**Demo**
- [ ] `slint-viewer demo/toast-demo.slint` launches with no errors
- [ ] All seven demo sections visible and functional without a host backend
- [ ] Anchor selector correctly repositions the toast
- [ ] Custom style section demonstrates override system

**Documentation**
- [ ] README covers all required sections
- [ ] Z-order rule and layout nesting prohibition documented with code example
- [ ] Zero-value contract documented explicitly
- [ ] Screen reader limitation documented
- [ ] Close button i18n limitation documented
- [ ] Host responsibilities documented including screen reader note
- [ ] CLA requirement mentioned in Contributing section

### 6.5 — Open the Pull Request

Title format (verify conventions in Discussion):
> `feat(widgets): Add Toast / Snackbar notification component`

PR body must include:

1. **Summary** — one paragraph: what it does, why it belongs upstream.
2. **API surface** — link to the Discussion where it was pre-approved.
3. **Screenshots** — all four kinds, both light and dark themes, animated
   dismiss, at least two anchor positions.
4. **Checklist confirmation** — state that all items above have been verified.
5. **Known limitations** — list explicitly: close button i18n, screen reader
   announcement, last-child placement, layout nesting prohibition, zero-value
   contract edge case.
6. **Non-goals** — reference the Non-Goals section from the Architecture
   document. Scope is intentionally bounded.

Respond to each review comment with a commit. Do not squash during review.
Squash at final approval per Slint's documented PR process.

---

## Implementation Risks and Mitigations

| Risk | Likelihood | Mitigation |
|---|---|---|
| Maintainers want a different API shape | Medium | Phase 1 Discussion resolves this before any code |
| Palette property names differ from assumed names | Medium | Audit in Phase 2.4 before writing any color logic |
| Exit animation timing — element disappears early | Low (fixed) | Composite visible binding: `visible \|\| opacity > 0.0` |
| `ToastStyle` zero brushes produce invisible toast | Low (fixed) | `.is-transparent()` fallback on every brush property |
| `ToastHost` constrained by parent layout | Medium | Document prohibition in README; cannot fix in component |
| Property forwarding causes alias optimisation bug | Low | One-way bindings throughout Phase 5.3 |
| `accessible-role: alert` requested by maintainers | Low | Flag in Discussion; offer to add if Slint adds to enum |
| Demo too complex for `slint-viewer` | Low | Test `slint-viewer` compatibility early in Phase 6.1 |
| Dual visible / active-visible ambiguity | Eliminated | `ToastHost` has no public `visible` — command-driven only |
| Width/height collapse causing layout reflow | Eliminated | Opacity-only animation approach; no dimension mutation |

---

## Delivery Artifacts

| File | Purpose |
|---|---|
| `ui/toast-types.slint` | Enums and style struct — the data layer |
| `ui/toast.slint` | The visual atom — core component |
| `ui/toast-host.slint` | The positioning container — command-driven |
| `demo/toast-demo.slint` | Self-contained demo for reviewer verification |
| `README.md` | Integration guide and API reference |
| GitHub Discussion | Pre-approval record and maintainer alignment |
| Pull Request | The upstream submission |

---

## Revision History

### v3 (this revision) — post peer review

- **Removed** `is-showing` boolean from Phase 4.3. Replaced with
  opacity-binding pattern: `visible: root.visible || display-opacity > 0.0`.
- **Removed** width/height collapse from Phase 4.4. Opacity-only approach.
- **Removed** public `visible` from `ToastHost` throughout Phase 5.
  `ToastHost` is now command-driven only via `show()`/`hide()`.
- **Replaced** placeholder color language ("verify exact property name") with
  the concrete default color table from the Architecture document.
- **Added** zero-value contract as a named, explicit contract in Phase 3.2.
- **Added** screen reader announcement limitation to Phase 6.2 README
  requirements and Phase 6.4 checklist.
- **Added** icon detection note in Phase 4.5 naming it as idiomatic Slint
  constraint.
- **Updated** risks table to reflect resolved and eliminated risks.

### v2 — post gap analysis

Added: CLA, `slint-viewer` setup, `Palette` audit step, accessibility
verification, `slint-fmt` discipline, full pre-submission checklist.

### v1 — initial

Six-phase structure, toolchain setup, file-by-file implementation guidance.
