# JSON数据格式

## 上传

|字段名称|可选值|
|-|-|
|type|"publish_lost", "publish_found"|
|userid_codec|string|
|item|JSON object|

## 推送

|字段名称|可选值|
|-|-|
|post_id|string|
|type|"post_lost", "post_found"|
|user|JSON string|
|item|JSON string|
|publish_time|string|

## item 对象

|字段名称|可选值|
|-|-|
|type|string|
|name|string|
|image|url|
|description|string|
|time|string|
|place|string|
|tags|array|

## user 对象

|字段名称|可选值|
|-|-|
|name|string|
|student_code|string|
|contact|string|

## 查询

|字段名称|可选值|
|-|-|
|user_flag|string,"0"|
|post_type|"lost","found","all"|
|item_type|string|
|keyword|string|

## 注册

|字段名称|可选值|
|-|-|
|userid_codec|string|
|user_name|string|
|student_code|string|
|password_codec|string|
|contact|string|

## 登录

|字段名称|可选值|
|-|-|
|userid_codec|string|
|password_codec|string|
