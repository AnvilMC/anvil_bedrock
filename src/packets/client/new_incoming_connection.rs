use packet_derive::packet;

use crate::packets::objects::{address::Address, time::Time};

#[packet(0x05)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct NewIncomingConnection {
    // pub server_address: [u8;6],
    // //pub internal_address: Address,
    // pub system_address: [[u8;6]; 20],
    // pub req_time: Time,
    // pub acc_time: Time,
    pub padding: u8
}
