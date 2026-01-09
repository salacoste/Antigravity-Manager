# Initial Analysis Findings

**Date**: 2026-01-09
**Analyst**: Claude Code
**Application**: Google Antigravity (macOS)

## Summary

Google Antigravity is an Electron-based desktop application built as a fork of VS Code. The main functionality is implemented as a built-in extension with heavily minified code.

## Discovered Endpoints

### Connectivity Check
```
URL: https://www.google.com/generate_204
Purpose: Network connectivity validation
Method: GET (likely)
Response: HTTP 204 No Content
```

This is Google's standard connectivity check endpoint, commonly used to verify internet access.

## Code Analysis Attempts

### Attempt 1: String Pattern Search
**Command**: `grep -o '"[a-zA-Z]*://[^"]*"' extension.js`
**Result**: Found connectivity check endpoint only
**Conclusion**: API endpoints likely constructed dynamically or heavily obfuscated

### Attempt 2: Google APIs Pattern Search
**Command**: `grep -a -o 'https://[a-zA-Z0-9.-]*\.googleapis\.com[^"]*' extension.js`
**Result**: No matches found
**Conclusion**: Endpoints may be:
- Built from string concatenation
- Stored in encoded/encrypted form
- Loaded from external configuration
- Hidden in minified variable names

### Attempt 3: Gemini-Specific Patterns
**Command**: `grep -o 'streamGenerateContent\|generateContent\|v1beta\|models/gemini' extension.js`
**Result**: No matches found
**Conclusion**: API method names are obfuscated or use generic HTTP client

## File System Analysis

### Main Extension Bundle
```
File: /Applications/Antigravity.app/Contents/Resources/app/extensions/antigravity/dist/extension.js
Size: ~3.8MB
Format: Minified JavaScript
Characteristics:
- Single-file bundle
- No line breaks
- Shortened variable names
- No comments
```

### Configuration Schema
```
File: /Applications/Antigravity.app/Contents/Resources/app/extensions/antigravity/schemas/mcp_config.schema.json
Purpose: MCP (Model Context Protocol) configuration
Significance: Indicates extensibility through MCP servers
```

## Architecture Insights

### VS Code Fork Customizations
1. **Branding**: Full rebrand to "Antigravity"
2. **Built-in Extension**: Antigravity extension included by default
3. **MCP Support**: Native integration with Model Context Protocol
4. **Update Server**: Likely custom update infrastructure

### Extension System
- **Type**: VS Code extension architecture
- **Activation**: Automatic (built-in extension)
- **Isolation**: Runs in extension host process
- **IPC**: Communicates with main process via Electron IPC

## Challenges Encountered

### Code Obfuscation
❌ **No Source Maps**: Cannot map minified code to original source
❌ **Heavy Minification**: Single-letter variable names, removed whitespace
❌ **Bundling**: All modules combined into single file
⚠️ **Dynamic Construction**: Likely uses runtime string building for URLs

### Limited Static Analysis
- String pattern matching ineffective
- Regex searches return minimal results
- Manual code reading impractical (3.8MB single line)

## Recommended Next Steps

### 1. Network Traffic Interception ⭐ (Highest Priority)
**Why**: Bypass code obfuscation entirely by observing actual network traffic
**Tools**:
- mitmproxy (HTTPS interception)
- Charles Proxy
- Wireshark
- macOS network monitoring

**Setup**:
```bash
# Install mitmproxy
brew install mitmproxy

# Run with transparent proxy
mitmproxy --mode transparent

# Or use regular proxy mode
mitmproxy -p 8080
```

### 2. Remote Debugging
**Why**: Access runtime values, set breakpoints, inspect variables
**Setup**:
```bash
# Launch with debugging enabled
/Applications/Antigravity.app/Contents/MacOS/Antigravity --remote-debugging-port=9222

# Access DevTools
# Open in Chrome: chrome://inspect
# Or directly: http://localhost:9222
```

### 3. Code Deobfuscation
**Why**: Make static analysis feasible
**Tools**:
- JavaScript beautifier (prettier, js-beautify)
- AST-based deobfuscation
- Manual symbol renaming

**Approach**:
```bash
# Beautify the code
npx prettier extension.js --write

# Or use js-beautify
npm install -g js-beautify
js-beautify extension.js -o extension.pretty.js
```

### 4. Binary Analysis
**Why**: May reveal compiled secrets or configurations
**Target**: Main Antigravity executable
**Tools**:
- `strings` command
- Hopper Disassembler
- IDA Pro

### 5. Configuration File Search
**Why**: May contain API endpoints or credentials
**Locations**:
- `~/Library/Application Support/Antigravity/`
- `~/Library/Preferences/com.google.antigravity.plist`
- Application bundle Resources folder

## Security Considerations

### Ethical Analysis
✅ **Legitimate Purpose**: Understanding API integration for compatible proxy development
✅ **Personal Use**: Analyzing locally installed application
⚠️ **Terms of Service**: Review Google's ToS regarding reverse engineering
⚠️ **API Abuse**: Do not use findings for unauthorized API access

### Data Handling
🔒 **Tokens**: Any discovered OAuth tokens are user-specific
🔒 **API Keys**: Embedded keys are Google's property
🔒 **Privacy**: Do not log or share personal data from app

## Expected Findings (Hypothesis)

Based on similar Google AI applications, we expect to find:

### API Endpoints
- `https://generativelanguage.googleapis.com/v1beta/models/gemini-*:generateContent`
- `https://generativelanguage.googleapis.com/v1beta/models/gemini-*:streamGenerateContent`
- OAuth 2.0 endpoints for Google authentication
- Quota/billing endpoints

### Request Format
```typescript
// Hypothetical structure
{
  model: "models/gemini-pro",
  contents: [{
    role: "user",
    parts: [{ text: "..." }]
  }],
  generationConfig: {
    temperature: 0.7,
    topP: 0.95,
    maxOutputTokens: 2048
  },
  safetySettings: [...]
}
```

### Authentication
- OAuth 2.0 with Google Identity
- Bearer tokens in Authorization header
- Refresh token flow for session persistence

## Conclusions

1. **Static analysis is limited** due to heavy code obfuscation
2. **Network interception is most promising** approach for endpoint discovery
3. **Runtime debugging** will reveal actual API usage patterns
4. **MCP integration** suggests extensible architecture worth exploring

## Action Items

- [ ] Set up mitmproxy for traffic interception
- [ ] Enable remote debugging and explore runtime
- [ ] Beautify extension.js for better readability
- [ ] Search configuration directories for API settings
- [ ] Document all discovered endpoints in `api/endpoints.md`
- [ ] Map request/response structures in `api/payloads.md`

## References

- [mitmproxy Documentation](https://docs.mitmproxy.org/)
- [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/)
- [Electron Debugging Guide](https://www.electronjs.org/docs/latest/tutorial/debugging-main-process)
