use base64::{decode, DecodeError};

/// Decodes a given Base64-encoded string and returns the decoded content as a String.
/// 
/// # Arguments
/// * `input` - A Base64-encoded string slice that needs to be decoded.
///
/// # Returns
/// A `Result<String, DecodeError>` containing the decoded string or an error if decoding fails.
pub fn decode_from_base64(input: &str) -> Result<String, DecodeError> {
    let decoded_bytes = decode(input)?;
    let decoded_string = String::from_utf8(decoded_bytes)
        .map_err(|_| DecodeError::InvalidByte(0, 0))?; // Maps UTF-8 conversion errors to DecodeError
    Ok(decoded_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_from_base64() {
        let sample_input = "QUxCTk0sIFBST0QwMDEsIDEyLCAyMDIzLTAxLTAx";
        let expected_output = "ALBNM, PROD001, 12, 2023-01-01";

        assert_eq!(decode_from_base64(sample_input).unwrap(), expected_output);
    }

    #[test]
    fn test_invalid_base64() {
        let invalid_input = "##INVALID##";

        assert!(decode_from_base64(invalid_input).is_err());
    }
}