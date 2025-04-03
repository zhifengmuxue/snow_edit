use std::cmp;
use std::ops::Range;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// `Line` 结构体表示文本中的一行。
/// 它封装了一个字符串，并提供了操作该字符串的方法。

#[derive(Clone, Copy)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Full => other.saturating_add(2),
            Self::Half => other.saturating_add(1),
        }
    }
}

struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

pub struct Line {
    fragments: Vec<TextFragment>,
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
        let fragements = line_str
            .graphemes(true)
            .map(|grapheme| {
                let unicode_width = grapheme.width();
                let rendered_width = match unicode_width {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };

                let replacement = match unicode_width {
                    0 => Some('·'),
                    _ => None,
                };

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();
        Self {
            fragments: fragements,
        }
    }

    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }

        let mut result = String::new();
        let mut current_pos = 0;
        for fragmet in &self.fragments {
            let fragment_end = fragmet.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end{
                break;
            }
            if fragment_end > range.start{
                if fragment_end > range.end || current_pos < range.start{
                    result.push('⋯');
                }else if let Some(char) = fragmet.replacement{
                    result.push(char);
                }else{
                    result.push_str(&fragmet.grapheme);
                }
            }
            current_pos = fragment_end;
        }
        result
    }

    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

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
}
