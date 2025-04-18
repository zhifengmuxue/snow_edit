mod command;
mod documentstatus;
mod fileinfo;
mod messagebar;
mod statusbar;
mod terminal;
mod uicomponent;
mod view;
use self::{
    command::{
        Command::{self, Edit, Move, System},
        System::{Quit, Resize, Save}, 
    },
    messagebar::MessageBar,
    terminal::Size,
};
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use statusbar::Statusbar;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
use uicomponent::UIComponent;
use view::View;
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

/// `Editor` 结构体是编辑器的核心，
#[derive(Default)]
pub struct Editor {
    should_quit: bool,       // 标志是否退出编辑器。
    view: View,              // 编辑器的视图，用于渲染内容。
    status_bar: Statusbar,   // 状态栏，用于显示状态信息。
    message_bar: MessageBar, // 消息栏，用于显示消息。
    terminal_size: Size,     // 终端的尺寸。
    title: String,           // 编辑器的标题。
    quit_times: u8,          // 退出确认次数
}

impl Editor {
    /// 构造方法，用于创建一个新的 `Editor` 实例。
    pub fn new() -> Result<Self, Error> {
        // 设置 panic 钩子，在程序崩溃时恢复终端状态。
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        // 初始化终端并进入原始模式。
        Terminal::initialize()?;

        // 创建默认视图并加载文件（如果提供了文件名）。
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.resize(size);
        editor
            .message_bar
            .update_message("HELP: Ctrl-S = save | Ctrl-D = quit");

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(file_name).is_err() {
                editor
                    .message_bar
                    .update_message(&format!("ERR: Could not open file: {file_name}"));
            }
        }
        editor.refresh_status();
        Ok(editor)
    }

    pub fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });

        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.file_name);
        self.status_bar.update_status(status);

        if title != self.title && matches!(Terminal::set_title(&title), Ok(_)) {
            self.title = title;
        }
    }

    /// 主运行循环，处理用户输入并刷新屏幕。
    pub fn run(&mut self) {
        loop {
            // 刷新屏幕内容。
            self.refresh_screen();

            // 如果标志为退出，则跳出循环。
            if self.should_quit {
                break;
            }

            // 读取用户输入事件并处理。
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}")
                    }
                }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    /// 处理用户输入事件。
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    fn process_command(&mut self, command: Command) {
        match command {
            System(Quit) => self.handle_quit(),
            System(Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(), // Reset quit times for all other commands
        }
        match command {
            System(Quit | Resize(_)) => {} // already handled above 1Has a conversation. Original line has a conversation.
            System(Save) => self.handle_save(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
        }
    }

    fn handle_save(&mut self) {
        if self.view.save().is_ok() {
            self.message_bar.update_message("File saved successfully.");
        } else {
            self.message_bar.update_message("Error writing file!");
        }
    }


    #[allow(clippy::arithmetic_side_effects)]
     fn handle_quit(&mut self) {
         if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES { 
             self.should_quit = true;
         } else if self.view.get_status().is_modified { 
             self.message_bar.update_message(&format!(
                 "WARNING! File has unsaved changes. Press Ctrl-D {} more times to quit.",
                 QUIT_TIMES - self.quit_times - 1
             ));
 
             self.quit_times += 1;
         }
     }

     fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message(""); 
        }
    }

    /// 刷新屏幕内容。
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }

        // 隐藏光标。
        let _ = Terminal::hide_caret();

        self.message_bar
            .render(self.terminal_size.height.saturating_sub(1));

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.view.render(0);
        }

        // 将光标移动到当前的位置。
        let _ = Terminal::move_caret_to(self.view.caret_position());

        // 显示光标并刷新终端。
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    /// 在 `Editor` 实例被销毁时执行清理操作。
    fn drop(&mut self) {
        // 恢复终端状态。
        let _ = Terminal::terminate();

        // 如果退出标志为真，打印退出消息。
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
