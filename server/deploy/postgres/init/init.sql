--add by zyx
-- role order user 表 使用deleted_at 字段来标记删除

-- 角色
DROP TABLE IF EXISTS "roles" CASCADE;
CREATE TABLE "roles" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    "remark" TEXT,
    "deleted_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO "roles" ("id","name") VALUES (1,'admin'), (2,'user'), (3,'guest');

-- 用户
DROP TABLE IF EXISTS "users" CASCADE;
CREATE TABLE "users" (
    "id" SERIAL PRIMARY KEY,
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    "balance" BIGINT NOT NULL DEFAULT 0,
    "invite_rebate_total" BIGINT NOT NULL DEFAULT 0, -- 邀请总收益
    "role_id" INTEGER,
    CONSTRAINT "fk_user_role_id" FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "chk_balance_positive" CHECK ("balance" >= 0)
);
CREATE INDEX idx_user_username ON "users" ("username");
INSERT INTO "users" ( "username", "password", "role_id") VALUES ( 'admin', '$2b$12$/MZyRsK.DcYHh6x4qCy6IOjxO/Wd4RlPSbW.7OiAYqTY4U4CipDIS', 1);
INSERT INTO "users" ("username", "password", "role_id") VALUES ( 'user', '$2b$12$afVdZp0thpWjIQt/oib50OhlJW3UsAjn9r808ufcLMl2mLgVsDciK', 2);
INSERT INTO "users" ("username", "password", "role_id") VALUES ( 'guest', '$2b$12$tOtsPnX8UlVgqYL1UbYlHuiCBjEgljGz3xhqbXstwoHdD3rMquSlW', 3);

-- 产品表
DROP TABLE IF EXISTS "apps" CASCADE;
CREATE TABLE "apps" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "app_id" VARCHAR(255) NOT NULL UNIQUE,
    "app_vername" VARCHAR(255) NOT NULL,
    "app_vercode" INTEGER NOT NULL,
    "app_download_url" VARCHAR(255) NOT NULL,
    "app_res_url" VARCHAR(255) NOT NULL,
    "app_update_info" TEXT,
    "sort_order" INTEGER NOT NULL DEFAULT 0,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    CONSTRAINT "chk_status_range" CHECK ("status" IN (0, 1))
);
COMMENT ON COLUMN "apps"."status" IS '0: 下架 1: 上架';
CREATE INDEX idx_apps_app_id ON "apps" ("app_id");

-- 商品表
DROP TABLE IF EXISTS "products" CASCADE;
CREATE TABLE "products" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    "price" INTEGER NOT NULL,
    "app_id" INTEGER NOT NULL,
    "product_id" VARCHAR(255) NOT NULL UNIQUE,
    "add_valid_days" INTEGER NOT NULL DEFAULT 0, -- 添加有效天数>0
    "image_url" VARCHAR,
    "tags" TEXT,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "remark" TEXT,
    "deleted_at" TIMESTAMPTZ,
    CONSTRAINT "fk_product_app_id" FOREIGN KEY ("app_id") REFERENCES "apps" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "chk_price_positive" CHECK ("price" > 0),
    CONSTRAINT "chk_status_range" CHECK ("status" IN (0, 1)),
    CONSTRAINT "chk_add_valid_days_positive" CHECK ("add_valid_days" > 0)
);
CREATE INDEX idx_products_app_id ON "products" ("app_id");
COMMENT ON COLUMN "products"."status" IS '0: 下架 1: 上架';

-- 支付方式
DROP TABLE IF EXISTS "pay_methods" CASCADE;
CREATE TABLE "pay_methods" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    "status" SMALLINT NOT NULL DEFAULT 1,
    "remark" TEXT,
    "config" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    CONSTRAINT "chk_status_range" CHECK ("status" IN (0, 1))
);
COMMENT ON COLUMN "pay_methods"."status" IS '0: 禁用 1: 启用';
CREATE INDEX idx_pay_methods_name ON "pay_methods" ("name");

-- 订单
DROP TABLE IF EXISTS "orders" CASCADE;
CREATE TABLE "orders" (
    "id" SERIAL PRIMARY KEY,
    "order_id" VARCHAR NOT NULL UNIQUE, --订单号
    "user_info" JSONB, -- 可以保留作为扩展信息
    "status" SMALLINT NOT NULL DEFAULT 0 ,
    "pay_method_id" INTEGER NOT NULL, -- 支付方式
    "original_price" BIGINT NOT NULL DEFAULT 0, -- 原价
    "final_price" BIGINT NOT NULL DEFAULT 0, -- 实付
    "remark" TEXT, -- 订单备注
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "created_by" INTEGER NOT NULL, -- 创建者
    "updated_by" INTEGER NOT NULL, -- 更新者
    CONSTRAINT "fk_order_pay_method_id" FOREIGN KEY ("pay_method_id") REFERENCES "pay_methods" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_order_created_by" FOREIGN KEY ("created_by") REFERENCES "users" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_order_updated_by" FOREIGN KEY ("updated_by") REFERENCES "users" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "chk_final_price_positive" CHECK (final_price >= 0),
    CONSTRAINT "chk_original_price_positive" CHECK (original_price >= 0),
    CONSTRAINT "chk_status_range" CHECK (status IN (0, 1, 2, 3))
);
CREATE INDEX idx_orders_order_id ON "orders" ("order_id");
CREATE INDEX idx_orders_created_by ON "orders" ("created_by");
CREATE INDEX idx_orders_updated_by ON "orders" ("updated_by");
CREATE INDEX idx_orders_pay_method_id ON "orders" ("pay_method_id");
COMMENT ON COLUMN "orders"."status" IS '0: 待支付 1: 已支付 2: 已取消 3: 已退款';

--优惠券 
DROP TABLE IF EXISTS "coupons" CASCADE;
CREATE TABLE "coupons" (
    "id" SERIAL PRIMARY KEY,
    "code" VARCHAR NOT NULL UNIQUE, -- 优惠券码
    "name" VARCHAR(255) NOT NULL UNIQUE, -- 优惠券名称
    "status" SMALLINT NOT NULL DEFAULT 0, -- 优惠券状态 0: 禁用 1: 启用
    "discount_type" SMALLINT NOT NULL, -- 优惠券折扣类型 0: 百分比 1: 折扣金额
    "discount_value" BIGINT NOT NULL, -- 优惠券折扣值 百分比或者是折扣金额
    "min_purchase_amount" BIGINT NOT NULL DEFAULT 0, -- 最低购买金额
    "start_time" TIMESTAMPTZ, -- 优惠券开始时间
    "end_time" TIMESTAMPTZ, -- 优惠券结束时间
    "usage_limit" INTEGER NOT NULL DEFAULT 0, -- 优惠券使用次数限制
    "scope_type" SMALLINT NOT NULL DEFAULT 0, -- 优惠券范围类型 0: 所有商品 1: 指定应用 2: 指定商品
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    CONSTRAINT "chk_discount_type_range" CHECK ("discount_type" IN (0, 1)),
    CONSTRAINT "chk_scope_type_range" CHECK ("scope_type" IN (0, 1, 2)),
    CONSTRAINT "chk_min_purchase_amount_positive" CHECK ("min_purchase_amount" >= 0),
    CONSTRAINT "chk_status_range" CHECK ("status" IN (0, 1))
);
CREATE INDEX idx_coupons_code ON "coupons" ("code");
CREATE INDEX idx_coupons_name ON "coupons" ("name");
CREATE INDEX idx_coupons_status ON "coupons" ("status");
CREATE INDEX idx_coupons_discount_type ON "coupons" ("discount_type");
CREATE INDEX idx_coupons_scope_type ON "coupons" ("scope_type");
CREATE INDEX idx_coupons_min_purchase_amount ON "coupons" ("min_purchase_amount");
COMMENT ON COLUMN "coupons"."discount_type" IS '0: 百分比 1: 折扣金额';
COMMENT ON COLUMN "coupons"."scope_type" IS '0: 所有商品 1: 指定应用 2: 指定商品';
COMMENT ON COLUMN "coupons"."status" IS '0: 禁用 1: 启用';

-- 优惠券关联应用
DROP TABLE IF EXISTS "coupons_apps" CASCADE;
CREATE TABLE "coupons_apps" (
    "id" SERIAL PRIMARY KEY,
    "coupon_id" INTEGER NOT NULL,
    "app_id" INTEGER NOT NULL,
    CONSTRAINT "fk_coupons_apps_coupon_id" FOREIGN KEY ("coupon_id") REFERENCES "coupons" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_coupons_apps_app_id" FOREIGN KEY ("app_id") REFERENCES "apps" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_coupons_apps_coupon_id ON "coupons_apps" ("coupon_id");
CREATE INDEX idx_coupons_apps_app_id ON "coupons_apps" ("app_id");

-- 优惠券关联商品
DROP TABLE IF EXISTS "coupons_products" CASCADE;
CREATE TABLE "coupons_products" (
    "id" SERIAL PRIMARY KEY,
    "coupon_id" INTEGER NOT NULL,
    "product_id" INTEGER NOT NULL,
    CONSTRAINT "fk_coupons_products_coupon_id" FOREIGN KEY ("coupon_id") REFERENCES "coupons" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_coupons_products_product_id" FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_coupons_products_coupon_id ON "coupons_products" ("coupon_id");
CREATE INDEX idx_coupons_products_product_id ON "coupons_products" ("product_id");

-- 订单商品
DROP TABLE IF EXISTS "order_products" CASCADE;
CREATE TABLE "order_products" (
    "id" SERIAL PRIMARY KEY,
    "order_id" INTEGER NOT NULL,
    "product_id" INTEGER NOT NULL,
    "num" INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT "fk_order_product_order_id" FOREIGN KEY ("order_id") REFERENCES "orders" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_order_product_product_id" FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_order_products_order_id ON "order_products" ("order_id");
CREATE INDEX idx_order_products_product_id ON "order_products" ("product_id");

-- 订单优惠券
DROP TABLE IF EXISTS "order_coupons" CASCADE;
CREATE TABLE "order_coupons" (
    "id" SERIAL PRIMARY KEY,
    "order_id" INTEGER NOT NULL,
    "coupon_id" INTEGER NOT NULL,
    "num" INTEGER NOT NULL DEFAULT 0,
    CONSTRAINT "fk_order_coupon_order_id" FOREIGN KEY ("order_id") REFERENCES "orders" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_order_coupon_coupon_id" FOREIGN KEY ("coupon_id") REFERENCES "coupons" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_order_coupons_order_id ON "order_coupons" ("order_id");
CREATE INDEX idx_order_coupons_coupon_id ON "order_coupons" ("coupon_id");

-- 注册码
DROP TABLE IF EXISTS "reg_codes" CASCADE;
CREATE TABLE "reg_codes" (
    "id" SERIAL PRIMARY KEY,
    "code" VARCHAR NOT NULL UNIQUE,
    "app_id" INTEGER NOT NULL, -- 应用ID
    "bind_device_info" JSONB, -- 绑定设备信息
    "valid_days" INTEGER NOT NULL DEFAULT 0, -- 有效天数 1: 1天 2: 3天 3: 7天 4: 30天
    "max_devices" INTEGER NOT NULL DEFAULT 1, -- 最大绑定设备数
    "status" SMALLINT NOT NULL DEFAULT 0, -- 状态 0: 未使用 1: 已使用 2: 已过期
    "binding_time" TIMESTAMPTZ, -- 绑定时间
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "deleted_at" TIMESTAMPTZ,
    CONSTRAINT "fk_reg_code_app_id" FOREIGN KEY ("app_id") REFERENCES "apps" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_reg_codes_app_id ON "reg_codes" ("app_id");
CREATE INDEX idx_reg_codes_status ON "reg_codes" ("status");

-- 订单对应的注册码
DROP TABLE IF EXISTS "order_reg_codes" CASCADE;
CREATE TABLE "order_reg_codes" (
    "id" SERIAL PRIMARY KEY,
    "order_id" INTEGER NOT NULL,
    "reg_code_id" INTEGER NOT NULL,
    CONSTRAINT "fk_order_reg_code_order_id" FOREIGN KEY ("order_id") REFERENCES "orders" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_order_reg_code_reg_code_id" FOREIGN KEY ("reg_code_id") REFERENCES "reg_codes" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_order_reg_codes_order_id ON "order_reg_codes" ("order_id");
CREATE INDEX idx_order_reg_codes_reg_code_id ON "order_reg_codes" ("reg_code_id");

--邀请记录
DROP TABLE IF EXISTS "invite_records" CASCADE;
CREATE TABLE "invite_records" (
    "id" SERIAL PRIMARY KEY,
    "user_id" INTEGER NOT NULL,  --用户id
    "inviter_user_id" INTEGER NOT NULL,  --邀请人userid
    "user_info" JSONB,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "fk_invite_record_user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT "fk_invite_record_inviter_user_id" FOREIGN KEY ("inviter_user_id") REFERENCES "users" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
CREATE INDEX idx_invite_records_user_id ON "invite_records" ("user_id");
CREATE INDEX idx_invite_records_invite_user_id ON "invite_records" ("inviter_user_id");

-- casbin rule
DROP TABLE IF EXISTS "casbin_rule" CASCADE;
CREATE TABLE "casbin_rule" (
    "id" SERIAL PRIMARY KEY,
    "ptype" VARCHAR NOT NULL,
    "v0" VARCHAR NOT NULL,
    "v1" VARCHAR NOT NULL,
    "v2" VARCHAR NOT NULL,
    "v3" VARCHAR,
    "v4" VARCHAR,
    "v5" VARCHAR,
    CONSTRAINT "unique_key" UNIQUE("ptype", "v0", "v1", "v2", "v3", "v4", "v5")
);
