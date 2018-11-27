pub struct FileDialogOptions {}

impl FileDialogOptions {
    pub fn default() -> FileDialogOptions {
        FileDialogOptions {}
    }

    pub fn set_show_hidden(&self) {
    }
}

pub enum FileDialogType {
    Open
}

