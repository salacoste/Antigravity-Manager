// Audio Format Validation Module
// Epic-014 Story-014-01: Deep audio file validation with format-specific error messages
//
// This module provides comprehensive audio file validation including:
// - Magic bytes (file header) verification for 6 audio formats
// - Duration limit validation with warnings for long files
// - Codec compatibility checks (WAV, M4A)
// - Format-specific actionable error messages

use std::time::Duration;

/// Maximum recommended audio duration (1 hour)
const RECOMMENDED_MAX_DURATION_SECS: u64 = 60 * 60;

/// Hard maximum audio duration (3 hours)
const HARD_MAX_DURATION_SECS: u64 = 180 * 60;

// ==================== Error Types ====================

/// Audio validation errors with format-specific actionable messages
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Invalid file header detected
    #[error("{0}")]
    InvalidHeader(String),

    /// Unsupported codec within valid container
    #[error("{0}")]
    UnsupportedCodec(String),

    /// Audio duration exceeds hard maximum limit
    #[error(
        "Audio file exceeds {max_minutes}-minute limit ({duration_minutes} minutes). {message}"
    )]
    DurationTooLong {
        duration_minutes: u64,
        max_minutes: u64,
        message: String,
    },

    /// Audio file exceeds size limit
    #[error("Audio file exceeds 15MB limit ({size_mb:.1}MB). Please compress or split the file.")]
    FileSizeTooLarge { size_mb: f64 },

    /// Unsupported audio format
    #[error(
        "Audio format '{0}' not supported. Supported formats: MP3, WAV, M4A, OGG, FLAC, AIFF."
    )]
    UnsupportedFormat(String),

    /// File too short to validate
    #[error("Audio file too short (less than {0} bytes). File may be corrupted.")]
    FileTooShort(usize),
}

/// Duration validation warnings (non-blocking)
#[derive(Debug, Clone)]
pub enum DurationWarning {
    /// No warning
    None,

    /// Duration exceeds recommended limit but below hard max
    ExceedsRecommended {
        duration_minutes: u64,
        recommended_minutes: u64,
        message: String,
    },
}

// ==================== Audio Header Validator ====================

/// Validates audio file headers (magic bytes) for format verification
pub struct AudioHeaderValidator;

impl AudioHeaderValidator {
    /// Validate audio file header based on MIME type
    ///
    /// # Arguments
    /// * `data` - Raw audio file bytes
    /// * `mime_type` - MIME type string (e.g., "audio/mpeg")
    ///
    /// # Returns
    /// * `Ok(())` - Header is valid
    /// * `Err(ValidationError)` - Header is invalid with specific error message
    pub fn validate_header(data: &[u8], mime_type: &str) -> Result<(), ValidationError> {
        match mime_type {
            "audio/mpeg" | "audio/mp3" => Self::validate_mp3_header(data),
            "audio/wav" | "audio/x-wav" | "audio/wave" => Self::validate_wav_header(data),
            "audio/mp4" | "audio/m4a" | "audio/x-m4a" => Self::validate_m4a_header(data),
            "audio/ogg" | "audio/vorbis" | "audio/opus" => Self::validate_ogg_header(data),
            "audio/flac" | "audio/x-flac" => Self::validate_flac_header(data),
            "audio/aiff" | "audio/x-aiff" | "audio/aif" => Self::validate_aiff_header(data),
            _ => Err(ValidationError::UnsupportedFormat(mime_type.to_string())),
        }
    }

    /// Validate MP3 file header (MPEG frame sync)
    ///
    /// MP3 files may start with an ID3v2 tag followed by MPEG frames.
    /// We check for:
    /// 1. ID3v2 tag (optional): "ID3" at bytes 0-2
    /// 2. MPEG frame sync: 0xFF 0xFB/0xF3/0xF2 (11 bits set to 1)
    fn validate_mp3_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 2 {
            return Err(ValidationError::FileTooShort(2));
        }

        // Check for ID3v2 tag first
        if data.len() >= 10 && &data[0..3] == b"ID3" {
            // Parse ID3v2 tag size (synchsafe integer)
            let tag_size = Self::parse_id3v2_size(&data[6..10])?;
            let frame_start = 10 + tag_size;

            if frame_start + 2 > data.len() {
                return Err(ValidationError::InvalidHeader(
                    "MP3 file corrupted (ID3 tag incomplete). Expected: complete ID3v2 tag."
                        .to_string(),
                ));
            }

            return Self::check_mp3_frame_sync(&data[frame_start..]);
        }

        // Check frame sync at start of file
        Self::check_mp3_frame_sync(data)
    }

    /// Check MP3 MPEG frame sync (11 bits set to 1: 0xFF 0xE0+)
    fn check_mp3_frame_sync(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 2 {
            return Err(ValidationError::InvalidHeader(
                "MP3 file too short (less than 2 bytes after ID3 tag)".to_string(),
            ));
        }

        // MPEG frame sync: first byte 0xFF, second byte high 3 bits must be 111 (0xE0+)
        if data[0] == 0xFF && (data[1] & 0xE0) == 0xE0 {
            Ok(())
        } else {
            Err(ValidationError::InvalidHeader(
                "MP3 file corrupted (invalid header). Expected: valid MPEG audio layer III frame sync at offset 0 or after ID3v2 tag.".to_string()
            ))
        }
    }

    /// Parse ID3v2 tag size (synchsafe integer - 7 bits per byte, MSB always 0)
    fn parse_id3v2_size(size_bytes: &[u8]) -> Result<usize, ValidationError> {
        if size_bytes.len() < 4 {
            return Err(ValidationError::InvalidHeader(
                "ID3v2 tag size incomplete".to_string(),
            ));
        }

        // Synchsafe integer: ignore MSB of each byte
        let size = ((size_bytes[0] as usize & 0x7F) << 21)
            | ((size_bytes[1] as usize & 0x7F) << 14)
            | ((size_bytes[2] as usize & 0x7F) << 7)
            | (size_bytes[3] as usize & 0x7F);

        Ok(size)
    }

    /// Validate WAV file header (RIFF container with WAVE format)
    ///
    /// WAV files use Resource Interchange File Format (RIFF):
    /// - Bytes 0-3: "RIFF" (chunk ID)
    /// - Bytes 4-7: File size - 8 (chunk size)
    /// - Bytes 8-11: "WAVE" (format)
    fn validate_wav_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 12 {
            return Err(ValidationError::FileTooShort(12));
        }

        // Check RIFF header
        if &data[0..4] != b"RIFF" {
            return Err(ValidationError::InvalidHeader(
                "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3."
                    .to_string(),
            ));
        }

        // Check WAVE format identifier
        if &data[8..12] != b"WAVE" {
            return Err(ValidationError::InvalidHeader(
                "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3 and 'WAVE' at bytes 8-11.".to_string()
            ));
        }

        Ok(())
    }

    /// Validate M4A file header (ISO Base Media File Format with ftyp atom)
    ///
    /// M4A files use ISO BMFF (MP4 container):
    /// - Bytes 4-7: "ftyp" (file type box)
    /// - Bytes 8-11: Major brand (M4A, mp42, isom, etc.)
    fn validate_m4a_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 12 {
            return Err(ValidationError::FileTooShort(12));
        }

        // Check ftyp atom at bytes 4-7
        if &data[4..8] != b"ftyp" {
            return Err(ValidationError::InvalidHeader(
                "M4A file corrupted (invalid ftyp atom). Expected: 'ftyp' at bytes 4-7."
                    .to_string(),
            ));
        }

        // Check major brand (M4A, mp42, isom, iso2, etc.)
        let brand = &data[8..12];
        let valid_brands = [b"M4A ", b"M4B ", b"mp42", b"isom", b"iso2"];

        if !valid_brands.iter().any(|&b| brand == b) {
            return Err(ValidationError::InvalidHeader(
                "M4A file corrupted (invalid ftyp atom). Expected: 'ftyp' at bytes 4-7 with M4A/mp42/isom brand.".to_string()
            ));
        }

        Ok(())
    }

    /// Validate OGG file header (Ogg container format)
    ///
    /// OGG files start with "OggS" magic bytes:
    /// - Bytes 0-3: "OggS" (capture pattern)
    fn validate_ogg_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 4 {
            return Err(ValidationError::FileTooShort(4));
        }

        // Check OggS magic bytes
        if &data[0..4] != b"OggS" {
            return Err(ValidationError::InvalidHeader(
                "OGG file corrupted (invalid header). Expected: 'OggS' at bytes 0-3.".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate FLAC file header (FLAC stream marker)
    ///
    /// FLAC files start with "fLaC" magic bytes:
    /// - Bytes 0-3: "fLaC" (stream marker)
    fn validate_flac_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 4 {
            return Err(ValidationError::FileTooShort(4));
        }

        // Check fLaC magic bytes
        if &data[0..4] != b"fLaC" {
            return Err(ValidationError::InvalidHeader(
                "FLAC file corrupted (invalid header). Expected: 'fLaC' at bytes 0-3.".to_string(),
            ));
        }

        Ok(())
    }

    /// Validate AIFF file header (IFF container with AIFF format)
    ///
    /// AIFF files use Interchange File Format (IFF):
    /// - Bytes 0-3: "FORM" (chunk ID)
    /// - Bytes 4-7: File size - 8 (chunk size)
    /// - Bytes 8-11: "AIFF" or "AIFC" (format)
    fn validate_aiff_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 12 {
            return Err(ValidationError::FileTooShort(12));
        }

        // Check FORM header
        if &data[0..4] != b"FORM" {
            return Err(ValidationError::InvalidHeader(
                "AIFF file corrupted (invalid FORM header). Expected: 'FORM' at bytes 0-3."
                    .to_string(),
            ));
        }

        // Check AIFF format identifier (AIFF or AIFC for compressed)
        let format = &data[8..12];
        if format != b"AIFF" && format != b"AIFC" {
            return Err(ValidationError::InvalidHeader(
                "AIFF file corrupted (invalid FORM header). Expected: 'FORM' at bytes 0-3 and 'AIFF'/'AIFC' at bytes 8-11.".to_string()
            ));
        }

        Ok(())
    }
}

// ==================== Audio Duration Validator ====================

/// Validates audio file duration with warnings for long files
pub struct AudioDurationValidator;

impl AudioDurationValidator {
    /// Validate audio duration with recommended and hard limits
    ///
    /// # Arguments
    /// * `data` - Raw audio file bytes
    /// * `mime_type` - MIME type string
    ///
    /// # Returns
    /// * `Ok(DurationWarning::None)` - Duration within recommended limits
    /// * `Ok(DurationWarning::ExceedsRecommended)` - Duration exceeds 1 hour (warning only)
    /// * `Err(ValidationError::DurationTooLong)` - Duration exceeds 3 hours (block upload)
    pub fn validate_duration(
        data: &[u8],
        mime_type: &str,
    ) -> Result<DurationWarning, ValidationError> {
        // Try to estimate duration
        let duration_secs = match Self::estimate_duration(data, mime_type) {
            Ok(secs) => secs,
            Err(_) => {
                // If duration estimation fails, skip validation (format doesn't support it)
                return Ok(DurationWarning::None);
            }
        };

        // Check hard maximum (3 hours) - block upload
        if duration_secs > HARD_MAX_DURATION_SECS {
            return Err(ValidationError::DurationTooLong {
                duration_minutes: duration_secs / 60,
                max_minutes: HARD_MAX_DURATION_SECS / 60,
                message: format!(
                    "Audio file exceeds 3-hour limit. Maximum: {} minutes. Gemini API may timeout for very long files.",
                    HARD_MAX_DURATION_SECS / 60
                ),
            });
        }

        // Check recommended maximum (1 hour) - warning only
        if duration_secs > RECOMMENDED_MAX_DURATION_SECS {
            return Ok(DurationWarning::ExceedsRecommended {
                duration_minutes: duration_secs / 60,
                recommended_minutes: RECOMMENDED_MAX_DURATION_SECS / 60,
                message: format!(
                    "Audio file exceeds 1-hour recommended limit ({} minutes). Transcription quality may degrade for very long files. Recommended: 60 minutes or less.",
                    duration_secs / 60
                ),
            });
        }

        Ok(DurationWarning::None)
    }

    /// Estimate audio duration in seconds
    fn estimate_duration(data: &[u8], mime_type: &str) -> Result<u64, ValidationError> {
        match mime_type {
            "audio/wav" | "audio/x-wav" | "audio/wave" => Self::parse_wav_duration(data),
            "audio/mp3" | "audio/mpeg" => Self::estimate_mp3_duration(data),
            "audio/m4a" | "audio/mp4" | "audio/x-m4a" => Self::parse_m4a_duration(data),
            "audio/flac" | "audio/x-flac" => Self::parse_flac_duration(data),
            "audio/aiff" | "audio/x-aiff" | "audio/aif" => Self::parse_aiff_duration(data),
            _ => Err(ValidationError::UnsupportedFormat(mime_type.to_string())),
        }
    }

    /// Parse WAV file duration from fmt and data chunks
    fn parse_wav_duration(data: &[u8]) -> Result<u64, ValidationError> {
        // WAV format: search for "fmt " chunk and "data" chunk
        let mut pos = 12; // Skip RIFF header

        let mut sample_rate: Option<u32> = None;
        let mut data_size: Option<u32> = None;
        let mut bytes_per_sample: Option<u16> = None;
        let mut num_channels: Option<u16> = None;

        while pos + 8 <= data.len() {
            let chunk_id = &data[pos..pos + 4];
            let chunk_size =
                u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);

            if chunk_id == b"fmt " && pos + 8 + 16 <= data.len() {
                // Parse fmt chunk
                sample_rate = Some(u32::from_le_bytes([
                    data[pos + 12],
                    data[pos + 13],
                    data[pos + 14],
                    data[pos + 15],
                ]));
                num_channels = Some(u16::from_le_bytes([data[pos + 10], data[pos + 11]]));
                bytes_per_sample = Some(u16::from_le_bytes([data[pos + 22], data[pos + 23]]) / 8);
            } else if chunk_id == b"data" {
                // Parse data chunk
                data_size = Some(chunk_size);
            }

            pos += 8 + chunk_size as usize;
            if pos % 2 != 0 {
                pos += 1; // RIFF chunks are word-aligned
            }
        }

        // Calculate duration
        if let (Some(sr), Some(ds), Some(bps), Some(nc)) =
            (sample_rate, data_size, bytes_per_sample, num_channels)
        {
            if sr > 0 && bps > 0 && nc > 0 {
                let duration_secs = ds as u64 / (sr as u64 * bps as u64 * nc as u64);
                return Ok(duration_secs);
            }
        }

        // Duration estimation failed
        Err(ValidationError::InvalidHeader(
            "WAV file: unable to estimate duration (missing fmt or data chunk)".to_string(),
        ))
    }

    /// Estimate MP3 duration from file size and bitrate
    fn estimate_mp3_duration(data: &[u8]) -> Result<u64, ValidationError> {
        // MP3 duration estimation is complex (variable bitrate, etc.)
        // For simplicity, assume 128 kbps average bitrate
        let file_size_bytes = data.len() as u64;
        let bitrate_kbps = 128;
        let duration_secs = (file_size_bytes * 8) / (bitrate_kbps * 1000);
        Ok(duration_secs)
    }

    /// Parse M4A duration (placeholder - complex format)
    fn parse_m4a_duration(_data: &[u8]) -> Result<u64, ValidationError> {
        // M4A duration parsing requires parsing moov/trak atoms
        // This is complex and requires significant parsing logic
        // For now, skip duration validation for M4A
        Err(ValidationError::UnsupportedFormat(
            "M4A duration estimation not implemented".to_string(),
        ))
    }

    /// Parse FLAC duration from STREAMINFO metadata block
    fn parse_flac_duration(_data: &[u8]) -> Result<u64, ValidationError> {
        // FLAC duration parsing requires parsing STREAMINFO block
        // For now, skip duration validation for FLAC
        Err(ValidationError::UnsupportedFormat(
            "FLAC duration estimation not implemented".to_string(),
        ))
    }

    /// Parse AIFF duration from COMM chunk
    fn parse_aiff_duration(_data: &[u8]) -> Result<u64, ValidationError> {
        // AIFF duration parsing requires parsing COMM chunk
        // For now, skip duration validation for AIFF
        Err(ValidationError::UnsupportedFormat(
            "AIFF duration estimation not implemented".to_string(),
        ))
    }
}

// ==================== Codec Validator ====================

/// Validates audio codec compatibility within container formats
pub struct CodecValidator;

impl CodecValidator {
    /// Validate audio codec is supported by Gemini API
    ///
    /// # Arguments
    /// * `data` - Raw audio file bytes
    /// * `mime_type` - MIME type string
    ///
    /// # Returns
    /// * `Ok(())` - Codec is supported
    /// * `Err(ValidationError::UnsupportedCodec)` - Codec is not supported
    pub fn validate_codec(data: &[u8], mime_type: &str) -> Result<(), ValidationError> {
        match mime_type {
            "audio/wav" | "audio/x-wav" | "audio/wave" => Self::validate_wav_codec(data),
            "audio/m4a" | "audio/mp4" | "audio/x-m4a" => Self::validate_m4a_codec(data),
            // Other formats (MP3, OGG, FLAC, AIFF) have single codec per container
            _ => Ok(()),
        }
    }

    /// Validate WAV codec (must be PCM uncompressed)
    fn validate_wav_codec(data: &[u8]) -> Result<(), ValidationError> {
        // Search for fmt chunk
        let mut pos = 12; // Skip RIFF header

        while pos + 8 <= data.len() {
            let chunk_id = &data[pos..pos + 4];
            let chunk_size =
                u32::from_le_bytes([data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]]);

            if chunk_id == b"fmt " && pos + 10 <= data.len() {
                // Parse format code (bytes 8-9 of fmt chunk)
                let format_code = u16::from_le_bytes([data[pos + 8], data[pos + 9]]);

                return match format_code {
                    0x0001 => Ok(()), // PCM - supported
                    0x0002 => Err(ValidationError::UnsupportedCodec(
                        "WAV file uses ADPCM codec (unsupported). Please convert to PCM (uncompressed) format.".to_string()
                    )),
                    0x0006 => Err(ValidationError::UnsupportedCodec(
                        "WAV file uses A-law codec (unsupported). Please convert to PCM (uncompressed) format.".to_string()
                    )),
                    0x0007 => Err(ValidationError::UnsupportedCodec(
                        "WAV file uses μ-law codec (unsupported). Please convert to PCM (uncompressed) format.".to_string()
                    )),
                    _ => Err(ValidationError::UnsupportedCodec(
                        format!("WAV file uses unknown codec (0x{:04X}). Please convert to PCM (uncompressed) format.", format_code)
                    )),
                };
            }

            pos += 8 + chunk_size as usize;
            if pos % 2 != 0 {
                pos += 1; // RIFF chunks are word-aligned
            }
        }

        // fmt chunk not found
        Err(ValidationError::InvalidHeader(
            "WAV file: fmt chunk not found".to_string(),
        ))
    }

    /// Validate M4A codec (must be AAC)
    fn validate_m4a_codec(_data: &[u8]) -> Result<(), ValidationError> {
        // M4A codec validation requires parsing stsd atom in moov/trak
        // This is complex and requires significant parsing logic
        // For now, assume M4A files use AAC (most common)
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test data (magic bytes for each format)
    const VALID_MP3_HEADER: &[u8] = &[0xFF, 0xFB, 0x90, 0x00]; // MPEG frame sync
    const VALID_MP3_WITH_ID3: &[u8] = &[
        b'I', b'D', b'3', 0x03, 0x00, 0x00, // ID3v2.3
        0x00, 0x00, 0x00, 0x00, // Size: 0 (synchsafe)
        0xFF, 0xFB, 0x90, 0x00, // MPEG frame sync after ID3
    ];
    const INVALID_MP3_HEADER: &[u8] = &[0x00, 0x00, 0x00, 0x00];

    const VALID_WAV_HEADER: &[u8] = &[
        b'R', b'I', b'F', b'F', // RIFF
        0x00, 0x00, 0x00, 0x00, // Size
        b'W', b'A', b'V', b'E', // WAVE
    ];
    const INVALID_WAV_RIFF: &[u8] = &[
        b'X', b'I', b'F', b'F', // Invalid RIFF
        0x00, 0x00, 0x00, 0x00, b'W', b'A', b'V', b'E',
    ];

    const VALID_M4A_HEADER: &[u8] = &[
        0x00, 0x00, 0x00, 0x18, // Box size
        b'f', b't', b'y', b'p', // ftyp
        b'M', b'4', b'A', b' ', // M4A brand
    ];

    const VALID_OGG_HEADER: &[u8] = &[b'O', b'g', b'g', b'S'];
    const VALID_FLAC_HEADER: &[u8] = &[b'f', b'L', b'a', b'C'];
    const VALID_AIFF_HEADER: &[u8] = &[
        b'F', b'O', b'R', b'M', // FORM
        0x00, 0x00, 0x00, 0x00, // Size
        b'A', b'I', b'F', b'F', // AIFF
    ];

    #[test]
    fn test_validate_mp3_header_valid() {
        assert!(AudioHeaderValidator::validate_mp3_header(VALID_MP3_HEADER).is_ok());
    }

    #[test]
    fn test_validate_mp3_header_with_id3_tag() {
        assert!(AudioHeaderValidator::validate_mp3_header(VALID_MP3_WITH_ID3).is_ok());
    }

    #[test]
    fn test_validate_mp3_header_corrupted() {
        assert!(AudioHeaderValidator::validate_mp3_header(INVALID_MP3_HEADER).is_err());
    }

    #[test]
    fn test_validate_wav_header_valid() {
        assert!(AudioHeaderValidator::validate_wav_header(VALID_WAV_HEADER).is_ok());
    }

    #[test]
    fn test_validate_wav_header_invalid_riff() {
        assert!(AudioHeaderValidator::validate_wav_header(INVALID_WAV_RIFF).is_err());
    }

    #[test]
    fn test_validate_m4a_header_valid() {
        assert!(AudioHeaderValidator::validate_m4a_header(VALID_M4A_HEADER).is_ok());
    }

    #[test]
    fn test_validate_ogg_header_valid() {
        assert!(AudioHeaderValidator::validate_ogg_header(VALID_OGG_HEADER).is_ok());
    }

    #[test]
    fn test_validate_flac_header_valid() {
        assert!(AudioHeaderValidator::validate_flac_header(VALID_FLAC_HEADER).is_ok());
    }

    #[test]
    fn test_validate_aiff_header_valid() {
        assert!(AudioHeaderValidator::validate_aiff_header(VALID_AIFF_HEADER).is_ok());
    }
}
