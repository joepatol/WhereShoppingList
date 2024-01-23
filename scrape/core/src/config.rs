// TypeStates
pub struct HasNot;
pub struct Has<T>(T);
pub type HasUInt = Has<u32>;

pub struct ConfigBuilder<I> {
    max_items: I,
}

impl ConfigBuilder<HasNot> {
    pub fn new() -> ConfigBuilder<HasNot> {
        ConfigBuilder { max_items: HasNot }
    }

    pub fn scrape_max_items(self, amt: u32) -> ConfigBuilder<HasUInt> {
        ConfigBuilder { max_items: Has { 0: amt } }
    }

    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(None)
    }
}

impl ConfigBuilder<HasUInt> {
    pub fn build(self) -> ScrapeConfig {
        ScrapeConfig::new(Some(self.max_items.0))
    } 
}

#[derive(Debug)]
pub struct ScrapeConfig {
    pub max_items: Option<u32>,
}

impl ScrapeConfig {
    pub fn new(max_items: Option<u32>) -> Self {
        ScrapeConfig { max_items }
    }
}
