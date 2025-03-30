use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::fmt::Display;
use std::io::{Error, Write, stdout};

/// 大小
#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

/// 位置
#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// 终端行为
pub struct Terminal;

impl Terminal {
    /// 终止终端
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 初始化
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    /// 清理屏幕
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// 清理一行
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// 移动光标
    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))?;
        Ok(())
    }

    /// 隐藏光标
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// 展示光标
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// 输出
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// 大小
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    /// 执行
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
