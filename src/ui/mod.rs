use specs::{Join, World, WorldExt};
use tcod::console::*;
use tcod::colors::*;
use crate::config::CONFIG;
use crate::components::{Position, Renderable};
use crate::handle_keys;
use tcod::console;

pub struct Ui {
    pub(crate) root: Root,
    pub con: Offscreen,
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

        // Con uses the whole screen right now.
        let con = Offscreen::new(CONFIG.width, CONFIG.height);
        Ui { root, con }
    }

    /// Collect all of the screens/consoles and render them to root.
    pub fn render(&mut self, entities: &mut World) -> bool {
        self.con.set_default_foreground(WHITE);
        self.con.clear();

        {
            // Draw all the entities
            let mut positions = &entities.read_storage::<Position>();
            let mut renderables = &entities.read_storage::<Renderable>();

            for (pos, render) in (positions, renderables).join() {
                self.con.put_char_ex(pos.x, pos.y, render.glyph, render.fg, render.bg);
            }
        }

        console::blit(&self.con,
                      (0,0),
                      (CONFIG.width, CONFIG.height),
                      &mut self.root,
                      (0,0), 1.0, 1.0);

        self.root.flush();

        if handle_keys(self, entities) {
            return false;
        }

        return true;
    }
}