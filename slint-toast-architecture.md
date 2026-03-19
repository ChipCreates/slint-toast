# Slint Toast Component — Architecture

> Upstream contribution proposal for the Slint component library.
> Pure UI. Language-agnostic. No timers. No business logic.

---

## Design Philosophy

This component is intentionally a **pure UI primitive**. It makes no assumptions
about the host language (Rust, C++, JavaScript, Python), the application's timer
mechanism, or any queuing strategy. All behavioral orchestration is the host
application's responsibility.

The component's only job is to render correctly, respond to Palette-driven
theming, expose correct accessibility properties, and fire the right callbacks.

---

## Submission Pathway

Before opening a pull request, open a **GitHub Discussion** in the Slint
repository to validate maintainer interest. Slint maintainers prefer to
discuss new component proposals before implementation work begins.

When the PR is opened:

- All contributions must be written entirely by the contributor with no
  third-party rights involved.
- Contributors will be asked to sign a **Contributor License Agreement (CLA)**
  during the PR process. This is required and non-negotiable.
- All contributions are accepted under the **MIT No Attribution License**.
- PRs are integrated as "rebase and merge" or "squash and merge". Linear
  history is expected. Fixup commits should be squashed before final approval.

---

## File Structure

```
slint-toast/
├── ui/
│   ├── toast-types.slint       # Enums and style struct — no visual elements
│   ├── toast.slint             # Toast — the visual atom
│   └── toast-host.slint        # ToastHost — positioning container
├── demo/
│   └── toast-demo.slint        # Self-contained live-preview demo
└── README.md
```

No Rust, C++, or any other language files belong in this component.
The `ui/` directory is the deliverable. The `demo/` directory is required
for reviewer verification and live preview tooling.

---

## File Responsibilities

### `toast-types.slint`

Declares all shared types with no visual elements whatsoever:

- `ToastKind` enum — `Info`, `Success`, `Warning`, `Error`
- `ToastAnchor` enum — six positional values (`BottomRight`, `BottomCenter`, etc.)
- `ToastStyle` struct — complete visual configuration (per-kind colors, radii,
  padding, animation durations)

Everything in this file is a pure data declaration. It is imported by both
`toast.slint` and `toast-host.slint`. Host applications may also import it
directly to construct `ToastStyle` values.

---

### `toast.slint`

The visual atom. Renders a single toast notification. Has no knowledge of
positioning, stacking, timers, or the host application.

**Imports:** `toast-types.slint`, `std-widgets.slint` (for `Palette`)

**Properties (all `in`):**

| Property | Type | Default | Notes |
|---|---|---|---|
| `text` | `string` | `""` | The notification message |
| `kind` | `ToastKind` | `Info` | Controls color resolution |
| `visible` | `bool` | `false` | Drives show/hide state and animations |
| `enabled` | `bool` | `true` | When false, buttons are non-interactive |
| `show-close` | `bool` | `true` | Whether the close button is rendered |
| `action-label` | `string` | `""` | Empty string = no action button rendered |
| `icon` | `image` | — | Optional. Detected via `icon.width > 0` |
| `style` | `ToastStyle` | — | Visual override. Zero fields use defaults |

**Callbacks (outbound only):**

| Callback | Fired when |
|---|---|
| `closed()` | User clicks the close button |
| `action()` | User clicks the action button |

**Animation behaviour:**

The component uses a private `display-opacity` float property for fading.
The element's `visible` binding is:

```slint
visible: root.visible || root.display-opacity > 0.0;
```

This guarantees the exit animation completes fully before the element is
removed from the render tree. No width/height mutation is used. No `Timer`
is used anywhere in this file.

---

### `toast-host.slint`

The positioning container. Anchors the toast overlay within the window and
owns the single `Toast` instance. Exposes a command-driven interface to the
host application.

**`ToastHost` is command-driven — there is no public `visible` property.**
Visibility is owned entirely by the internal `active-visible` boolean, which
is written only by `show()` and `hide()`. This eliminates the dual-source-of-
truth ambiguity that would result from a host being able to set both a public
`visible` property and call `show()`/`hide()` simultaneously.

**Imports:** `toast-types.slint`, `toast.slint`

**Properties (all `in`):**

| Property | Type | Default | Notes |
|---|---|---|---|
| `text` | `string` | `""` | Forwarded to internal `Toast.text` |
| `kind` | `ToastKind` | `Info` | Forwarded to internal `Toast.kind` |
| `enabled` | `bool` | `true` | Forwarded to internal `Toast.enabled` |
| `show-close` | `bool` | `true` | Forwarded to internal `Toast.show-close` |
| `action-label` | `string` | `""` | Forwarded to internal `Toast.action-label` |
| `icon` | `image` | — | Forwarded to internal `Toast.icon` |
| `anchor` | `ToastAnchor` | `BottomRight` | Controls overlay position |
| `style` | `ToastStyle` | — | Forwarded to internal `Toast.style` |

**Internal state (not public):**

| Property | Type | Purpose |
|---|---|---|
| `active-visible` | `bool` | Sole source of truth for toast visibility |
| `active-text` | `string` | Written by `show()`, forwarded to `Toast.text` |
| `active-kind` | `ToastKind` | Written by `show()`, forwarded to `Toast.kind` |

**Functions (command input — called by host app):**

| Function | Signature | Behaviour |
|---|---|---|
| `show` | `(text: string, kind: ToastKind)` | Writes `active-text`, `active-kind`, sets `active-visible = true` |
| `hide` | `()` | Sets `active-visible = false` |

**Callbacks (outbound only — fired by component):**

| Callback | Fired when |
|---|---|
| `toast-closed()` | Internal `Toast.closed` fires |
| `toast-action()` | Internal `Toast.action` fires |

---

### `demo/toast-demo.slint`

A standalone demo compatible with `slint-viewer`. Required for reviewer
verification. No backend or host language code — all interactivity via
Slint `Button` and state. Must cover:

- All four `ToastKind` variants rendered simultaneously
- Interactive dismiss with animation via `show()`/`hide()`
- Toast with action button; label showing when action fires
- Toast with `show-close: false`
- Toast with `enabled: false`
- `ToastAnchor` selector across all six positions
- Custom `ToastStyle` override demonstrating the full theming system

---

## Accessibility

Accessibility is a hard requirement for upstream inclusion.

### `Toast` accessibility contract

| Element | `accessible-role` | `accessible-label` |
|---|---|---|
| Message text | `text` | bound to `text` property |
| Close button | `button` | `"Close"` (static English string) |
| Action button | `button` | bound to `action-label` property |

When `enabled = false`, close and action buttons must not be reachable via
keyboard navigation.

### Screen reader announcement limitation

Because Slint's `AccessibleRole` enum does not include an `alert` role,
this component cannot guarantee that screen readers will proactively announce
toast notifications when they appear. The `text` role is used for the message
element as the closest available option. If `alert` is added to Slint's
`AccessibleRole` in a future release, this component should be updated
accordingly.

This limitation must be documented in the README. Host applications requiring
guaranteed screen reader announcement should implement supplemental
announcement logic at the application level.

### i18n limitation

The close button `accessible-label` is the static English string `"Close"`.
Full localisation is out of scope for this component and is deferred to a
future enhancement. This must be documented in the README.

---

## Theming Contract

### Palette integration — hybrid strategy

Slint's `Palette` does not provide semantic colors for all four toast kinds.
The component uses a hybrid fallback strategy:

- Where `Palette` has a direct semantic equivalent, use it.
- Where `Palette` has no equivalent, use defined accessible fallback values.

This strategy is named and documented here explicitly so it is not discovered
as an ambiguity during code review.

### Default color table

These values apply when the corresponding `ToastStyle` brush field is
transparent (unset). All values are WCAG AA compliant.

| Kind | Palette source | Light fallback bg | Dark fallback bg | Foreground |
|---|---|---|---|---|
| `Info` | `Palette.accent-background` | — | — | `Palette.accent-foreground` |
| `Success` | none | `#2e7d32` | `#388e3c` | `#ffffff` |
| `Warning` | none | `#e65100` | `#f57c00` | `#000000` (WCAG AA on amber) |
| `Error` | none | `#c62828` | `#ef5350` | `#ffffff` |

`Info` uses `Palette.accent-background` and `Palette.accent-foreground`
directly, making it automatically responsive to light/dark mode and all
Slint styles. The remaining three kinds use hardcoded accessible fallbacks
because no Slint Palette equivalent exists. If Slint adds semantic palette
entries for success/warning/error in a future release, the fallbacks should
be updated to use them.

### Zero-value contract

Slint does not support struct field default values. All `ToastStyle` fields
initialise to zero (`brush` → transparent, `length` → `0px`,
`duration` → `0ms`). **Zero values are treated as unset** by the component's
private fallback properties. This is the only viable approach in current Slint
and is a documented contract, not an implementation detail.

Consequence: a host cannot use `ToastStyle` to request zero padding or
instant animations. These edge cases are out of scope.

### Animation defaults

Applied when the corresponding `ToastStyle` duration field is `0ms` (unset):

| Field | Default |
|---|---|
| `fade-in-duration` | `180ms` |
| `fade-out-duration` | `220ms` |
| `slide-duration` | `200ms` |

### Shape defaults

Applied when the corresponding `ToastStyle` length field is `0px` (unset):

| Field | Default |
|---|---|
| `border-radius` | `6px` |
| `padding` | `14px` |

---

## Animation Contract

The component owns the animation mechanics. The host controls only timing
via `ToastStyle` duration fields.

The key implementation detail is that the element's Slint `visible` property
is driven by a composite binding:

```slint
visible: root.visible || root.display-opacity > 0.0;
```

This means:

- When `visible` flips to `true` → element becomes visible, fade-in plays
- When `visible` flips to `false` → fade-out plays; element remains in tree
  until `display-opacity` reaches `0.0`, at which point it is removed
- No `is-showing` lifecycle boolean is needed
- No `Timer` is used
- Width/height are not mutated to suppress layout — opacity handles visibility

The component does **not** fire any callback when animation completes. If the
host needs to sequence the next toast after the exit animation, it must budget
for `style.fade-out-duration` (or the `220ms` default) in its own timer.

---

## Z-Order and Overlay Placement

Slint renders children in declaration order. `ToastHost` does not manipulate
its own z-order — the host application is responsible for correct placement.

**Rule: `ToastHost` must be the last direct child of the root `Window`.**

It must not be nested inside any layout element (`HorizontalLayout`,
`VerticalLayout`, `GridLayout`, etc.). Layout elements constrain their children
to their bounds, which prevents the overlay from floating above other content.

```slint
export component AppWindow inherits Window {
    // ... all other UI — layouts, panels, content ...

    // ToastHost MUST be declared last and MUST be a direct child of Window
    toast-host := ToastHost {
        anchor: ToastAnchor.BottomRight;
    }
}
```

This constraint must be documented prominently in the README.

---

## Responsibility Boundary

| Concern | Owner |
|---|---|
| Rendering a toast | `Toast` component |
| Positioning the overlay | `ToastHost` component |
| Accessibility properties | `Toast` component (internal) |
| Palette-derived and fallback default colors | `Toast` component (internal) |
| Firing `closed` / `action` events | `Toast` component |
| Animation mechanics and exit persistence | `Toast` component (internal) |
| Z-order via last-child placement | **Host application** |
| Auto-dismiss timer | **Host application** |
| Toast queuing / sequencing | **Host application** |
| Priority / interruption logic | **Host application** |
| Multiple simultaneous toasts | **Host application** |
| Calling `show()` / `hide()` | **Host application** |
| Reacting to `toast-closed()` | **Host application** |
| Reacting to `toast-action()` | **Host application** |
| Screen reader announcement (if required) | **Host application** |

---

## What the Host Application Must Implement

To use this component correctly, a host application must:

1. **Place `ToastHost` as the last direct child** of the root `Window`
   (not nested in any layout)
2. **Own a timer** — on expiry, call `toast-host.hide()`
3. **Own a queue** — on `toast-closed()` or timer expiry, pop the next entry
   and call `toast-host.show(text, kind)`
4. **Set properties** (`action-label`, `icon`, `style`) before calling `show()`
5. **React to `toast-action()`** with application-specific logic
6. **Implement supplemental screen reader announcement** if the application
   requires guaranteed assistive technology notification

A minimal host implementation requires approximately 20–30 lines in any
supported language.

---

## Non-Goals

The following are explicitly out of scope and will not be added:

- Built-in `Timer` or auto-dismiss
- Toast queue or stack management
- Multi-toast simultaneous display
- Persistence or history
- OS-level notifications
- Progress indicators within a toast
- i18n for the close button label
- Guaranteed screen reader announcement
- Any Rust-, C++-, or JS-specific types or callbacks

---

## Compatibility

- Requires Slint 1.x
- No external dependencies
- Compatible with all Slint-supported host languages (Rust, C++, JavaScript, Python)
- `Info` kind adapts to Fluent, Cosmic, and Material styles via `Palette`
- `Success`, `Warning`, `Error` use defined accessible fallback colors
- Full `ToastStyle` override available for all kinds in all themes

---

## README Requirements

The component README must document:

- Import path and basic usage example
- All public properties and callbacks for both `Toast` and `ToastHost`
- The z-order / last-child-of-Window rule (with code example)
- The layout nesting prohibition
- The host responsibility list (timer, queue, screen reader)
- The zero-value contract for `ToastStyle`
- The screen reader announcement limitation
- The close button i18n limitation
- CLA requirement for contributors

---

## Revision History

### v3 (this revision) — post peer review

- **Removed** `ToastHost.visible` public property. `ToastHost` is now
  command-driven only. Single source of truth: `active-visible`.
- **Replaced** `is-showing` lifecycle boolean with opacity-binding animation
  persistence pattern.
- **Named** the Palette fallback strategy "hybrid" and formalised it with a
  complete default color table including light/dark values.
- **Named** zero-value = unset as an explicit documented contract.
- **Added** screen reader announcement limitation to accessibility section
  and responsibility boundary table.
- **Added** layout nesting prohibition to z-order section.
- **Removed** width/height collapse animation approach throughout.
- **Added** icon detection note referencing `icon.width > 0` as idiomatic
  Slint constraint.

### v2 — post gap analysis

Added: `enabled`, full accessibility spec, `Palette` integration, `ToastAnchor`,
`ToastStyle`, z-order guidance, `public function` pattern, CLA notes.

### v1 — initial

Original three-file architecture from maintainer-approved design session.
