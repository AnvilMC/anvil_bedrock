use specs::{world::EntitiesRes, Component, Entity, FlaggedStorage, ReadStorage, VecStorage};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Uid(pub u64);

impl Into<u64> for Uid {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<u64> for Uid {
    fn from(uid: u64) -> Self {
        Self(uid)
    }
}

impl Component for Uid {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

#[derive(Debug)]
pub struct UidAllocator {
    index: u64,
    mapping: HashMap<u64, Entity>,
}

impl UidAllocator {
    pub fn new() -> Self {
        Self {
            index: 0,
            mapping: HashMap::new(),
        }
    }

    // Useful for when a single entity is deleted because it doesn't reconstruct the
    // entire hashmap
    pub fn remove_entity(&mut self, id: u64) -> Option<Entity> {
        self.mapping.remove(&id)
    }
}

impl Default for UidAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkerAllocator<Uid> for UidAllocator {
    fn allocate(&mut self, entity: Entity, id: Option<u64>) -> Uid {
        let id = id.unwrap_or_else(|| {
            let id = self.index;
            self.index += 1;
            id
        });
        self.mapping.insert(id, entity);
        Uid(id)
    }

    fn retrieve_entity_internal(&self, id: u64) -> Option<Entity> {
        self.mapping.get(&id).copied()
    }

    fn maintain(&mut self, entities: &EntitiesRes, storage: &ReadStorage<Uid>) {
        self.mapping = (entities, storage)
            .join()
            .map(|(e, m)| (m.id(), e))
            .collect();
    }
}
