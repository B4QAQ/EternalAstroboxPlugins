# Eternal 设备通信协议文档

> 基于 `interconnect.js` 模块，用于手机 App 与手表端的数据同步交互

## 概述

- **通信方式**: 快应用 `@system.interconnect` API
- **消息格式**: JSON 对象 `{ type, status, data }`
- **通信模式**: 请求-响应模式（手机端发起请求，手表端响应）

---

## 消息结构

```typescript
interface Message {
  type: string    // 消息类型/操作名
  status?: string // 状态码，成功为 "OK"，失败为错误描述
  data?: any      // 携带的数据
}
```

---

## API 列表

### 一、GET 请求（读取数据）

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `GET_APIKEY` | `APIKEY` | 无 | `string` (API密钥) | 获取当前 API Key |
| `GET_CITYLIST` | `CITYLIST` | 无 | `City[]` (城市列表) | 获取城市列表 |
| `GET_WARNDATA` | `WARNDATA` | `{ cityindex: number }` | `{ cityindex, result: WarnData[] }` | 获取指定城市的预警数据 |
| `GET_WEATHERDATA` | `WEATHERDATA` | `{ cityindex: number }` | `{ cityindex, result: WeatherData }` | 获取指定城市的天气数据 |
| `GET_ALLCITIESDATA` | `ALLCITIESDATA` | 无 | `WeatherData[]` | 获取所有城市的天气数据 |
| `GET_SETTINGS` | `SETTINGS` | 无 | `Settings` (设置对象) | 获取应用设置 |
| `GET_DEVICEINFO` | `DEVICEINFO` | 无 | `DeviceInfo` (设备信息) | 获取设备信息 |

---

### 二、PUT 请求（写入/更新数据）

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `PUT_WEATHERDATA` | `PUT_WEATHERDATA_DONE` | `{ cityindex: number, result: WeatherData }` | `{ cityindex: number }` | 更新指定城市的天气数据 |
| `PUT_WARNDATA` | `PUT_WARNDATA_DONE` | `{ cityindex: number, result: WarnData[] }` | `{ cityindex: number }` | 更新指定城市的预警数据 |
| `PUT_SETTINGS` | `PUT_SETTINGS_DONE` | `Settings` (设置对象) | `Settings` | 保存应用设置 |
| `PUT_CITY` | `PUT_CITY_DONE` | `{ result: City }` | `City` | 添加城市 |

---

### 三、DELETE 请求（删除数据）

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `DEL_CITY` | `DEL_CITY_DONE` | `{ cityindex: number }` | `{ cityindex: number }` | 删除指定索引的城市 |

---

### 四、城市排序

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `ORDER_CITY` | `ORDER_CITY_DONE` | `{ cityindex: number, offset: number }` | `{ cityindex, offset }` | 调整城市顺序 |

---

### 五、文件操作

> 互联通道传输的是 JSON 文本，无法直接携带原生 `ArrayBuffer`。因此 `UPLOAD_FILE` 的 `data` 字段统一使用 **base64 编码字符串**（可带 data URI 前缀），手表端会自动解码并写入。

#### 上传文件

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `UPLOAD_FILE` | `UPLOAD_FILE_DONE` | 见下方 | `{ uri: string }` | 写入二进制文件到指定 URI |

**请求参数 `data`:**

| 字段 | 类型 | 必填 | 说明 |
|------|------|-----|------|
| `uri` | string | 是 | 目标文件 URI，父目录不存在时会自动创建 |
| `data` | string \| ArrayBuffer | 是 | 文件内容；字符串时按 base64 解码（自动剥离 `data:...;base64,` 前缀） |
| `append` | boolean | 否 | 是否追加模式，默认 `false`（覆盖写入）。为 `true` 时在文件末尾追加，此时 `position` 无效 |
| `position` | number | 否 | 写入起始位置（字节偏移），仅在 `append` 为 `false` 时生效 |

**注意事项**:
- 二进制数据请先在手机端做 base64 编码后放入 `data`，直接传原始字节会因 JSON 序列化失败。
- 目标 URI 的父目录若不存在，手表端会自动创建（如 `internal://files/bg/`）。
- 背景图请上传为 PNG 格式，路径约定为 `internal://files/bg/<天气名>.png`（天气名见背景管理章节）。

#### 删除文件

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `DEL_FILE` | `DEL_FILE_DONE` | `{ uri: string }` | `{ uri: string }` | 删除指定 URI 的文件 |

---

### 六、背景图管理

手表支持为每种天气显示自定义背景图，背景文件存放于 `internal://files/bg/`。

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `REFRESH_BG` | `REFRESH_BG_DONE` | 无 | `{ list: string[] }` | 重新扫描背景目录，返回已存在的背景名数组；上传/删除背景后需调用以刷新手表缓存 |
| `GET_BG_INFO` | `GET_BG_INFO_DONE` | 无 | `{ supported: string[], installed: string[] }` | 获取支持自定义的全部天气背景名（`supported`）与当前已安装的背景名（`installed`） |

**背景命名规则**:
- 文件名与天气图标名一致，仅支持 PNG，如 `sunny.png`、`rain-l.png`、`overcast.png`。
- 完整列表可通过 `GET_BG_INFO` 获取，当前支持 17 种天气类别（加 `unknown` 兜底共 18 个名字）。
- 上传后调用 `REFRESH_BG` 使手表立即生效；删除背景文件后同样需要调用。

**`GET_BG_INFO_DONE` 响应示例:**
```json
{
  "type": "GET_BG_INFO_DONE",
  "status": "OK",
  "data": {
    "supported": ["sunny", "sunny-n", "cloudy", "rain-l", "rain-m", "overcast"],
    "installed": ["sunny", "rain-l"]
  }
}
```

---

## 详细示例

### 1. 获取城市列表

**请求:**
```json
{
  "type": "GET_CITYLIST"
}
```

**响应 (成功):**
```json
{
  "type": "CITYLIST",
  "status": "OK",
  "data": [
    { "name": "北京", "lat": 39.9042, "lon": 116.4074 },
    { "name": "上海", "lat": 31.2304, "lon": 121.4737 }
  ]
}
```

---

### 2. 获取指定城市天气数据（仅做演示，请以实际和API为准）

**请求:**
```json
{
  "type": "GET_WEATHERDATA",
  "data": {
    "cityindex": 0
  }
}
```

**响应 (成功):**
```json
{
  "type": "WEATHERDATA",
  "status": "OK",
  "data": {
    "cityindex": 0,
    "result": {
      "temp": 25,
      "humidity": 60,
      "weather": "晴"
    }
  }
}
```

---

### 3. 更新城市天气数据

**请求:**
```json
{
  "type": "PUT_WEATHERDATA",
  "data": {
    "cityindex": 0,
    "result": {
      "temp": 26,
      "humidity": 55,
      "weather": "多云"
    }
  }
}
```

**响应 (成功):**
```json
{
  "type": "PUT_WEATHERDATA_DONE",
  "status": "OK",
  "data": {
    "cityindex": 0
  }
}
```

**响应 (失败):**
```json
{
  "type": "PUT_WEATHERDATA_DONE",
  "status": "错误描述信息",
  "data": {
    "cityindex": 0
  }
}
```

---

### 4. 添加城市

**请求:**
```json
{
  "type": "PUT_CITY",
  "data": {
    "result": {
      "name": "广州",
      "lat": 23.1291,
      "lon": 113.2644
    }
  }
}
```

**响应 (成功):**
```json
{
  "type": "PUT_CITY_DONE",
  "status": "OK",
  "data": {
    "name": "广州",
    "lat": 23.1291,
    "lon": 113.2644
  }
}
```

---

### 5. 删除城市

**请求:**
```json
{
  "type": "DEL_CITY",
  "data": {
    "cityindex": 2
  }
}
```

**响应 (成功):**
```json
{
  "type": "DEL_CITY_DONE",
  "status": "OK",
  "data": {
    "cityindex": 2
  }
}
```

**响应 (失败 - 城市不存在):**
```json
{
  "type": "DEL_CITY_DONE",
  "status": "城市不存在",
  "data": {
    "cityindex": 2
  }
}
```

---

### 6. 调整城市顺序

**请求:**
```json
{
  "type": "ORDER_CITY",
  "data": {
    "cityindex": 0,
    "offset": 1
  }
}
```

**响应:**
```json
{
  "type": "ORDER_CITY_DONE",
  "status": "OK",
  "data": {
    "cityindex": 0,
    "offset": 1
  }
}
```

---

### 7. 上传文件

`data` 为文件内容的 base64 字符串。推荐手机端先将图片/二进制读为字节数组再 Base64 编码。

**请求:**
```json
{
  "type": "UPLOAD_FILE",
  "data": {
    "uri": "internal://files/bg/sunny.png",
    "data": "iVBORw0KGgoAAAANSUhEUgAA...(base64)",
    "append": false
  }
}
```

**响应 (成功):**
```json
{
  "type": "UPLOAD_FILE_DONE",
  "status": "OK",
  "data": {
    "uri": "internal://files/bg/sunny.png"
  }
}
```

**响应 (失败):**
```json
{
  "type": "UPLOAD_FILE_DONE",
  "status": "写入失败: 300",
  "data": {
    "uri": "internal://files/bg/sunny.png"
  }
}
```

**上传背景图的完整流程:**
1. `UPLOAD_FILE` 写入 `internal://files/bg/<天气名>.png`（PNG，base64 编码）
2. `REFRESH_BG` 通知手表刷新背景缓存
3. 收到 `REFRESH_BG_DONE`（status 为 `OK`）后生效

---

### 8. 删除文件

**请求:**
```json
{
  "type": "DEL_FILE",
  "data": {
    "uri": "internal://cache/example.png"
  }
}
```

**响应 (成功):**
```json
{
  "type": "DEL_FILE_DONE",
  "status": "OK",
  "data": {
    "uri": "internal://cache/example.png"
  }
}
```

---

### 九、SimpleFetch 桥接网络（`SF_*`）

当手机端安装了 AstroBox 插件并支持 SimpleFetch 时，手表可通过互联通道代理 HTTP 请求。所有桥接消息以 `SF_` 为前缀，手表端对 `SF_` 消息单独路由，不触发未知消息提示。

| 消息类型 | 方向 | 说明 |
|---------|------|------|
| `SF_HANDSHAKE` | 手机→手表 | 手机端发起桥接握手，手表收到后激活桥接并回复 ACK |
| `SF_HANDSHAKE_ACK` | 手表→手机 | 握手确认 |
| `SF_PING` / `SF_PONG` | 双向 | 心跳保活（手表每 10s 发 PING，5s 未收到 PONG 则断开桥接） |
| `SF_REQUEST` | 手表→手机 | 发起代理请求（普通 fetch 或 SSE） |
| `SF_RESPONSE` | 手机→手表 | 返回响应；大体积响应以 base64 分片传输（`chunk`/`totalChunks`） |
| `SF_CLOSE` | 手表→手机 | 关闭 SSE 连接 |
| `SF_SSE_EVENT` / `SF_SSE_END` / `SF_SSE_ERROR` | 手机→手表 | SSE 事件推送 / 结束 / 错误 |

桥接激活期间 `global.NetworkStatus` 为 `bridge`，手表的天气等网络请求会改走 SimpleFetch 通道；连接断开后自动切回原生网络。

---

## 连接状态

| 状态码 | 含义 |
|-------|------|
| `0` | 未初始化 |
| `1` | 已连接 |
| `2` | 已断开 |

---

## 诊断状态码

| 状态码 | 含义 |
|-------|------|
| `0` | 连接成功 |
| `204` | 连接超时 |
| `1000` | 其他连接错误 |
| `1001` | 对端应用未安装 |

---

## 错误处理

1. 所有操作失败时，响应消息的 `status` 字段会包含错误描述（非 `OK`）。
2. 接收到带有错误状态的消息时，手表端会弹出错误提示；但 `SF_` 前缀的桥接消息由 SimpleFetch 模块自行处理，不弹全局提示。
3. 以 `SF_` 开头的消息会被路由到 SimpleFetch 模块，其他未知消息类型会弹出警告并记录日志。

---

## 手机端实现建议

### 发送请求示例 (Android)

```kotlin
// 建立连接后发送请求
fun getCityList() {
    val request = mapOf(
        "type" to "GET_CITYLIST"
    )
    interconnect.send(request)
}

// 监听响应
override fun onMessage(data: Map<String, Any>) {
    when (data["type"]) {
        "CITYLIST" -> {
            if (data["status"] == "OK") {
                val cities = data["data"] as List<City>
                // 处理城市列表
            } else {
                // 处理错误
            }
        }
    }
}
```

### iOS 实现

```swift
func getCityList() {
    let request: [String: Any] = [
        "type": "GET_CITYLIST"
    ]
    interconnect.send(request)
}

func onMessage(data: [String: Any]) {
    guard let type = data["type"] as? String else { return }
    switch type {
    case "CITYLIST":
        if data["status"] as? String == "OK",
           let cities = data["data"] as? [[String: Any]] {
            // 处理城市列表
        }
    default:
        break
    }
}
```

---

## 数据类型定义

```typescript
// 城市信息
interface City {
  name: string
  lat: number
  lon: number
  [key: string]: any
}

// 天气数据
interface WeatherData {
  temp?: number
  humidity?: number
  weather?: string
  [key: string]: any
}

// 预警数据
interface WarnData {
  title: string
  level: string
  content: string
  [key: string]: any
}

// 应用设置
interface Settings {
  [key: string]: any
}

// 设备信息
interface DeviceInfo {
  brand?: string
  model?: string
  system?: string
  [key: string]: any
}
```

---

## 版本历史

| 版本 | 日期 | 说明 |
|-----|------|------|
| 1.0 | 2026-07 | 初始版本 |
| 1.1 | 2026-08 | 文件上传改为 base64 传输并支持 append/position/自动建目录；新增背景管理 `REFRESH_BG`/`GET_BG_INFO`；补充 SimpleFetch 桥接 `SF_*` 协议 |