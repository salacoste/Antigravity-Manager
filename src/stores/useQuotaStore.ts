import { create } from 'zustand';
import { Account } from '../types/account';

// Quota alert severity levels
export type AlertSeverity = 'info' | 'warning' | 'critical';

// Quota alert types
export interface QuotaAlert {
  id: string;
  accountId: string;
  accountEmail: string;
  modelName: string;
  threshold: number;
  currentPercentage: number;
  severity: AlertSeverity;
  timestamp: number;
  acknowledged: boolean;
  mitigationAction?: string;
}

// Account quota snapshot for historical tracking
export interface QuotaSnapshot {
  timestamp: number;
  accountId: string;
  quotas: {
    [modelName: string]: number; // percentage
  };
}

// Monitoring configuration
export interface MonitoringConfig {
  enabled: boolean;
  thresholds: number[]; // e.g., [80, 90, 95]
  checkIntervalMs: number;
  historyDays: number;
}

interface QuotaState {
  // Current quota data (derived from accounts)
  quotas: Record<string, Account['quota']>;

  // Alert management
  alerts: QuotaAlert[];
  unacknowledgedCount: number;

  // Historical data for charts
  history: QuotaSnapshot[];

  // Monitoring configuration
  monitoring: MonitoringConfig;

  // Actions
  setQuotas: (accountsMap: Record<string, Account>) => void;
  addAlert: (alert: QuotaAlert) => void;
  acknowledgeAlert: (alertId: string, mitigationAction?: string) => void;
  clearAlerts: () => void;
  addSnapshot: (snapshot: QuotaSnapshot) => void;
  setMonitoringConfig: (config: Partial<MonitoringConfig>) => void;
  checkQuotaThresholds: (accounts: Account[]) => void;
}

const DEFAULT_MONITORING_CONFIG: MonitoringConfig = {
  enabled: true,
  thresholds: [80, 90, 95],
  checkIntervalMs: 60000, // 1 minute
  historyDays: 7,
};

export const useQuotaStore = create<QuotaState>((set, get) => ({
  quotas: {},
  alerts: [],
  unacknowledgedCount: 0,
  history: [],
  monitoring: DEFAULT_MONITORING_CONFIG,

  setQuotas: (accountsMap) => {
    const quotas: Record<string, Account['quota']> = {};
    Object.values(accountsMap).forEach(account => {
      if (account.quota) {
        quotas[account.id] = account.quota;
      }
    });
    set({ quotas });
  },

  addAlert: (alert) => {
    const { alerts } = get();
    // Check if alert already exists for this account/model/threshold combo
    const exists = alerts.some(
      a =>
        a.accountId === alert.accountId &&
        a.modelName === alert.modelName &&
        a.threshold === alert.threshold &&
        !a.acknowledged
    );

    if (!exists) {
      const newAlerts = [...alerts, alert];
      const unacknowledgedCount = newAlerts.filter(a => !a.acknowledged).length;
      set({ alerts: newAlerts, unacknowledgedCount });
    }
  },

  acknowledgeAlert: (alertId, mitigationAction) => {
    const { alerts } = get();
    const updatedAlerts = alerts.map(alert =>
      alert.id === alertId
        ? { ...alert, acknowledged: true, mitigationAction }
        : alert
    );
    const unacknowledgedCount = updatedAlerts.filter(a => !a.acknowledged).length;
    set({ alerts: updatedAlerts, unacknowledgedCount });
  },

  clearAlerts: () => {
    set({ alerts: [], unacknowledgedCount: 0 });
  },

  addSnapshot: (snapshot) => {
    const { history, monitoring } = get();
    const cutoffTime = Date.now() - monitoring.historyDays * 24 * 60 * 60 * 1000;
    const filteredHistory = history.filter(s => s.timestamp >= cutoffTime);
    set({ history: [...filteredHistory, snapshot] });
  },

  setMonitoringConfig: (config) => {
    set({ monitoring: { ...get().monitoring, ...config } });
  },

  checkQuotaThresholds: (accounts) => {
    const { monitoring, addAlert } = get();
    if (!monitoring.enabled) return;

    accounts.forEach(account => {
      if (!account.quota || account.quota.is_forbidden) return;

      account.quota.models.forEach(model => {
        const usedPercentage = 100 - model.percentage;

        monitoring.thresholds.forEach(threshold => {
          if (usedPercentage >= threshold) {
            let severity: AlertSeverity = 'info';
            if (threshold >= 95) severity = 'critical';
            else if (threshold >= 90) severity = 'warning';

            addAlert({
              id: `${account.id}-${model.name}-${threshold}-${Date.now()}`,
              accountId: account.id,
              accountEmail: account.email,
              modelName: model.name,
              threshold,
              currentPercentage: usedPercentage,
              severity,
              timestamp: Date.now(),
              acknowledged: false,
            });
          }
        });
      });
    });
  },
}));
