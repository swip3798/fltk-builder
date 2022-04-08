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
    for (k,v) in map.iter() {
        println!("K: {}, vt: {:?}, b: {:?}", k, (&*v).type_id(), std::any::TypeId::of::<Box<dyn Any>>());
    }
    let val = map.get(id).ok_or(IdMapError::IdNotFound)?;
    let cast = val
        .downcast_ref::<F>()
        .ok_or(IdMapError::WrongType)?
        .clone();
    Ok(cast)
}

pub fn set_widget_to_id<F>(id: &'static str, value: F) -> Result<(), IdMapError>
where
    F: Any + Clone + Send + Sync + 'static,
{
    let id_map = ID_MAP.clone();
    let mut map = id_map.write()?;
    println!("{:?}", std::any::TypeId::of::<F>());
    let value: Box<dyn Any + Send + Sync> = Box::new(value);
    map.insert(id, value);
    Ok(())
}