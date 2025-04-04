use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// 表示一个字形的宽度。
#[derive(Clone, Copy)]
enum GraphemeWidth {
    /// 半宽字符（如 ASCII 字符）。
    Half,
    /// 全宽字符（如中文字符）。
    Full,
}

impl GraphemeWidth {
    /// 将当前宽度与另一个值相加，返回结果。
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Full => other.saturating_add(2),
            Self::Half => other.saturating_add(1),
        }
    }
}

/// 表示一段文本片段。
/// 每个片段包含一个字形及其渲染宽度和替代字符（如果有）。
struct TextFragment {
    /// 字形的实际内容。
    grapheme: String,
    /// 字形的渲染宽度。
    rendered_width: GraphemeWidth,
    /// 替代字符（用于不可见字符的显示）。
    replacement: Option<char>,
}

/// `Line` 结构体表示文本中的一行。
/// 它由多个 `TextFragment` 组成。
pub struct Line {
    /// 文本片段的集合。
    fragments: Vec<TextFragment>,
}

impl Line {
    /// 从字符串创建一个新的 `Line` 实例。
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }

    /// 将字符串转换为文本片段的向量。
    pub fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replace_character(grapheme)
                .map_or_else(
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


    fn replace_character(for_str: &str) -> Option<char>{
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('␣'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next(){
                    if ch.is_control() && chars.next().is_none(){
                        return  Some('▯');
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

    pub fn insert_char(&mut self, character: char, grapheme_index: usize){
        let mut result = String::new();
        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == grapheme_index {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }
        if grapheme_index >= self.fragments.len() {
            result.push(character);
        }
        self.fragments = Self::str_to_fragments(&result);
    }
}