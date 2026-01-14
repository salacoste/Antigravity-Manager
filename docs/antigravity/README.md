# Antigravity App Analysis

## Overview

Comprehensive reverse engineering documentation and tooling for Google's Antigravity desktop application (macOS).

**Application Path**: `/Applications/Antigravity.app/`
**Version**: TBD (extract from app to determine)
**Technology**: Electron-based (VS Code fork)
**Purpose**: AI assistant with Gemini integration

## 🚀 Quick Start

**New to reverse engineering? Start here:**

1. **[QUICK-START.md](QUICK-START.md)** - 3-step guide to extract and de-minify code (15 minutes)
2. **[REVERSE-ENGINEERING-WORKFLOW.md](REVERSE-ENGINEERING-WORKFLOW.md)** - Complete workflow documentation
3. **[tools/README.md](tools/README.md)** - Automation tools reference

**Complete setup:**
```bash
cd docs/antigravity/tools
npm install                  # Install de-minification tools
npm run extract              # Extract source from .app bundle
npm run deminify             # De-minify code for analysis
code ../deminified/          # Open for analysis
```

## Analysis Goals

1. **API Endpoints Discovery**: Identify all Google API endpoints used
2. **Request/Response Structures**: Document payload formats
3. **Authentication Flow**: Understand OAuth/token management
4. **Model Routing**: Determine how model selection works
5. **Network Protocols**: WebSocket vs HTTP patterns
6. **Code De-minification**: Convert minified code to readable format
7. **Architecture Mapping**: Document VS Code customizations

## 📁 Documentation Structure

```
docs/antigravity/
├── README.md                           # This file - Overview and index
├── QUICK-START.md                      # 🚀 3-step quick start guide
├── REVERSE-ENGINEERING-WORKFLOW.md     # 📚 Complete workflow documentation
│
├── tools/                              # 🛠️ De-minification toolkit
│   ├── package.json                    # Node.js dependencies
│   ├── README.md                       # Tools documentation
│   └── node_modules/                   # Babel, Prettier, js-beautify
│
├── scripts/                            # 🤖 Automation scripts
│   ├── extract.js                      # Extract source from .app bundle
│   ├── deminify.js                     # Multi-stage de-minification
│   └── analyze.js                      # Analysis utilities (TBD)
│
├── extracted/                          # 📦 Raw extracted files (generated)
│   ├── extraction-metadata.json        # Extraction statistics
│   └── out/vs/                         # VS Code framework files
│
├── deminified/                         # ✨ De-minified code (generated)
│   ├── deminification-log.json         # Processing results
│   └── out/vs/                         # Readable source code
│
├── api/                                # 🌐 API documentation
│   ├── endpoints.md                    # API endpoints catalog
│   ├── payloads.md                     # Request/response structures
│   └── authentication.md               # Auth mechanisms
│
├── architecture/                       # 🏗️ Architecture documentation
│   ├── app-structure.md                # Application structure
│   ├── vs-code-fork.md                 # VS Code customizations
│   └── extension-system.md             # Antigravity extension details
│
├── network/                            # 📡 Network analysis
│   ├── traffic-analysis.md             # Network traffic observations
│   ├── websockets.md                   # WebSocket connections
│   └── http-requests.md                # HTTP request patterns
│
└── findings/                           # 🔍 Research findings
    ├── initial-analysis.md             # First analysis results
    ├── code-patterns.md                # Interesting code patterns
    └── security-notes.md               # Security observations
```

## 🔬 Analysis Methods

### 1. Static Code Analysis (✅ Ready)

**Automated De-minification Pipeline:**
- **Extraction**: `npm run extract` copies source from `.app` bundle
- **De-minification**: Multi-stage pipeline (Babel → Prettier → js-beautify)
- **Analysis**: Open `deminified/` in VS Code or preferred editor

**Key Locations:**
- **Application Bundle**: `/Applications/Antigravity.app/Contents/Resources/app/`
- **Main Files**:
  - `out/vs/workbench/workbench.desktop.main.js` (Main workbench, ~2.3MB minified)
  - `out/vs/code/electron-browser/workbench/jetskiAgent.js` (Jetski agent)
  - `extensions/antigravity/dist/extension.js` (3.8MB minified)

**Tools:**
- Babel AST parser for proper JavaScript formatting
- Prettier for consistent code style
- js-beautify for fallback beautification

### 2. Network Traffic Monitoring (Planned)

- [ ] mitmproxy setup for HTTPS interception
- [ ] Chrome DevTools remote debugging
- [ ] macOS network monitoring (Wireshark/Charles Proxy)
- [ ] WebSocket traffic capture

### 3. Runtime Analysis (Planned)

- [ ] Electron DevTools access
- [ ] Console logging injection
- [ ] Extension debugging mode
- [ ] Behavior profiling

## 🔑 Key Findings

_Updated: 2026-01-09_

### Application Structure
- **Base**: VS Code fork (product name: "Antigravity")
- **Main Extension**: `antigravity` extension in bundled extensions
- **Code**: Heavily minified JavaScript
  - Main workbench: ~2.3MB (65K+ character lines)
  - Extension: 3.8MB single file
  - Total: ~200 JavaScript files in VS Code framework

### De-minification Results
- ✅ **Extraction**: Automated via `extract.js` script
- ✅ **De-minification**: 3-stage pipeline successfully formats code
- ✅ **Success Rate**: ~94% (186/198 files de-minified successfully)
- ⚠️ **Variable Names**: Remain mangled (no source maps available)

### Discovered Endpoints
- `https://www.google.com/generate_204` - Connectivity check
- _(More to be documented after network analysis)_

### Next Steps - Updated Workflow
1. ✅ ~~Setup de-minification tools~~ **COMPLETED**
2. ✅ ~~Create extraction scripts~~ **COMPLETED**
3. [ ] Run extraction and de-minification
4. [ ] Analyze de-minified code for custom implementations
5. [ ] Setup network traffic interception
6. [ ] Map API endpoints and protocols
7. [ ] Document authentication flow
8. [ ] Reverse engineer Jetski agent logic

## 🧰 Tools Used

### De-minification Tools (✅ Ready)
- **Node.js** - Runtime for automation scripts
- **@babel/core** - AST-based JavaScript parsing and formatting
- **Prettier** - Consistent code style formatting
- **js-beautify** - Fallback beautification
- **Custom Scripts** - `extract.js`, `deminify.js` automation

### Analysis Tools
- **grep** - Text pattern search in de-minified code
- **VS Code** - IDE for exploring de-minified codebase
- **find** - File system exploration

### Network Analysis Tools (Planned)
- **mitmproxy** - HTTPS traffic interception
- **Charles Proxy** - GUI-based HTTP/HTTPS proxy
- **Wireshark** - Low-level packet capture
- **Chrome DevTools** - WebSocket and fetch inspection

### Runtime Debugging (Planned)
- **Electron DevTools** - Application debugging
- **Chrome Remote Debugging** - Protocol inspection

## References

- [Electron Documentation](https://www.electronjs.org/docs/latest/)
- [VS Code Extension API](https://code.visualstudio.com/api)
- [Google AI APIs](https://ai.google.dev/)
