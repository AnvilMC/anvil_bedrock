use packet_derive::MCPEPacketDataAuto;

#[derive(MCPEPacketDataAuto, Debug)]
pub struct BiomeIdArray {
    biomes: [u8; 256],
}

impl Default for BiomeIdArray {
    fn default() -> Self {
        println!("A3");
        Self { biomes: [0; 256] }
    }
}

impl BiomeIdArray {
    pub fn set_biome(&mut self, x: u8, z: u8, biome_id: u8) {
        self.biomes[(x << 4 | z) as usize] = biome_id;
    }
    pub fn get_biome(&mut self, x: u8, z: u8) -> u8 {
        self.biomes[(x << 4 | z) as usize]
    }
}
