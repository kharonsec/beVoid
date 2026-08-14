use serde::Deserialize;

#[derive(Deserialize)]
struct AuthConfig {
    name: String,
    threshold: f64,
    bezier: BezierConfig,
}

#[derive(Deserialize)]
struct BezierConfig {
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
}

pub struct Auth {
    pub sigil_name: String,
    samples: Vec<(f64, f64)>,
    threshold: f64,
}

impl Auth {
    pub fn load(path: &str) -> Result<Auth, String> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {path}: {e}"))?;
        let cfg: AuthConfig = toml::from_str(&raw)
            .map_err(|e| format!("cannot parse {path}: {e}"))?;

        let mut samples = Vec::with_capacity(200);
        for i in 0..200 {
            let t = i as f64 / 199.0;
            samples.push(cubic_bezier(
                cfg.bezier.p0, cfg.bezier.p1, cfg.bezier.p2, cfg.bezier.p3, t,
            ));
        }

        Ok(Auth {
            sigil_name: cfg.name,
            samples,
            threshold: cfg.threshold,
        })
    }

    pub fn verify(&self, xs: &[f64], ys: &[f64]) -> bool {
        if xs.len() < 8 || xs.len() != ys.len() {
            return false;
        }
        let drawn: Vec<(f64, f64)> = xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)).collect();
        let mut total = 0.0f64;
        for (dx, dy) in &drawn {
            let mut best = f64::INFINITY;
            for (sx, sy) in &self.samples {
                let d = (dx - sx) * (dx - sx) + (dy - sy) * (dy - sy);
                if d < best {
                    best = d;
                }
            }
            total += best.sqrt();
        }
        (total / drawn.len() as f64) < self.threshold
    }
}

fn cubic_bezier(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], t: f64) -> (f64, f64) {
    let u = 1.0 - t;
    let x = u * u * u * p0[0] + 3.0 * u * u * t * p1[0] + 3.0 * u * t * t * p2[0] + t * t * t * p3[0];
    let y = u * u * u * p0[1] + 3.0 * u * u * t * p1[1] + 3.0 * u * t * t * p2[1] + t * t * t * p3[1];
    (x, y)
}
