import { TrendingDown, Activity, Target, BarChart3 } from 'lucide-react';
import { useBudgetStore } from '../../stores/useBudgetStore';
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip, Legend } from 'recharts';
import { LineChart, Line, XAxis, YAxis, CartesianGrid } from 'recharts';

function SavingsDashboard() {
    const {
        savings,
        complexityDistribution,
        historicalSavings,
        classificationAccuracy
    } = useBudgetStore();

    // Prepare complexity distribution data for pie chart
    const complexityData = [
        { name: 'Simple', value: complexityDistribution.simple, color: '#22c55e' },
        { name: 'Moderate', value: complexityDistribution.moderate, color: '#f59e0b' },
        { name: 'Complex', value: complexityDistribution.complex, color: '#ef4444' },
    ].filter(item => item.value > 0);

    // Prepare historical data for line chart (last 30 days)
    const chartData = historicalSavings.slice(-30).map(h => ({
        date: new Date(h.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
        saved: h.totalSaved,
        requests: h.requestCount,
    }));

    return (
        <div className="space-y-4">
            {/* Summary Cards */}
            <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                {/* Total Saved */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
                    <div className="flex items-center justify-between mb-2">
                        <div className="p-2 bg-green-50 dark:bg-green-900/20 rounded-lg">
                            <TrendingDown className="w-5 h-5 text-green-500 dark:text-green-400" />
                        </div>
                    </div>
                    <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-1">
                        ${savings.totalSaved.toFixed(2)}
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400">Total Saved</div>
                    <div className="text-xs text-green-600 dark:text-green-400 mt-1">
                        {savings.percentageSaved.toFixed(1)}% reduction
                    </div>
                </div>

                {/* Requests Optimized */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
                    <div className="flex items-center justify-between mb-2">
                        <div className="p-2 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
                            <Activity className="w-5 h-5 text-blue-500 dark:text-blue-400" />
                        </div>
                    </div>
                    <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-1">
                        {savings.requestsOptimized}
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400">Requests Optimized</div>
                    <div className="text-xs text-blue-600 dark:text-blue-400 mt-1">
                        of {savings.totalRequests} total
                    </div>
                </div>

                {/* Classification Accuracy */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
                    <div className="flex items-center justify-between mb-2">
                        <div className="p-2 bg-purple-50 dark:bg-purple-900/20 rounded-lg">
                            <Target className="w-5 h-5 text-purple-500 dark:text-purple-400" />
                        </div>
                    </div>
                    <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-1">
                        {classificationAccuracy.toFixed(1)}%
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400">Classification Accuracy</div>
                    <div className={`text-xs mt-1 ${
                        classificationAccuracy >= 85
                            ? 'text-green-600 dark:text-green-400'
                            : 'text-orange-600 dark:text-orange-400'
                    }`}>
                        {classificationAccuracy >= 85 ? 'Excellent' : 'Good'}
                    </div>
                </div>

                {/* Distribution Insights */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
                    <div className="flex items-center justify-between mb-2">
                        <div className="p-2 bg-orange-50 dark:bg-orange-900/20 rounded-lg">
                            <BarChart3 className="w-5 h-5 text-orange-500 dark:text-orange-400" />
                        </div>
                    </div>
                    <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-1">
                        {complexityDistribution.simple + complexityDistribution.moderate + complexityDistribution.complex}
                    </div>
                    <div className="text-xs text-gray-500 dark:text-gray-400">Total Classifications</div>
                    <div className="text-xs text-orange-600 dark:text-orange-400 mt-1">
                        {((complexityDistribution.simple / (complexityDistribution.simple + complexityDistribution.moderate + complexityDistribution.complex)) * 100).toFixed(0)}% simple tasks
                    </div>
                </div>
            </div>

            {/* Charts Row */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {/* Complexity Distribution Pie Chart */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-base-content mb-4">
                        Complexity Distribution
                    </h3>
                    {complexityData.length > 0 ? (
                        <ResponsiveContainer width="100%" height={250}>
                            <PieChart>
                                <Pie
                                    data={complexityData}
                                    cx="50%"
                                    cy="50%"
                                    labelLine={false}
                                    label={({ name, percent }) => `${name} ${((percent || 0) * 100).toFixed(0)}%`}
                                    outerRadius={80}
                                    fill="#8884d8"
                                    dataKey="value"
                                >
                                    {complexityData.map((entry, index) => (
                                        <Cell key={`cell-${index}`} fill={entry.color} />
                                    ))}
                                </Pie>
                                <Tooltip />
                                <Legend />
                            </PieChart>
                        </ResponsiveContainer>
                    ) : (
                        <div className="h-[250px] flex items-center justify-center text-gray-400">
                            No data available
                        </div>
                    )}
                </div>

                {/* Historical Savings Trend */}
                <div className="bg-white dark:bg-base-100 rounded-xl p-6 shadow-sm border border-gray-100 dark:border-base-200">
                    <h3 className="text-lg font-semibold text-gray-900 dark:text-base-content mb-4">
                        Cost Savings Trend (30 Days)
                    </h3>
                    {chartData.length > 0 ? (
                        <ResponsiveContainer width="100%" height={250}>
                            <LineChart data={chartData}>
                                <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
                                <XAxis
                                    dataKey="date"
                                    tick={{ fontSize: 12 }}
                                    stroke="#9ca3af"
                                />
                                <YAxis
                                    tick={{ fontSize: 12 }}
                                    stroke="#9ca3af"
                                />
                                <Tooltip
                                    contentStyle={{
                                        backgroundColor: '#fff',
                                        border: '1px solid #e5e7eb',
                                        borderRadius: '8px'
                                    }}
                                />
                                <Line
                                    type="monotone"
                                    dataKey="saved"
                                    stroke="#22c55e"
                                    strokeWidth={2}
                                    dot={{ fill: '#22c55e', r: 3 }}
                                    activeDot={{ r: 5 }}
                                />
                            </LineChart>
                        </ResponsiveContainer>
                    ) : (
                        <div className="h-[250px] flex items-center justify-center text-gray-400">
                            No historical data available
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}

export default SavingsDashboard;
