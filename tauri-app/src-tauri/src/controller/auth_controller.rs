use tauri::command;
use crate::utils::token::generate_token;

#[command]
pub fn login_command(username: String, password: String) -> Result<LoginRes, String> {
    let mut token = "".to_string();
    if username == "admin" && password == "password" {
        token = generate_token(&*username, "secret".as_ref(), 15 * 24 * 60 * 60).unwrap();
    }
    Ok(LoginRes::new(username, token))
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginRes {
    username: String,
    token: String,
}

impl LoginRes {

    fn new(username: String, token: String) -> Self {
        LoginRes {
            username,
            token,
        }
    }
    fn default () -> Self {
        LoginRes {
            username: "admin".to_string(),
            token: "token".to_string()
        }
    }
}