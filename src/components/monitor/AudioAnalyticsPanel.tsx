/**
 * Audio Analytics Dashboard Component (Epic-014 Story-014-04)
 *
 * Displays audio transcription analytics including duration stats, format distribution,
 * and file size metrics for operational insights.
 */

import React, { useEffect, useState } from 'react';
import { request as invoke } from '../../utils/request';
import { useTranslation } from 'react-i18next';
import { Music, FileAudio, Clock, HardDrive, RefreshCw, TrendingUp } from 'lucide-react';

interface DurationStats {
  min_secs: number;
  max_secs: number;
  avg_secs: number;
  p50_secs: number;
  p95_secs: number;
  p99_secs: number;
}

interface FormatStats {
  format: string;
  count: number;
  percentage: number;
}

interface FileSizeDistribution {
  under_1mb: number;
  mb_1_to_5: number;
  mb_5_to_10: number;
  mb_10_to_15: number;
  over_15mb: number;
  avg_size_mb: number;
  p95_size_mb: number;
}

interface AudioAnalytics {
  duration_stats: DurationStats;
  format_distribution: FormatStats[];
  file_size_distribution: FileSizeDistribution;
  total_requests: number;
  success_rate: number;
  total_audio_hours: number;
}

interface AudioAnalyticsPanelProps {
  className?: string;
  days?: number; // Default 30 days
}

export const AudioAnalyticsPanel: React.FC<AudioAnalyticsPanelProps> = ({
  className = '',
  days = 30,
}) => {
  const { t } = useTranslation();
  const [analytics, setAnalytics] = useState<AudioAnalytics | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedDays, setSelectedDays] = useState(days);

  useEffect(() => {
    loadAnalytics();
  }, [selectedDays]);

  const loadAnalytics = async () => {
    try {
      setLoading(true);
      const data = await invoke<AudioAnalytics>('get_audio_analytics', {
        days: selectedDays,
      });
      setAnalytics(data);
    } catch (e) {
      console.error('Failed to load audio analytics:', e);
    } finally {
      setLoading(false);
    }
  };

  const formatDuration = (seconds: number): string => {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m ${seconds % 60}s`;
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    return `${hours}h ${mins}m`;
  };

  const formatFileSize = (mb: number): string => {
    return `${mb.toFixed(1)} MB`;
  };

  if (loading) {
    return (
      <div className={`bg-white dark:bg-base-100 rounded-lg shadow-md p-6 ${className}`}>
        <div className="flex items-center justify-center">
          <span className="loading loading-spinner loading-lg"></span>
        </div>
      </div>
    );
  }

  if (!analytics || analytics.total_requests === 0) {
    return (
      <div className={`bg-white dark:bg-base-100 rounded-lg shadow-md p-6 ${className}`}>
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center gap-2">
            <Music size={20} className="text-primary" />
            <h2 className="text-lg font-bold">Audio Transcription Analytics</h2>
          </div>
        </div>
        <p className="text-gray-500 dark:text-gray-400 text-center py-8">
          No audio transcription data available for the last {selectedDays} days
        </p>
      </div>
    );
  }

  return (
    <div className={`bg-white dark:bg-base-100 rounded-lg shadow-md p-6 ${className}`}>
      {/* Header */}
      <div className="flex items-center justify-between mb-6">
        <div className="flex items-center gap-2">
          <Music size={20} className="text-primary" />
          <h2 className="text-lg font-bold">Audio Transcription Analytics</h2>
        </div>
        <div className="flex items-center gap-2">
          {/* Time Period Selector */}
          <select
            className="select select-sm select-bordered"
            value={selectedDays}
            onChange={(e) => setSelectedDays(Number(e.target.value))}
          >
            <option value={7}>Last 7 Days</option>
            <option value={30}>Last 30 Days</option>
            <option value={90}>Last 90 Days</option>
          </select>
          <button
            onClick={loadAnalytics}
            className="btn btn-sm btn-ghost btn-circle"
            title="Refresh"
          >
            <RefreshCw size={16} />
          </button>
        </div>
      </div>

      {/* Overview Stats */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        <div className="stat bg-base-200 rounded-lg">
          <div className="stat-figure text-primary">
            <FileAudio size={32} />
          </div>
          <div className="stat-title">Total Requests</div>
          <div className="stat-value text-primary">{analytics.total_requests}</div>
          <div className="stat-desc">Success Rate: {analytics.success_rate.toFixed(1)}%</div>
        </div>

        <div className="stat bg-base-200 rounded-lg">
          <div className="stat-figure text-secondary">
            <Clock size={32} />
          </div>
          <div className="stat-title">Total Audio</div>
          <div className="stat-value text-secondary">
            {analytics.total_audio_hours.toFixed(1)}h
          </div>
          <div className="stat-desc">
            Avg: {formatDuration(analytics.duration_stats.avg_secs)}
          </div>
        </div>

        <div className="stat bg-base-200 rounded-lg">
          <div className="stat-figure text-accent">
            <TrendingUp size={32} />
          </div>
          <div className="stat-title">P95 Duration</div>
          <div className="stat-value text-accent">
            {formatDuration(analytics.duration_stats.p95_secs)}
          </div>
          <div className="stat-desc">Median: {formatDuration(analytics.duration_stats.p50_secs)}</div>
        </div>

        <div className="stat bg-base-200 rounded-lg">
          <div className="stat-figure text-info">
            <HardDrive size={32} />
          </div>
          <div className="stat-title">Avg File Size</div>
          <div className="stat-value text-info">
            {formatFileSize(analytics.file_size_distribution.avg_size_mb)}
          </div>
          <div className="stat-desc">
            P95: {formatFileSize(analytics.file_size_distribution.p95_size_mb)}
          </div>
        </div>
      </div>

      {/* Format Distribution */}
      <div className="mb-6">
        <h3 className="text-md font-semibold mb-3">Format Distribution</h3>
        <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-3">
          {analytics.format_distribution.map((format) => (
            <div
              key={format.format}
              className="bg-base-200 rounded-lg p-3 text-center"
            >
              <div className="text-2xl font-bold text-primary">{format.count}</div>
              <div className="text-xs text-gray-500 uppercase">{format.format}</div>
              <div className="text-xs text-gray-400">{format.percentage.toFixed(1)}%</div>
            </div>
          ))}
        </div>
      </div>

      {/* File Size Distribution */}
      <div>
        <h3 className="text-md font-semibold mb-3">File Size Distribution</h3>
        <div className="grid grid-cols-2 md:grid-cols-5 gap-3">
          <div className="bg-base-200 rounded-lg p-3 text-center">
            <div className="text-2xl font-bold text-success">
              {analytics.file_size_distribution.under_1mb}
            </div>
            <div className="text-xs text-gray-500">&lt; 1 MB</div>
          </div>
          <div className="bg-base-200 rounded-lg p-3 text-center">
            <div className="text-2xl font-bold text-info">
              {analytics.file_size_distribution.mb_1_to_5}
            </div>
            <div className="text-xs text-gray-500">1-5 MB</div>
          </div>
          <div className="bg-base-200 rounded-lg p-3 text-center">
            <div className="text-2xl font-bold text-warning">
              {analytics.file_size_distribution.mb_5_to_10}
            </div>
            <div className="text-xs text-gray-500">5-10 MB</div>
          </div>
          <div className="bg-base-200 rounded-lg p-3 text-center">
            <div className="text-2xl font-bold text-warning">
              {analytics.file_size_distribution.mb_10_to_15}
            </div>
            <div className="text-xs text-gray-500">10-15 MB</div>
          </div>
          <div className="bg-base-200 rounded-lg p-3 text-center">
            <div className="text-2xl font-bold text-error">
              {analytics.file_size_distribution.over_15mb}
            </div>
            <div className="text-xs text-gray-500">&gt; 15 MB</div>
          </div>
        </div>
      </div>

      {/* Duration Stats Footer */}
      <div className="mt-6 p-4 bg-base-200 rounded-lg">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
          <div>
            <div className="text-xs text-gray-500">Min Duration</div>
            <div className="text-sm font-semibold">
              {formatDuration(analytics.duration_stats.min_secs)}
            </div>
          </div>
          <div>
            <div className="text-xs text-gray-500">Max Duration</div>
            <div className="text-sm font-semibold">
              {formatDuration(analytics.duration_stats.max_secs)}
            </div>
          </div>
          <div>
            <div className="text-xs text-gray-500">P50 (Median)</div>
            <div className="text-sm font-semibold">
              {formatDuration(analytics.duration_stats.p50_secs)}
            </div>
          </div>
          <div>
            <div className="text-xs text-gray-500">P99</div>
            <div className="text-sm font-semibold">
              {formatDuration(analytics.duration_stats.p99_secs)}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
