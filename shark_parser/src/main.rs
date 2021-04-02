#![feature(exclusive_range_pattern)]

use etherparse::PacketHeaders;
use mcpe_protocol::prelude::*;
use raknet::prelude::*;

fn from_hex(i: u8) -> u8 {
    match i {
        b'0'..=b'9' => i - b'0',
        b'A'..=b'F' => i - b'A' + 10,
        b'a'..=b'f' => i - b'a' + 10,
        e => panic!("WINDOZE FATALE ERREAURE {}", e),
    }
}

fn main() {
    let shark = std::fs::read_to_string("nukkit2.txt").unwrap();

    let o: Vec<_> = shark
        .split("+---------+---------------+----------+")
        .skip(1)
        .map(|x| {
            let bytes: Vec<u8> = (x.lines().find(|x| x.starts_with("|")).unwrap()[0..])
                .split("|")
                .skip(1)
                .flat_map(|x| {
                    let x = x.trim();
                    if x.len() != 2 {
                        return None;
                    }
                    Some(
                        from_hex(x.bytes().next().unwrap()) * 16
                            + from_hex(x.bytes().nth(1).unwrap()),
                    )
                })
                .collect();
            let time = x
                .lines()
                .find(|x| x.contains("ETHER"))
                .unwrap()
                .split(" ")
                .next()
                .unwrap();
            let p = PacketHeaders::from_ethernet_slice(&bytes).unwrap();

            Packet(p.payload.to_vec(), time.to_owned())
        })
        .collect();
    let mut frame_manager = FrameManager::default();
    for i in o {
        let mut iter = i.0.iter();

        match *Iterator::next(&mut iter).unwrap() {
            0x80..=0x88 => {
                let packet_phoenix = FramePacket::decode(&mut iter).unwrap();
                let (ack, paket) = frame_manager.process(packet_phoenix);

                if let Some(ej) = paket {
                    let mut e = ej.iter();
                    match *Iterator::next(&mut e).unwrap() {
                        0x09 => {
                            let packet_phoenix = ConnectionRequest::decode(&mut e).unwrap();
                            println!("{:?}", packet_phoenix);
                        }
                        0xFE => {
                            let _packet_phoenix = match GamePacket::decode(&mut e) {
                                Some(e) => e,
                                None => {
                                    println!(
                                        "Can't decode game packet {} {:?}",
                                        i.1,
                                        &ej[..ej.len().min(25)]
                                    );
                                    continue;
                                }
                            };

                            std::fs::write("game_packet.bin", &_packet_phoenix.0).unwrap();

                            let mut iter = _packet_phoenix.0.iter();

                            while let Ok(e) = ByteArray::decode(&mut iter) {
                                let mut iter = e.0.iter();
                                let uint = UnsignedVarInt::decode(&mut iter).unwrap().0 & 0x3FF;
                                match uint {
                                    0x01 => {
                                        let packet: LoginPacket =
                                            LoginPacket::decode(&mut iter).unwrap();
                                        println!("{:?}", packet);
                                    }
                                    0x02 => {
                                        let packet = PlayStatus::decode(&mut iter).unwrap();
                                        println!("{:?}", packet);
                                    }
                                    0x08 => {
                                        println!("Resource pack status {:?}", iter.read_to_end());
                                    }
                                    0x45 => {
                                        let request_chunk_radius_packet =
                                            RequestChunkRadiusPacket::decode(&mut iter).unwrap();
                                        println!("{:?}", request_chunk_radius_packet);
                                    }
                                    0x17 => {
                                        let tick = TickSyncPacket::decode(&mut iter).unwrap();
                                        println!("{:?}", tick);
                                    }
                                    0x0B => {
                                        let tick = match StartGamePacket::decode(&mut iter) {
                                            Ok(e) => e,
                                            Err(e) => {
                                                println!("{}", e);
                                                continue;
                                            }
                                        };
                                        println!("{:?}", tick);
                                    }
                                    0x9C => {
                                        println!("Violation : {:?}", iter.read_to_end());
                                    }
                                    e => {
                                        let p = iter.read_to_end();
                                        println!("Game Packet {} {:?}", e, &p[0..p.len().min(25)]);
                                    }
                                }
                            }

                            // TODO Do things with Game Packet
                        }
                        0x00 => {
                            let ping = ConnectedPing::decode(&mut e).unwrap();
                            println!("{:?}", ping);
                        }
                        e => {
                            println!("Nous sommes a la HEC! {}", e);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Packet(Vec<u8>, String);
