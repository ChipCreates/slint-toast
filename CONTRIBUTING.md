# Contributing

Thank you for your interest in contributing to this component.

This component is intended for upstream submission to the [Slint](https://slint.dev) component library. Please read this document before opening a pull request.

---

## Before You Start

### Open a GitHub Discussion first

Do not open a pull request without prior discussion. Open a **GitHub Discussion** in the [Slint repository](https://github.com/slint-ui/slint/discussions) to validate maintainer interest and confirm the API shape before writing any code. Slint maintainers prefer to review proposals before implementation work begins.

Key questions to resolve in the Discussion:
- Is the proposed API shape acceptable for upstream inclusion?
- Is `accessible-role: text` acceptable for the message element, or should `alert` be added to `AccessibleRole` as part of this PR?
- Where in the repository does this component land?
- Any concerns about the `ToastStyle` zero-value contract?

### Contributor License Agreement

All contributors must sign the Slint **Contributor License Agreement (CLA)** during the PR process. This is required and non-negotiable. All contributions are accepted under the **MIT No Attribution License**.

---

## Scope

This component is intentionally a **pure UI primitive**. The following are explicitly out of scope and will not be accepted:

- Built-in `Timer` or auto-dismiss logic
- Toast queue or stack management
- Multi-toast simultaneous display
- Persistence, history, or OS-level notifications
- Progress indicators within a toast
- i18n for the close button label
- Guaranteed screen reader announcement
- Any Rust-, C++-, JS-, or Python-specific types, callbacks, or files

If your contribution adds any of the above, it will not be merged.

---

## What Belongs in This Repo

```
ui/
    toast_types.slint   — enums and ToastStyle struct only, no visual elements
    toast.slint         — visual atom, no timers, no positioning
    toast_host.slint    — overlay container, command-driven
demo/
    toast-demo.slint    — self-contained slint-viewer demo, no backend
README.md
CHANGELOG.md
CONTRIBUTING.md
LICENSE
.gitattributes
```

No Rust, C++, JavaScript, Python, build scripts, or binary files belong here.

---

## Development Setup

### Required tools

- **Slint VS Code extension** — includes live preview via `Slint: Show Preview` (no separate install needed)
- **`slint-viewer`** — for command-line verification: `cargo install slint-viewer`
- **`slint-fmt`** — ships with the Slint toolchain; required before any PR

### Verify your toolchain

```
slint-viewer demo/toast-demo.slint
```

If this opens the demo with no errors, your environment is ready.

---

## Making Changes

### Code style

- All `.slint` files must pass `slint-fmt` with no diff before committing
- All public properties use the `in` direction
- All callbacks are outbound only (no `in-out` callbacks)
- Property forwarding across component boundaries uses one-way bindings, not `<=>`
- No hardcoded magic numbers — all sizing goes through `resolved-*` private properties
- Every `.slint` file must have an SPDX license header: `// SPDX-License-Identifier: MIT`

### Running the formatter

```
slint-fmt ui/toast_types.slint
slint-fmt ui/toast.slint
slint-fmt ui/toast_host.slint
slint-fmt demo/toast-demo.slint
```

Commit only after all files format cleanly.

### Pre-submission checklist

Before opening a PR, verify every item in the checklist in `docs/slint-toast-implementation-plan.md` §6.4. This covers functionality, animation, accessibility, theming, code quality, demo, and documentation.

---

## Pull Request Guidelines

- **One PR per concern** — do not bundle unrelated changes
- **No fixup commits during review** — respond to each review comment with a commit; squash only at final approval per Slint's documented PR process
- **Linear history** — rebase onto `main` before opening the PR; no merge commits
- **PR title format**: `feat(widgets): Add Toast / Snackbar notification component`

The PR body must include:
1. Summary — what it does and why it belongs upstream
2. Link to the GitHub Discussion where the API was pre-approved
3. Screenshots — all four kinds in light and dark themes, at least two anchor positions
4. Confirmation that the full §6.4 checklist has been verified
5. Known limitations listed explicitly

---

## License

By contributing, you agree that your contributions will be licensed under the **MIT No Attribution License**. See [LICENSE](LICENSE) for details.
