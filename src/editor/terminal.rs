use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::{Attribute, Print};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen, SetTitle};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};

/// 表示终端的尺寸（宽度和高度）。
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub height: usize,  // 终端的高度（行数）。
    pub width: usize,   // 终端的宽度（列数）。
}

/// 表示终端中的光标位置。
#[derive(Clone, Copy, Default)]
pub struct Position {
    pub col: usize,         // 光标所在的列。
    pub row: usize,         // 光标所在的行。   
}

impl Position {
    /// 计算两个位置的差值，结果不会为负数。
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}

/// `Terminal` 结构体封装了终端的行为和操作。
pub struct Terminal;

impl Terminal {
    // ==================== 初始化和终止 ====================

    /// 初始化终端，进入原始模式并清理屏幕。
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::disable_line_wrap()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    /// 终止终端，恢复到正常模式。
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::enable_line_wrap()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    // ==================== 屏幕操作 ====================

    /// 清理整个屏幕。
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// 清理当前行。
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    // ==================== 光标操作 ====================

    /// 将光标移动到指定位置。
    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    /// 隐藏光标。
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// 显示光标。
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    // ==================== 文本输出 ====================

    /// 输出字符串到终端。
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// 在指定行打印文本。
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    // ==================== 尺寸获取 ====================

    /// 获取终端的尺寸（宽度和高度）。
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    // ==================== 内部辅助方法 ====================

    /// 刷新终端，执行所有排队的命令。
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// 将命令加入队列。
    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    /// 进入替代屏幕。
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// 离开替代屏幕。
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// 行内不可见
    pub fn disable_line_wrap() -> Result<(), Error> {
        Self::queue_command(DisableLineWrap)?;
        Ok(())
    }

    /// 行内可见
    pub fn enable_line_wrap() -> Result<(), Error> {
        Self::queue_command(EnableLineWrap)?;
        Ok(())
    }

    /// 设置终端标题。
    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_command(SetTitle(title))?;
        Ok(())
    }

    pub fn print_inverted_row(row: usize, line_text: &str) -> Result<(), Error>{
        let width = Self::size()?.width;
        Self::print_row(row, &format!(
            "{}{:width$.width$}{}",
            Attribute::Reverse,
            line_text,
            Attribute::Reset,
        ))
    }
}