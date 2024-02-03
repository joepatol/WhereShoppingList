use std::iter::FromIterator;
use std::future::Future;
use super::RateLimiter;
use anyhow::Result;

pub trait Transform<T: Send + Sync, I: Send + Sync> {
    fn transform(self, func: impl Fn(T) -> Result<I, anyhow::Error>) -> ResultCollector<I>;
}

pub trait AsyncTransform<T: Send + Sync, I: Send + Sync> {
    fn transform_async<F, R>(self, func: impl Fn(T) -> F + Send + Sync, rate_limiter: &R) -> impl Future<Output = ResultCollector<I>> + Send + Sync
    where
        F: Future<Output = Result<I, anyhow::Error>> + Send + Sync,
        R: RateLimiter + Send + Sync,
    ;
}

#[derive(Debug)]
pub struct ResultCollector<T: Send + Sync> {
    pub successes: Vec<T>,
    pub errors: Vec<anyhow::Error>,
}

impl<T: Send + Sync> ResultCollector<T> {
    pub fn new() -> Self {
        ResultCollector {
            successes: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn split_into_iter(self) -> (impl Iterator<Item = T>, impl Iterator<Item = anyhow::Error>) {
        (self.successes.into_iter(), self.errors.into_iter())
    }

    pub fn collect(&mut self, result: Result<T, anyhow::Error>) {
        match result {
            Ok(success) => self.successes.push(success),
            Err(error) => self.errors.push(error),
        }
    }

    pub fn extend(&mut self, other: ResultCollector<T>) {
        self.successes.extend(other.successes);
        self.errors.extend(other.errors);
    }

    pub fn extend_with_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Result<T, anyhow::Error>>,
    {
        for result in iter {
            self.collect(result);
        }
    }

    pub fn iter_ok(&self) -> impl Iterator<Item = &T> {
        self.successes.iter()
    }

    pub fn map_extract<I, E>(self, ok_func: impl Fn(T) -> I, err_func: impl Fn(anyhow::Error) -> E) -> (Vec<I>, Vec<E>) {
        let (ok_iter, err_iter) = self.split_into_iter();
        (
            ok_iter.map(ok_func).collect(),
            err_iter.map(err_func).collect(),
        )
    }
}

impl<T: Send + Sync + Clone> ResultCollector<T> {
    pub fn explode<II, E>(self, iter: &E) -> ResultCollector<(T, II)> 
    where
        E: Iterator<Item = II> + Send + Sync + Clone,
        II: Send + Sync,
    {
        self.transform(| success_element | 
            iter.clone().map(|iter_element| 
                Ok((success_element.clone(), iter_element))
            )
            .collect::<Result<Vec<(T, II)>>>())
            .flatten()        
    }
}

impl<T: Send + Sync> ResultCollector<Vec<T>> {
    pub fn flatten(self) -> ResultCollector<T> {
        let mut collector = ResultCollector::new();
        collector.successes = self.successes.into_iter().flatten().collect();
        collector.errors = self.errors;
        collector
    }
}

impl<T: Send + Sync> FromIterator<Result<T, anyhow::Error>> for ResultCollector<T> {
    fn from_iter<I: IntoIterator<Item = Result<T, anyhow::Error>>>(iter: I) -> Self {
        let mut collector = ResultCollector::new();
        collector.extend_with_iter(iter);
        collector
    }
}

impl<T: Send + Sync> FromIterator<Result<Vec<T>, anyhow::Error>> for ResultCollector<T> {
    fn from_iter<I: IntoIterator<Item = Result<Vec<T>, anyhow::Error>>>(iter: I) -> Self {
        let mut collector = ResultCollector::new();
        for result in iter {
            match result {
                Ok(v) => collector.successes.extend(v),
                Err(e) => collector.errors.push(e)
            }
        }
        collector
    }
}

impl<T: Send + Sync> From<anyhow::Error> for ResultCollector<T> {
    fn from(error: anyhow::Error) -> Self {
        let mut collector = ResultCollector::new();
        collector.errors.push(error);
        collector
    }
}

impl<T: Send + Sync> From<Vec<T>> for ResultCollector<T> {
    fn from(value: Vec<T>) -> Self {
        let mut collector = ResultCollector::new();
        collector.successes.extend(value);
        collector
    }
}

impl<T: Send + Sync, I: Send + Sync> Transform<T, I> for ResultCollector<T> {
    fn transform(self, func: impl Fn(T) -> Result<I, anyhow::Error>) -> ResultCollector<I> {
        let mut results: ResultCollector<I> = self.successes.into_iter().map(|inp| (func)(inp)).collect();
        results.errors.extend(self.errors);
        results
    }
}

impl<T: Send + Sync, I: Send + Sync> AsyncTransform<T, I> for ResultCollector<T> {
    async fn transform_async<F, R>(self, func: impl Fn(T) -> F, rate_limiter: &R) -> ResultCollector<I>
    where
        F: Future<Output = Result<I, anyhow::Error>> + Send + Sync,
        R: RateLimiter + Send + Sync, 
    {
        let mut results: ResultCollector<I> = rate_limiter
            .run(
            self.successes
                .into_iter()
                .map(|inp| (func)(inp) )
                .collect()
            )
            .await
            .into_iter()
            .flatten()
            .collect();

        results.errors.extend(self.errors);
        results
    }
}