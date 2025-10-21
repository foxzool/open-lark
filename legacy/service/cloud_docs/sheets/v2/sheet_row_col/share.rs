use serde::Serialize;

/// 需要插入行列的维度信息
#[derive(Serialize, Default, Debug)]
pub(crate) struct UpdateDimension {
    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    pub(crate) sheet_id: String,
    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    #[serde(rename = "majorDimension")]
    pub(crate) major_dimension: String,
    /// 插入的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4
    /// 行或列开始插入空行或列。包含第 4 行或列。
    #[serde(rename = "startIndex")]
    pub(crate) start_index: i32,
    /// 插入的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则从第 8 行结束插入行。第 8
    /// 行不再插入空行。 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7
    /// 时，则在第 4、5、6、7 行插入空白行，共插入 4 行。
    #[serde(rename = "endIndex")]
    pub(crate) end_index: i32,
}
