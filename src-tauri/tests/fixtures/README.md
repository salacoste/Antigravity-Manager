# Test Fixtures for Image Generation E2E Tests

This directory contains test fixtures and mock data for the image generation E2E test suite.

## Directory Structure

```
fixtures/
├── images/          # Test image files (base images, masks)
├── responses/       # Mock API response data
└── README.md        # This file
```

## Usage

### Mock Responses (`responses/`)

Contains JSON files with mock API responses to avoid quota exhaustion during testing.

- `image_generation.json` - Mock responses for image generation tests

**Purpose**: Allow test execution in CI/CD without consuming Google API quota.

### Test Images (`images/`)

Contains sample images for testing image editing endpoints.

**Note**: Currently empty. Live image editing tests are deferred to avoid quota usage.

## Quota Protection Strategy

To prevent quota exhaustion (lesson learned from Epic-006):

1. **Most tests use mocks**: Only validate request/response structure
2. **2 live API tests maximum**: Basic generation + one integration test
3. **CI/CD uses mocks only**: No live API calls in GitHub Actions
4. **Explicit opt-in for live tests**: Use `cargo test -- --ignored`

## Adding New Fixtures

### Mock Response Format

```json
{
  "created": 1704067200,
  "data": [
    {
      "b64_json": "base64_encoded_image_data"
    }
  ]
}
```

### Test Image Requirements

- **Format**: PNG, JPG, or WEBP
- **Size**: Reasonable for testing (< 1MB)
- **Purpose**: Document intended use (base image, mask, etc.)

## Related Documentation

- Test Implementation: `../image_generation_e2e.rs`
- Test Guide: `docs/testing/image-generation-tests.md`
- Epic Documentation: `docs/epics/Epic-007-Gemini-3-Pro-Image-Compliance.md`
