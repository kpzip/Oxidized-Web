use std::{fs, io::Error};

static mut RESOURCES: Vec<CachedResource> = vec![];

pub fn get_resource_data(name: &str) -> Result<String, Error> {
    unsafe { if RESOURCES.iter().any(|s| s.name == name) {

    } }

    fs::read_to_string(name)
}

unsafe fn get_cached_resource


struct CachedResource<'a> {
    data: String,
    name: &'a str,
}
