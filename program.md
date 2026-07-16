# 简明天气同步器 - 项目文档

## 版本历史

### v0.3.1 (2026-07-16)

**UI优化和功能完善**

#### 修复

1. **城市管理Tab**
   - 刷新按钮现在可以正常工作
   - 点击刷新按钮会从设备请求城市列表

2. **设置Tab授权状态显示**
   - APIKey状态卡片重新设计：
     - 第一行：`APIKey状态` + `已验证`（绿色）/ `未验证`（红色）
     - 第二行：APIKey显示（每4个字符一组，空格分隔）+ 提示文字
     - 第三行：请求用量 + 进度条（使用flex布局实现）
   - 移除了单独的用户类型卡片
   - 买断制显示请求用量进度条

---

### v0.3.0 (2026-07-16)

**UI重构和功能完善**

#### 新功能

1. **同步数据Tab改进**
   - 移除城市搜索，改为从城市管理中选择城市（下拉菜单）
   - 验证成功后不再打开URL
   - 验证成功后自动刷新设备信息

2. **城市管理Tab重构**
   - 添加城市按钮置顶
   - 城市列表显示：城市名 + 排序按钮（上下箭头）
   - 第二行显示：adm1 adm2 + 删除按钮
   - 刷新按钮在标题行右侧

3. **设置Tab改进**
   - 显示APIKey状态和请求用量信息
   - 从 `/api/v2/getInfo` 获取设备信息
   - 免费版用户显示"升级为付费版"选项
   - 帮助文档改为 https://docs.b4qaq.cn/docs/eternal
   - QQ交流群改为 1076096725
   - 移除"支持本项目"选项

4. **验证流程修复**
   - APIKey验证成功后不再打开付款URL
   - 使用 `dialog::open_url()` 正确打开URL

---

### v0.2.0 (2026-07-16)

**重大更新：重构UI和通信协议**

#### 新功能

1. **三Tab页面结构**
   - 同步数据：天气同步功能
   - 城市管理：城市管理（增删改排序）
   - 设置：帮助文档、QQ群、构建信息

2. **设备验证流程**
   - 打开插件时自动检测设备连接
   - 支持APIKey验证
   - 支持免费版和付费版验证
   - 支持爱发电付款流程

3. **通信协议重构**
   - 采用 INTERCONNECT_API.md 标准格式
   - 支持 GET_APIKEY、GET_DEVICEINFO、PUT_SETTINGS
   - 支持 GET_CITYLIST、PUT_CITY、DEL_CITY
   - 支持 PUT_WEATHERDATA

#### 技术改进

- 重构 `state.rs`：添加 DeviceInfo、CityInfo、VerificationStatus 等结构
- 重构 `build.rs`：三Tab UI实现
- 重构 `event_handler.rs`：完整验证流程和通信协议实现
- 更新 `Cargo.toml`：添加 urlencoding、sha2、rsa、base64 依赖

#### 文件结构

```
src/
├── lib.rs              # 插件入口
├── logger.rs           # 日志模块
└── ui/
    ├── mod.rs          # UI模块入口
    ├── state.rs        # 状态管理
    ├── build.rs        # UI构建
    ├── event_handler.rs # 事件处理
    ├── api_client.rs   # API客户端
    └── icons.rs        # SVG图标
```

#### 通信协议示例

```json
// 获取APIKey
{ "type": "GET_APIKEY" }

// 获取设备信息
{ "type": "GET_DEVICEINFO" }

// 保存设置
{
  "type": "PUT_SETTINGS",
  "data": { "APIKey": "xxx" }
}

// 获取城市列表
{ "type": "GET_CITYLIST" }

// 添加城市
{
  "type": "PUT_CITY",
  "data": {
    "result": { "name": "北京", "lat": "39.90", "lon": "116.41" }
  }
}

// 删除城市
{
  "type": "DEL_CITY",
  "data": { "cityindex": 0 }
}

// 同步天气
{
  "type": "PUT_WEATHERDATA",
  "data": {
    "cityindex": 0,
    "result": { ...天气数据... }
  }
}
```

---

## 开发说明

### 构建命令

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 打包
python scripts/build_dist.py --release --package
```

### 输出位置

- WASM: `dist/simple-weather-astrobox-v2-plugin.wasm`
- ABP: `dist/简明天气同步器.abp`

### 环境配置

创建 `.env.local` 文件配置API：

```
WEATHER_API_HOST=https://your-api-host
WEATHER_API_CLIENT_TYPE=your-client-type
WEATHER_API_KEY=your-api-key
```

---

## API 参考

### 服务端API

| API | 说明 |
|-----|------|
| `GET /api/v2/verify/Eternal?data=xxx` | 设备验证，重定向到付款页 |
| `GET /api/v2/verifyCheck/Eternal?data=xxx&type=xxx` | 检查授权，返回APIKey+signature |
| `POST /api/v2/getInfo/Eternal` | 获取设备信息 |
| `POST /api/v2/3f/getWeather/Eternal` | 获取天气数据 |
| `POST /api/v2/3f/getCity/Eternal` | 搜索城市 |
| `POST /api/v2/3f/getWarn/Eternal` | 获取天气预警 |

### 设备通信消息

| 类型 | 方向 | 说明 |
|------|------|------|
| GET_APIKEY | 插件→设备 | 请求APIKey |
| APIKEY | 设备→插件 | 返回APIKey |
| GET_DEVICEINFO | 插件→设备 | 请求设备信息 |
| DEVICEINFO | 设备→插件 | 返回设备信息 |
| PUT_SETTINGS | 插件→设备 | 保存设置 |
| GET_CITYLIST | 插件→设备 | 请求城市列表 |
| CITYLIST | 设备→插件 | 返回城市列表 |
| PUT_CITY | 插件→设备 | 添加城市 |
| DEL_CITY | 插件→设备 | 删除城市 |
| PUT_WEATHERDATA | 插件→设备 | 同步天气数据 |

---

## 验证流程

```
打开插件
    ↓
检测是否连接设备
    ↓ 有设备
检查本地是否保存了APIKey
    ↓ 有APIKey
直接使用，等待同步时唤醒应用
    ↓ 没有APIKey
唤醒应用，发送 GET_APIKEY
    ↓ 有回复且有效
保存APIKey，退出应用
    ↓ 无效或无回复
发送 GET_DEVICEINFO 获取设备信息
拼接结果: 设备名.设备ID.唯一ID.时间戳
调用 GET /api/v2/verify/Eternal?data=xxx
    ↓
302重定向到爱发电付款页面
    ↓ 付款成功
调用 GET /api/v2/verifyCheck/Eternal?data=xxx
返回 { APIKey, signature }
    ↓
用 RSA-SHA256 公钥验证 signature
验证成功后发送 PUT_SETTINGS 保存APIKey
```
