// TypeStates
pub struct HasNot;
pub struct Has<T>(T);
pub type HasUInt = Has<usize>;

pub struct ConfigBuilder<I, R> {
    max_requests: I,
    max_concurrent_requests: R,
}

impl ConfigBuilder<HasNot, HasNot> {
    pub fn new() -> Self {
        Self { max_requests: HasNot, max_concurrent_requests: HasNot }
    }
}

impl<R> ConfigBuilder<HasNot, R> {
    pub fn scrape_max_items(self, amt: usize) -> ConfigBuilder<HasUInt, R> {
        ConfigBuilder { 
            max_requests: Has { 0: amt }, 
            max_concurrent_requests: self.max_concurrent_requests,
        }
    }
}

impl<I> ConfigBuilder<I, HasNot> {
    pub fn max_concurrent_requests(self, nr: usize) -> ConfigBuilder<I, HasUInt> {
        ConfigBuilder { 
            max_requests: self.max_requests, 
            max_concurrent_requests: Has { 0: nr }
        }
    }
}

impl ConfigBuilder<HasNot, HasNot> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(None, None)
    }
}

impl ConfigBuilder<HasUInt, HasNot> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(Some(self.max_requests.0), None)
    }
}

impl ConfigBuilder<HasNot, HasUInt> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(None, Some(self.max_concurrent_requests.0))
    }
}

impl ConfigBuilder<HasUInt, HasUInt> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(Some(self.max_requests.0), Some(self.max_concurrent_requests.0))
    }
}

#[derive(Debug)]
pub struct ScrapeConfig {
    pub max_requests: Option<usize>,
    pub max_concurrent_requests: Option<usize>,
}

impl ScrapeConfig {
    pub fn new(max_requests: Option<usize>, max_concurrent_requests: Option<usize>) -> Self {
        ScrapeConfig { max_requests, max_concurrent_requests }
    }
}