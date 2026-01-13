import React from 'react';
import { useTranslation } from 'react-i18next';
import { CheckCircle, AlertTriangle, XCircle } from 'lucide-react';
import { QuotaInfoResponse } from '../../services/quotaManagerService';

interface QuotaStatusCardProps {
  accountId: string;
  modelId: string;
  quota: QuotaInfoResponse;
}

export const QuotaStatusCard: React.FC<QuotaStatusCardProps> = ({
  modelId,
  quota,
}) => {
  const { t } = useTranslation();

  // Calculate percentage and determine status
  const percentage = Math.round(quota.remaining_fraction * 100);
  const status = quota.status;

  // Format reset time
  const formatResetTime = (isoString: string): string => {
    const date = new Date(isoString);
    return new Intl.DateTimeFormat('en-US', {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  };

  // Status-based styling
  const getStatusConfig = () => {
    switch (status) {
      case 'healthy':
        return {
          icon: <CheckCircle className="w-4 h-4" />,
          iconColor: 'text-success',
          bgColor: 'bg-success/10',
          borderColor: 'border-success',
          progressColor: 'bg-success',
          textColor: 'text-success',
        };
      case 'low':
        return {
          icon: <AlertTriangle className="w-4 h-4" />,
          iconColor: 'text-warning',
          bgColor: 'bg-warning/10',
          borderColor: 'border-warning',
          progressColor: 'bg-warning',
          textColor: 'text-warning',
        };
      case 'exhausted':
        return {
          icon: <XCircle className="w-4 h-4" />,
          iconColor: 'text-error',
          bgColor: 'bg-error/10',
          borderColor: 'border-error',
          progressColor: 'bg-error',
          textColor: 'text-error',
        };
    }
  };

  const statusConfig = getStatusConfig();

  return (
    <div
      className={`bg-white dark:bg-base-100 rounded-xl p-4 shadow-sm border ${statusConfig.borderColor} dark:border-base-200`}
    >
      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <div className={`${statusConfig.iconColor}`}>{statusConfig.icon}</div>
          <h3 className="text-sm font-semibold text-gray-900 dark:text-base-content">
            {quota.display_name}
          </h3>
        </div>
        <span
          className={`text-xs font-bold ${statusConfig.textColor} px-2 py-1 rounded-md ${statusConfig.bgColor}`}
        >
          {percentage}%
        </span>
      </div>

      {/* Progress Bar */}
      <div className="mb-3">
        <div className="w-full bg-gray-100 dark:bg-base-300 rounded-full h-2 overflow-hidden">
          <div
            className={`h-full rounded-full transition-all duration-700 ${statusConfig.progressColor}`}
            style={{ width: `${percentage}%` }}
          ></div>
        </div>
      </div>

      {/* Details */}
      <div className="flex items-center justify-between text-xs">
        <span className="text-gray-500 dark:text-gray-400">
          {t('quota.remaining')}: {percentage}%
        </span>
        <span className="text-gray-400 dark:text-gray-500">
          {t('quota.resetTime')}: {formatResetTime(quota.reset_time)}
        </span>
      </div>

      {/* Status Messages */}
      {status === 'low' && (
        <div className="alert alert-warning mt-3 py-2 text-xs">
          <AlertTriangle className="w-3.5 h-3.5" />
          <span>{t('quota.warningMessage')}</span>
        </div>
      )}

      {status === 'exhausted' && (
        <div className="alert alert-error mt-3 py-2 text-xs">
          <XCircle className="w-3.5 h-3.5" />
          <span>{t('quota.errorMessage')}</span>
        </div>
      )}

      {/* Model ID (small footer) */}
      <div className="mt-2 pt-2 border-t border-gray-100 dark:border-base-200">
        <span className="text-[10px] text-gray-400 dark:text-gray-500 font-mono">
          {modelId}
        </span>
      </div>
    </div>
  );
};
