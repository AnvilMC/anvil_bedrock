use std::{net::SocketAddr, sync::Arc};

use crate::{EcsWorld, GamePacketSendablePacket, PacketRec, PlayerInfo, ReceivablePacket};

use std::collections::HashMap;

use crossbeam::channel::{Receiver, SendError, Sender};
use rayon::prelude::*;

use parking_lot::RwLock;

pub struct EventHandler<T: Send + Sync> {
    // RwLock is there for synchronizing between thread the inner variables since it is in lazy_static so global.
    // Vec is here because an event handler is a list of event closures to call
    // Box is here because dyn Fn(&WorldManager<N>, &T) is not sized
    // Sync + Send is for conccurency
    not_sync: RwLock<Vec<Box<dyn Fn(&EcsWorld, &T) + Sync + Send>>>,
    sync: RwLock<Vec<Box<dyn Fn(&mut EcsWorld, &mut T) + Sync + Send>>>,
}

impl<T: Send + Sync> EventHandler<T> {
    pub fn new() -> Self {
        Self {
            not_sync: RwLock::new(Vec::new()),
            sync: RwLock::new(Vec::new()),
        }
    }

    pub fn register_sync(&self, func: impl Fn(&mut EcsWorld, &mut T) + Sync + Send + 'static) {
        self.sync.write().push(box func);
    }

    pub fn register_async(&self, func: impl Fn(&EcsWorld, &T) + Sync + Send + 'static) {
        self.not_sync.write().push(box func);
    }

    pub fn execute_event(&self, server: &mut EcsWorld, t: &mut T) {
        self.sync.read().iter().for_each(|x| x(&mut *server, t));
        self.not_sync.read().par_iter().for_each(|x| x(&*server, t));
    }
}

pub struct WorldManager {
    pub worlds: Vec<EcsWorld>,
    pub packet_handlers: Vec<Arc<Sender<PacketRec>>>,
    pub players_loc: HashMap<SocketAddr, Arc<Sender<PacketRec>>>,
    pub default_handler: Arc<Sender<PacketRec>>,
}

impl WorldManager {
    pub fn new() -> Self {
        let (s, r) = crossbeam::channel::unbounded();
        let s = Arc::new(s);
        Self {
            worlds: vec![EcsWorld::new(r)],
            packet_handlers: vec![s.clone()],
            default_handler: s,
            players_loc: HashMap::new(),
        }
    }

    pub fn send(
        &mut self,
        adrr: &SocketAddr,
        packet: PacketRec,
    ) -> Result<(), SendError<PacketRec>> {
        if let Some(e) = self.players_loc.get(adrr) {
            e.send(packet)
        } else {
            self.players_loc
                .insert(adrr.clone(), self.default_handler.clone());
            self.default_handler.send(packet)
        }
    }
}
