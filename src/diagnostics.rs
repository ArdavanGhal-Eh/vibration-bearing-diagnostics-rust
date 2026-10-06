// Statistical Health Indicators & Peak Harmonics Fault Detection
// Author: Ardavan Ghal-Eh | Sharif University of Technology

use crate::bearing::FaultFrequencies;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiagnosticReport {
    pub rms_acceleration_g: f64,
    pub peak_acceleration_g: f64,
    pub crest_factor: f64,
    pub kurtosis: f64,
    pub detected_fault: String,
    pub fault_severity: String,
    pub confidence_score_percent: f64,
}

pub struct VibrationDiagnostics;

impl VibrationDiagnostics {
    pub fn compute_time_domain_stats(signal: &[f64]) -> (f64, f64, f64, f64) {
        let n = signal.len() as f64;
        if n < 2.0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let mean = signal.iter().sum::<f64>() / n;
        let mut sum_sq_diff = 0.0;
        let mut sum_fourth_diff = 0.0;
        let mut peak = 0.0;

        for &val in signal {
            let abs_val = val.abs();
            if abs_val > peak {
                peak = abs_val;
            }
            let diff = val - mean;
            sum_sq_diff += diff * diff;
            sum_fourth_diff += diff * diff * diff * diff;
        }

        let variance = sum_sq_diff / n;
        let rms = variance.sqrt();
        let crest_factor = if rms > 1e-6 { peak / rms } else { 0.0 };

        let kurtosis = if variance > 1e-6 {
            (sum_fourth_diff / n) / (variance * variance)
        } else {
            3.0
        };

        (rms, peak, crest_factor, kurtosis)
    }

    pub fn diagnose_bearing(
        spectrum: &[f64],
        freq_resolution_hz: f64,
        fault_freqs: &FaultFrequencies,
        kurtosis: f64,
        rms: f64,
        peak: f64,
        crest_factor: f64,
    ) -> DiagnosticReport {
        // Average spectrum noise floor
        let avg_floor = spectrum.iter().sum::<f64>() / spectrum.len() as f64;

        let find_peak_near = |target_hz: f64, tolerance: f64| -> f64 {
            let min_f = target_hz * (1.0 - tolerance);
            let max_f = target_hz * (1.0 + tolerance);
            let min_idx = (min_f / freq_resolution_hz).floor() as usize;
            let max_idx = (max_f / freq_resolution_hz).ceil() as usize;

            let mut max_mag = 0.0;
            for i in min_idx..=max_idx.min(spectrum.len() - 1) {
                if spectrum[i] > max_mag {
                    max_mag = spectrum[i];
                }
            }
            max_mag
        };

        // Check BPFO & BPFI peaks
        let bpfo_peak = find_peak_near(fault_freqs.bpfo_hz, 0.03);
        let bpfi_peak = find_peak_near(fault_freqs.bpfi_hz, 0.03);

        let bpfo_ratio = bpfo_peak / avg_floor;
        let bpfi_ratio = bpfi_peak / avg_floor;

        let mut fault = "✅ بیرینگ سالم (Healthy Bearing)".to_string();
        let mut severity = "Normal".to_string();
        let mut conf = 95.0;

        if bpfi_ratio > 3.5 || (kurtosis > 4.5 && bpfi_ratio > 2.5) {
            fault = "🔴 خرابی رینگ داخلی (Inner Race Defect - BPFI Harmonics)".to_string();
            severity = if bpfi_ratio > 6.0 { "Critical (تعویض فوری)" } else { "Warning (پایش هفتگی)" }.to_string();
            conf = 92.0;
        } else if bpfo_ratio > 3.5 || (kurtosis > 4.0 && bpfo_ratio > 2.5) {
            fault = "🟠 خرابی رینگ خارجی (Outer Race Defect - BPFO Harmonics)".to_string();
            severity = if bpfo_ratio > 6.0 { "Critical (تعویض فوری)" } else { "Warning (پایش هفتگی)" }.to_string();
            conf = 94.0;
        } else if kurtosis > 4.2 {
            fault = "🟡 ضربات نامنظم یا عدم روان‌کاری (Lubrication / Spalling Risk)".to_string();
            severity = "Moderate (روان‌کاری و بررسی مجدد)".to_string();
            conf = 85.0;
        }

        DiagnosticReport {
            rms_acceleration_g: (rms * 100.0).round() / 100.0,
            peak_acceleration_g: (peak * 100.0).round() / 100.0,
            crest_factor: (crest_factor * 100.0).round() / 100.0,
            kurtosis: (kurtosis * 100.0).round() / 100.0,
            detected_fault: fault,
            fault_severity: severity,
            confidence_score_percent: conf,
        }
    }
}
