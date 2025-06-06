# Slint 音乐播放器

一个使用 Rust 和 Slint UI 框架开发的现代化音乐播放器应用程序。

## 功能特性

### 🎵 核心播放功能
- 支持常见音频格式播放
- 播放控制：播放/暂停、上一首/下一首
- 音量控制和进度条调节
- 播放列表管理

### 🎨 用户界面
- 现代化的 UI 设计
- 多页面导航系统
- 自定义窗口控件（最小化、最大化、关闭）
- 可拖拽的无边框窗口
- 响应式布局设计

### 📱 页面功能
- **首页**: 推荐音乐和热门内容
- **音乐库**: 管理本地音乐文件
- **喜欢的音乐**: 收藏音乐列表
- **最近播放**: 播放历史记录
- **下载管理**: 下载的音乐文件管理
- **歌手页面**: 歌手信息和专辑
- **主题设置**: 个性化界面主题

### 🔧 技术特性
- 多线程音频处理
- 高效的音频解码
- 响应式状态管理
- 模块化组件设计

## 技术栈

- **Rust**: 系统编程语言，提供高性能和内存安全
- **Slint**: 现代化的 GUI 框架
- **rodio**: Rust 音频播放库
- **构建工具**: Cargo

## 项目结构

```
music-player/
├── src/
│   └── main.rs          # 主应用程序逻辑
├── ui/
│   ├── app/             # 应用主要组件
│   ├── components/      # 可复用UI组件
│   └── pages/           # 各个功能页面
├── assets/              # 静态资源文件
│   ├── *.png/svg        # 图标和图片
│   └── *.mp3            # 示例音频文件
├── build.rs             # 构建脚本
├── Cargo.toml           # 项目配置和依赖
└── README.md            # 项目说明文档
```

## 安装和运行

### 前提条件

- Rust 1.70 或更高版本
- Cargo 包管理器

### 安装 Rust

如果你还没有安装 Rust，请访问 [https://rustup.rs/](https://rustup.rs/) 并按照说明安装。

### 克隆并运行项目

```bash
# 克隆项目
git clone https://github.com/jychen6688/slint_music_player.git
cd slint_music_player

# 编译并运行
cargo run
```

### 构建发布版本

```bash
cargo build --release
```

编译后的可执行文件将位于 `target/release/` 目录中。

## 使用说明

### 基本操作

1. **添加音乐**: 将音频文件拖拽到播放器中或通过文件菜单添加
2. **播放控制**: 使用底部播放控制栏进行播放、暂停、切换歌曲
3. **音量调节**: 通过音量滑块调整播放音量
4. **进度控制**: 点击进度条跳转到指定播放位置

### 界面导航

- 使用左侧导航栏在不同页面间切换
- 点击窗口标题栏可拖拽移动窗口
- 使用右上角按钮进行窗口操作

### 播放列表管理

- 在播放列表中添加、删除歌曲
- 双击歌曲名称开始播放
- 支持播放顺序调整

## 开发

### 项目依赖

主要依赖项在 `Cargo.toml` 中定义：

- `slint = "1.9.1"` - UI 框架
- `rodio = "0.20.1"` - 音频播放
- `slint-build = "1.9.1"` - 构建时依赖

### 代码结构

- `src/main.rs`: 包含主要的应用逻辑和事件处理
- `ui/`: Slint UI 定义文件
- `music` 模块: 音频播放功能封装

### 自定义和扩展

你可以通过以下方式扩展应用功能：

1. 添加新的 UI 页面到 `ui/pages/` 目录
2. 创建新的可复用组件到 `ui/components/` 目录
3. 在 `src/main.rs` 中添加新的事件处理逻辑
4. 扩展音频处理功能

## 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 更新日志

### v0.1.0
- 初始版本发布
- 基本音频播放功能
- 现代化 UI 界面
- 播放列表管理
- 多页面导航系统

## 联系方式

- 项目链接: [https://github.com/jychen6688/slint_music_player](https://github.com/jychen6688/slint_music_player)
- 问题反馈: [GitHub Issues](https://github.com/jychen6688/slint_music_player/issues)

## 致谢
- [![Powered by DartNode](https://dartnode.com/branding/DN-Open-Source-sm.png)](https://dartnode.com "Powered by DartNode - Free VPS for Open Source") - 提供VPS
- [Slint](https://slint-ui.com/) - 优秀的 Rust GUI 框架
- [rodio](https://github.com/RustAudio/rodio) - 强大的 Rust 音频库
- 所有为开源社区做出贡献的开发者们
