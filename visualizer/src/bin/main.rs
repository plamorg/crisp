use visualizer::*;

use std::cmp;
use std::error::Error;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

const MINIMUM_NOTE: u8 = 23;
const MAXIMUM_NOTE: u8 = 109;

fn get_bars() -> Vec<bar::Bar> {
    let mut bars: Vec<bar::Bar> = Vec::new();
    for i in MINIMUM_NOTE..MAXIMUM_NOTE + 1 {
        bars.push(bar::Bar::new(i));
    }
    bars
}

fn adjust_bars(bars: &mut Vec<bar::Bar>, delta: i16) {
    for bar in bars.iter_mut() {
        bar.adjust(delta);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = channel();
    thread::spawn(|| match midi::run_midi(tx) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    });

    let mut graphics = render::Graphics::new()?;
    let mut bars = get_bars();
    loop {
        let now = Instant::now();
        if graphics.has_quit() {
            break;
        }
        graphics.clear_canvas();
        if let Ok((status, note)) = rx.try_recv() {
            let status = match status {
                0x90 => true,
                _ => false,
            };
            bars[cmp::max(MINIMUM_NOTE, cmp::min(MAXIMUM_NOTE, note - MINIMUM_NOTE)) as usize]
                .set_active(status);
        };
        graphics.draw_bars(&bars);
        graphics.present_canvas();
        adjust_bars(&mut bars, now.elapsed().as_millis() as i16);
    }

    Ok(())
}
