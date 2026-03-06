# NoIdea AI 影视创作工作室

一款基于 AI 技术的短剧制作工具，支持从小说文本自动生成分镜、角色、场景，并制作成完整视频。

## 功能特性

-| 功能描述
-|
🎬 | 小说分析 - 自动解析小说，提取角色、场景、剧情
🎨 | 角色 & 场景生成 - AI 生成一致性人物和场景图片
📽️ | 分镜视频制作 - 自动生成分镜头并合成视频
🎙️ | AI 配音 - 多角色语音合成
💾 | 本地优先架构 - 所有数据存储在本地机器
🌐 | 多端访问 - 本地作为桌面应用使用，或通过 Web 从其他设备访问

## 系统架构

NoIdea 将客户端和服务端合二为一，集成在单个可执行文件中：

```
┌─────────────────────────────────────────────┐
│          NoIdea 应用程序                     │
├─────────────────────────────────────────────┤
│  前端: React + TypeScript                  │
│  后端:  Rust (Tauri 框架)                 │
├─────────────────────────────────────────────┤
│  数据层:                                     │
│  ├─ SQLite 数据库                          │
│  └─ LiteS3 服务 (本地文件存储)              │
├─────────────────────────────────────────────┤
│  网络:                                       │
│  ├─ 本地 IPC (桌面应用)                    │
│  └─ HTTP 服务器 (Web 访问)                 │
└─────────────────────────────────────────────┘
```

## 技术栈

### 前端
- **React** + **TypeScript**
- **Tailwind CSS**
- **React Query** - 数据获取

### 后端
- **Rust** - 核心应用逻辑
- **Tauri** - 桌面应用框架
- **SQLite** - 本地数据库 (通过 sqlx crate)
- **LiteS3** - 本地 S3 兼容存储服务

### 外部 API
- 文本、图像和视频生成的 AI 提供商
- 通过设置面板配置

## 安装

### 前置要求
- Rust 工具链 (1.70+)
- Node.js (18+)
- Tauri CLI

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/your-username/noidea.git
cd noidea

# 安装依赖
npm install

# 构建应用
npm run tauri build

# 或在开发模式下运行
npm run tauri dev
```

### 发布版本

从 [发布页面](https://github.com/your-username/noidea/releases) 下载最新版本。

## 使用方法

### 桌面模式
1. 启动 NoIdea 可执行文件
2. 创建新项目或打开现有项目
3. 导入小说文本
4. 配置 AI 提供商设置
5. 生成分镜和视频

### Web 访问模式
1. 启动 NoIdea 应用
2. 在设置中启用 Web 服务器
3. 通过浏览器访问 `http://localhost:PORT`
4. 将 URL 分享给同一网络下的其他设备

## 开发

### 项目结构

```
noidea/
├── src-tauri/           # Rust 后端
│   ├── src/
│   │   ├── main.rs      # 应用入口
│   │   ├── api/         # HTTP API 端点
│   │   ├── db/          # 数据库层 (SQLite)
│   │   ├── storage/     # LiteS3 存储服务
│   │   ├── tasks/       # 异步任务管理
│   │   └── ai/          # AI 提供商集成
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                 # React 前端
│   ├── components/       # React 组件
│   ├── pages/           # 应用页面
│   ├── api/             # API 客户端
│   └── hooks/           # React 钩子
├── package.json
└── README.md
```

### 开发模式运行

```bash
# 安装依赖
npm install

# 启动开发服务器 (Rust + React)
npm run tauri dev

# 运行测试
npm test

# 运行 Rust 测试
cd src-tauri && cargo test
```

### 构建

```bash
# 开发版本构建
npm run tauri build --debug

# 发布版本构建
npm run tauri build

# 输出文件位于 src-tauri/target/release/bundle/
```

## 配置

### 数据库
- 位置: `<app-data>/noidea/database.sqlite`
- 启动时自动迁移
- 架构变更前自动备份

### 存储
- 位置: `<app-data>/noidea/storage/`
- S3 兼容 API
- 自动清理未使用的资源

### 网络
- 默认 HTTP 端口: 3000
- 可在设置中配置
- 支持 Web 访问的 CORS

## API 端点

### 项目管理
- `GET /api/projects` - 列出所有项目
- `POST /api/projects` - 创建新项目
- `GET /api/projects/:id` - 获取项目详情
- `PUT /api/projects/:id` - 更新项目
- `DELETE /api/projects/:id` - 删除项目

### 任务管理
- `GET /api/tasks` - 列出任务
- `POST /api/tasks` - 创建任务
- `GET /api/tasks/:id` - 获取任务状态
- `DELETE /api/tasks/:id` - 取消任务

### 存储管理
- `GET /api/storage/*` - 检索存储的文件
- `PUT /api/storage/*` - 上传文件
- `DELETE /api/storage/*` - 删除文件

## 开发路线图

- [ ] 小说文本导入和解析
- [ ] 角色资料生成
- [ ] 场景可视化
- [ ] 分镜创作
- [ ] AI 图像生成
- [ ] AI 视频生成
- [ ] 语音合成
- [ ] 视频导出
- [ ] 项目共享
- [ ] 多用户协作

## 贡献

欢迎贡献！请随时提交 Pull Request。

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交你的更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开一个 Pull Request

## 许可证

MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 支持

如有问题、疑问或建议，请在 [GitHub Issues](https://github.com/your-username/noidea/issues) 上提交 issue。

## 致谢

使用 ❤️ 构建，基于 [Tauri](https://tauri.app/)、[React](https://reactjs.org/) 和 [Rust](https://www.rust-lang.org/)。
