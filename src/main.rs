use crate::{app::App, renderer::Renderer};

mod app;
mod renderer;
mod game_context;

fn main() -> Result<(), String> {
    let mut app: App = App::new()?;
    let mut renderer: Renderer = Renderer::new(&mut app)?;

    app.run(&mut renderer);
    println!("Hello, world!");
    Ok(())
}
