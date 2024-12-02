use tauri::command;
use crate::model::coupons::Coupons;
use crate::utils::time::str_to_date_time;

#[command]
pub fn coupon_command() -> Result<Vec<Coupons>, String> {
    Ok(default_item())
}

fn default_item() -> Vec<Coupons> {
    vec![
        Coupons::new(218, "满2000减200".to_string(), 0,
                     "200.00".to_string(), 100, 0,
                     "100.00".to_string(), str_to_date_time("2024-11-20 16:33:11"),
                     str_to_date_time("2024-11-21 16:33:11"), 1, str_to_date_time("2024-11-26 16:33:11"),
                     str_to_date_time("2024-11-28 16:33:11"), 50, "测试".to_string()),
        Coupons::new(10, "满100减100".to_string(), 0,
                     "10.00".to_string(), 100, 0,
                     "1000.00".to_string(), str_to_date_time("2024-11-20 16:33:11"),
                     str_to_date_time("2024-11-21 16:33:11"), 1, str_to_date_time("2024-11-28 16:33:11"),
                     str_to_date_time("2024-11-28 16:33:11"), 50, "".to_string()),
        Coupons::new(7, "满100打五折".to_string(), 1,
                     "5.00".to_string(), 100, 0,
                     "100.00".to_string(), str_to_date_time("2024-11-20 16:33:11"),
                     str_to_date_time("2024-11-21 16:33:11"), 1, str_to_date_time("2024-11-28 16:33:11"),
                     str_to_date_time("2024-11-28 16:33:11"), 50, "".to_string()),
        Coupons::new(6, "满100减20元".to_string(), 0,
                     "20.00".to_string(), 100, 2,
                     "100.00".to_string(), str_to_date_time("2021-11-20 16:33:11"),
                     str_to_date_time("2021-11-21 16:33:11"), 1, str_to_date_time("2021-11-28 16:33:11"),
                     str_to_date_time("2021-11-28 16:33:11"), 50, "".to_string()),
        Coupons::new(5, "满0.05打5折".to_string(), 1,
                     "5.00".to_string(), 100, 2,
                     "0.05".to_string(), str_to_date_time("2021-11-20 16:33:11"),
                     str_to_date_time("2021-11-21 16:33:11"), 1, str_to_date_time("2021-11-28 16:33:11"),
                     str_to_date_time("2021-11-28 16:33:11"), 50, "折扣".to_string()),
        Coupons::new(4, "满0.04减0.02".to_string(), 0,
                     "5.00".to_string(), 100, 2,
                     "0.84".to_string(), str_to_date_time("2021-11-20 16:33:11"),
                     str_to_date_time("2021-11-21 16:33:11"), 1, str_to_date_time("2021-11-28 16:33:11"),
                     str_to_date_time("2021-11-28 16:33:11"), 50, "满减".to_string()),
        Coupons::new(3, "满100减20".to_string(), 0,
                     "5.00".to_string(), 100, 2,
                     "0.84".to_string(), str_to_date_time("2021-11-20 16:33:11"),
                     str_to_date_time("2021-11-21 16:33:11"), 1, str_to_date_time("2021-11-28 16:33:11"),
                     str_to_date_time("2021-11-28 16:33:11"), 50, "满减".to_string()),
    ]
}