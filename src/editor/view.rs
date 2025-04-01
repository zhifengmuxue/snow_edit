use super::terminal::{Position, Size, Terminal};
use std::{env, fmt::format, io::Error};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 从editor接收所有与文本相关的事件，如字符按键、换行符
/// 用于提高渲染效率


pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size){
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0} )?;
        Terminal::clear_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<(), Error> {
        if !self.needs_redraw{
            return Ok(());
        }
        let Size { height, width} = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 2;

        for current_row in 0..height{
            if let Some(line) = self.buffer.lines.get(current_row){
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                }else{
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            }else if current_row == vertical_center && self.buffer.is_empty(){
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            }else{
                Self::render_line(current_row, "~")?;
            }
        }
        self.needs_redraw = false;
        Ok(())
    }

    /// 构建欢迎信息
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
             return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    /// 绘制空行
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
}

impl  Default for View {
    fn default() -> Self {
        Self { 
            buffer: Buffer::default(), 
            needs_redraw: true, 
            size: Terminal::size().unwrap_or_default()
        }
    }
}
