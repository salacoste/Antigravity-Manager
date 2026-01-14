#!/usr/bin/env bash
set -euo pipefail

REMOTE="${1:-}"
REMOTE_DIR="${2:-/opt/antigravity}"
[[ -n "$REMOTE" ]] || { echo "用法: $0 user@server [远程目录]"; exit 1; }

# 自动检测本地数据目录（按优先级）
LOCAL=""
SEARCH_PATHS=(
    "$HOME/.antigravity_tools"
    "$HOME/.local/share/antigravity_tools"
    "$HOME/Library/Application Support/Antigravity Tools"
    "$HOME/Library/Application Support/com.antigravity.tools"
)

for d in "${SEARCH_PATHS[@]}"; do
    if [[ -d "$d" ]]; then
        LOCAL="$d"
        break
    fi
done

[[ -n "$LOCAL" ]] || { echo "错误: 未找到本地数据目录，已搜索:"; printf '  %s\n' "${SEARCH_PATHS[@]}"; exit 1; }

echo "同步: $LOCAL -> $REMOTE:$REMOTE_DIR/.antigravity_tools/"
rsync -avz --progress "$LOCAL/" "$REMOTE:$REMOTE_DIR/.antigravity_tools/"

# 尝试重启服务（可能需要 sudo）
echo "重启远程服务..."
ssh "$REMOTE" "sudo systemctl restart antigravity 2>/dev/null || systemctl restart antigravity 2>/dev/null || echo '提示: 请手动重启服务'"

echo "完成！"
