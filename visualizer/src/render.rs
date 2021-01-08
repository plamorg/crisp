extern crate sdl2;

pub struct Graphics {
    context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
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

        Ok(Graphics {
            context,
            video_subsystem,
            canvas,
            event_pump,
        })
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

    pub fn draw_bar(&mut self) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(10, 10, 780, 580));
    }

    pub fn clear_canvas(&mut self) {
        self.canvas.clear();
    }

    pub fn present_canvas(&mut self) {
        self.canvas.present();
    }
}
