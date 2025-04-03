use std::cmp;
use std::ops::Range;

/// `Line` 结构体表示文本中的一行。
/// 它封装了一个字符串，并提供了操作该字符串的方法。
pub struct Line {
    /// 存储行内容的字符串。
    string: String,
}

impl Line {
    /// 从字符串创建一个新的 `Line` 实例。
    ///
    /// # 参数
    /// - `line_str`: 用于初始化 `Line` 的字符串切片。
    ///
    /// # 返回值
    /// 返回一个包含指定字符串的 `Line` 实例。
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }

    /// 获取指定范围内的子字符串。
    ///
    /// # 参数
    /// - `range`: 要获取的字符串范围。
    ///
    /// # 返回值
    /// 返回范围内的子字符串。如果范围超出字符串长度，则返回空字符串。
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len()); // 确保范围不超出字符串长度。
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    /// 获取行的长度。
    ///
    /// # 返回值
    /// 返回行中字符的数量。
    pub fn len(&self) -> usize{
        self.string.len()
    }
}