use crate::types::error::AppError;
use redis::{AsyncCommands, Client, FromRedisValue, RedisError, ToRedisArgs};
use serde::{Serialize, de::DeserializeOwned};
use std::time::Duration;

#[derive(Clone)]
pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    /// 创建一个新的 RedisCache 实例
    pub fn new(redis_url: &str) -> Result<Self, RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }

    /// 获取一个异步连接
    async fn get_conn(&self) -> Result<redis::aio::Connection, RedisError> {
        self.client.get_async_connection().await
    }

    /// 从缓存中获取一个值
    /// 值会从 JSON 字符串反序列化
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, AppError> {
        let mut conn = self.get_conn().await?;
        let result: Option<String> = conn.get(key).await?;

        match result {
            Some(val_str) => {
                let val: T = serde_json::from_str(&val_str)?;
                Ok(Some(val))
            }
            None => Ok(None),
        }
    }

    /// 向缓存中设置一个值
    /// 值会被序列化为 JSON 字符串
    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<Duration>,
    ) -> Result<(), AppError> {
        let mut conn = self.get_conn().await?;
        let val_str = serde_json::to_string(value)?;

        if let Some(duration) = ttl {
            conn.set_ex(key, val_str, duration.as_secs() as usize)
                .await?;
        } else {
            conn.set(key, val_str).await?;
        }
        Ok(())
    }

    /// 从缓存中删除一个值
    pub async fn del(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.get_conn().await?;
        conn.del(key).await?;
        Ok(())
    }
}

// 将 Redis 错误转换为自定义的 AppError
impl From<RedisError> for AppError {
    fn from(err: RedisError) -> Self {
        tracing::error!("Redis error: {:?}", err);
        AppError::ExternalService {
            service: "Redis".to_string(),
            error: err.to_string(),
        }
    }
}

// 将 serde_json 错误转换为自定义的 AppError
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        tracing::error!("Serialization/Deserialization error: {:?}", err);
        AppError::BusinessLogic {
            code: "SERIALIZATION_ERROR".to_string(),
            message: err.to_string(),
        }
    }
}
