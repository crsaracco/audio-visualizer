# audio-visualizer

This is a pretty simple audio visualizer in Rust, using Piston, PortAudio, and RustFFT.

It shows a "spectrum analyzer" graph, which plots the frequencies (20Hz - 20kHz) and their relative loudness over time:

![output.gif](output.gif)

The code is a little messy at the moment though...

## Building and Running

You'll probably want to build this in release mode.

For now it only accepts `wav` inputs. You can also supply some graphing arguments: y-axis offset, and y-axis scale (in that order):

```
cargo run --release -- <wav file> 0.0 0.2
```

## Future Ideas


 - It would be neat to use this in conjunction with the [rust-vst](https://github.com/rust-dsp/rust-vst) crate to
create a spectrum analyzer VST that you can plug into your Digital Audio Workstation.
 - Having some overlays that show where the exact frequencies are would be helpful, since this is a log-log plot.
 - I'd like to be able to detect what notes are being played at any given time, maybe some peak-detection and some careful
 harmonics math could do this.
