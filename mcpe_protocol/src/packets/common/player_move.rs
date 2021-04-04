use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{Le, UnsignedVarLong, Vec3f};

#[packet(0x13)]
#[derive(Debug)]
pub struct PlayerMovePacket {
    pub entity_runtime_id: UnsignedVarLong,
    pub position: Vec3f,
    pub pitch: Le<f32>,
    pub yaw: Le<f32>,
    pub head_yaw: Le<f32>,
    pub mode: PlayerMoveMode,
    pub on_ground: bool,
    pub riding_eid: UnsignedVarLong,
    pub teleport_cause: Option<TeleportationCause>,
}

impl PlayerMovePacket {
    pub fn new(mode: PlayerMoveMode, position: Vec3f, pitch: f32, yaw: f32, player: u64) -> Self {
        Self {
            entity_runtime_id: UnsignedVarLong(player),
            position,
            pitch: Le(pitch),
            yaw: Le(yaw),
            head_yaw: Le(yaw),
            teleport_cause: if mode == PlayerMoveMode::Teleport {
                Some(TeleportationCause {
                    cause: Le(0),
                    entity_type: Le(0),
                })
            } else {
                None
            },
            mode,
            on_ground: false,
            riding_eid: UnsignedVarLong(0),
        }
    }
}

impl crate::traits::MCPEPacketData for PlayerMovePacket {
    fn decode(
        reader: &mut impl crate::traits::Reader,
    ) -> Result<Self, crate::prelude::MCPEPacketDataError> {
        use crate::traits::PacketReader;
        let mut has_cause = false;
        Ok(Self {
            entity_runtime_id: reader
                .auto_decode()
                .map_err(|x| x.map("entity_runtime_id"))?,
            position: reader.auto_decode().map_err(|x| x.map("position"))?,
            pitch: reader.auto_decode().map_err(|x| x.map("pitch"))?,
            yaw: reader.auto_decode().map_err(|x| x.map("yaw"))?,
            head_yaw: reader.auto_decode().map_err(|x| x.map("head_yaw"))?,
            mode: {
                let mode: PlayerMoveMode = reader.auto_decode().map_err(|x| x.map("mode"))?;
                if mode == PlayerMoveMode::Teleport {
                    has_cause = true;
                }
                mode
            },
            on_ground: reader.auto_decode().map_err(|x| x.map("on_ground"))?,
            riding_eid: reader.auto_decode().map_err(|x| x.map("riding_eid"))?,
            teleport_cause: if has_cause {
                Some(reader.auto_decode().map_err(|x| x.map("teleport_cause"))?)
            } else {
                None
            },
        })
    }
    fn encode(
        &self,
        writer: &mut impl crate::traits::Writer,
    ) -> Result<(), crate::prelude::MCPEPacketDataError> {
        self.entity_runtime_id
            .encode(writer)
            .map_err(|x| x.map("entity_runtime_id"))?;
        self.position
            .encode(writer)
            .map_err(|x| x.map("position"))?;
        self.pitch.encode(writer).map_err(|x| x.map("pitch"))?;
        self.yaw.encode(writer).map_err(|x| x.map("yaw"))?;
        self.head_yaw
            .encode(writer)
            .map_err(|x| x.map("head_yaw"))?;
        self.mode.encode(writer).map_err(|x| x.map("mode"))?;
        self.on_ground
            .encode(writer)
            .map_err(|x| x.map("on_ground"))?;
        self.riding_eid
            .encode(writer)
            .map_err(|x| x.map("riding_eid"))?;
        self.teleport_cause
            .encode(writer)
            .map_err(|x| x.map("teleport_cause"))?;
        Ok(())
    }
}

#[derive(Debug, MCPEPacketDataAuto)]
pub struct TeleportationCause {
    cause: Le<i32>,
    entity_type: Le<i32>,
}

#[repr(u8)]
#[packet_derive::mcpe_packet_data_enum(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerMoveMode {
    Normal = 0,
    Reset = 1,
    Teleport = 2,
    Rotation = 3,
}

// impl MCPEPacketData for PlayerMoveMode {
//     fn decode(
//         reader: &mut impl crate::traits::Reader,
//     ) -> Result<Self, crate::prelude::MCPEPacketDataError> {
//         Ok(match reader.next()? {
//             0 => Self::Normal,
//             1 => Self::Reset,
//             2 => Self::Teleport,
//             3 => Self::Rotation,
//             e => {
//                 return Err(MCPEPacketDataError::new(
//                     "player_move_mode",
//                     format!("Invalid identifier expected [0; 3] found {}", e),
//                 ))
//             }
//         })
//     }

//     fn encode(
//         &self,
//         writer: &mut impl crate::traits::Writer,
//     ) -> Result<(), crate::prelude::MCPEPacketDataError> {
//         writer.write(self.clone() as u8)
//     }
// }
