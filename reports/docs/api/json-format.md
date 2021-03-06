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
| itemid      | string | 物品唯一标识码                                      |
| type        | string | 物品类型，目前包括 "cards"、"credentials"、"keys"、 "others" |
| name        | string | 物品名称                                            |
| image       | string | 图片（使用 base64），目前限制 1 张图片              |
| desc        | string | 备注                                                |
| pickup_time | string | 拾取时间                                            |
| place       | string | 拾取地点                                            |
| contact     | string | 联系方式（默认填写用户 contact）                    |
| post_time   | string | 发布时间                                            |

### userdata

| Key      | Value  | Desc.                                                    |
|----------|--------|----------------------------------------------------------|
| username | string | 用户名                                                   |
| contact  | string | 联系方式（用于在发布 LOST & FOUND 时自动填充联系方式）   |
| group    | string | 用户组（比如 normal 表示普通用户，admin 表示管理员用户） |

### err 值定义

| Value | Desc.                                   |
|-------|-----------------------------------------|
| 0101  | 注册失败：用户已存在                    |
| 0102  | 注册失败：未知错误                      |
| 0201  | 登录失败：用户不存在                    |
| 0202  | 登录失败：用户名或密码错误              |
| 0301  | 查询用户失败：userid 不合法或用户不存在 |
| 0401  | 获取数据失败：userid 不合法或用户不存在 |
| 0402  | 获取数据失败：未知错误                  |
| 0501  | 发布失败：userid 不合法或用户不存在 |

## 操作类型 (Send)

### 注册（OK）

| Key      | Value      | Desc.                                                  |
|----------|------------|--------------------------------------------------------|
| type     | "register" | 操作类型标记                                           |
| username | string     | 用户名                                                 |
| password | string     | 经过处理的密码                                         |
| contact  | string     | 联系方式（用于在发布 LOST & FOUND 时自动填充联系方式） |

### 登录（OK）

| Key      | Value   | Desc.          |
|----------|---------|----------------|
| type     | "login" | 操作类型标记   |
| username | string  | 用户名         |
| password | string  | 经过处理的密码 |

### 查询用户是否存在（OK）

| Key    | Value        | Desc.            |
|--------|--------------|------------------|
| type   | "query_user" | 操作类型标记     |
| userid | string       | 用户的唯一识别码 |

### 获取用户数据（OK）

| Key    | Value            | Desc.            |
|--------|------------------|------------------|
| type   | "query_userdata" | 操作类型标记     |
| userid | string           | 用户的唯一识别码 |

### 发布 LOST & FOUND（OK）

| Key    | Value                           | Desc.            |
|--------|---------------------------------|------------------|
| type   | "publish"            | 操作类型标记     |
| lof    | true / false                    | islost           |
| userid | string                          | 用户的唯一识别码 |
| item   | JSON object                     | 发布的物品       |

### 删除 LOST & FOUND（OK）

| Key    | Value                         | Desc.            |
|--------|-------------------------------|------------------|
| type   | "delete" | 操作类型标记     |
| userid | string                        | 用户的唯一识别码 |
| itemid | string                        | 物品唯一标识码   |

### 筛选 LOST & FOUND（OK）

| Key        | Value                  | Desc.                                          |
|------------|------------------------|------------------------------------------------|
| type       | "select_all"               | 操作类型标记 |
| lof        | true / false                | |
| item_type  | "cards" / "keys" / .... | |
| time_begin | string(number), "none" | 筛选条件：开始的时间戳               |
| time_end   | string(number), "none" | 筛选条件：结束的时间戳      |

### 筛选我的 LOST & FOUND

| Key        | Value                  | Desc.              |
|------------|------------------------|--------------------|
| type       | "select_me"            | 操作类型标识       |
| userid     | string                 | 用户的唯一识别码   |
| lof        | true / false           | islost             |

## 操作类型 (Recv)

### 注册（Result）（OK）

| Key         | Value      | Desc.              |
|-------------|------------|--------------------|
| type        | "result"   | 操作类型标记       |
| result_type | "register" | 结果类型标记       |
| stat        | bool       | 注册是否成功       |
| err         | string     | 错误信息（如果有） |

### 登录（Result）（OK）

| Key         | Value    | Desc.                        |
|-------------|----------|------------------------------|
| type        | "result" | 操作类型标记                 |
| result_type | "login"  | 结果类型标记                 |
| stat        | bool     | 登录是否成功                 |
| err         | string   | 错误信息（如果有）           |
| userid      | string   | 用户的唯一识别码（如果成功） |

### 查询用户是否存在（Result）（OK）

| Key         | Value        | Desc.              |
|-------------|--------------|--------------------|
| type        | "result"     | 操作类型标记       |
| result_type | "query_user" | 结果类型标记       |
| stat        | bool         | 查询是否成功       |
| err         | string       | 错误信息（如果有） |

### 获取用户数据（Result）（OK）

| Key         | Value            | Desc.              |
| ----------- | ---------------- | ------------------ |
| type        | "result"         | 操作类型标记       |
| result_type | "query_userdata" | 结果类型标记       |
| stat        | bool             | 成功状态           |
| err         | string           | 错误信息（如果有） |
| data        | userdata         | 用户数据           |

### 发布 LOST & FOUND（Result）（OK）

| Key         | Value                           | Desc.                                |
|-------------|---------------------------------|--------------------------------------|
| type        | "result"                        | 操作类型标记                         |
| result_type | "publish" | 结果类型标记                         |
| stat        | bool                            | 发布是否成功                         |
| err         | string                          | 错误信息（如果有）                   |

### 删除 LOST & FOUND（Result）（OK）

| Key         | Value                         | Desc.              |
|-------------|-------------------------------|--------------------|
| type        | "result"                      | 操作类型标记       |
| result_type | "delete" | 结果类型标记       |
| stat        | bool                          | 删除是否成功       |
| err         | string                        | 错误信息（如果有） |

### 筛选 LOST & FOUND（Result）（OK）

| Key         | Value    | Desc.                                   |
|-------------|----------|-----------------------------------------|
| type        | "result" | 操作类型标记                            |
| result_type | "select" | 结果类型标记                            |
| stat        | bool     | 筛选是否成功                            |
| err         | string   | 错误信息（如果有）                      |
| posts_len   | string   | 返回的条数                              |
| posts_id    | array    | 相关 LOST & FOUND 组成的数组 |

