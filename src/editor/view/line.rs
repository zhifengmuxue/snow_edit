use core::fmt;
use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// 表示一个字形的宽度。
#[derive(Clone, Copy)]
enum GraphemeWidth {
    Half,           // 半宽字符（如 ASCII 字符）。
    Full,           // 全宽字符（如中文字符）。
}

impl GraphemeWidth {
    // 将当前宽度与另一个值相加，返回结果。
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Full => other.saturating_add(2),
            Self::Half => other.saturating_add(1),
        }
    }
}

/// 表示一段文本片段。
struct TextFragment {
    grapheme: String,                   // 字形的实际内容。
    rendered_width: GraphemeWidth,      // 字形的渲染宽度。
    replacement: Option<char>,          // 替代字符（用于不可见字符的显示）。
}

/// `Line` 结构体表示文本中的一行。
#[derive(Default)]
pub struct Line {
    fragments: Vec<TextFragment>,   // 文本片段的集合。
}

impl Line {
    /// 从字符串创建一个新的 `Line` 实例。
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }

    /// 将字符串转换为文本片段的向量。
    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replace_character(grapheme).map_or_else(
                    || {
                        let unicode_width = grapheme.width();
                        let rendered_width = match unicode_width {
                            0 | 1 => GraphemeWidth::Half,
                            _ => GraphemeWidth::Full,
                        };
                        (None, rendered_width)
                    },
                    |replacement| (Some(replacement), GraphemeWidth::Half),
                );
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect()
    }

    /// 替换不可见字符为替代字符。
    fn replace_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }

    /// 获取指定范围内的可见字形。
    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);

            if current_pos >= range.end {
                break;
            }

            if fragment_end > range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('⋯'); // 超出范围时显示省略号。
                } else if let Some(char) = fragment.replacement {
                    result.push(char); // 使用替代字符。
                } else {
                    result.push_str(&fragment.grapheme); // 添加实际字形。
                }
            }

            current_pos = fragment_end;
        }

        result
    }

    /// 获取行中字形的数量。
    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    /// 计算从行首到指定字形索引的宽度。
    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }

    /// 在指定位置插入一个字符。
    pub fn insert_char(&mut self, character: char, at: usize) {
        let mut result = String::new();
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == at {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }
        if at >= self.fragments.len() {
            result.push(character);
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    /// 删除指定索引的字形。
    pub fn delete(&mut self, at: usize) {
        let mut result = String::new();
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index != at {
                result.push_str(&fragment.grapheme);
            }
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    /// 将一行添加到另一行
    pub fn append(&mut self, other: &Self) {
        let mut concat = self.to_string();
        concat.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&concat);
    }

    /// 分割两个line 
    pub fn split(&mut self, at: usize) -> Self {
        if at > self.fragments.len() {
            return Self::default();
        }
        let remainder = self.fragments.split_off(at);
        Self {
            fragments: remainder,
        }
    }
}

/// 实现 `Display` trait，用于格式化输出。
impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(formatter, "{result}")
    }
}
