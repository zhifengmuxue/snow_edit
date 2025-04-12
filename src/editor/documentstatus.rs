
#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus{
    pub total_lines: usize,       // 文档的总行数。
    pub current_line_index: usize,      // 当前行号。
    pub is_modified: bool,          // 文档是否被修改。
    pub file_name: String,   // 文档的文件名。
}

impl  DocumentStatus {
    /// 判断是否有修改
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified{
            String::from("(modified)")
        }else{
            String::new()
        }
    }

    /// 返回一共有多少行
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    // 返回当前所在行号
    pub fn position_indicator_to_string(&self) -> String{
        format!(
            "{}/{}",
            self.current_line_index.saturating_add(1),
            self.total_lines
        )
    }
}