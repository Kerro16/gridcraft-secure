# Security hardening of this fork

Based on upstream `storytold/gridcraft` at commit d8c5f588.

## Changes
- Removed the `craft-update` dependency (external git crate `storytold/craft-libs`): the app no longer
  checks for, downloads or installs updates, and makes no network requests at startup.
- Removed the `--finish-update` self-update helper and the startup update check from
  `apps/gridcraft/src/main.rs`.
- `crates/ui-egui/src/updates_ui.rs` is now a stub; Help > Check for Updates only shows a notice.

## Not yet done
- Remaining git dependencies (`craft-libs`: craft-pdf, craft-print, craft-ui, craft-window, craft-i18n,
  craft-fonts-build; forks of egui, winit, winprint.rs) are pinned to exact commits but not fully audited.
- The optional local control server (`--control`, 127.0.0.1 + token) is unchanged and off by default.
