use crate::ColorMixerError;

pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3], ColorMixerError> {
    if hex.len() != 6 {
        return Err(ColorMixerError {
            msg: "expected hex string of length 6 (XXXXXX)",
        });
    }
    let r = u8::from_str_radix(&hex[..2], 16);
    let g = u8::from_str_radix(&hex[2..4], 16);
    let b = u8::from_str_radix(&hex[4..], 16);
    if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
        Ok([r, g, b])
    } else {
        Err(ColorMixerError {
            msg: "failed to parse hex color",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_invalid_length() {
        assert!(hex_to_rgb("blah").is_err());
        assert!(hex_to_rgb("0xAABB").is_err());
    }

    #[test]
    fn fails_invalid_hex() {
        assert!(hex_to_rgb("XXAABB").is_err());
    }

    #[test]
    fn parses_hex() {
        assert_eq!(hex_to_rgb("0099FF"), Ok([0, 153, 255]));
    }
}
