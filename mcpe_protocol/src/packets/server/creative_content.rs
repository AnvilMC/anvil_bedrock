use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{Le, UnsignedVarInt, VarInt, VecIndexed};

#[packet(0x91)]
#[derive(MCPEPacketDataAuto)]
pub struct CreativeContentPacket {
    inventory: VecIndexed<Slot, UnsignedVarInt>,
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
