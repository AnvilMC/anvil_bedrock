pub struct World {
    pub name: String,
    pub player_entities: Vec<EntityPlayer>,
}

pub struct EntityPlayer {
    pub uuid: String,
    pub username: String,
    pub position: (i32, i32, i32),
}
