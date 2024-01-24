use std::future::Future;
use futures::future::join_all;
use anyhow::Result;

pub struct RateLimiter {
    concurrent_requests: Option<usize>,
}

impl RateLimiter {
    pub fn new(concurrent_requests: Option<usize>) -> Self {
        Self { concurrent_requests }
    }
    
    pub async fn run<T>(&self, futures: Vec<impl Future<Output = Result<Vec<T>>> + Send>) -> Result<Vec<T>> {
        match self.concurrent_requests {
            Some(limit) => Ok(RateLimiter::run_with_limit(futures, limit).await?),
            None => Ok(RateLimiter::run_batch(futures).await?)
        }
    }

    async fn run_with_limit<T>(futures: Vec<impl Future<Output = Result<Vec<T>>> + Send>, limit: usize) -> Result<Vec<T>> {
        // TODO: make this method nicer, problem with below code is that it's not Send and/or Sync
        // NOTE: it requires itertools crate itertools = "0.12.0"
        // for batch in futures.into_iter().chunks(limit).into_iter() {
        //     result.extend(RateLimiter::run_batch(batch.collect::<Vec<_>>()).await?);
        // }

        let mut result = Vec::new();
        let mut batch = Vec::new();
        let mut cur_limit = limit;

        for (nr_loaded, future) in futures.into_iter().enumerate() {
            batch.push(future);

            if nr_loaded == cur_limit {
                result.extend(RateLimiter::run_batch(batch).await?);
                batch = Vec::new();
                cur_limit += limit
            }
        }
        result.extend(RateLimiter::run_batch(batch).await?);

        Ok(result)
    }

    async fn run_batch<T>(futures: Vec<impl Future<Output = Result<Vec<T>>> + Send>) -> Result<Vec<T>> {
        Ok(join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<T>>>>()?
            .into_iter()
            .flatten()
            .collect()
        )
    }
}