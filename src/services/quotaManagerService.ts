/**
 * QuotaManager Service
 *
 * Service layer for QuotaManager Dashboard Integration (Epic-001 Phase 3)
 * Provides typed wrappers around Tauri commands for quota monitoring.
 */

import { invoke } from '@tauri-apps/api/core';

// Type Definitions

export interface QuotaInfoResponse {
  remaining_fraction: number;
  reset_time: string; // ISO 8601
  display_name: string;
  status: 'healthy' | 'low' | 'exhausted';
}

export interface QuotaStatus {
  account_id: string;
  model_quotas: Record<string, QuotaInfoResponse>;
}

export interface TierInfo {
  account_id: string;
  tier: 'FREE' | 'PRO' | 'ULTRA';
}

export interface MonitorStats {
  total_accounts: number;
  cached_quotas: number;
  expired_entries: number;
  exhausted_quotas: number;
  last_sync_duration_ms: number;
  sync_success_count: number;
  sync_error_count: number;
}

// Service Functions

/**
 * Get quotas for all models for a specific account
 *
 * @param accountId - Account ID (email)
 * @param accessToken - Valid OAuth2 access token
 * @param projectId - GCP project ID
 * @returns QuotaStatus with model quotas and their statuses
 */
export async function getAccountQuotas(
  accountId: string,
  accessToken: string,
  projectId: string
): Promise<QuotaStatus> {
  return invoke<QuotaStatus>('get_account_quotas', {
    accountId,
    accessToken,
    projectId,
  });
}

/**
 * Get subscription tier for an account
 *
 * @param accountId - Account ID (email)
 * @param accessToken - Valid OAuth2 access token
 * @returns TierInfo with account tier (FREE/PRO/ULTRA)
 */
export async function getAccountTier(
  accountId: string,
  accessToken: string
): Promise<TierInfo> {
  return invoke<TierInfo>('get_account_tier', {
    accountId,
    accessToken,
  });
}

/**
 * Get quota monitor statistics
 *
 * @returns MonitorStats with cache metrics and sync statistics
 */
export async function getQuotaManagerStats(): Promise<MonitorStats> {
  return invoke<MonitorStats>('get_quota_manager_stats');
}

/**
 * Clear tier cache for an account (force refresh)
 *
 * @param accountId - Account ID (email)
 */
export async function clearTierCache(accountId: string): Promise<void> {
  return invoke<void>('clear_tier_cache', { accountId });
}
