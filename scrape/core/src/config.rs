// TypeStates
pub struct HasNot;
pub struct Has<T>(T);
pub type HasUInt = Has<usize>;

pub struct ConfigBuilder<R> {
    max_concurrent_requests: R,
}

impl ConfigBuilder<HasNot> {
    pub fn new() -> Self {
        Self { max_concurrent_requests: HasNot }
    }
}

impl ConfigBuilder<HasNot> {
    pub fn max_concurrent_requests(self, nr: usize) -> ConfigBuilder<HasUInt> {
        ConfigBuilder {
            max_concurrent_requests: Has { 0: nr }
        }
    }

    pub fn buid(self) -> ScrapeConfig {
        ScrapeConfig::new(None)
    }
}

impl ConfigBuilder<HasUInt> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(Some(self.max_concurrent_requests.0))
    }
}

#[derive(Debug)]
pub struct ScrapeConfig {
    pub max_concurrent_requests: Option<usize>,
}

impl ScrapeConfig {
    pub fn new(max_concurrent_requests: Option<usize>) -> Self {
        ScrapeConfig { max_concurrent_requests }
    }
}