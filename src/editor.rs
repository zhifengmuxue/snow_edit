mod terminal;
mod view;
mod editorcommand;
mod statusbar;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use editorcommand::EditorCommand;
use statusbar::Statusbar;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
use view::View;



/// `Editor` 结构体是编辑器的核心，
pub struct Editor {
    should_quit: bool,          // 标志是否退出编辑器。
    view: View,                 // 编辑器的视图，用于渲染内容。
    status_bar: Statusbar,      // 状态栏，用于显示状态信息。
    
}

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus{
    total_lines: usize,       // 文档的总行数。
    current_line_index: usize,      // 当前行号。
    is_modified: bool,          // 文档是否被修改。
    file_name: Option<String>,   // 文档的文件名。
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
        let mut view = View::new(2);
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            view,
            status_bar: Statusbar::new(1),
        })
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
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit){
                    self.should_quit = true;
                } else {
                    self.view.handle_command(command);
                    if let EditorCommand::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        } 
    }

    /// 刷新屏幕内容。
    fn refresh_screen(&mut self) {
        // 隐藏光标。
        let _ = Terminal::hide_caret();

        // 渲染视图内容。
        self.view.render();
        self.status_bar.render();

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
