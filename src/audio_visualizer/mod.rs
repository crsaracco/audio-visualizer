use graphics;

use piston::window::WindowSettings;
use piston::event_loop;
use piston::input;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

// Trait uses:
use piston::input::{RenderEvent, UpdateEvent};
use graphics::Transformed;

use chan;

const WINDOW_SIZE: [u32; 2] = [1920, 1080];

const BLACK: [f32; 4] = [0.114, 0.125, 0.129, 1.0];
const RED:   [f32; 4] = [0.984, 0.286, 0.204, 1.0];

pub fn audio_visualizer(recv_graph_samples: chan::Receiver<(i16, i16)>) {
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new(
        "Audio Visualizer",
        WINDOW_SIZE
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0
    };

    let mut events = event_loop::Events::new(event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &recv_graph_samples);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    rotation: f64   // Rotation for the square
}

impl App {
    fn render(&mut self, args: &input::RenderArgs, recv_graph_samples: &chan::Receiver<(i16, i16)>) {
        const BLACK:  [f32; 4] = [0.114, 0.125, 0.129, 1.0];
        const RED:    [f32; 4] = [0.984, 0.286, 0.204, 1.0];
        const ORANGE: [f32; 4] = [0.996, 0.502, 0.098, 1.0];
        const YELLOW: [f32; 4] = [0.980, 0.741, 0.184, 1.0];
        const GREEN:  [f32; 4] = [0.722, 0.733, 0.149, 1.0];
        const BLUE:   [f32; 4] = [0.514, 0.647, 0.596, 1.0];
        const PURPLE: [f32; 4] = [0.827, 0.525, 0.608, 1.0];

        let mut total: u64 = 0;
        let mut count: u64 = 0;

        for i in 0..750 {
            match recv_graph_samples.recv() {
                Some(t) => {
                    total += (t.0.abs() + t.1.abs()) as u64;
                    count += 2;
                },
                None => break,
            };
        }

        let border_size = 2.0;

        // TODO: figure out how to make this more generic and loop through it
        let square1_size = (total as f64)/(count as f64)/25.0;

        let square1 = graphics::rectangle::square(0.0, 0.0, square1_size);

        let border1 = graphics::rectangle::Border{color: RED, radius: border_size};

        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            graphics::clear(BLACK, gl);

            // Make it rotate
            let transform1 = c.transform.trans(x, y).rot_rad(rotation*1.0).trans(-(square1_size/2.0), -(square1_size/2.0));

            // Draw boxes rotating around the middle of the screen.
            graphics::Rectangle::new(BLACK).border(border1).draw(square1, &Default::default(), transform1, gl);
        });
    }

    fn update(&mut self, args: &input::UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 0.5 * args.dt;
    }
}