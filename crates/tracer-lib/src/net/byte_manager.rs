use crate::errors::conversion_issue::ConvertError;

pub fn convert_to_be_bytes(d: f64) -> Result<[u8; 4], ConvertError> {
    Some(d * 10_000_000.0)
        .filter(|v| (i32::MIN as f64..=i32::MAX as f64).contains(v))
        .map(|v| (v as i32).to_be_bytes())
        .ok_or(ConvertError::OutOfRange)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_value() {
        assert_eq!(convert_to_be_bytes(1.0), Ok([0x00, 0x98, 0x96, 0x80]));
    }

    #[test]
    fn negative_value_is_twos_complement() {
        assert_eq!(convert_to_be_bytes(-1.0), Ok([0xFF, 0x67, 0x69, 0x80]));
    }

    #[test]
    fn too_large_is_an_error() {
        assert_eq!(convert_to_be_bytes(300.0), Err(ConvertError::OutOfRange));
    }

    #[test]
    fn nan_is_an_error() {
        assert_eq!(convert_to_be_bytes(f64::NAN), Err(ConvertError::OutOfRange));
    }
}