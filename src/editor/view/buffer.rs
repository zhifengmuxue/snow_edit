use std::fs::read_to_string;
use std::io::Error;

use super::line::Line;
use super::Location;

/// `Buffer` 结构体用于存储文本内容。
/// 它包含一个字符串向量，每个元素表示文本中的一行。
#[derive(Default)]
pub struct Buffer {
    /// 存储文本内容的行向量。
    pub lines: Vec<Line>,
}

impl Buffer {
    /// 从指定的文件加载内容到缓冲区。
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
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    /// 获取缓冲区的行数。
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// 插入字符串到指定位置。
    pub fn insert_char(&mut self, character: char, at: Location){
        if at.line_index > self.lines.len() {
            return;
        }
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }
}