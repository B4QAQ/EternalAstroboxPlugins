# Eternal 永昼天气插件 与 simple-weather-astrobox-v2-plugin 差异对比

## 一、核心架构差异

### 1. 目标快应用不同

| 项目           | 目标快应用包名                    | 功能     |
| -------------- | --------------------------------- | -------- |
| simple-weather | `com.application.zaona.weather` | 简明天气 |
| Eternal        | `moe.mcns.Eternal`              | 永昼天气 |

### 2. 通信协议差异

#### simple-weather：单向发送

- **只需要发送数据到快应用**，不需要等待响应
- 发送 JSON 格式的天气数据
- 没有请求-响应机制

#### Eternal：请求-响应模式

- 需要与快应用进行**双向通信**
- 消息格式：`{ type, status?, data? }`
- 需要等待快应用响应并解析结果

### 3. 配对/激活流程

#### simple-weather：无需配对

- 直接使用，无需验证
- 用户选择城市后直接同步

#### Eternal：需要配对激活

```
设备连接
    ↓
发送 GET_APIKEY
    ↓
有有效 Key → 进入主界面
无 Key → 获取设备信息 (GET_DEVICEINFO)
    ↓
构建激活数据 (product.deviceId.serial.timestamp)
    ↓
用户付款
    ↓
验证签名
    ↓
保存 APIKey
```

### 4. 设备检测

#### simple-weather

- 不检测设备状态
- 点击发送时才检查设备是否连接

#### Eternal

- 启动时检测设备连接状态
- 使用 `get_connected_device_list()` 检测已连接设备
- 设备连接/断开时触发 `DeviceAction` 事件刷新状态

---

## 二、功能差异

### simple-weather 功能

| 功能       | 说明                     |
| ---------- | ------------------------ |
| 城市搜索   | 输入城市名搜索           |
| 历史记录   | 保存最近同步的城市       |
| 天气同步   | 发送天气数据到快应用     |
| 天数选择   | 选择同步 3/7/10/15/30 天 |
| 逐小时同步 | 可选同步逐小时天气       |
| 预警同步   | 可选同步天气预警         |
| 设置持久化 | 保存用户设置到文件       |

### Eternal 功能

| 功能          | 说明                   |
| ------------- | ---------------------- |
| 设备检测      | 检测设备连接状态       |
| 自动配对      | 检测到设备自动开始配对 |
| APIKey 验证   | 验证用户是否已激活     |
| 设备信息获取  | 获取设备信息用于激活   |
| 付款验证      | 验证付款签名           |
| APIKey 持久化 | 按设备存储 APIKey      |
| 城市列表获取  | 从快应用获取城市列表   |
| 天气同步      | 发送天气数据           |

## 五、UI 差异

### simple-weather UI

- 使用 Tabs 组件（两个 Tab：同步数据、设置）
- 城市搜索框 + 搜索按钮
- 历史城市 Grid 布局
- 天数下拉选择
- 开关组件（逐小时、预警）
- 使用自定义 SVG 图标

### Eternal UI

- 使用 Tabs 组件（三个 Tab：同步数据，城市添加、设置）
- 设备状态卡片
- 配对流程指示
- 城市列表卡片
- 无自定义 SVG，使用标准组件

---

## 六、API 调用差异

### simple-weather

```rust
// 从环境变量获取 API 配置
const WEATHER_API_HOST: Option<&str> = option_env!("WEATHER_API_HOST");
const WEATHER_API_CLIENT_TYPE: Option<&str> = option_env!("WEATHER_API_CLIENT_TYPE");
const WEATHER_API_KEY: Option<&str> = option_env!("WEATHER_API_KEY");

// 调用天气 API
let sync_url = api_url("/api/weather/sync")?;
let json = api_client::post_json(&sync_url, &payload_json)?;
```

### Eternal

```rust
// 需要验证 APIKey
pub async fn check_activation(mode: String) {
    let api_key = state.api_key;
    // 调用后端验证 APIKey
    let response = api_client::verify_api_key(&api_key).await?;
}

// 获取设备信息用于激活
pub async fn get_device_info() {
    let msg = BuildRequest("GET_DEVICEINFO", Value::Null);
    let response = send_request(device_addr, pkg_name, msg, 6000).await?;
    let data = BuildActivationData(response.data);
}
```

---

## 七、持久化差异

### simple-weather

保存用户设置：

```rust
struct StoredApiSettings {
    sync_hourly_enabled: bool,
    sync_alerts_enabled: bool,
    selected_days: u32,
    selected_location_id: String,
    recent_locations: Vec<LocationOption>,
}
// 存储到 api_settings.json
```

### Eternal

按设备存储 APIKey：

```rust
struct Storage {
    device_api_keys: HashMap<String, String>,  // deviceAddr -> apiKey
}
// 存储到 storage.json
```

---

## 八、关键代码差异点

### 1. 事件处理

**simple-weather**：同步处理

```rust
fn ui_event_processor(event_type, event_id, event_payload) {
    match event_id {
        SEND_BUTTON_EVENT => send_weather_data(),  // 同步调用
        // ...
    }
}
```

**Eternal**：需要异步等待响应

```rust
async fn start_pairing_flow() {
    let response = WaitForResponse(TypeAPIKey, 6000).await?;
    // 处理响应...
}
```

### 2. 设备操作

**simple-weather**：发送时检查

```rust
async fn send_via_interconnect(data: &str) -> Result<(), String> {
    let devices = get_connected_device_list().await;
    if devices.is_empty() {
        return Err("没有连接的设备");
    }
    // 发送数据...
}
```

**Eternal**：启动时检测 + 事件驱动

```rust
// OnDeviceAction 事件触发
fn on_device_action(event_payload: String) {
    ActionDeviceRefreshSync();  // 刷新设备状态
}

// OnUiRender 首次触发
fn on_ui_render(element_id: String) {
    if should_scan {
        go startPairingFlow();  // 异步开始配对
    }
}
```

---

## 九、重构建议

基于 simple-weather-astrobox-v2-plugin 重构 Eternal 插件：

### 需要新增的模块

1. **`protocol.rs`** - 通信协议层

   - 消息类型常量
   - 请求构建/响应解析
   - 异步等待机制
2. **`storage.rs`** - APIKey 持久化

   - 按设备存储 APIKey
   - 读取/写入 JSON 文件
3. **`activation.rs`** - 激活/配对逻辑

   - 配对流程状态机
   - 设备信息获取
   - 签名验证

### 需要修改的模块

1. **`state.rs`**

   - 添加设备连接状态
   - 添加配对步骤状态
   - 添加 APIKey 字段
2. **`event_handler.rs`**

   - 处理 DeviceAction 事件
   - 处理 InterconnectMessage 响应
3. **`build.rs`**

   - 新增配对流程 UI
   - 新增设备状态卡片
   - 新增城市列表显示
     ### 需要新增的功能
