use super::{
    terminal::{Size, Terminal},
    DocumentStatus,
};

pub struct Statusbar {
    current_status: DocumentStatus,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl Statusbar  {
    /// 新建一个状态栏实例。
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            current_status: DocumentStatus::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }

    /// 刷新状态栏。
    pub fn resize(&mut self, size: Size){
        self.width = size.width;
        self.position_y = size.height.saturating_sub(self.margin_bottom).saturating_sub(1);
        self.needs_redraw = true;
    }

    /// 更新状态栏状态。
    pub fn update_status(&mut self, new_status: DocumentStatus){
        if self.current_status != new_status {
            self.current_status = new_status;
            self.needs_redraw = true;
        }
    }

    /// 渲染
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let mut status = format!("{:?}", self.current_status);
        status.truncate(self.width);
        let result = Terminal::print_row(self.position_y, &status);

        debug_assert!(result.is_ok(), "Could not render status bar");
        self.needs_redraw = false;
    }
}