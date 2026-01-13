//! Verified Algorithm for mmWave Radiotherapy Setup (Bressler et al. 2024).
//!
//! This example simulates a radar signal reflected from a target and runs the verified
//! processing pipeline: Coarse FFT -> Fine CZT -> Phase Extraction -> Displacement.

use math_explorer::physics::medical::radar_gating::{FmcwConfig, chirp_z_transform, C};
use num_complex::Complex;
use std::f64::consts::PI;

fn main() {
    println!("=== mmWave Radiotherapy Setup Verification ===");

    // 1. Simulation Setup
    // Use TI IWR6843 default config (4 GHz bandwidth, 60 GHz center freq)
    let config = FmcwConfig::iwr6843_default();
    println!("Config: {:#?}", config);
    println!("Range Resolution (FFT): {:.4} m", config.range_resolution());

    // Simulation Parameters
    // We increase sample rate to 10 MHz to handle the ~266 kHz beat frequency without aliasing.
    let sample_rate = 10.0e6; // 10 MHz ADC
    // Number of samples per chirp determined by chirp duration (50us)
    let n_samples = (sample_rate * config.chirp_time).round() as usize;
    println!("Sample Rate: {:.2} MHz", sample_rate / 1.0e6);
    println!("Samples per Chirp: {}", n_samples);

    // Use actual chirp time from config as the observation window
    let sim_chirp_time = config.chirp_time;

    // Target Definition
    let true_range = 0.5; // Target at 0.5 meters
    let displacement_mm = 0.1; // Small displacement of 0.1 mm
    let true_displacement = displacement_mm / 1000.0;

    println!("\n--- Simulation ---");
    println!("True Range: {:.6} m", true_range);
    println!("True Displacement: {:.6} mm", displacement_mm);

    // 2. Generate Synthetic IF Signal
    // Beat frequency for the static range: f_b = 2 * B * R / (c * T)
    let beat_freq = (2.0 * config.bandwidth * true_range) / (C * sim_chirp_time);
    println!("Expected Beat Frequency: {:.2} Hz", beat_freq);

    // Create signal with phase shift corresponding to displacement
    // Phase shift dphi = 4 * pi * d / lambda
    let lambda = config.wavelength();
    let phase_shift = (4.0 * PI * true_displacement) / lambda;
    println!("Expected Phase Shift: {:.6} rad", phase_shift);

    let signal: Vec<Complex<f64>> = (0..n_samples)
        .map(|n| {
            let t = n as f64 / sample_rate; // Time within chirp
            // IF signal: exp(i * (2*pi*f_b*t + phase_shift))
            let phase = 2.0 * PI * beat_freq * t + phase_shift;
            Complex::new(0.0, phase).exp()
        })
        .collect();

    // 3. Step 1: Coarse Search (FFT)
    println!("\n--- Step 1: Coarse Search (FFT) ---");
    // We use CZT over full bandwidth 0 -> sample_rate / 2 (Nyquist) or just full sample_rate.
    // Standard FFT covers 0 to Fs.
    let fft_output = chirp_z_transform(&signal, 0.0, sample_rate, sample_rate, n_samples);

    // Find peak magnitude
    let mut max_mag = -1.0;
    let mut peak_idx = 0;
    for (i, val) in fft_output.iter().enumerate() {
        if val.norm() > max_mag {
            max_mag = val.norm();
            peak_idx = i;
        }
    }

    let coarse_freq = peak_idx as f64 * sample_rate / n_samples as f64;
    // Calculate range from coarse frequency using Eq (1)
    let coarse_range = config.range_from_beat_frequency(coarse_freq);

    println!("Peak Index: {}", peak_idx);
    println!("Coarse Frequency: {:.2} Hz", coarse_freq);
    println!("Coarse Range: {:.4} m", coarse_range);
    println!("Error: {:.4} m", (coarse_range - true_range).abs());


    // 4. Step 2: Fine Search (Zoom CZT)
    println!("\n--- Step 2: Fine Search (CZT Zoom) ---");
    // Zoom in around the coarse frequency.
    // Bandwidth of zoom: e.g., 20 kHz window (~0.75m window) to pinpoint peak
    let zoom_bandwidth = 20_000.0;
    let start_freq = coarse_freq - zoom_bandwidth / 2.0;
    let zoom_bins = 100;

    let czt_output = chirp_z_transform(&signal, start_freq, zoom_bandwidth, sample_rate, zoom_bins);

    let mut zoom_max_mag = -1.0;
    let mut zoom_peak_idx = 0;
    for (i, val) in czt_output.iter().enumerate() {
        if val.norm() > zoom_max_mag {
            zoom_max_mag = val.norm();
            zoom_peak_idx = i;
        }
    }

    let refined_freq = start_freq + zoom_bandwidth * (zoom_peak_idx as f64 / zoom_bins as f64);
    let refined_range = config.range_from_beat_frequency(refined_freq);

    println!("Zoom Peak Index: {}", zoom_peak_idx);
    println!("Refined Frequency: {:.2} Hz", refined_freq);
    println!("Refined Range: {:.6} m", refined_range);
    println!("Error: {:.6} m", (refined_range - true_range).abs());

    // 5. Step 3: Phase Extraction & Displacement
    println!("\n--- Step 3: Phase & Displacement ---");
    let peak_complex = czt_output[zoom_peak_idx];

    // The extracted phase at the peak frequency includes:
    // 1. The initial phase shift (what we want).
    // 2. Residual phase from the slight frequency mismatch if we aren't EXACTLY on the beat frequency.
    //
    // Ideally, if refined_freq == beat_freq, then the phase term 2*pi*f*t is perfectly cancelled by the CZT kernel,
    // leaving only the constant phase term.

    let extracted_phase = peak_complex.arg(); // Phase in (-pi, pi]
    println!("Extracted Phase: {:.6} rad", extracted_phase);

    // Eq (5): d = lambda * phi / (4 * pi)
    let calculated_displacement = config.displacement_from_phase(extracted_phase);

    println!("Calculated Displacement: {:.6} mm", calculated_displacement * 1000.0);
    println!("Displacement Error: {:.6} mm", (calculated_displacement * 1000.0 - displacement_mm).abs());

    println!("\n=== Verification Complete ===");
}
