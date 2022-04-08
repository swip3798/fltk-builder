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
    IdNotFound,
    WrongType,
}

impl<T> From<PoisonError<T>> for IdMapError {
    fn from(_: PoisonError<T>) -> Self {
        IdMapError::PoisonErrorLock
    }
}

pub fn get_widget_by_id<F>(id: &str) -> Result<F, IdMapError>
where
    F: Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let map = id_map.read()?;
    let cast = map.get(id).as_ref().unwrap().downcast_ref::<F>().unwrap().clone();
    Ok(cast)
}

pub fn set_widget_to_id<F>(id: &'static str, value: F) -> Result<(), IdMapError>
where
    F: Any + Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let mut map = id_map.write()?;
    map.insert(id, Box::new(value));
    Ok(())
}