use wavemaker::wav::*;

type Sample = i16;
const SAMPLE_SIZE: u16 = std::mem::size_of::<Sample>() as u16;

const DURATION: u32 = 5;
const SR: u32 = 44100;
const NCHANNELS: u16 = 1;
const NSAMPLES: u32 = NCHANNELS as u32 * DURATION * SR;

fn main() {
    // Let's first define the required metadata which will go into the header of our WAV file 
    let config = Config {
        sample_size: SAMPLE_SIZE,
        channels: NCHANNELS,
        sample_rate: SR,
        bits_per_sample: 8 as u16 * SAMPLE_SIZE,
        nsamples: NSAMPLES,
        duration: 5,
    };
    let frequency = 220.0;
    let amplitude = 0.2; 

    // We can then use the config to initialise a new WAV file
    let mut wav = Wave::new(&config);    

    // Let's now create a sine using the frequency and amplitude we defined above  
    wav.write_sine("sine.wav", frequency, amplitude);

    // We can also create a WAVE variable by reading it from a file
    // NOTE: At this point this also requires us to pass in a config which we define ourselves. 
    // Removing this necessity is a task for a future refactor
    let _wav = read("sine.wav", &config); 

    // We can now read the associated sample data directly from the struct field
    wav.read_data();

    // We can additionally read only a subset of the sample data by specifying a time limit in ms
    let time = 50;
    wav.read_data_until(time);
}

