mod terminal;
mod view;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use editorcommand::EditorCommand;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
use view::View;
mod editorcommand;

/// `Editor` 组件主要负责在不同的 UI 组件之间进行协调，
/// 处理用户输入事件并更新视图。

/// `Editor` 结构体是编辑器的核心，
/// 包含光标位置、视图和退出标志等。
pub struct Editor {
    /// 标志是否退出编辑器。
    should_quit: bool,
    /// 编辑器的视图，用于渲染内容。
    view: View,
}

impl Editor {
    /// 构造方法，用于创建一个新的 `Editor` 实例。
    ///
    /// # 返回值
    /// 如果成功，返回 `Editor` 实例；如果失败，返回 `Error`。
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
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            view,
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
        }
    }

    /// 处理用户输入事件。
    ///
    /// # 参数
    /// - `event`: 用户输入事件。
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not process event: {err:?}")
                    }
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Received and discarded unsupported or non-press event.");
            }
        }
    }

    /// 刷新屏幕内容。
    fn refresh_screen(&mut self) {
        // 隐藏光标。
        let _ = Terminal::hide_caret();

        // 渲染视图内容。
        self.view.render();

        // 将光标移动到当前的位置。
        let _ = Terminal::move_caret_to(self.view.get_position());

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
