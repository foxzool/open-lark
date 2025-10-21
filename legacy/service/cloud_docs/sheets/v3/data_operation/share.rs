use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub(crate) struct FindCondition {
    /// 查找范围，参考 名词解释 Range
    pub(crate) range: String,
    /// 是否忽略大小写，默认为 false
    ///
    /// - true：表示忽略字符串中字母大小写差异
    /// - false：表示区分字符串中字母大小写
    pub(crate) match_case: Option<bool>,
    /// 是否完全匹配整个单元格，默认值为 false
    ///
    /// - true：表示完全匹配单元格，比如 find 取值为 "hello"，则单元格中的内容必须为 "hello"
    /// - false：表示允许部分匹配单元格，比如 find 取值为 "hello"，则单元格中的内容包含 "hello"
    ///   即可
    pub(crate) match_entire_cell: Option<bool>,
    /// 是否为正则匹配，默认值为 false
    ///
    /// - true：表示使用正则匹配
    /// - false：表示不使用正则匹配
    pub(crate) search_by_regex: Option<bool>,
    ///
    // 是否仅搜索单元格公式，默认值为 false
    /// - true：表示仅搜索单元格公式
    /// - false：表示仅搜索单元格内容
    pub(crate) include_formulas: Option<bool>,
}

/// 符合条件的信息
#[derive(Deserialize, Debug)]
pub struct FindReplaceResult {
    /// 符合查找条件的单元格数组，不包含公式
    pub matched_cells: Vec<String>,
    /// 符合查找条件的含有公式的单元格数组
    pub matched_formula_cells: Vec<String>,
    /// 符合查找条件的总行数
    pub rows_count: i32,
}
