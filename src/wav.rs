#![allow(unused_variables)]
#![allow(dead_code)]

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

    fn from_bytes(buf: &[u8]) -> Self {
        let riff_id = std::str::from_utf8(&buf[0 .. 4])
            .expect("Error while converting from bytes to string");
        
        let buf_as_array = <&[u8; 4]>::try_from(&buf[4 .. 8]).expect("Error converting from slice to array");
        let size = u32::from_le_bytes(*buf_as_array);

        let riff_ftype = std::str::from_utf8(&buf[8..])
        .expect("Error while converting from bytes to string");
        
        RiffHeader{ 
            id: FourCC::from_str(riff_id).unwrap(),
            size, 
            ftype: FourCC::from_str(riff_ftype).unwrap()
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

    fn from_bytes(buf: &[u8]) -> Self {
        let fmt_id = std::str::from_utf8(&buf[0 .. 4])
            .expect("Error while converting from bytes to string");

        let buf_as_array = <&[u8; 4]>::try_from(&buf[4 .. 8]).expect("Error converting from slice to array");
        let size = u32::from_le_bytes(*buf_as_array);

        let buf_as_array = <&[u8; 2]>::try_from(&buf[8 .. 10]).expect("Error converting from slice to array");
        let fmt_tag = u16::from_le_bytes(*buf_as_array);
        
        let buf_as_array = <&[u8; 2]>::try_from(&buf[10 .. 12]).expect("Error converting from slice to array");
        let channels = u16::from_le_bytes(*buf_as_array);

        let buf_as_array = <&[u8; 4]>::try_from(&buf[12 .. 16]).expect("Error converting from slice to array");
        let samples_per_sec = u32::from_le_bytes(*buf_as_array);

        let buf_as_array = <&[u8; 4]>::try_from(&buf[16 .. 20]).expect("Error converting from slice to array");
        let bytes_per_sec = u32::from_le_bytes(*buf_as_array);

        let buf_as_array = <&[u8; 2]>::try_from(&buf[20 .. 22]).expect("Error converting from slice to array");
        let block_align = u16::from_le_bytes(*buf_as_array);

        let buf_as_array = <&[u8; 2]>::try_from(&buf[22..]).expect("Error converting from slice to array");
        let bits_per_sample = u16::from_le_bytes(*buf_as_array);

        FmtChunk {
            id: FourCC::from_str(fmt_id).unwrap(),
            size, 
            fmt_tag,
            channels,
            samples_per_sec,
            bytes_per_sec,
            block_align,
            bits_per_sample
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

    fn from_bytes(buf: &[u8]) -> Self {
        let data_id = std::str::from_utf8(&buf[0 .. 4])
        .expect("Error while converting from bytes to string");

        let buf_as_array = <&[u8; 4]>::try_from(&buf[4..]).expect("Error converting from slice to array");
        let size = u32::from_le_bytes(*buf_as_array);

        DataHeader {
            id: FourCC::from_str(data_id).unwrap(),
            size
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

    fn from_bytes(buf: [u8; 44]) -> Self {
        WavHeader {
            riff_header: RiffHeader::from_bytes(&buf[0 .. 11]),
            fmt_chunk: FmtChunk::from_bytes(&buf[12 .. 35]),
            data_header: DataHeader::from_bytes(&buf[36 .. 43]),
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

        // println!("printing from encoded header");
        // for (i, b) in encoded.iter().enumerate() {
        //     println!("Byte #{}: {:x}", i, b);
        // }
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
        }
        println!("Printing from buffer:\n");
    
        let buf_as_u8 = samples_as_u8(&buf);
        // for (i, byte) in buf_as_u8.iter().enumerate() {
        //     println!("Byte #{}: {:x}", i, byte);
        // }
        let mut file = OpenOptions::new().
        append(true).
        open(p).
        unwrap(); 
    
        match file.write_all(&buf_as_u8) {
            Err(why) => panic!("couldn't write sample data to {}: {}", p, why),
            Ok(_) => println!("successfully appended sample data to {}", p),
        }
    }

    pub fn read_header(&self, p: &str) -> [u8; 62] {
        const BYTES_HEADER: usize = 62;
        let mut header_data: [u8; BYTES_HEADER] = [0; BYTES_HEADER];
        let mut file = File::open(p).unwrap();

        match file.read_exact(&mut header_data) {
            Err(why) => panic!("couldn't read wav header data into buffer: {}", why), 
            Ok(_) => {}
        };

        for (i, b) in header_data.iter().enumerate() {
            println!("Byte #{}: {:x} as char {}", i+1, b, char::from(*b));
        }
        header_data
    }

    /* 
     * Read data currently only reads the sample data into a buffer 
     * The header data is currently not extracted 
     */
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

        let buf_as_i16 = samples_as_i16(&buf);
        
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

/* 
 * We have to convert our sample data, which is i16, to u8 when writing a .wav
 * and back to i16 from u8 when reading a .wav. The following two functions take
 * care of this. 
 * 
 * FUTURE REFACTOR: 
 * Rename as_i6_slice to samples_in_i16 & samples_as_u8 samples_in_u8
*/
fn samples_as_i16(slice_u8: &[u8]) -> Vec<i16> {
    let mut temp: [u8; 2] = [0, 0];
    let mut counter = 0;
    let mut sample: i16;

    let mut slice_i16: Vec<i16> = vec![];

    for (i, v) in slice_u8.iter().enumerate() {
        let byte = i+1;
        /* IMPORTANT 
         * Checking hexedit reveals that the actual sample data begins by byte 63:
         */
        if byte > 62 {
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

fn samples_as_u8(slice_i16: &[i16]) -> Vec<u8> {
    let mut slice_u8: Vec<u8> = Vec::new();

    for sample in slice_i16.iter() {
        let sample_bytes = sample.to_le_bytes();
        for b in sample_bytes {
            slice_u8.push(b);
        }
    }
    slice_u8
}
