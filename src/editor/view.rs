use std::env;
mod buffer;
mod line;
mod location;
use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
use buffer::Buffer;
use location::Location;

/// `View` 结构体负责管理文本的渲染和显示。
/// 它从 `Editor` 接收与文本相关的事件（如字符按键、换行符），
/// 并通过优化渲染逻辑提高效率。

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `View` 结构体定义了编辑器的视图。
pub struct View {
    /// 当前缓冲区，存储文本内容。
    buffer: Buffer,
    /// 标记是否需要重新渲染。
    needs_redraw: bool,
    /// 当前视图的尺寸（宽度和高度）。
    size: Size,
    /// 当前光标的位置。
    location: Location,
    /// 滚动偏移量，用于确定视图的起始位置。
    scroll_offset: Location,
}

impl View {
    /// 渲染单行文本。
    ///
    /// # 参数
    /// - `at`: 行号。
    /// - `line_text`: 要渲染的文本内容。
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Fail to render line: {}", result.unwrap_err());
    }

    /// 渲染整个视图。
    ///
    /// 如果 `needs_redraw` 为 `false`，则跳过渲染。
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 2;
        let top = self.scroll_offset.y;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }

    /// 构建欢迎信息。
    ///
    /// # 参数
    /// - `width`: 视图的宽度。
    ///
    /// # 返回值
    /// 返回一条居中的欢迎信息字符串。
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

    /// 处理编辑器命令。
    ///
    /// # 参数
    /// - `command`: 要处理的命令。
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
        }
    }

    /// 加载文件内容到缓冲区。
    ///
    /// # 参数
    /// - `file_name`: 要加载的文件名。
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    /// 获取当前光标的位置。
    ///
    /// # 返回值
    /// 返回光标的绝对位置。
    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    /// 根据方向移动光标位置。
    ///
    /// # 参数
    /// - `direction`: 移动的方向。
    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = self.size;
        match direction {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => x = x.saturating_sub(1),
            Direction::Right => x = x.saturating_add(1),
            Direction::PageUp => y = 0,
            Direction::PageDown => y = height.saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = width.saturating_sub(1),
        }
        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    /// 调整视图的尺寸。
    ///
    /// # 参数
    /// - `to`: 新的尺寸。
    fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }

    /// 确保光标位置在视图范围内。
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { height, width } = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }
}

impl Default for View {
    /// 创建一个默认的 `View` 实例。
    ///
    /// 默认情况下，缓冲区为空，`needs_redraw` 为 `true`，尺寸为终端的当前大小。
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}