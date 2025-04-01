use super::terminal::{Size, Terminal};
use std::{env, io::Error};
mod buffer;
use buffer::Buffer;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 从editor接收所有与文本相关的事件，如字符按键、换行符
/// 用于提高渲染效率

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    /// 刷新屏幕
    pub fn render_welcome_screen() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        
        for current_row in 0..height {
            Terminal::clear_line()?;

            #[allow(clippy::integer_division)]
             if current_row == height / 2 {
                 Self::draw_welcome_message()?;
             } else {
                 Self::draw_empty_row()?;
             }
             if current_row.saturating_add(1) < height {
                 Terminal::print("\r\n")?;
             }
        }
        Ok(())
    }

    pub fn render_buffer(&self) -> Result<(), Error>{
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height{
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row){
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            } else {
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(),Error>{
        if self.buffer.is_empty(){
            Self::render_welcome_screen()?;
        }else{
            self.render_buffer()?;
        }
        Ok(())
    }

    /// 绘制欢迎信息
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    /// 绘制空行
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name)  {
            self.buffer = buffer;
        }
    }
}
