use std::collections::HashMap;

use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::ItemDef;

use crate::prelude::{Le, UnsignedVarInt, VarInt, VecIndexed};

#[packet(0x91)]
#[derive(MCPEPacketDataAuto)]
pub struct CreativeContentPacket {
    pub inventory: VecIndexed<Slot, UnsignedVarInt>,
}

#[derive(serde::Deserialize)]
struct JsonItem {
    id: i32,
    damage: Option<i32>,
    nbt: Option<i32>,
}

impl Default for CreativeContentPacket {
    fn default() -> Self {
        Self {
            inventory: {
                let object: Vec<JsonItem> =
                    serde_json::from_str(include_str!("creativeitems.json")).unwrap();
                let palette: Vec<ItemDef> =
                    serde_json::from_str(include_str!("internal_ids.json")).unwrap();
                let mut legacy_network_map = palette
                    .iter()
                    .flat_map(|x| {
                        if let Some(e) = x.oldId {
                            let fullId =
                                (x.id as i32) << 16 | (x.oldData.unwrap_or(0) as i32 & 0x7FFF) << 1;
                            Some((fullId, x.id << 1 | (x.oldData.map(|_| 1).unwrap_or(0))))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>();
                object
                    .iter()
                    .filter(|x| x.nbt.is_none())
                    .enumerate()
                    .flat_map(|(i, x)| {
                        let data = x.damage.unwrap_or(-1);

                        let full_network_id = x.id << 16 | (data & 0x7FFF) << 1;
                        let full_network_id =
                            if let Some(e) = legacy_network_map.get(&full_network_id) {
                                e
                            } else {
                                if let Some(e) =
                                    legacy_network_map.get(&(x.id << 16 | (0 & 0x7FFF) << 1))
                                {
                                    e
                                } else {
                                    //println!("Unknown item mapping {}", x.id);
                                    return None;
                                }
                            };
                        let network_id = full_network_id >> 1;
                        let aux = 1;
                        let meta = if (full_network_id & 0x1) != 0 {
                            0
                        } else {
                            x.damage.unwrap_or(-1)
                        };
                        let aux = aux | ((meta & 0x7FFF) << 8);
                        Some(Slot {
                            position: UnsignedVarInt(i as u32 + 1),
                            item: Item {
                                network_id: VarInt(network_id as i32),
                                aux_value: VarInt(aux),
                                has_compound_tag: Le(0),
                                can_place_on: Vec::new().into(),
                                can_destroy: Vec::new().into(),
                            },
                        })
                    })
                    .collect::<Vec<_>>()
                    .into()
            },
        }
    }
}

#[derive(MCPEPacketDataAuto)]
pub struct Slot {
    position: UnsignedVarInt,
    item: Item,
}

#[derive(MCPEPacketDataAuto)]
pub struct Item {
    network_id: VarInt,
    aux_value: VarInt,         /*
                               if durable {
                                   count
                               } else {
                                   int meta = clearData ? 0 : (item.hasMeta() ? item.getDamage() : -1);
                                   count | ((meta & 0x7FFF) << 8)
                               }
                               */
    has_compound_tag: Le<i16>, // SET TO 0 compounds tags are only availible when item.hasCompoundTag() || isDurable
    can_place_on: VecIndexed<String, VarInt>,
    can_destroy: VecIndexed<String, VarInt>,
    /*
    if (item.getId() == 513)
      putVarLong(0L);
    */
}
