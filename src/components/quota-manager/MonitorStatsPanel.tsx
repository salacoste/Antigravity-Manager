import React, { useEffect, useState, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import { Users, Database, AlertTriangle, TrendingUp } from 'lucide-react';
import { getQuotaManagerStats, MonitorStats } from '../../services/quotaManagerService';

interface MonitorStatsPanelProps {
  autoRefresh?: boolean;
  refreshInterval?: number; // milliseconds, default 30000
}

export const MonitorStatsPanel: React.FC<MonitorStatsPanelProps> = ({
  autoRefresh = true,
  refreshInterval = 30000,
}) => {
  const { t } = useTranslation();
  const [stats, setStats] = useState<MonitorStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [lastUpdated, setLastUpdated] = useState<Date | null>(null);

  // Fetch stats function
  const fetchStats = useCallback(async () => {
    try {
      setLoading(true);
      const data = await getQuotaManagerStats();
      setStats(data);
      setLastUpdated(new Date());
      setError(null);
    } catch (err) {
      console.error('Failed to fetch quota manager stats:', err);
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  // Initial fetch and auto-refresh setup
  useEffect(() => {
    fetchStats();

    if (!autoRefresh) return;

    const interval = setInterval(() => {
      fetchStats();
    }, refreshInterval);

    return () => clearInterval(interval);
  }, [autoRefresh, refreshInterval, fetchStats]);

  // Calculate health score
  const calculateHealthScore = (): number => {
    if (!stats || stats.cached_quotas === 0) return 0;
    return Math.round(
      ((stats.cached_quotas - stats.exhausted_quotas) / stats.cached_quotas) * 100
    );
  };

  // Loading state
  if (loading && !stats) {
    return (
      <div className="flex items-center justify-center p-8">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  // Error state
  if (error && !stats) {
    return (
      <div className="alert alert-error">
        <AlertTriangle className="w-4 h-4" />
        <span>Failed to load monitor statistics</span>
      </div>
    );
  }

  if (!stats) {
    return (
      <div className="alert alert-info">
        <span>{t('quota.monitorStats')}: No data available</span>
      </div>
    );
  }

  const healthScore = calculateHealthScore();

  return (
    <div className="space-y-3">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">
          {t('quota.monitorStats')}
        </h2>
        {lastUpdated && (
          <span className="text-xs text-gray-400 dark:text-gray-500">
            {t('quota.lastUpdated')}: {lastUpdated.toLocaleTimeString()}
          </span>
        )}
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        {/* Total Accounts */}
        <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
          <div className="flex items-center justify-between mb-2">
            <div className="p-1.5 bg-blue-50 dark:bg-blue-900/20 rounded-md">
              <Users className="w-4 h-4 text-blue-500 dark:text-blue-400" />
            </div>
          </div>
          <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-0.5">
            {stats.total_accounts}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">
            {t('quota.totalAccounts')}
          </div>
        </div>

        {/* Cached Quotas */}
        <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
          <div className="flex items-center justify-between mb-2">
            <div className="p-1.5 bg-green-50 dark:bg-green-900/20 rounded-md">
              <Database className="w-4 h-4 text-green-500 dark:text-green-400" />
            </div>
          </div>
          <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-0.5">
            {stats.cached_quotas}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">
            {t('quota.cachedQuotas')}
          </div>
          {stats.expired_entries > 0 && (
            <div className="text-[10px] text-gray-400 dark:text-gray-500 mt-1">
              {stats.expired_entries} {t('quota.expired')}
            </div>
          )}
        </div>

        {/* Exhausted Quotas */}
        <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
          <div className="flex items-center justify-between mb-2">
            <div className="p-1.5 bg-orange-50 dark:bg-orange-900/20 rounded-md">
              <AlertTriangle className="w-4 h-4 text-orange-500 dark:text-orange-400" />
            </div>
          </div>
          <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-0.5">
            {stats.exhausted_quotas}
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">
            {t('quota.exhaustedQuotas')}
          </div>
        </div>

        {/* Health Score */}
        <div className="bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border border-gray-100 dark:border-base-200">
          <div className="flex items-center justify-between mb-2">
            <div className="p-1.5 bg-purple-50 dark:bg-purple-900/20 rounded-md">
              <TrendingUp className="w-4 h-4 text-purple-500 dark:text-purple-400" />
            </div>
          </div>
          <div className="text-2xl font-bold text-gray-900 dark:text-base-content mb-0.5">
            {healthScore}%
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400">
            {t('quota.healthScore')}
          </div>
          <div
            className={`text-[10px] mt-1 ${
              healthScore >= 80
                ? 'text-green-600 dark:text-green-400'
                : healthScore >= 50
                ? 'text-orange-600 dark:text-orange-400'
                : 'text-red-600 dark:text-red-400'
            }`}
          >
            {healthScore >= 80
              ? t('quota.healthy')
              : healthScore >= 50
              ? t('quota.low')
              : t('quota.exhausted')}
          </div>
        </div>
      </div>

      {/* Sync Statistics (Optional detailed info) */}
      {stats.sync_success_count > 0 && (
        <div className="text-xs text-gray-400 dark:text-gray-500 text-center">
          Sync stats: {stats.sync_success_count} success, {stats.sync_error_count} errors
          {stats.last_sync_duration_ms > 0 && (
            <> · Last sync: {stats.last_sync_duration_ms}ms</>
          )}
        </div>
      )}
    </div>
  );
};
