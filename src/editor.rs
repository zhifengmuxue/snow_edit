mod terminal;
mod view;
use crossterm::event::{
    Event, KeyCode, KeyEvent, 
    KeyEventKind, KeyModifiers, read,
};
use std::{
    cmp::min,
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::{Position, Size, Terminal};
use view::View;

/// `Editor` 组件主要负责在不同的 UI 组件之间进行协调，
/// 处理用户输入事件并更新视图。

/// 表示光标的位置。
#[derive(Clone, Copy, Default)]
struct Location {
    /// 光标的列位置。
    x: usize,
    /// 光标的行位置。
    y: usize,
}

/// `Editor` 结构体是编辑器的核心，
/// 包含光标位置、视图和退出标志等。
pub struct Editor {
    /// 标志是否退出编辑器。
    should_quit: bool,
    /// 当前光标的位置。
    location: Location,
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
            location: Location::default(),
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

    /// 移动光标到指定位置。
    ///
    /// # 参数
    /// - `key_code`: 表示移动方向的按键代码。
    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();

        // 根据按键代码更新光标位置。
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }

        // 更新光标位置。
        self.location = Location { x, y };
    }

    /// 处理用户输入事件。
    ///
    /// # 参数
    /// - `event`: 用户输入事件。
    fn evaluate_event(&mut self, event: Event) {
        match event {
            // 处理按键事件。
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                modifiers,
                ..
            }) => match (code, modifiers) {
                // 如果按下 `Ctrl + D`，设置退出标志。
                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }

                // 处理光标移动相关的按键。
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home,
                    _,
                ) => {
                    self.move_point(code);
                }
                _ => (),
            },

            // 处理终端窗口大小调整事件。
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size { height, width });
            }
            _ => (),
        }
    }

    /// 刷新屏幕内容。
    fn refresh_screen(&mut self) {
        // 隐藏光标。
        let _ = Terminal::hide_caret();

        // 渲染视图内容。
        self.view.render();

        // 将光标移动到当前的位置。
        let _ = Terminal::move_caret_to(Position {
            col: self.location.x,
            row: self.location.y,
        });

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