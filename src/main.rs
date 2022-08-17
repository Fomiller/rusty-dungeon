mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}
use prelude::*;

const FPS_CAP: f32 = 60.0;

struct State {
    map: Map,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.map.render(ctx)
    }
}

impl State {
    fn new() -> Self {
        Self { map: Map::new() }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Dungeon")
        .with_fps_cap(FPS_CAP)
        .build()?;
    main_loop(context, State::new())
}
