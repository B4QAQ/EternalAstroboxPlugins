use std::sync::{OnceLock, RwLock};
use tracing::{info, warn};

const SETTINGS_FILE: &str = "api_settings.json";
const WEATHER_API_HOST: Option<&str> = option_env!("WEATHER_API_HOST");
const WEATHER_API_CLIENT_TYPE: Option<&str> = option_env!("WEATHER_API_CLIENT_TYPE");
const WEATHER_API_KEY: Option<&str> = option_env!("WEATHER_API_KEY");

/// RSA公钥，用于验证APIKey签名
pub const RSA_PUBLIC_KEY: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAutMRnVK8jgWfNMpPN1OA
g6itqCrF2KBzRqHUYE6iTGre9vvsOPR8XgYoHxhfEOy+21ofTELLr2Gw1zp6WsgP
7ij7Vk+LI7xO1sUSXnaFEDx8V12gGzgleq1S5UwX985GxTky+SSgQI1/PjuOOy/6
HuOo7fPkXr2YMsVbNHw/eBGWrqqLn5A8rYAO7zYXZY8LM/EnraN72Qs3oh5WjksW
ZQwVnGDk+iY53LsWqjHlcD6jXc7c47juskZGGho3qFSlw3FwcBh/eHhedK6WCPv6
aqkLFUQ8DAQdJqlGy+ZddKpx2LTz7QbYXVM8/C8d4Zv8VgUgznC5g6y3fxvz9K/F
+wIDAQAB
-----END PUBLIC KEY-----"#;

/// 应用名称，用于服务端API
pub const APP_NAME: &str = "Eternal";

/// 快应用包名
pub const QA_PKG_NAME: &str = "moe.mcns.Eternal";

/// API服务器地址
pub const API_BASE_URL: &str = "https://api.b4qaq.cn";

fn default_bool_true() -> bool {
    true
}

/// 主Tab枚举
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MainTab {
    SyncData,
    CityManage,
    Settings,
}

/// 设备信息结构（只保留验证和显示需要的字段）
#[derive(Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct DeviceInfo {
    // 验证需要的字段
    pub product: String,
    pub deviceId: String,
    pub serial: String,
    // 屏幕信息
    pub screenWidth: u32,
    pub screenHeight: u32,
    pub screenShape: String,
    pub screenDensity: u32,
    // 设备基本信息
    pub deviceType: String,
    pub brand: String,
    pub model: String,
}

/// 城市信息
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CityInfo {
    pub name: String,
    pub lat: String,
    pub lon: String,
    #[serde(default)]
    pub adm1: String,
    #[serde(default)]
    pub adm2: String,
    #[serde(default)]
    pub country: String,
}

/// UI状态
pub struct UiState {
    pub root_element_id: Option<String>,
    pub current_tab: MainTab,
    pub settings_loaded: bool,

    // APIKey相关
    pub api_key: String,
    pub api_key_verified: bool,
    pub api_key_visible: bool, // APIKey是否可见（默认隐藏）
    pub device_info: Option<DeviceInfo>,
    pub server_device_info: Option<serde_json::Value>, // 从服务器获取的设备信息（含用量等）
    pub verification_status: VerificationStatus,

    // 同步设置
    pub sync_hourly_enabled: bool,
    pub sync_alerts_enabled: bool,
    pub selected_days: u32,

    // 城市管理
    pub city_list: Vec<CityInfo>,
    pub selected_city_index: Option<usize>,

    // 选中位置信息
    pub selected_location_id: String,
    pub selected_location_name: String,
    pub selected_location_adm1: String,
    pub selected_location_adm2: String,
    pub selected_location_lat: String,
    pub selected_location_lon: String,

    // 同步状态
    pub last_sync_time_ms: u64,
    pub last_sync_location: String,
}

/// 验证状态
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerificationStatus {
    NotStarted,
    CheckingDevice,
    GettingAPIKey,
    GettingDeviceInfo,
    WaitingPayment,
    VerifyingPayment,
    Verified,
    Failed,
}

static UI_STATE: OnceLock<RwLock<UiState>> = OnceLock::new();

pub fn ui_state() -> &'static RwLock<UiState> {
    UI_STATE.get_or_init(|| {
        let state = UiState {
            root_element_id: None,
            current_tab: MainTab::SyncData,
            settings_loaded: false,

            api_key: String::new(),
            api_key_verified: false,
            api_key_visible: false, // 默认隐藏APIKey
            device_info: None,
            server_device_info: None,
            verification_status: VerificationStatus::NotStarted,

            sync_hourly_enabled: default_bool_true(),
            sync_alerts_enabled: default_bool_true(),
            selected_days: 7,

            city_list: Vec::new(),
            selected_city_index: None,

            selected_location_id: String::new(),
            selected_location_name: String::new(),
            selected_location_adm1: String::new(),
            selected_location_adm2: String::new(),
            selected_location_lat: String::new(),
            selected_location_lon: String::new(),

            last_sync_time_ms: 0,
            last_sync_location: String::new(),
        };
        RwLock::new(state)
    })
}

/// 获取API基础地址（优先使用环境变量，否则使用默认地址）
pub fn server_api_base() -> &'static str {
    WEATHER_API_HOST
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(API_BASE_URL)
}

pub fn server_api_client_type() -> Result<&'static str, String> {
    let client_type = WEATHER_API_CLIENT_TYPE
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "WEATHER_API_CLIENT_TYPE 未配置".to_string())?;

    Ok(client_type)
}

pub fn server_api_key() -> Result<&'static str, String> {
    let api_key = WEATHER_API_KEY
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "WEATHER_API_KEY 未配置".to_string())?;

    Ok(api_key)
}

/// 存储的设置结构
#[derive(serde::Serialize, serde::Deserialize)]
struct StoredApiSettings {
    #[serde(default = "default_bool_true")]
    sync_hourly_enabled: bool,
    #[serde(default = "default_bool_true")]
    sync_alerts_enabled: bool,
    #[serde(default)]
    selected_days: u32,
    #[serde(default)]
    selected_city_index: Option<usize>,
    #[serde(default)]
    api_key: String,
    #[serde(default)]
    city_list: Vec<CityInfo>,
}

pub fn load_api_settings_once() {
    let should_load = {
        let mut state = ui_state()
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if state.settings_loaded {
            false
        } else {
            state.settings_loaded = true;
            true
        }
    };

    if !should_load {
        return;
    }

    match std::fs::read_to_string(SETTINGS_FILE) {
        Ok(content) => match serde_json::from_str::<StoredApiSettings>(&content) {
            Ok(stored) => {
                let mut state = ui_state()
                    .write()
                    .unwrap_or_else(|poisoned| poisoned.into_inner());
                state.sync_hourly_enabled = stored.sync_hourly_enabled;
                state.sync_alerts_enabled = stored.sync_alerts_enabled;
                state.selected_days = if stored.selected_days == 0 {
                    7
                } else {
                    stored.selected_days
                };
                state.selected_city_index = stored.selected_city_index;
                state.api_key = stored.api_key;
                state.city_list = stored.city_list;

                // 如果有APIKey，标记为已验证
                if !state.api_key.is_empty() {
                    state.api_key_verified = true;
                    state.verification_status = VerificationStatus::Verified;
                }

                if state.selected_city_index.is_none() && !state.city_list.is_empty() {
                    state.selected_city_index = Some(0);
                }
                info!("loaded api settings from disk");
            }
            Err(e) => {
                warn!("failed to parse api settings: {}", e);
            }
        },
        Err(e) => {
            warn!("api settings not loaded: {}", e);
        }
    }
}

pub fn save_all_settings() -> Result<(), String> {
    let state = ui_state()
        .read()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let stored = StoredApiSettings {
        sync_hourly_enabled: state.sync_hourly_enabled,
        sync_alerts_enabled: state.sync_alerts_enabled,
        selected_days: state.selected_days,
        selected_city_index: state.selected_city_index,
        api_key: state.api_key.clone(),
        city_list: state.city_list.clone(),
    };

    let content = serde_json::to_string_pretty(&stored).map_err(|e| e.to_string())?;
    std::fs::write(SETTINGS_FILE, content).map_err(|e| e.to_string())?;
    Ok(())
}
