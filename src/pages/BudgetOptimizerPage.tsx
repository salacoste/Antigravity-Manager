import { useEffect } from 'react';
import { TrendingUp, Info } from 'lucide-react';
import BudgetOptimizerWidget from '../components/budget/BudgetOptimizerWidget';
import SavingsDashboard from '../components/budget/SavingsDashboard';
import { useBudgetStore } from '../stores/useBudgetStore';

function BudgetOptimizerPage() {
    const { initializeMockData } = useBudgetStore();

    // Initialize mock data on mount (for development)
    useEffect(() => {
        initializeMockData();
    }, [initializeMockData]);

    return (
        <div className="h-full w-full overflow-y-auto">
            <div className="p-5 space-y-5 max-w-7xl mx-auto">
                {/* Page Header */}
                <div className="flex items-center justify-between">
                    <div>
                        <h1 className="text-2xl font-bold text-gray-900 dark:text-base-content flex items-center gap-2">
                            <TrendingUp className="w-7 h-7 text-green-500" />
                            Budget Optimizer
                        </h1>
                        <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                            Intelligent request classification to optimize API costs
                        </p>
                    </div>
                </div>

                {/* Info Banner */}
                <div className="alert alert-info shadow-sm">
                    <Info className="w-5 h-5" />
                    <div>
                        <h3 className="font-semibold">How Budget Optimizer Works</h3>
                        <div className="text-sm mt-1">
                            The system analyzes request complexity and automatically routes to cost-efficient models:
                            <ul className="list-disc list-inside mt-1 ml-2">
                                <li><strong>Simple</strong> tasks → gemini-2.0-flash-lite (fastest, cheapest)</li>
                                <li><strong>Moderate</strong> tasks → gemini-2.0-flash (balanced)</li>
                                <li><strong>Complex</strong> tasks → gemini-2.5-pro (highest quality)</li>
                            </ul>
                        </div>
                    </div>
                </div>

                {/* Budget Configuration Widget */}
                <BudgetOptimizerWidget />

                {/* Savings Dashboard */}
                <SavingsDashboard />

                {/* Classification History Table */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-base-content mb-4">
                        Recent Classifications
                    </h3>
                    <div className="overflow-x-auto">
                        <table className="table table-sm">
                            <thead>
                                <tr>
                                    <th>Timestamp</th>
                                    <th>Model</th>
                                    <th>Tier</th>
                                    <th>Tokens</th>
                                    <th>Cost</th>
                                    <th>Saved</th>
                                    <th>Accuracy</th>
                                </tr>
                            </thead>
                            <tbody>
                                {useBudgetStore.getState().classifications.slice(0, 10).map((record) => (
                                    <tr key={record.id}>
                                        <td className="text-xs">
                                            {new Date(record.timestamp).toLocaleString('en-US', {
                                                month: 'short',
                                                day: 'numeric',
                                                hour: '2-digit',
                                                minute: '2-digit',
                                            })}
                                        </td>
                                        <td className="text-xs font-mono">{record.model}</td>
                                        <td>
                                            <span className={`badge badge-sm ${
                                                record.classifiedTier === 'simple' ? 'badge-success' :
                                                record.classifiedTier === 'moderate' ? 'badge-warning' :
                                                'badge-error'
                                            }`}>
                                                {record.classifiedTier}
                                            </span>
                                        </td>
                                        <td className="text-xs">{record.requestTokens.toLocaleString()}</td>
                                        <td className="text-xs">${record.actualCost.toFixed(4)}</td>
                                        <td className="text-xs text-green-600 dark:text-green-400">
                                            ${record.saved.toFixed(4)}
                                        </td>
                                        <td className="text-xs">
                                            <div className="flex items-center gap-1">
                                                <div
                                                    className="radial-progress text-xs"
                                                    style={{
                                                        '--value': record.accuracy * 100,
                                                        '--size': '2rem',
                                                        '--thickness': '3px'
                                                    } as React.CSSProperties}
                                                >
                                                    {(record.accuracy * 100).toFixed(0)}%
                                                </div>
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default BudgetOptimizerPage;
