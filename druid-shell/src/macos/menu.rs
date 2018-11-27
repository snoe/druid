pub struct Menu {}

impl Menu {
    pub fn new() -> Menu {
        Menu {}
    }

    pub fn add_item(&self, id: usize, label : &str) {
    }

    pub fn add_dropdown(&self, sub_menu : Menu, label : &str) {
    }
}
