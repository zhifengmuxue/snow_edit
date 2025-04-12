use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub path: Option<PathBuf>, // 文件路径
}

impl FileInfo {
    /// 构造方法
    pub fn from(file_name: &str) -> Self {
        Self {
            path: Some(PathBuf::from(file_name)),
        }
    }
}

impl Display for FileInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]");
        write!(formatter, "{name}")
    }
}
