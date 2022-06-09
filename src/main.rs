use bracket_lib::prelude::{main_loop, BError, BTermBuilder};
use stellar::state::State;

fn main() -> BError {
    let context = BTermBuilder::simple80x50().with_title("Stellar").build()?;

    main_loop(context, State::new())
}
