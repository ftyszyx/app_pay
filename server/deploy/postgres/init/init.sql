
-- 角色
DROP TABLE IF EXISTS "roles" CASCADE;
CREATE TABLE "roles" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO "roles" ("name") VALUES ('admin'), ('user');

-- 用户
DROP TABLE IF EXISTS "user" CASCADE;
CREATE TABLE "user" (
    "id" SERIAL PRIMARY KEY,
    "username" VARCHAR(255) NOT NULL UNIQUE,
    "password" VARCHAR(255) NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "balance" BIGINT NOT NULL DEFAULT 0,
    "inviter_id" INTEGER ,
    "invite_count" INTEGER NOT NULL DEFAULT 0,
    "invite_rebate_total" BIGINT NOT NULL DEFAULT 0,
    "role_id" INTEGER,
    CONSTRAINT "fk_user_role_id" FOREIGN KEY ("role_id") REFERENCES "roles" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);

-- 产品分类
DROP TABLE IF EXISTS "product_categories" CASCADE;
CREATE TABLE "product_categories" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "sort_order" INTEGER NOT NULL DEFAULT 0,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 产品
DROP TABLE IF EXISTS "products" CASCADE;
CREATE TABLE "products" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "price" INTEGER NOT NULL,
    "category_id" INTEGER NOT NULL,
    "image_url" VARCHAR,
    "tags" JSONB,
    "sort_order" INTEGER NOT NULL DEFAULT 0,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "delivery_mode" SMALLINT NOT NULL DEFAULT 0,
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "description" TEXT,
    CONSTRAINT "fk_product_category_id"
    FOREIGN KEY ("category_id")
    REFERENCES "product_categories" ("id")
    ON DELETE CASCADE
    ON UPDATE CASCADE
);


-- 订单
DROP TABLE IF EXISTS "orders" CASCADE;
CREATE TABLE "orders" (
    "id" SERIAL PRIMARY KEY,
    "order_sn" VARCHAR NOT NULL UNIQUE,
    "user_id" INTEGER NOT NULL,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "product_info" JSONB NOT NULL,
    "payment_method" VARCHAR,
    "original_price" DECIMAL(10, 2) NOT NULL,
    "coupon_info" JSONB,
    "final_price" DECIMAL(10, 2) NOT NULL,
    "remark" TEXT,
    "transaction_time" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "fk_order_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE
);

-- 注册码
DROP TABLE IF EXISTS "registration_codes" CASCADE;
CREATE TABLE "reg_codes" (
    "id" SERIAL PRIMARY KEY,
    "code" VARCHAR NOT NULL UNIQUE,
    "product_id" INTEGER NOT NULL,
    "order_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "device_info" JSONB,
    "expires_at" TIMESTAMPTZ,
    "max_devices" INTEGER NOT NULL DEFAULT 1,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "binding_time" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT "fk_reg_code_product_id" FOREIGN KEY ("product_id") REFERENCES "products" ("id") ON DELETE CASCADE,
    CONSTRAINT "fk_reg_code_order_id" FOREIGN KEY ("order_id") REFERENCES "orders" ("id") ON DELETE CASCADE,
    CONSTRAINT "fk_reg_code_user_id" FOREIGN KEY ("user_id") REFERENCES "user" ("id") ON DELETE CASCADE
);

--优惠券 
DROP TABLE IF EXISTS "coupons" CASCADE;
CREATE TABLE "coupons" (
    "id" SERIAL PRIMARY KEY,
    "code" VARCHAR NOT NULL UNIQUE,
    "status" SMALLINT NOT NULL DEFAULT 0,
    "scope_type" SMALLINT NOT NULL DEFAULT 0,
    "discount_type" SMALLINT NOT NULL,
    "scope_info" JSONB,
    "discount_info" JSONB NOT NULL,
    "expires_at" TIMESTAMPTZ,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
