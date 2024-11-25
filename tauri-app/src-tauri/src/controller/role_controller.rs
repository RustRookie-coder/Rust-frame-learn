use tauri::command;

#[command]
pub fn role_command() -> Result<Vec<Roles>, String> {
    Ok(default_items())
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Roles {
    id: i32,
    status: i32,
    name: String,
    desc: String,
    rules: Vec<Rules>,
}

impl Roles {
    #[inline]
    fn new(id: i32,
           status: i32,
           name: String,
           desc: String,
           rules: Vec<Rules>) -> Self {
        Roles {
            id,
            status,
            name,
            desc,
            rules,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Rules {
    id: i32,
    pivot: Pivot,
}

impl Rules {
    #[inline]
    fn new(id: i32, pivot: Pivot) -> Self {
        Rules {
            id,
            pivot,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Pivot {
    id: i32,
    role_id: i32,
    rule_id: i32,
}

impl Pivot {
    #[inline]
    fn new(id: i32, role_id: i32, rule_id: i32) -> Self {
        Pivot {
            id,
            role_id,
            rule_id,
        }
    }
}

fn default_items() -> Vec<Roles> {
    vec![
        Roles::new(38, 1, "测试角色".to_string(), "测试角色\nDESC".to_string(),
                   vec![Rules::new(97, Pivot::new(5235, 38, 97)),
                        Rules::new(21, Pivot::new(5236, 38, 21)),
                        Rules::new(173, Pivot::new(5237, 38, 173)),
                        Rules::new(158, Pivot::new(5273, 38, 158)),
                        Rules::new(157, Pivot::new(5274, 38, 157)),
                        Rules::new(6, Pivot::new(5275, 38, 6)),
                        Rules::new(22, Pivot::new(5276, 38, 22)),
                        Rules::new(102, Pivot::new(5277, 38, 102)),
                        Rules::new(103, Pivot::new(5278, 38, 103)),
                        Rules::new(104, Pivot::new(5279, 38, 104)),
                        Rules::new(105, Pivot::new(5280, 38, 105)),
                        Rules::new(105, Pivot::new(5280, 38, 105)),
                        Rules::new(106, Pivot::new(5281, 38, 106)),
                        Rules::new(7, Pivot::new(5282, 38, 7)),
                        Rules::new(18, Pivot::new(5283, 38, 18)),
                        Rules::new(89, Pivot::new(5284, 38, 89)),
                        Rules::new(90, Pivot::new(5285, 38, 90)),
                        Rules::new(91, Pivot::new(5286, 38, 91)),
                        Rules::new(92, Pivot::new(5287, 38, 92)),
                        Rules::new(93, Pivot::new(5288, 38, 93)),
                        Rules::new(182, Pivot::new(5289, 38, 182)),
                        Rules::new(183, Pivot::new(5290, 38, 183)),
                        Rules::new(17, Pivot::new(5291, 38, 17)),
                        Rules::new(94, Pivot::new(5292, 38, 94)),
                        Rules::new(95, Pivot::new(5293, 38, 95)),
                        Rules::new(96, Pivot::new(5294, 38, 96)),
                   ]),
        Roles::new(36, 1, "后勤".to_string(), "首页与菜单".to_string(),
                   vec![Rules::new(5, Pivot::new(5230, 36, 5)),
                        Rules::new(10, Pivot::new(5231, 36, 10)),
                        Rules::new(176, Pivot::new(5234, 36, 176)),
                        Rules::new(175, Pivot::new(5238, 36, 175)),
                   ]),
        Roles::new(35, 1, "普通管理员".to_string(), "鸿运当头666".to_string(),
                   vec![]),
        Roles::new(31, 1, "测试角色".to_string(), "测试角色\nDESC".to_string(),
                   vec![]),
        Roles::new(21, 1, "技术".to_string(), "技术测试".to_string(), vec![]),
        Roles::new(19, 1, "销售".to_string(), "销售".to_string(), vec![]),
        Roles::new(3, 1, "运营".to_string(), "上架商品等权限".to_string(),
                   vec![Rules::new(5, Pivot::new(195, 3, 5)),
                        Rules::new(10, Pivot::new(196, 3, 10)),
                       Rules::new(11, Pivot::new(197, 3, 11)),
                       Rules::new(6, Pivot::new(199, 3, 6)),
                       Rules::new(13, Pivot::new(200, 3, 13)),
                       Rules::new(14, Pivot::new(201, 3, 14)),
                       Rules::new(15, Pivot::new(202, 3, 15)),
                       Rules::new(7, Pivot::new(203, 3, 7)),
                       Rules::new(18, Pivot::new(204, 3, 18)),
                   ]),
        Roles::new(2, 1, "超级管理员".to_string(), "最高权限".to_string(),
                   vec![Rules::new(11, Pivot::new(114, 2, 11)),
                        Rules::new(5, Pivot::new(163, 2, 5)),
                        Rules::new(10, Pivot::new(164, 2, 10)),
                        Rules::new(6, Pivot::new(165, 2, 6)),
                        Rules::new(13, Pivot::new(166, 2, 13)),
                   ]),
    ]
}