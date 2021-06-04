use ggez::{graphics, ContextBuilder, GameResult, conf};
use ggez::event;

mod lib;

fn main() -> GameResult<()> {
    let (mut context, event_loop) = ContextBuilder::new("maze_maker", "Amirhosein_GPR").build().expect("Error extracting GameResult in ContextBuilder");

    let game_state = lib::maze_maker::Game::new(&mut context);

    graphics::set_resizable(&mut context, true)?;
    graphics::set_drawable_size(&mut context, 720.0, 720.0)?;

    graphics::set_window_title(&mut context, "Maze maker");

    event::run(context, event_loop, game_state);

    Ok(())
}