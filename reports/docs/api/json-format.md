# JSON 数据格式

<!-- TOC GFM -->

* [对象与类型定义](#对象与类型定义)
  - [item](#item)
  - [userdata](#userdata)
  - [err 值定义](#err-值定义)
* [操作类型 (Send)](#操作类型-send)
  - [注册](#注册)
  - [登录](#登录)
  - [查询用户是否存在](#查询用户是否存在)
  - [获取用户数据](#获取用户数据)
  - [发布 LOST & FOUND](#发布-lost--found)
  - [删除 LOST & FOUND](#删除-lost--found)
  - [筛选 LOST & FOUND](#筛选-lost--found)
* [操作类型 (Recv)](#操作类型-recv)
  - [注册（Result）](#注册result)
  - [登录（Result）](#登录result)
  - [查询用户是否存在（Result）](#查询用户是否存在result)
  - [获取用户数据（Result）](#获取用户数据result)
  - [发布 LOST & FOUND（Result）](#发布-lost--foundresult)
  - [删除 LOST & FOUND（Result）](#删除-lost--foundresult)
  - [筛选 LOST & FOUND（Result）](#筛选-lost--foundresult)

<!-- /TOC -->

## 对象与类型定义

### item

| Key         | Value  | Desc.                                               |
|-------------|--------|-----------------------------------------------------|
| type        | string | 物品类型，目前包括 "cards"、"credentials"、"others" |
| name        | string | 物品名称                                            |
| image       | string | 图片（使用 base64），目前限制 1 张图片              |
| desc        | string | 备注                                                |
| pickup_time | string | 拾取时间                                            |
| place       | string | 拾取地点                                            |
| contact     | string | 联系方式（默认填写用户 contact）                    |
| post_time   | string | 发布时间                                            |
| tags        | string | 关键词描述（空格分割，比如“红色”、“方形”）          |

### userdata

| Key      | Value  | Desc.                                                  |
|----------|--------|--------------------------------------------------------|
| username | string | 用户名                                                 |
| contact  | string | 联系方式（用于在发布 LOST & FOUND 时自动填充联系方式） |

### err 值定义

| Value | Desc.                |
|-------|----------------------|
| 0101  | 注册失败：用户已存在 |
| 0102  | 注册失败：未知错误   |

## 操作类型 (Send)

### 注册

| Key      | Value      | Desc.                                                  |
|----------|------------|--------------------------------------------------------|
| type     | "register" | 操作类型标记                                           |
| username | string     | 用户名                                                 |
| password | string     | 经过处理的密码                                         |
| contact  | string     | 联系方式（用于在发布 LOST & FOUND 时自动填充联系方式） |

### 登录

| Key      | Value   | Desc.          |
|----------|---------|----------------|
| type     | "login" | 操作类型标记   |
| username | string  | 用户名         |
| password | string  | 经过处理的密码 |

### 查询用户是否存在

| Key    | Value        | Desc.            |
|--------|--------------|------------------|
| type   | "query_user" | 操作类型标记     |
| userid | string       | 用户的唯一识别码 |

### 获取用户数据

| Key    | Value            | Desc.            |
|--------|------------------|------------------|
| type   | "query_userdata" | 操作类型标记     |
| userid | string           | 用户的唯一识别码 |

### 发布 LOST & FOUND

| Key    | Value                           | Desc.            |
|--------|---------------------------------|------------------|
| type   | "publish_lost", "publish_found" | 操作类型标记     |
| userid | string                          | 用户的唯一识别码 |
| item   | JSON object                     | 发布的物品       |

### 删除 LOST & FOUND

| Key     | Value                         | Desc.            |
|---------|-------------------------------|------------------|
| type    | "delete_lost", "delete_found" | 操作类型标记     |
| userid  | string                        | 用户的唯一识别码 |
| post_id | string                        | 物品唯一标识码   |

### 筛选 LOST & FOUND

| Key       | Value                                         | Desc.                                      |
|-----------|-----------------------------------------------|--------------------------------------------|
| type      | "select"                                      | 操作类型标记                               |
| userid    | string                                        | 用户的唯一识别码                           |
| item_type | "lost", "found", "all"                        | 筛选物品类型                               |
| item_tags | string                                        | 关键词描述（空格分割，比如“红色”、“方形”） |
| sort_type | "none", "recent_post_time" | 排序方式                                   |
| sort_cnt  | string                                        | 一个数字，表示排序后的前 sort_cnt 个       |

## 操作类型 (Recv)

### 注册（Result）

| Key         | Value      | Desc.              |
|-------------|------------|--------------------|
| type        | "result"   | 操作类型标记       |
| result_type | "register" | 结果类型标记       |
| stat        | bool       | 注册是否成功       |
| err         | string     | 错误信息（如果有） |

### 登录（Result）

| Key         | Value    | Desc.              |
|-------------|----------|--------------------|
| type        | "result" | 操作类型标记       |
| result_type | "login"  | 结果类型标记       |
| stat        | bool     | 登录是否成功       |
| err         | string   | 错误信息（如果有） |
| userid      | string   | 用户的唯一识别码   |

### 查询用户是否存在（Result）

| Key         | Value        | Desc.            |
|-------------|--------------|------------------|
| type        | "result"     | 操作类型标记     |
| result_type | "query_user" | 结果类型标记     |

### 获取用户数据（Result）

| Key         | Value        | Desc.        |
|-------------|--------------|--------------|
| type        | "result"     | 操作类型标记 |
| result_type | "query_user" | 结果类型标记 |
| data        | userdata     | 用户数据     |

### 发布 LOST & FOUND（Result）

| Key         | Value                           | Desc.                                |
|-------------|---------------------------------|--------------------------------------|
| type        | "result"                        | 操作类型标记                         |
| result_type | "publish_lost", "publish_found" | 结果类型标记                         |
| stat        | bool                            | 发布是否成功                         |
| err         | string                          | 错误信息（如果有）                   |

### 删除 LOST & FOUND（Result）

| Key         | Value                         | Desc.              |
|-------------|-------------------------------|--------------------|
| type        | "result"                      | 操作类型标记       |
| result_type | "delete_lost", "delete_found" | 结果类型标记       |
| stat        | bool                          | 删除是否成功       |
| err         | string                        | 错误信息（如果有） |

### 筛选 LOST & FOUND（Result）

| Key         | Value    | Desc.                                   |
|-------------|----------|-----------------------------------------|
| type        | "result" | 操作类型标记                            |
| result_type | "select" | 结果类型标记                            |
| stat        | bool     | 筛选是否成功                            |
| err         | string   | 错误信息（如果有）                      |
| posts_len   | string   | 返回的条数                              |
| posts_id    | array    | 相关 LOST & FOUND 的 post_id 组成的数组 |

