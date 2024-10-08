use jsonwebtoken::errors::Error;
use rs_counter_study::utils::token::{create_token, decode_token};

#[test]
pub fn test_token_create() {
    let token = mock_create_token().unwrap();
    assert_ne!(token.len(), 0)
}

#[test]
pub fn test_token_decode() {
    let token = mock_create_token().unwrap();
    let secret = [b's', b'e', b'c', b'r', b'e', b't'];
    let res = decode_token(token, &secret).unwrap();
    assert_eq!(res, String::from("test_user"))
}

fn mock_create_token() -> Result<String, String> {
    let secret = [b's', b'e', b'c', b'r', b'e', b't'];
    let user_id = "test_user";
    let token = create_token(user_id, &secret, 60);
    if token.clone().err().is_none() { Ok(token.clone().unwrap().clone()) } else { Err(String::from("require token error")) }
}

#[test]
#[should_panic]
fn greater_than_100() {
    Guess::new(200);
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
}
