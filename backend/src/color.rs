pub struct ImaginaryColor {
    pub css: String,
    pub name: String,
    pub srgb_fallback: String,
}

const NAMES: &[&str] = &[
    "Octarine",
    "Sussurian Ochre",
    "Vantaviolet",
    "Bleens",
    "Ultravoid",
    "Gloompeach",
    "Xanadu-less Green",
    "Melancholy Magenta",
    "Plurple",
    "Synesthetic Beige",
];

const HUE_NAMES: &[&str] = &[
    "Impossible Crimson",
    "Undreamt Orange",
    "Post-Yellow",
    "Fevergreen",
    "Beyondcyan",
    "Void Blue",
    "Paracobalt",
    "Null Magenta",
];

pub fn from_freq(freq_hz: f32) -> ImaginaryColor {
    let h = (freq_hz * 0.037).fract();

    let r = 1.15 + 0.45 * (h * std::f32::consts::TAU).sin();
    let g = -0.35 + 0.6 * ((h + 0.33).fract() * std::f32::consts::TAU).sin();
    let b = 0.9 + 0.5 * ((h + 0.66).fract() * std::f32::consts::TAU).cos();

    let srgb_fallback = clamp_to_srgb_hex(prophoto_to_srgb(r, g, b));

    let name = if (freq_hz as usize / 97).is_multiple_of(2) {
        let idx = (freq_hz as usize / 137) % NAMES.len();
        NAMES[idx]
    } else {
        let idx = (freq_hz as usize / 251) % HUE_NAMES.len();
        HUE_NAMES[idx]
    };

    ImaginaryColor {
        css: format!("color(prophoto-rgb {r:.3} {g:.3} {b:.3})"),
        name: format!("{name} (hue {:.2})", h * 360.0),
        srgb_fallback,
    }
}

fn prophoto_to_srgb(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let x = 0.7977 * r + 0.1352 * g + 0.0313 * b;
    let y = 0.2880 * r + 0.7119 * g + 0.0001 * b;
    let z = 0.0000 * r + 0.0000 * g + 0.8249 * b;

    let sr = 3.2406 * x - 1.5372 * y - 0.4986 * z;
    let sg = -0.9689 * x + 1.8758 * y + 0.0415 * z;
    let sb = 0.0557 * x - 0.2040 * y + 1.0570 * z;

    (sr, sg, sb)
}

fn clamp_to_srgb_hex((r, g, b): (f32, f32, f32)) -> String {
    let to_u8 = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", to_u8(r), to_u8(g), to_u8(b))
}
