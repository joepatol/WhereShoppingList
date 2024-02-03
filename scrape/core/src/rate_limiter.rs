use std::future::Future;
use futures::future::join_all;
use anyhow::Result;
use tokio::sync::Semaphore;

pub trait RateLimiter {
    fn run<T: Send + Sync>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> impl Future<Output = Vec<Result<T>>> + Send + Sync;
}

pub struct SemaphoreRateLimiter {
    semaphore: Semaphore,
}

impl SemaphoreRateLimiter {
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

impl RateLimiter for SemaphoreRateLimiter {
    async fn run<T>(&self, futures: Vec<impl Future<Output = T> + Send + Sync>) -> Vec<Result<T>> {
        join_all(
            futures
            .into_iter()
            .map(|f| self.run_one(f))
        )
        .await
    }
}