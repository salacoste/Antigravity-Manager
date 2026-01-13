import React, { useEffect, useState } from 'react';
import { X, Download, Sparkles, ArrowRight } from 'lucide-react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

interface UpdateInfo {
  has_update: boolean;
  latest_version: string;
  current_version: string;
  download_url: string;
}

interface UpdateNotificationProps {
  onClose: () => void;
}

export const UpdateNotification: React.FC<UpdateNotificationProps> = ({ onClose }) => {
  const { t } = useTranslation();
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [isVisible, setIsVisible] = useState(false);
  const [isClosing, setIsClosing] = useState(false);

  useEffect(() => {
    checkForUpdates();
  }, []);

  const checkForUpdates = async () => {
    try {
      const info = await invoke<UpdateInfo>('check_for_updates');
      if (info.has_update) {
        setUpdateInfo(info);
        // Small delay to ensure smooth entry animation
        setTimeout(() => setIsVisible(true), 100);
      } else {
        onClose();
      }
    } catch (error) {
      console.error('Failed to check for updates:', error);
      onClose();
    }
  };

  const handleDownload = () => {
    if (updateInfo?.download_url) {
      window.open(updateInfo.download_url, '_blank');
      handleClose();
    }
  };

  const handleClose = () => {
    setIsClosing(true);
    setIsVisible(false);
    setTimeout(onClose, 400); // Wait for exit animation
  };

  if (!updateInfo) {
    return null;
  }

  return (
    <div
      className={`
        fixed top-6 right-6 z-[100]
        transition-all duration-500 ease-[cubic-bezier(0.34,1.56,0.64,1)]
        ${isVisible && !isClosing ? 'translate-y-0 opacity-100 scale-100' : '-translate-y-4 opacity-0 scale-95'}
      `}
    >
      {/* 
        Glassmorphism Container 
        - Backdrop blur
        - Translucent background with subtle gradient
        - High-quality border simulation
      */}
      <div className="
        relative overflow-hidden
        w-80 p-5
        rounded-2xl
        border border-white/20 dark:border-white/10
        shadow-[0_8px_32px_0_rgba(31,38,135,0.15)]
        backdrop-blur-xl
        bg-white/70 dark:bg-slate-900/60
        group
      ">
        {/* Decorative background gradient blobs */}
        <div className="absolute -top-10 -right-10 w-32 h-32 bg-blue-500/20 rounded-full blur-3xl pointer-events-none group-hover:bg-blue-500/30 transition-colors duration-500"></div>
        <div className="absolute -bottom-10 -left-10 w-32 h-32 bg-purple-500/20 rounded-full blur-3xl pointer-events-none group-hover:bg-purple-500/30 transition-colors duration-500"></div>

        {/* Content */}
        <div className="relative z-10">
          <div className="flex items-start justify-between mb-3">
            <div className="flex items-center gap-2">
              <div className="p-1.5 rounded-lg bg-gradient-to-br from-blue-500 to-purple-600 shadow-sm">
                <Sparkles className="w-4 h-4 text-white" />
              </div>
              <div>
                <h3 className="font-bold text-gray-800 dark:text-white leading-tight">{t('update_notification.title')}</h3>
                <p className="text-xs font-medium text-blue-600 dark:text-blue-400">
                  v{updateInfo.latest_version}
                </p>
              </div>
            </div>

            <button
              onClick={handleClose}
              className="
                p-1 rounded-full 
                text-gray-400 hover:text-gray-600 dark:text-gray-500 dark:hover:text-gray-300
                hover:bg-black/5 dark:hover:bg-white/10
                transition-all duration-200
              "
              aria-label={t('common.cancel')}
            >
              <X className="w-4 h-4" />
            </button>
          </div>

          <div className="mb-4">
            <p className="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">
              {t('update_notification.message', { current: updateInfo.current_version })}
            </p>
          </div>

          <button
            onClick={handleDownload}
            className="
              w-full group/btn
              relative overflow-hidden
              bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-500 hover:to-purple-500
              text-white font-medium
              py-2.5 px-4 rounded-xl
              shadow-lg shadow-blue-500/25
              transition-all duration-300
              flex items-center justify-center gap-2
              active:scale-[0.98]
            "
          >
            <Download className="w-4 h-4" />
            <span>{t('update_notification.action')}</span>
            <ArrowRight className="w-4 h-4 opacity-0 -translate-x-2 group-hover/btn:opacity-100 group-hover/btn:translate-x-0 transition-all duration-300" />

            {/* Shimmer effect */}
            <div className="absolute inset-0 -translate-x-full group-hover/btn:animate-[shimmer_1.5s_infinite] bg-gradient-to-r from-transparent via-white/20 to-transparent z-20 pointer-events-none" />
          </button>
        </div>
      </div>
    </div>
  );
};
