use vibration_diagnostics::{
    BearingGeometry, BearingKinematics, FftProcessor, VibrationDiagnostics,
};
use std::f64::consts::PI;
use std::time::Instant;

fn main() {
    println!("============================================================");
    println!("⚡ Real-Time Edge Vibration FFT & Bearing Diagnostic Engine (Rust)");
    println!("   Author: Ardavan Ghal-Eh | Sharif University of Tech");
    println!("============================================================");

    // 1. Shaft and Bearing Setup (SKF 6205 at 1750 RPM)
    let rpm = 1750.0;
    let bearing = BearingGeometry::skf_6205();
    let fault_freqs = BearingKinematics::calculate_fault_frequencies(&bearing, rpm);

    println!("Bearing: {}", bearing.name);
    println!("Shaft Speed: {:.1} RPM (Rotational Freq: {:.2} Hz)", rpm, fault_freqs.shaft_speed_hz);
    println!("------------------------------------------------------------");
    println!("Theoretical Kinematic Fault Frequencies:");
    println!("  - BPFO (Outer Race): {:.2} Hz", fault_freqs.bpfo_hz);
    println!("  - BPFI (Inner Race): {:.2} Hz", fault_freqs.bpfi_hz);
    println!("  - BSF  (Ball Spin):  {:.2} Hz", fault_freqs.bsf_hz);
    println!("  - FTF  (Cage):       {:.2} Hz", fault_freqs.ftf_hz);
    println!("------------------------------------------------------------");

    // 2. Synthesize High-Sampling Vibration Stream with Inner-Race Pitting Defect
    let sample_rate_hz = 5000.0; // 5 kHz sampling
    let n_samples = 2048;        // Radix-2 buffer
    let dt = 1.0 / sample_rate_hz;

    let mut raw_signal = vec![0.0; n_samples];
    for (i, sample) in raw_signal.iter_mut().enumerate() {
        let t = i as f64 * dt;
        // Background rotational unbalance & motor 1X harmonic
        let unbalance = 0.8 * (2.0 * PI * fault_freqs.shaft_speed_hz * t).sin();
        // High-frequency resonance carrier (structural natural freq ~ 1200 Hz)
        let carrier = (2.0 * PI * 1200.0 * t).sin();
        // Impulsive impact bursts repeating at BPFI (~158 Hz)
        let impact_mod = (2.0 * PI * fault_freqs.bpfi_hz * t).sin().powi(8);
        // Random white noise (sensor noise floor)
        let noise = 0.15 * (t * 12345.67).sin();

        *sample = unbalance + 2.5 * impact_mod * carrier + noise;
    }

    // 3. Time-Domain Indicators
    let (rms, peak, crest_factor, kurtosis) = VibrationDiagnostics::compute_time_domain_stats(&raw_signal);

    // 4. Benchmarked In-Place Windowing & FFT Execution
    let start_time = Instant::now();
    let mut windowed_signal = raw_signal.clone();
    FftProcessor::apply_hanning_window(&mut windowed_signal);
    let spectrum = FftProcessor::compute_power_spectrum(&windowed_signal);
    let elapsed = start_time.elapsed();

    let freq_resolution = sample_rate_hz / n_samples as f64;

    // 5. Automated Diagnostic Assessment
    let report = VibrationDiagnostics::diagnose_bearing(
        &spectrum,
        freq_resolution,
        &fault_freqs,
        kurtosis,
        rms,
        peak,
        crest_factor,
    );

    println!("📊 Time-Domain Health Indicators:");
    println!("  - RMS Acceleration:  {:.2} g", report.rms_acceleration_g);
    println!("  - Peak Acceleration: {:.2} g", report.peak_acceleration_g);
    println!("  - Crest Factor:      {:.2}", report.crest_factor);
    println!("  - Kurtosis Index:    {:.2} (Normal ~3.0, Damaged >4.5)", report.kurtosis);
    println!("------------------------------------------------------------");
    println!("🔍 Diagnostic Verdict:");
    println!("  - Status:   {}", report.detected_fault);
    println!("  - Severity: {}", report.fault_severity);
    println!("  - Confidence Score: {:.1}%", report.confidence_score_percent);
    println!("------------------------------------------------------------");
    println!("⏱️ Execution Performance: 2048-point FFT + Windowing in {:?}", elapsed);
    println!("🚀 Edge Processing Capability: {:.0} FFT buffers/sec", 1.0 / elapsed.as_secs_f64());
    println!("============================================================");
}
