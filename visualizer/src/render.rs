extern crate sdl2;

use crate::bar;

pub struct Graphics {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl Graphics {
    pub fn new() -> Result<Graphics, String> {
        let context = sdl2::init()?;
        let video_subsystem = context.video()?;

        let window = video_subsystem
            .window("Crisp Visualizer", 640, 480)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = context.event_pump()?;

        Ok(Graphics { canvas, event_pump })
    }

    pub fn has_quit(&mut self) -> bool {
        let mut quit = false;
        for event in self.event_pump.poll_iter() {
            quit = match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => true,
                _ => false,
            };
            if quit {
                break;
            }
        }
        return quit;
    }

    pub fn draw_bars(&mut self, bars: &Vec<bar::Bar>) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        let (width, height) = self.canvas.output_size().unwrap();
        let bar_width = width / (bars.len() as u32);
        let mut current_bar = 0;
        for bar in bars.iter() {
            if bar.is_active() {
                let x = (bar_width * current_bar) as i32;
                let bar_height = ((bar.get_value() as f64 / 1000.0) * height as f64) as u32;
                let y = (height - bar_height) as i32;
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(x, y, bar_width, bar_height))
                    .unwrap();
            }
            current_bar += 1
        }
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.clear();
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }
}
