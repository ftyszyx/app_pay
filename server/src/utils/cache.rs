use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// 缓存项结构
#[derive(Clone, Debug)]
struct CacheItem<V> {
    value: V,
    expires_at: Option<Instant>,
    created_at: Instant,
}

impl<V> CacheItem<V> {
    fn new(value: V, ttl: Option<Duration>) -> Self {
        let created_at = Instant::now();
        let expires_at = ttl.map(|duration| created_at + duration);

        Self {
            value,
            expires_at,
            created_at,
        }
    }

    fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Instant::now() > expires_at
        } else {
            false
        }
    }
}

/// 内存缓存实现
#[derive(Clone)]
pub struct InMemoryCache<K, V> {
    store: Arc<RwLock<HashMap<K, CacheItem<V>>>>,
    default_ttl: Option<Duration>,
}

impl<K, V> InMemoryCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// 创建新的内存缓存
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Some(Duration::from_secs(300)), // 默认5分钟TTL
        }
    }

    /// 创建带有自定义默认TTL的缓存
    pub fn with_ttl(default_ttl: Duration) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Some(default_ttl),
        }
    }

    /// 插入缓存项
    pub fn set(&self, key: K, value: V, ttl: Option<Duration>) {
        let ttl = ttl.or(self.default_ttl);
        let item = CacheItem::new(value, ttl);

        if let Ok(mut store) = self.store.write() {
            store.insert(key, item);
        }
    }

    /// 获取缓存项
    pub fn get(&self, key: &K) -> Option<V> {
        if let Ok(store) = self.store.read() {
            if let Some(item) = store.get(key) {
                if !item.is_expired() {
                    return Some(item.value.clone());
                }
            }
        }
        None
    }

    /// 删除缓存项
    pub fn remove(&self, key: &K) -> Option<V> {
        if let Ok(mut store) = self.store.write() {
            store.remove(key).map(|item| item.value)
        } else {
            None
        }
    }

    /// 清空缓存
    pub fn clear(&self) {
        if let Ok(mut store) = self.store.write() {
            store.clear();
        }
    }

    /// 清理过期项
    pub fn cleanup_expired(&self) {
        if let Ok(mut store) = self.store.write() {
            store.retain(|_, item| !item.is_expired());
        }
    }

    /// 获取缓存统计信息
    pub fn stats(&self) -> CacheStats {
        if let Ok(store) = self.store.read() {
            let total_items = store.len();
            let expired_items = store.values().filter(|item| item.is_expired()).count();

            CacheStats {
                total_items,
                expired_items,
                active_items: total_items - expired_items,
            }
        } else {
            CacheStats::default()
        }
    }
}

impl<K, V> Default for InMemoryCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

/// 缓存统计信息
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_items: usize,
    pub expired_items: usize,
    pub active_items: usize,
}

/// 缓存键生成器
pub struct CacheKeyGenerator;

impl CacheKeyGenerator {
    /// 为用户生成缓存键
    pub fn user_key(user_id: i32) -> String {
        format!("user:{}", user_id)
    }

    /// 为角色生成缓存键
    pub fn role_key(role_id: i32) -> String {
        format!("role:{}", role_id)
    }

    /// 为应用生成缓存键
    pub fn app_key(app_id: i32) -> String {
        format!("app:{}", app_id)
    }

    /// 为列表查询生成缓存键
    pub fn list_key(entity: &str, page: u64, page_size: u64, filters: &str) -> String {
        format!("list:{}:{}:{}:{}", entity, page, page_size, filters)
    }
}

/// 缓存管理器 - 统一管理不同类型的缓存
pub struct CacheManager {
    pub user_cache: InMemoryCache<i32, entity::users::Model>,
    pub role_cache: InMemoryCache<i32, entity::roles::Model>,
    pub app_cache: InMemoryCache<i32, entity::apps::Model>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            user_cache: InMemoryCache::with_ttl(Duration::from_secs(600)), // 10分钟
            role_cache: InMemoryCache::with_ttl(Duration::from_secs(1800)), // 30分钟
            app_cache: InMemoryCache::with_ttl(Duration::from_secs(300)),  // 5分钟
        }
    }

    /// 清理所有过期缓存
    pub fn cleanup_all(&self) {
        self.user_cache.cleanup_expired();
        self.role_cache.cleanup_expired();
        self.app_cache.cleanup_expired();
    }

    /// 获取所有缓存统计
    pub fn get_all_stats(&self) -> HashMap<String, CacheStats> {
        let mut stats = HashMap::new();
        stats.insert("users".to_string(), self.user_cache.stats());
        stats.insert("roles".to_string(), self.role_cache.stats());
        stats.insert("apps".to_string(), self.app_cache.stats());
        stats
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局缓存管理器实例
pub static CACHE_MANAGER: std::sync::OnceLock<CacheManager> = std::sync::OnceLock::new();

/// 获取全局缓存管理器
pub fn get_cache_manager() -> &'static CacheManager {
    CACHE_MANAGER.get_or_init(CacheManager::default)
}

/// 缓存装饰器宏
#[macro_export]
macro_rules! cached_operation {
    ($cache:expr, $key:expr, $operation:expr) => {{
        if let Some(cached_result) = $cache.get(&$key) {
            tracing::debug!("Cache hit for key: {:?}", $key);
            cached_result
        } else {
            tracing::debug!("Cache miss for key: {:?}", $key);
            let result = $operation;
            $cache.set($key, result.clone(), None);
            result
        }
    }};
}
