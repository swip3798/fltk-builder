use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, PoisonError, RwLock},
};

lazy_static! {
    // WTF is this type?!
    pub static ref ID_MAP: Arc<RwLock<HashMap<&'static str, Box<dyn Any + Send + Sync>>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

#[derive(Debug)]
pub enum IdMapError {
    PoisonErrorLock,
    KeyNotFound,
    WrongType,
}

impl<T> From<PoisonError<T>> for IdMapError {
    fn from(_: PoisonError<T>) -> Self {
        IdMapError::PoisonErrorLock
    }
}

pub fn get_widget_by_id<F>(key: &str) -> Result<F, IdMapError>
where
    F: Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let map = id_map.read()?;
    let val = map.get(key).ok_or(IdMapError::KeyNotFound)?;
    let cast = val
        .downcast_ref::<F>()
        .ok_or(IdMapError::WrongType)?
        .clone();
    Ok(cast)
}

pub fn set_widget_to_id<F>(key: &'static str, value: F) -> Result<(), IdMapError>
where
    F: Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let mut map = id_map.write()?;
    let value = Box::new(value);
    map.insert(key, value);
    Ok(())
}