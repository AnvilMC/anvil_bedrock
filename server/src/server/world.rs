use std::{net::SocketAddr, sync::Arc};

use crossbeam::channel::Sender;
use mcpe_protocol::prelude::{Le, PlayerMoveMode, PlayerMovePacket, Vec3f};

use crate::{GamePacketSendablePacket, Server};

use rayon::prelude::*;

use lazy_static::lazy_static;
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

#[derive(Clone)]
pub struct EntityPlayer {
    pub uuid: String,
    pub username: String,
    pub position: Vec3f,
    pub yaw: f32,
    pub pitch: f32,
    pub eid: i64,
    pub sender: Arc<Sender<GamePacketSendablePacket>>,
    pub socket_adress: SocketAddr,
}

impl EntityPlayer {
    pub fn new(
        uuid: String,
        username: String,
        sender: Arc<Sender<GamePacketSendablePacket>>,
        socket_adress: SocketAddr,
    ) -> Self {
        Self {
            uuid,
            username,
            position: Vec3f(Le(0.0), Le(0.0), Le(0.0)),
            yaw: 0.0,
            pitch: 0.0,
            eid: 1,
            sender,
            socket_adress,
        }
    }

    pub fn set_pos(&mut self, position: Vec3f, yaw: f32, pitch: f32) {
        self.position = position;
        self.yaw = yaw;
        self.pitch = pitch;
        self.send_pos(PlayerMoveMode::Normal);
    }

    pub fn teleport(&mut self, position: Vec3f, yaw: f32, pitch: f32) {
        self.position = position;
        self.yaw = yaw;
        self.pitch = pitch;
        self.send_pos(PlayerMoveMode::Teleport);
    }

    pub fn send_pos(&self, mode: PlayerMoveMode) {
        self.sender
            .send(GamePacketSendablePacket::PlayerMovePacket(
                PlayerMovePacket::new(mode, self.position.clone(), self.pitch, self.yaw, self.eid),
            ))
            .unwrap();
    }
}
