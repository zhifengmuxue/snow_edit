use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    /// REPL
    fn repl(&mut self) -> Result<(), std::io::Error> {
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
            println!("Code: {code:?} Modifiers: {modifiers:?} \r");
            match code {
                Char('d') if *modifiers == KeyModifiers::CONTROL =>{
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    /// 刷新屏幕
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit{
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        }else{
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    /// 绘制‘～’标识
    fn draw_rows() -> Result<(), std::io::Error>{
        let height = Terminal::size()?.1;
        for current_row in 0..height{
            print!("~");
            if current_row + 1 < height{
                print!("\r\n");
            }
        }
        Ok(())
    }
}
