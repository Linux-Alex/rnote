// Imports
use crate::workspacebrowser::RnFileRow;
use crate::RnAppWindow;
use gtk4::{gio, glib, glib::clone};

/// Create a new `open` action.
pub(crate) fn open(filerow: &RnFileRow, appwindow: &RnAppWindow) -> gio::SimpleAction {
    let action_open_file = gio::SimpleAction::new("open-file", None);
    action_open_file.connect_activate(
        clone!(@weak filerow as filerow, @weak appwindow => move |_action_open_file, _| {
            if let Some(current_file) = filerow.current_file() {
                 appwindow.open_file_w_dialogs(current_file, None, true);
            }
        }),
    );

    action_open_file
}
