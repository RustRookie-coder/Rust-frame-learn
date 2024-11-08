use tauri::command;

#[command]
pub fn manage_command(search: String) -> Result<Vec<Manager>, String> {
    let mut res: Vec<Manager> = mock_manage_data();
    if search.is_empty() {}
    Ok(res)
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Manager {
    id: u64,
    status: i32,
    username: String,
    avatar: String,
    status_loading: bool,
    is_super: i32,
    role_id: u64,
    role: Role,
}

impl Manager {
    fn new(id: u64, status: i32, username: String,
           avatar: String,
           status_loading: bool,
           is_super: i32,
           role_id: u64,
           role: Role, ) -> Self {
        Manager {
            id,
            status,
            username,
            avatar,
            status_loading,
            is_super,
            role_id,
            role,
        }
    }
    fn default() -> Self {
        Manager {
            id: 0,
            status: 0,
            username: "".to_string(),
            avatar: "".to_string(),
            status_loading: false,
            is_super: 0,
            role_id: 0,
            role: Role::new(1, "".to_string()),
        }
    }
}

fn mock_manage_data() -> Vec<Manager> {
    let mut res: Vec<Manager> = Vec::new();
    let super_manager = Manager::new(50, 1, "super-admin".to_string(),
                                     "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e90c768.jpg".to_string(),
                                     false, 0, 2, Role::new(2, "超级管理员".to_string()));
    let normal_manager = Manager::new(51, 1, "admin".to_string(),
                                      "http://tangzhe123-com.oss-cn-shenzhen.aliyuncs.com/public/6291c4e8a6a04.jpg".to_string(),
                                      true, 1, 3, Role::new(3, "管理员".to_string()));
    res.push(super_manager);
    res.push(normal_manager);
    res
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Role {
    id: u64,
    name: String,
}

impl Role {
    fn new(id: u64, name: String) -> Self {
        Role {
            id,
            name,
        }
    }
}