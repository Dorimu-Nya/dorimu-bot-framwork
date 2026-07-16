use serde::{Deserialize, Serialize};

// 放一些各个事件通用的模型

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub content_type: Option<String>,
    pub filename: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub size: Option<i64>,
    pub url: Option<String>,
}
