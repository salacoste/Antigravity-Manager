import { useState } from 'react';
import { Settings, DollarSign, Zap, Check, X } from 'lucide-react';
import { useBudgetStore } from '../../stores/useBudgetStore';

function BudgetOptimizerWidget() {
    const { enabled, manualBudget, setEnabled, setManualBudget } = useBudgetStore();
    const [budgetInput, setBudgetInput] = useState<string>(manualBudget?.toString() || '');
    const [isEditing, setIsEditing] = useState(false);

    const handleToggle = () => {
        setEnabled(!enabled);
    };

    const handleBudgetSave = () => {
        const value = parseFloat(budgetInput);
        if (!isNaN(value) && value > 0) {
            setManualBudget(value);
            setIsEditing(false);
        } else if (budgetInput === '') {
            setManualBudget(null);
            setIsEditing(false);
        }
    };

    const handleBudgetCancel = () => {
        setBudgetInput(manualBudget?.toString() || '');
        setIsEditing(false);
    };

    return (
        <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
            <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-3">
                    <div className="p-2 bg-green-50 dark:bg-green-900/20 rounded-lg">
                        <Settings className="w-6 h-6 text-green-500 dark:text-green-400" />
                    </div>
                    <div>
                        <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">
                            Budget Optimizer
                        </h2>
                        <p className="text-sm text-gray-500 dark:text-gray-400">
                            Automatically classify requests to optimize costs
                        </p>
                    </div>
                </div>

                <label className="cursor-pointer label gap-2">
                    <span className="label-text text-sm font-medium">
                        {enabled ? 'Enabled' : 'Disabled'}
                    </span>
                    <input
                        type="checkbox"
                        className="toggle toggle-success"
                        checked={enabled}
                        onChange={handleToggle}
                    />
                </label>
            </div>

            {/* Budget Configuration */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Auto Mode Card */}
                <div className={`p-4 rounded-lg border-2 transition-all ${
                    !manualBudget
                        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                        : 'border-gray-200 dark:border-base-300 bg-gray-50 dark:bg-base-200'
                }`}>
                    <div className="flex items-center gap-2 mb-2">
                        <Zap className={`w-5 h-5 ${!manualBudget ? 'text-blue-500' : 'text-gray-400'}`} />
                        <h3 className="font-semibold text-gray-900 dark:text-base-content">
                            Auto Mode
                        </h3>
                        {!manualBudget && (
                            <span className="badge badge-primary badge-sm">Active</span>
                        )}
                    </div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                        Automatically classify requests based on complexity analysis
                    </p>
                </div>

                {/* Manual Budget Card */}
                <div className={`p-4 rounded-lg border-2 transition-all ${
                    manualBudget
                        ? 'border-purple-500 bg-purple-50 dark:bg-purple-900/20'
                        : 'border-gray-200 dark:border-base-300 bg-gray-50 dark:bg-base-200'
                }`}>
                    <div className="flex items-center gap-2 mb-2">
                        <DollarSign className={`w-5 h-5 ${manualBudget ? 'text-purple-500' : 'text-gray-400'}`} />
                        <h3 className="font-semibold text-gray-900 dark:text-base-content">
                            Manual Budget
                        </h3>
                        {manualBudget && (
                            <span className="badge badge-secondary badge-sm">Active</span>
                        )}
                    </div>

                    {!isEditing ? (
                        <div>
                            <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
                                {manualBudget
                                    ? `Current budget: $${manualBudget.toFixed(2)}/day`
                                    : 'Set a daily budget limit'
                                }
                            </p>
                            <button
                                className="btn btn-sm btn-outline btn-primary"
                                onClick={() => setIsEditing(true)}
                            >
                                {manualBudget ? 'Edit Budget' : 'Set Budget'}
                            </button>
                        </div>
                    ) : (
                        <div className="space-y-2">
                            <input
                                type="number"
                                className="input input-sm input-bordered w-full"
                                placeholder="Enter daily budget"
                                value={budgetInput}
                                onChange={(e) => setBudgetInput(e.target.value)}
                                min="0"
                                step="0.01"
                            />
                            <div className="flex gap-2">
                                <button
                                    className="btn btn-xs btn-success gap-1"
                                    onClick={handleBudgetSave}
                                >
                                    <Check className="w-3 h-3" />
                                    Save
                                </button>
                                <button
                                    className="btn btn-xs btn-ghost gap-1"
                                    onClick={handleBudgetCancel}
                                >
                                    <X className="w-3 h-3" />
                                    Cancel
                                </button>
                            </div>
                        </div>
                    )}
                </div>
            </div>

            {/* Status Indicator */}
            {enabled && (
                <div className="mt-4 p-3 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-900/30">
                    <div className="flex items-center gap-2">
                        <div className="w-2 h-2 rounded-full bg-green-500 animate-pulse" />
                        <span className="text-sm font-medium text-green-700 dark:text-green-300">
                            Budget optimizer is active
                        </span>
                    </div>
                </div>
            )}
        </div>
    );
}

export default BudgetOptimizerWidget;
