use std::iter::FromIterator;
use std::future::Future;
use super::AsyncExecutor;
use anyhow::Result;

pub trait Transform<T: Send + Sync, I: Send + Sync> {
    type Collected: Send + Sync;
    fn transform(self, func: impl Fn(T) -> I) -> ResultCollector<Self::Collected>;
}

pub trait AsyncTransform<InputT, ResolvedOutT, FutureOutT> 
where
    InputT: Send + Sync,
    ResolvedOutT: Send + Sync,
    FutureOutT: Future<Output = ResolvedOutT> + Send + Sync,
{
    type Collected: Send + Sync;
    fn transform_async<R: AsyncExecutor + Send + Sync>(self, func: impl Fn(InputT) -> FutureOutT + Send + Sync, executor: &R) -> impl Future<Output = ResultCollector<Self::Collected>> + Send + Sync;
}

/// Collect results of operations that return an `anyhow::Result<T>`.
/// `Ok` and `Err` values are collected into their own vectors,
/// allowing you to operate on the `Ok` result while keeping track
/// of all errors that occurred along the way.
/// 
/// # Example
/// ```
/// use scrape_core::{ResultCollector, Transform};
/// use anyhow::{Result, anyhow};
/// 
/// fn func(val: i32) -> Result<Vec<i32>> {
///     if val <= 0 {
///         return Err(anyhow!("{}", val));
///     };
///     Ok(vec![val, val + 1])
/// }
/// 
/// let collector = ResultCollector::from(vec![-1, 0, 1, 3]);
/// let result = collector.transform(|e| func(e)).flatten();
/// 
/// assert_eq!(result.successes, vec![1, 2, 3, 4]);
/// assert_eq!(result.list_error_messages(), vec!["-1".to_owned(), "0".to_owned()]);
/// ```
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

    pub fn from_value(value: T) -> ResultCollector<T> {
        ResultCollector {
            successes: vec![value],
            errors: Vec::new(),
        }
    }

    /// Split the ResultCollector into two iterators over 
    /// the Err and Ok variants
    pub fn split_into_iter(self) -> (impl Iterator<Item = T>, impl Iterator<Item = anyhow::Error>) {
        (self.successes.into_iter(), self.errors.into_iter())
    }

    // List the collected error messages as strings
    pub fn list_error_messages(&self) -> Vec<String> {
        self.errors.iter().map(|e| e.to_string()).collect()
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

    /// Create an Iterator over the Ok variants
    pub fn iter_ok(&self) -> impl Iterator<Item = &T> {
        self.successes.iter()
    }

    /// Extract both vectors into a tuple after applying a transformation
    pub fn map_extract<I, E>(self, ok_func: impl Fn(T) -> I, err_func: impl Fn(anyhow::Error) -> E) -> (Vec<I>, Vec<E>) {
        let (ok_iter, err_iter) = self.split_into_iter();
        (
            ok_iter.map(ok_func).collect(),
            err_iter.map(err_func).collect(),
        )
    }
}

impl<T: Send + Sync + Clone> ResultCollector<T> {
    /// Explode the `Ok` vector using an iterator. 
    /// Turns the `ResultCollector<T>` into a `ResultCollector<(T, I)>`.
    /// This will return a cartesian product of both iterators
    /// 
    /// # Example
    /// ``` 
    /// use scrape_core::ResultCollector;
    /// 
    /// let collector = ResultCollector::from(vec![1, 2]);
    /// let exploded = collector.explode(&vec!["a", "b"].into_iter());
    ///
    /// let expected = vec![
    ///    (1, "a"),
    ///    (1, "b"),
    ///    (2, "a"),
    ///    (2, "b"),
    /// ];
    /// assert_eq!(expected, exploded.successes);
    /// ```
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
    /// Flatten a ResultCollector containing vectors into a plain vector.
    /// Turning `ResultCollector<Vec<T>>` into `ResultCollector<T>`
    /// 
    /// # Example
    /// ```
    /// use scrape_core::ResultCollector;
    /// 
    /// let collector = ResultCollector::from(vec![vec![1, 2], vec![3,4]]);
    /// let flat = collector.flatten();
    ///
    /// assert_eq!(flat.successes, vec![1, 2, 3, 4]);
    /// ```
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

impl<T: Send + Sync, I: Send + Sync> Transform<T, Result<I, anyhow::Error>> for ResultCollector<T> {
    type Collected = I;

    /// Transform all success elements of this collector using a closure. Turning
    /// this `ResultCollector<T>` into `ResultCollector<I>`
    /// 
    /// # Example
    /// ```
    /// use scrape_core::{ResultCollector, Transform};
    ///  
    /// let collector = ResultCollector::from(vec![1, 2]);
    /// let transformed = collector.transform(|e| Ok(e + 1));
    ///
    /// assert_eq!(transformed.successes, vec![2, 3]);
    /// ```
    /// 
    /// Errors get collected, and the `Ok`` vector loses the failed value
    /// ```
    /// let collector = ResultCollector::from(vec!["a"]);
    /// let transformed: ResultCollector<&str> = collector.transform(|_| Err(anyhow!("fail")));
    /// 
    /// assert_eq!(transformed.list_error_messages(), vec!["fail".to_owned()]);
    /// assert_eq!(transformed.successes, Vec::<&str>::default());
    /// ```
    fn transform(self, func: impl Fn(T) -> Result<I, anyhow::Error>) -> ResultCollector<Self::Collected> {
        let mut results: ResultCollector<I> = self.successes.into_iter().map(|inp| (func)(inp)).collect();
        results.errors.extend(self.errors);
        results
    }
}

impl
<  
    F: Future<Output = Result<I, anyhow::Error>> + Send + Sync,
    I: Send + Sync,
    T: Send + Sync,
> 
AsyncTransform<T, Result<I, anyhow::Error>, F> for ResultCollector<T> 
{
    type Collected = I;

    /// Transform this `ResultCollector<T>` into `ResultCollector<I>` using a
    /// closure that returns a future. The future should resolve to a `Result<I>`
    /// 
    /// AsyncExecutor is used to expose control over the execution of the futures. E.g. to limit the
    /// number of concurrent requests.
    async fn transform_async<R: AsyncExecutor + Send + Sync>(self, func: impl Fn(T) -> F, executor: &R) -> ResultCollector<Self::Collected> {
        let mut results: ResultCollector<I> = executor
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

impl
<  
    F: Future<Output = ResultCollector<I>> + Send + Sync,
    I: Send + Sync,
    T: Send + Sync,
> 
AsyncTransform<T, ResultCollector<I>, F> for ResultCollector<T> {
    type Collected = I;

    /// Transform this `ResultCollector<T>` into `ResultCollector<I>` using a
    /// closure that returns a future. The future should resolve to a `ResultCollector<I>`
    /// 
    /// AsyncExecutor is used to expose control over the execution of the futures. E.g. to limit the
    /// number of concurrent requests.
    async fn transform_async<R: AsyncExecutor + Send + Sync>(self, func: impl Fn(T) -> F + Send + Sync, executor: &R) -> ResultCollector<Self::Collected> {
        let mut new_collector = ResultCollector::new();
        let results = executor
            .run(
            self.successes
                .into_iter()
                .map(|inp| (func)(inp) )
                .collect()
            )
            .await;

        for result in results.into_iter() {
            match result {
                Ok(coll) => new_collector.extend(coll),
                Err(e) => new_collector.errors.push(e),
            };
        }
        new_collector
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use anyhow::{anyhow, Result};
    use crate::{AsyncTransform, SimpleRateLimiter, Transform};
    use super::ResultCollector;

    fn test_func(val: i32) -> Result<Vec<i32>> {
        if val <= 0 {
            return Err(anyhow!("{}", val));
        };
        Ok(vec![val, val + 1])
    }

    async fn test_async_returns_result(val: i32) -> Result<Vec<i32>> {
        if val <= 0 {
            return Err(anyhow!("{}", val));
        };
        Ok(vec![val, val + 1])
    }

    async fn test_async_return_result_collector(val: i32) -> ResultCollector<i32> {
        ResultCollector::from(vec![val + 1])
    }

    #[tokio::test]
    async fn test_transform_async_result_collector() {
        let rate_limiter = SimpleRateLimiter::default();
        let collector = ResultCollector::from(vec![1, 2, 3]);
        let result = collector.transform_async(|v| test_async_return_result_collector(v), &rate_limiter).await;

        assert_eq!(result.successes, vec![2, 3, 4]);
    }

    #[test]
    fn test_from_value() {
        let collector = ResultCollector::from_value(1);
        assert_eq!(collector.successes, vec![1]);
    }

    #[tokio::test]
    async fn test_transform_async() {
        let rate_limiter = SimpleRateLimiter::new(None);
        let collector = ResultCollector::from(vec![-1, 0, 1, 3]);
        let result = collector.transform_async(|e| test_async_returns_result(e), &rate_limiter).await.flatten();
        assert_eq!(result.successes, vec![1, 2, 3, 4]);
        assert_eq!(result.list_error_messages(), vec!["-1".to_owned(), "0".to_owned()]);
    }

    #[test]
    fn test_combination() {
        let collector = ResultCollector::from(vec![-1, 0, 1, 3]);
        let result = collector.transform(|e| test_func(e)).flatten();
        assert_eq!(result.successes, vec![1, 2, 3, 4]);
        assert_eq!(result.list_error_messages(), vec!["-1".to_owned(), "0".to_owned()]);
    }

    #[test]
    fn test_transform() {
        let collector = ResultCollector::from(vec![1, 2]);
        let transformed = collector.transform(|e| Ok(e + 1));

        assert_eq!(transformed.successes, vec![2, 3]);
    }

    #[test]
    fn test_transform_error_variant() {
        let collector = ResultCollector::from(vec!["a"]);
        let transformed: ResultCollector<&str> = collector.transform(|_| Err(anyhow!("fail")));

        assert_eq!(transformed.list_error_messages(), vec!["fail".to_owned()]);
        assert_eq!(transformed.successes, Vec::<&str>::default());
    }

    #[test]
    fn test_flatten() {
        let collector = ResultCollector::from(vec![vec![1, 2], vec![3,4]]);
        let flat = collector.flatten();

        assert_eq!(flat.successes, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_explode() {
        let collector = ResultCollector::from(vec![1, 2]);
        let exploded = collector.explode(&vec!["a", "b"].into_iter());

        let expected = vec![
            (1, "a"),
            (1, "b"),
            (2, "a"),
            (2, "b"),
        ];

        assert_eq!(expected, exploded.successes);
    }

    #[test]
    fn test_new() {
        let collector: ResultCollector<usize> = ResultCollector::new();

        assert_eq!(collector.successes.len(), 0);
        assert_eq!(collector.errors.len(), 0);
    }

    #[test]
    fn test_collect() {
        let mut collector = ResultCollector::new();
        collector.collect(Ok("a"));

        assert_eq!(collector.successes, vec!["a"]);
    }

    #[test]
    fn test_extend() {
        let mut collector = ResultCollector::from(vec![1, 2]);
        collector.collect(Err(anyhow!("oops")));
        let mut other = ResultCollector::from(vec![3, 4]);
        other.collect(Err(anyhow!("oops2")));

        collector.extend(other);

        assert_eq!(collector.successes, vec![1, 2, 3, 4]);
        assert_eq!(collector.list_error_messages(), vec!["oops".to_owned(), "oops2".to_owned()]);
    }
}