struct PotentiallyCachedResource<'a> {
    cache: CacheType<'a>,
}

enum CacheType<'a> {
    Cached(&'a Resource),
    Uncached(&'a dyn FnOnce() -> &'a Resource),
}

impl<'a> PotentiallyCachedResource<'a> {
    fn get(&mut self) -> &'a Resource {
        match self.cache {
            CacheType::Cached(res) => res,
            CacheType::Uncached(supp) => {
                let res: &Resource = supp();
                self.cache = CacheType::Cached(res);
                res
            }
        }
    }
}

enum ResourceType {
    HTML,
    CSS,
    JS,
    Image,
}

struct Resource {
    data: String,
    resourcetype: ResourceType,
}
