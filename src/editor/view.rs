use std::{cmp::min, io::Error};
mod buffer;
mod line;
use super::{
    NAME, VERSION,
    documentstatus::DocumentStatus,
    command::{Edit, Move},
    terminal::{Position, Size, Terminal},
    uicomponent::UIComponent,
};
use buffer::Buffer;
use line::Line;

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize, // 当前光标所在的字形索引。
    pub line_index: usize,     // 当前光标所在的行索引。
}

/// `View` 结构体定义了编辑器的视图。
#[derive(Default)]
pub struct View {
    buffer: Buffer,          // 当前缓冲区，存储文本内容。
    needs_redraw: bool,      // 标记是否需要重新渲染。
    size: Size,              // 当前视图的尺寸（宽度和高度）。
    text_location: Location, // 当前光标的位置。
    scroll_offset: Position, // 滚动偏移量，用于确定视图的起始位置。
}

impl View {
    // ==================== 渲染相关方法 ====================

    /// 渲染单行文本。
    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    /// 生成欢迎信息。
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        if remaining_width < len {
            return "~".to_string();
        }

        format!("{:<1}{:^remaining_width$}", "~", welcome_message)
    }

    /// 获取文档状态。
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_index: self.text_location.line_index,
            file_name: format!("{}", self.buffer.file_info),
            is_modified: self.buffer.dirty,
        }
    }

    // ==================== 编辑器命令相关方法 ====================

    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.insert_char(character),
            Edit::Delete => self.delete(),
            Edit::DeleteBackward => self.delete_backward(),
            Edit::InsertNewline => self.insert_newline(),
        }
    }

    pub fn handle_move_command(&mut self, command: Move){
        let Size { height, .. } = self.size;
        match command {
             Move::Up => self.move_up(1),
             Move::Down => self.move_down(1),
             Move::Left => self.move_left(),
             Move::Right => self.move_right(),
             Move::PageUp => self.move_up(height.saturating_sub(1)),
             Move::PageDown => self.move_down(height.saturating_sub(1)),
             Move::StartOfLine => self.move_to_start_of_line(),
             Move::EndOfLine => self.move_to_end_of_line(),
         }
         self.scroll_text_location_into_view();
    }

    /// 加载文件。
    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.set_needs_redraw(true);
        Ok(())
    }

    // ==================== 文本编辑相关方法 ====================

    /// 插入新字符。
    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        self.buffer.insert_char(character, self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            self.handle_move_command(Move::Right);
        }
        self.set_needs_redraw(true);
    }

    /// 插入新行
    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.handle_move_command(Move::Right);
        self.set_needs_redraw(true);
    }

    /// 文件保存
    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }

    /// 删除光标左侧的字符。
    fn delete_backward(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
            self.handle_move_command(Move::Left);
            self.delete();
        }
    }

    /// 删除光标上的字符
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.set_needs_redraw(true);
    }

    // ==================== 光标移动相关方法 ====================

    /// 光标向上移动
    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    /// 光标向下移动
    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    /// 光标向右移动
    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    /// 光标向左移动
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    /// 光标移动到行首
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    /// 光标移动到行尾
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    // ==================== 滚动相关方法 ====================

    /// 竖直滚动
    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    /// 水平滚动
    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    /// 滚动文本位置到可见区域。
    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    // ==================== 辅助方法 ====================

    /// 获取当前光标位置。
    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    /// 获取当前光标在缓冲区中的位置。
    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    /// 对齐有效字素
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    /// 对齐有效行。
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }
}

impl UIComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), Error> {
        let Size { width, height } = self.size;
        let end_y = origin_y.saturating_add(height);

        #[allow(clippy::integer_division)]
        let top_third = height / 3;
        let scroll_top = self.scroll_offset.row;
        for current_row in origin_y..end_y {
            let line_idx = current_row
                .saturating_sub(origin_y)
                .saturating_add(scroll_top);
            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right))?;
            } else if current_row == top_third && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}
