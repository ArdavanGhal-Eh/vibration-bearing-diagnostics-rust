# ⚡ Real-Time Edge Vibration FFT & Bearing Fault Diagnostic Engine (Rust + Python)

A high-performance predictive maintenance and digital signal processing engine developed in **Rust** with companion **Python** spectral analytics. Performs real-time Cooley-Tukey Radix-2 Fast Fourier Transforms (FFT), time-domain statistical indicator extraction (Kurtosis, RMS, Crest Factor), and automated kinematic bearing defect identification (BPFO, BPFI, BSF, FTF) under 100 microseconds per buffer.

---

## 📌 Problem & Engineering Motivation
In rotating machinery (turbines, CNC spindles, EV powertrains, industrial pumps, and wind turbines):
1. **Catastrophic Failure Prevention:** Bearing failure accounts for over 45% of all industrial motor breakdowns. Microscopic fatigue spalling on bearing raceways produces subtle periodic micro-impacts long before audible noise or thermal runaway occurs.
2. **Edge Computing Constraint:** Low-power IIoT edge sensors cannot stream raw 10 kHz - 50 kHz accelerometer waveforms to the cloud due to bandwidth costs; signal processing, Hanning windowing, FFT, and anomaly matching must happen directly on edge microcontrollers.
3. **Rust Memory & Speed Advantage:** Rust delivers C-level execution speed with zero garbage collection spikes, ensuring deterministic real-time processing of high-speed sensor streams.

---

## 🌟 System Architecture

```text
┌───────────────────────────────────────────────┐
│     Raw Accelerometer Vibration Stream x[n]   │
│         (10 kHz - 50 kHz Sampling)            │
└───────────────────────┬───────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────┐
│        Rust Diagnostic Pipeline Core          │
│   (Zero-Allocation, Thread-Safe In-Place)     │
└───────┬───────────────────────────────┬───────┘
        │                               │
        ▼                               ▼
┌──────────────────────────┐    ┌──────────────────────────┐
│ Time-Domain Indicators   │    │  Spectral FFT Engine     │
│ - RMS Acceleration       │    │  - Hanning Windowing     │
│ - Crest Factor           │    │  - Cooley-Tukey Radix-2  │
│ - Kurtosis (Peak Spikes) │    │  - Frequency Resolution  │
└──────────────────────────┘    └──────────────────────────┘
                        │
                        ▼
┌───────────────────────────────────────────────┐
│    Kinematic Defect Matcher & Severity Alarm  │
│    - BPFO (Outer Race), BPFI (Inner Race)     │
│    - Harmonic Peak Detection vs Noise Floor   │
└───────────────────────────────────────────────┘
```

---

## 📐 Mathematical Formulation

### 1. Rolling Element Bearing Defect Frequencies
Given shaft speed $f_r = \frac{\text{RPM}}{60}$, roller diameter $d$, pitch diameter $D$, roller count $Z$, and contact angle $\alpha$:
- **BPFO** (Ball Pass Frequency Outer Race):
  $$f_{BPFO} = \frac{Z}{2} f_r \left( 1 - \frac{d}{D} \cos\alpha \right)$$
- **BPFI** (Ball Pass Frequency Inner Race):
  $$f_{BPFI} = \frac{Z}{2} f_r \left( 1 + \frac{d}{D} \cos\alpha \right)$$
- **BSF** (Ball Spin Frequency):
  $$f_{BSF} = \frac{D}{2 d} f_r \left( 1 - \left(\frac{d}{D}\cos\alpha\right)^2 \right)$$

### 2. Statistical Kurtosis (Impulsive Shock Detector)
$$\text{Kurtosis} = \frac{\frac{1}{N} \sum_{n=1}^N (x_n - \mu)^4}{\sigma^4}$$
*(A healthy Gaussian bearing exhibits $\text{Kurtosis} \approx 3.0$; surface micro-pitting triggers sharp acceleration spikes driving $\text{Kurtosis} > 4.5$)*.

### 3. Fast Fourier Transform (Cooley-Tukey Radix-2)
$$X[k] = \sum_{n=0}^{N/2-1} x[2n] W_{N/2}^{kn} + W_N^k \sum_{n=0}^{N/2-1} x[2n+1] W_{N/2}^{kn}$$

---

## 🎯 Real-World Applications & Cross-Industry Impact

### ⚙️ Mechanical, Automotive & Aerospace Engineering
- **Wind Turbine Gearbox Monitoring:** Early detection of planetary gear and generator bearing degradation in offshore installations.
- **EV Traction Motor Spindle Diagnostics:** Identifying electrical pitting caused by stray inverter shaft currents.
- **Aviation Propulsion Turbines:** Continuous in-flight vibration monitoring of high-pressure shaft support bearings.

### 🌐 Cross-Industry & Software Applications
- **Edge Predictive Maintenance (PdM) Gateways:** Running deterministic Rust firmware on ARM Cortex-M or Raspberry Pi industrial nodes.
- **SCADA & Plant Historian Integration:** Feeding structured health alarms and Kurtosis indices into enterprise ERP/MES platforms.

---

## 🚀 Installation & Benchmark Execution

### 1. Build & Run Rust Computational Core
```bash
cargo build --release
cargo run --release
```

### 2. Generate Spectral Plots & Fault Markers (Python)
```bash
cd python_visualizer
pip install -r requirements.txt
python plot_vibration_spectrum.py
```

---

## 🛠️ Tech Stack
- **Computational Core:** Rust 1.75+, `serde`, in-place complex arithmetic
- **Digital Signal Processing:** Radix-2 Cooley-Tukey FFT, Hanning windowing, kinematic bearing formulas
- **Analytics & Plotting:** Python 3.10+, `numpy`, `matplotlib`

---

## 👨‍💻 Author
**Ardavan Ghal-Eh**  
Mechanical Engineering Student, Sharif University of Technology  
*Focus: Condition Monitoring, Vibration Analysis & High-Performance Computational Systems*
