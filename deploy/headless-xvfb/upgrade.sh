#!/usr/bin/env bash
set -euo pipefail

# 权限检查
[[ $EUID -eq 0 ]] || { echo "错误: 需要 root 权限，请使用 sudo"; exit 1; }

cd "$(dirname "$0")"

REPO="lbjlaq/Antigravity-Manager"

# 获取远程最新版本
LATEST=$(curl -sfS "https://api.github.com/repos/${REPO}/releases/latest" | jq -r '.tag_name // empty' | sed 's/^v//')
[[ -n "$LATEST" ]] || { echo "错误: 无法获取版本号"; exit 1; }

# 获取当前版本
CURRENT=""
[[ -f .version ]] && CURRENT=$(cat .version)

if [[ "$CURRENT" == "$LATEST" ]]; then
    echo "已是最新版本 v${LATEST}"
    exit 0
fi

echo "升级: v${CURRENT:-未知} -> v${LATEST}"

# 备份
cp antigravity.AppImage antigravity.AppImage.bak

# 下载新版本
if ! wget -q --show-progress -O antigravity.AppImage \
    "https://github.com/${REPO}/releases/latest/download/Antigravity.Tools_${LATEST}_amd64.AppImage"; then
    echo "下载失败，恢复备份"
    mv antigravity.AppImage.bak antigravity.AppImage
    exit 1
fi
chmod +x antigravity.AppImage

# 重启服务
systemctl restart antigravity

sleep 3
if systemctl is-active --quiet antigravity; then
    echo "$LATEST" > .version
    rm -f antigravity.AppImage.bak
    echo "升级成功！v${LATEST}"
else
    echo "启动失败，回滚到 v${CURRENT:-备份}"
    mv antigravity.AppImage.bak antigravity.AppImage
    systemctl start antigravity
    exit 1
fi
