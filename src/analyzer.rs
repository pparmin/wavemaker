#![allow(dead_code)]

pub struct Analyzer {
    pub data: Vec<i16>
}

impl Analyzer {
    pub fn calc_freq(self) {
        let offset: usize = 20;
        let mut i = 0;
        let mut diff_signal: Vec<i16> = vec![];
        println!("data length: {}", self.data.len());
        while i+offset < self.data.len() {
            println!("sample #{}: Original signal {} -- offset index {}: offset signal {}", i, self.data[i], i+offset, self.data[i+offset]);
            if i >= self.data.len() {
                println!("index for offset signal is about to exceed original array...breaking");
                break 
            }
            diff_signal.push((self.data[i] - self.data[i+offset]).abs());
            i += 1;
        }
        println!("printing difference signal...");
        for (i, s) in diff_signal.iter().enumerate() {
            println!("val #{}: {}", i, s)
        }
    }
}