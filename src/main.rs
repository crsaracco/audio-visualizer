// Extern crates
extern crate chan;
extern crate hound;
extern crate portaudio;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rustfft;

// Crate uses
use std::thread;
use std::env;
use chan::Receiver;

// Our own modules
mod wav_reader;
mod audio_player;
mod audio_visualizer;

// Consts
const SAMPLE_RATE: usize = 44100;
const DRAW_RATE: usize = SAMPLE_RATE / 60 * 2; // Samples per frame, for the visualizer (times two for a nice buffer)

fn main() {
    // Get command line arguments
    let args: Vec<_> = env::args().collect();
    let filename = args[1].clone();

    // Create a channel so we can read the .wav in one thread, buffer up some samples,
    // and play it in another thread
    let (send_audio_samples, recv_audio_samples) = chan::sync(SAMPLE_RATE);

    // Create a channel so that we can pass samples from the audio player to the audio visualizer,
    // again in another thread
    let (send_graph_samples, recv_graph_samples) = chan::sync(DRAW_RATE);

    // Collect all our threads so we can .join() later
    let mut threads = vec![];

    // Create the thread that reads our .wav file
    threads.push(thread::spawn(move || {
        wav_reader::read_samples(&filename, send_audio_samples);
    }));

    // Create the thread that plays our audio
    threads.push(thread::spawn(move || {
        audio_player::run(recv_audio_samples, send_graph_samples);
    }));

    // Create the thread that visualizes our audio!
    threads.push(thread::spawn(move || {
        audio_visualizer::audio_visualizer(recv_graph_samples, args[2].parse().unwrap(), args[3].parse().unwrap());
    }));

    // Wait for all the threads to finish
    for thread in threads {
        let _ = thread.join();
    }
}

/// Print samples from a channel (debug)
fn print_samples(recv_audio_samples: Receiver<(i16, i16)>) {
    let mut counter = 0;

    loop {
        counter += 1;

        match recv_audio_samples.recv() {
            Some(pair) => {
                if counter % 1024 == 0 {
                    println!("L: {} | R: {}", pair.0, pair.1);
                }
            }
            None => {
                println!("End of file.");
                break;
            }
        };
    }
}
