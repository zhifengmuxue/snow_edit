use super::terminal::Size;
use super::terminal::Terminal;
use super::uicomponent::UIComponent;
use std::io::Error;
use std::time::Duration;
use std::time::Instant;

const DEFAULT_DURATION: Duration = Duration::new(5, 0);

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}


#[derive(Default)]
pub struct MessageBar {
    current_message: Message, // 当前显示的消息。
    needs_redraw: bool,      // 是否需要重绘。
    cleared_after_expiry: bool,
}

impl MessageBar {
    /// 更新状态
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message{
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UIComponent for MessageBar {
    /// 标记是否需要重绘。
    fn set_needs_redraw(&mut self, needs_redraw: bool) {
        self.needs_redraw = needs_redraw;
    }

    /// 检查是否需要重绘。
    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    /// 设置组件大小。
    fn set_size(&mut self, _size: Size) {}

    /// 绘制组件。
    fn draw(&mut self, origin: usize) -> Result<(), Error> {
        if self.current_message.is_expired(){
            self.cleared_after_expiry = true;
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin, message)
    }
}
