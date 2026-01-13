import { useEffect, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { Activity, AlertTriangle, CheckCircle, Clock, TrendingDown } from 'lucide-react';
import { useAccountStore } from '../../stores/useAccountStore';
import { useQuotaStore } from '../../stores/useQuotaStore';
import { Account } from '../../types/account';

interface AccountQuotaCardProps {
  account: Account;
  onViewDetails?: (accountId: string) => void;
}

function AccountQuotaCard({ account, onViewDetails }: AccountQuotaCardProps) {
  const { t } = useTranslation();

  // Calculate overall health status
  const healthStatus = useMemo(() => {
    if (!account.quota || account.quota.is_forbidden) {
      return { status: 'error', color: 'red', label: t('quota.status.blocked') };
    }

    const minQuota = Math.min(...account.quota.models.map(m => m.percentage));

    if (minQuota >= 50) {
      return { status: 'healthy', color: 'green', label: t('quota.status.healthy') };
    } else if (minQuota >= 20) {
      return { status: 'warning', color: 'yellow', label: t('quota.status.warning') };
    } else {
      return { status: 'critical', color: 'red', label: t('quota.status.critical') };
    }
  }, [account, t]);

  // Get primary models
  const primaryModels = useMemo(() => {
    if (!account.quota) return [];
    return account.quota.models
      .filter(m =>
        m.name.toLowerCase().includes('gemini') ||
        m.name.toLowerCase().includes('claude')
      )
      .slice(0, 3);
  }, [account]);

  return (
    <div className="bg-white dark:bg-base-100 rounded-lg p-4 shadow-sm border border-gray-100 dark:border-base-200 hover:shadow-md transition-shadow">
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <div className={`w-2 h-2 rounded-full bg-${healthStatus.color}-500`}></div>
          <span className="font-medium text-sm text-gray-900 dark:text-base-content truncate max-w-[180px]">
            {account.email}
          </span>
        </div>
        <span className={`text-xs px-2 py-0.5 rounded-full bg-${healthStatus.color}-100 dark:bg-${healthStatus.color}-900/20 text-${healthStatus.color}-700 dark:text-${healthStatus.color}-400`}>
          {healthStatus.label}
        </span>
      </div>

      {/* Quota bars */}
      <div className="space-y-2 mb-3">
        {primaryModels.map(model => {
          const usedPercentage = 100 - model.percentage;
          const barColor =
            model.percentage >= 50
              ? 'bg-green-500'
              : model.percentage >= 20
              ? 'bg-yellow-500'
              : 'bg-red-500';

          return (
            <div key={model.name}>
              <div className="flex items-center justify-between mb-1">
                <span className="text-xs text-gray-600 dark:text-gray-400 truncate max-w-[140px]">
                  {model.name}
                </span>
                <span className="text-xs font-medium text-gray-900 dark:text-base-content">
                  {model.percentage}%
                </span>
              </div>
              <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5">
                <div
                  className={`h-1.5 rounded-full ${barColor} transition-all duration-300`}
                  style={{ width: `${model.percentage}%` }}
                ></div>
              </div>
            </div>
          );
        })}
      </div>

      {/* Reset time */}
      {account.quota && account.quota.models.length > 0 && (
        <div className="flex items-center gap-1.5 text-xs text-gray-500 dark:text-gray-400">
          <Clock className="w-3 h-3" />
          <span>{t('quota.reset_at')}: {account.quota.models[0].reset_time}</span>
        </div>
      )}

      {/* View details button */}
      {onViewDetails && (
        <button
          onClick={() => onViewDetails(account.id)}
          className="mt-2 w-full text-xs text-blue-600 dark:text-blue-400 hover:underline"
        >
          {t('quota.view_details')}
        </button>
      )}
    </div>
  );
}

export default function QuotaMonitoringWidget() {
  const { t } = useTranslation();
  const { accounts, fetchAccounts } = useAccountStore();
  const { quotas, monitoring, checkQuotaThresholds, addSnapshot } = useQuotaStore();

  // Load accounts on mount
  useEffect(() => {
    fetchAccounts();
  }, [fetchAccounts]);

  // Check quota thresholds periodically
  useEffect(() => {
    if (!monitoring.enabled || accounts.length === 0) return;

    checkQuotaThresholds(accounts);

    const interval = setInterval(() => {
      checkQuotaThresholds(accounts);
    }, monitoring.checkIntervalMs);

    return () => clearInterval(interval);
  }, [accounts, monitoring.enabled, monitoring.checkIntervalMs, checkQuotaThresholds]);

  // Add quota snapshot periodically (every hour for historical tracking)
  useEffect(() => {
    if (accounts.length === 0) return;

    const createSnapshot = () => {
      accounts.forEach(account => {
        if (account.quota && !account.quota.is_forbidden) {
          const quotaMap: Record<string, number> = {};
          account.quota.models.forEach(model => {
            quotaMap[model.name] = model.percentage;
          });

          addSnapshot({
            timestamp: Date.now(),
            accountId: account.id,
            quotas: quotaMap,
          });
        }
      });
    };

    // Create initial snapshot
    createSnapshot();

    // Create snapshot every hour
    const interval = setInterval(createSnapshot, 60 * 60 * 1000);

    return () => clearInterval(interval);
  }, [accounts, addSnapshot]);

  // Calculate summary statistics
  const stats = useMemo(() => {
    const healthyCount = accounts.filter(acc => {
      if (!acc.quota || acc.quota.is_forbidden) return false;
      const minQuota = Math.min(...acc.quota.models.map(m => m.percentage));
      return minQuota >= 50;
    }).length;

    const warningCount = accounts.filter(acc => {
      if (!acc.quota || acc.quota.is_forbidden) return false;
      const minQuota = Math.min(...acc.quota.models.map(m => m.percentage));
      return minQuota >= 20 && minQuota < 50;
    }).length;

    const criticalCount = accounts.filter(acc => {
      if (!acc.quota || acc.quota.is_forbidden) return false;
      const minQuota = Math.min(...acc.quota.models.map(m => m.percentage));
      return minQuota < 20;
    }).length;

    const blockedCount = accounts.filter(
      acc => acc.quota?.is_forbidden || !acc.quota
    ).length;

    return { healthyCount, warningCount, criticalCount, blockedCount };
  }, [accounts]);

  return (
    <div className="space-y-4">
      {/* Summary Cards */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div className="bg-green-50 dark:bg-green-900/20 rounded-lg p-3 border border-green-200 dark:border-green-900/30">
          <div className="flex items-center gap-2 mb-1">
            <CheckCircle className="w-4 h-4 text-green-600 dark:text-green-400" />
            <span className="text-xs text-green-700 dark:text-green-300">{t('quota.healthy')}</span>
          </div>
          <div className="text-2xl font-bold text-green-900 dark:text-green-100">
            {stats.healthyCount}
          </div>
        </div>

        <div className="bg-yellow-50 dark:bg-yellow-900/20 rounded-lg p-3 border border-yellow-200 dark:border-yellow-900/30">
          <div className="flex items-center gap-2 mb-1">
            <Activity className="w-4 h-4 text-yellow-600 dark:text-yellow-400" />
            <span className="text-xs text-yellow-700 dark:text-yellow-300">{t('quota.warning')}</span>
          </div>
          <div className="text-2xl font-bold text-yellow-900 dark:text-yellow-100">
            {stats.warningCount}
          </div>
        </div>

        <div className="bg-red-50 dark:bg-red-900/20 rounded-lg p-3 border border-red-200 dark:border-red-900/30">
          <div className="flex items-center gap-2 mb-1">
            <AlertTriangle className="w-4 h-4 text-red-600 dark:text-red-400" />
            <span className="text-xs text-red-700 dark:text-red-300">{t('quota.critical')}</span>
          </div>
          <div className="text-2xl font-bold text-red-900 dark:text-red-100">
            {stats.criticalCount}
          </div>
        </div>

        <div className="bg-gray-50 dark:bg-gray-900/20 rounded-lg p-3 border border-gray-200 dark:border-gray-900/30">
          <div className="flex items-center gap-2 mb-1">
            <TrendingDown className="w-4 h-4 text-gray-600 dark:text-gray-400" />
            <span className="text-xs text-gray-700 dark:text-gray-300">{t('quota.blocked')}</span>
          </div>
          <div className="text-2xl font-bold text-gray-900 dark:text-gray-100">
            {stats.blockedCount}
          </div>
        </div>
      </div>

      {/* Account Cards Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {accounts.map(account => (
          <AccountQuotaCard key={account.id} account={account} />
        ))}
      </div>

      {accounts.length === 0 && (
        <div className="text-center py-12 text-gray-500 dark:text-gray-400">
          <Activity className="w-12 h-12 mx-auto mb-3 opacity-50" />
          <p>{t('quota.no_accounts')}</p>
        </div>
      )}
    </div>
  );
}
