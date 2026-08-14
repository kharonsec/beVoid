use rustfft::num_complex::Complex;
use rustfft::FftPlanner;

use crate::color::{self, ImaginaryColor};

pub struct HumReading {
    pub emotion: String,
    pub freq_hz: f32,
    pub vibes: f32,
    pub color: ImaginaryColor,
}

pub fn analyze(samples: &[f32], sample_rate: u32) -> Result<HumReading, String> {
    if samples.is_empty() || sample_rate == 0 {
        return Err("nothing to hum about".into());
    }

    let n = samples.len().next_power_of_two().max(256);

    let mut buffer: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); n];
    for (i, s) in samples.iter().take(n).enumerate() {
        let window = 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / n as f32).cos();
        buffer[i] = Complex::new(s * window, 0.0);
    }

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(n);
    fft.process(&mut buffer);

    let mut magnitudes = vec![0.0f32; n / 2];
    for (i, m) in magnitudes.iter_mut().enumerate() {
        *m = buffer[i].norm();
    }

    let mut peak_bin = 1usize;
    for (i, m) in magnitudes.iter().enumerate().skip(1) {
        if *m > magnitudes[peak_bin] {
            peak_bin = i;
        }
    }

    let freq_hz = peak_bin as f32 * sample_rate as f32 / n as f32;

    let bin_hz = sample_rate as f32 / n as f32;
    let total: f32 = magnitudes.iter().sum();
    let centroid: f32 = magnitudes
        .iter()
        .enumerate()
        .map(|(i, m)| i as f32 * bin_hz * m)
        .sum::<f32>()
        / total.max(f32::EPSILON);
    let vibes = (centroid / (sample_rate as f32 / 2.0)).clamp(0.0, 1.0);

    let emotion = match freq_hz {
        f if f < 120.0 => "despair",
        f if f < 200.0 => "yearning",
        f if f < 300.0 => "melancholy",
        f if f < 450.0 => "pondering",
        f if f < 650.0 => "contentment",
        f if f < 900.0 => "giddiness",
        _ => "euphoria",
    }
    .to_string();

    Ok(HumReading {
        emotion,
        freq_hz,
        vibes,
        color: color::from_freq(freq_hz),
    })
}
