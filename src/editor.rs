mod terminal;

use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
use std::io::Error;
use terminal::{Terminal, Size, Position};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    /// 运行主方法
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    /// REPL
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit{
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    /// 按键事件
    fn evaluate_event(&mut self, event: &Event){
        if let Key(KeyEvent{
            code, modifiers, ..
        }) = event{
            match code {
                Char('d') if *modifiers == KeyModifiers::CONTROL =>{
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    /// 刷新屏幕
    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit{
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        }else{
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    /// 绘制‘～’标识
    fn draw_rows() -> Result<(), Error>{
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height{
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_row + 1 < height{
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
