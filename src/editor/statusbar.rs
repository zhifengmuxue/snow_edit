use super::{
    documentstatus::DocumentStatus,
    terminal::{Size, Terminal}, uicomponent::UIComponent,
};

#[derive(Default)]
pub struct Statusbar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    size: Size,
}

impl Statusbar {
    /// 更新状态栏状态。
    pub fn update_status(&mut self, new_status: DocumentStatus) {
        if self.current_status != new_status {
            self.current_status = new_status;
            self.set_needs_redraw(true);
        }
    }
}

impl UIComponent for Statusbar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let beginning = format!(
            "{} - {line_count} {modified_indicator}",
            self.current_status.file_name
        );

        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");

        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        Terminal::print_inverted_row(origin_y, &to_print)?;

        Ok(())
    }
}
