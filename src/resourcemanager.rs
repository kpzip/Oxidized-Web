use std::error::Error;
use std::fs;
use std::mem::drop;
use std::sync::RwLock;

static RESOURCES: RwLock<Vec<CachedResource>> = RwLock::new(Vec::new());

pub fn get_resource_data(name: String) -> Result<String, Box<dyn Error>> {
    //Try to aquire read permissions
    let reader = match RESOURCES.read() {
        Ok(r) => r,
        Err(e) => return Err(Box::new(e)),
    };
    for resource in reader.iter() {
        if *resource.name == *name {
            return Ok(resource.data.clone());
        }
    }
    drop(reader);
    let filedata: String = match fs::read_to_string(&name) {
        Ok(s) => s,
        Err(e) => return Err(Box::new(e)),
    };
    let mut writer = match RESOURCES.write() {
        Ok(w) => w,
        Err(e) => return Err(Box::new(e)),
    };
    writer.push(CachedResource::new(name, filedata.clone()));
    drop(writer);
    Ok(filedata)
}

//set to pub in order to remove warnings
pub struct CachedResource {
    data: String,
    name: String,
}

impl CachedResource {
    fn new(name: String, data: String) -> CachedResource {
        CachedResource { data, name }
    }
}
