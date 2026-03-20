# Changelog

All notable changes to this project are documented here.

---

## v3 — Current

Post peer review. API stabilized for upstream submission.

### Breaking changes
- `ToastHost.visible` property removed. `ToastHost` is now command-driven exclusively via `show()` and `hide()`. Any code setting `visible` directly must be updated to call `show()`/`hide()`.

### Changes
- **`ToastHost`** — removed public `visible` property. Sole source of truth is internal `active-visible`, written only by `show()` and `hide()`. Eliminates dual-source-of-truth ambiguity.
- **Animation** — replaced `is-showing` lifecycle boolean with opacity-binding persistence pattern: `visible: root.show || root.display-opacity > 0.0`. Exit animation now guaranteed to complete before element is removed from the render tree. No `Timer` required.
- **Animation** — removed width/height collapse approach. Opacity-only animation preserves layout stability.
- **`ToastStyle`** — zero-value contract named and documented explicitly. Zero = unset is a defined contract, not implicit behavior.
- **Theming** — Palette fallback strategy formalized as "hybrid": `Palette` where a direct semantic mapping exists, defined accessible fallback colors where it does not. Full default color table documented.
- **Accessibility** — screen reader announcement limitation documented. No `alert` role in Slint's `AccessibleRole` enum means proactive announcement cannot be guaranteed by this component.
- **Icon detection** — `icon.width > 0 && icon.height > 0` pattern documented as idiomatic Slint constraint, not a design choice.
- **Property rename** — `visible` input property on `Toast` renamed to `show` to avoid conflict with Slint's built-in `visible` property on all elements.
- **Color check** — replaced `.is-transparent()` (not available on `brush`) with `== transparent` comparison.
- **Enum correction** — `ColorScheme.Dark` corrected to `ColorScheme.dark`.
- **Accessibility** — `accessible-role: text` corrected to `accessible-role: AccessibleRole.text` to resolve enum ambiguity.
- **Close button glyph** — changed from `✕` (U+2715, not in all default fonts) to `×` (U+00D7, universally available).
- **`ToastHost` sizing** — toast instance now constrained to `min(360px, available-width)` with content-driven height via `self.preferred-height`. Previously defaulted to 100% × 100% of the overlay, filling the entire window.

---

## v2 — Post gap analysis

### Added
- `enabled` property on `Toast` — when false, close and action buttons are non-interactive and removed from keyboard focus traversal
- Full accessibility specification: `accessible-role` and `accessible-label` on message text, close button, and action button
- `Palette` integration — `Info` kind uses `Palette.accent-background` / `Palette.accent-foreground`
- `ToastAnchor` enum with six positional values
- `ToastStyle` struct with per-kind colors, border-radius, padding, and animation durations
- Z-order guidance — `ToastHost` must be last direct child of root `Window`
- `public function show() / hide()` command pattern on `ToastHost`
- CLA requirement documented

---

## v1 — Initial

Original three-file architecture from maintainer-approved design session.

### Established
- `toast_types.slint` — shared enums and style struct
- `toast.slint` — visual atom, no timers, no positioning
- `toast_host.slint` — overlay container with anchor positioning
- `demo/toast-demo.slint` — self-contained `slint-viewer` demo
