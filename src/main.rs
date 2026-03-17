use crate::app::App;

mod app;
mod game_context;
mod renderer;
mod ui;

fn main() -> Result<(), String> {
    let mut app: App = App::new()?;

    app.run();
    println!("Hello, world!");
    Ok(())
}
