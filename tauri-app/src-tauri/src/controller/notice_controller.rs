use tauri::command;

#[command]
pub fn notice_command() -> Result<Vec<Notice>, String> {
    let mut res = mock_notice_data();
    Ok(res)
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Notice {
    id: i32,
    title: String,
    content: String,
    order: i32,
}

impl Notice {
    #[inline]
    pub fn new(id: i32,
               title: String,
               content: String,
               order: i32) -> Self {
        Notice {
            id,
            title,
            content,
            order,
        }
    }
}

fn mock_notice_data() -> Vec<Notice> {
    let mut mock_data = Vec::new();
    mock_data.push(Notice::new(1, "测试公告".to_string(), "测试公告是否运行成功".to_string(), 10));
    mock_data.push(Notice::new(2, "测试公告2".to_string(), "测试公告是否运行成功2".to_string(), 11));
    mock_data
}
