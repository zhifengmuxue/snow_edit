use super::terminal::{Size, Terminal};
use std::env;
mod buffer;
use buffer::Buffer;

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
}

impl View {
    /// 调整视图的尺寸。
    ///
    /// # 参数
    /// - `to`: 新的尺寸。
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    /// 渲染单行文本。
    ///
    /// # 参数
    /// - `at`: 行号。
    /// - `line_text`: 要渲染的文本内容。
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line ");
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

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line);
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
        }
    }
}