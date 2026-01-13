# Antigravity Tools 服务器部署

> 无需改动源码，使用 Xvfb 在 Linux 服务器上运行 GUI 应用

## 一键安装

```bash
curl -fsSL https://raw.githubusercontent.com/lbjlaq/Antigravity-Manager/main/deploy/headless-xvfb/install.sh | sudo bash
```

或者（更安全，先审查脚本）：

```bash
curl -O https://raw.githubusercontent.com/lbjlaq/Antigravity-Manager/main/deploy/headless-xvfb/install.sh
cat install.sh  # 审查脚本
sudo bash install.sh
```

## 目录结构

安装后在 `/opt/antigravity/` 下：

```
antigravity/
├── antigravity.AppImage          # 主程序
├── .antigravity_tools/           # 数据目录
│   ├── accounts/
│   │   └── {uuid}.json
│   ├── accounts.json
│   └── gui_config.json
├── logs/
│   └── app.log
├── .version                      # 当前版本号
├── install.sh                    # 部署脚本
├── sync.sh                       # 账号同步
└── upgrade.sh                    # 升级脚本
```

## 同步本地账号

在本地 Mac/Linux 执行：

```bash
# 下载同步脚本
curl -O https://raw.githubusercontent.com/lbjlaq/Antigravity-Manager/main/deploy/headless-xvfb/sync.sh
chmod +x sync.sh

# 同步到服务器
./sync.sh root@your-server /opt/antigravity
```

## 升级

```bash
cd /opt/antigravity
sudo ./upgrade.sh
```

## 运维命令

```bash
systemctl status antigravity          # 查看状态
systemctl restart antigravity         # 重启服务
systemctl stop antigravity            # 停止服务
tail -f /opt/antigravity/logs/app.log # 查看日志
curl localhost:8045/healthz           # 健康检查
```

## 备份与迁移

```bash
# 备份整个目录
tar czf antigravity-backup.tar.gz /opt/antigravity

# 恢复到新服务器
tar xzf antigravity-backup.tar.gz -C /opt
cd /opt/antigravity && sudo ./install.sh
```

## 技术原理

- **Xvfb**: 虚拟显示器，让 GUI 程序在无显示器环境运行
- **HOME 重定向**: `HOME=$PWD` 使程序在当前目录创建 `.antigravity_tools/`，实现便携部署

## ⚠️ 方案局限性 (Limitations)

这种通过 Xvfb 运行 GUI 应用的“伪无头”方案存在以下缺点：
- **资源占用高**: 即使没有显示器，后台依然运行着完整的 WebView 渲染引擎，内存和 CPU 开销显著高于纯后端服务。
- **依赖较多**: 需要安装 Xvfb 及 GTK/WebKit 等图形库依赖。
- **启动速度**: 相比原生 Web 服务，GUI 应用的启动和初始化稍慢。

> [!TIP]
> 我们正在开发 **Web 原生模式 (PR #398)**，未来将支持真正的轻量化服务器部署，敬请期待。

## 系统要求

- Ubuntu 20.04+ / Debian 11+ / RHEL 8+ / CentOS Stream 8+
- 自动安装依赖：xvfb, libwebkit2gtk, libgtk-3, wget, curl, jq
