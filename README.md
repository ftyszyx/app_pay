# LicenseHub

## 系统说明

一个提供注册码生成和验证的系统

## 背景：

最近写了一个pc端的程序，考虑如何通过软件获取收益呢？

一般的方案通过软件注册码的方式。

1. 用户下载软件无门槛
2. 软件有一定的试用期，在试用期内可以免费使用
3. 如果超过试用期，打开软件时需要填写注册码。
4. 注册码需要软件开发者提供，每个注册码会有时间限制，也可以是不限时间。

搜索了一些目前开源的方案：

1. kamiFaka：https://github.com/Baiyuetribe/kamiFaka (注册码生成方式不满足需求)
1. dujiaoka: https://github.com/assimon/dujiaoka (注册码生成方式不满足需求)
1. xxgkamiexe: https://github.com/xiaoxiaoguai-yyds/xxgkamiexe (基本满足需求，可是无试用期)

其中xxgkamiexe大部分满足，但是没有实现试用期的功能
所以自己实现了一个。

## 技术方案

整个系统采用前后端分离设计。
1. 前端就是一个管理员后台，使用vue3.
2. 后端：最近在学 rust，想拿一个项目练手，所以就用 rust了。 web框架使用salvo：
https://github.com/salvo-rs/salvo
3. 先不加入支付，只用实现注册码生成和验证接口即可。


## 项目结构

admin: 前端代码

server: 后端rust代码

pub: 服务器部署相关

## 本地测试


### 启动服务器
需要有redis和postgres环境
数据库文件在pub/deploy/postgres/init.sql
初始账号密码：admin/admin

```
cd server
cargo run
```

### 启动前端
需要有node.js环境

```
cd admin
npm run dev
```

## 服务器部署

需要有docker 环境

### 启动服务器

```
cd  pub
update_server.sh

```
### 启动前端

需要nginx配置

```
location / {
        try_files $uri $uri/ /index.html;
    }
```

编译生成对应的前端资源
```
python build_web.py --base-url {your server_url}
```

将目录下的资源同步到网站目录即可

```
rsync -avz web/ /opt/1panel/www/sites/index/ 
```

## todos

1. 角色权限管理



