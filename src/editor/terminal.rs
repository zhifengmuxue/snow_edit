use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{Command, queue};
use std::io::{Error, Write, stdout};


/// 表示终端的尺寸（宽度和高度）。
#[derive(Default, Clone, Copy)]
pub struct Size {
    /// 终端的高度（行数）。
    pub height: usize,
    /// 终端的宽度（列数）。
    pub width: usize,
}

/// 表示终端中的光标位置。
#[derive(Clone, Copy, Default)]
pub struct Position {
    /// 光标所在的列。
    pub col: usize,
    /// 光标所在的行。
    pub row: usize,
}

impl Position{
    pub const fn saturating_sub(self, other: Self) -> Self{
        Self{
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col), 
        }
    }
}

/// `Terminal` 结构体封装了终端的行为和操作。
pub struct Terminal;

impl Terminal {
    /// 终止终端，恢复到正常模式。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::show_caret()?;
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 初始化终端，进入原始模式并清理屏幕。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::enter_alternate_screen()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    /// 清理整个屏幕。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    /// 清理当前行。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    /// 将光标移动到指定位置。
    ///
    /// # 参数
    /// - `pos`: 光标的新位置。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn move_caret_to(pos: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(pos.col as u16, pos.row as u16))?;
        Ok(())
    }

    /// 进入替代屏幕。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?;
        Ok(())
    }

    /// 离开替代屏幕。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?;
        Ok(())
    }

    /// 隐藏光标。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    /// 显示光标。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    /// 输出字符串到终端。
    ///
    /// # 参数
    /// - `string`: 要输出的字符串。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    /// 在指定行打印文本。
    ///
    /// # 参数
    /// - `row`: 要打印的行号。
    /// - `line_text`: 要打印的文本内容。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { col: 0, row })?;
        Self::clear_line()?;
        Self::print(line_text)?;
        Ok(())
    }

    /// 获取终端的尺寸（宽度和高度）。
    ///
    /// # 返回值
    /// 如果成功，返回 `Size`；如果失败，返回 `Error`。
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    /// 刷新终端，执行所有排队的命令。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    /// 将命令加入队列。
    ///
    /// # 参数
    /// - `command`: 要加入队列的命令。
    ///
    /// # 返回值
    /// 如果成功，返回 `Ok(())`；如果失败，返回 `Error`。
    fn queue_command(command: impl Command) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}