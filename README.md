# Wavemaker
Wavemaker is a small library which lets you read and write .wav files from scratch. This library is inspired by the first chapter, [wave](https://mu.krj.st/wave/), of [project μ](https://mu.krj.st/), which was originally written in C.

## Usage

### Creating a .wav file
Creating a .wav file can be achieved in just a few lines of code: 

```rust
let config = Config {
        sample_size: SAMPLE_SIZE,
        channels: NCHANNELS,
        sample_rate: SR,
        bits_per_sample: 8 as u16 * SAMPLE_SIZE,
        nsamples: NSAMPLES,
        duration: 5,
    };
    
    let wav_file = Wave::new(&config);    
    wav_file.write_header("output.wav");

    let frequency = 220.0;
    wav_file.write_data("output.wav", frequency);
``` 

The config is used to pass in general parameters, such as the sample size or the bit depth.