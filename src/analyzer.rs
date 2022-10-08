#![allow(dead_code)]

pub struct Analyzer {
    pub data: Vec<i16>
}

#[derive(Debug)]
pub struct Minimum {
    position: usize,
    value: f32
}

impl Analyzer {
    pub fn calc_freq(self) {
        println!("data length: {}", self.data.len());

        let amdf_data = self.amdf();

        let local_minima = find_peaks(amdf_data);
        for m in &local_minima {
            println!("Minimum: {:?}", *m);  
        }

        let period = local_minima[0].position;
        println!("Period in samples: {}", period);
        let freq = 1 as f32 / period as f32;
        println!("frequency: {} Hz", freq * 44100.0);
    }

    fn amdf(self) -> Vec<f32> {
        let mut amdf: Vec<f32> = vec![];

        // First we vary the offset (k) in a range of L - 1 
        // The meaning of the upper bound of the summation, L- offset (k) - 1, is to prevent 
        // the index from accessing values outside the sample data vector
        for offset in 0..self.data.len() - 1  {
            let mut sum: i32 = 0;
            let length = self.data.len() - offset; 
            
            // then we calculate the difference signal between the original signal and all
            // offset signals and sum them up
            for n in 0..self.data.len() - offset - 1 {
                let diff_signal = (self.data[n] - self.data[n+offset]).abs() as i32;
                sum += diff_signal;
            }
            // the result will be the sum divided by L - k
            let result = sum as f32 / length as f32;
            amdf.push(result);
        }
        amdf
    }
}

pub fn find_peaks(amdf: Vec<f32>) -> Vec<Minimum> {
    let mut local_minima: Vec<Minimum> = vec![];
    let mut minimum;  
    for i in 0..amdf.len() {
        if i == 0 {
            continue
        } else if i == amdf.len() - 2 {
            break
        }
        // current   previous                 next
        if amdf[i] < amdf[i - 1] && amdf[i] < amdf[i + 1] {
            minimum = amdf[i];
            let p = Minimum {
                position: i,
                value: minimum
            };
            local_minima.push(p);
            println!("new local minimum found at pos {} -- val: {}", i, minimum);
        } 
    }
    local_minima
}