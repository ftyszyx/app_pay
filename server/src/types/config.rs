use std::env;
use crate::types::error::AppError;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub server: ServerConfig,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
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

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Config {
            database: DatabaseConfig::from_env()?,
            redis: RedisConfig::from_env()?,
            jwt: JwtConfig::from_env()?,
            server: ServerConfig::from_env()?,
        })
    }
}

impl DatabaseConfig {
    fn from_env() -> Result<Self, AppError> {
        Ok(DatabaseConfig {
            url: env::var("DATABASE_URL")
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