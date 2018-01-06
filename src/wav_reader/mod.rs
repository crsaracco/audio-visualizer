use hound;
use chan::Sender;

/// Read samples from a .wav file
pub fn read_samples(filename: &str, send_audio_samples: Sender<(i16, i16)>) {
    // Get an iterator over samples in the .wav file:
    let mut reader = hound::WavReader::open(filename).unwrap();
    let mut sample_iterator = reader.samples::<i16>();

    loop {
        let left = match sample_iterator.next() {
            Some(Ok(t)) => t,
            _ => break,
        };
        let right = match sample_iterator.next() {
            Some(Ok(t)) => t,
            _ => break,
        };
        send_audio_samples.send((left, right));
    }
    println!("End of file.");
}
