use std::time::Instant;
use tracing::{info, warn};

/// 性能监控结构体
pub struct PerformanceMonitor {
    start_time: Instant,
    operation: String,
}

impl PerformanceMonitor {
    /// 创建新的性能监控实例
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            start_time: Instant::now(),
            operation: operation.into(),
        }
    }

    /// 记录操作完成并返回耗时
    pub fn finish(self) -> u64 {
        let elapsed = self.start_time.elapsed();
        let elapsed_ms = elapsed.as_millis() as u64;

        if elapsed_ms > 1000 {
            warn!(
                "Slow operation detected: {} took {}ms",
                self.operation, elapsed_ms
            );
        } else {
            info!(
                "Operation completed: {} took {}ms",
                self.operation, elapsed_ms
            );
        }

        elapsed_ms
    }

    /// 记录中间检查点
    pub fn checkpoint(&self, checkpoint: &str) {
        let elapsed = self.start_time.elapsed();
        info!(
            "Checkpoint '{}' in operation '{}': {}ms",
            checkpoint,
            self.operation,
            elapsed.as_millis()
        );
    }
}

/// 性能监控宏
#[macro_export]
macro_rules! monitor_performance {
    ($operation:expr, $code:block) => {{
        let monitor = $crate::utils::performance::PerformanceMonitor::new($operation);
        let result = $code;
        monitor.finish();
        result
    }};
}

/// 数据库查询性能监控
pub struct DatabasePerformanceMonitor;

impl DatabasePerformanceMonitor {
    /// 监控数据库查询性能
    pub fn monitor_query<T, F>(query_name: &str, query_fn: F) -> T
    where
        F: FnOnce() -> T,
    {
        let monitor = PerformanceMonitor::new(format!("DB Query: {}", query_name));
        let result = query_fn();
        let elapsed = monitor.finish();

        // 记录慢查询
        if elapsed > 500 {
            warn!("Slow database query: {} took {}ms", query_name, elapsed);
        }

        result
    }
}

/// API请求性能统计
pub struct ApiMetrics {
    pub total_requests: u64,
    pub average_response_time: f64,
    pub error_count: u64,
}

impl Default for ApiMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            average_response_time: 0.0,
            error_count: 0,
        }
    }
}

/// 简单的指标收集器
pub static API_METRICS: std::sync::Mutex<ApiMetrics> = std::sync::Mutex::new(ApiMetrics {
    total_requests: 0,
    average_response_time: 0.0,
    error_count: 0,
});

/// 更新API指标
pub fn update_api_metrics(response_time_ms: u64, is_error: bool) {
    if let Ok(mut metrics) = API_METRICS.lock() {
        metrics.total_requests += 1;

        if is_error {
            metrics.error_count += 1;
        }

        // 简单的移动平均计算
        let new_avg = (metrics.average_response_time * (metrics.total_requests - 1) as f64
            + response_time_ms as f64)
            / metrics.total_requests as f64;
        metrics.average_response_time = new_avg;
    }
}

/// 获取当前API指标
pub fn get_api_metrics() -> ApiMetrics {
    API_METRICS.lock().unwrap().clone()
}

impl Clone for ApiMetrics {
    fn clone(&self) -> Self {
        Self {
            total_requests: self.total_requests,
            average_response_time: self.average_response_time,
            error_count: self.error_count,
        }
    }
}
