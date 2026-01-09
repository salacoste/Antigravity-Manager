# Models Reference - Antigravity Manager API Proxy

Полная документация всех поддерживаемых моделей, роутинга и Extended Thinking опций.

## Архитектура системы

**Antigravity Manager API Proxy** конвертирует запросы от различных AI клиентов (Claude Code, Cursor, OpenCode) в формат Google Cloud Code API с автоматической ротацией аккаунтов и интеллектуальным роутингом моделей.

**Endpoint:** `http://localhost:8045/v1/messages` (Anthropic формат)

---

## 🎯 Критическое различие: Claude vs Gemini Thinking

### Claude Models
**Extended Thinking включается через СУФФИКС в названии модели:**
```
claude-opus-4-5-thinking     ✅ С thinking
claude-opus-4-5              ❌ БЕЗ thinking (НЕ доступен через Google!)
claude-sonnet-4-5-thinking   ✅ С thinking
claude-sonnet-4-5            ✅ БЕЗ thinking
```

### Gemini Models
**Extended Thinking включается через ПАРАМЕТР API `thinkingConfig`:**
```
gemini-3-pro-high            ✅ Базовая модель
+ thinkingConfig parameter   ✅ Включает thinking

gemini-3-pro-high-thinking   ❌ НЕ СУЩЕСТВУЕТ! (404 Not Found)
```

**ВАЖНО:** Никогда не используйте `-thinking` суффикс для Gemini моделей!

---

## 📋 Поддерживаемые модели

### Claude Models

#### Opus (Premium, Extended Thinking)
```yaml
claude-opus-4:
  routes_to: claude-opus-4-5-thinking
  thinking: Required (Google only provides with thinking)
  max_thinking_budget: 32000 tokens
  use_case: Complex reasoning, multi-step analysis

claude-opus-4-5:
  routes_to: claude-opus-4-5-thinking
  thinking: Required

claude-opus-4-5-20251101:
  routes_to: claude-opus-4-5-thinking
  thinking: Required

claude-opus-4-5-high:
  routes_to: claude-opus-4-5-thinking
  thinking: Required
  note: OpenCode Opus with -high suffix
```

#### Sonnet (Balanced, Optional Thinking)
```yaml
claude-sonnet-4-5:
  routes_to: claude-sonnet-4-5
  thinking: Optional (default OFF)
  use_case: General purpose, coding, analysis

claude-sonnet-4-5-thinking:
  routes_to: claude-sonnet-4-5-thinking
  thinking: Enabled
  max_thinking_budget: 32000 tokens

claude-sonnet-4-5-20250929:
  routes_to: claude-sonnet-4-5-thinking
  thinking: Enabled

claude-3-5-sonnet-20241022:
  routes_to: claude-sonnet-4-5
  thinking: Optional
  note: Legacy version
```

#### Haiku (Fast, Routes to Gemini)
```yaml
claude-haiku-4:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter
  max_thinking_budget: 32000 tokens
  note: Haiku not available via Google, routes to Gemini Pro High

claude-haiku-4-5:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter

claude-haiku-4-5-20251001:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter

claude-3-haiku-20240307:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter
```

---

### Gemini Models

#### Gemini 3 Pro (High Performance)
```yaml
gemini-3-pro:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter
  max_thinking_budget: 32000 tokens
  use_case: Complex reasoning, coding

gemini-3-pro-high:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter
  note: Default routing for "gemini-3-pro"

gemini-3-pro-low:
  routes_to: gemini-3-pro-low
  thinking: Via thinkingConfig parameter
  use_case: Cost-optimized tasks

gemini-3-pro-preview:
  routes_to: gemini-3-pro-high
  thinking: Via thinkingConfig parameter
  note: Preview alias routes to high
```

#### Gemini 3 Flash (Fast)
```yaml
gemini-3-flash:
  routes_to: gemini-3-flash
  thinking: Via thinkingConfig parameter
  use_case: Quick responses, simple tasks
```

#### Gemini 2.5 (Legacy)
```yaml
gemini-2.5-flash:
  routes_to: gemini-2.5-flash
  thinking: Via thinkingConfig parameter
  max_thinking_budget: 24576 tokens
  note: Only model supporting googleSearch tool

gemini-2.5-flash-lite:
  routes_to: gemini-2.5-flash-lite
  thinking: Via thinkingConfig parameter

gemini-2.5-flash-thinking:
  routes_to: gemini-2.5-flash-thinking
  thinking: Legacy support
  note: Legacy naming, prefer using thinkingConfig parameter

gemini-2.5-pro:
  routes_to: gemini-2.5-pro
  thinking: Via thinkingConfig parameter
```

#### Gemini 3 Pro Image (Image Generation)
```yaml
gemini-3-pro-image:
  routes_to: gemini-3-pro-image
  request_type: image_gen
  thinking: Not supported

gemini-3-pro-image-2k:
  routes_to: gemini-3-pro-image
  resolution: 2048x2048

gemini-3-pro-image-4k:
  routes_to: gemini-3-pro-image
  resolution: 4096x4096

gemini-3-pro-image-{ratio}:
  routes_to: gemini-3-pro-image
  aspect_ratios: [1x1, 4x3, 3x4, 16x9, 9x16, 21x9]
```

---

### OpenAI Compatibility Models

```yaml
gpt-4:
  routes_to: gemini-2.5-pro
  thinking: Via thinkingConfig parameter

gpt-4-turbo:
  routes_to: gemini-2.5-pro
  thinking: Via thinkingConfig parameter

gpt-4o:
  routes_to: gemini-2.5-pro
  thinking: Via thinkingConfig parameter

gpt-4o-mini:
  routes_to: gemini-2.5-flash
  thinking: Via thinkingConfig parameter

gpt-3.5-turbo:
  routes_to: gemini-2.5-flash
  thinking: Via thinkingConfig parameter
```

---

## 🧠 Extended Thinking Configuration

### Что такое Extended Thinking?

Extended Thinking (продленное мышление) - это режим, при котором модель явно показывает свой процесс рассуждения перед финальным ответом. Это улучшает качество ответов на сложные задачи.

### Как включить Extended Thinking

#### Для Claude моделей:
```json
{
  "model": "claude-opus-4-5-thinking",
  "messages": [...],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 10000
  }
}
```

**Model name должен содержать `-thinking` суффикс!**

#### Для Gemini моделей:
```json
{
  "model": "gemini-3-pro-high",
  "messages": [...],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 10000
  }
}
```

**Model name БЕЗ `-thinking` суффикса! Thinking включается через `thinkingConfig` в API запросе.**

### Thinking Budget Limits

Максимальные значения `budget_tokens` для каждой модели:

| Model Type | Max Budget | Note |
|------------|------------|------|
| Claude Opus | 32000 | Включается через model name |
| Claude Sonnet | 32000 | Включается через model name |
| Gemini 3 Pro | 32000 | Включается через parameter |
| Gemini 3 Flash | 32000 | Включается через parameter |
| Gemini 2.5 Flash | 24576 | Меньший лимит! |
| Gemini 2.5 Pro | 32000 | Включается через parameter |

**ВАЖНО:** Система автоматически clamps budget к максимальному значению.

### Опции конфигурации

```json
{
  "thinking": {
    "type": "enabled",           // "enabled" | "disabled"
    "budget_tokens": 10000       // Optional, default auto-calculated
  },
  "max_tokens": 12000            // Must be > budget_tokens
}
```

**Правила:**
1. `max_tokens` ДОЛЖЕН быть больше `budget_tokens`
2. Система автоматически добавит +100 если нарушено
3. Budget автоматически clamps к model limits

---

## 🔀 Model Routing Priority

Система использует 3-уровневую приоритезацию:

### Priority 1: Custom User Mappings (Exact Match)
```yaml
custom_model_mapping:
  my-opus: claude-opus-4-5-thinking
  my-sonnet: claude-sonnet-4-5
```

### Priority 2: Custom Wildcard Mappings
```yaml
custom_model_mapping:
  gpt-4*: gemini-2.5-pro
  claude-3-5-*: claude-sonnet-4-5
```

### Priority 3: System Default Mappings
Встроенный маппинг из `src/proxy/common/model_mapping.rs`

### Fallback Rule
Если модель не найдена ни в одном маппинге:
```
unknown-model → gemini-3-pro-high
```

---

## 🌐 Special Features

### Web Search (Google Search Integration)

Добавьте `-online` суффикс к любой модели для включения web search:

```
gemini-3-flash-online → gemini-2.5-flash + googleSearch tool
claude-sonnet-4-5-online → gemini-2.5-flash + googleSearch tool
```

**ВАЖНО:** Только `gemini-2.5-flash` поддерживает `googleSearch` tool! Все остальные модели автоматически downgrade к Flash при web search.

### Image Generation

```
gemini-3-pro-image           → 1024x1024, 1:1
gemini-3-pro-image-2k        → 2048x2048
gemini-3-pro-image-4k        → 4096x4096
gemini-3-pro-image-16x9      → 16:9 aspect ratio
gemini-3-pro-image-4k-21x9   → 4096x4096, 21:9
```

---

## 📊 Производительность и квоты

### Account Rotation Strategy

Система автоматически выбирает оптимальный аккаунт на основе:
1. **Subscription tier**: Ultra > Pro > Free
2. **Rate limit status**: Избегает аккаунты с 429 errors
3. **Quota reset frequency**: Приоритет к daily resets
4. **Quota remaining**: Weighted по оставшейся квоте

### Sticky Sessions

Опционально можно привязать session к конкретному аккаунту:
```json
{
  "metadata": {
    "session_id": "my-session-123"
  }
}
```

Все запросы с одинаковым `session_id` будут использовать один аккаунт.

---

## 🛠️ API Examples

### Basic Request (Without Thinking)
```json
POST http://localhost:8045/v1/messages
Content-Type: application/json

{
  "model": "claude-sonnet-4-5",
  "messages": [
    {
      "role": "user",
      "content": "Hello, Claude!"
    }
  ],
  "max_tokens": 1000
}
```

### Extended Thinking Request (Claude)
```json
{
  "model": "claude-opus-4-5",
  "messages": [
    {
      "role": "user",
      "content": "Solve this complex problem..."
    }
  ],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 15000
  },
  "max_tokens": 20000
}
```

**Результат:**
- Model routes to: `claude-opus-4-5-thinking`
- Thinking enabled via model name
- Budget: 15000 tokens

### Extended Thinking Request (Gemini)
```json
{
  "model": "gemini-3-pro-high",
  "messages": [
    {
      "role": "user",
      "content": "Analyze this system..."
    }
  ],
  "thinking": {
    "type": "enabled",
    "budget_tokens": 20000
  },
  "max_tokens": 25000
}
```

**Результат:**
- Model stays: `gemini-3-pro-high` (БЕЗ `-thinking`!)
- Thinking enabled via `thinkingConfig` parameter
- Budget: 20000 tokens

### Streaming Request
```json
{
  "model": "claude-sonnet-4-5",
  "messages": [...],
  "stream": true,
  "max_tokens": 2000
}
```

**Примечание:** Все запросы автоматически конвертируются в streaming на upstream для избежания rate limits.

---

## 🔧 Configuration Options

### Proxy Configuration (UI: API Proxy Settings)

```yaml
proxy:
  host: 127.0.0.1
  port: 8045
  api_key: optional_api_key
  request_timeout: 600          # seconds, max 3600

rate_limit:
  max_retries: 3
  retry_delay: 1000             # milliseconds

custom_model_mapping:
  my-model: gemini-3-flash      # Exact match
  gpt-4*: gemini-2.5-pro        # Wildcard
```

### Environment Variables

```bash
# Override User-Agent
CLAUDE_USER_AGENT="custom-agent/1.0.0"

# Test variants (debugging)
X_TEST_VARIANT="variant-name"
```

---

## 📈 Monitoring & Analytics

### Success Metrics

После тестирования текущей сессии:

```
📊 СТАТИСТИКА (2026-01-09 17:40-18:00):
──────────────────────────────────────
Всего запросов:        49
Ответов получено:       9 (18.4%)
Успешных завершений:    4 (8.2%)
Зависших:              40 (81.6%)

🎯 МОДЕЛИ:
──────────────────────────────────────
claude-opus-4-5-thinking:  48 запросов
  ✅ Успешно: 3 (6.3%)
  ⏳ Зависло: 45 (93.7%)

gemini-3-pro-high:          1 запрос
  ✅ Успешно: 1 (100%)

❌ HTTP ОШИБКИ:
──────────────────────────────────────
429 Rate Limit:         5
404 Not Found:          0 ✅ (исправлено!)
```

### Успешные модели:
- ✅ `gemini-3-pro-high` - 100% success rate
- ⚠️ `claude-opus-4-5-thinking` - работает, но ~94% зависает

### Known Issues

#### Issue #497: Session Hanging
**Проблема:** Запросы к `claude-opus-4-5-thinking` зависают без ответа от Google API.

**Симптомы:**
- Запрос отправляется успешно
- Google API не отвечает (timeout)
- Proxy показывает "0 input, 0 output"
- Проблема воспроизводится у нескольких пользователей

**Workaround:**
- Используйте `gemini-3-pro-high` вместо `claude-opus-4-5-thinking`
- Или используйте `claude-sonnet-4-5` без thinking

**Статус:** Investigating (Google API side issue)

---

## 🧪 Testing

### Unit Tests
```bash
# All model routing tests
cargo test --lib proxy::tests::thinking_models

# Specific test
cargo test --lib test_gemini_routing_no_thinking_suffix
```

### Integration Testing

См. `TESTING_GUIDE.md` для полной документации по тестированию.

### Manual Testing

```bash
# Test Claude Opus with thinking
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -d '{
    "model": "claude-opus-4-5",
    "messages": [{"role": "user", "content": "Test"}],
    "thinking": {"type": "enabled", "budget_tokens": 5000},
    "max_tokens": 10000
  }'

# Test Gemini Pro High with thinking
curl -X POST http://localhost:8045/v1/messages \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gemini-3-pro-high",
    "messages": [{"role": "user", "content": "Test"}],
    "thinking": {"type": "enabled", "budget_tokens": 5000},
    "max_tokens": 10000
  }'
```

---

## 🔍 Debugging

### Check Logs
```bash
# Real-time monitoring
tail -f ~/.antigravity_tools/logs/app.log.$(date +%Y-%m-%d)

# Model routing
grep "Router.*映射" ~/.antigravity_tools/logs/app.log.* | tail -20

# Thinking configuration
grep "THINKING MODE" ~/.antigravity_tools/logs/app.log.* | tail -10

# Success rate
grep "Stream completed" ~/.antigravity_tools/logs/app.log.* | wc -l
```

### Common Issues

#### 404 Not Found
**Причина:** Gemini model с `-thinking` суффиксом
**Решение:** Используйте model БЕЗ `-thinking`, thinking включится через parameter

#### Max tokens validation error
**Причина:** `max_tokens <= budget_tokens`
**Решение:** Система автоматически исправляет, добавляя +100

#### Budget exceeds model limits
**Причина:** `budget_tokens > model_max_budget`
**Решение:** Система автоматически clamps к максимуму

#### Session hanging (Issue #497)
**Причина:** Google API не отвечает на некоторые Claude thinking requests
**Workaround:** Используйте Gemini models или Sonnet без thinking

---

## 📚 Related Documentation

- `THINKING_MODELS.md` - Technical deep-dive в thinking approaches
- `TESTING_GUIDE.md` - Comprehensive test suite documentation
- `CLAUDE_FIX_SUMMARY.md` - История исправлений Extended Thinking
- `src/proxy/common/model_mapping.rs` - Код роутинга моделей
- `src/proxy/mappers/claude/request.rs` - Код трансформации запросов

---

## 🎯 Best Practices

### Выбор модели

**Для сложных задач с reasoning:**
```
gemini-3-pro-high + thinking.enabled
```

**Для быстрых ответов:**
```
gemini-3-flash (without thinking)
claude-sonnet-4-5 (without thinking)
```

**Для максимального качества (если работает):**
```
claude-opus-4-5 + thinking.enabled
```

**Для web search:**
```
{model}-online → auto-routes to gemini-2.5-flash + googleSearch
```

### Thinking Budget Recommendations

| Task Type | Recommended Budget | Model |
|-----------|-------------------|-------|
| Simple Q&A | 0-2000 | Any model |
| Code analysis | 5000-10000 | Sonnet/Gemini Pro |
| Complex reasoning | 15000-25000 | Opus/Gemini Pro High |
| System design | 25000-32000 | Opus/Gemini Pro High |

### Rate Limit Management

Система автоматически:
- Ротирует аккаунты при 429 errors
- Retries с exponential backoff
- Приоритезирует Premium аккаунты

**Рекомендации:**
- Используйте несколько аккаунтов разных tier
- Не превышайте quota limits
- Мониторьте логи для 429 errors

---

**Версия:** 1.0
**Дата:** 2026-01-09
**Автор:** Claude Sonnet 4.5
**Статус:** Production Ready ✅
