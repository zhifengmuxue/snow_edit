use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

/// 表示光标移动的方向。
pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

/// 表示编辑器的命令。
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    /// 尝试从用户输入事件转换为 `EditorCommand`。
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
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                },
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
                _ => Err(format!("Unsupported key event: {event:?}")),
            },
            // 处理终端窗口大小调整事件。
            Event::Resize(width_u16, height_u16) => Ok(
                Self::Resize(Size {
                    width: width_u16 as usize,
                    height: height_u16 as usize,
                })
            ),
            // 如果事件类型不支持，返回错误信息。
            _ => Err(format!("Unsupported event: {event:?}")),
        }
    }
}