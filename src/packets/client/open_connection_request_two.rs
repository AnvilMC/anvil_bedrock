use packet_derive::{packet, Biscuit};

use crate::packets::objects::{address::Address, client_guid::ClientGUID, magic::Magic};

#[packet(0x07)]
#[derive(Debug, Biscuit)]
pub struct OpenConnectionRequestTwo {
    pub magic: Magic,
    pub server_address: Address,
    pub mtu: i16,
    pub id: ClientGUID,
}
