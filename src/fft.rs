// Cooley-Tukey Radix-2 FFT and Windowing Functions
// Author: Ardavan Ghal-Eh | Sharif University of Technology

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    #[inline(always)]
    pub fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }

    #[inline(always)]
    pub fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }

    #[inline(always)]
    pub fn magnitude(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

pub struct FftProcessor;

impl FftProcessor {
    pub fn apply_hanning_window(signal: &mut [f64]) {
        let n = signal.len();
        if n <= 1 { return; }
        for (i, val) in signal.iter_mut().enumerate() {
            let multiplier = 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos());
            *val *= multiplier;
        }
    }

    // In-place Radix-2 Decimation-In-Time (DIT) FFT
    pub fn compute_fft(data: &mut [Complex]) {
        let n = data.len();
        assert!(n.is_power_of_two(), "FFT size must be a power of two");

        // Bit-reversal permutation
        let mut j = 0;
        for i in 0..n {
            if i < j {
                data.swap(i, j);
            }
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;
        }

        // Cooley-Tukey butterfly computations
        let mut len = 2;
        while len <= n {
            let half = len / 2;
            let angle = -2.0 * PI / len as f64;
            let w_step = Complex::new(angle.cos(), angle.sin());

            let mut i = 0;
            while i < n {
                let mut w = Complex::new(1.0, 0.0);
                for k in 0..half {
                    let u = data[i + k];
                    let v = data[i + k + half].mul(w);
                    data[i + k] = u.add(v);
                    data[i + k + half] = u.sub(v);
                    w = w.mul(w_step);
                }
                i += len;
            }
            len <<= 1;
        }
    }

    pub fn compute_power_spectrum(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        let mut complex_buffer: Vec<Complex> = signal.iter().map(|&s| Complex::new(s, 0.0)).collect();
        Self::compute_fft(&mut complex_buffer);

        let half = n / 2;
        let mut spectrum = Vec::with_capacity(half);
        for item in complex_buffer.iter().take(half) {
            let mag = item.magnitude() * (2.0 / n as f64);
            spectrum.push(mag);
        }
        spectrum
    }
}
