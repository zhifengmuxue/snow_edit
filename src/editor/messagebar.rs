use super::terminal::Terminal;
use super::terminal::Size;
use std::io::Error;
use super::uicomponent::UIComponent;
#[derive(Default)]
pub struct MessageBar {
    current_message: String,    // 当前显示的消息。
    needs_redraw: bool,         // 是否需要重绘。
}

impl MessageBar {
    /// 更新状态
    pub fn update_status(&mut self, new_message: String) {
        if new_message != self.current_message {
            self.current_message = new_message;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for MessageBar {
    /// 标记是否需要重绘。
    fn mark_redraw(&mut self, value: bool){
        self.needs_redraw = value;
    }

    /// 检查是否需要重绘。
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    /// 设置组件大小。
    fn set_size(&mut self, _size: Size) {
        
    }

    /// 绘制组件。
    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        Terminal::print_row(origin, &self.current_message)
    }
}