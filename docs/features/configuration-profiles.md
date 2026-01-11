# Configuration Profiles

## Overview

Configuration Profiles provide **one-click preset configurations** for common AI interaction scenarios. Each profile is a pre-tuned combination of model selection, thinking mode settings, token limits, and temperature that optimizes the AI's behavior for specific use cases.

**Purpose**: Eliminate manual configuration for common scenarios while maintaining full customization flexibility.

**Key Benefits**:
- **Instant Setup**: Apply production-ready configurations with one click
- **Best Practice Defaults**: Each profile embodies proven parameter combinations
- **Use Case Optimization**: Profiles tailored for chat, code, analysis, and creative tasks
- **Learning Tool**: See how different parameters affect AI behavior
- **Consistency**: Reproducible configurations across sessions

**Integration with Other Features**:
- **Thinking Activation**: 4 profiles with thinking mode pre-configured
- **OpenAI Auto-Injection**: Works seamlessly when OpenAI protocol used
- **Permissive Mode**: First-time requests succeed without configuration friction
- **Auto-Correction**: Budget violations automatically fixed when applying profiles

---

## Profile Catalog

### Base Profiles (Thinking Disabled - 4 Profiles)

Standard configurations without extended reasoning capabilities, optimized for speed and efficiency.

| Profile | Model | Max Tokens | Temperature | Use Cases | Speed | Description |
|---------|-------|------------|-------------|-----------|-------|-------------|
| **Fast General** | gemini-3-pro-high | 8,192 | 0.7 | Chat, General, Quick | ⚡ Fast | Quick responses for everyday conversations and simple tasks |
| **Balanced** | gemini-3-pro-high | 16,384 | 0.7 | Coding, Analysis, Balanced | ⚖️ Medium | Balance between response quality and generation speed |
| **High Quality** | gemini-3-pro-high | 32,000 | 0.5 | Complex, Detailed, Quality | 🎯 Slower | Maximum quality for detailed explanations and complex tasks |
| **Creative** | gemini-3-pro-high | 16,384 | 1.0 | Creative, Brainstorming, Writing | 🎨 Medium | High temperature for diverse and imaginative responses |

### Thinking Profiles (Thinking Enabled - 4 Profiles)

Advanced configurations with extended reasoning capabilities for complex problem-solving.

| Profile | Model | Max Tokens | Thinking Budget | Temperature | Use Cases | Description |
|---------|-------|------------|-----------------|-------------|-----------|-------------|
| **Thinking Fast** | gemini-3-pro-high | 16,384 | 4,096 | 0.7 | Reasoning, Problem-Solving, Fast | Quick reasoning with compact thinking budget |
| **Thinking Balanced** | gemini-3-pro-high | 24,576 | 8,192 | 0.7 | Analysis, Research, Balanced | Moderate reasoning depth for most analytical tasks |
| **Thinking Deep** | gemini-3-pro-high | 32,768 | 16,384 | 0.5 | Complex, Deep Analysis, Research | Deep reasoning for sophisticated analysis |
| **Thinking Maximum** | gemini-3-pro-high | 65,536 | 32,768 | 0.5 | Expert, Critical, Maximum | Maximum reasoning capability for critical decisions |

---

## Detailed Profile Specifications

### Fast General

**Profile ID**: `fast-general`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": { "enabled": false },
  "maxTokens": 8192,
  "temperature": 0.7
}
```

**Recommended For**:
- Casual chat and everyday questions
- Quick information lookups
- Simple task instructions
- Real-time conversations

**Performance Characteristics**:
- **Response Time**: ~2-5 seconds
- **Token Efficiency**: High (minimal overhead)
- **Cost**: Low (fewer tokens consumed)

**When NOT to Use**: Complex reasoning, detailed analysis, long-form content

---

### Balanced

**Profile ID**: `balanced`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": { "enabled": false },
  "maxTokens": 16384,
  "temperature": 0.7
}
```

**Recommended For**:
- Code generation and debugging
- Technical documentation
- Moderate complexity analysis
- Tutorial creation

**Performance Characteristics**:
- **Response Time**: ~5-10 seconds
- **Token Efficiency**: Balanced
- **Cost**: Moderate

**Sweet Spot**: General-purpose programming and technical tasks

---

### High Quality

**Profile ID**: `quality`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": { "enabled": false },
  "maxTokens": 32000,
  "temperature": 0.5
}
```

**Recommended For**:
- Comprehensive documentation
- Detailed technical explanations
- Architecture design documents
- Production-ready code review

**Performance Characteristics**:
- **Response Time**: ~10-20 seconds
- **Token Efficiency**: Lower (prioritizes quality)
- **Cost**: Higher

**Temperature Note**: Lower temperature (0.5) reduces randomness for more focused, deterministic output

---

### Creative

**Profile ID**: `creative`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": { "enabled": false },
  "maxTokens": 16384,
  "temperature": 1.0
}
```

**Recommended For**:
- Creative writing and storytelling
- Brainstorming sessions
- Marketing copy generation
- Diverse idea generation

**Performance Characteristics**:
- **Response Time**: ~5-10 seconds
- **Diversity**: High (temperature 1.0 maximizes variation)
- **Predictability**: Low (intentionally less deterministic)

**Temperature Impact**: Maximum temperature (1.0) enables most creative and varied responses

---

### Thinking Fast

**Profile ID**: `thinking-fast`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": {
    "enabled": true,
    "budgetTokens": 4096
  },
  "maxTokens": 16384,
  "temperature": 0.7
}
```

**Recommended For**:
- Quick problem-solving tasks
- Simple logical reasoning
- Code optimization suggestions
- Fast analytical questions

**Thinking Allocation**:
- **Thinking Budget**: 4,096 tokens (~25% of total)
- **Response Space**: ~12,288 tokens (~75% of total)
- **Reasoning Depth**: Light reasoning for straightforward problems

**Performance Characteristics**:
- **Response Time**: ~8-15 seconds
- **Reasoning Capability**: Basic to moderate
- **Cost**: Moderate (thinking overhead)

**Budget Constraint**: Automatically validated (16,384 > 4,096 + 100 ✅)

---

### Thinking Balanced

**Profile ID**: `thinking-balanced`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": {
    "enabled": true,
    "budgetTokens": 8192
  },
  "maxTokens": 24576,
  "temperature": 0.7
}
```

**Recommended For**:
- Moderate complexity analysis
- Research and information synthesis
- Multi-step problem solving
- Code architecture decisions

**Thinking Allocation**:
- **Thinking Budget**: 8,192 tokens (~33% of total)
- **Response Space**: ~16,384 tokens (~67% of total)
- **Reasoning Depth**: Moderate reasoning for typical analytical tasks

**Performance Characteristics**:
- **Response Time**: ~15-25 seconds
- **Reasoning Capability**: Moderate to advanced
- **Cost**: Higher (extended reasoning)

**Sweet Spot**: Most analytical and research tasks

---

### Thinking Deep

**Profile ID**: `thinking-deep`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": {
    "enabled": true,
    "budgetTokens": 16384
  },
  "maxTokens": 32768,
  "temperature": 0.5
}
```

**Recommended For**:
- Complex architectural decisions
- Advanced algorithm design
- Sophisticated debugging
- Scientific reasoning

**Thinking Allocation**:
- **Thinking Budget**: 16,384 tokens (~50% of total)
- **Response Space**: ~16,384 tokens (~50% of total)
- **Reasoning Depth**: Deep reasoning for complex problems

**Performance Characteristics**:
- **Response Time**: ~25-40 seconds
- **Reasoning Capability**: Advanced to expert
- **Cost**: High (extensive reasoning)

**Temperature Note**: Lower temperature (0.5) for more focused, rigorous reasoning

---

### Thinking Maximum

**Profile ID**: `thinking-max`

**Configuration**:
```json
{
  "model": "gemini-3-pro-high",
  "thinking": {
    "enabled": true,
    "budgetTokens": 32768
  },
  "maxTokens": 65536,
  "temperature": 0.5
}
```

**Recommended For**:
- Critical system design decisions
- Expert-level problem solving
- Comprehensive security analysis
- Maximum reasoning capacity tasks

**Thinking Allocation**:
- **Thinking Budget**: 32,768 tokens (50% of total, at model limit)
- **Response Space**: ~32,768 tokens (50% of total)
- **Reasoning Depth**: Maximum reasoning capability

**Performance Characteristics**:
- **Response Time**: ~40-60 seconds
- **Reasoning Capability**: Expert-level
- **Cost**: Highest (maximum reasoning allocation)

**Model Limit**: 32,768 tokens is Gemini 3 Pro High's maximum thinking budget

**Budget Constraint**: Automatically validated (65,536 > 32,768 + 100 ✅)

---

## UI Features

### Profile Card Design

**Visual Components**:
- **Icon + Color**: Each profile has unique icon and color scheme
- **Thinking Badge**: Purple badge indicates thinking-enabled profiles
- **Token Display**: Shows maxTokens and thinkingBudget in readable format
- **Use Case Tags**: Up to 3 primary use cases displayed
- **Active Indicator**: CheckCircle icon for currently active profile
- **Stats Grid**: Key metrics (max tokens, thinking budget, response time)

**Interactive Features**:
- **Hover Effects**: Scale animation (1.02x) and enhanced shadow
- **Click to Apply**: Entire card clickable for profile selection
- **Apply Button**: Explicit action button with state feedback
- **Active State**: Visual distinction for currently applied profile

### Category Filters

**Three Filter Options**:
1. **All** (8 profiles) - Shows all available profiles
2. **Base** (4 profiles) - Standard profiles without thinking mode
3. **Thinking** (4 profiles) - Profiles with extended reasoning enabled

**Filter Behavior**:
- Toggle between categories with single click
- Profile count displayed in filter buttons
- Responsive grid layout adjusts to filtered results

### Responsive Design

**Grid Layout**:
- **Mobile** (< 768px): 1 column
- **Tablet** (768px - 1024px): 2 columns
- **Desktop** (> 1024px): 4 columns

**Card Sizing**: Auto-height cards with consistent padding and spacing

---

## How to Use

### Step 1: Access Configuration Profiles

**Location**: API Proxy page → Configuration Profiles section

**Visual Indicators**:
- Section header with Sparkles icon and title
- 8 profile cards in responsive grid
- Category filter buttons at top-right

### Step 2: Filter Profiles (Optional)

**Filter by Category**:
```
All (8) | Base (4) | Thinking (4)
```

**Filter by Use Case**:
- Review use case tags on each card
- Match your task to profile use cases
- Example: "coding" → Balanced or Thinking Balanced

### Step 3: Review Profile Details

**Check Each Profile Card**:
- **Name & Description**: Understand profile purpose
- **Badges**: Thinking enabled? Token limits?
- **Use Cases**: Matches your task?
- **Stats**: Appropriate budget and speed?

### Step 4: Select and Apply Profile

**Application Methods**:
1. **Click entire card** → Auto-applies profile
2. **Click "Apply Profile" button** → Explicit action

**Visual Feedback**:
- Active profile shows CheckCircle indicator
- Border color changes to blue with ring
- Apply button changes to "Active" with success styling

### Step 5: Verify Configuration

**Check Configuration Panel**:
- Model selection updated
- Thinking mode toggle matches profile
- Budget tokens set correctly
- Max tokens and temperature applied

**Example Verification** (Thinking Balanced):
```
✅ Model: gemini-3-pro-high
✅ Thinking: Enabled
✅ Budget Tokens: 8192
✅ Max Tokens: 24576
✅ Temperature: 0.7
```

### Step 6: Customize (Optional)

**After Applying Profile**:
- All settings remain editable
- Adjust thinking budget if needed
- Fine-tune temperature
- Modify max tokens

**Profile as Starting Point**: Use profiles as baseline, then customize for specific needs

---

## Integration with Other Features

### OpenAI Protocol Auto-Injection

**Scenario**: Using OpenAI protocol with Thinking profiles

**How It Works**:
1. Select Thinking profile (e.g., "Thinking Balanced")
2. Profile sets `thinking.enabled = true` and `budgetTokens = 8192`
3. OpenAI client makes request (no native thinking support)
4. Proxy auto-injects `thinkingConfig` during protocol conversion
5. Gemini receives proper thinking configuration

**Benefit**: Thinking profiles work seamlessly with OpenAI protocol clients

**See**: [OpenAI Protocol Auto-Injection](#openai-protocol-auto-injection) in `thinking-activation.md`

---

### First-Time Permissive Mode

**Scenario**: Applying Thinking profile on first request

**How It Works**:
1. Select Thinking profile (e.g., "Thinking Fast")
2. Make first request in conversation
3. No thinking history exists yet
4. Permissive mode activates automatically
5. Signature validation deferred to upstream
6. Thinking mode activates successfully

**Benefit**: Zero-friction first-time experience with thinking profiles

**Log Output**:
```
[Thinking-Mode] First thinking request detected. Using permissive mode -
signature validation will be handled by upstream API.
```

**See**: [First-Time Permissive Mode](#first-time-permissive-mode) in `thinking-activation.md`

---

### maxOutputTokens Auto-Correction

**Scenario**: Profile with budget constraint violation

**Example**: Custom profile modification causes violation
```json
{
  "maxTokens": 8000,        // User modified this
  "thinking": {
    "budgetTokens": 8192    // From Thinking Balanced profile
  }
}
```

**Problem**: `8000 ≤ 8192` violates constraint ❌

**Auto-Correction**:
1. System detects violation (8000 ≤ 8192)
2. Calculates adjusted value (8192 + 100 = 8292)
3. Auto-fixes maxOutputTokens → 8292
4. Logs warning with guidance

**Benefit**: Profile configurations always valid even after manual adjustment

**All Preset Profiles Are Pre-Validated**: Built-in profiles never trigger auto-correction

**See**: [maxOutputTokens Auto-Correction](#maxoutputtokens-auto-correction) in `thinking-activation.md`

---

## Customization Best Practices

### When to Use Presets vs. Custom

**Use Preset Profiles When**:
- Starting new conversation or project
- Common use case matches available profiles
- Unsure about optimal parameters
- Want proven, tested configurations

**Create Custom Configuration When**:
- Unique requirements not covered by presets
- Need fine-tuned parameter combinations
- Specific model preferences
- Advanced optimization for particular workflow

### Customizing After Profile Application

**Safe Customization Pattern**:
1. **Apply closest matching profile** (baseline)
2. **Test with profile defaults** (verify behavior)
3. **Adjust one parameter at a time** (isolate changes)
4. **Validate constraints** (ensure thinking budget < max tokens)
5. **Document your changes** (note why you customized)

**Example Custom Workflow**:
```
1. Apply "Thinking Deep" profile
2. Test with complex code analysis task
3. Increase thinking budget: 16384 → 20000
4. Increase max tokens: 32768 → 40000
5. Verify: 40000 > 20000 + 100 ✅
6. Document: "Extended budget for large codebase analysis"
```

### Temperature Tuning Guidelines

**Temperature Effects**:
- **0.0 - 0.3**: Very focused, deterministic, repetitive
- **0.4 - 0.7**: Balanced, predictable with some variety
- **0.8 - 1.0**: Creative, diverse, less predictable
- **> 1.0**: Highly experimental, chaotic (not recommended)

**Profile Defaults**:
- High Quality/Thinking Deep/Thinking Maximum: **0.5** (focused reasoning)
- Fast General/Balanced/Thinking Fast/Thinking Balanced: **0.7** (balanced)
- Creative: **1.0** (maximum diversity)

**Customization Examples**:
- Code generation → Lower to 0.3-0.5 (more deterministic)
- Creative writing → Raise to 0.9-1.0 (more diverse)
- Technical docs → Keep at 0.5 (balanced accuracy)

### Token Budget Optimization

**Thinking Budget Guidelines**:
- **Simple tasks**: 2K-4K tokens (quick reasoning)
- **Moderate complexity**: 8K-12K tokens (standard analysis)
- **Complex tasks**: 16K-24K tokens (deep reasoning)
- **Maximum capability**: 32K tokens (expert-level, Gemini 3 Pro limit)

**Response Space Guidelines**:
- Ensure at least **100 tokens** for final response (absolute minimum)
- Recommend **2:1 ratio** (maxTokens : thinkingBudget) for quality output
- Example: 8K thinking budget → 16K+ max tokens

**Profile Ratios**:
| Profile | Thinking Budget | Max Tokens | Ratio | Quality |
|---------|----------------|------------|-------|---------|
| Thinking Fast | 4K | 16K | 4:1 | Good |
| Thinking Balanced | 8K | 24K | 3:1 | Better |
| Thinking Deep | 16K | 32K | 2:1 | Best |
| Thinking Maximum | 32K | 65K | 2:1 | Maximum |

---

## Technical Implementation

### Component Location

**File**: `src/components/proxy/ConfigurationProfiles.tsx`

**Lines**: 377 total

**Dependencies**:
- React hooks: `useState`
- i18next: `useTranslation`
- Lucide icons: 8 icons imported

### Profile Data Structure

```typescript
interface Profile {
  id: string;                    // Unique identifier
  name: string;                  // Display name
  description: string;           // Short description
  model: string;                 // Model identifier
  thinking: ThinkingConfig;      // Thinking configuration
  maxTokens: number;             // Max output tokens
  temperature: number;           // Temperature (0.0-1.0)
  useCase: string[];             // Use case tags
  icon: React.ReactNode;         // React icon component
  color: string;                 // Tailwind color class
}

interface ThinkingConfig {
  enabled: boolean;              // Thinking mode toggle
  budgetTokens?: number;         // Optional budget (if enabled)
}
```

### Integration with ApiProxy Page

**Props Interface**:
```typescript
interface ConfigurationProfilesProps {
  currentModel?: string;
  currentThinking?: ThinkingConfig;
  currentMaxTokens?: number;
  currentTemperature?: number;
  onProfileSelect?: (profile: Profile) => void;
}
```

**Callback Flow**:
1. User clicks profile card
2. `onProfileSelect(profile)` callback triggered
3. Parent component (ApiProxy) receives profile data
4. Configuration state updated via store
5. UI reflects new configuration

### Active Profile Detection

```typescript
const isProfileActive = (profile: Profile): boolean => {
  if (!currentModel) return false;

  return (
    profile.model === currentModel &&
    profile.thinking.enabled === currentThinking?.enabled &&
    profile.maxTokens === currentMaxTokens &&
    profile.temperature === currentTemperature &&
    (!profile.thinking.enabled ||
     profile.thinking.budgetTokens === currentThinking?.budgetTokens)
  );
};
```

**Matching Logic**: Exact match on all 5 parameters required for "active" state

### i18n Translation Keys

**Profile Names & Descriptions**:
```
profiles.fast-general.name
profiles.fast-general.description
profiles.balanced.name
profiles.balanced.description
... (16 keys total for 8 profiles)
```

**UI Labels**:
```
profiles.title
profiles.subtitle
profiles.thinking_enabled
profiles.max_tokens
profiles.thinking_budget
profiles.response_time
profiles.active
profiles.apply_profile
```

**Filter Labels**:
```
profiles.filter.all
profiles.filter.base
profiles.filter.thinking
```

**Use Case Tags**:
```
profiles.use_case.chat
profiles.use_case.general
profiles.use_case.coding
profiles.use_case.analysis
... (20+ use case keys)
```

---

## Code References

### Component Structure

| Component | Lines | Purpose |
|-----------|-------|---------|
| `ProfileCard` | 135-268 | Individual profile card with interaction |
| `ConfigurationProfiles` | 270-377 | Main container with filtering and grid |
| `PROFILES` constant | 33-133 | Profile definitions array |

### Key Functions

**Profile Filtering** (Lines 307-312):
```typescript
const filteredProfiles = PROFILES.filter(profile => {
  if (selectedCategory === 'all') return true;
  if (selectedCategory === 'base') return !profile.thinking.enabled;
  if (selectedCategory === 'thinking') return profile.thinking.enabled;
  return true;
});
```

**Profile Selection Handler** (Lines 301-305):
```typescript
const handleSelectProfile = (profile: Profile) => {
  if (onProfileSelect) {
    onProfileSelect(profile);
  }
};
```

**Active Profile Check** (Lines 288-299):
- Exact match on all configuration parameters
- Returns boolean for visual state

### Styling Classes

**Card States**:
- Default: `border-gray-200 dark:border-gray-700`
- Hover: `hover:border-blue-300 hover:scale-[1.02]`
- Active: `border-blue-500 ring-2 ring-blue-200`

**Icon Colors** (Lines 45, 57, 69, 81, 95, 107, 119, 131):
- Fast General: `bg-yellow-500`
- Balanced: `bg-blue-500`
- High Quality: `bg-green-500`
- Creative: `bg-purple-500`
- Thinking Fast: `bg-indigo-500`
- Thinking Balanced: `bg-blue-600`
- Thinking Deep: `bg-purple-600`
- Thinking Maximum: `bg-pink-600`

---

## Related Documentation

- **Thinking Activation Architecture**: `/docs/features/thinking-activation.md`
  - OpenAI Protocol Auto-Injection (Lines 621-1063)
  - First-Time Permissive Mode (Lines 1066-1548)
  - maxOutputTokens Auto-Correction (Lines 1551-1936)
- **Epic-005 Overview**: `/docs/epics/epic-005-gemini-3-pro-high-compliance.md`
- **Component Implementation**: `src/components/proxy/ConfigurationProfiles.tsx`
- **ApiProxy Integration**: `src/pages/ApiProxy.tsx`

---

## Version History

| Version | Date | Changes | Story |
|---------|------|---------|-------|
| 1.0.0 | 2026-01-11 | Initial documentation | Story-005-08 |
