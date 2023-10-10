use std::mem::drop;
use std::sync::RwLock;
use std::{fs, io::Error};

static RESOURCES: RwLock<Vec<CachedResource>> = RwLock::new(Vec::new());

pub fn get_resource_data(name: &str) -> &'static String {
    let reader = RESOURCES.read().expect("Poisoned Resource Manager!").iter();
    for resource in reader {
        if *resource.name == *name {
            return &resource.data;
        }
    }
    let filedata: String = match fs::read_to_string(name) {
        Ok(s) => s,
        Err(_) => return &String::new(),
    };
    let writer = RESOURCES.write().expect("Poisoned Resource Manager!");
    writer.push(CachedResource::new(name, filedata));
    drop(writer);
    return &filedata;
}

struct CachedResource<'a> {
    data: String,
    name: &'a str,
}

impl CachedResource<'_> {
    fn new(name: &str, data: String) -> CachedResource {
        CachedResource { data, name }
    }
}
