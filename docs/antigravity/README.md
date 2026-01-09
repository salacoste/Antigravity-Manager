# Antigravity App Analysis

## Overview

Reverse engineering documentation for Google's Antigravity desktop application (macOS).

**Application Path**: `/Applications/Antigravity.app/`
**Version**: TBD
**Technology**: Electron-based (VS Code fork)
**Purpose**: AI assistant with Gemini integration

## Analysis Goals

1. **API Endpoints Discovery**: Identify all Google API endpoints used
2. **Request/Response Structures**: Document payload formats
3. **Authentication Flow**: Understand OAuth/token management
4. **Model Routing**: Determine how model selection works
5. **Network Protocols**: WebSocket vs HTTP patterns

## Documentation Structure

```
docs/antigravity/
├── README.md                    # This file
├── api/
│   ├── endpoints.md             # API endpoints catalog
│   ├── payloads.md              # Request/response structures
│   └── authentication.md        # Auth mechanisms
├── architecture/
│   ├── app-structure.md         # Application structure
│   ├── vs-code-fork.md          # VS Code customizations
│   └── extension-system.md      # Antigravity extension details
├── network/
│   ├── traffic-analysis.md      # Network traffic observations
│   ├── websockets.md            # WebSocket connections
│   └── http-requests.md         # HTTP request patterns
└── findings/
    ├── initial-analysis.md      # First analysis results
    ├── code-patterns.md         # Interesting code patterns
    └── security-notes.md        # Security observations
```

## Analysis Methods

### 1. Static Code Analysis
- **Location**: `/Applications/Antigravity.app/Contents/Resources/app/`
- **Main Extension**: `extensions/antigravity/dist/extension.js` (3.8MB minified)
- **Source Maps**: TBD

### 2. Network Traffic Monitoring
- [ ] mitmproxy setup
- [ ] Chrome DevTools remote debugging
- [ ] macOS network monitoring

### 3. Runtime Analysis
- [ ] Electron DevTools
- [ ] Console logging
- [ ] Extension debugging

## Key Findings

_Updated: 2026-01-09_

### Application Structure
- **Base**: VS Code fork (product name: "Antigravity")
- **Main Extension**: `antigravity` extension in bundled extensions
- **Code**: Heavily minified JavaScript (3.8MB single file)

### Discovered Endpoints
- `https://www.google.com/generate_204` - Connectivity check

### Next Steps
1. [ ] Setup network traffic interception
2. [ ] Enable remote debugging
3. [ ] Deobfuscate extension.js
4. [ ] Map API endpoints
5. [ ] Document authentication flow

## Tools Used

- `grep` - Text pattern search
- `find` - File system exploration
- `asar` - Electron archive unpacker (not needed - code is unpacked)
- `mitmproxy` - Network traffic analysis (planned)
- Chrome DevTools - Runtime debugging (planned)

## References

- [Electron Documentation](https://www.electronjs.org/docs/latest/)
- [VS Code Extension API](https://code.visualstudio.com/api)
- [Google AI APIs](https://ai.google.dev/)
