# jhlaf

<!-- TOC GFM -->

* [项目描述](#项目描述)
* [分工说明](#分工说明)
* [开发环境](#开发环境)
  - [前端开发环境](#前端开发环境)
  - [后端测试环境](#后端测试环境)

<!-- /TOC -->

## 项目描述

一个简易的失物招领平台，作为精弘试用期小组作业。

~~奇怪的黑历史又增加了。~~

~~因为时间关系，很多动画效果没有做。~~

## 分工说明

- 前端:
  - 页面与模板: [komorebi531](https://github.com/komorebi531)、[OHaiYo-lzy](https://github.com/OHaiYo-lzy)、[timber3252](http://github.com/timber3252)(辅助)
  - 渲染与交互: [AllenZCH](https://github.com/AllenZCH)、[timber3252](http://github.com/timber3252)(辅助)
- 后端: [timber3252](http://github.com/timber3252)
- 运维: [OHaiYo-lzy](https://github.com/OHaiYo-lzy)、[timber3252](http://github.com/timber3252)(辅助)

## 开发环境

### 前端开发环境

确保 node 和 npm 已经安装，然后执行以下指令以安装依赖：

```bash
npm install
```

然后输入以下指令运行测试实例，在 http://localhost:3000 中预览效果。

```bash
node app.js
```

### 后端测试环境

确保安装并配置好 Rust 环境和 PostgreSQL 环境，在 server 目录下执行 `cargo run` 即可。

