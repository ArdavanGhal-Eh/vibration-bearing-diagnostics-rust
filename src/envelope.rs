// Hilbert Transform Envelope Demodulation (High-Frequency Resonance Technique - HFRT)
// Part of Edge Vibration FFT & Bearing Diagnostics Suite in Rust
// Author: Ardavan Ghal-Eh | Sharif University of Technology

use crate::fft::{Complex, FftProcessor};

pub struct EnvelopeDemodulator;

impl EnvelopeDemodulator {
    // Computes analytic signal z[n] = x[n] + j * H{x[n]} using frequency-domain Hilbert transform
    pub fn extract_envelope(signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        assert!(n.is_power_of_two(), "Signal length must be a power of two");

        let mut complex_sig: Vec<Complex> = signal.iter().map(|&s| Complex::new(s, 0.0)).collect();

        // 1. Forward FFT
        FftProcessor::compute_fft(&mut complex_sig);

        // 2. Frequency-domain Hilbert multiplier:
        // H[0] = 1, H[N/2] = 1
        // H[1..N/2-1] = 2
        // H[N/2+1..N-1] = 0
        let half = n / 2;
        for i in 1..half {
            complex_sig[i].re *= 2.0;
            complex_sig[i].im *= 2.0;
        }
        for i in (half + 1)..n {
            complex_sig[i].re = 0.0;
            complex_sig[i].im = 0.0;
        }

        // 3. Inverse FFT (via conjugate trick: IFFT(X) = conj(FFT(conj(X))) / N)
        for c in complex_sig.iter_mut() {
            c.im = -c.im;
        }
        FftProcessor::compute_fft(&mut complex_sig);
        for c in complex_sig.iter_mut() {
            c.im = -c.im / (n as f64);
            c.re /= n as f64;
        }

        // 4. Instantaneous envelope magnitude
        complex_sig.iter().map(|c| c.magnitude()).collect()
    }
}
