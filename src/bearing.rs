// Rolling Element Bearing Kinematic Fault Frequencies
// Author: Ardavan Ghal-Eh | Sharif University of Technology

use std::f64::consts::PI;

#[derive(Clone, Debug)]
pub struct BearingGeometry {
    pub name: String,
    pub num_rollers: usize,    // Z
    pub roller_diameter_mm: f64, // d
    pub pitch_diameter_mm: f64,  // D
    pub contact_angle_deg: f64,  // alpha
}

impl BearingGeometry {
    pub fn skf_6205() -> Self {
        Self {
            name: "SKF 6205 (Deep Groove Ball Bearing)".to_string(),
            num_rollers: 9,
            roller_diameter_mm: 7.94,
            pitch_diameter_mm: 39.04,
            contact_angle_deg: 0.0,
        }
    }

    pub fn skf_6308() -> Self {
        Self {
            name: "SKF 6308 (Heavy Duty Industrial)".to_string(),
            num_rollers: 8,
            roller_diameter_mm: 15.08,
            pitch_diameter_mm: 65.0,
            contact_angle_deg: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FaultFrequencies {
    pub shaft_speed_hz: f64, // fr
    pub bpfo_hz: f64,        // Ball Pass Frequency Outer Race
    pub bpfi_hz: f64,        // Ball Pass Frequency Inner Race
    pub bsf_hz: f64,         // Ball Spin Frequency
    pub ftf_hz: f64,         // Fundamental Train Frequency (Cage)
}

pub struct BearingKinematics;

impl BearingKinematics {
    pub fn calculate_fault_frequencies(geom: &BearingGeometry, rpm: f64) -> FaultFrequencies {
        let fr = rpm / 60.0;
        let z = geom.num_rollers as f64;
        let d = geom.roller_diameter_mm;
        let capital_d = geom.pitch_diameter_mm;
        let alpha = geom.contact_angle_deg * (PI / 180.0);
        let cos_alpha = alpha.cos();

        let ratio = (d / capital_d) * cos_alpha;

        let bpfo = (z / 2.0) * fr * (1.0 - ratio);
        let bpfi = (z / 2.0) * fr * (1.0 + ratio);
        let bsf = (capital_d / (2.0 * d)) * fr * (1.0 - ratio * ratio);
        let ftf = 0.5 * fr * (1.0 - ratio);

        FaultFrequencies {
            shaft_speed_hz: fr,
            bpfo_hz: bpfo,
            bpfi_hz: bpfi,
            bsf_hz: bsf,
            ftf_hz: ftf,
        }
    }
}
