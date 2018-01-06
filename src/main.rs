// Extern crates:
extern crate chan;
extern crate hound;
extern crate piston_window;
extern crate portaudio;

// Crate uses:
use std::thread;
use chan::Receiver;

// Our own modules:
mod wav_reader;
mod audio_player;
mod audio_visualizer;

// Consts:
const FILENAME: &str = "Poldoore - That Game You're Playing.wav";
const SAMPLE_RATE: usize = 44100;

fn main() {
    // Create a channel so we can read the .wav in one thread, buffer up some samples,
    // and play it in another thread:
    let (send_audio_samples, recv_audio_samples) = chan::sync(SAMPLE_RATE);

    // Collect all our threads so we can .join() later:
    let mut threads = vec![];

    // Create the thread that reads our .wav file:
    threads.push(thread::spawn(move || {
        wav_reader::read_samples(FILENAME, send_audio_samples);
    }));

    // Create the thread that plays our audio:
    threads.push(thread::spawn(move || {
        audio_player::run(recv_audio_samples);
    }));

    // Create the thread that visualizes our audio!
    threads.push(thread::spawn(move || {
        audio_visualizer::audio_visualizer();
    }));

    // Wait for all the threads to finish:
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
