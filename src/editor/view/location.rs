use crate::editor::terminal::Position;

/// `Location` 结构体表示光标或其他元素在二维平面上的位置。
/// 包含 `x` 和 `y` 坐标，分别表示列和行。
#[derive(Copy, Clone, Default)]
pub struct Location {
    /// 光标的列位置（水平坐标）。
    pub x: usize,
    /// 光标的行位置（垂直坐标）。
    pub y: usize,
}

impl From<Location> for Position {
    /// 将 `Location` 转换为 `Position`。
    ///
    /// # 参数
    /// - `loc`: 要转换的 `Location` 实例。
    ///
    /// # 返回值
    /// 返回一个 `Position` 实例，其中 `col` 对应 `x`，`row` 对应 `y`。
    fn from(loc: Location) -> Self {
        Self {
            col: loc.x,
            row: loc.y,
        }
    }
}

impl Location {
    /// 计算两个 `Location` 之间的差值。
    ///
    /// # 参数
    /// - `other`: 要与当前 `Location` 进行比较的另一个 `Location`。
    ///
    /// # 返回值
    /// 返回一个新的 `Location`，其 `x` 和 `y` 坐标是两个位置的差值。
    /// 如果结果为负数，则使用 `saturating_sub` 将其限制为 0。
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}