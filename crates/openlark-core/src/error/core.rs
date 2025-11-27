//! 重试与恢复策略（V3 错误体系复用）

use std::time::Duration;

/// 重试策略（供 CoreError 使用）
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub backoff_factor: f64,
    pub max_delay: Option<Duration>,
}

impl RetryPolicy {
    /// 创建不重试策略
    pub fn no_retry() -> Self {
        Self {
            max_retries: 0,
            base_delay: Duration::from_secs(1),
            backoff_factor: 1.0,
            max_delay: None,
        }
    }

    /// 创建固定延迟重试策略
    pub fn fixed(max_retries: u32, delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay: delay,
            backoff_factor: 1.0,
            max_delay: Some(delay),
        }
    }

    /// 创建指数退避重试策略
    pub fn exponential(max_retries: u32, base_delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay,
            backoff_factor: 2.0,
            max_delay: Some(Duration::from_secs(300)), // 5分钟最大延迟
        }
    }

    /// 是否可重试
    pub fn is_retryable(&self) -> bool {
        self.max_retries > 0
    }

    /// 计算重试延迟
    pub fn retry_delay(&self, attempt: u32) -> Option<Duration> {
        if attempt >= self.max_retries {
            return None;
        }

        let delay = if self.backoff_factor == 1.0 {
            self.base_delay
        } else {
            let seconds = self.base_delay.as_secs_f64() * self.backoff_factor.powi(attempt as i32);
            Duration::from_secs_f64(seconds)
        };

        Some(self.max_delay.map_or(delay, |max| delay.min(max)))
    }

    /// 直接获取延迟（超过最大重试返回0）
    pub fn delay(&self, attempt: u32) -> Duration {
        self.retry_delay(attempt).unwrap_or(Duration::ZERO)
    }

    /// 是否使用指数退避
    pub fn use_exponential_backoff(&self) -> bool {
        self.backoff_factor > 1.0
    }

    /// 最大重试次数（保持旧接口）
    pub fn max_retries(&self) -> u32 {
        self.max_retries
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::exponential(3, Duration::from_secs(1))
    }
}

/// 恢复策略
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// 重试并使用指数退避
    RetryWithBackoff,
    /// 验证后重试
    ValidateAndRetry,
    /// 重新认证
    Reauthenticate,
    /// 请求权限
    RequestPermission,
    /// 手动干预
    ManualIntervention,
    /// 延迟重试
    RetryWithDelay,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_policy_exponential_delay() {
        let policy = RetryPolicy::exponential(3, Duration::from_millis(100));
        assert!(policy.is_retryable());
        assert!(policy.use_exponential_backoff());
        assert_eq!(policy.retry_delay(0), Some(Duration::from_millis(100)));
        assert!(policy.retry_delay(3).is_none());
    }

    #[test]
    fn retry_policy_fixed_delay() {
        let policy = RetryPolicy::fixed(2, Duration::from_secs(1));
        assert_eq!(policy.retry_delay(1), Some(Duration::from_secs(1)));
        assert_eq!(policy.delay(5), Duration::ZERO);
    }
}
