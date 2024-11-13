use tauri::command;

#[command]
pub fn rule_command() -> Result<Rules, String> {
    Ok(default_items())
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Rules {
    list: Vec<Items>,
}

impl Rules {
    #[inline]
    pub fn new(list: Vec<Items>) -> Self {
        Rules {
            list
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Items {
    id: i32,
    rule_id: i32,
    status: i32,
    name: String,
    desc: String,
    path: String,
    condition: String,
    menu: i32,
    order: i32,
    icon: String,
    method: String,
    child: Vec<Items>,
}

impl Items {
    #[inline]
    pub fn new(id: i32,
               rule_id: i32,
               status: i32,
               name: String,
               desc: String,
               path: String,
               condition: String,
               menu: i32,
               order: i32,
               icon: String,
               method: String,
               child: Vec<Items>) -> Self {
        Items {
            id,
            rule_id,
            status,
            name,
            desc,
            path,
            condition,
            menu,
            order,
            icon,
            method,
            child,
        }
    }
}

fn default_items() -> Rules {
    //主控
    let item1 = Items::new(5, 0, 1, "后台面板".to_string(), "index".to_string(), String::new(), String::new(), 1, 1, "help".to_string(), "GET".to_string(),
                           vec![Items::new(174, 10, 1, "后台面板统计1".to_string(), "".to_string(), "".to_string(), "getStatistics1".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                Items::new(175, 10, 1, "后台面板统计2".to_string(), "".to_string(), "".to_string(), "getStatistics2".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                Items::new(176, 10, 1, "后台面板统计3".to_string(), "".to_string(), "".to_string(), "getStatistics3".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                           ],
    );

    let item2 = Items::new(6, 0, 1, "商品管理".to_string(), "shop_goods_list".to_string(), String::new(), String::new(), 1, 2, "shopping-bag".to_string(), "GET".to_string(),
                           vec![Items::new(61, 13, 1, "商品列表分页".to_string(), "".to_string(), "".to_string(), "getGoodsList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                Items::new(127, 13, 1, "查看指定商品".to_string(), "".to_string(), "".to_string(), "readGoods".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                Items::new(128, 13, 1, "配置商品规格".to_string(), "".to_string(), "".to_string(), "updateGoodsSkus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(129, 13, 1, "配置商品轮播图".to_string(), "".to_string(), "".to_string(), "setGoodsBanner".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(131, 13, 1, "恢复商品".to_string(), "".to_string(), "".to_string(), "restoreGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(132, 13, 1, "彻底删除商品".to_string(), "".to_string(), "".to_string(), "destroyGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(133, 13, 1, "批量删除商品".to_string(), "".to_string(), "".to_string(), "deleteGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(134, 13, 1, "修改商品状态".to_string(), "".to_string(), "".to_string(), "updateGoodsStatus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(135, 13, 1, "创建商品".to_string(), "".to_string(), "".to_string(), "createGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(136, 13, 1, "更新商品".to_string(), "".to_string(), "".to_string(), "updateGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(137, 13, 1, "审核商品".to_string(), "".to_string(), "".to_string(), "checkGoods".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(140, 13, 1, "创建商品规格".to_string(), "".to_string(), "".to_string(), "createGoodsSkusCard".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(141, 13, 1, "排序商品规格".to_string(), "".to_string(), "".to_string(), "sortGoodsSkusCard".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(142, 13, 1, "更新商品规格".to_string(), "".to_string(), "".to_string(), "updateGoodsSkusCard".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(143, 13, 1, "删除商品规格".to_string(), "".to_string(), "".to_string(), "deleteGoodsSkusCard".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(145, 13, 1, "创建指定商品规格的值".to_string(), "".to_string(), "".to_string(), "createGoodsSkusCardValue".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(147, 13, 1, "更新指定商品规格的值".to_string(), "".to_string(), "".to_string(), "updateGoodsSkusCardValue".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                Items::new(148, 13, 1, "删除指定商品规格的值".to_string(), "".to_string(), "".to_string(), "deleteGoodsSkusCardValue".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                           ],
    );

    let item3 = Items::new(173, 0, 1, "用户管理".to_string(), "".to_string(), String::new(), String::new(), 1, 3, "user".to_string(), "GET".to_string(),
                           vec![Items::new(21, 173, 1, "用户管理".to_string(), "user_user-list_list".to_string(), "/user/list".to_string(), "".to_string(), 1, 20, "".to_string(), "GET".to_string(), vec![
                               Items::new(97, 21, 1, "会员列表".to_string(), "".to_string(), "".to_string(), "getUserList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(98, 21, 1, "创建会员".to_string(), "".to_string(), "".to_string(), "createUser".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(99, 21, 1, "更新会员".to_string(), "".to_string(), "".to_string(), "updateUser".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(100, 21, 1, "更新会员状态".to_string(), "".to_string(), "".to_string(), "updateUserStatus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(101, 21, 1, "删除会员".to_string(), "".to_string(), "".to_string(), "deleteUser".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                           ]),
                                Items::new(22, 173, 1, "会员等级".to_string(), "user_user-level_list".to_string(), "/level/list".to_string(), "".to_string(), 1, 20, "data-analysis".to_string(), "GET".to_string(), vec![
                                    Items::new(102, 22, 1, "会员等级列表".to_string(), "".to_string(), "".to_string(), "getUserLevelList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(103, 22, 1, "创建会员等级".to_string(), "".to_string(), "".to_string(), "createUserLevel".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(104, 22, 1, "更新会员等级".to_string(), "".to_string(), "".to_string(), "updateUserLevel".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(105, 22, 1, "更新会员等级状态".to_string(), "".to_string(), "".to_string(), "updateUserLevelStatus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(106, 22, 1, "删除会员等级".to_string(), "".to_string(), "".to_string(), "deleteUserLevel".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                ]),
                                Items::new(176, 10, 1, "后台面板统计3".to_string(), "".to_string(), "".to_string(), "getStatistics3".to_string(), 1, 20, "".to_string(), "GET".to_string(), vec![]),
                           ],
    );

    let item4 = Items::new(7, 0, 1, "订单管理".to_string(), "order_order_list".to_string(), String::new(), String::new(), 1, 4, "message-box".to_string(), "GET".to_string(),
                           vec![Items::new(18, 7, 1, "订单管理".to_string(), "order_order_list".to_string(), "/order/list".to_string(), "".to_string(), 1, 1, "reading".to_string(), "GET".to_string(), vec![
                               Items::new(89, 18, 1, "订单列表".to_string(), "".to_string(), "".to_string(), "getOrderList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(90, 18, 1, "批量删除订单".to_string(), "".to_string(), "".to_string(), "deleteOrder".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(91, 18, 1, "订单发货".to_string(), "".to_string(), "".to_string(), "shipOrder".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(92, 18, 1, "拒绝/同意退货".to_string(), "".to_string(), "".to_string(), "refundOrder".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(93, 18, 1, "导出订单".to_string(), "".to_string(), "".to_string(), "exportOrder".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(182, 18, 1, "查看物流信息".to_string(), "".to_string(), "".to_string(), "getShipInfo".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(183, 18, 1, "获取快递列表".to_string(), "".to_string(), "".to_string(), "getExpressCompanyList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                           ]),
                                Items::new(17, 7, 1, "评论管理".to_string(), "shop_comment_list".to_string(), "/comment/list".to_string(), "".to_string(), 0, 50, "comment".to_string(), "GET".to_string(), vec![
                                    Items::new(94, 17, 1, "评论列表".to_string(), "".to_string(), "".to_string(), "getCommentList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(95, 17, 1, "回复评论".to_string(), "".to_string(), "".to_string(), "reviewComment".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(96, 17, 1, "更新评论状态".to_string(), "".to_string(), "".to_string(), "updateCommentStatus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                ]),
                                Items::new(176, 10, 1, "后台面板统计3".to_string(), "/comment/list".to_string(), "".to_string(), "getStatistics3".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                           ],
    );
    let item5 = Items::new(8, 0, 1, "管理员管理".to_string(), "user_user-list_list".to_string(), String::new(), String::new(), 1, 5, "management".to_string(), "GET".to_string(),
                           vec![Items::new(25, 8, 1, "管理员列表".to_string(), "set_manager".to_string(), "/manager/list".to_string(), "".to_string(), 1, 20, "coordinate".to_string(), "GET".to_string(), vec![
                               Items::new(108, 25, 1, "删除管理员".to_string(), "".to_string(), "".to_string(), "deleteManager".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(109, 25, 1, "管理员列表".to_string(), "".to_string(), "".to_string(), "getManagerList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(110, 25, 1, "创建管理员".to_string(), "".to_string(), "".to_string(), "createManager".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(111, 25, 1, "更新管理员".to_string(), "".to_string(), "".to_string(), "updateManager".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(112, 25, 1, "更新管理员状态".to_string(), "".to_string(), "".to_string(), "updateManagerStatus".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                           ]),
                                Items::new(27, 8, 1, "权限管理".to_string(), "".to_string(), "/access/list".to_string(), "".to_string(), 1, 20, "connection".to_string(), "GET".to_string(), vec![
                                    Items::new(28, 27, 1, "添加规则".to_string(), "".to_string(), "".to_string(), "createRule".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(29, 27, 1, "更新规则".to_string(), "".to_string(), "".to_string(), "updateRule".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(30, 27, 1, "删除规则".to_string(), "".to_string(), "".to_string(), "deleteRule".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(32, 27, 1, "更新规则状态".to_string(), "".to_string(), "".to_string(), "updateRuleStatus".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(31, 27, 1, "规则列表".to_string(), "".to_string(), "".to_string(), "getRuleList".to_string(), 0, 20, "".to_string(), "GET".to_string(), vec![]),
                                ]),
                                Items::new(33, 8, 1, "角色管理".to_string(), "".to_string(), "/role/list".to_string(), "".to_string(), 1, 20, "histogram".to_string(), "GET".to_string(), vec![
                                    Items::new(34, 33, 1, "添加角色".to_string(), "".to_string(), "".to_string(), "createRole".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(35, 33, 1, "更新角色".to_string(), "".to_string(), "".to_string(), "updateRole".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(36, 33, 1, "删除角色".to_string(), "".to_string(), "".to_string(), "deleteRole".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(37, 33, 1, "角色列表".to_string(), "".to_string(), "".to_string(), "getRoleList".to_string(), 0, 20, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(38, 33, 1, "更新角色状态".to_string(), "".to_string(), "".to_string(), "updateRoleStatus".to_string(), 0, 20, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(180, 33, 1, "给角色配置权限".to_string(), "".to_string(), "".to_string(), "setRoleRules".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                ]),
                           ],
    );
    let item6 = Items::new(9, 0, 1, "系统设置".to_string(), "set_base".to_string(), String::new(), String::new(), 1, 6, "setting".to_string(), "GET".to_string(),
                           vec![Items::new(23, 9, 1, "基础设置".to_string(), "set_base".to_string(), "/setting/base".to_string(), "".to_string(), 1, 19, "baseball".to_string(), "GET".to_string(), vec![
                               Items::new(121, 23, 1, "获取配置信息".to_string(), "".to_string(), "".to_string(), "getSysSetting".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(123, 23, 1, "设置配置信息".to_string(), "".to_string(), "".to_string(), "setSysSetting".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                           ]),
                                Items::new(26, 9, 1, "交易设置".to_string(), "set_payment".to_string(), "/setting/buy".to_string(), "getStatistics2".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![
                                    Items::new(122, 26, 1, "上传相关配置文件".to_string(), "".to_string(), "".to_string(), "sysconfigUpload".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(124, 26, 1, "获取配置".to_string(), "".to_string(), "".to_string(), "getSysSetting".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(125, 26, 1, "设置配置".to_string(), "".to_string(), "".to_string(), "setSysSetting".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                ]),
                                Items::new(24, 9, 1, "物流设置".to_string(), "set_express".to_string(), "/setting/ship".to_string(), "".to_string(), 1, 21, "bicycle".to_string(), "GET".to_string(), vec![]),
                           ],
    );
    let item7 = Items::new(177, 0, 1, "分销模块".to_string(), "".to_string(), String::new(), String::new(), 1, 7, "shopping-cart".to_string(), "GET".to_string(),
                           vec![Items::new(178, 177, 1, "分销员管理".to_string(), "".to_string(), "/distribution/index".to_string(), "".to_string(), 1, 50, "user-filled".to_string(), "GET".to_string(), vec![
                               Items::new(342, 178, 1, "分销数据统计".to_string(), "".to_string(), "".to_string(), "getAgentStatistics".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(343, 178, 1, "分销推广员列表".to_string(), "".to_string(), "".to_string(), "getAgentList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(344, 178, 1, "推广订单列表".to_string(), "".to_string(), "".to_string(), "getUserBillList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                           ]),
                                Items::new(179, 177, 1, "分销设置".to_string(), "".to_string(), "/distribution/setting".to_string(), "".to_string(), 1, 50, "set-up".to_string(), "GET".to_string(), vec![
                                    Items::new(345, 179, 1, "获取分销配置".to_string(), "".to_string(), "".to_string(), "getDistributionSetting".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(346, 179, 1, "修改分销配置".to_string(), "".to_string(), "".to_string(), "setDistributionSetting".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                ]),
                           ],
    );
    let item8 = Items::new(172, 0, 1, "其他模块".to_string(), "".to_string(), String::new(), String::new(), 1, 8, "mostly-cloudy".to_string(), "GET".to_string(),
                           vec![Items::new(11, 172, 1, "图库管理".to_string(), "image".to_string(), "/image/list".to_string(), "".to_string(), 1, 20, "picture-filled".to_string(), "GET".to_string(), vec![
                               Items::new(62, 11, 1, "指定图库下的图片列表".to_string(), "".to_string(), "".to_string(), "getCurrentImageList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(63, 11, 1, "图库列表".to_string(), "".to_string(), "".to_string(), "getImageClassList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                               Items::new(64, 11, 1, "创建图库".to_string(), "".to_string(), "".to_string(), "createImageClass".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(65, 11, 1, "更新图库".to_string(), "".to_string(), "".to_string(), "updateImageClass".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(66, 11, 1, "删除图库".to_string(), "".to_string(), "".to_string(), "deleteImageClass".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(67, 11, 1, "上传图片".to_string(), "".to_string(), "".to_string(), "uploadImage".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(68, 11, 1, "批量删除图片".to_string(), "".to_string(), "".to_string(), "deleteImage".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                               Items::new(69, 11, 1, "更新图片".to_string(), "".to_string(), "".to_string(), "updateImage".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                           ]),
                                Items::new(149, 172, 1, "公告管理".to_string(), "set_notice".to_string(), "/notice/list".to_string(), "".to_string(), 1, 50, "".to_string(), "GET".to_string(), vec![
                                    Items::new(150, 149, 1, "公告列表".to_string(), "".to_string(), "".to_string(), "getNoticeList".to_string(), 0, 50, "".to_string(), "GET".to_string(), vec![]),
                                    Items::new(151, 149, 1, "创建公告".to_string(), "".to_string(), "".to_string(), "createNotice".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(152, 149, 1, "更新公告".to_string(), "".to_string(), "".to_string(), "updateNotice".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                    Items::new(153, 149, 1, "删除公告".to_string(), "".to_string(), "".to_string(), "deleteNotice".to_string(), 0, 50, "".to_string(), "POST".to_string(), vec![]),
                                ]),
                           ],
    );
    Rules::new(vec![item1, item2, item3, item4, item5, item6, item7, item8])
}