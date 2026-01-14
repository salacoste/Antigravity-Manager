/**
 * Detection Monitoring Dashboard Component (Story-024-04 Part 2)
 *
 * Displays detection statistics and recent security events from the backend.
 * Updates in real-time via Tauri events.
 */

import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { AlertTriangle, Shield, Activity, Clock } from 'lucide-react';

interface DetectionStatistics {
  total_events: number;
  events_by_type: Record<string, number>;
  events_by_severity: Record<string, number>;
  events_by_account: Record<string, number>;
  events_by_model: Record<string, number>;
  hourly_rate: number;
}

interface DetectionEvent {
  timestamp: string;
  event_type: string;
  severity: string;
  account_id: string;
  model_id: string;
  request_id: string;
  user_agent?: string;
  upstream_status?: number;
  context: string;
}

export const DetectionPanel: React.FC<{ className?: string }> = ({ className = '' }) => {
  const [stats, setStats] = useState<DetectionStatistics | null>(null);
  const [recentEvents, setRecentEvents] = useState<DetectionEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadData();

    // Listen for real-time detection alerts
    const unlistenPromise = listen('detection://alert', () => {
      loadData(); // Refresh on new alert
    });

    // Refresh every 30 seconds
    const interval = setInterval(loadData, 30000);

    return () => {
      unlistenPromise.then(fn => fn());
      clearInterval(interval);
    };
  }, []);

  const loadData = async () => {
    try {
      const [statistics, events] = await Promise.all([
        invoke<DetectionStatistics>('get_detection_statistics'),
        invoke<DetectionEvent[]>('get_recent_detection_events', { limit: 10 }),
      ]);
      setStats(statistics);
      setRecentEvents(events);
      setError(null);
    } catch (err: any) {
      console.error('Failed to load detection data:', err);
      setError(err.toString());
    } finally {
      setLoading(false);
    }
  };

  const getSeverityClass = (severity: string): string => {
    switch (severity.toUpperCase()) {
      case 'CRITICAL':
        return 'badge-error';
      case 'HIGH':
        return 'badge-warning';
      case 'MEDIUM':
        return 'badge-info';
      case 'LOW':
        return 'badge-success';
      default:
        return 'badge-ghost';
    }
  };

  const getSeverityIcon = (severity: string) => {
    switch (severity.toUpperCase()) {
      case 'CRITICAL':
      case 'HIGH':
        return <AlertTriangle size={16} />;
      case 'MEDIUM':
        return <Shield size={16} />;
      default:
        return <Activity size={16} />;
    }
  };

  if (loading && !stats) {
    return (
      <div className={`card bg-base-100 shadow-xl ${className}`}>
        <div className="card-body">
          <div className="flex items-center justify-center p-8">
            <span className="loading loading-spinner loading-lg"></span>
          </div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className={`alert alert-warning ${className}`}>
        <AlertTriangle size={20} />
        <span>
          <strong>Detection Monitor</strong>: {error.includes('not running') ? 'Proxy service must be running' : 'Failed to load data'}
        </span>
      </div>
    );
  }

  if (!stats) return null;

  return (
    <div className={`card bg-base-100 shadow-xl ${className}`}>
      <div className="card-body">
        <h2 className="card-title flex items-center gap-2">
          <Shield className="text-primary" size={24} />
          Detection Monitoring
          <span className="text-xs badge badge-neutral">{stats.total_events} events</span>
        </h2>

        {/* Statistics Cards */}
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 my-4">
          <div className="stat bg-base-200 rounded-lg p-4">
            <div className="stat-title text-xs">Total Events</div>
            <div className="stat-value text-2xl">{stats.total_events}</div>
          </div>
          <div className="stat bg-base-200 rounded-lg p-4">
            <div className="stat-title text-xs flex items-center gap-1">
              <Clock size={12} />
              Hourly Rate
            </div>
            <div className="stat-value text-2xl">{stats.hourly_rate.toFixed(1)}/hr</div>
          </div>
          <div className="stat bg-base-200 rounded-lg p-4">
            <div className="stat-title text-xs">Event Types</div>
            <div className="stat-value text-2xl">{Object.keys(stats.events_by_type).length}</div>
          </div>
          <div className="stat bg-base-200 rounded-lg p-4">
            <div className="stat-title text-xs">Accounts</div>
            <div className="stat-value text-2xl">{Object.keys(stats.events_by_account).length}</div>
          </div>
        </div>

        {/* Events by Severity */}
        <div className="mb-4">
          <h3 className="font-semibold mb-2 flex items-center gap-2">
            <AlertTriangle size={16} />
            By Severity
          </h3>
          <div className="space-y-2">
            {Object.entries(stats.events_by_severity)
              .sort(([, a], [, b]) => b - a)
              .map(([severity, count]) => (
                <div key={severity} className="flex justify-between items-center bg-base-200 rounded-lg p-2">
                  <div className="flex items-center gap-2">
                    {getSeverityIcon(severity)}
                    <span className={`badge badge-sm ${getSeverityClass(severity)}`}>
                      {severity}
                    </span>
                  </div>
                  <span className="font-mono font-semibold">{count}</span>
                </div>
              ))}
          </div>
        </div>

        {/* Recent Events */}
        <div>
          <h3 className="font-semibold mb-2">Recent Events (Last 10)</h3>
          {recentEvents.length === 0 ? (
            <div className="text-center text-sm text-base-content/60 py-4">
              No detection events recorded yet
            </div>
          ) : (
            <div className="space-y-2 max-h-96 overflow-y-auto">
              {recentEvents.map((event, idx) => (
                <div key={idx} className="card bg-base-300 p-3">
                  <div className="flex justify-between items-start mb-2">
                    <div className="flex items-center gap-2">
                      <span className={`badge badge-sm ${getSeverityClass(event.severity)}`}>
                        {event.severity}
                      </span>
                      <span className="font-semibold text-sm">{event.event_type}</span>
                    </div>
                    <span className="text-xs opacity-60">
                      {new Date(event.timestamp).toLocaleString()}
                    </span>
                  </div>
                  <p className="text-sm mb-2">{event.context}</p>
                  <div className="text-xs opacity-60">
                    <span>Account: {event.account_id}</span>
                    <span className="mx-2">•</span>
                    <span>Model: {event.model_id}</span>
                    {event.upstream_status && (
                      <>
                        <span className="mx-2">•</span>
                        <span>Status: {event.upstream_status}</span>
                      </>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
