"""
Vibration Waveform & FFT Spectrum Visualizer with Bearing Fault Markers
Part of Edge Vibration FFT & Bearing Diagnostics Suite in Rust
Author: Ardavan Ghal-Eh | Sharif University of Technology
"""

import numpy as np
import matplotlib.pyplot as plt


def plot_bearing_vibration_analysis():
    fs = 5000.0
    n = 2048
    t = np.linspace(0, (n - 1) / fs, n)
    dt = 1.0 / fs

    # Fault frequencies for SKF 6205 at 1750 RPM
    fr = 1750.0 / 60.0  # 29.17 Hz
    bpfo = 104.3        # Hz
    bpfi = 158.2        # Hz

    # Synthetic signal
    carrier = np.sin(2 * np.pi * 1200.0 * t)
    impacts = (np.sin(2 * np.pi * bpfi * t)) ** 8
    signal = 0.8 * np.sin(2 * np.pi * fr * t) + 2.5 * impacts * carrier + np.random.normal(0, 0.15, n)

    # FFT
    window = np.hanning(n)
    spectrum = np.abs(np.fft.rfft(signal * window)) * (2.0 / n)
    freqs = np.fft.rfftfreq(n, d=dt)

    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(13, 7), gridspec_kw={'height_ratios': [1, 1.2]})

    # 1. Time-Domain Acceleration Waveform
    ax1.plot(t[:400] * 1000, signal[:400], color='navy', linewidth=1.2, label='Vibration Signal (Acceleration g)')
    ax1.set_title("Time-Domain Acceleration Waveform (Transient Shock Impacts Visible)", fontsize=11, fontweight='bold')
    ax1.set_xlabel("Time (milliseconds)")
    ax1.set_ylabel("Acceleration (g)")
    ax1.grid(True, linestyle=':', alpha=0.6)
    ax1.legend(loc='upper right')

    # 2. FFT Spectrum with Bearing Fault Markers
    ax2.plot(freqs, spectrum, color='darkslategray', linewidth=1.2, label='Vibration Power Spectrum')
    
    # Fault lines
    ax2.axvline(bpfi, color='red', linestyle='--', linewidth=1.6, label=f'BPFI Harmonic 1X ({bpfi:.1f} Hz)')
    ax2.axvline(2 * bpfi, color='darkred', linestyle=':', linewidth=1.4, label=f'BPFI Harmonic 2X ({2*bpfi:.1f} Hz)')
    ax2.axvline(bpfo, color='orange', linestyle='--', linewidth=1.2, label=f'BPFO Marker ({bpfo:.1f} Hz)')

    ax2.set_xlim(0, 800)
    ax2.set_title("Vibration FFT Spectrum & Bearing Fault Diagnosis (BPFI Peak Confirmation)", fontsize=11, fontweight='bold')
    ax2.set_xlabel("Frequency (Hz)")
    ax2.set_ylabel("Magnitude (g)")
    ax2.grid(True, linestyle=':', alpha=0.6)
    ax2.legend(loc='upper right')

    plt.tight_layout()
    output_png = "/working_dir/c_db360fcc6464ba55/daily_projects_day7/vibration-bearing-diagnostics-rust/python_visualizer/bearing_fault_fft.png"
    plt.savefig(output_png, dpi=200)
    print("✅ Bearing FFT Visualization generated:", output_png)


if __name__ == "__main__":
    plot_bearing_vibration_analysis()
