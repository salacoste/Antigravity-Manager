// Epic-025 Story-025-04: Thinking Quality Monitoring Dashboard (Week 5 - Foundation)

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import {
  Activity,
  TrendingUp,
  AlertCircle,
  CheckCircle2,
  Target,
  Zap,
} from 'lucide-react';

interface QualityMetrics {
  total_requests: number;
  first_time_right: number;
  escalations_needed: number;
  average_quality_score: number;
  average_efficiency: number;
  average_completeness: number;
  average_coherence: number;
  average_budget_utilization: number;
  last_updated: number | null;
}

interface WeeklyFeedback {
  period_start: string;
  period_end: string;
  total_requests: number;
  avg_quality_score: number;
  first_time_right_rate: number;
  avg_budget_utilization: number;
  avg_efficiency: number;
  avg_completeness: number;
  avg_coherence: number;
  recommendations: string[];
  tuning_priority: 'Critical' | 'High' | 'Medium' | 'Low';
}

const QualityDashboardPage = () => {
  const { t } = useTranslation();
  const [metrics, setMetrics] = useState<QualityMetrics | null>(null);
  const [weeklyFeedback, setWeeklyFeedback] = useState<WeeklyFeedback | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadData();
    // Refresh every 30 seconds
    const interval = setInterval(loadData, 30000);
    return () => clearInterval(interval);
  }, []);

  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      const [metricsData, feedbackData] = await Promise.all([
        invoke<QualityMetrics>('get_quality_metrics'),
        invoke<WeeklyFeedback>('get_weekly_feedback', { days: 7 }),
      ]);
      setMetrics(metricsData);
      setWeeklyFeedback(feedbackData);
    } catch (err) {
      setError(err as string);
      console.error('Failed to load quality data:', err);
    } finally {
      setLoading(false);
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'Critical':
        return 'error';
      case 'High':
        return 'warning';
      case 'Medium':
        return 'info';
      default:
        return 'success';
    }
  };

  const getScoreColor = (score: number) => {
    if (score >= 0.9) return 'text-success';
    if (score >= 0.75) return 'text-warning';
    return 'text-error';
  };

  const formatPercentage = (value: number) => {
    return `${(value * 100).toFixed(1)}%`;
  };

  if (loading && !metrics) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <AlertCircle className="h-6 w-6" />
        <span>Failed to load quality data: {error}</span>
      </div>
    );
  }

  const ftrRate = metrics
    ? metrics.total_requests > 0
      ? metrics.first_time_right / metrics.total_requests
      : 0
    : 0;

  return (
    <div className="container mx-auto p-6 space-y-6">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold flex items-center gap-3">
            <Activity className="h-8 w-8" />
            {t('quality.title', 'Thinking Quality Monitoring')}
          </h1>
          <p className="text-base-content/70 mt-2">
            {t('quality.subtitle', 'Track thinking quality metrics to optimize budget classifier')}
          </p>
        </div>
        <button className="btn btn-primary" onClick={loadData}>
          <TrendingUp className="h-5 w-5" />
          {t('quality.refresh', 'Refresh')}
        </button>
      </div>

      {/* Quality Score Gauges */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {/* Overall Quality */}
        <div className="card bg-base-200 shadow-lg">
          <div className="card-body">
            <h3 className="card-title text-sm">
              <Target className="h-5 w-5" />
              {t('quality.overall_score', 'Overall Quality')}
            </h3>
            <div className={`text-3xl font-bold ${getScoreColor(metrics?.average_quality_score || 0)}`}>
              {formatPercentage(metrics?.average_quality_score || 0)}
            </div>
            <p className="text-xs text-base-content/60">
              {t('quality.overall_desc', 'Weighted average of all dimensions')}
            </p>
          </div>
        </div>

        {/* Efficiency */}
        <div className="card bg-base-200 shadow-lg">
          <div className="card-body">
            <h3 className="card-title text-sm">
              <Zap className="h-5 w-5" />
              {t('quality.efficiency', 'Efficiency')}
            </h3>
            <div className={`text-3xl font-bold ${getScoreColor(metrics?.average_efficiency || 0)}`}>
              {formatPercentage(metrics?.average_efficiency || 0)}
            </div>
            <p className="text-xs text-base-content/60">
              {t('quality.efficiency_desc', 'Budget utilization: 75-95% optimal')}
            </p>
          </div>
        </div>

        {/* Completeness */}
        <div className="card bg-base-200 shadow-lg">
          <div className="card-body">
            <h3 className="card-title text-sm">
              <CheckCircle2 className="h-5 w-5" />
              {t('quality.completeness', 'Completeness')}
            </h3>
            <div className={`text-3xl font-bold ${getScoreColor(metrics?.average_completeness || 0)}`}>
              {formatPercentage(metrics?.average_completeness || 0)}
            </div>
            <p className="text-xs text-base-content/60">
              {t('quality.completeness_desc', 'Responses finished naturally')}
            </p>
          </div>
        </div>

        {/* Coherence */}
        <div className="card bg-base-200 shadow-lg">
          <div className="card-body">
            <h3 className="card-title text-sm">
              <Activity className="h-5 w-5" />
              {t('quality.coherence', 'Coherence')}
            </h3>
            <div className={`text-3xl font-bold ${getScoreColor(metrics?.average_coherence || 0)}`}>
              {formatPercentage(metrics?.average_coherence || 0)}
            </div>
            <p className="text-xs text-base-content/60">
              {t('quality.coherence_desc', 'Thinking/output balance')}
            </p>
          </div>
        </div>
      </div>

      {/* First-Time-Right Rate */}
      <div className="card bg-base-100 shadow-lg">
        <div className="card-body">
          <h2 className="card-title">
            <CheckCircle2 className="h-6 w-6" />
            {t('quality.ftr_title', 'First-Time-Right Rate')}
          </h2>
          <div className="stats stats-horizontal shadow">
            <div className="stat">
              <div className="stat-title">{t('quality.ftr_rate', 'FTR Rate')}</div>
              <div className={`stat-value ${getScoreColor(ftrRate)}`}>{formatPercentage(ftrRate)}</div>
              <div className="stat-desc">{t('quality.ftr_target', 'Target: >90%')}</div>
            </div>
            <div className="stat">
              <div className="stat-title">{t('quality.total_requests', 'Total Requests')}</div>
              <div className="stat-value text-primary">{metrics?.total_requests || 0}</div>
              <div className="stat-desc">{t('quality.since_start', 'Since monitoring started')}</div>
            </div>
            <div className="stat">
              <div className="stat-title">{t('quality.escalations', 'Escalations')}</div>
              <div className="stat-value text-warning">{metrics?.escalations_needed || 0}</div>
              <div className="stat-desc">{t('quality.needed_retry', 'Needed higher budget')}</div>
            </div>
          </div>
        </div>
      </div>

      {/* Budget Utilization */}
      <div className="card bg-base-100 shadow-lg">
        <div className="card-body">
          <h2 className="card-title">
            <TrendingUp className="h-6 w-6" />
            {t('quality.budget_util_title', 'Budget Utilization')}
          </h2>
          <div className="flex items-center gap-4">
            <div className="flex-1">
              <div className="text-sm text-base-content/70 mb-2">
                {t('quality.avg_utilization', 'Average Utilization')}
              </div>
              <progress
                className="progress progress-primary w-full h-6"
                value={metrics?.average_budget_utilization || 0}
                max={1}
              ></progress>
              <div className="text-right text-sm mt-1">
                {formatPercentage(metrics?.average_budget_utilization || 0)}
              </div>
            </div>
            <div className="text-sm">
              <div className="badge badge-success mb-1">75-95%</div>
              <div className="text-xs text-base-content/60">
                {t('quality.optimal_range', 'Optimal Range')}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Weekly Recommendations */}
      {weeklyFeedback && (
        <div className="card bg-base-100 shadow-lg">
          <div className="card-body">
            <h2 className="card-title">
              <AlertCircle className="h-6 w-6" />
              {t('quality.recommendations_title', 'Weekly Tuning Recommendations')}
              <span className={`badge badge-${getPriorityColor(weeklyFeedback.tuning_priority)}`}>
                {weeklyFeedback.tuning_priority}
              </span>
            </h2>
            <div className="space-y-2">
              {weeklyFeedback.recommendations.map((rec, idx) => (
                <div
                  key={idx}
                  className={`alert ${
                    rec.includes('CRITICAL')
                      ? 'alert-error'
                      : rec.includes('⚠️')
                      ? 'alert-warning'
                      : rec.includes('✅')
                      ? 'alert-success'
                      : 'alert-info'
                  }`}
                >
                  <span>{rec}</span>
                </div>
              ))}
            </div>
            <div className="text-sm text-base-content/60 mt-4">
              {t('quality.period', 'Analysis period')}: {new Date(weeklyFeedback.period_start).toLocaleDateString()} -{' '}
              {new Date(weeklyFeedback.period_end).toLocaleDateString()} ({weeklyFeedback.total_requests}{' '}
              {t('quality.requests', 'requests')})
            </div>
          </div>
        </div>
      )}

      {/* Footer Info */}
      <div className="text-center text-sm text-base-content/60">
        {metrics?.last_updated && (
          <p>
            {t('quality.last_updated', 'Last updated')}: {new Date(metrics.last_updated * 1000).toLocaleString()}
          </p>
        )}
      </div>
    </div>
  );
};

export default QualityDashboardPage;
