use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::queue;
use std::io::{stdout, Write, Error};

/// 大小
#[derive(Clone, Copy)]
pub struct Size{
    pub height: u16,
    pub width: u16,
}

/// 位置
#[derive(Clone, Copy)]
pub struct Position{
    pub x: u16,
    pub y: u16,
}

/// 终端行为
pub struct Terminal;

impl Terminal{
    /// 终止终端
    pub fn terminate() -> Result<(), Error>{
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 初始化
    pub fn initialize() -> Result<(), Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0, y:0})?;
        Self::execute()?;
        Ok(())
    }

    /// 清理屏幕
    pub fn clear_screen() -> Result<(), Error>{
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    /// 清理一行
    pub fn clear_line() -> Result<(), Error>{
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// 移动光标
    pub fn move_cursor_to(pos: Position) -> Result<(), Error>{
        queue!(stdout(), MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    /// 隐藏光标
    pub fn hide_cursor() -> Result<(),Error>{
        queue!(stdout(), Hide)?;
        Ok(())
    }

    /// 展示光标
    pub fn show_cursor() -> Result<(), Error>{
        queue!(stdout(), Show)?;
        Ok(())
    }

    /// 输出
    pub fn print(string: &str) -> Result<(), Error>{
        queue!(stdout(), Print(string))?;
        Ok(())
    }

    /// 大小
    pub fn size() -> Result<Size, Error>{
        let (width, height) = size()?;
        Ok(Size { height, width})
    }

    /// 执行
    pub fn execute() -> Result<(),Error>{
        stdout().flush()?;
        Ok(())
    }
}