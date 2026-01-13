import { useEffect, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { Activity, TrendingUp } from 'lucide-react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import QuotaMonitoringWidget from '../components/quota/QuotaMonitoringWidget';
import AlertNotificationSystem from '../components/alerts/AlertNotificationSystem';
import { useAccountStore } from '../stores/useAccountStore';
import { useQuotaStore } from '../stores/useQuotaStore';

function QuotaHistoryChart() {
  const { t } = useTranslation();
  const { accounts } = useAccountStore();
  const { history } = useQuotaStore();

  // Transform history data for chart
  const chartData = useMemo(() => {
    if (history.length === 0) return [];

    // Group snapshots by timestamp
    const timestampMap = new Map<number, Record<string, number>>();

    history.forEach(snapshot => {
      const timestamp = snapshot.timestamp;
      if (!timestampMap.has(timestamp)) {
        timestampMap.set(timestamp, {});
      }

      const data = timestampMap.get(timestamp)!;

      // Get account email for legend
      const account = accounts.find(a => a.id === snapshot.accountId);
      const accountLabel = account ? account.email.split('@')[0] : snapshot.accountId.slice(0, 8);

      // Use the primary model quota (first available)
      const primaryQuota = Object.values(snapshot.quotas)[0];
      if (primaryQuota !== undefined) {
        data[accountLabel] = primaryQuota;
      }
    });

    // Convert to array and sort by timestamp
    return Array.from(timestampMap.entries())
      .map(([timestamp, quotas]) => ({
        timestamp,
        date: new Date(timestamp).toLocaleDateString(),
        time: new Date(timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        ...quotas,
      }))
      .sort((a, b) => a.timestamp - b.timestamp)
      .slice(-48); // Last 48 data points (2 days if hourly)
  }, [history, accounts]);

  // Get unique account keys for lines
  const accountKeys = useMemo(() => {
    if (chartData.length === 0) return [];
    const keys = new Set<string>();
    chartData.forEach(data => {
      Object.keys(data).forEach(key => {
        if (key !== 'timestamp' && key !== 'date' && key !== 'time') {
          keys.add(key);
        }
      });
    });
    return Array.from(keys).slice(0, 5); // Limit to 5 accounts for readability
  }, [chartData]);

  // Color palette for lines
  const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6'];

  if (chartData.length === 0) {
    return (
      <div className="bg-white dark:bg-base-100 rounded-lg p-8 shadow-sm border border-gray-200 dark:border-base-200">
        <div className="text-center text-gray-500 dark:text-gray-400">
          <TrendingUp className="w-12 h-12 mx-auto mb-3 opacity-50" />
          <p>{t('quota.no_history_data')}</p>
          <p className="text-xs mt-1">{t('quota.history_accumulating')}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white dark:bg-base-100 rounded-lg p-6 shadow-sm border border-gray-200 dark:border-base-200">
      <div className="flex items-center gap-3 mb-4">
        <div className="p-2 bg-blue-100 dark:bg-blue-900/20 rounded-lg">
          <TrendingUp className="w-5 h-5 text-blue-600 dark:text-blue-400" />
        </div>
        <div>
          <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">
            {t('quota.history_title')}
          </h2>
          <p className="text-sm text-gray-500 dark:text-gray-400">
            {t('quota.history_subtitle')}
          </p>
        </div>
      </div>

      <ResponsiveContainer width="100%" height={300}>
        <LineChart data={chartData}>
          <CartesianGrid strokeDasharray="3 3" stroke="#e5e7eb" />
          <XAxis
            dataKey="time"
            stroke="#9ca3af"
            style={{ fontSize: '12px' }}
          />
          <YAxis
            stroke="#9ca3af"
            style={{ fontSize: '12px' }}
            domain={[0, 100]}
            label={{ value: 'Quota %', angle: -90, position: 'insideLeft', style: { fontSize: '12px' } }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: 'rgba(255, 255, 255, 0.95)',
              border: '1px solid #e5e7eb',
              borderRadius: '8px',
              fontSize: '12px',
            }}
            formatter={(value: number) => [`${value.toFixed(1)}%`, 'Quota']}
          />
          <Legend
            wrapperStyle={{ fontSize: '12px' }}
            iconType="line"
          />
          {accountKeys.map((key, index) => (
            <Line
              key={key}
              type="monotone"
              dataKey={key}
              stroke={colors[index % colors.length]}
              strokeWidth={2}
              dot={false}
              activeDot={{ r: 4 }}
            />
          ))}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}

export default function QuotaMonitoringPage() {
  const { t } = useTranslation();
  const { fetchAccounts } = useAccountStore();

  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts]);

  return (
    <div className="h-full w-full overflow-y-auto">
      <div className="p-5 space-y-6 max-w-7xl mx-auto">
        {/* Page Header */}
        <div className="flex items-center gap-3">
          <div className="p-3 bg-gradient-to-br from-blue-500 to-purple-600 rounded-xl shadow-lg">
            <Activity className="w-6 h-6 text-white" />
          </div>
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-base-content">
              {t('quota.page_title')}
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              {t('quota.page_subtitle')}
            </p>
          </div>
        </div>

        {/* Real-time Quota Monitoring */}
        <section>
          <QuotaMonitoringWidget />
        </section>

        {/* Historical Quota Chart */}
        <section>
          <QuotaHistoryChart />
        </section>

        {/* Alert Notification System */}
        <section>
          <AlertNotificationSystem />
        </section>
      </div>
    </div>
  );
}
