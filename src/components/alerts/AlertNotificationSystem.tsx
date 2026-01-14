import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { AlertTriangle, Bell, CheckCircle, Info, XCircle } from 'lucide-react';
import { useQuotaStore, QuotaAlert, AlertSeverity } from '../../stores/useQuotaStore';
import { formatDistanceToNow } from 'date-fns';

interface AlertRowProps {
  alert: QuotaAlert;
  onAcknowledge: (alertId: string, action: string) => void;
}

function AlertRow({ alert, onAcknowledge }: AlertRowProps) {
  const { t } = useTranslation();
  const [showActionInput, setShowActionInput] = useState(false);
  const [action, setAction] = useState('');

  const severityConfig = useMemo(() => {
    const configs: Record<AlertSeverity, { icon: React.ReactNode; color: string; bg: string }> = {
      info: {
        icon: <Info className="w-4 h-4" />,
        color: 'text-blue-600 dark:text-blue-400',
        bg: 'bg-blue-50 dark:bg-blue-900/20',
      },
      warning: {
        icon: <AlertTriangle className="w-4 h-4" />,
        color: 'text-yellow-600 dark:text-yellow-400',
        bg: 'bg-yellow-50 dark:bg-yellow-900/20',
      },
      critical: {
        icon: <XCircle className="w-4 h-4" />,
        color: 'text-red-600 dark:text-red-400',
        bg: 'bg-red-50 dark:bg-red-900/20',
      },
    };
    return configs[alert.severity];
  }, [alert.severity]);

  const handleAcknowledge = () => {
    if (showActionInput && action.trim()) {
      onAcknowledge(alert.id, action.trim());
      setShowActionInput(false);
      setAction('');
    } else {
      setShowActionInput(true);
    }
  };

  return (
    <tr className={`${alert.acknowledged ? 'opacity-60' : ''} hover:bg-gray-50 dark:hover:bg-base-200`}>
      {/* Severity */}
      <td className="px-4 py-3">
        <div className={`flex items-center gap-2 ${severityConfig.color}`}>
          {severityConfig.icon}
          <span className="text-xs font-medium capitalize">{alert.severity}</span>
        </div>
      </td>

      {/* Account */}
      <td className="px-4 py-3">
        <div className="text-sm text-gray-900 dark:text-base-content truncate max-w-[200px]">
          {alert.accountEmail}
        </div>
      </td>

      {/* Model */}
      <td className="px-4 py-3">
        <div className="text-sm text-gray-700 dark:text-gray-300 truncate max-w-[150px]">
          {alert.modelName}
        </div>
      </td>

      {/* Threshold */}
      <td className="px-4 py-3">
        <span className="text-sm font-medium text-gray-900 dark:text-base-content">
          {alert.threshold}%
        </span>
      </td>

      {/* Current Usage */}
      <td className="px-4 py-3">
        <div className="flex items-center gap-2">
          <div className="w-24 bg-gray-200 dark:bg-gray-700 rounded-full h-2">
            <div
              className={`h-2 rounded-full ${
                alert.currentPercentage >= 95
                  ? 'bg-red-500'
                  : alert.currentPercentage >= 90
                  ? 'bg-yellow-500'
                  : 'bg-blue-500'
              }`}
              style={{ width: `${Math.min(alert.currentPercentage, 100)}%` }}
            ></div>
          </div>
          <span className="text-sm font-medium text-gray-900 dark:text-base-content">
            {alert.currentPercentage.toFixed(1)}%
          </span>
        </div>
      </td>

      {/* Time */}
      <td className="px-4 py-3">
        <span className="text-xs text-gray-500 dark:text-gray-400">
          {formatDistanceToNow(alert.timestamp, { addSuffix: true })}
        </span>
      </td>

      {/* Action */}
      <td className="px-4 py-3">
        {alert.acknowledged ? (
          <div className="flex items-center gap-2">
            <CheckCircle className="w-4 h-4 text-green-500" />
            {alert.mitigationAction && (
              <span className="text-xs text-gray-600 dark:text-gray-400 truncate max-w-[150px]" title={alert.mitigationAction}>
                {alert.mitigationAction}
              </span>
            )}
          </div>
        ) : showActionInput ? (
          <div className="flex items-center gap-2">
            <input
              type="text"
              value={action}
              onChange={(e) => setAction(e.target.value)}
              placeholder={t('alerts.action_placeholder')}
              className="input input-xs input-bordered w-32"
              autoFocus
            />
            <button
              onClick={handleAcknowledge}
              disabled={!action.trim()}
              className="btn btn-xs btn-success"
            >
              {t('common.confirm')}
            </button>
            <button
              onClick={() => {
                setShowActionInput(false);
                setAction('');
              }}
              className="btn btn-xs btn-ghost"
            >
              {t('common.cancel')}
            </button>
          </div>
        ) : (
          <button
            onClick={handleAcknowledge}
            className="btn btn-xs btn-outline btn-primary"
          >
            {t('alerts.acknowledge')}
          </button>
        )}
      </td>
    </tr>
  );
}

export default function AlertNotificationSystem() {
  const { t } = useTranslation();
  const { alerts, unacknowledgedCount, acknowledgeAlert, clearAlerts } = useQuotaStore();
  const [filter, setFilter] = useState<'all' | 'unacknowledged'>('unacknowledged');

  const filteredAlerts = useMemo(() => {
    let filtered = [...alerts];

    if (filter === 'unacknowledged') {
      filtered = filtered.filter(a => !a.acknowledged);
    }

    // Sort by timestamp (newest first)
    return filtered.sort((a, b) => b.timestamp - a.timestamp);
  }, [alerts, filter]);

  const handleAcknowledge = (alertId: string, action: string) => {
    acknowledgeAlert(alertId, action);
  };

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="p-2 bg-orange-100 dark:bg-orange-900/20 rounded-lg">
            <Bell className="w-5 h-5 text-orange-600 dark:text-orange-400" />
          </div>
          <div>
            <h2 className="text-lg font-semibold text-gray-900 dark:text-base-content">
              {t('alerts.title')}
            </h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              {t('alerts.subtitle', { count: unacknowledgedCount })}
            </p>
          </div>
        </div>

        <div className="flex items-center gap-2">
          {/* Filter */}
          <div className="flex gap-1 bg-gray-100 dark:bg-base-200 rounded-lg p-1">
            <button
              onClick={() => setFilter('unacknowledged')}
              className={`px-3 py-1 text-xs rounded-md transition-colors ${
                filter === 'unacknowledged'
                  ? 'bg-white dark:bg-base-100 text-gray-900 dark:text-base-content shadow-sm'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-base-content'
              }`}
            >
              {t('alerts.filter.unacknowledged')} ({unacknowledgedCount})
            </button>
            <button
              onClick={() => setFilter('all')}
              className={`px-3 py-1 text-xs rounded-md transition-colors ${
                filter === 'all'
                  ? 'bg-white dark:bg-base-100 text-gray-900 dark:text-base-content shadow-sm'
                  : 'text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-base-content'
              }`}
            >
              {t('alerts.filter.all')} ({alerts.length})
            </button>
          </div>

          {/* Clear all */}
          {alerts.length > 0 && (
            <button
              onClick={clearAlerts}
              className="btn btn-sm btn-ghost text-gray-600 dark:text-gray-400"
            >
              {t('alerts.clear_all')}
            </button>
          )}
        </div>
      </div>

      {/* Alerts Table */}
      <div className="bg-white dark:bg-base-100 rounded-lg shadow-sm border border-gray-200 dark:border-base-200 overflow-hidden">
        {filteredAlerts.length > 0 ? (
          <div className="overflow-x-auto">
            <table className="table table-sm w-full">
              <thead className="bg-gray-50 dark:bg-base-200">
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.severity')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.account')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.model')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.threshold')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.usage')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.time')}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium text-gray-700 dark:text-gray-300">
                    {t('alerts.table.action')}
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 dark:divide-base-200">
                {filteredAlerts.map(alert => (
                  <AlertRow
                    key={alert.id}
                    alert={alert}
                    onAcknowledge={handleAcknowledge}
                  />
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <div className="text-center py-12 text-gray-500 dark:text-gray-400">
            <Bell className="w-12 h-12 mx-auto mb-3 opacity-50" />
            <p>
              {filter === 'all'
                ? t('alerts.no_alerts')
                : t('alerts.no_unacknowledged_alerts')}
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
