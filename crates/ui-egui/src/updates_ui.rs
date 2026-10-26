//! Software updates are disabled in this build: it never contacts the network. Help ▸ Check for
//! Updates only shows a notice.

use crate::SheetApp;

#[derive(Default)]
pub(crate) struct Runtime;

/// Startup checks do nothing; a manual one (Help ▸ Check for Updates…) says updates are off.
pub(crate) fn start(app: &mut SheetApp, manual: bool) {
    if manual {
        app.message = Some(("GridCraft".into(), "Automatic updates are disabled in this build.".into()));
    }
}

pub(crate) fn tick(_app: &mut SheetApp, _ctx: &egui::Context) {}

pub(crate) fn show(_app: &mut SheetApp, _ctx: &egui::Context) {}
