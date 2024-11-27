use crate::model::goods::Skus;
use crate::utils::time::str_to_date_time;
use tauri::command;

#[command]
pub fn skus_command() -> Result<Vec<Skus>, String> {
    Ok(default_items())
}

fn default_items() -> Vec<Skus> {
    vec![Skus::new(612, "身高".to_string(), 0, 1, 1666, "160, 170, 165, 180".to_string(), str_to_date_time("2024-11-25 16:33:11"), str_to_date_time("2024-11-25 16:33:11")),
         Skus::new(160, "内存".to_string(), 0, 1, 1001, "32g, 64g, 128g".to_string(), str_to_date_time("2020-02-18 20:22:56"), str_to_date_time("2024-11-24 16:55:49")),
         Skus::new(613, "幼儿".to_string(), 0, 1, 51, "70, 80, 90, 100, 110, 120".to_string(), str_to_date_time("2024-11-25 16:33:11"), str_to_date_time("2024-11-25 16:33:11")),
         Skus::new(610, "测试1".to_string(), 0, 1, 50, "145".to_string(), str_to_date_time("2024-11-25 16:33:11"), str_to_date_time("2024-11-25 16:33:11")),
         Skus::new(158, "尺寸".to_string(), 0, 1, 50, "S, M, L, XL, XXL, XXXL".to_string(), str_to_date_time("2020-02-16 21:23:56"), str_to_date_time("2024-11-19 16:21:09")),
    ]
}