use std::future::Future;
use futures::future::join_all;
use rand::Rng;
use anyhow::Result;
use tokio::sync::Semaphore;

pub trait RateLimiter {
    fn run<T: Send + Sync>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> impl Future<Output = Vec<Result<T>>> + Send + Sync;
}

pub struct SimpleRateLimiter {
    semaphore: Semaphore,
}

impl Default for SimpleRateLimiter {
    fn default() -> Self {
        SimpleRateLimiter::new(None)
    }
}

impl SimpleRateLimiter {
    pub fn new(concurrent_requests: Option<usize>) -> Self {
        match concurrent_requests {
            Some(number) => Self { semaphore: Semaphore::new(number) },
            None => Self { semaphore: Semaphore::new(Semaphore::MAX_PERMITS)},
        }
    }

    async fn run_one<T>(&self, future: impl Future<Output = T> + Send + Sync) -> Result<T> {
        let permit = self.semaphore.acquire().await?;
        let future_result = future.await;
        drop(permit);
        Ok(future_result)
    }
}

impl RateLimiter for SimpleRateLimiter {
    async fn run<T>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> Vec<Result<T>> {
        join_all(
            futures
            .into_iter()
            .map(|f| self.run_one(f))
        )
        .await
    }
}

pub struct RandomDelayRateLimiter {
    semaphore: Semaphore,
    min_delay_ms: usize,
    max_delay_ms: usize,
}

impl RandomDelayRateLimiter {
    pub fn new(concurrent_requests: Option<usize>, min_delay_ms: usize, max_delay_ms: usize) -> Self {
        let semaphore = match concurrent_requests {
            Some(number) => Semaphore::new(number),
            None => Semaphore::new(Semaphore::MAX_PERMITS),
        };
        Self { semaphore, min_delay_ms, max_delay_ms }
    }

    async fn run_one<T>(&self, future: impl Future<Output = T> + Send + Sync) -> Result<T> {
        let delay_seconds = rand::thread_rng().gen_range(self.min_delay_ms..self.max_delay_ms + 1);
        let permit = self.semaphore.acquire().await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(delay_seconds as u64)).await;
        let future_result = future.await;
        drop(permit);
        Ok(future_result)
    }
}

impl RateLimiter for RandomDelayRateLimiter {
    async fn run<T>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> Vec<Result<T>> {
        join_all(
            futures
            .into_iter()
            .map(|f| self.run_one(f))
        )
        .await
    }
}