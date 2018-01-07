use graphics;

use piston::window::WindowSettings;
use piston::event_loop;
use piston::input;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

// Trait uses:
use piston::input::{RenderEvent, UpdateEvent};

use chan;

const WINDOW_SIZE: [u32; 2] = [(1920.0) as u32, (1080.0) as u32];
const BUFFER_MULTIPLIER: usize = 2;
const BUFFER_SIZE: usize = (WINDOW_SIZE[0] as usize)*BUFFER_MULTIPLIER;
const SAMPLES_PER_FRAME: usize = (44100.0/60.0*1.2) as usize;

pub fn audio_visualizer(recv_graph_samples: chan::Receiver<(i16, i16)>, sub: f64, mult: f64) {
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
        buffer: Vec::new(),
        draw_buffer: vec![0.0; WINDOW_SIZE[0] as usize],
    };

    let mut events = event_loop::Events::new(event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &recv_graph_samples, sub, mult);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    buffer: Vec<Complex<f64>>,
    draw_buffer: Vec<f64>,
}

impl App {
    fn render(&mut self, args: &input::RenderArgs, recv_graph_samples: &chan::Receiver<(i16, i16)>, sub: f64, mult: f64) {
        const BLACK:  [f32; 4] = [0.114, 0.125, 0.129, 1.0];

        for i in 0..SAMPLES_PER_FRAME {
            match recv_graph_samples.recv() {
                Some(t) => {
                    let value = t.0;
                    self.buffer.push(Complex::new(value as f64, 0.0));
                },
                None => break,
            };
        }

        while self.buffer.len() > BUFFER_SIZE {
            self.buffer.remove(0);
        }

        if self.buffer.len() == BUFFER_SIZE {
            // Stupid FFT crate uses the input buffer as scratch data, which messes up the entire
            // buffer. Copy it to a new buffer so we don't break everything
            let mut input = self.buffer.clone();

            // Perform FFT on the samples
            let mut output: Vec<Complex<f64>> = vec![Complex::zero(); self.buffer.len()];
            let mut planner = FFTplanner::new(false);
            let fft = planner.plan_fft(self.buffer.len());
            fft.process(&mut input, &mut output);

            let line_width = 1.0;

            // Set up our draw buffer (x,y values)

            for sample in 0..WINDOW_SIZE[0] as usize {
                let x =
                    ((sample as f64 / BUFFER_SIZE as f64).powi(2) /
                        (1.0 / (BUFFER_MULTIPLIER as f64)).powi(2) *
                        BUFFER_SIZE as f64) as usize / 2;
                let y = output[x].to_polar().0.log(10.0).powi(4);


                self.draw_buffer[sample] = self.draw_buffer[sample] * 0.75 + y * 0.25;

                /*
                if y > self.draw_buffer[sample] {
                    self.draw_buffer[sample] = y;
                }
                else {
                    self.draw_buffer[sample] = self.draw_buffer[sample] * 0.5;
                }
                */
            }

            // We can't borrow self in a closure that uses self (apparently), so we have to make
            // a reference outside of the closure.
            let mut draw_buffer = &self.draw_buffer;

            self.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                graphics::clear(BLACK, gl);

                for sample in 0..WINDOW_SIZE[0] as usize {
                    let line = graphics::Line::new(hsv_to_rgb(sample as f32 / WINDOW_SIZE[0] as f32 * 360.0, 1.0, 0.75), line_width);

                    line.draw([sample as f64, WINDOW_SIZE[1] as f64, sample as f64, WINDOW_SIZE[1] as f64 - (draw_buffer[sample]*mult-sub)], &Default::default(), c.transform, gl);
                }

            });
        }
    }

    fn update(&mut self, args: &input::UpdateArgs) {
    }
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> [f32; 4] {
    let c: f32 = value * saturation;
    let x: f32 = c * (1.0 - ((hue/60.0) % 2.0 - 1.0).abs()) as f32;
    let m: f32 = value - c;

    let rgb_prime: [f32; 3] = match hue {
        a if a >= 0.0 && a < 60.0 => [c, x, 0.0],
        a if a >= 60.0 && a < 120.0 => [x, c, 0.0],
        a if a >= 120.0 && a < 180.0 => [0.0, c, x],
        a if a >= 180.0 && a < 240.0 => [0.0, x, c],
        a if a >= 240.0 && a < 300.0 => [x, 0.0, c],
        a if a >= 300.0 && a < 360.0 => [c, 0.0, x],
        _ => [0.0, 0.0, 0.0],
    };



    return [rgb_prime[0]+m, rgb_prime[1]+m, rgb_prime[2]+m, 1.0];
}