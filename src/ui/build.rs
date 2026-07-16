use super::SYNC_CARD_ID;
use super::event_handler::*;
use super::icons;
use super::state::*;
use crate::astrobox::psys_host;
use crate::astrobox::psys_host::ui_v3 as ui;

/// 渲染主UI
pub fn render_main_ui(element_id: &str) {
    {
        let mut state = ui_state()
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.root_element_id = Some(element_id.to_string());
    }

    crate::ui::state::load_api_settings_once();

    let ui_tree = build_main_ui();
    psys_host::ui_v3::render(element_id, ui_tree);
}

/// 构建主UI
pub fn build_main_ui() -> ui::Element {
    let state = ui_state()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let container = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .padding(20);

    let tabs = build_tabs(&state);
    let content = match state.current_tab {
        MainTab::SyncData => build_sync_tab(&state),
        MainTab::CityManage => build_city_manage_tab(&state),
        MainTab::Settings => build_settings_tab(&state),
    };

    container.child(tabs).child(content)
}

/// 渲染同步卡片
pub fn render_sync_card(card_id: &str) {
    tracing::info!("render_sync_card called: card_id={}", card_id);
    if card_id != SYNC_CARD_ID {
        tracing::info!(
            "render_sync_card id mismatch: expected {}, got {}",
            SYNC_CARD_ID,
            card_id
        );
    }
    let text = build_sync_card_text();
    tracing::info!("render_sync_card content: {}", text);
    psys_host::ui_v3::render_to_text_card(card_id, &text);
}

// ========== Tab 构建函数 ==========

fn build_tabs(state: &UiState) -> ui::Element {
    let tabs_root = ui::Element::new(ui::ElementType::TabsRoot, None)
        .flex()
        .justify_center()
        .margin_bottom(20);

    let tabs_list = ui::Element::new(ui::ElementType::TabsList, None)
        .flex()
        .bg("#1E1E1F")
        .radius(999)
        .padding(4)
        .gap(4);

    let sync_trigger = build_tab_trigger(
        "同步数据",
        icons::send_tab_svg(),
        state.current_tab == MainTab::SyncData,
        TAB_SYNC_EVENT,
    );

    let city_trigger = build_tab_trigger(
        "城市管理",
        icons::location_pin_svg(),
        state.current_tab == MainTab::CityManage,
        TAB_CITY_EVENT,
    );

    let settings_trigger = build_tab_trigger(
        "设置",
        icons::api_tab_svg(),
        state.current_tab == MainTab::Settings,
        TAB_SETTINGS_EVENT,
    );

    tabs_root
        .child(tabs_list.child(sync_trigger).child(city_trigger).child(settings_trigger))
}

fn build_tab_trigger(label: &str, icon_svg: String, is_active: bool, event_id: &str) -> ui::Element {
    let icon = ui::Element::new(ui::ElementType::Svg, Some(&icon_svg))
        .width(22)
        .height(22);

    let text = ui::Element::new(ui::ElementType::Span, Some(label)).size(14);

    ui::Element::new(ui::ElementType::TabsTrigger, None)
        .without_default_styles()
        .on(ui::Event::Click, event_id)
        .radius(999)
        .padding_top(10)
        .padding_bottom(10)
        .padding_left(14)
        .padding_right(14)
        .bg(if is_active { "#2A2A2A" } else { "#1E1E1F" })
        .text_color(if is_active { "#FFFFFF" } else { "#BBBBBB" })
        .flex()
        .align_center()
        .gap(5)
        .child(icon)
        .child(text)
}

// ========== 同步数据Tab ==========

fn build_sync_tab(state: &UiState) -> ui::Element {
    if !state.api_key_verified {
        return build_verification_ui(state);
    }

    build_weather_sync_ui(state)
}

fn build_verification_ui(state: &UiState) -> ui::Element {
    let status_text = match state.verification_status {
        VerificationStatus::NotStarted => "尚未验证设备，请点击下方按钮开始验证",
        VerificationStatus::CheckingDevice => "正在检测设备连接...",
        VerificationStatus::GettingAPIKey => "正在获取APIKey...",
        VerificationStatus::GettingDeviceInfo => "正在获取设备信息...",
        VerificationStatus::WaitingPayment => "请完成付款后点击检查授权",
        VerificationStatus::VerifyingPayment => "正在验证付款...",
        VerificationStatus::Verified => "已验证",
        VerificationStatus::Failed => "验证失败，请重试",
    };

    let root = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .gap(12);

    let status_label = ui::Element::new(ui::ElementType::P, Some(status_text))
        .size(14)
        .text_color("#BBBBBB");

    // 根据状态显示不同的按钮
    let mut root = root.child(status_label);

    match state.verification_status {
        VerificationStatus::NotStarted | VerificationStatus::Failed => {
            // 初始状态或失败状态：显示开始验证按钮
            let verify_button = build_icon_text_button_full(
                "开始验证",
                icons::api_tab_svg(),
                UPGRADE_TO_PAID_EVENT,
            ).bg("#0090FF26").text_color("#0090FF");
            root = root.child(verify_button);
        }
        VerificationStatus::WaitingPayment => {
            // 等待付款状态：显示检查付款状态按钮
            let check_button = build_icon_text_button_full(
                "检查付款状态",
                icons::refresh_svg(),
                CHECK_PAYMENT_EVENT,
            ).bg("#0090FF26").text_color("#0090FF");
            root = root.child(check_button);
        }
        _ => {
            // 其他状态（检测中、获取中、验证中）：不显示按钮
        }
    }

    root
}

fn build_weather_sync_ui(state: &UiState) -> ui::Element {
    let root = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .gap(8);

    // 选择城市下拉菜单
    let city_label = ui::Element::new(ui::ElementType::P, Some("选择城市"))
        .size(15)
        .margin_bottom(8);

    let selected_city_name = if let Some(idx) = state.selected_city_index {
        if idx < state.city_list.len() {
            let city = &state.city_list[idx];
            if city.adm1.is_empty() {
                city.name.clone()
            } else {
                format!("{} · {}", city.name, city.adm1)
            }
        } else {
            "请选择城市".to_string()
        }
    } else {
        "请选择城市".to_string()
    };

    let mut city_select = ui::Element::new(ui::ElementType::Select, Some(&selected_city_name))
        .on(ui::Event::Change, SELECT_CITY_DROPDOWN_EVENT)
        .radius(12)
        .padding(12)
        .bg("#1E1E1F")
        .width_full();

    if state.city_list.is_empty() {
        let option = ui::Element::new(ui::ElementType::Option, Some("请先添加城市"));
        city_select = city_select.child(option);
    } else {
        for city in &state.city_list {
            let label = if city.adm1.is_empty() {
                city.name.clone()
            } else {
                format!("{} · {}", city.name, city.adm1)
            };
            let option = ui::Element::new(ui::ElementType::Option, Some(&label));
            city_select = city_select.child(option);
        }
    }

    let days_card = build_days_card(state).margin_top(10);

    let hourly_card = build_settings_card(
        icons::hourly_sync_svg(),
        "同步逐小时天气数据",
        Some("开启后同步最近一周逐小时天气"),
        Some(build_switch(state.sync_hourly_enabled, HOURLY_SYNC_TOGGLE_EVENT)),
        None,
    );

    let alerts_card = build_settings_card(
        icons::alerts_svg(),
        "同步天气预警数据",
        Some("开启后同步天气预警灾害信息"),
        Some(build_switch(state.sync_alerts_enabled, ALERTS_SYNC_TOGGLE_EVENT)),
        None,
    ).margin_bottom(10);

    let send_button = build_icon_text_button_full(
        "同步数据",
        icons::send_tab_svg(),
        SEND_BUTTON_EVENT,
    ).bg("#0090FF26").text_color("#0090FF");

    root.child(city_label)
        .child(city_select)
        .child(days_card)
        .child(hourly_card)
        .child(alerts_card)
        .child(send_button)
}

// ========== 城市管理Tab ==========

fn build_city_manage_tab(state: &UiState) -> ui::Element {
    let root = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .gap(8);

    // 添加城市按钮
    let add_city_button = build_icon_text_button_full(
        "添加城市",
        icons::location_pin_svg(),
        "add_city",
    ).bg("#0090FF26").text_color("#0090FF");

    // 城市列表标题行（包含刷新按钮）
    let list_title = ui::Element::new(ui::ElementType::P, Some("城市列表"))
        .size(16)
        .flex_shrink(0.0); // 防止被压缩

    let spacer = ui::Element::new(ui::ElementType::Div, None)
        .flex_grow(1.0); // 占据剩余空间

    let refresh_button = ui::Element::new(ui::ElementType::Button, Some("刷新"))
        .without_default_styles()
        .on(ui::Event::Click, GET_CITYLIST_EVENT)
        .bg("#2A2A2A")
        .text_color("#FFFFFF")
        .radius(8)
        .padding_left(12)
        .padding_right(12)
        .padding_top(6)
        .padding_bottom(6)
        .size(14)
        .flex_shrink(0.0); // 防止被压缩

    let list_header = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Row)
        .align_center()
        .width_full()
        .margin_top(10)
        .gap(8)
        .child(list_title)
        .child(spacer)
        .child(refresh_button);

    // 城市列表
    let city_list_container = if state.city_list.is_empty() {
        ui::Element::new(ui::ElementType::P, Some("暂无城市，请先添加"))
            .size(14)
            .text_color("#888888")
            .margin_top(10)
    } else {
        let mut container = ui::Element::new(ui::ElementType::Div, None)
            .flex()
            .flex_direction(ui::FlexDirection::Column)
            .gap(8)
            .margin_top(10);

        for (idx, city) in state.city_list.iter().enumerate() {
            let city_item = build_city_item(city, idx, state.selected_city_index == Some(idx));
            container = container.child(city_item);
        }
        container
    };

    root.child(add_city_button)
        .child(list_header)
        .child(city_list_container)
}

fn build_city_item(city: &CityInfo, idx: usize, is_selected: bool) -> ui::Element {
    let item = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .bg("#1E1E1F")
        .radius(12)
        .padding(12);

    // 第一行: 城市名 + 排序按钮
    let row1 = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Row)
        .align_center()
        .width_full();

    let name_text = ui::Element::new(ui::ElementType::P, Some(&city.name))
        .size(15)
        .text_color(if is_selected { "#0090FF" } else { "#FFFFFF" });

    let spacer1 = ui::Element::new(ui::ElementType::Div, None).width_full();

    // 排序按钮
    let order_up = ui::Element::new(ui::ElementType::Button, Some("↑"))
        .without_default_styles()
        .on(ui::Event::Click, &format!("{}{},-1", CITY_ORDER_PREFIX, idx))
        .bg("#2A2A2A")
        .text_color("#FFFFFF")
        .radius(6)
        .padding_left(10)
        .padding_right(10)
        .padding_top(4)
        .padding_bottom(4)
        .size(12);

    let order_down = ui::Element::new(ui::ElementType::Button, Some("↓"))
        .without_default_styles()
        .on(ui::Event::Click, &format!("{}{},1", CITY_ORDER_PREFIX, idx))
        .bg("#2A2A2A")
        .text_color("#FFFFFF")
        .radius(6)
        .margin_left(4)
        .padding_left(10)
        .padding_right(10)
        .padding_top(4)
        .padding_bottom(4)
        .size(12);

    // 第二行: adm1 adm2 + 删除按钮
    let row2 = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Row)
        .align_center()
        .width_full()
        .margin_top(8);

    let adm_text = if city.adm1.is_empty() {
        "".to_string()
    } else if city.adm2.is_empty() {
        city.adm1.clone()
    } else {
        format!("{} {}", city.adm1, city.adm2)
    };

    let adm_label = ui::Element::new(ui::ElementType::P, Some(&adm_text))
        .size(13)
        .text_color("#888888");

    let spacer2 = ui::Element::new(ui::ElementType::Div, None).width_full();

    let delete_btn = ui::Element::new(ui::ElementType::Button, Some("删除"))
        .without_default_styles()
        .on(ui::Event::Click, &format!("{}{}", DELETE_CITY_PREFIX, idx))
        .bg("#FF4444")
        .text_color("#FFFFFF")
        .radius(6)
        .padding_left(10)
        .padding_right(10)
        .padding_top(4)
        .padding_bottom(4)
        .size(12);

    item.child(row1.child(name_text).child(spacer1).child(order_up).child(order_down))
        .child(row2.child(adm_label).child(spacer2).child(delete_btn))
}

// ========== 设置Tab ==========

fn build_settings_tab(state: &UiState) -> ui::Element {
    let root = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .gap(8);

    // 授权状态标题
    let auth_title = build_section_title("授权状态");

    // APIKey状态卡片（显示验证状态和APIKey）
    let api_key_card = build_apikey_status_card(state);

    // 从服务器获取的设备信息
    let mut info_cards = Vec::new();

    if let Some(ref info) = state.server_device_info {
        // userType
        if let Some(user_type) = info.get("userType").and_then(|v| v.as_str()) {
            // 如果是免费版，显示升级按钮
            if user_type == "free" {
                let upgrade_card = build_settings_card(
                    icons::afd_svg(),
                    "升级为付费版",
                    Some("享受更多请求额度"),
                    Some(build_more_link_icon()),
                    Some(UPGRADE_TO_PAID_EVENT),
                );
                info_cards.push(upgrade_card);
            }
        }

        // billingMode
        if let Some(billing_mode) = info.get("billingMode").and_then(|v| v.as_str()) {
            let mode_card = build_settings_card(
                icons::hash_svg(),
                "计费模式",
                Some(billing_mode),
                None,
                None,
            );
            info_cards.push(mode_card);
        }

        // 到期时间（订阅制）
        if let Some(expired_at) = info.get("expiredAt").and_then(|v| v.as_str()) {
            let expire_card = build_settings_card(
                icons::calendar_svg(),
                "到期时间",
                Some(expired_at),
                None,
                None,
            );
            info_cards.push(expire_card);
        }

        // 剩余金额（按量制）
        if let Some(remaining) = info.get("remainingAmount").and_then(|v| v.as_str()) {
            let remain_card = build_settings_card(
                icons::afd_svg(),
                "剩余金额",
                Some(&format!("{} 元", remaining)),
                None,
                None,
            );
            info_cards.push(remain_card);
        }
    }

    // 刷新设备信息按钮
    let refresh_button = build_icon_text_button_full(
        "刷新授权信息",
        icons::refresh_svg(),
        REFRESH_DEVICE_INFO_EVENT,
    );

    // 更多内容
    let more_title = build_section_title("更多内容");

    let help_card = build_settings_card(
        icons::help_svg(),
        "帮助文档",
        Some("操作步骤与常见问题解答"),
        Some(build_more_link_icon()),
        Some(OPEN_HELP_DOC_EVENT),
    );

    let qq_card = build_settings_card(
        icons::qq_group_svg(),
        "QQ交流群",
        Some("1076096725"),
        Some(build_more_link_icon()),
        Some(OPEN_QQ_GROUP_EVENT),
    );

    // 构建信息
    let build_title = build_section_title("构建信息");

    let build_time_raw = option_env!("AB_BUILD_TIME").unwrap_or("unknown");
    let build_user = option_env!("AB_BUILD_USER").unwrap_or("unknown");
    let build_branch = option_env!("AB_BUILD_GIT_BRANCH").unwrap_or("unknown");
    let build_hash = short_git_hash(option_env!("AB_BUILD_GIT_HASH").unwrap_or("unknown"));
    let build_time = format_beijing_time(build_time_raw);

    let build_time_row = build_settings_card(
        icons::time_svg(),
        "构建时间",
        None,
        Some(build_value_text(&build_time)),
        None,
    );
    let build_user_row = build_settings_card(
        icons::user_svg(),
        "构建用户",
        None,
        Some(build_value_text(build_user)),
        None,
    );
    let build_branch_row = build_settings_card(
        icons::branch_svg(),
        "当前分支",
        None,
        Some(build_value_text(build_branch)),
        None,
    );
    let build_hash_row = build_settings_card(
        icons::hash_svg(),
        "当前hash",
        None,
        Some(build_value_text(&build_hash)),
        None,
    );

    let mut root = root
        .child(auth_title)
        .child(api_key_card);

    for card in info_cards {
        root = root.child(card);
    }

    root.child(refresh_button)
        .child(more_title)
        .child(help_card)
        .child(qq_card)
        .child(build_title)
        .child(build_time_row)
        .child(build_user_row)
        .child(build_branch_row)
        .child(build_hash_row)
}

/// 构建APIKey状态卡片
fn build_apikey_status_card(state: &UiState) -> ui::Element {
    let container = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .bg("#1E1E1F")
        .radius(12)
        .padding(12)
        .gap(10);

    // 第一行：APIKey状态 + 已验证/未验证
    let row1 = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Row)
        .align_center()
        .width_full()
        .gap(8);

    let status_label = ui::Element::new(ui::ElementType::P, Some("APIKey状态"))
        .size(15)
        .flex_shrink(0.0); // 防止被压缩

    let spacer1 = ui::Element::new(ui::ElementType::Div, None)
        .flex_grow(1.0); // 占据剩余空间

    let (status_text, status_color) = if state.api_key_verified {
        ("已验证", "#00FF00") // 绿色
    } else {
        ("未验证", "#FF4444") // 红色
    };

    let status_value = ui::Element::new(ui::ElementType::P, Some(status_text))
        .size(15)
        .text_color(status_color)
        .flex_shrink(0.0); // 防止被压缩

    // 第二行：APIKey显示/隐藏（可点击切换）
    let row2 = if state.api_key_verified && !state.api_key.is_empty() {
        // 根据可见状态显示APIKey或星号
        let api_key_display = if state.api_key_visible {
            state.api_key.clone() // 直接显示原始APIKey
        } else {
            // 32位连续星号
            "********************************".to_string()
        };

        let hint_text = if state.api_key_visible {
            "点击以隐藏APIKey,请勿传播/分享"
        } else {
            "点击以显示APIKey,请勿传播/分享"
        };

        let api_key_text = ui::Element::new(ui::ElementType::P, Some(&api_key_display))
            .size(13)
            .text_color("#888888")
            .margin_bottom(4);

        let hint_label = ui::Element::new(ui::ElementType::P, Some(hint_text))
            .size(12)
            .text_color("#666666");

        // 整个区域可点击
        ui::Element::new(ui::ElementType::Div, None)
            .flex()
            .flex_direction(ui::FlexDirection::Column)
            .width_full()
            .on(ui::Event::Click, TOGGLE_APIKEY_VISIBLE_EVENT)
            .child(api_key_text)
            .child(hint_label)
    } else {
        ui::Element::new(ui::ElementType::Div, None)
    };

    // 第三行：请求用量 + 进度条（仅当有数据时显示）
    let row3 = if let Some(ref info) = state.server_device_info {
        // 买断制显示请求用量
        if let Some(all_req_str) = info.get("ALLRequests").and_then(|v| v.as_str()) {
            let used_req = info.get("UseRequests").and_then(|v| v.as_str()).unwrap_or("0");
            let all_req: f64 = all_req_str.parse().unwrap_or(0.0);
            let used: f64 = used_req.parse().unwrap_or(0.0);

            let usage_text = format!("请求用量：{} / {}", used_req, all_req_str);

            let usage_row = ui::Element::new(ui::ElementType::Div, None)
                .flex()
                .flex_direction(ui::FlexDirection::Row)
                .align_center()
                .width_full()
                .margin_top(4);

            let usage_label = ui::Element::new(ui::ElementType::P, Some(&usage_text))
                .size(13)
                .text_color("#BBBBBB");

            // 进度条容器（使用flex布局）
            let progress_container = ui::Element::new(ui::ElementType::Div, None)
                .flex()
                .width_full()
                .height(6)
                .bg("#2A2A2A")
                .radius(3)
                .margin_top(6);

            // 进度条前景（使用flex-grow来控制宽度比例）
            let remaining_percent = if all_req > 0.0 { 100.0 - (used / all_req * 100.0) } else { 0.0 };
            let used_flex = if all_req > 0.0 { (used / all_req * 100.0) as f32 } else { 0.0 };
            let remaining_flex = remaining_percent as f32;

            let progress_bar = ui::Element::new(ui::ElementType::Div, None)
                .flex()
                .flex_grow(used_flex)
                .height_full()
                .bg("#0090FF")
                .radius(3);

            let progress_remaining = ui::Element::new(ui::ElementType::Div, None)
                .flex()
                .flex_grow(remaining_flex)
                .height_full();

            ui::Element::new(ui::ElementType::Div, None)
                .flex()
                .flex_direction(ui::FlexDirection::Column)
                .width_full()
                .child(usage_row.child(usage_label))
                .child(progress_container.child(progress_bar).child(progress_remaining))
        } else {
            ui::Element::new(ui::ElementType::Div, None)
        }
    } else {
        ui::Element::new(ui::ElementType::Div, None)
    };

    container
        .child(row1.child(status_label).child(spacer1).child(status_value))
        .child(row2)
        .child(row3)
}

// ========== 辅助函数 ==========

fn build_switch(is_on: bool, event_id: &str) -> ui::Element {
    ui::Element::new(ui::ElementType::Switch, None)
        .on(ui::Event::Change, event_id)
        .prop("checked", if is_on { "true" } else { "false" })
}

fn build_settings_card(
    icon_svg: String,
    title: &str,
    desc: Option<&str>,
    right: Option<ui::Element>,
    click_event: Option<&str>,
) -> ui::Element {
    let icon = ui::Element::new(ui::ElementType::Svg, Some(&icon_svg))
        .width(22)
        .height(22)
        .text_color("#FFFFFF");

    let icon_wrap = ui::Element::new(ui::ElementType::Div, None)
        .width(22)
        .height(22)
        .flex()
        .align_center()
        .justify_center()
        .child(icon);

    let mut text_col = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full();

    let title_el = ui::Element::new(ui::ElementType::P, Some(title)).size(15);
    text_col = text_col.child(title_el);

    if let Some(desc_text) = desc {
        let desc_el = ui::Element::new(ui::ElementType::P, Some(desc_text))
            .size(13)
            .text_color("#888888");
        text_col = text_col.child(desc_el);
    }

    let mut row = ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Row)
        .align_center()
        .width_full()
        .bg("#1E1E1F")
        .radius(12)
        .padding_left(12)
        .padding_right(12)
        .padding_top(10)
        .padding_bottom(10)
        .gap(10)
        .child(icon_wrap)
        .child(text_col);

    if let Some(right_el) = right {
        let right_wrap = ui::Element::new(ui::ElementType::Div, None)
            .flex()
            .align_center()
            .justify_end()
            .child(right_el);
        row = row.child(right_wrap);
    }

    if let Some(event_id) = click_event {
        row = row.on(ui::Event::Click, event_id);
    }

    row
}

fn build_section_title(text: &str) -> ui::Element {
    ui::Element::new(ui::ElementType::P, Some(text))
        .size(13)
        .text_color("#888888")
        .margin_left(12)
        .margin_top(10)
}

fn build_more_link_icon() -> ui::Element {
    let svg = icons::more_link_svg();
    ui::Element::new(ui::ElementType::Svg, Some(&svg))
        .width(18)
        .height(18)
        .text_color("#0088FF")
}

fn build_value_text(value: &str) -> ui::Element {
    ui::Element::new(ui::ElementType::P, Some(value))
        .size(13)
        .text_color("#BBBBBB")
}

fn build_icon_text_button_full(label: &str, icon_svg: String, event_id: &str) -> ui::Element {
    let icon = ui::Element::new(ui::ElementType::Svg, Some(&icon_svg))
        .width(22)
        .height(22);

    let text = ui::Element::new(ui::ElementType::Span, Some(label)).size(14);

    ui::Element::new(ui::ElementType::Button, None)
        .without_default_styles()
        .on(ui::Event::Click, event_id)
        .radius(12)
        .padding(14)
        .bg("#2A2A2A")
        .width_full()
        .flex()
        .align_center()
        .gap(8)
        .child(icon)
        .child(text)
}

fn build_days_card(state: &UiState) -> ui::Element {
    let selected_text = format!("{}天", state.selected_days);
    let options = [3u32, 7, 10, 15, 30];

    let mut select = ui::Element::new(ui::ElementType::Select, Some(&selected_text))
        .on(ui::Event::Change, DAYS_DROPDOWN_EVENT)
        .radius(8)
        .padding_left(12)
        .padding_right(12)
        .bg("#2A2A2A")
        .size(14);

    for day in options.iter() {
        let option_text = format!("{}天", day);
        let option = ui::Element::new(ui::ElementType::Option, Some(&option_text));
        select = select.child(option);
    }

    build_settings_card(
        icons::calendar_svg(),
        "同步天气天数",
        None,
        Some(select),
        None,
    )
}

fn build_sync_card_text() -> String {
    let state = ui_state()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    let location = if state.last_sync_location.is_empty() {
        "暂无"
    } else {
        state.last_sync_location.as_str()
    };

    let (time_text, expired) = if state.last_sync_time_ms == 0 {
        ("暂无".to_string(), false)
    } else {
        let elapsed = now_ms().saturating_sub(state.last_sync_time_ms);
        let expire_ms = state.selected_days as u64 * 24 * 60 * 60 * 1000;
        let expired = expire_ms > 0 && elapsed > expire_ms;
        (format_relative(elapsed), expired)
    };

    let expired_mark = if expired { " (已过期)" } else { "" };
    format!(
        "地区: {}\n时间: {}{}",
        location, time_text, expired_mark
    )
}

fn format_relative(elapsed_ms: u64) -> String {
    let seconds = elapsed_ms / 1000;
    if seconds < 60 {
        return "刚刚".to_string();
    }
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}分钟前", minutes);
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}小时前", hours);
    }
    let days = hours / 24;
    format!("{}天前", days)
}

fn format_beijing_time(raw: &str) -> String {
    if let Some((y, m, d, hh, mm, ss)) = parse_iso_utc(raw) {
        let (y2, m2, d2, hh2) = add_hours(y, m, d, hh, 8);
        return format!(
            "{:04}‑{:02}‑{:02}_{:02}:{:02}:{:02}",
            y2, m2, d2, hh2, mm, ss
        );
    }
    raw.to_string()
}

fn parse_iso_utc(raw: &str) -> Option<(i32, i32, i32, i32, i32, i32)> {
    if raw.len() < 19 {
        return None;
    }
    let base = &raw[..19];
    let mut parts = base.split('T');
    let date = parts.next()?;
    let time = parts.next()?;
    let mut dparts = date.split('-');
    let y: i32 = dparts.next()?.parse().ok()?;
    let m: i32 = dparts.next()?.parse().ok()?;
    let d: i32 = dparts.next()?.parse().ok()?;
    let mut tparts = time.split(':');
    let hh: i32 = tparts.next()?.parse().ok()?;
    let mm: i32 = tparts.next()?.parse().ok()?;
    let ss: i32 = tparts.next()?.parse().ok()?;
    Some((y, m, d, hh, mm, ss))
}

fn add_hours(mut y: i32, mut m: i32, mut d: i32, mut hh: i32, add: i32) -> (i32, i32, i32, i32) {
    hh += add;
    while hh >= 24 {
        hh -= 24;
        d += 1;
        let dim = days_in_month(y, m);
        if d > dim {
            d = 1;
            m += 1;
            if m > 12 {
                m = 1;
                y += 1;
            }
        }
    }
    (y, m, d, hh)
}

fn days_in_month(y: i32, m: i32) -> i32 {
    match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) { 29 } else { 28 }
        }
        _ => 30,
    }
}

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

fn short_git_hash(hash: &str) -> String {
    let trimmed = hash.trim();
    if trimmed.is_empty() || trimmed == "unknown" {
        return "unknown".to_string();
    }
    trimmed.chars().take(7).collect()
}

pub fn rerender_main_ui() {
    let element_id = {
        let state = ui_state()
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.root_element_id.clone()
    };

    if let Some(element_id) = element_id {
        let ui_tree = build_main_ui();
        psys_host::ui_v3::render(&element_id, ui_tree);
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}