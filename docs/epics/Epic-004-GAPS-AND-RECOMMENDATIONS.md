# Epic-004 Implementation Gaps & Recommendations

**Дата анализа**: 2026-01-11
**Модель**: claude-4.5-sonnet (Standard, NO Thinking)
**Model ID**: 333
**Статус после анализа**: ~95% реализовано (vs 100% в предыдущей оценке)

---

## 🎯 Executive Summary

После детального анализа эталонной модели из reverse engineering и текущей реализации обнаружено **3 критических gap'а** и **2 medium gap'а**, которые нужно устранить для 100% compliance.

**Good News**: 90% функциональности УЖЕ реализована Epic-003 ✅

**Bad News**: Остались несколько критических проблем, которые могут вызвать неправильное поведение для standard model.

---

## 🚨 Critical Gaps (Must Fix - P0)

### GAP #1: User-Agent Hardcoded (КРИТИЧЕСКИЙ)

**Severity**: P0 - CRITICAL
**Impact**: Anti-detection failure, может привести к блокировке
**Effort**: 2 часа

#### Проблема

**Эталон из RE** (`claude-4-5-sonnet-workflow.md:46-50`):
```http
User-Agent: antigravity/1.13.3 {platform}/{arch}
```

Должен быть **динамический**, например:
- macOS ARM64: `antigravity/1.13.3 darwin/arm64`
- Windows x64: `antigravity/1.13.3 windows/x86_64`
- Linux ARM64: `antigravity/1.13.3 linux/arm64`

**Текущая реализация** (`src-tauri/src/proxy/upstream/client.rs:25-26`):
```rust
let user_agent = std::env::var("CLAUDE_USER_AGENT")
    .unwrap_or_else(|_| "antigravity/1.13.3 darwin/arm64".to_string());
```

**Проблема**: User-Agent **HARDCODED** как "darwin/arm64" для всех платформ!

Если proxy запущен на Windows/Linux, он будет отправлять неправильный User-Agent:
- Windows машина → User-Agent: `antigravity/1.13.3 darwin/arm64` ❌
- Linux машина → User-Agent: `antigravity/1.13.3 darwin/arm64` ❌

#### Дополнительная проблема

User-Agent устанавливается в **ДВУХ местах**:
1. Client builder (`client.rs:37`) - используется для всех requests
2. Request headers (`client.rs:102-107`) - устанавливается заново для каждого request

Это дублирование и потенциальная несогласованность.

#### Решение

**Файл**: `src-tauri/src/proxy/upstream/client.rs`

**Шаг 1**: Создать helper function для формирования User-Agent

```rust
// Add at top of file or in a common utils module
use crate::proxy::mappers::claude::request::{get_platform, get_architecture};

// OR copy platform detection functions here if they're private:
fn get_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "linux"  // Fallback
    }
}

fn get_architecture() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        "x86_64"  // Fallback
    }
}

/// Build dynamic User-Agent string based on actual platform/architecture
/// Format: "antigravity/{version} {platform}/{arch}"
fn build_user_agent() -> String {
    const ANTIGRAVITY_VERSION: &str = "1.13.3";
    format!(
        "antigravity/{} {}/{}",
        ANTIGRAVITY_VERSION,
        get_platform(),
        get_architecture()
    )
}
```

**Шаг 2**: Обновить UpstreamClient::new()

```rust
pub fn new(proxy_config: Option<crate::proxy::config::UpstreamProxyConfig>) -> Self {
    // Allow override via env var, otherwise use dynamic detection
    let user_agent = std::env::var("CLAUDE_USER_AGENT")
        .unwrap_or_else(|_| build_user_agent());  // ✅ Dynamic

    tracing::info!("🔧 UpstreamClient User-Agent: {}", user_agent);

    let mut builder = Client::builder()
        // ... other settings ...
        .user_agent(user_agent.clone());  // ✅ Use same value

    // ...
}
```

**Шаг 3**: Обновить call_v1_internal()

```rust
pub async fn call_v1_internal(
    &self,
    method: &str,
    access_token: &str,
    body: Value,
    query_string: Option<&str>,
) -> Result<Response, String> {
    let mut headers = header::HeaderMap::new();

    // ... other headers ...

    // ✅ Use same dynamic User-Agent (remove duplication)
    let user_agent = std::env::var("CLAUDE_USER_AGENT")
        .unwrap_or_else(|_| build_user_agent());
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_str(&user_agent).map_err(|e| e.to_string())?,
    );

    // ...
}
```

**Альтернативное решение**: Store User-Agent в `UpstreamClient` struct:

```rust
pub struct UpstreamClient {
    http_client: Client,
    user_agent: String,  // ✅ Store once
}

impl UpstreamClient {
    pub fn new(...) -> Self {
        let user_agent = std::env::var("CLAUDE_USER_AGENT")
            .unwrap_or_else(|_| build_user_agent());

        // ...

        Self {
            http_client,
            user_agent,  // ✅ Store
        }
    }

    pub async fn call_v1_internal(...) -> Result<Response, String> {
        // Use self.user_agent instead of rebuilding
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str(&self.user_agent)
                .map_err(|e| e.to_string())?,
        );
    }
}
```

#### Тестирование

**Unit Test**:
```rust
#[test]
fn test_build_user_agent_format() {
    let ua = build_user_agent();
    assert!(ua.starts_with("antigravity/1.13.3 "));

    // Should contain platform and architecture
    assert!(ua.contains("darwin/") || ua.contains("windows/") || ua.contains("linux/"));
    assert!(ua.contains("/arm64") || ua.contains("/x86_64"));
}

#[test]
fn test_user_agent_matches_platform() {
    let ua = build_user_agent();

    #[cfg(target_os = "macos")]
    assert!(ua.contains("darwin"));

    #[cfg(target_os = "windows")]
    assert!(ua.contains("windows"));

    #[cfg(target_os = "linux")]
    assert!(ua.contains("linux"));
}

#[test]
fn test_user_agent_matches_architecture() {
    let ua = build_user_agent();

    #[cfg(target_arch = "aarch64")]
    assert!(ua.ends_with("arm64"));

    #[cfg(target_arch = "x86_64")]
    assert!(ua.ends_with("x86_64"));
}
```

**Integration Test**:
```rust
#[tokio::test]
async fn test_upstream_client_user_agent() {
    let client = UpstreamClient::new(None);

    // Mock request to verify User-Agent header
    // (Would need to capture actual HTTP request headers)
    // Verify format: "antigravity/1.13.3 {platform}/{arch}"
}
```

#### Acceptance Criteria

- [ ] User-Agent динамически формируется на основе реальной platform/architecture
- [ ] Windows: `antigravity/1.13.3 windows/x86_64` или `windows/arm64`
- [ ] Linux: `antigravity/1.13.3 linux/x86_64` или `linux/arm64`
- [ ] macOS: `antigravity/1.13.3 darwin/arm64` или `darwin/x86_64`
- [ ] Env var override (`CLAUDE_USER_AGENT`) продолжает работать
- [ ] User-Agent НЕ дублируется (один источник правды)
- [ ] Unit tests проходят на всех платформах
- [ ] Integration test проверяет формат User-Agent в HTTP headers

---

### GAP #2: Thinking Mode Detection Logic (КРИТИЧЕСКИЙ)

**Severity**: P0 - CRITICAL for claude-4.5-sonnet
**Impact**: Standard model может случайно активировать thinking mode
**Effort**: 3 часа

#### Проблема

**Текущая логика** (`request.rs:329-331`):
```rust
let target_model_supports_thinking = mapped_model.contains("-thinking")
    || mapped_model.starts_with("claude-")
    || mapped_model.starts_with("gemini-");
```

**Проблема**: Эта логика говорит что **ВСЕ Claude модели** поддерживают thinking mode!

Для "claude-4.5-sonnet" (standard):
- `mapped_model = "claude-4.5-sonnet"`
- `mapped_model.starts_with("claude-")` → **TRUE** ❌
- `target_model_supports_thinking` → **TRUE** ❌ НЕПРАВИЛЬНО!

#### Сценарий ошибки

1. Пользователь явно отправляет:
   ```json
   {
     "model": "claude-4.5-sonnet",
     "thinking": { "type": "enabled" },
     "messages": [...]
   }
   ```

2. `is_thinking_enabled` = true (явно включено)
3. `target_model_supports_thinking` = true (потому что `starts_with("claude-")`)
4. Проверка `if is_thinking_enabled && !target_model_supports_thinking` **НЕ СРАБОТАЕТ**
5. Система НЕ отключит thinking mode
6. Request будет отправлен с `thinkingConfig` для standard model
7. Model ID будет 333, но будет thinkingConfig → **НЕСОГЛАСОВАННОСТЬ** ❌

#### Решение

**Option A**: Explicit suffix check (Recommended)

```rust
let target_model_supports_thinking = mapped_model.contains("-thinking");
```

**Обоснование**: Только модели с суффиксом "-thinking" поддерживают thinking mode для Claude.

**Option B**: Explicit whitelist

```rust
fn model_supports_thinking(model_name: &str) -> bool {
    matches!(
        model_name,
        "claude-4.5-sonnet-thinking"
            | "claude-opus-4-5-thinking"
            | "gemini-3-pro-high"
            | "gemini-2.5-pro"
    )
}

let target_model_supports_thinking = model_supports_thinking(&mapped_model);
```

**Обоснование**: Explicit whitelist более безопасен, но требует обновления при добавлении новых моделей.

**Option C**: Check model ID

```rust
fn model_supports_thinking_by_id(model_name: &str) -> bool {
    let model_id = get_model_id(model_name);

    // Only specific model IDs support thinking
    matches!(
        model_id,
        334 | // claude-4.5-sonnet-thinking
        335 | // claude-opus-4-5-thinking
        // ... other thinking-capable model IDs
        _ => false
    )
}
```

**Рекомендация**: **Option A** (suffix check) - самый простой и надежный.

#### Дополнительное улучшение

Добавить explicit logging когда thinking mode отключается:

```rust
if is_thinking_enabled && !target_model_supports_thinking {
    tracing::warn!(
        "[Thinking-Mode] Model '{}' does NOT support thinking mode. \
         User requested thinking: {{ type: 'enabled' }}, but it will be IGNORED. \
         Model ID: {}",
        mapped_model,
        get_model_id(&mapped_model)
    );
    is_thinking_enabled = false;
}
```

#### Тестирование

**Unit Tests**:
```rust
#[test]
fn test_standard_sonnet_does_not_support_thinking() {
    // claude-4.5-sonnet (standard) should NOT support thinking
    assert!(!model_supports_thinking("claude-4.5-sonnet"));

    // Only thinking variant supports it
    assert!(model_supports_thinking("claude-4.5-sonnet-thinking"));
}

#[test]
fn test_explicit_thinking_ignored_for_standard_model() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // Standard model
        thinking: Some(ThinkingConfig {
            type_: "enabled".to_string(),
            budget_tokens: Some(4096),
        }),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        // ... other fields ...
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // Should have model ID 333 (standard)
    assert_eq!(body["modelId"], 333);

    // Should NOT have thinkingConfig
    let gen_config = &body["request"]["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());
}
```

#### Acceptance Criteria

- [ ] Standard model (claude-4.5-sonnet) НЕ поддерживает thinking mode
- [ ] Явный `thinking: { type: "enabled" }` игнорируется для standard model
- [ ] Warning логируется когда thinking игнорируется
- [ ] Thinking variant (claude-4.5-sonnet-thinking) продолжает работать
- [ ] Model ID 333 НИКОГДА не сочетается с thinkingConfig
- [ ] Unit tests проходят для обоих сценариев

---

### GAP #3: Missing Integration Tests for Standard Model

**Severity**: P0 - CRITICAL (regression prevention)
**Impact**: Нет автоматической проверки правильности работы standard model
**Effort**: 3 часа

#### Проблема

**Текущее состояние тестов**:
- ✅ Unit tests для `get_model_id("claude-4.5-sonnet")` → 333 EXISTS
- ✅ Unit tests для API provider (26) и model provider (3) EXISTS
- ❌ Integration tests используют только "claude-4.5-sonnet-thinking" (334)
- ❌ НЕТ integration tests для full request transformation с model ID 333
- ❌ НЕТ tests проверяющих отсутствие thinkingConfig для standard
- ❌ НЕТ regression tests для предотвращения thinking activation

**Найденные integration tests** используют только thinking model:
```rust
#[test]
fn test_request_includes_model_id() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),  // ❌ Only thinking!
        // ...
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();
    assert_eq!(body["modelId"], 334);  // ❌ Only checks 334, not 333!
}
```

#### Решение

**Создать comprehensive test suite для claude-4.5-sonnet (standard)**

**Файл**: `src-tauri/src/proxy/mappers/claude/request.rs` (в конце test module)

```rust
// ==================== Story #1 (Epic-004): Standard Model (ID 333) Tests ====================
// Complete integration tests for claude-4.5-sonnet (standard, no thinking)
// Reference: docs/epics/Epic-004-Claude-4.5-Sonnet-Standard-Compliance.md

#[test]
fn test_standard_sonnet_model_id_333() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // ✅ Standard model
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(2048),
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,  // NO thinking config
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let result = transform_claude_request_in(&req, "test-project");
    assert!(result.is_ok());

    let (body, _violations) = result.unwrap();

    // ✅ Should have model ID 333 (standard, not 334)
    assert_eq!(body["modelId"], 333);

    // ✅ Should have correct providers
    assert_eq!(body["apiProvider"], 26);
    assert_eq!(body["modelProvider"], 3);

    // ✅ Should have model name
    assert_eq!(body["model"], "claude-4.5-sonnet");
}

#[test]
fn test_standard_sonnet_no_thinking_config() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Write a Python function".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(4096),
        temperature: Some(0.7),
        top_p: None,
        top_k: None,
        thinking: None,  // NO thinking
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should NOT have thinkingConfig in generationConfig
    let gen_config = &body["request"]["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());

    // ✅ Should have other generation config fields
    assert_eq!(gen_config["maxOutputTokens"], 4096);
    assert_eq!(gen_config["temperature"], 0.7);
}

#[test]
fn test_standard_sonnet_explicit_thinking_ignored() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),  // Standard model
        thinking: Some(ThinkingConfig {
            type_: "enabled".to_string(),
            budget_tokens: Some(3000),
        }),  // ❌ Trying to enable thinking
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: Some(8192),
        temperature: None,
        top_p: None,
        top_k: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Model ID should be 333 (standard)
    assert_eq!(body["modelId"], 333);

    // ✅ thinkingConfig should be IGNORED (not present)
    let gen_config = &body["request"]["generationConfig"];
    assert!(gen_config["thinkingConfig"].is_null());

    // ✅ maxOutputTokens should be preserved
    assert_eq!(gen_config["maxOutputTokens"], 8192);
}

#[test]
fn test_standard_sonnet_metadata_compliance() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Hello".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: Some(Metadata {
            user_id: Some("test-user-123".to_string()),
            workspace_id: Some("workspace-abc".to_string()),
            cloudaicompanion_project: Some("project-xyz".to_string()),
        }),
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have Antigravity metadata
    let metadata = &body["request"]["metadata"];
    assert_eq!(metadata["ideType"], "ANTIGRAVITY");
    assert_eq!(metadata["ideVersion"], "1.13.3");
    assert!(metadata["platform"].is_string());
    assert!(metadata["architecture"].is_string());

    // ✅ Should have extended session metadata
    assert_eq!(metadata["sessionId"], "test-user-123");
    assert_eq!(metadata["workspace_id"], "workspace-abc");
    assert_eq!(metadata["cloudaicompanion_project"], "project-xyz");
}

#[test]
fn test_standard_sonnet_tool_configuration() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: Some(vec![Tool {
            type_: None,
            name: Some("test_tool".to_string()),
            description: Some("A test tool".to_string()),
            input_schema: Some(json!({
                "type": "object",
                "properties": {
                    "param": { "type": "string" }
                },
                "required": ["param"]
            })),
        }]),
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: Some(ToolChoice::Auto),  // AUTO mode
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have tool configuration
    assert!(body["request"]["tools"].is_array());

    // ✅ Should have correct tool config mode
    let tool_config = &body["request"]["toolConfig"]["functionCallingConfig"];
    assert_eq!(tool_config["mode"], "AUTO");
}

#[test]
fn test_standard_sonnet_grounding_config() {
    let req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let (body, _violations) = transform_claude_request_in(&req, "test-project").unwrap();

    // ✅ Should have geminiSettings with recitationPolicy
    let gemini_settings = &body["request"]["geminiSettings"];
    assert_eq!(gemini_settings["recitationPolicy"]["action"], "BLOCK");
    assert_eq!(gemini_settings["recitationPolicy"]["threshold"], "LOW");
}

#[test]
fn test_standard_vs_thinking_model_ids() {
    let standard_req = ClaudeRequest {
        model: "claude-4.5-sonnet".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::String("Test".to_string()),
        }],
        system: None,
        tools: None,
        stream: false,
        max_tokens: None,
        temperature: None,
        top_p: None,
        top_k: None,
        thinking: None,
        metadata: None,
        output_config: None,
        tool_choice: None,
    };

    let thinking_req = ClaudeRequest {
        model: "claude-4.5-sonnet-thinking".to_string(),
        ..standard_req.clone()
    };

    let (standard_body, _) = transform_claude_request_in(&standard_req, "test-project").unwrap();
    let (thinking_body, _) = transform_claude_request_in(&thinking_req, "test-project").unwrap();

    // ✅ Model IDs should be different
    assert_eq!(standard_body["modelId"], 333);
    assert_eq!(thinking_body["modelId"], 334);

    // ✅ API/Model providers should be same (both Anthropic via Vertex)
    assert_eq!(standard_body["apiProvider"], 26);
    assert_eq!(thinking_body["apiProvider"], 26);
    assert_eq!(standard_body["modelProvider"], 3);
    assert_eq!(thinking_body["modelProvider"], 3);
}
```

#### Acceptance Criteria

- [ ] Все integration tests для standard model проходят
- [ ] Model ID 333 корректно устанавливается
- [ ] thinkingConfig НИКОГДА не присутствует для standard model
- [ ] Metadata (ideType, platform, arch) присутствует
- [ ] Tool configuration работает (AUTO/ANY/NONE/VALIDATED)
- [ ] Grounding config (geminiSettings) присутствует
- [ ] Extended session metadata работает
- [ ] Regression tests предотвращают thinking activation
- [ ] Tests проходят в CI/CD pipeline

---

## ⚠️ Medium Priority Gaps (Should Fix - P1)

### GAP #4: Platform Detection Code Duplication

**Severity**: P1 - MEDIUM
**Impact**: Code duplication, maintenance burden
**Effort**: 1 час

#### Проблема

Функции `get_platform()` и `get_architecture()` определены в `request.rs`, но НЕ используются в `client.rs` для User-Agent.

**Текущее состояние**:
- `request.rs:213-239` - Определяет `get_platform()` и `get_architecture()` для metadata
- `client.rs:25-26` - Hardcoded User-Agent без использования этих функций
- **ДУБЛИРОВАНИЕ ЛОГИКИ**: Если мы исправим GAP #1, нам придется дублировать функции

#### Решение

**Option A**: Move to common module

```rust
// Create src-tauri/src/proxy/common/platform.rs

/// Detect platform using compile-time cfg macros
pub fn get_platform() -> &'static str {
    if cfg!(target_os = "macos") {
        "darwin"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "linux"
    }
}

/// Detect CPU architecture using compile-time cfg macros
pub fn get_architecture() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        "x86_64"
    }
}

/// Build User-Agent string for Antigravity compliance
pub fn build_user_agent() -> String {
    const ANTIGRAVITY_VERSION: &str = "1.13.3";
    format!(
        "antigravity/{} {}/{}",
        ANTIGRAVITY_VERSION,
        get_platform(),
        get_architecture()
    )
}
```

**Update imports**:
```rust
// In request.rs
use crate::proxy::common::platform::{get_platform, get_architecture};

// In client.rs
use crate::proxy::common::platform::build_user_agent;
```

**Option B**: Re-export from request.rs

```rust
// In request.rs - make functions public
pub fn get_platform() -> &'static str { ... }
pub fn get_architecture() -> &'static str { ... }

// In client.rs
use crate::proxy::mappers::claude::request::{get_platform, get_architecture};
```

**Рекомендация**: **Option A** - common module более чистый и явный.

#### Acceptance Criteria

- [ ] Platform detection functions в одном месте
- [ ] Request mapper использует common functions
- [ ] Upstream client использует common functions
- [ ] User-Agent строится через helper function
- [ ] Нет дублирования кода

---

### GAP #5: Missing Validation Logging

**Severity**: P1 - MEDIUM
**Impact**: Сложнее debug, меньше observability
**Effort**: 1 час

#### Проблема

Недостаточно logging для валидации standard model behavior:
- Не логируется когда thinking игнорируется
- Не логируется model ID routing decision
- Не логируется User-Agent формирование
- Не логируется metadata injection

#### Решение

**Добавить structured logging** на критических точках:

**1. Model ID routing** (`request.rs:176-183`):
```rust
fn get_model_id(model_name: &str) -> u32 {
    let model_id = match model_name {
        "claude-4.5-sonnet-thinking" => 334,
        "claude-4.5-sonnet" => 333,
        _ => 0,
    };

    tracing::debug!(
        "[Model-Routing] Model: '{}' → Model ID: {}",
        model_name,
        model_id
    );

    model_id
}
```

**2. Thinking mode detection** (уже частично есть, улучшить):
```rust
if is_thinking_enabled && !target_model_supports_thinking {
    tracing::warn!(
        "[Thinking-Mode] ⚠️  Model '{}' (ID: {}) does NOT support thinking. \
         User requested thinking: {{ type: 'enabled' }}, but it will be IGNORED.",
        mapped_model,
        get_model_id(&mapped_model)
    );
    is_thinking_enabled = false;
}
```

**3. User-Agent formation** (`client.rs`):
```rust
fn build_user_agent() -> String {
    let platform = get_platform();
    let arch = get_architecture();
    let ua = format!("antigravity/1.13.3 {}/{}", platform, arch);

    tracing::debug!(
        "[User-Agent] Built: '{}' (platform: {}, arch: {})",
        ua, platform, arch
    );

    ua
}
```

**4. Metadata injection** (`request.rs:539-567`):
```rust
let mut metadata = json!({
    "ideType": IDE_TYPE,
    "ideVersion": IDE_VERSION,
    "platform": get_platform(),
    "architecture": get_architecture()
});

tracing::debug!(
    "[Metadata-Injection] ideType: {}, ideVersion: {}, platform: {}, arch: {}",
    IDE_TYPE,
    IDE_VERSION,
    get_platform(),
    get_architecture()
);
```

#### Acceptance Criteria

- [ ] Model ID routing логируется
- [ ] Thinking mode decisions логируются
- [ ] User-Agent формирование логируется
- [ ] Metadata injection логируется
- [ ] Logs structured и машиночитаемые
- [ ] Debug level для normal operations
- [ ] Warn level для potential issues

---

## 📊 Summary

### Gap Overview

| Gap ID | Priority | Description | Effort | Impact |
|--------|----------|-------------|--------|--------|
| GAP #1 | P0 | User-Agent hardcoded (darwin/arm64) | 2h | CRITICAL - Anti-detection |
| GAP #2 | P0 | Thinking mode detection logic | 3h | CRITICAL - Incorrect behavior |
| GAP #3 | P0 | Missing integration tests | 3h | CRITICAL - No regression prevention |
| GAP #4 | P1 | Platform detection duplication | 1h | MEDIUM - Maintenance |
| GAP #5 | P1 | Missing validation logging | 1h | MEDIUM - Observability |

**Total Effort**:
- **P0 (Critical)**: 8 hours
- **P1 (Medium)**: 2 hours
- **Total**: 10 hours

### Implementation Priority

**Phase 1 (P0 - Critical)**: 8 hours
1. GAP #1: User-Agent dynamic generation (2h)
2. GAP #2: Thinking mode detection fix (3h)
3. GAP #3: Integration tests suite (3h)

**Phase 2 (P1 - Medium)**: 2 hours
4. GAP #4: Code deduplication (1h)
5. GAP #5: Enhanced logging (1h)

### Testing Strategy

**Unit Tests**: 15+ new tests
- Platform detection functions
- User-Agent formation
- Thinking mode detection
- Model ID routing

**Integration Tests**: 8+ new tests
- Full request transformation for standard model
- Model ID 333 validation
- No thinkingConfig assertion
- Metadata compliance
- Tool configuration
- Grounding config

**Regression Tests**: 3+ tests
- Prevent thinking activation for standard
- Prevent Model ID mixup (333 ↔ 334)
- Ensure User-Agent matches platform

---

## 🎯 Updated Epic-004 Scope

### Original Scope (Epic-004 spec)
- 6 stories, 9 hours
- **Status**: ~95% implemented (Epic-003 did most of the work)

### New Scope (Gap Remediation)
- 5 gaps, 10 hours
- **Status**: Ready for implementation

### Recommended Approach

**Option 1: Transform Epic-004 to Gap Remediation Epic**
- Rename stories to focus on gaps
- Keep 9-10 hour estimate
- Focus on fixes + testing

**Option 2: Create New "Epic-004.5: Standard Model Validation"**
- Keep Epic-004 as "Implemented via Epic-003"
- Create new mini-epic for gaps
- Clear separation of implementation vs validation

**Recommendation**: **Option 1** - Transform Epic-004 ⭐

---

## ✅ Validation Checklist

Before marking Epic-004 complete:

**Code Fixes**:
- [ ] GAP #1: User-Agent динамический (darwin/windows/linux + arm64/x86_64)
- [ ] GAP #2: Thinking mode detection исправлен (только "-thinking" suffix)
- [ ] GAP #4: Platform functions в common module
- [ ] GAP #5: Enhanced logging добавлен

**Testing**:
- [ ] GAP #3: 8+ integration tests для standard model написаны и проходят
- [ ] 15+ unit tests написаны и проходят
- [ ] 3+ regression tests предотвращают thinking activation
- [ ] All tests проходят в CI/CD

**Documentation**:
- [ ] Epic-004 spec обновлен (gap focus)
- [ ] COMPREHENSIVE-ANALYSIS updated (95% → 100%)
- [ ] Code comments ссылаются на Epic-004 stories
- [ ] GAPS-AND-RECOMMENDATIONS archived после fix

**Compliance**:
- [ ] Model ID 333 для "claude-4.5-sonnet" ✅
- [ ] User-Agent: "antigravity/1.13.3 {platform}/{arch}" ✅
- [ ] NO thinkingConfig for standard model ✅
- [ ] ideType: ANTIGRAVITY metadata ✅
- [ ] Tool config (AUTO/ANY/NONE/VALIDATED) ✅
- [ ] Grounding config (geminiSettings) ✅
- [ ] Extended session metadata ✅

---

**Document Status**: ✅ COMPLETE
**Next Step**: Review gaps with Product Owner and decide on implementation approach
**Estimated Time to 100% Compliance**: 10 hours (1.25 days)
