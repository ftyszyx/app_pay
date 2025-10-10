# LicenseHub

pay for app

# 系统说明

## 背景：

我做了一个软件，想为这个软件做一个付费系统

需求如下：

用户可以在这个系统上买注册码，在我的软件输入后，就会绑定软件安装的设备。

1. 注册码需要有有效期
2. 注册码需要有设备绑定限制
3. 需要提供接口给客户端验证注册码是否有效


## 参考：

1. 展示页：https://tech.baiyue.one/#/byfaka
2. 前台：http://8.134.209.8:3232/
3. 后台：http://8.134.209.8:3232/admin#/
4. 说明文档：http://8.134.209.8:3200/
1. https://github.com/Baiyuetribe/kamiFaka


1. https://github.com/xiaoxiaoguai-yyds/xxgkami

相当于是一个商城：

https://github.com/macrozheng/mall

## 系统：

1. 用户可以购买系统的商品，这个商品对应的是一个注册码，
1. 注册码有有效期、不同有效期的价格不一样。
1. 购买完后，系统会自动生成一个注册码
1. 用户使用此注册码绑定设备后，系统会有记录
1. 注册码会限制绑定设备数量，默认是一个设备。


## todo

1. 增加 权限管理 使用- **权限**: Casbin
2. 支付功能:支付宝和微信 
1. 图片上传功能


1.  不需要支付
1. apps 增加app_valid_key字段，可以通过app_valid_key来验证注册码是否有效
1. apps 增加试用期时长字段，表示试用期时长
2. reg_code增加 类型字段code_type 0: 时间类型  1：次数类型
3. 如果是时间 类型，reg_code增加 expire_time字段，表示过期时间,还有使用时间 use_time字段，表示使用时间
4. 如果是次数类型，reg_code增加 total_count字段，表示总次数，还有使用次数 use_count字段，表示使用次数
5. reg_code增加device_id字段，表示绑定的设备id
1. 增加一个api 接口，可以查询注册码是否有效,
    {
        "code": "123456",
        "app_key": "123456",
        "device_id": "123456"
    }
    返回 ：
    成功:返回过期时间或者还有多少次
    失败：返回错误信息



2. 增加app_valid_key的验证api 接口，

LicenseHub - 许可证中心，突出软件许可管理的核心功能


## todos
1. 角色权限管理



