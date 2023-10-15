use std::fs;
use std::io::Error;
use std::mem::drop;
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

static RESOURCES: RwLock<Vec<CachedResource>> = RwLock::new(Vec::new());

pub fn get_resource_data(name: String) -> Result<String, ServerError> {
    //Try to aquire read permissions
    let reader = match RESOURCES.read() {
        Ok(r) => r,
        Err(e) => {
            return Err(ServerError::PoisonedResourceError(
                PoisonedError::PoisonedRead(e),
            ))
        }
    };
    for resource in reader.iter() {
        if *resource.name == *name {
            return Ok(resource.data.clone());
        }
    }
    drop(reader);
    let filedata: String = match fs::read_to_string(&name) {
        Ok(s) => s,
        Err(e) => return Err(ServerError::FsError(e)),
    };
    let mut writer = match RESOURCES.write() {
        Ok(w) => w,
        Err(e) => {
            return Err(ServerError::PoisonedResourceError(
                PoisonedError::PoisonedWrite(e),
            ))
        }
    };
    writer.push(CachedResource::new(name, filedata.clone()));
    drop(writer);
    Ok(filedata)
}

//Need these two enums to do proper error handling, unfortunately it gets a bit messy
#[derive(Debug)]
pub enum ServerError {
    FsError(Error),
    PoisonedResourceError(PoisonedError),
}

#[derive(Debug)]
pub enum PoisonedError {
    PoisonedRead(PoisonError<RwLockReadGuard<'static, Vec<CachedResource>>>),
    PoisonedWrite(PoisonError<RwLockWriteGuard<'static, Vec<CachedResource>>>),
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
