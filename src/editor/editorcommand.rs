use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

/// 表示光标移动的方向。
pub enum Direction {
    /// 向上一页。
    PageUp,
    /// 向下一页。
    PageDown,
    /// 移动到行首。
    Home,
    /// 移动到行尾。
    End,
    /// 向上移动一行。
    Up,
    /// 向左移动一个字符。
    Left,
    /// 向右移动一个字符。
    Right,
    /// 向下移动一行。
    Down,
}

/// 表示编辑器的命令。
pub enum EditorCommand {
    /// 移动光标的命令，包含方向。
    Move(Direction),
    /// 调整终端大小的命令，包含新的尺寸。
    Resize(Size),
    /// 退出编辑器的命令。
    Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    /// 尝试从用户输入事件转换为 `EditorCommand`。
    ///
    /// # 参数
    /// - `event`: 用户输入事件。
    ///
    /// # 返回值
    /// 如果成功，返回对应的 `EditorCommand`；如果失败，返回错误信息。
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            // 处理按键事件。
            Event::Key(KeyEvent {
                code,
                modifiers,
                ..
            }) => match (code, modifiers) {
                // 如果按下 `Ctrl + D`，返回退出命令。
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                // 如果按下方向键，返回对应的移动命令。
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                // 如果按键不支持，返回错误信息。
                _ => Err(format!("Unsupported key event: {:?}", event)),
            },
            // 处理终端窗口大小调整事件。
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            // 如果事件类型不支持，返回错误信息。
            _ => Err(format!("Unsupported event: {:?}", event)),
        }
    }
}