import React from 'react';
import { Gem, Diamond, Circle } from 'lucide-react';

interface AccountTierBadgeProps {
  tier: 'FREE' | 'PRO' | 'ULTRA';
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const AccountTierBadge: React.FC<AccountTierBadgeProps> = ({
  tier,
  size = 'md',
  className = '',
}) => {
  // Size variants
  const sizeConfig = {
    sm: {
      container: 'px-2 py-0.5 text-[10px]',
      icon: 'w-2.5 h-2.5',
    },
    md: {
      container: 'px-2.5 py-1 text-xs',
      icon: 'w-3 h-3',
    },
    lg: {
      container: 'px-3 py-1.5 text-sm',
      icon: 'w-4 h-4',
    },
  };

  // Tier-based styling
  const getTierConfig = () => {
    switch (tier) {
      case 'FREE':
        return {
          icon: <Circle className={`${sizeConfig[size].icon}`} />,
          bgColor: 'bg-base-200',
          textColor: 'text-base-content',
          borderColor: 'border-base-300',
          label: '🆓 FREE',
        };
      case 'PRO':
        return {
          icon: <Diamond className={`${sizeConfig[size].icon} fill-current`} />,
          bgColor: 'bg-gradient-to-r from-blue-600 to-indigo-600',
          textColor: 'text-white',
          borderColor: 'border-blue-600',
          label: '⭐ PRO',
        };
      case 'ULTRA':
        return {
          icon: <Gem className={`${sizeConfig[size].icon} fill-current`} />,
          bgColor: 'bg-gradient-to-r from-purple-600 to-pink-600',
          textColor: 'text-white',
          borderColor: 'border-purple-600',
          label: '💎 ULTRA',
        };
    }
  };

  const tierConfig = getTierConfig();

  return (
    <span
      className={`
        inline-flex items-center gap-1
        ${sizeConfig[size].container}
        ${tierConfig.bgColor}
        ${tierConfig.textColor}
        font-bold rounded-md shadow-sm
        ${tier === 'FREE' ? `border ${tierConfig.borderColor} dark:border-white/10` : ''}
        ${className}
      `}
    >
      {tierConfig.icon}
      {tier}
    </span>
  );
};
