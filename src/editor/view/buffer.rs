use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::Write;
use crate::editor::fileinfo::FileInfo;

use super::line::Line;
use super::Location;

/// 存储文本内容,进行底层交互。
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,               // 存储文本内容的行向量。
    pub file_info: FileInfo,      // 文件信息
    pub dirty: bool,                    // 标志是否已经被修改（脏数据）。
}

impl Buffer {
    /// 读取文件，加载到缓冲区。
    pub fn load(file_name: &str) -> Result<Self, Error> {
        // 读取文件内容为字符串
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();

        // 将文件内容按行分割并存储到 `lines` 向量中
        for value in contents.lines() {
            lines.push(Line::from(value));
        }

        // 返回包含行数据的 `Buffer` 实例
        Ok(Self { 
            lines ,
            file_info: FileInfo::from(file_name),
            dirty: false,
        })
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
        if at.line_index > self.height() {
            return;
        }
        if at.line_index == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
            self.dirty = true;
        }
    }

    /// 删除字符。
    pub fn delete(&mut self, at: Location){
        if let Some(line) = self.lines.get(at.line_index){
            if at.grapheme_index >= line.grapheme_count()
            && self.height() > at.line_index.saturating_add(1){
                let next_line = self.lines.remove(at.line_index.saturating_add(1));

                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;

            } else if at.grapheme_index < line.grapheme_count() {
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

    /// 插入一行
    pub fn insert_newline(&mut self, at: Location){
        if at.line_index == self.height() {
            self.lines.push(Line::default());
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index){
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
            self.dirty = true;
        }
    }

    /// 保存缓冲区内容到文件。
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(path) = &self.file_info.path {
            let mut file = File::create(path)?;
            for line in &self.lines {
                writeln!(file, "{line}")?; 
            }
            self.dirty = false;
        }
        Ok(())
    }

}