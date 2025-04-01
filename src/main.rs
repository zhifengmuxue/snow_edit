// 启用 Clippy 的一些警告规则，用于提高代码质量。
#![warn(
    clippy::all,                  // 启用所有 Clippy 检查。
    clippy::pedantic,             // 启用严格的 Clippy 检查。
    clippy::print_stdout,         // 警告直接使用 `println!`。
    clippy::arithmetic_side_effects, // 警告可能的算术副作用。
    clippy::as_conversions,       // 警告使用 `as` 进行类型转换。
    clippy::integer_division      // 警告整数除法操作。
)]

// 导入编辑器模块。
mod editor;
use editor::Editor;

/// 程序的入口点。
/// 创建一个 `Editor` 实例并运行主循环。
fn main() {
    // 初始化编辑器并运行主循环。
    // 如果初始化失败，程序会直接 panic。
    Editor::new().unwrap().run();
}