// Epic-025 Story-025-03: Budget Escalation Monitoring Widget
//
// Real-time visualization of budget sufficiency detection and automatic escalations
// Displays: Escalation metrics, history, budget ladder, and success rates

import { useEffect, useState } from 'react';
import { TrendingUp, ArrowUp, CheckCircle, XCircle } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';

// ========== Types ==========

interface EscalationMetrics {
    total_escalations: number;
    successful_escalations: number;
    failed_escalations: number;
    escalation_success_rate: number;
    unnecessary_escalations: number;
    unnecessary_rate: number;
    budget_4k_to_12k: number;
    budget_12k_to_24k: number;
    budget_24k_to_pro: number;
    avg_escalation_latency_ms: number;
}

interface EscalationEvent {
    request_id: string;
    original_budget: number;
    escalated_budget: number;
    model_switch: string | null;
    timestamp: string;
    success: boolean;
    finish_reason: string;
}

// ========== Component ==========

export default function BudgetEscalationWidget() {
    const [metrics, setMetrics] = useState<EscalationMetrics | null>(null);
    const [history, setHistory] = useState<EscalationEvent[]>([]);
    const [loading, setLoading] = useState(true);

    // Fetch metrics and history
    const fetchData = async () => {
        try {
            const [metricsData, historyData] = await Promise.all([
                invoke<EscalationMetrics>('get_escalation_metrics'),
                invoke<EscalationEvent[]>('get_escalation_history', { limit: 10 }),
            ]);
            setMetrics(metricsData);
            setHistory(historyData);
        } catch (error) {
            console.error('[BudgetEscalation] Failed to fetch data:', error);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchData();
        // Refresh every 10 seconds
        const interval = setInterval(fetchData, 10000);
        return () => clearInterval(interval);
    }, []);

    const formatBudget = (budget: number): string => {
        if (budget >= 1000) {
            return `${(budget / 1000).toFixed(0)}K`;
        }
        return budget.toString();
    };

    const formatTimestamp = (timestamp: string): string => {
        return new Date(timestamp).toLocaleTimeString();
    };

    if (loading) {
        return (
            <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
                <div className="animate-pulse">
                    <div className="h-6 bg-gray-200 dark:bg-gray-700 rounded w-1/3 mb-4"></div>
                    <div className="h-24 bg-gray-200 dark:bg-gray-700 rounded"></div>
                </div>
            </div>
        );
    }

    if (!metrics) {
        return null;
    }

    return (
        <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
            {/* Header */}
            <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-3">
                    <div className="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                        <TrendingUp className="w-6 h-6 text-blue-500 dark:text-blue-400" />
                    </div>
                    <div>
                        <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">
                            Budget Escalation
                        </h2>
                        <p className="text-sm text-gray-500 dark:text-gray-400">
                            Automatic budget sufficiency detection
                        </p>
                    </div>
                </div>

                <div className="text-right">
                    <div className="text-2xl font-bold text-blue-600 dark:text-blue-400">
                        {metrics.escalation_success_rate.toFixed(1)}%
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400">Success Rate</div>
                </div>
            </div>

            {/* Metrics Grid */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
                <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
                    <div className="text-2xl font-bold text-green-600 dark:text-green-400">
                        {metrics.successful_escalations}
                    </div>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mt-1">Successful</div>
                </div>

                <div className="bg-red-50 dark:bg-red-900/20 p-4 rounded-lg">
                    <div className="text-2xl font-bold text-red-600 dark:text-red-400">
                        {metrics.failed_escalations}
                    </div>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mt-1">Failed</div>
                </div>

                <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg">
                    <div className="text-2xl font-bold text-yellow-600 dark:text-yellow-400">
                        {metrics.unnecessary_escalations}
                    </div>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mt-1">Unnecessary</div>
                </div>

                <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
                    <div className="text-2xl font-bold text-purple-600 dark:text-purple-400">
                        {metrics.avg_escalation_latency_ms.toFixed(0)}ms
                    </div>
                    <div className="text-xs text-gray-600 dark:text-gray-400 mt-1">Avg Latency</div>
                </div>
            </div>

            {/* Budget Ladder */}
            <div className="mb-6">
                <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                    Escalation Ladder
                </h3>
                <div className="grid grid-cols-3 gap-3">
                    <div className="bg-gray-50 dark:bg-base-200 p-3 rounded-lg">
                        <div className="flex items-center gap-2 mb-1">
                            <ArrowUp className="w-4 h-4 text-blue-500" />
                            <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
                                4K → 12K
                            </span>
                        </div>
                        <div className="text-xl font-bold text-gray-900 dark:text-base-content">
                            {metrics.budget_4k_to_12k}
                        </div>
                    </div>

                    <div className="bg-gray-50 dark:bg-base-200 p-3 rounded-lg">
                        <div className="flex items-center gap-2 mb-1">
                            <ArrowUp className="w-4 h-4 text-purple-500" />
                            <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
                                12K → 24K
                            </span>
                        </div>
                        <div className="text-xl font-bold text-gray-900 dark:text-base-content">
                            {metrics.budget_12k_to_24k}
                        </div>
                    </div>

                    <div className="bg-gray-50 dark:bg-base-200 p-3 rounded-lg">
                        <div className="flex items-center gap-2 mb-1">
                            <ArrowUp className="w-4 h-4 text-orange-500" />
                            <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
                                24K → Pro 32K
                            </span>
                        </div>
                        <div className="text-xl font-bold text-gray-900 dark:text-base-content">
                            {metrics.budget_24k_to_pro}
                        </div>
                    </div>
                </div>
            </div>

            {/* Recent History */}
            {history.length > 0 && (
                <div>
                    <h3 className="text-sm font-semibold text-gray-700 dark:text-gray-300 mb-3">
                        Recent Escalations
                    </h3>
                    <div className="space-y-2">
                        {history.slice(0, 5).map((event, idx) => (
                            <div
                                key={idx}
                                className="flex items-center justify-between p-3 bg-gray-50 dark:bg-base-200 rounded-lg"
                            >
                                <div className="flex items-center gap-3">
                                    {event.success ? (
                                        <CheckCircle className="w-5 h-5 text-green-500" />
                                    ) : (
                                        <XCircle className="w-5 h-5 text-red-500" />
                                    )}
                                    <div>
                                        <div className="text-sm font-medium text-gray-900 dark:text-base-content">
                                            {formatBudget(event.original_budget)} →{' '}
                                            {formatBudget(event.escalated_budget)}
                                        </div>
                                        <div className="text-xs text-gray-500 dark:text-gray-400">
                                            {formatTimestamp(event.timestamp)}
                                            {event.model_switch && (
                                                <span className="ml-2 text-orange-500">
                                                    ({event.model_switch})
                                                </span>
                                            )}
                                        </div>
                                    </div>
                                </div>
                                <div className="text-xs text-gray-400 dark:text-gray-500">
                                    {event.finish_reason}
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            {/* No Data Message */}
            {metrics.total_escalations === 0 && (
                <div className="text-center py-8 text-gray-500 dark:text-gray-400">
                    <p className="text-sm">No escalations yet</p>
                    <p className="text-xs mt-1">
                        Budget escalations will appear here when insufficient budgets are detected
                    </p>
                </div>
            )}
        </div>
    );
}
