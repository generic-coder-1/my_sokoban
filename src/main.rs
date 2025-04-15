use std::io;
mod main_menu;
mod quit_menu;
mod board;
mod tiles;
mod tools;
mod edit_menu;
mod menu;
use app::App;
mod app;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    App::new().run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}
