use hashbrown::HashMap;
use parking_lot::RwLock;
use std::sync::Arc;
use std::any::{TypeId, Any};

pub trait Injectable: Any {}

#[derive(Default)]
pub struct Container {
    init: RwLock<HashMap<TypeId, Box<FnOnce() -> Box<(dyn Injectable + Send + Sync)>>>>,
    instances: RwLock<HashMap<TypeId, Arc<Box<dyn Injectable + Send + Sync>>>>
}

impl Container {
    pub fn from_factory_map(map: HashMap<TypeId, Box<FnOnce() -> Box<(dyn Injectable + Send + Sync)>>>) -> Self {
        Container {
            init: RwLock::new(map),
            instances: RwLock::new(HashMap::default())
        }
    }

    pub fn add_factory(&mut self, id: TypeId, factory: Box<FnOnce() -> Box<(dyn Injectable + Send + Sync)>>) -> &mut Self {
        self.init.write().insert(id, factory);
        self
    }

    pub fn get(&self, key: TypeId) -> Arc<Box<Injectable + Send + Sync>> {
        if let Some(instance) = self.instances.read().get(&key) {
            Arc::clone(instance)
        } else if let Some(ref init_func) = self.init.write().remove(&key) {
            let instance = Arc::new(init_func());
            let ret = Arc::clone(&instance);

            self.instances.write().insert(key, instance);
            ret
        } else {
            panic!("This should not happen ever")
        }
    }
}
