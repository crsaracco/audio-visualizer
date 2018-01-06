use portaudio as pa;
use chan;
use std::{thread, time};

// Constants:
const NUM_CHANNELS: i32 = 2;
const FRAMES_PER_BUFFER: u32 = 1024;
const BUFFER_SECONDS: f64 = 0.100;  // Buffer samples for 100ms -- reduces chances of underrun
const SAMPLE_RATE: f64 = 44100.0;

/// "Run" the audio thread
/// Probably want to run this in a separate thread and send samples over a channel.
pub fn run(recv_audio_samples: chan::Receiver<(i16, i16)>) -> Result<(), pa::Error> {
    // Sleep a little so we don't underrun our audio buffer (probably not even needed but whatever):
    thread::sleep(time::Duration::new(0, 100_000));

    // Fire up ye olde PortAudio:
    println!("=============");
    let pa = try!(pa::PortAudio::new());
    println!("=============");

    // Set up our settings - set a buffer amount to try to reduce underruns:
    let mut settings = try!(pa.default_output_stream_settings(NUM_CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    settings.params.suggested_latency = BUFFER_SECONDS;

    // This callback function will be called by PortAudio when it needs more audio samples.
    // It may be called at interrupt level on some machines, so don't do anything that could mess
    // up the system like dynamic resource allocation or I/O. (although doing so seems to be fine on
    // my machine...?)
    //
    // The job of this callback is to fill up the buffer that PortAudio tells us to fill up.
    // Each "frame" represents one sample for each channel that we have, so we need to put a total
    // of (NUM_CHANNELS * frames) samples into the buffer.
    // The samples are "interleaved" by default, so the structure of buffer looks like:
    // [ch0_sample0, ch1_sample0, ch0_sample1, ch1_sample1, ch0_sample2, ch1_sample2, ...]
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut i = 0;
        for _ in 0..frames {
            match recv_audio_samples.recv() {
                Some(pair) => {
                    buffer[i]   = (pair.0 as f32)/32768.0;
                    buffer[i+1] = (pair.1 as f32)/32768.0;
                    i += 2;
                }
                None => {
                    // Something...
                }
            };
            /*
            let sample = recv_audio.recv().unwrap();
            send_points.send(sample.clone());
            buffer[i]   = sample as f32;
            buffer[i+1] = sample as f32;
            i += 2;
            */
        }
        pa::Continue
    };

    // Now that we have the settings and the callback function set up, we can finally open the
    // stream, through which we will actually play audio:
    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    // And now that we have the stream, we can start playing sounds!
    try!(stream.start());

    // We're using PortAudio in non-blocking mode, so execution will fall through immedately.
    // Sleep to make sure we keep playing audio
    loop {
        thread::sleep(time::Duration::new(1, 0));
    }

    // We're done playing, gracefully shut down the stream:
    try!(stream.stop());
    try!(stream.close());

    Ok(())
}