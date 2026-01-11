import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { ArrowLeft, TrendingUp } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { ProxyMonitor } from '../components/proxy/ProxyMonitor';
import { ComplianceMetrics } from '../components/proxy/ComplianceMetrics';

const Monitor: React.FC = () => {
    const navigate = useNavigate();
    const { t } = useTranslation();
    const [showCompliance, setShowCompliance] = useState(true);

    return (
        <div className="h-full w-full flex flex-col bg-gray-50 dark:bg-base-200">
            {/* Header */}
            <div className="bg-white dark:bg-base-100 border-b border-gray-200 dark:border-base-300 px-4 py-3 flex items-center justify-between shadow-sm z-10">
                <div className="flex items-center gap-4">
                    <button
                        onClick={() => navigate('/api-proxy')}
                        className="btn btn-ghost btn-sm btn-circle"
                        title={t('common.back')}
                    >
                        <ArrowLeft size={20} />
                    </button>
                    <div>
                        <h1 className="text-lg font-bold text-gray-900 dark:text-base-content">
                            {t('monitor.page_title')}
                        </h1>
                        <p className="text-xs text-gray-500 dark:text-gray-400">
                            {t('monitor.page_subtitle')}
                        </p>
                    </div>
                </div>

                {/* Compliance Toggle */}
                <button
                    onClick={() => setShowCompliance(!showCompliance)}
                    className={`btn btn-sm ${showCompliance ? 'btn-primary' : 'btn-ghost'}`}
                    title={showCompliance ? t('monitor.hide_compliance') : t('monitor.show_compliance')}
                >
                    <TrendingUp size={16} />
                    <span className="hidden sm:inline">{t('monitor.compliance')}</span>
                </button>
            </div>

            {/* Main Content */}
            <div className="flex-1 p-4 overflow-auto">
                <div className="flex flex-col gap-4">
                    {/* Compliance Dashboard */}
                    {showCompliance && (
                        <ComplianceMetrics />
                    )}

                    {/* Request Monitor */}
                    <ProxyMonitor className="h-[600px] border border-gray-200 dark:border-base-300 shadow-md" />
                </div>
            </div>
        </div>
    );
};

export default Monitor;