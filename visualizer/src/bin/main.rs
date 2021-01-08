use visualizer::*;

use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    thread::spawn(|| match midi::run_midi() {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    });

    let mut graphics = render::Graphics::new()?;
    loop {
        if graphics.has_quit() {
            break;
        }
        graphics.clear_canvas();
        graphics.present_canvas();
    }

    Ok(())
}
