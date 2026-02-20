// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Token bucket rate limiter.

use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::Mutex;

use crate::error::{HttpError, HttpResult};

/// Token bucket rate limiter.
///
/// Implements the token bucket algorithm for rate limiting.
/// Default: 10 tokens/second (Zalo OA API limit).
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<Bucket>>,
}

struct Bucket {
    tokens: f64,
    last_update: Instant,
    capacity: f64,
    refill_rate: f64,
}

impl RateLimiter {
    /// Creates new rate limiter.
    ///
    /// # Arguments
    ///
    /// * `refill_rate` — tokens per second (default: 10 for Zalo OA)
    /// * `capacity` — max bucket size (default: same as refill_rate)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::rate_limiter::RateLimiter;
    ///
    /// let limiter = RateLimiter::new(10.0, 10.0); // 10 req/s
    /// ```
    #[must_use]
    pub fn new(refill_rate: f64, capacity: f64) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Bucket {
                tokens: capacity,
                last_update: Instant::now(),
                capacity,
                refill_rate,
            })),
        }
    }

    /// Creates default limiter for Zalo OA API (10 req/s).
    #[must_use]
    pub fn zalo_default() -> Self {
        Self::new(10.0, 10.0)
    }

    /// Acquires a token, waiting if necessary.
    ///
    /// # Errors
    ///
    /// Returns error if wait time exceeds timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::rate_limiter::RateLimiter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let limiter = RateLimiter::zalo_default();
    ///     limiter.acquire().await.unwrap();
    ///     // Make API call...
    /// }
    /// ```
    pub async fn acquire(&self) -> HttpResult<()> {
        self.acquire_with_timeout(Duration::from_secs(60)).await
    }

    /// Acquires a token with timeout.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::RateLimited`] if timeout exceeded.
    pub async fn acquire_with_timeout(&self, timeout: Duration) -> HttpResult<()> {
        let mut bucket = self.inner.lock().await;
        bucket.refill();

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            Ok(())
        } else {
            let wait_secs = (1.0 - bucket.tokens) / bucket.refill_rate;
            let wait_duration = Duration::from_secs_f64(wait_secs);

            if wait_duration > timeout {
                return Err(HttpError::RateLimited);
            }

            drop(bucket);
            tokio::time::sleep(wait_duration).await;

            let mut bucket = self.inner.lock().await;
            bucket.refill();
            bucket.tokens -= 1.0;
            Ok(())
        }
    }

    /// Returns current available tokens.
    pub async fn available_tokens(&self) -> f64 {
        let bucket = self.inner.lock().await;
        bucket.tokens
    }
}

impl Bucket {
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);
        self.last_update = now;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn allows_burst_up_to_capacity() {
        let limiter = RateLimiter::new(10.0, 5.0);
        
        for _ in 0..5 {
            limiter.acquire().await.unwrap();
        }
    }

    #[tokio::test]
    async fn rate_limits_after_capacity_exhausted() {
        let limiter = RateLimiter::new(10.0, 2.0);
        
        limiter.acquire().await.unwrap();
        limiter.acquire().await.unwrap();
        
        let tokens = limiter.available_tokens().await;
        assert!(tokens < 1.0);
    }

    #[tokio::test]
    async fn creates_zalo_default() {
        let limiter = RateLimiter::zalo_default();
        assert!(limiter.available_tokens().await <= 10.0);
    }
}
