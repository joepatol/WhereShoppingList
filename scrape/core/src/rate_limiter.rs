use std::future::Future;
use futures::future::join_all;
use anyhow::Result;

pub struct RateLimiter {
    concurrent_requests: usize,
}

impl RateLimiter {
    pub fn new(concurrent_requests: usize) -> Self {
        Self { concurrent_requests }
    }
    
    pub async fn run<T>(&self, futures: Vec<impl Future<Output = Result<Vec<T>>>>) -> Result<Vec<T>> {
        let mut cur_limit: usize = self.concurrent_requests;
        let mut result = Vec::new();
        let mut batch = Vec::new();

        for (nr_loaded, future) in futures.into_iter().enumerate() {
            batch.push(future);

            if nr_loaded == cur_limit {
                result.extend(RateLimiter::scrape_batch(batch).await?);
                batch = Vec::new();
                cur_limit += self.concurrent_requests;
            }
        };
        result.extend(RateLimiter::scrape_batch(batch).await?);
        Ok(result)
    }

    async fn scrape_batch<T>(futures: Vec<impl Future<Output = Result<Vec<T>>>>) -> Result<Vec<T>> {
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