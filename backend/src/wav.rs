pub struct Wav {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub fn parse(bytes: &[u8]) -> Result<Wav, String> {
    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not a RIFF/WAVE file".into());
    }

    let mut offset = 12usize;
    let mut sample_rate = 0u32;
    let mut channels = 1u32;
    let mut bits = 16u32;
    let mut data = Vec::new();

    while offset + 8 <= bytes.len() {
        let id = &bytes[offset..offset + 4];
        let size = u32::from_le_bytes(
            bytes[offset + 4..offset + 8]
                .try_into()
                .map_err(|_| "truncated chunk header".to_string())?,
        ) as usize;
        let body = &bytes[offset + 8..(offset + 8 + size).min(bytes.len())];

        match id {
            b"fmt " => {
                let format = u16::from_le_bytes([body[0], body[1]]);
                if format != 1 && format != 3 {
                    return Err(format!("unsupported WAV format {}", format));
                }
                channels = u16::from_le_bytes([body[2], body[3]]) as u32;
                sample_rate = u32::from_le_bytes([body[4], body[5], body[6], body[7]]);
                bits = u16::from_le_bytes([body[14], body[15]]) as u32;
            }
            b"data" => data.extend_from_slice(body),
            _ => {}
        }

        offset += 8 + size + (size & 1);
    }

    if data.is_empty() {
        return Err("WAV contains no data chunk".into());
    }
    if channels == 0 {
        return Err("WAV declares zero channels".into());
    }

    let samples = match bits {
        16 => decode_pcm16(&data, channels),
        32 => decode_f32(&data, channels),
        _ => return Err(format!("unsupported bit depth {}", bits)),
    }?;

    Ok(Wav {
        sample_rate,
        samples,
    })
}

fn decode_pcm16(data: &[u8], channels: u32) -> Result<Vec<f32>, String> {
    if !data.len().is_multiple_of(2) {
        return Err("odd-sized PCM16 data".into());
    }
    let mut out = Vec::with_capacity(data.len() / 2 / channels as usize);
    for chunk in data.chunks_exact(2 * channels as usize) {
        let mut acc = 0f32;
        for pair in chunk.chunks_exact(2) {
            let v = i16::from_le_bytes([pair[0], pair[1]]) as f32;
            acc += v / 32768.0;
        }
        out.push(acc / channels as f32);
    }
    Ok(out)
}

fn decode_f32(data: &[u8], channels: u32) -> Result<Vec<f32>, String> {
    if !data.len().is_multiple_of(4) {
        return Err("odd-sized float32 data".into());
    }
    let mut out = Vec::with_capacity(data.len() / 4 / channels as usize);
    for chunk in data.chunks_exact(4 * channels as usize) {
        let mut acc = 0f32;
        for quad in chunk.chunks_exact(4) {
            acc += f32::from_le_bytes(quad.try_into().map_err(|_| "truncated float".to_string())?);
        }
        out.push(acc / channels as f32);
    }
    Ok(out)
}
