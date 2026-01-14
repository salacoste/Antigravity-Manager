# Story-014-01: Audio Format Validation Enhancement

**Epic**: Epic-014 - Gemini 2.0 Flash Experimental Audio Specialist (Team 1, Gemini Specialists)
**Priority**: P1 (HIGH - better UX)
**Effort**: 3 days
**Assignee**: Dev 1A (Team 1 Lead)
**Status**: ✅ READY FOR EXECUTION (Start: 2026-01-22)
**Created**: 2026-01-12

---

## Objective

Implement deep audio file validation with format-specific error messages to catch corrupted files, validate duration limits, check codec compatibility, and provide actionable user feedback before calling Gemini API. This will reduce failed API calls by 15-20% and significantly improve user experience.

---

## Business Context

### Problem Statement

Current audio validation is basic:
- **MIME type only**: File extension check without header validation
- **Corrupted files pass**: Invalid files uploaded, waste API quota
- **Generic errors**: "Invalid format" without specific guidance
- **No duration checks**: Very long files (>1 hour) cause issues

**Impact**:
```yaml
wasted_quota: "15-20% of failed audio API calls"
  - "Corrupted files uploaded (invalid headers)"
  - "Unsupported codecs within valid containers"
  - "Extremely long files (>1 hour) timeout"

poor_ux: "Generic error messages"
  - "User doesn't know what's wrong"
  - "No actionable guidance to fix issue"
  - "Support tickets increase"

bandwidth_waste: "15MB corrupted file uploads"
  - "Base64 encoding overhead"
  - "Network bandwidth consumed"
  - "Slow response times"
```

### Success Metrics

**Primary KPI**: 15-20% reduction in failed audio API calls
**User Experience**: Format-specific error messages with fix guidance
**Performance**: Validation overhead <50ms per file
**Test Coverage**: 15+ tests (100% coverage)

### Business Value

- **Cost savings**: $50-100/month per 10K audio requests (reduced failed calls)
- **User satisfaction**: Immediate actionable feedback
- **Support efficiency**: 30% reduction in audio-related tickets
- **Competitive advantage**: Professional-grade audio validation

---

## Acceptance Criteria

### AC1: Audio File Header Validation (Magic Bytes)

**GIVEN** an audio file upload via Whisper API
**WHEN** the file is validated before Gemini API call
**THEN** the file header (magic bytes) MUST be verified for each format

**Magic Bytes Verification**:
```yaml
MP3:
  magic_bytes: [0xFF, 0xFB] or [0xFF, 0xF3] or [0xFF, 0xF2]
  position: "Start of file or after ID3 tag"
  validation: "Check MPEG audio layer III frame sync"
  error_message: "MP3 file corrupted (invalid header). Expected: valid MPEG audio layer III frame sync at offset 0 or after ID3v2 tag."

WAV:
  magic_bytes: [0x52, 0x49, 0x46, 0x46] (RIFF header)
  position: "Bytes 0-3"
  additional_check: "Bytes 8-11 must be 'WAVE'"
  validation: "RIFF container with WAVE format"
  error_message: "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3 and 'WAVE' at bytes 8-11."

M4A:
  magic_bytes: [0x66, 0x74, 0x79, 0x70] (ftyp atom)
  position: "Bytes 4-7"
  additional_check: "Major brand must be M4A/mp42/isom"
  validation: "ISO Base Media File Format with M4A brand"
  error_message: "M4A file corrupted (invalid ftyp atom). Expected: 'ftyp' at bytes 4-7 with M4A/mp42/isom brand."

OGG:
  magic_bytes: [0x4F, 0x67, 0x67, 0x53] (OggS)
  position: "Bytes 0-3"
  validation: "Ogg container format"
  error_message: "OGG file corrupted (invalid header). Expected: 'OggS' at bytes 0-3."

FLAC:
  magic_bytes: [0x66, 0x4C, 0x61, 0x43] (fLaC)
  position: "Bytes 0-3"
  validation: "FLAC audio stream"
  error_message: "FLAC file corrupted (invalid header). Expected: 'fLaC' at bytes 0-3."

AIFF:
  magic_bytes: [0x46, 0x4F, 0x52, 0x4D] (FORM)
  position: "Bytes 0-3"
  additional_check: "Bytes 8-11 must be 'AIFF' or 'AIFC'"
  validation: "IFF container with AIFF format"
  error_message: "AIFF file corrupted (invalid FORM header). Expected: 'FORM' at bytes 0-3 and 'AIFF'/'AIFC' at bytes 8-11."
```

**Implementation**:
```rust
pub struct AudioHeaderValidator;

impl AudioHeaderValidator {
    pub fn validate_header(
        data: &[u8],
        mime_type: &str,
    ) -> Result<(), ValidationError> {
        match mime_type {
            "audio/mpeg" | "audio/mp3" => Self::validate_mp3_header(data),
            "audio/wav" | "audio/x-wav" => Self::validate_wav_header(data),
            "audio/mp4" | "audio/m4a" => Self::validate_m4a_header(data),
            "audio/ogg" | "audio/vorbis" => Self::validate_ogg_header(data),
            "audio/flac" => Self::validate_flac_header(data),
            "audio/aiff" | "audio/x-aiff" => Self::validate_aiff_header(data),
            _ => Err(ValidationError::UnsupportedFormat(mime_type.to_string())),
        }
    }

    fn validate_mp3_header(data: &[u8]) -> Result<(), ValidationError> {
        // Check for ID3 tag first
        if data.len() >= 3 && &data[0..3] == b"ID3" {
            // Skip ID3v2 tag to find frame sync
            let tag_size = Self::parse_id3v2_size(&data[6..10]);
            let frame_start = 10 + tag_size;
            if frame_start + 2 <= data.len() {
                return Self::check_mp3_frame_sync(&data[frame_start..]);
            }
        }

        // Check frame sync at start
        Self::check_mp3_frame_sync(data)
    }

    fn check_mp3_frame_sync(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 2 {
            return Err(ValidationError::InvalidHeader(
                "MP3 file too short (less than 2 bytes)".to_string()
            ));
        }

        // Check for MPEG frame sync (11 bits set to 1)
        if data[0] == 0xFF && (data[1] & 0xE0) == 0xE0 {
            Ok(())
        } else {
            Err(ValidationError::InvalidHeader(
                "MP3 file corrupted (invalid header). Expected: valid MPEG audio layer III frame sync at offset 0 or after ID3v2 tag.".to_string()
            ))
        }
    }

    fn validate_wav_header(data: &[u8]) -> Result<(), ValidationError> {
        if data.len() < 12 {
            return Err(ValidationError::InvalidHeader(
                "WAV file too short (less than 12 bytes)".to_string()
            ));
        }

        // Check RIFF header
        if &data[0..4] != b"RIFF" {
            return Err(ValidationError::InvalidHeader(
                "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3.".to_string()
            ));
        }

        // Check WAVE format
        if &data[8..12] != b"WAVE" {
            return Err(ValidationError::InvalidHeader(
                "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3 and 'WAVE' at bytes 8-11.".to_string()
            ));
        }

        Ok(())
    }

    // Similar implementations for M4A, OGG, FLAC, AIFF...
}
```

---

### AC2: Duration Limit Validation

**GIVEN** an audio file upload
**WHEN** the file duration can be determined
**THEN** warn if duration exceeds recommended limits

**Duration Limits**:
```yaml
recommended_max: "60 minutes (1 hour)"
  reason: "Optimal transcription quality and reasonable processing time"
  action: "WARN (not block)"

hard_max: "180 minutes (3 hours)"
  reason: "Gemini API timeout risk"
  action: "ERROR (block upload)"

duration_detection:
  mp3: "Parse MPEG frame headers, calculate from bitrate + file size"
  wav: "Parse fmt chunk (sample rate + data chunk size)"
  m4a: "Parse moov/trak atoms (duration in timescale units)"
  ogg: "Parse Vorbis comment header (not always reliable)"
  flac: "Parse STREAMINFO metadata block"
  aiff: "Parse COMM chunk (sample rate + frames)"
```

**Implementation**:
```rust
pub struct AudioDurationValidator;

impl AudioDurationValidator {
    pub fn validate_duration(
        data: &[u8],
        mime_type: &str,
    ) -> Result<DurationWarning, ValidationError> {
        let duration_secs = Self::estimate_duration(data, mime_type)?;

        if duration_secs > 180 * 60 {
            Err(ValidationError::DurationTooLong {
                duration_minutes: duration_secs / 60,
                max_minutes: 180,
                message: "Audio file exceeds 3-hour limit (Gemini API timeout risk). Maximum: 180 minutes.".to_string(),
            })
        } else if duration_secs > 60 * 60 {
            Ok(DurationWarning::ExceedsRecommended {
                duration_minutes: duration_secs / 60,
                recommended_minutes: 60,
                message: "Audio file exceeds 1-hour recommended limit. Transcription quality may degrade for very long files. Recommended: 60 minutes or less.".to_string(),
            })
        } else {
            Ok(DurationWarning::None)
        }
    }

    fn estimate_duration(data: &[u8], mime_type: &str) -> Result<u64, ValidationError> {
        match mime_type {
            "audio/wav" => Self::parse_wav_duration(data),
            "audio/mp3" | "audio/mpeg" => Self::estimate_mp3_duration(data),
            "audio/m4a" | "audio/mp4" => Self::parse_m4a_duration(data),
            "audio/flac" => Self::parse_flac_duration(data),
            "audio/aiff" => Self::parse_aiff_duration(data),
            _ => Ok(0), // Unknown format, skip duration check
        }
    }

    fn parse_wav_duration(data: &[u8]) -> Result<u64, ValidationError> {
        // Parse fmt chunk for sample rate
        let sample_rate = Self::parse_wav_sample_rate(data)?;
        let data_size = Self::parse_wav_data_size(data)?;
        let bytes_per_sample = Self::parse_wav_bits_per_sample(data)? / 8;
        let num_channels = Self::parse_wav_num_channels(data)?;

        let duration_secs = data_size / (sample_rate * bytes_per_sample * num_channels);
        Ok(duration_secs as u64)
    }

    // Similar implementations for other formats...
}
```

---

### AC3: Codec Compatibility Validation

**GIVEN** an audio file with container format
**WHEN** the codec is detected
**THEN** validate codec is supported by Gemini API

**Supported Codecs**:
```yaml
MP3:
  container: "MPEG"
  codec: "MP3 (MPEG-1 Audio Layer III)"
  validation: "Check MPEG version and layer bits"
  unsupported: "MPEG-2.5, MPEG-2 Layer I/II (rare)"

WAV:
  container: "RIFF/WAV"
  codec: "PCM (uncompressed)"
  validation: "Check format code in fmt chunk (0x0001 = PCM)"
  unsupported: "ADPCM, μ-law, A-law (compressed variants)"

M4A:
  container: "ISO Base Media (MP4)"
  codec: "AAC (Advanced Audio Coding)"
  validation: "Check codec 4CC in stsd atom (mp4a)"
  unsupported: "ALAC (Apple Lossless), AC-3"

OGG:
  container: "Ogg"
  codec: "Vorbis or Opus"
  validation: "Check codec identification header"
  unsupported: "Speex, Theora (video)"

FLAC:
  container: "Native FLAC"
  codec: "FLAC (Free Lossless Audio Codec)"
  validation: "Check STREAMINFO block"
  unsupported: "N/A (FLAC is single codec)"

AIFF:
  container: "IFF/AIFF"
  codec: "PCM (uncompressed)"
  validation: "Check compression type in COMM chunk"
  unsupported: "AIFF-C with non-PCM codecs"
```

**Implementation**:
```rust
pub struct CodecValidator;

impl CodecValidator {
    pub fn validate_codec(
        data: &[u8],
        mime_type: &str,
    ) -> Result<(), ValidationError> {
        match mime_type {
            "audio/wav" => Self::validate_wav_codec(data),
            "audio/m4a" => Self::validate_m4a_codec(data),
            // Other formats...
            _ => Ok(()), // Skip codec validation for simple formats
        }
    }

    fn validate_wav_codec(data: &[u8]) -> Result<(), ValidationError> {
        let format_code = Self::parse_wav_format_code(data)?;

        match format_code {
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
        }
    }

    // Similar implementations for M4A, etc.
}
```

---

### AC4: Format-Specific Error Messages

**GIVEN** a validation failure
**WHEN** returning error to user
**THEN** provide format-specific actionable guidance

**Error Message Templates**:
```yaml
invalid_header:
  template: "{FORMAT} file corrupted ({SPECIFIC_ISSUE}). Expected: {EXPECTED_VALUE}."
  examples:
    - "MP3 file corrupted (invalid header). Expected: valid MPEG audio layer III frame sync at offset 0 or after ID3v2 tag."
    - "WAV file corrupted (invalid RIFF header). Expected: 'RIFF' at bytes 0-3 and 'WAVE' at bytes 8-11."

unsupported_codec:
  template: "{FORMAT} file uses {CODEC} codec (unsupported). Please convert to {RECOMMENDED_CODEC} format."
  examples:
    - "WAV file uses ADPCM codec (unsupported). Please convert to PCM (uncompressed) format."
    - "M4A file uses ALAC codec (unsupported). Please convert to AAC format."

duration_limit:
  template: "Audio file exceeds {LIMIT}-{UNIT} limit ({ACTUAL} {UNIT}). {RECOMMENDATION}."
  examples:
    - "Audio file exceeds 3-hour limit (185 minutes). Maximum: 180 minutes."
    - "Audio file exceeds 1-hour recommended limit (75 minutes). Transcription quality may degrade for very long files. Recommended: 60 minutes or less."

file_size:
  template: "Audio file exceeds 15MB limit ({ACTUAL_SIZE}MB). Please compress or split the file."
  example: "Audio file exceeds 15MB limit (18.5MB). Please compress or split the file."

unsupported_format:
  template: "Audio format '{FORMAT}' not supported. Supported formats: MP3, WAV, M4A, OGG, FLAC, AIFF."
  example: "Audio format 'audio/wma' not supported. Supported formats: MP3, WAV, M4A, OGG, FLAC, AIFF."
```

**Implementation**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("{0}")]
    InvalidHeader(String),

    #[error("{0}")]
    UnsupportedCodec(String),

    #[error("Audio file exceeds {max_minutes}-minute limit ({duration_minutes} minutes). {message}")]
    DurationTooLong {
        duration_minutes: u64,
        max_minutes: u64,
        message: String,
    },

    #[error("Audio file exceeds 15MB limit ({size_mb:.1}MB). Please compress or split the file.")]
    FileSizeTooLarge { size_mb: f64 },

    #[error("Audio format '{0}' not supported. Supported formats: MP3, WAV, M4A, OGG, FLAC, AIFF.")]
    UnsupportedFormat(String),
}

pub enum DurationWarning {
    None,
    ExceedsRecommended {
        duration_minutes: u64,
        recommended_minutes: u64,
        message: String,
    },
}
```

---

### AC5: Comprehensive Test Coverage

**Unit Tests** (10+ tests minimum):
```yaml
Header Validation Tests:
  - test_validate_mp3_header_valid
  - test_validate_mp3_header_with_id3_tag
  - test_validate_mp3_header_corrupted
  - test_validate_wav_header_valid
  - test_validate_wav_header_invalid_riff
  - test_validate_m4a_header_valid
  - test_validate_ogg_header_valid
  - test_validate_flac_header_valid
  - test_validate_aiff_header_valid

Duration Validation Tests:
  - test_duration_under_recommended (30 min → OK)
  - test_duration_exceeds_recommended (75 min → WARNING)
  - test_duration_exceeds_hard_max (200 min → ERROR)

Codec Validation Tests:
  - test_wav_pcm_codec_valid
  - test_wav_adpcm_codec_invalid
  - test_m4a_aac_codec_valid

Error Message Tests:
  - test_error_message_format_specific
  - test_error_message_actionable_guidance
```

**Integration Tests** (5+ tests minimum):
```yaml
End-to-End Validation:
  - test_e2e_valid_audio_file_passes_all_checks
  - test_e2e_corrupted_mp3_rejected_with_specific_error
  - test_e2e_long_duration_file_warning
  - test_e2e_unsupported_codec_rejected
  - test_e2e_validation_performance_under_50ms
```

---

## Implementation Details

### Module Structure

```
src-tauri/src/
├── proxy/handlers/
│   └── audio.rs                    (MODIFY - integrate validation)
└── utils/
    └── audio_validation.rs         (NEW - 600 lines)
        ├── AudioHeaderValidator
        │   ├── validate_header()
        │   ├── validate_mp3_header()
        │   ├── validate_wav_header()
        │   ├── validate_m4a_header()
        │   ├── validate_ogg_header()
        │   ├── validate_flac_header()
        │   └── validate_aiff_header()
        ├── AudioDurationValidator
        │   ├── validate_duration()
        │   ├── estimate_duration()
        │   ├── parse_wav_duration()
        │   ├── parse_mp3_duration()
        │   └── parse_m4a_duration()
        ├── CodecValidator
        │   ├── validate_codec()
        │   ├── validate_wav_codec()
        │   └── validate_m4a_codec()
        └── ValidationError (enum)

tests/
└── audio/
    └── validation_tests.rs         (NEW - 15 tests)
```

### Integration with audio.rs

```rust
// src-tauri/src/proxy/handlers/audio.rs

use crate::utils::audio_validation::{
    AudioHeaderValidator,
    AudioDurationValidator,
    CodecValidator,
    ValidationError,
    DurationWarning,
};

pub async fn handle_audio_transcription(
    file_data: Vec<u8>,
    filename: String,
    config: &ProxyConfig,
) -> Result<TranscriptionResponse, HandlerError> {
    // Step 1: Existing file size validation
    if file_data.len() > MAX_AUDIO_SIZE {
        return Err(HandlerError::Validation(
            ValidationError::FileSizeTooLarge {
                size_mb: file_data.len() as f64 / 1_048_576.0,
            }
        ));
    }

    // Step 2: Detect MIME type (existing logic)
    let mime_type = detect_mime_type(&file_data, &filename)?;

    // Step 3: NEW - Validate audio file header
    AudioHeaderValidator::validate_header(&file_data, &mime_type)
        .map_err(HandlerError::Validation)?;

    // Step 4: NEW - Validate codec compatibility
    CodecValidator::validate_codec(&file_data, &mime_type)
        .map_err(HandlerError::Validation)?;

    // Step 5: NEW - Validate duration (with warnings)
    match AudioDurationValidator::validate_duration(&file_data, &mime_type) {
        Ok(DurationWarning::ExceedsRecommended { message, .. }) => {
            warn!("Audio duration warning: {}", message);
            // Continue processing with warning log
        }
        Ok(DurationWarning::None) => {
            // No warning, continue
        }
        Err(e) => {
            // Hard error (exceeds 3-hour limit)
            return Err(HandlerError::Validation(e));
        }
    }

    // Step 6: Continue with existing transcription logic
    // ... (Gemini API call, etc.)
}
```

---

## Test Strategy

### Phase 1: Unit Testing (Day 1)
**Focus**: Individual validator functions

```bash
cargo test --package antigravity_tools_lib audio_validation
```

**Test Data**: `tests/data/audio_samples/`
- `valid_mp3.mp3` (clean MP3 file)
- `corrupted_mp3.mp3` (invalid header)
- `long_audio.wav` (75-minute file)
- `adpcm_wav.wav` (unsupported codec)

---

### Phase 2: Integration Testing (Day 2)
**Focus**: End-to-end validation flow

```bash
cargo test --package antigravity_tools_lib handlers::audio::integration
```

---

### Phase 3: Performance Testing (Day 3)
**Focus**: Validation overhead <50ms

```bash
cargo bench --package antigravity_tools_lib audio_validation_bench
```

---

## Dependencies

### Internal Dependencies
- `src-tauri/src/proxy/handlers/audio.rs` - STABLE (modify validation logic)
- No external crate dependencies (use std::io for binary parsing)

---

## Success Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Failed API call reduction | 15-20% | Compare before/after validation deployment |
| Validation overhead | <50ms | Benchmark validation functions |
| Test coverage | 100% | `cargo tarpaulin` |
| User satisfaction | 30% fewer support tickets | Support ticket tracking |

---

## Definition of Done

### Code Complete
- ✅ `audio_validation.rs` implemented (600 lines)
- ✅ `audio.rs` integrated with validators
- ✅ Format-specific error messages

### Testing Complete
- ✅ 10+ unit tests passing
- ✅ 5+ integration tests passing
- ✅ Performance benchmarks <50ms

### Quality Gates Passed
- ✅ 15-20% reduction in failed API calls validated
- ✅ Error messages actionable and format-specific
- ✅ No performance regression

### Documentation Complete
- ✅ Inline rustdoc comments
- ✅ Error message examples documented
- ✅ Validation logic explained

---

## Risk Assessment

**Risk 1**: False positives (valid files rejected)
- **Impact**: HIGH (user frustration)
- **Probability**: LOW
- **Mitigation**: Comprehensive test suite with real-world files

**Risk 2**: Performance overhead >50ms
- **Impact**: MEDIUM (request latency)
- **Probability**: LOW
- **Mitigation**: Optimize binary parsing, benchmark regularly

---

## Future Enhancements

- Audio quality detection (bitrate, sample rate recommendations)
- Automatic format conversion (e.g., ADPCM → PCM)
- Content-based validation (silence detection, clipping detection)

---

**Story Status**: ✅ READY FOR EXECUTION
**Assignee**: Dev 1A (Team 1 Lead)
**Start Date**: 2026-01-22 (Day 1)
**Expected Completion**: 2026-01-24 (Day 3)
