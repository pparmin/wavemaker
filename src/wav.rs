use std::{fs::File, str::FromStr};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use serde::{Serialize};
use mp4::FourCC;


const M_PI: f64 = 3.14159265358979323846;
const SAMPLE_MAX: u16 = 32767;

#[derive(Serialize, PartialEq, Debug)]
struct RiffHeader {
    id: FourCC,
    size: u32,
    ftype: FourCC,
}

impl RiffHeader {
    fn new(nsamples: u32, sample_size: u16) -> Self {
        RiffHeader {
            id: FourCC::from_str("RIFF").unwrap(),
            size: 36 + nsamples * sample_size as u32,
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
    fn new(sample_rate: u32, nchannels: u16, sample_size: u16, bits_per_sample: u16) -> Self {
        FmtChunk {
            id: FourCC::from_str("fmt ").unwrap(),
            size: 16,
            fmt_tag: 1,
            channels: nchannels,
            samples_per_sec: sample_rate,
            bytes_per_sec: nchannels as u32 * sample_rate * sample_size as u32,
            block_align: nchannels * sample_size,
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
    fn new(nsamples: u32, sample_size: u16) -> Self {
        DataHeader {
            id: FourCC::from_str("data").unwrap(),
            size: nsamples * sample_size as u32,
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
struct WavHeader {
    riff_header: RiffHeader,
    fmt_chunk: FmtChunk,
    data_header: DataHeader,
}

impl WavHeader {
    fn new(config: &Config) -> Self {
        WavHeader {
            riff_header: RiffHeader::new(config.nsamples, config.sample_size),
            fmt_chunk: FmtChunk::new(config.sample_rate, config.channels, config.sample_size, config.bits_per_sample),
            data_header: DataHeader::new(config.nsamples, config.sample_size),
        }
    }
}


#[derive(Serialize, PartialEq, Debug)]
pub struct Wave<'a> {
    header: WavHeader,
    config: &'a Config,
}

impl<'a> Wave<'a> {
    pub fn new(config: &'a Config) -> Self {
        Wave { 
            header: WavHeader::new(config),
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

    pub fn write_data(&self, p: &str, frequency: f64) {
        let mut buf = Vec::new();
        let amplitude = 0.2;
        for i in 0 .. self.config.nsamples {
            let sample = SAMPLE_MAX as f64 * amplitude * (2.0 * M_PI * frequency * i as f64/self.config.sample_rate as f64).sin();
            buf.push(sample as i16);
            //println!("SAMPLE as FLOAT: {} - SAMPLE AS i16: {}", sample, sample as i16)
        }
        println!("Printing from buffer:\n");
    
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

    pub fn read_data(&self, p: &str) {
        // 1. extract metadata from header 
        // 2. write data to buffer
        let mut buf = Vec::new();
        let mut file = File::open(p).unwrap();

        let n = match file.read_to_end(&mut buf) {
            Ok(n) => {
                println!("Successfully read file \"{}\"", p);
                n
            },
            Err(why) => panic!("the data could not be read into the buffer: {}", why)
        };
        println!("Length of buffer: {}", n);

        let buf_as_i16 = as_i16_slice(&buf);
        
        println!("DATA SECTION");
        for sample in buf_as_i16 {
        println!("SAMPLE VALUE as hex {:x} - dec {}", sample, sample);
        }
    }
}

#[derive(Serialize, PartialEq, Debug)]
pub struct Config {
    pub sample_size: u16,
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub nsamples: u32,
    pub duration: u32,
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

fn as_i16_slice(slice_u8: &[u8]) -> Vec<i16> {
    let mut temp: [u8; 2] = [0, 0];
    let mut counter = 0;
    let mut sample: i16;

    let mut slice_i16: Vec<i16> = vec![];

    for (i, v) in slice_u8.iter().enumerate() {
        let byte = i+1;
        //println!("Byte #{} - value (char): {} - value (hex): {:x}  value (float): {}", i+1, *v as char, *v, *v as f64);        
        /* IMPORTANT 
         * Checking hexedit reveals that the actual sample data only begins by byte 65:
         */
        if byte > 64 {
            if counter == 0 {
                temp[0] = *v;
                counter += 1;

            } else if counter == 1 {
                temp[1] = *v; 
                sample = i16::from_le_bytes(temp); 
                counter = 0;
                temp = [0, 0];
                slice_i16.push(sample);
            } 
        }
    }
    slice_i16
}

fn as_u8_slice(slice_i16: &[i16]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice_i16.as_ptr() as *const u8, 
            slice_i16.len() * std::mem::size_of::<u16>(),
        )
    }
}
