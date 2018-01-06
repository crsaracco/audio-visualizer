use piston_window;

const WINDOW_SIZE: piston_window::Size = piston_window::Size {
    width: 1024,
    height: 768,
};

const BLACK: [f32; 4] = [0.114, 0.125, 0.129, 1.0];
const RED:   [f32; 4] = [0.984, 0.286, 0.204, 1.0];

pub fn audio_visualizer() {
    use piston_window::*;

    // Make a window
    let mut window: PistonWindow = WindowSettings::new("Audio Visualizer", WINDOW_SIZE)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Main loop of the visualizer
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {

            // Clear the window with "black" (dark grey)
            clear(BLACK, graphics);

            // Draw a red rectangle
            rectangle(
                RED,
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics,
            );

        });
    }
}
