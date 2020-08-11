use tcod::console::*;
use crate::config::CONFIG;

pub struct Ui {
    pub root: Root,
}

impl Ui {
    pub fn new() -> Self {

        // Set up the console.
        let root = Root::initializer()
            .font(CONFIG.font.clone(), FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(CONFIG.width, CONFIG.height)
            .title("Rustlike")
            .init();

        Ui { root }
    }
}