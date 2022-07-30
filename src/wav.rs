use std::{fs::File, str::FromStr};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize};
use mp4::FourCC;


type Sample = i16;


//const SAMPLE_SIZE: u16 = std::mem::size_of::<Sample>() as u16;

const M_PI: f64 = 3.14159265358979323846;
const SAMPLE_MAX: u16 = 32767;
// const DURATION: u32 = 5;
// const SR: u32 = 44100;
// const NCHANNELS: u16 = 1;
// const NSAMPLES: u32 = NCHANNELS as u32 * DURATION * SR;

#[derive(Serialize, PartialEq, Debug)]
struct RiffHeader {
    id: FourCC,
    size: u32,
    ftype: FourCC,
}

impl RiffHeader {
    fn new(NSAMPLES: u32, SAMPLE_SIZE: u16) -> Self {
        RiffHeader {
            id: FourCC::from_str("RIFF").unwrap(),
            size: 36 + NSAMPLES * SAMPLE_SIZE as u32,
            ftype: FourCC::from_str("WAVE").unwrap(),
        }
    }
}

// Metadata of WAV file
#[derive(Serialize, PartialEq, Debug)]
struct FmtChunk {
    id: FourCC,
    size: u32,
    // type of encoding
    fmt_tag: u16,
    // 1 = mono, 2 = stereo
    channels: u16,
    // aka sampling rate
    samples_per_sec: u32,
    // channels × (samples/sec) × (bits/sample) ÷ 8
    bytes_per_sec: u32,
    // number of bytes of a single sample of audio
    block_align: u16,
    // aka bit depth
    bits_per_sample: u16,
}

impl FmtChunk {
    fn new(SR: u32, NCHANNELS: u16, SAMPLE_SIZE: u16, bits_per_sample: u16) -> Self {
        FmtChunk {
            id: FourCC::from_str("fmt ").unwrap(),
            size: 16,
            fmt_tag: 1,
            channels: NCHANNELS,
            samples_per_sec: SR,
            bytes_per_sec: NCHANNELS as u32 * SR * SAMPLE_SIZE as u32,
            block_align: NCHANNELS * SAMPLE_SIZE,
            bits_per_sample,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
struct DataHeader {
    id: FourCC,
    size: u32,
}

impl DataHeader {
    fn new(NSAMPLES: u32, SAMPLE_SIZE: u16) -> Self {
        DataHeader {
            id: FourCC::from_str("data").unwrap(),
            size: NSAMPLES * SAMPLE_SIZE as u32,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct WavHeader {
    riff_header: RiffHeader,
    fmt_chunk: FmtChunk,
    data_header: DataHeader,
}

impl WavHeader {
    pub fn new(config: Config) -> Self {
        WavHeader {
            riff_header: RiffHeader::new(config.nsamples, config.sample_size),
            fmt_chunk: FmtChunk::new(config.sample_rate, config.channels, config.sample_size, config.bits_per_sample),
            data_header: DataHeader::new(config.nsamples, config.sample_size),
        }
    }

    pub fn create_wav(&self, p: &str) {
        let path = Path::new(p);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}. Error: {}", display, why),
            Ok(file) => file,
        };

        // We have to serialize the struct's data and turn it into a Vec<u8>
		// this actually works with write_all(&[u8]) because Vec<T> implements 
		// AsRef<[T]>, so &Vec<T> can be coerced into &[T]
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        match file.write_all(&encoded) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

    }
}


fn as_u8_slice(slice_i16: &[i16]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice_i16.as_ptr() as *const u8, 
            slice_i16.len() * std::mem::size_of::<u16>(),
        )
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Wave {
    header: WavHeader,
    config: Config,
}

impl Wave {
    pub fn new(config: Config) -> Self {
        Wave { 
            header: WavHeader::new(config),
            /* TODO: fix the config issue (lifetimes?) & get the buffer to print (shouldn't the buffer also be written in src/main.rs?) */
            config,
        }
    }

    pub fn write_header(&self, p: &str) {
        let path = Path::new(p);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}. Error: {}", display, why),
            Ok(file) => file,
        };

        // We have to serialize the struct's data and turn it into a Vec<u8>
		// this actually works with write_all(&[u8]) because Vec<T> implements 
		// AsRef<[T]>, so &Vec<T> can be coerced into &[T]
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        match file.write_all(&encoded) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    // buf: &mut [Sample; config.nsamples as usize]
    pub fn write_data(&self, p: &str) {
        let mut buf = Vec::new();
        // let mut t_buf: [Sample; self.config.nsamples] = [0; self.config.nsamples as usize]; 
        let amplitude = 0.2;
        for i in 0 .. self.config.nsamples {
            let sample = SAMPLE_MAX as f64 * amplitude * (2.0 * M_PI * 440.0 * i as f64/self.config.sample_rate as f64).sin();
            buf.push(sample as i16);
            // t_buf[i as usize] = sample as i16;
        }
        println!("Printing from buffer:\n");
        // for (i, sample) in buf.into_iter().enumerate() {
        //     println!("i: {}, sample value: {}", i, sample);
        // }
    
        let buf_as_u8 = as_u8_slice(&buf);
        let mut file = OpenOptions::new().
        append(true).
        open(p).
        unwrap(); 
    
        match file.write_all(buf_as_u8) {
            Err(why) => panic!("couldn't write sample data to {}: {}", p, why),
            Ok(_) => println!("successfully appended sample data to {}", p),
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
// Config allows you to set the different parameters, such as NCHANNELS, SR, bit depth, etc. 
pub struct Config {
    sample_size: u16,
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    nsamples: u32,
    duration: u32,
}

impl Config {
    pub fn create(sample_size: u16, channels: u16, sample_rate: u32, bits_per_sample: u16, duration: u32) -> Self {
        Config { 
            sample_size, 
            channels,
            sample_rate, 
            bits_per_sample,
            duration,
            nsamples: channels as u32 * duration * sample_rate
        }
    }
}
