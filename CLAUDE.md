# 项目初始化说明

## 沟通语言

中文

## 开发文档与参考

- 参考插件源码：`Daymatter-AstroBox-Plugin/，simple-weather-astrobox-v2-plugin/，Debug-Helper-AstroBox-Plugin/`
- 官方插件文档：`plugin-dev/`
- API参考： qweather.mdx base.mdx
- 通信协议参考：INTERCONNECT_API.md

## 开发要求

- 每次对话结束后进行git（不要带你的名字）
- 每次对话后总结所有修改，讲解语法和其扩展语法
- 只要是你写的代码，给我写完整但不啰嗦的注释
- 每次我让你检查错误/提出新需求时，请先与我确认方案，得到我同意后再开始修改
- 不要私自加SVG图标，能用标准组件实现的用标准组件
- 需要任何资源文件请再规划时与我沟通
- 每次交流后自动将项目信息存到program.md
- 每次修改后自动将版本号++

## 常用命令（快速开始）

```bash
# 初始化子模块
git submodule update --init --remote --recursive

# 构建插件
python scripts/build_dist.py

# 打包插件
python scripts/build_dist.py --release --package
```

## 产物位置

- WASM：`dist/`
- ABP：`dist/`
