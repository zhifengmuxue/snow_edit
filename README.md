# Snow Edit

## 简介

Snow Edit 是一个用 Rust 编写的轻量级文本编辑器，旨在提供简洁高效的编辑体验。这个项目专注于创建一个性能优良、内存安全的终端文本编辑器，同时提供友好的用户界面。

## 特性

- 📝 基础文本编辑功能（插入、删除、光标移动）
- 🔄 文件加载和保存
- 👁️ 视图滚动和光标定位
- 📊 状态栏显示文件信息和编辑状态
- 💬 消息栏提示用户操作
- 🎨 友好的用户界面

## 项目结构
```Rust
src/
├── main.rs             // 程序入口点
├── editor.rs           // 编辑器核心逻辑
└── editor/             // 编辑器组件
    ├── command.rs      // 编辑命令定义
    ├── documentstatus.rs // 文档状态管理
    ├── fileinfo.rs     // 文件信息处理
    ├── messagebar.rs   // 消息栏组件
    ├── statusbar.rs    // 状态栏组件
    ├── terminal.rs     // 终端交互
    ├── uicomponent.rs  // UI组件接口
    ├── view.rs         // 文本视图
    └── view/           // 视图相关组件
        ├── buffer.rs   // 文本缓冲区
        └── line.rs     // 行处理
```


## 安装与运行

### 安装
Rust 编程环境（rustc, cargo）
```bash
# 克隆仓库
git clone https://github.com/yourusername/snow_edit.git
cd snow_edit

# 编译项目
cargo build --release
```
### 运行
```bash
# 启动编辑器
cargo run [文件路径]

# 或直接使用编译后的可执行文件
./target/release/snows_edit [文件路径]
```

### 使用方法
基本操作
- 移动光标: 箭头键
- 翻页: Page Up/Down
- 行首/行尾: Home/End
- 插入文本: 直接输入字符
- 删除: Delete/Backspace
- 保存文件: Ctrl+S
- 退出: Ctrl+D

### 贡献
欢迎提交 Pull Requests 和 Issues！