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

#### 上传文件

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `UPLOAD_FILE` | `UPLOAD_FILE_DONE` | `{ uri: string, data: ArrayBuffer }` | `{ uri: string }` | 写入二进制文件到指定 URI |

**注意**: `data` 参数为 `ArrayBuffer` 类型，用于传输二进制文件数据。

#### 删除文件

| 请求类型 | 响应类型 | 请求参数 | 响应数据 | 说明 |
|---------|---------|---------|---------|------|
| `DEL_FILE` | `DEL_FILE_DONE` | `{ uri: string }` | `{ uri: string }` | 删除指定 URI 的文件 |

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

**请求:**
```json
{
  "type": "UPLOAD_FILE",
  "data": {
    "uri": "internal://cache/example.png",
    "data": "<ArrayBuffer 二进制数据>"
  }
}
```

**响应 (成功):**
```json
{
  "type": "UPLOAD_FILE_DONE",
  "status": "OK",
  "data": {
    "uri": "internal://cache/example.png"
  }
}
```

**响应 (失败):**
```json
{
  "type": "UPLOAD_FILE_DONE",
  "status": "写入失败: 1001",
  "data": {
    "uri": "internal://cache/example.png"
  }
}
```

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

1. 所有操作失败时，响应消息的 `status` 字段会包含错误描述
2. 接收到带有错误状态的消息时，手表端会弹出错误提示
3. 未知的消息类型会被忽略并记录日志

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