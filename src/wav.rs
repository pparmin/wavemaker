use std::{fs::File, str::FromStr};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use serde::{Serialize};
use mp4::FourCC;


type Sample = i16;


const SAMPLE_SIZE: u16 = std::mem::size_of::<Sample>() as u16;

const M_PI: f64 = 3.14159265358979323846;
const SAMPLE_MAX: u16 = 32767;
const DURATION: u32 = 5;
const SR: u32 = 44100;
const NCHANNELS: u16 = 1;
const NSAMPLES: u32 = NCHANNELS as u32 * DURATION * SR;

// static mut BUF: [Sample; NSAMPLES as usize] = [0; NSAMPLES as usize];

#[derive(Serialize, PartialEq, Debug)]
struct RiffHeader {
    id: FourCC,
    size: u32,
    ftype: FourCC,
}

impl RiffHeader {
    fn new() -> Self {
        RiffHeader {
            id: FourCC::from_str("RIFF").unwrap(),
            size: 36 + NSAMPLES * SAMPLE_SIZE as u32,
            ftype: FourCC::from_str("WAVE").unwrap(),
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
struct FmtChunk {
    id: FourCC,
    size: u32,
    fmt_tag: u16,
    channels: u16,
    samples_per_sec: u32,
    bytes_per_sec: u32,
    block_align: u16,
    bits_per_sample: u16,
}

// Possible issue:
// The type conversion between u16 and u32 may cause the wrong sizes in the WAV header 
// --> Possibly requires re-write/changing back 
impl FmtChunk {
    fn new() -> Self {
        FmtChunk {
            id: FourCC::from_str("fmt ").unwrap(),
            size: 16,
            fmt_tag: 1,
            channels: NCHANNELS,
            samples_per_sec: SR,
            bytes_per_sec: NCHANNELS as u32 * SR * SAMPLE_SIZE as u32,
            block_align: NCHANNELS * SAMPLE_SIZE,
            bits_per_sample: 8 as u16 * SAMPLE_SIZE,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
struct DataHeader {
    id: FourCC,
    size: u32,
}

impl DataHeader {
    fn new() -> Self {
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

// TO-DO: 
// 1. Implement a function that allows to write WAV data to the file after creating it. THIS SHOULD BE DONE IN src/main.rs
//    --> fill_data(&wav-file, data: [String; NSAMPLES]) 
// 2. Rewrite initiation function (WavHeader::new()) in order to allow setting certain parameters that are currently hard-coded
//    --> DURATION, SR, NCHANNELS, etc.
impl WavHeader {
    pub fn new() -> Self {
        WavHeader {
            riff_header: RiffHeader::new(),
            fmt_chunk: FmtChunk::new(),
            data_header: DataHeader::new(),
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

    // TO-DO: check hound (https://github.com/ruuda/hound) for loop implementation
    pub fn write_data(&self, buf: &mut [Sample; NSAMPLES as usize], p: &str) {
        let mut t_buf: [Sample; NSAMPLES as usize] = [0; NSAMPLES as usize]; 
        let amplitude = 0.2;
        for i in 0 .. NSAMPLES {
            let sample = SAMPLE_MAX as f64 * amplitude * (2.0 * M_PI * 440.0 * i as f64/SR as f64).sin();
            //println!("i: {}, sample value: {} - AFTER ROUNDING: {}", i, sample, sample as i16);
            t_buf[i as usize] = sample as i16;
        }
        println!("Printing from buffer:\n");
        for (i, sample) in t_buf.into_iter().enumerate() {
            println!("i: {}, sample value: {}", i, sample);
        }

        let buf_as_u8 = as_u8_slice(&t_buf);
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


fn as_u8_slice(slice_i16: &[i16]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice_i16.as_ptr() as *const u8, 
            slice_i16.len() * std::mem::size_of::<u16>(),
        )
    }
}

pub struct Wave {
    header: WavHeader,
    data: [Sample; NSAMPLES as usize],
}

impl Wave {
    pub fn new() -> Self {
        Wave { 
            header: WavHeader::new(),
            data: [0; NSAMPLES as usize], 
        }
    }

    pub fn write_data() {
        todo!()
    }
}

// Config allows you to set the different parameters, such as NCHANNELS, SR, bit depth, etc. 
pub struct Config {

}
