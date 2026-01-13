import { create } from 'zustand';

// Types for budget optimization
export interface SavingsMetrics {
    totalSaved: number;
    percentageSaved: number;
    requestsOptimized: number;
    totalRequests: number;
}

export interface ComplexityDistribution {
    simple: number;
    moderate: number;
    complex: number;
}

export interface ClassificationRecord {
    id: string;
    timestamp: number;
    model: string;
    requestTokens: number;
    classifiedTier: 'simple' | 'moderate' | 'complex';
    actualCost: number;
    estimatedOriginalCost: number;
    saved: number;
    accuracy: number;
}

export interface HistoricalSavings {
    date: string;
    totalSaved: number;
    requestCount: number;
}

interface BudgetState {
    // Configuration
    enabled: boolean;
    manualBudget: number | null;

    // Metrics
    savings: SavingsMetrics;
    complexityDistribution: ComplexityDistribution;
    classifications: ClassificationRecord[];
    historicalSavings: HistoricalSavings[];

    // Accuracy metrics
    classificationAccuracy: number;
    falsePositives: number;
    falseNegatives: number;

    // Actions
    setEnabled: (enabled: boolean) => void;
    setManualBudget: (budget: number | null) => void;
    setSavings: (savings: SavingsMetrics) => void;
    setComplexityDistribution: (distribution: ComplexityDistribution) => void;
    addClassification: (record: ClassificationRecord) => void;
    setHistoricalSavings: (history: HistoricalSavings[]) => void;
    setClassificationAccuracy: (accuracy: number) => void;

    // Mock data initialization (for development)
    initializeMockData: () => void;
}

export const useBudgetStore = create<BudgetState>((set) => ({
    // Initial state
    enabled: true,
    manualBudget: null,
    savings: {
        totalSaved: 0,
        percentageSaved: 0,
        requestsOptimized: 0,
        totalRequests: 0,
    },
    complexityDistribution: {
        simple: 0,
        moderate: 0,
        complex: 0,
    },
    classifications: [],
    historicalSavings: [],
    classificationAccuracy: 0,
    falsePositives: 0,
    falseNegatives: 0,

    // Actions
    setEnabled: (enabled) => set({ enabled }),

    setManualBudget: (budget) => set({ manualBudget: budget }),

    setSavings: (savings) => set({ savings }),

    setComplexityDistribution: (distribution) => set({ complexityDistribution: distribution }),

    addClassification: (record) => set((state) => ({
        classifications: [record, ...state.classifications].slice(0, 100), // Keep last 100
    })),

    setHistoricalSavings: (history) => set({ historicalSavings: history }),

    setClassificationAccuracy: (accuracy) => set({ classificationAccuracy: accuracy }),

    // Initialize mock data for development
    initializeMockData: () => {
        const now = Date.now();
        const mockClassifications: ClassificationRecord[] = [];

        // Generate 50 mock classification records
        for (let i = 0; i < 50; i++) {
            const tiers: Array<'simple' | 'moderate' | 'complex'> = ['simple', 'moderate', 'complex'];
            const tier = tiers[Math.floor(Math.random() * tiers.length)];
            const tokens = tier === 'simple' ? 100 + Math.random() * 400 :
                          tier === 'moderate' ? 500 + Math.random() * 1000 :
                          1500 + Math.random() * 3000;

            const actualCost = tokens * 0.001; // Mock cost calculation
            const estimatedOriginalCost = tokens * 0.0015; // Higher cost for comparison

            mockClassifications.push({
                id: `cls-${i}`,
                timestamp: now - (i * 3600000), // One hour apart
                model: tier === 'simple' ? 'gemini-2.0-flash-lite' :
                       tier === 'moderate' ? 'gemini-2.0-flash' :
                       'gemini-2.5-pro',
                requestTokens: Math.round(tokens),
                classifiedTier: tier,
                actualCost,
                estimatedOriginalCost,
                saved: estimatedOriginalCost - actualCost,
                accuracy: 0.85 + Math.random() * 0.15,
            });
        }

        // Calculate complexity distribution
        const distribution = {
            simple: mockClassifications.filter(c => c.classifiedTier === 'simple').length,
            moderate: mockClassifications.filter(c => c.classifiedTier === 'moderate').length,
            complex: mockClassifications.filter(c => c.classifiedTier === 'complex').length,
        };

        // Calculate total savings
        const totalSaved = mockClassifications.reduce((sum, c) => sum + c.saved, 0);
        const totalRequests = mockClassifications.length;
        const requestsOptimized = mockClassifications.filter(c => c.saved > 0).length;

        // Generate historical savings (last 30 days)
        const historicalSavings: HistoricalSavings[] = [];
        for (let i = 29; i >= 0; i--) {
            const date = new Date(now - i * 86400000).toISOString().split('T')[0];
            historicalSavings.push({
                date,
                totalSaved: 5 + Math.random() * 20,
                requestCount: 50 + Math.floor(Math.random() * 100),
            });
        }

        set({
            classifications: mockClassifications,
            complexityDistribution: distribution,
            savings: {
                totalSaved,
                percentageSaved: 33.5,
                requestsOptimized,
                totalRequests,
            },
            historicalSavings,
            classificationAccuracy: 87.5,
            falsePositives: 6,
            falseNegatives: 8,
        });
    },
}));
