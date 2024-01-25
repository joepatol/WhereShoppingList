use std::future::Future;
use futures::future::join_all;
use anyhow::Result;
use tokio::sync::Semaphore;

pub struct RateLimiter {
    semaphore: Semaphore,
}

impl RateLimiter {
    pub fn new(concurrent_requests: Option<usize>) -> Self {
        match concurrent_requests {
            Some(number) => Self { semaphore: Semaphore::new(number) },
            None => Self { semaphore: Semaphore::new(Semaphore::MAX_PERMITS)},
        }
    }
    
    pub async fn run<T>(&self, futures: Vec<impl Future<Output = Result<Vec<T>>> + Send>) -> Result<Vec<T>> {
        Ok(
            join_all(
                futures
                .into_iter()
                .map(|f| self.run_one(f))
            )
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<T>>>>()?
            .into_iter()
            .flatten()
            .collect()
        )
    }

    async fn run_one<T>(&self, future: impl Future<Output = Result<Vec<T>>> + Send) -> Result<Vec<T>> {
        let permit = self.semaphore.acquire().await?;
        let future_result = future.await?;
        drop(permit);
        Ok(future_result)
    }
}