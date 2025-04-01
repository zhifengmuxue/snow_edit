use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

/// 大小
#[derive(Default, Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

/// 位置
#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

/// 终端行为
pub struct Terminal;

impl Terminal {
    /// 终止终端
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 初始化
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
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
    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    /// 进入替代屏幕
    pub fn enter_alternate_screen() -> Result<(), Error>{
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// 离开替代屏幕
    pub fn leave_alternate_screen() -> Result<(),Error>{
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// 隐藏光标
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// 展示光标
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// 输出
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// 成行打印
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error>{
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
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
