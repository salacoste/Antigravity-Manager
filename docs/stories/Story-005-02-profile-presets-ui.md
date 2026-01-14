# Story-005-02: Profile Presets UI Implementation

**Epic**: Epic-005-Gemini-3-Pro-High-Compliance
**Priority**: MEDIUM
**Effort**: 4 hours
**Type**: CODE (Frontend)
**Status**: PENDING
**Sequential Order**: 2/8 (Must complete Story-005-01 first)

---

## 📋 User Story

**As a** power user configuring Antigravity Manager API requests
**I want** profile presets dropdown with pre-configured max_tokens and thinking_budget values
**So that** I can quickly select optimal configurations without manual calculation

---

## 🎯 Context and Background

### Current State

Users must manually configure:
- `max_tokens`: Output token limit (no guidance)
- `thinking_budget`: Extended thinking token budget (for thinking models)

**Challenges**:
- No guidance on optimal values for different use cases
- Users may exceed model limits (32000 for Gemini 3 Pro)
- No awareness of recommended configurations from documentation

### Gap Analysis

**Reference**: `gemini-3-pro-high-COMPARISON.md:566-624`

```yaml
status: ⚠️ DOCUMENTATION-ONLY

what_missing:
  - ❌ No profile constants in code
  - ❌ No profile selection logic
  - ❌ No profile presets UI

impact: "LOW - Nice-to-have UX enhancement"

current_behavior:
  - Users configure max_tokens and thinking_budget manually
  - No preset guidance
  - Risk of exceeding model limits

documented_profiles:
  base_model:
    - PRODUCTION (8192 tokens, balanced)
    - LOW_LATENCY (2048 tokens, fast)
    - HIGH_QUALITY (16384 tokens, maximum quality)
    - TOOL_OPTIMIZED (4096 tokens, function calling)

  thinking_variant:
    - BALANCED_THINKING (8000 budget, 16000 max)
    - DEEP_THINKING (32000 budget, 40000 max)
    - EFFICIENT_THINKING (2000 budget, 6000 max)
    - COMPREHENSIVE_THINKING (20000 budget, 30000 max)
```

### Why This Matters

**UX Parity with Antigravity v1.13.3**:
- Documented workflows reference these profiles
- Users expect guidance for optimal configurations
- Professional tools provide preset templates

**Risk Mitigation**:
- Prevents users from exceeding 32000 token budget limit
- Reduces 400 errors from invalid configurations
- Improves API success rate

---

## 🔨 Implementation Specification

### Component Architecture

**New Component**: `src/components/proxy/ConfigurationProfiles.tsx`

```typescript
interface ProfilePreset {
  id: string;
  name: string;
  category: 'base' | 'thinking';
  max_tokens: number;
  thinking_budget?: number;
  description: string;
}

interface ConfigurationProfilesProps {
  onProfileSelect: (profile: ProfilePreset) => void;
  selectedProfileId?: string;
  className?: string;
}
```

### Profile Definitions

**Base Model Profiles** (4 presets):

```typescript
const BASE_PROFILES: ProfilePreset[] = [
  {
    id: 'production',
    name: 'PRODUCTION',
    category: 'base',
    max_tokens: 8192,
    description: 'Balanced configuration for production workloads'
  },
  {
    id: 'low_latency',
    name: 'LOW_LATENCY',
    category: 'base',
    max_tokens: 2048,
    description: 'Fast responses with minimal token usage'
  },
  {
    id: 'high_quality',
    name: 'HIGH_QUALITY',
    category: 'base',
    max_tokens: 16384,
    description: 'Maximum quality for comprehensive responses'
  },
  {
    id: 'tool_optimized',
    name: 'TOOL_OPTIMIZED',
    category: 'base',
    max_tokens: 4096,
    description: 'Optimized for function calling and tool use'
  }
];
```

**Thinking Variant Profiles** (4 presets):

```typescript
const THINKING_PROFILES: ProfilePreset[] = [
  {
    id: 'balanced_thinking',
    name: 'BALANCED_THINKING',
    category: 'thinking',
    max_tokens: 16000,
    thinking_budget: 8000,
    description: 'Balanced extended thinking for complex reasoning'
  },
  {
    id: 'deep_thinking',
    name: 'DEEP_THINKING',
    category: 'thinking',
    max_tokens: 40000,  // Note: Exceeds 32000 limit, validation needed
    thinking_budget: 32000,
    description: 'Maximum thinking capacity for difficult problems'
  },
  {
    id: 'efficient_thinking',
    name: 'EFFICIENT_THINKING',
    category: 'thinking',
    max_tokens: 6000,
    thinking_budget: 2000,
    description: 'Quick reasoning with minimal thinking overhead'
  },
  {
    id: 'comprehensive_thinking',
    name: 'COMPREHENSIVE_THINKING',
    category: 'thinking',
    max_tokens: 30000,
    thinking_budget: 20000,
    description: 'Thorough analysis with extended reasoning time'
  }
];
```

### UI Component Structure

```tsx
import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Sparkles, Zap, BrainCircuit, Layers } from 'lucide-react';
import { cn } from '../../utils/cn';

export function ConfigurationProfiles({
  onProfileSelect,
  selectedProfileId,
  className
}: ConfigurationProfilesProps) {
  const { t } = useTranslation();
  const [activeCategory, setActiveCategory] = useState<'base' | 'thinking'>('base');

  // Load last selected profile from localStorage
  useEffect(() => {
    const savedProfile = localStorage.getItem('last_selected_profile');
    if (savedProfile) {
      const profile = ALL_PROFILES.find(p => p.id === savedProfile);
      if (profile) {
        onProfileSelect(profile);
      }
    }
  }, []);

  const handleProfileSelect = (profile: ProfilePreset) => {
    onProfileSelect(profile);
    // Persist selection to localStorage
    localStorage.setItem('last_selected_profile', profile.id);
  };

  return (
    <div className={cn('bg-white dark:bg-base-100 rounded-xl p-4', className)}>
      {/* Category Tabs */}
      <div className="flex gap-2 mb-4">
        <button
          onClick={() => setActiveCategory('base')}
          className={cn(
            'flex-1 py-2 px-4 rounded-lg font-medium text-sm transition-colors',
            activeCategory === 'base'
              ? 'bg-blue-500 text-white'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300'
          )}
        >
          <Layers className="inline-block w-4 h-4 mr-2" />
          {t('profiles.base_models')}
        </button>
        <button
          onClick={() => setActiveCategory('thinking')}
          className={cn(
            'flex-1 py-2 px-4 rounded-lg font-medium text-sm transition-colors',
            activeCategory === 'thinking'
              ? 'bg-purple-500 text-white'
              : 'bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300'
          )}
        >
          <BrainCircuit className="inline-block w-4 h-4 mr-2" />
          {t('profiles.thinking_variants')}
        </button>
      </div>

      {/* Profile Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-3">
        {(activeCategory === 'base' ? BASE_PROFILES : THINKING_PROFILES).map(profile => (
          <ProfileCard
            key={profile.id}
            profile={profile}
            selected={selectedProfileId === profile.id}
            onSelect={() => handleProfileSelect(profile)}
          />
        ))}
      </div>

      {/* Budget Warning for DEEP_THINKING */}
      {selectedProfileId === 'deep_thinking' && (
        <div className="mt-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg">
          <p className="text-sm text-yellow-800 dark:text-yellow-200">
            ⚠️ {t('profiles.warning_exceeds_limit')}
          </p>
        </div>
      )}
    </div>
  );
}
```

### Profile Card Component

```tsx
interface ProfileCardProps {
  profile: ProfilePreset;
  selected: boolean;
  onSelect: () => void;
}

function ProfileCard({ profile, selected, onSelect }: ProfileCardProps) {
  const { t } = useTranslation();

  const getCategoryIcon = () => {
    if (profile.category === 'thinking') {
      return <BrainCircuit className="w-5 h-5" />;
    }
    switch (profile.id) {
      case 'low_latency': return <Zap className="w-5 h-5" />;
      case 'high_quality': return <Sparkles className="w-5 h-5" />;
      default: return <Layers className="w-5 h-5" />;
    }
  };

  return (
    <button
      onClick={onSelect}
      className={cn(
        'p-4 rounded-lg border-2 transition-all text-left',
        selected
          ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
          : 'border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800 hover:border-blue-300'
      )}
    >
      <div className="flex items-start gap-3">
        <div className={cn(
          'p-2 rounded-lg',
          selected
            ? 'bg-blue-100 dark:bg-blue-800 text-blue-600 dark:text-blue-300'
            : 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-400'
        )}>
          {getCategoryIcon()}
        </div>

        <div className="flex-1">
          <h4 className="font-semibold text-sm text-gray-900 dark:text-white mb-1">
            {t(`profiles.${profile.id}.name`)}
          </h4>
          <p className="text-xs text-gray-600 dark:text-gray-400 mb-2">
            {t(`profiles.${profile.id}.description`)}
          </p>

          <div className="flex gap-3 text-xs">
            <span className="text-gray-500 dark:text-gray-500">
              <strong>max_tokens:</strong> {profile.max_tokens.toLocaleString()}
            </span>
            {profile.thinking_budget && (
              <span className="text-purple-600 dark:text-purple-400">
                <strong>budget:</strong> {profile.thinking_budget.toLocaleString()}
              </span>
            )}
          </div>
        </div>

        {selected && (
          <div className="text-blue-500">
            <CheckCircle className="w-5 h-5" />
          </div>
        )}
      </div>
    </button>
  );
}
```

### Integration with ApiProxy Page

**Location**: `src/pages/ApiProxy.tsx` (after proxy configuration section)

```tsx
// Add import
import { ConfigurationProfiles } from '../components/proxy/ConfigurationProfiles';

// Add state
const [selectedProfile, setSelectedProfile] = useState<ProfilePreset | null>(null);

// Add section in render (inside CollapsibleCard pattern)
<CollapsibleCard
  title={t('proxy.configuration_profiles')}
  icon={<Sparkles size={20} />}
  defaultExpanded={false}
>
  <ConfigurationProfiles
    onProfileSelect={(profile) => {
      setSelectedProfile(profile);
      // Show toast notification
      showToast({
        type: 'success',
        message: t('profiles.selected', { name: t(`profiles.${profile.id}.name`) })
      });
    }}
    selectedProfileId={selectedProfile?.id}
  />

  {/* Usage Example Card */}
  {selectedProfile && (
    <div className="mt-4 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
      <h4 className="font-medium text-sm mb-2">
        {t('profiles.usage_example')}
      </h4>
      <pre className="text-xs bg-white dark:bg-gray-900 p-3 rounded overflow-x-auto">
{`# Example API Request with ${selectedProfile.name}
curl -X POST http://localhost:8045/v1/messages \\
  -H "Content-Type: application/json" \\
  -H "Authorization: Bearer YOUR_API_KEY" \\
  -d '{
    "model": "gemini-3-pro-high",
    "max_tokens": ${selectedProfile.max_tokens},${selectedProfile.thinking_budget ? `
    "thinking": {
      "type": "enabled",
      "budget_tokens": ${selectedProfile.thinking_budget}
    },` : ''}
    "messages": [...]
  }'`}
      </pre>
    </div>
  )}
</CollapsibleCard>
```

### i18n Translations

**English** (`src/locales/en.json`):

```json
{
  "profiles": {
    "base_models": "Base Model Profiles",
    "thinking_variants": "Thinking Variants",
    "selected": "Profile selected: {{name}}",
    "usage_example": "Usage Example",
    "warning_exceeds_limit": "Warning: This profile exceeds the 32000 token budget limit for Gemini 3 Pro High. Requests will be clamped to maximum allowed values.",

    "production": {
      "name": "Production",
      "description": "Balanced configuration for production workloads"
    },
    "low_latency": {
      "name": "Low Latency",
      "description": "Fast responses with minimal token usage"
    },
    "high_quality": {
      "name": "High Quality",
      "description": "Maximum quality for comprehensive responses"
    },
    "tool_optimized": {
      "name": "Tool Optimized",
      "description": "Optimized for function calling and tool use"
    },

    "balanced_thinking": {
      "name": "Balanced Thinking",
      "description": "Balanced extended thinking for complex reasoning"
    },
    "deep_thinking": {
      "name": "Deep Thinking",
      "description": "Maximum thinking capacity for difficult problems"
    },
    "efficient_thinking": {
      "name": "Efficient Thinking",
      "description": "Quick reasoning with minimal thinking overhead"
    },
    "comprehensive_thinking": {
      "name": "Comprehensive Thinking",
      "description": "Thorough analysis with extended reasoning time"
    }
  }
}
```

**Chinese** (`src/locales/zh.json`):

```json
{
  "profiles": {
    "base_models": "基础模型配置",
    "thinking_variants": "思考模式变体",
    "selected": "已选择配置：{{name}}",
    "usage_example": "使用示例",
    "warning_exceeds_limit": "⚠️ 警告：此配置超出 Gemini 3 Pro High 的 32000 token 预算限制。请求将被限制在允许的最大值内。",

    "production": {
      "name": "生产环境",
      "description": "适用于生产工作负载的均衡配置"
    },
    "low_latency": {
      "name": "低延迟",
      "description": "最小 token 使用量的快速响应"
    },
    "high_quality": {
      "name": "高质量",
      "description": "全面响应的最高质量"
    },
    "tool_optimized": {
      "name": "工具优化",
      "description": "针对函数调用和工具使用优化"
    },

    "balanced_thinking": {
      "name": "均衡思考",
      "description": "适用于复杂推理的均衡扩展思考"
    },
    "deep_thinking": {
      "name": "深度思考",
      "description": "解决困难问题的最大思考能力"
    },
    "efficient_thinking": {
      "name": "高效思考",
      "description": "最小思考开销的快速推理"
    },
    "comprehensive_thinking": {
      "name": "全面思考",
      "description": "扩展推理时间的彻底分析"
    }
  }
}
```

### localStorage Persistence

**Key**: `last_selected_profile`
**Value**: Profile ID string (e.g., `"production"`, `"deep_thinking"`)

**Behavior**:
- Save on profile selection
- Load on component mount
- Auto-apply last selected profile
- Clear on user logout (optional)

---

## ✅ Acceptance Criteria

### AC-1: Profile Selection UI

**Given** I navigate to API Proxy configuration page
**When** I view the Configuration Profiles section
**Then** I see:
- Two tabs: "Base Model Profiles" and "Thinking Variants"
- 4 profile cards in each category
- Each card shows: icon, name, description, max_tokens, thinking_budget (if applicable)
- Selected profile is visually highlighted with blue border and checkmark

**Validation**: Manual UI testing with both light and dark themes

### AC-2: Profile Auto-Fill Functionality

**Given** I select a profile (e.g., "Production")
**When** the profile is selected
**Then**:
- Profile ID is saved to localStorage key `last_selected_profile`
- Toast notification shows: "Profile selected: Production"
- Usage example updates with correct max_tokens value
- For thinking profiles: thinking_budget is displayed in usage example

**Validation**: Browser DevTools localStorage inspection + UI visual confirmation

### AC-3: Budget Limit Warning

**Given** I select "Deep Thinking" profile (40000 max_tokens, 32000 budget)
**When** the profile is selected
**Then**:
- Yellow warning banner appears below profile grid
- Warning text: "⚠️ Warning: This profile exceeds the 32000 token budget limit..."
- Warning is NOT shown for other profiles

**Validation**: Manual testing with DEEP_THINKING profile selection

### AC-4: localStorage Persistence

**Given** I selected "High Quality" profile in previous session
**When** I close browser and reopen ApiProxy page
**Then**:
- "High Quality" profile is automatically selected
- Usage example shows high_quality configuration
- No manual re-selection needed

**Validation**:
1. Select profile
2. Close browser tab
3. Reopen page
4. Verify auto-selection

### AC-5: i18n Support

**Given** I switch language between English and Chinese
**When** viewing Configuration Profiles section
**Then**:
- Tab labels translate correctly (Base Models ↔ 基础模型配置)
- Profile names translate (Production ↔ 生产环境)
- Profile descriptions translate
- Warning messages translate
- Toast notifications translate

**Validation**: Language switcher testing (Settings → Language)

### AC-6: Responsive Design

**Given** I view Configuration Profiles on different screen sizes
**When** resizing browser window
**Then**:
- Desktop (>768px): 2-column profile grid
- Mobile (<768px): 1-column profile grid
- Touch targets ≥44px for mobile usability
- No horizontal scroll on mobile
- DaisyUI responsive classes applied correctly

**Validation**: Browser DevTools responsive design mode (375px, 768px, 1920px)

---

## 🧪 Testing Strategy

### Unit Tests

**File**: `src/components/proxy/ConfigurationProfiles.test.tsx`

```typescript
describe('ConfigurationProfiles', () => {
  test('renders base profiles by default', () => {
    render(<ConfigurationProfiles onProfileSelect={jest.fn()} />);
    expect(screen.getByText('PRODUCTION')).toBeInTheDocument();
    expect(screen.getByText('8,192')).toBeInTheDocument();
  });

  test('switches to thinking profiles on tab click', () => {
    render(<ConfigurationProfiles onProfileSelect={jest.fn()} />);
    fireEvent.click(screen.getByText('Thinking Variants'));
    expect(screen.getByText('BALANCED_THINKING')).toBeInTheDocument();
  });

  test('calls onProfileSelect with correct profile', () => {
    const onSelect = jest.fn();
    render(<ConfigurationProfiles onProfileSelect={onSelect} />);
    fireEvent.click(screen.getByText('PRODUCTION'));
    expect(onSelect).toHaveBeenCalledWith(
      expect.objectContaining({ id: 'production', max_tokens: 8192 })
    );
  });

  test('persists selection to localStorage', () => {
    const onSelect = jest.fn();
    render(<ConfigurationProfiles onProfileSelect={onSelect} />);
    fireEvent.click(screen.getByText('HIGH_QUALITY'));
    expect(localStorage.getItem('last_selected_profile')).toBe('high_quality');
  });

  test('loads last selected profile from localStorage', () => {
    localStorage.setItem('last_selected_profile', 'deep_thinking');
    const onSelect = jest.fn();
    render(<ConfigurationProfiles onProfileSelect={onSelect} />);
    expect(onSelect).toHaveBeenCalledWith(
      expect.objectContaining({ id: 'deep_thinking' })
    );
  });

  test('shows warning for DEEP_THINKING profile', () => {
    render(<ConfigurationProfiles
      onProfileSelect={jest.fn()}
      selectedProfileId="deep_thinking"
    />);
    expect(screen.getByText(/exceeds the 32000 token budget limit/)).toBeInTheDocument();
  });
});
```

### Integration Tests

**File**: `src/pages/ApiProxy.integration.test.tsx`

```typescript
describe('ApiProxy - Profile Integration', () => {
  test('profile selection updates usage example', async () => {
    render(<ApiProxy />);

    // Select profile
    fireEvent.click(screen.getByText('PRODUCTION'));

    // Wait for usage example to appear
    await waitFor(() => {
      expect(screen.getByText(/max_tokens": 8192/)).toBeInTheDocument();
    });
  });

  test('profile persists across page navigation', async () => {
    const { rerender } = render(<ApiProxy />);

    // Select profile
    fireEvent.click(screen.getByText('HIGH_QUALITY'));

    // Navigate away and back
    rerender(<Dashboard />);
    rerender(<ApiProxy />);

    // Verify profile still selected
    await waitFor(() => {
      expect(screen.getByText(/max_tokens": 16384/)).toBeInTheDocument();
    });
  });
});
```

### Visual Regression Tests

**Tool**: Percy.io or Chromatic

**Scenarios**:
1. Profile grid - base profiles view
2. Profile grid - thinking profiles view
3. Selected profile state (blue border + checkmark)
4. Warning banner display (DEEP_THINKING)
5. Responsive layouts (mobile, tablet, desktop)
6. Dark mode rendering

---

## 🔗 Dependencies and Relationships

### Sequential Dependencies

**Blocks**:
- **Story-005-08** (Update Configuration Profiles Documentation)
  - Documentation will reference this UI implementation
  - Screenshots of profile selection UI needed

**Blocked By**:
- **Story-005-01** (Model ID Discovery & Implementation)
  - MUST complete first per sequential execution order
  - No technical dependency, organizational only

### Technical Dependencies

**Frontend Dependencies**:
- React 19 (hooks: useState, useEffect)
- react-i18next (i18n support)
- lucide-react (icons: Sparkles, BrainCircuit, Layers, Zap)
- DaisyUI + Tailwind CSS (styling)
- cn utility (className merging)

**No Backend Changes Required**: This is frontend-only feature

---

## 📊 Success Metrics

### Implementation Metrics

- **Component Size**: ~300 lines (ConfigurationProfiles.tsx + ProfileCard)
- **Test Coverage**: ≥85% (6 unit tests + 2 integration tests)
- **i18n Keys**: 18 translation keys (9 profiles × 2 languages)
- **Bundle Impact**: +2KB gzipped (minimal)

### User Experience Metrics

- **Profile Selection Time**: <2 seconds (vs 30+ seconds manual calculation)
- **Configuration Errors**: -40% (reduced invalid max_tokens)
- **Usage**: Target 60% of users adopt profile presets within 30 days

### Quality Metrics

- **Accessibility**: WCAG 2.1 AA compliant (keyboard navigation, ARIA labels)
- **Performance**: <100ms render time
- **Browser Support**: Chrome 90+, Firefox 88+, Safari 14+

---

## ⚠️ Risks and Mitigation

### Risk 1: DEEP_THINKING Profile Exceeds Budget Limit

**Description**: DEEP_THINKING profile (40000 max_tokens) exceeds Gemini 3 Pro High's 32000 token budget limit

**Probability**: HIGH (100% - this is known issue)
**Impact**: MEDIUM (users see warning, proxy clamps values)

**Mitigation**:
1. ✅ Display prominent yellow warning banner
2. ✅ Document clamping behavior in tooltip
3. ✅ Consider capping max_tokens to 32000 in profile definition
4. Alternative: Add "ULTRA_THINKING" profile with 32000/24000 values

**Decision Required**: Should we cap DEEP_THINKING at 32000 or keep 40000 with warning?

### Risk 2: Profile Values Become Outdated

**Description**: Gemini 4 may have different token limits, requiring profile updates

**Probability**: MEDIUM (future model releases)
**Impact**: MEDIUM (profiles become inaccurate)

**Mitigation**:
1. Add version metadata to profiles (e.g., `model_version: "gemini-3"`)
2. Document profile maintenance in Story-005-08
3. Add unit tests validating profile values against model limits
4. Consider dynamic profiles from backend configuration

### Risk 3: localStorage Conflicts

**Description**: Multiple browser tabs may have different selected profiles

**Probability**: LOW (edge case)
**Impact**: LOW (minor UX inconsistency)

**Mitigation**:
1. Document single-tab usage assumption
2. Consider localStorage change event listener for sync
3. Add "Clear Cache" button in Settings if needed

### Risk 4: Mobile UX Degradation

**Description**: 8 profile cards may feel overwhelming on mobile screens

**Probability**: MEDIUM
**Impact**: MEDIUM (reduced mobile usability)

**Mitigation**:
1. ✅ Use 1-column grid on mobile (<768px)
2. ✅ Implement collapsible categories
3. Consider "Recommended" badge on PRODUCTION profile
4. A/B test mobile layouts if analytics available

---

## 📝 Notes for Developers

### Code Organization

```
src/
├── components/
│   └── proxy/
│       └── ConfigurationProfiles.tsx  # NEW COMPONENT
├── pages/
│   └── ApiProxy.tsx  # INTEGRATION POINT
├── locales/
│   ├── en.json  # ADD TRANSLATION KEYS
│   └── zh.json  # ADD TRANSLATION KEYS
└── types/
    └── profiles.ts  # NEW TYPE DEFINITIONS (optional)
```

### DaisyUI Component Reference

**Components Used**:
- `toggle` - Enable/disable switches
- `rounded-xl` - Card border radius
- Color utilities: `bg-blue-500`, `text-gray-600`
- Dark mode: `dark:bg-base-100`, `dark:text-white`

**Consistency Pattern**: Follow existing CollapsibleCard pattern from ApiProxy.tsx:66-110

### Accessibility Requirements

- **Keyboard Navigation**: All profiles selectable via Tab + Enter
- **Screen Readers**: ARIA labels on profile cards
- **Focus Indicators**: Visible focus ring on profile selection
- **Color Contrast**: ≥4.5:1 for text, ≥3:1 for UI elements
- **Touch Targets**: ≥44px hit area on mobile

### Performance Considerations

- **Lazy Loading**: Consider lazy import if ApiProxy page grows large
- **Memoization**: Use `useMemo` for profile filtering if performance issues
- **localStorage**: Access only once on mount, not on every render

---

## 🔍 Related Documentation

### Primary References

- **Epic-005**: `docs/epics/Epic-005-Gemini-3-Pro-High-Compliance.md`
- **COMPARISON**: `docs/antigravity/workflows/models/gemini/gemini-3-pro-high-COMPARISON.md:566-624`
- **CLAUDE.md**: Project architecture and development patterns

### Code References

- **ApiProxy Page**: `src/pages/ApiProxy.tsx` (CollapsibleCard pattern)
- **i18n Setup**: `src/locales/en.json`, `src/locales/zh.json`
- **Type Definitions**: `src/types/config.ts` (reference for new ProfilePreset type)

### Design System References

- **DaisyUI Docs**: https://daisyui.com/components/
- **Lucide Icons**: https://lucide.dev/icons/
- **Tailwind CSS**: https://tailwindcss.com/docs

---

## ✏️ Story Status Tracker

- [ ] **Phase 1: Component Structure** (1.5 hours)
  - [ ] Create `ConfigurationProfiles.tsx` component
  - [ ] Define ProfilePreset interface and constants
  - [ ] Implement ProfileCard subcomponent
  - [ ] Add category tabs (base vs thinking)

- [ ] **Phase 2: Integration** (1.5 hours)
  - [ ] Integrate with ApiProxy.tsx page
  - [ ] Add localStorage persistence
  - [ ] Implement usage example display
  - [ ] Add toast notifications

- [ ] **Phase 3: i18n & Polish** (0.5 hours)
  - [ ] Add English translations (9 profiles)
  - [ ] Add Chinese translations (9 profiles)
  - [ ] Test language switching
  - [ ] Add warning banner for DEEP_THINKING

- [ ] **Phase 4: Testing** (0.5 hours)
  - [ ] Write 6 unit tests (ConfigurationProfiles.test.tsx)
  - [ ] Write 2 integration tests (ApiProxy.integration.test.tsx)
  - [ ] Manual UI testing (light/dark modes)
  - [ ] Mobile responsive testing

**Total Effort**: 4 hours

---

**Story Created**: 2026-01-11
**Last Updated**: 2026-01-11
**Created By**: Documentation Team (Epic-005 Sequential Story Creation)
**Next Story**: Story-005-03 (Error Recovery Docs & Observability)
