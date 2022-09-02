use wavemaker::wav::*;

type Sample = i16;
const SAMPLE_SIZE: u16 = std::mem::size_of::<Sample>() as u16;

const DURATION: u32 = 5;
const SR: u32 = 44100;
const NCHANNELS: u16 = 1;
const NSAMPLES: u32 = NCHANNELS as u32 * DURATION * SR;

fn main() {
    let config = Config {
        sample_size: SAMPLE_SIZE,
        channels: NCHANNELS,
        sample_rate: SR,
        bits_per_sample: 8 as u16 * SAMPLE_SIZE,
        nsamples: NSAMPLES,
        duration: 5,
    };

    let wav_file = Wave::new(&config);    
    // wav_file.write_header("output.wav");    

    // let frequency = 220.0;
    // wav_file.write_data("output.wav", frequency);
    wav_file.read_data("output.wav");
}
