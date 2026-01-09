# Antigravity Application Structure

## Application Bundle

**Path**: `/Applications/Antigravity.app/`

```
Antigravity.app/
└── Contents/
    ├── MacOS/
    │   └── Antigravity              # Main executable
    ├── Resources/
    │   ├── app/                     # Application code (unpacked)
    │   │   ├── product.json         # Product configuration
    │   │   ├── extensions/          # Built-in extensions
    │   │   │   └── antigravity/     # Main Antigravity extension
    │   │   └── node_modules.asar    # Node dependencies (packed)
    │   └── ...
    └── Info.plist                   # macOS app metadata
```

## Technology Stack

### Base Framework
- **Electron**: Desktop application framework
- **VS Code**: Forked from Microsoft Visual Studio Code
- **Product Name**: "Antigravity" (configured in product.json)

### Code Organization
- **Main Process**: Standard Electron main process
- **Renderer Process**: VS Code-based UI
- **Extensions**: Antigravity functionality as VS Code extension

## Antigravity Extension

**Location**: `/Applications/Antigravity.app/Contents/Resources/app/extensions/antigravity/`

### Directory Structure

```
antigravity/
├── package.json                 # Extension manifest
├── dist/
│   └── extension.js             # Bundled extension code (3.8MB, minified)
└── schemas/
    └── mcp_config.schema.json   # MCP configuration schema
```

### Extension Details

**File**: `package.json`
- Extension ID: TBD
- Activation events: TBD
- Contributes: Commands, views, configuration

**File**: `dist/extension.js`
- Size: ~3.8MB
- Format: Minified JavaScript bundle
- Contains: All Antigravity logic (API clients, UI, authentication)

### MCP Configuration Schema

**File**: `schemas/mcp_config.schema.json`
- Purpose: Configuration schema for Model Context Protocol
- Indicates: MCP integration for extending AI capabilities

## Product Configuration

**File**: `/Applications/Antigravity.app/Contents/Resources/app/product.json`

Key properties:
- `nameShort`: "Antigravity"
- `nameLong`: "Antigravity"
- `applicationName`: "antigravity"
- `win32AppId`: TBD
- `extensionsGallery`: TBD
- `quality`: TBD

## Code Characteristics

### Bundling Strategy
- **Main Code**: Single bundled file (`extension.js`)
- **Dependencies**: Packed in `node_modules.asar`
- **Assets**: Unpacked in extension folder

### Minification
- **Level**: Heavy minification
- **Source Maps**: Not found in standard locations
- **Deobfuscation**: Required for detailed analysis

## Notable Absences

❌ **No .asar Archive**: Application code is unpacked (unusual for production Electron apps)
❌ **No Source Maps**: Cannot easily trace back to original source
❌ **No TypeScript Sources**: Only compiled JavaScript

## Comparison to Standard VS Code

### Similarities
- Same extension architecture
- Same UI framework
- Same configuration system

### Differences
- Custom branding ("Antigravity")
- Built-in `antigravity` extension
- MCP integration
- Likely custom update server
- Custom telemetry endpoints

## Security Observations

### Code Access
- ✅ Application code is accessible (not encrypted)
- ✅ Can be analyzed statically
- ⚠️ Minification provides obfuscation

### Network Security
- 🔍 HTTPS endpoints expected
- 🔍 OAuth tokens likely stored in system keychain
- 🔍 API keys embedded in code (to be verified)

## Next Analysis Steps

1. Extract `package.json` details from antigravity extension
2. Deobfuscate `extension.js` to improve readability
3. Search for API endpoint strings in minified code
4. Identify OAuth flow implementation
5. Map command contributions and UI components

## Analysis Date
Created: 2026-01-09
Last Updated: 2026-01-09
