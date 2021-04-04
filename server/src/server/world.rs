use crate::EntityPlayer;

use rayon::prelude::*;

use parking_lot::RwLock;

pub struct EventHandler<T: Send + Sync> {
    // RwLock is there for synchronizing between thread the inner variables since it is in lazy_static so global.
    // Vec is here because an event handler is a list of event closures to call
    // Box is here because dyn Fn(&WorldManager<N>, &T) is not sized
    // Sync + Send is for conccurency
    not_sync: RwLock<Vec<Box<dyn Fn(&WorldManager, &T) + Sync + Send>>>,
    sync: RwLock<Vec<Box<dyn Fn(&WorldManager, &mut T) + Sync + Send>>>,
}

impl<T: Send + Sync> EventHandler<T> {
    pub fn new() -> Self {
        Self {
            not_sync: RwLock::new(Vec::new()),
            sync: RwLock::new(Vec::new()),
        }
    }

    pub fn register_sync(&self, func: impl Fn(&WorldManager, &mut T) + Sync + Send + 'static) {
        self.sync.write().push(box func);
    }

    pub fn register_async(&self, func: impl Fn(&WorldManager, &T) + Sync + Send + 'static) {
        self.not_sync.write().push(box func);
    }

    pub fn execute_event(&self, server: &WorldManager, t: &mut T) {
        self.sync.read().iter().for_each(|x| x(server, t));
        self.not_sync.read().par_iter().for_each(|x| x(server, t));
    }
}

pub struct WorldManager {
    pub worlds: Vec<World>,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            worlds: vec![World {
                name: "AnvilWorld".to_owned(),
                player_entities: Vec::new(),
            }],
        }
    }
}

pub struct World {
    pub name: String,
    pub player_entities: Vec<EntityPlayer>,
}
