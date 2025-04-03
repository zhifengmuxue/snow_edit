use std::fs::read_to_string;
use std::io::Error;

use super::line::Line;

/// `Buffer` 结构体用于存储文本内容。
/// 它包含一个字符串向量，每个元素表示文本中的一行。
#[derive(Default)]
pub struct Buffer {
    /// 存储文本内容的行向量。
    pub lines: Vec<Line>,
}

impl Buffer {
    /// 从指定的文件加载内容到缓冲区。
    ///
    /// # 参数
    /// - `file_name`: 要加载的文件名。
    ///
    /// # 返回值
    /// 如果成功，返回包含文件内容的 `Buffer` 实例；如果失败，返回 `Error`。
    pub fn load(file_name: &str) -> Result<Self, Error> {
        // 读取文件内容为字符串
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();

        // 将文件内容按行分割并存储到 `lines` 向量中
        for value in contents.lines() {
            lines.push(Line::from(value));
        }

        // 返回包含行数据的 `Buffer` 实例
        Ok(Self { lines })
    }

    /// 检查缓冲区是否为空。
    ///
    /// # 返回值
    /// 如果缓冲区为空，返回 `true`；否则返回 `false`。
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }
}