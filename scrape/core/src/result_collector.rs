use std::iter::FromIterator;

#[derive(Debug)]
pub struct ResultCollector<T> {
    pub successes: Vec<T>,
    pub errors: Vec<anyhow::Error>,
}

impl<T> ResultCollector<T> {
    pub fn new() -> Self {
        ResultCollector {
            successes: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn split_into_iter(self) -> (impl Iterator<Item = T>, impl Iterator<Item = anyhow::Error>) {
        (self.successes.into_iter(), self.errors.into_iter())
    }

    pub fn inherit_errs<I>(&mut self, other: ResultCollector<I>) {
        self.errors.extend(other.errors);
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
}

impl<T> FromIterator<Result<T, anyhow::Error>> for ResultCollector<T> {
    fn from_iter<I: IntoIterator<Item = Result<T, anyhow::Error>>>(iter: I) -> Self {
        let mut collector = ResultCollector::new();
        collector.extend_with_iter(iter);
        collector
    }
}

impl<T> FromIterator<Result<Vec<T>, anyhow::Error>> for ResultCollector<T> {
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

impl<T> From<anyhow::Error> for ResultCollector<T> {
    fn from(error: anyhow::Error) -> Self {
        let mut collector = ResultCollector::new();
        collector.errors.push(error);
        collector
    }
}