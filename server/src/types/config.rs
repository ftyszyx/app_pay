use crate::types::error::AppError;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub server: ServerConfig,
    pub oss: OssConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub db_url: String,
    pub db_name: String,
    pub db_user: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_days: u32,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct OssConfig {
    pub region: String,
    pub bucket: String,
    // pub endpoint: String,
    pub role_arn: String,
    pub access_key_id: String,
    pub sts_expire_secs: u32,
    pub access_key_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Config {
            database: DatabaseConfig::from_env()?,
            redis: RedisConfig::from_env()?,
            jwt: JwtConfig::from_env()?,
            server: ServerConfig::from_env()?,
            oss: OssConfig::from_env()?,
        })
    }
}

impl DatabaseConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(DatabaseConfig {
            db_name: env::var("DB_NAME")
                .map_err(|_| AppError::Message("DB_NAME must be set".to_string()))?,
            db_user: env::var("DB_USER")
                .map_err(|_| AppError::Message("DB_USER must be set".to_string()))?,
            db_url: env::var("DB_URL")
                .map_err(|_| AppError::Message("DATABASE_URL must be set".to_string()))?,
            max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .map_err(|_| AppError::Message("Invalid DB_MAX_CONNECTIONS value".to_string()))?,
            min_connections: env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .map_err(|_| AppError::Message("Invalid DB_MIN_CONNECTIONS value".to_string()))?,
            connect_timeout_secs: env::var("DB_CONNECT_TIMEOUT")
                .unwrap_or_else(|_| "8".to_string())
                .parse()
                .map_err(|_| AppError::Message("Invalid DB_CONNECT_TIMEOUT value".to_string()))?,
        })
    }
}

impl RedisConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(RedisConfig {
            url: env::var("REDIS_URL")
                .map_err(|_| AppError::Message("REDIS_URL must be set".to_string()))?,
        })
    }
}

impl JwtConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(JwtConfig {
            secret: env::var("JWT_SECRET")
                .map_err(|_| AppError::Message("JWT_SECRET must be set".to_string()))?,
            expire_days: env::var("JWT_EXPIRE")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .map_err(|_| AppError::Message("Invalid JWT_EXPIRE value".to_string()))?,
        })
    }
}

impl ServerConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(ServerConfig {
            host: env::var("LISTEN_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("LISTEN_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| AppError::Message("Invalid LISTEN_PORT value".to_string()))?,
        })
    }
}

impl OssConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(OssConfig {
            region: env::var("OSS_REGION").unwrap_or_default(),
            bucket: env::var("OSS_BUCKET").unwrap_or_default(),
            // endpoint: env::var("OSS_ENDPOINT").unwrap_or_default(),
            role_arn: env::var("OSS_ROLE_ARN").unwrap_or_default(),
            sts_expire_secs: env::var("OSS_STS_EXPIRE_SECS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse::<u32>()
                .map_err(|_| AppError::Message("Invalid OSS_STS_EXPIRE_SECS value".to_string()))?,
            access_key_id: env::var("OSS_ACCESS_KEY_ID").unwrap_or_default(),
            access_key_secret: env::var("OSS_ACCESS_KEY_SECRET").unwrap_or_default(),
        })
    }
}
