use std::f64::consts::PI;
use vibration_bearing_diagnostics_rust::{
    BearingGeometry, BearingKinematics, FftProcessor, VibrationDiagnostics,
};

#[test]
fn test_bearing_kinematic_fault_frequencies() {
    let geom = BearingGeometry::skf_6205();
    let rpm = 1800.0; // 30 Hz shaft frequency
    let freqs = BearingKinematics::calculate_fault_frequencies(&geom, rpm);

    assert_eq!(freqs.shaft_speed_hz, 30.0);
    // BPFO = (Z/2) * fr * (1 - d/D) = 4.5 * 30 * (1 - 7.94/39.04) ~ 107.5 Hz
    assert!((freqs.bpfo_hz - 107.5).abs() < 1.0);
    // BPFI = (Z/2) * fr * (1 + d/D) = 4.5 * 30 * (1 + 7.94/39.04) ~ 162.5 Hz
    assert!((freqs.bpfi_hz - 162.5).abs() < 1.0);
    assert!(freqs.bsf_hz > 0.0);
    assert!(freqs.ftf_hz > 0.0 && freqs.ftf_hz < freqs.shaft_speed_hz);
}

#[test]
fn test_fft_recovers_known_frequency_and_amplitude() {
    let n = 256;
    let fs = 256.0; // 1 Hz resolution
    let target_freq = 40.0;
    let amplitude = 5.0;

    let mut signal = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / fs;
        signal.push(amplitude * (2.0 * PI * target_freq * t).cos());
    }

    let spectrum = FftProcessor::compute_power_spectrum(&signal);
    assert_eq!(spectrum.len(), n / 2);

    // Peak should be at bin index 40
    let mut max_idx = 0;
    let mut max_val = 0.0;
    for (idx, &val) in spectrum.iter().enumerate() {
        if val > max_val {
            max_val = val;
            max_idx = idx;
        }
    }

    assert_eq!(max_idx, 40);
    assert!((max_val - amplitude).abs() < 0.2);
}

#[test]
fn test_time_domain_stats() {
    // Zero-mean square wave signal [-2, 2, -2, 2, ...]
    let mut signal = Vec::new();
    for i in 0..100 {
        signal.push(if i % 2 == 0 { 2.0 } else { -2.0 });
    }

    let (rms, peak, crest_factor, kurtosis) = VibrationDiagnostics::compute_time_domain_stats(&signal);

    assert_eq!(peak, 2.0);
    assert!((rms - 2.0).abs() < 1e-4);
    assert!((crest_factor - 1.0).abs() < 1e-4);
    assert!(kurtosis >= 1.0);
}

#[test]
fn test_diagnostics_inner_race_defect() {
    let geom = BearingGeometry::skf_6205();
    let fault_freqs = BearingKinematics::calculate_fault_frequencies(&geom, 1800.0);

    // Create a 1000-bin spectrum with low noise floor (0.1) and a sharp spike at BPFI (~162.5 Hz)
    let freq_res = 1.0;
    let mut spectrum = vec![0.1; 500];
    let bpfi_bin = fault_freqs.bpfi_hz.round() as usize;
    spectrum[bpfi_bin] = 4.0; // High harmonic peak

    let report = VibrationDiagnostics::diagnose_bearing(
        &spectrum,
        freq_res,
        &fault_freqs,
        4.8, // Elevated kurtosis
        1.5,
        5.2,
        3.4,
    );

    assert!(report.detected_fault.contains("خرابی رینگ داخلی"));
    assert!(report.confidence_score_percent > 90.0);
}
