mod map;
use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Welcome to Rusty Dungeon!")
    }
}

impl State {
    fn new() -> Self {
        Self {}
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Rusty Dungeon")
        .build()?;
    main_loop(context, State::new())
}
