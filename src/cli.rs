#![allow(unused_imports)]

use wavemaker::wav::*;
use clap::{Parser, Subcommand, Args};

type Sample = i16;
const SAMPLE_SIZE: u16 = std::mem::size_of::<Sample>() as u16;

const DURATION: u32 = 5;
const SR: u32 = 44100;
const NCHANNELS: u16 = 1;
const NSAMPLES: u32 = NCHANNELS as u32 * DURATION * SR;

#[derive(Subcommand, Debug)]
enum Commands {
    /// Reads a WAV file into buffer  
    Read(Read),
    /// Writes a sine wave to a WAV file 
    WriteSine(WriteSine)
}

#[derive(Args, Debug)]
struct Read {
    /// Path to WAV file to be read into buffer
    #[arg(short = 'p', long = "path")]
    #[arg(required = true)]
    path: String,

    /// The time in ms until which the sample data will be read (defaults to 1000ms)
    #[arg(short = 't', long = "time")]
    #[arg(default_value_t = 1000)]
    time: u32,
}

// Currently not fully implemented!
#[derive(Args, Debug)]
struct WriteSine {
    /// Frequency of sine wave
    #[arg(short = 'f', long = "freq")]
    frequency: f64,

    /// Amplitude of sine wave
    #[arg(short = 'a', long = "ampl")]
    amplitude: f64
}



/// CLI arguments for reading and writing WAV files
#[derive(Parser, Debug)]
#[command(name = "Wavemaker")]
#[command(author = "Philipp Armingeon <philipp.armingeon@googlemail.com>")]
#[command(version = "1.0")]
#[command(about = "CLI version of Wavemaker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    println!("commands: {:?}", cli.command);

    match &cli.command {
        Commands::Read(reader) => {
            println!("Arguments passed to read subcommand: {:?} {:?}", reader.path, reader.time);
            let config = Config {
                sample_size: SAMPLE_SIZE,
                channels: NCHANNELS,
                sample_rate: SR,
                bits_per_sample: 8 as u16 * SAMPLE_SIZE,
                nsamples: NSAMPLES,
                duration: 5,
            };
            let wav = read(&reader.path, &config);
            wav.read_data_until_ms(reader.time);
        }
        Commands::WriteSine(_write) => {
            println!("Writing from CLI has not been implemented yet");
        }
    }
}

