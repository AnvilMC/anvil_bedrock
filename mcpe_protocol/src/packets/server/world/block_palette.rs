use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    prelude::{Le, VarInt},
    traits::MCPEPacketData,
};

#[derive(Debug, Clone)]
pub struct PalettedBlockStorage {
    palette: Vec<i32>, // Default capacity 16
    bit_array: BitArray,
}

impl PalettedBlockStorage {
    pub fn new(version: &'static BitArrayVersion) -> Self {
        Self {
            palette: {
                let mut vec = Vec::with_capacity(16);
                vec.push(GLOBAL_BLOCK_PALETTE.get_or_create_runtime_id(0, 0));
                vec
            },
            bit_array: version.create_palette([0; 4096]),
        }
    }

    fn get_palette_header(&self, runtime: bool) -> u8 {
        return (self.bit_array.version.bits as u8) << 1 | if runtime { 1 } else { 0 };
    }

    pub fn set_block(&mut self, index: i32, runtime_id: i32) {
        let id = self.id_for(runtime_id);
        self.bit_array.set(index, id);
    }

    fn id_for(&mut self, runtime_id: i32) -> i32 {
        if let Some(index) = self.palette.iter().position(|x| x == &runtime_id) {
            index as i32
        } else {
            let index = self.palette.len() as i32;
            let version = self.bit_array.version;
            if index > version.max_entry_value {
                self.resize(version.next_version.unwrap());
            }

            self.palette.push(runtime_id);
            return index;
        }
    }

    fn resize(&mut self, version: &'static BitArrayVersion) {
        let mut new_bit_array = version.create_palette([0; 4096]);
        (0..4096).for_each(|x| {
            new_bit_array.set(x, self.bit_array.get(x));
        });
        self.bit_array = new_bit_array;
    }
}

impl MCPEPacketData for PalettedBlockStorage {
    fn decode(
        _: &mut impl crate::traits::Reader,
    ) -> Result<Self, crate::prelude::MCPEPacketDataError> {
        todo!()
    }

    fn encode(
        &self,
        writer: &mut impl crate::traits::Writer,
    ) -> Result<(), crate::prelude::MCPEPacketDataError> {
        self.get_palette_header(true).encode(writer)?;

        for var4 in &self.bit_array.words {
            Le(*var4).encode(writer)?;
        }

        VarInt(self.palette.len() as i32).encode(writer)?;

        for i in &self.palette {
            VarInt(*i).encode(writer)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct BitArrayVersion {
    bits: i32,
    entries_per_word: i32,
    max_entry_value: i32,
    next_version: Option<&'static BitArrayVersion>,
}

macro_rules! bit_array_version {
    ($n:ident,$a:expr,$b:expr,$c:expr) => {
        pub const $n: BitArrayVersion = BitArrayVersion {
            bits: $a,
            entries_per_word: $b,
            max_entry_value: (1 << $a) - 1,
            next_version: $c,
        };
    };
}
bit_array_version!(V16, 16, 2, None);
bit_array_version!(V8, 8, 4, Some(&V16));
bit_array_version!(V6, 6, 5, Some(&V8));
bit_array_version!(V5, 5, 6, Some(&V6));
bit_array_version!(V4, 4, 8, Some(&V5));
bit_array_version!(V3, 3, 10, Some(&V4));
bit_array_version!(V2, 2, 16, Some(&V3));
bit_array_version!(V1, 1, 32, Some(&V2));

impl BitArrayVersion {
    fn create_palette(&'static self, words: [i32; 4096]) -> BitArray {
        match self.bits {
            3..=6 => BitArray::from_padded(self, words),
            _ => BitArray::from_pow2(self, words),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BitArray {
    pow2: bool,
    version: &'static BitArrayVersion,
    words: [i32; 4096],
}

impl BitArray {
    fn from_pow2(version: &'static BitArrayVersion, words: [i32; 4096]) -> Self {
        /*
        TODO: Add check
        int expectedWordsLength = MathHelper.ceil((float)size / (float)version.entriesPerWord);
        if (words.length != expectedWordsLength) {
            throw new IllegalArgumentException("Invalid length given for storage, got: " + words.length + " but expected: " + expectedWordsLength);
        }
         */
        Self {
            pow2: true,
            version,
            words,
        }
    }
    fn from_padded(version: &'static BitArrayVersion, words: [i32; 4096]) -> Self {
        /*
        TODO: Add check
        int expectedWordsLength = MathHelper.ceil((float)size / (float)version.entriesPerWord);
        if (words.length != expectedWordsLength) {
            throw new IllegalArgumentException("Invalid length given for storage, got: " + words.length + " but expected: " + expectedWordsLength);
        }
         */
        Self {
            pow2: false,
            version,
            words,
        }
    }

    pub fn set(&mut self, index: i32, value: i32) {
        if self.pow2 {
            // TODO: Add check
            // Preconditions.checkElementIndex(index, this.size);
            // Preconditions.checkArgument(value >= 0 && value <= this.version.maxEntryValue, "Max value: %s. Received value", this.version.maxEntryValue, value);
            let bit_index = index * self.version.bits;
            let array_index = (bit_index >> 5) as usize;
            let offset = bit_index & 31;
            self.words[array_index] = self.words[array_index]
                & !((self.version.max_entry_value) << offset)
                | (value & self.version.max_entry_value) << offset;
        } else {
            // TODO: Add check
            // Preconditions.checkElementIndex(index, this.size);
            // Preconditions.checkArgument(value >= 0 && value <= this.version.maxEntryValue, "Max value: %s. Received value", this.version.maxEntryValue, value);
            let array_index = (index / self.version.entries_per_word) as usize;
            let offset = index % self.version.entries_per_word * self.version.bits;
            self.words[array_index] = self.words[array_index]
                & !(self.version.max_entry_value << offset)
                | (value & self.version.max_entry_value) << offset;
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        if self.pow2 {
            // TODO: Add check
            //Preconditions.checkElementIndex(index, this.size);
            let bit_index = index * self.version.bits as i32;
            let array_index = (bit_index >> 5) as usize;
            let word_offset = bit_index & 31;
            // Don't know the >>> operator so just used >>
            self.words[array_index] >> word_offset & (self.version.max_entry_value as i32)
        } else {
            // TODO: Add check
            // Preconditions.checkElementIndex(index, this.size);
            let array_index = (index / self.version.entries_per_word) as usize;
            let offset = index % self.version.entries_per_word * self.version.bits;
            // Don't know the >>> operator so just used >>
            self.words[array_index] >> offset & self.version.max_entry_value
        }
    }
}

lazy_static! {
    pub static ref GLOBAL_BLOCK_PALETTE: GlobalBlockPalette = GlobalBlockPalette::new();
}

pub struct GlobalBlockPalette {
    legacy_to_runtime_id: HashMap<i32, i32>,
    _runtime_id_to_legacy: HashMap<i32, i32>,
}

impl GlobalBlockPalette {
    pub fn get_or_create_runtime_id(&self, id: i32, meta: i32) -> i32 {
        let legacy_id = id << 6 | meta;
        if let Some(e) = self.legacy_to_runtime_id.get(&legacy_id) {
            *e
        } else if let Some(e) = self.legacy_to_runtime_id.get(&(legacy_id << 6)) {
            *e
        } else {
            panic!("No runtime ID for unknown block {}", id);
        }
    }

    fn new() -> Self {
        let legacy_to_runtime_id: HashMap<i32, i32> = include_str!("id_map.txt")
            .split(",")
            .map(|x| {
                let mut i = x.split(":");
                (
                    i.next().unwrap().parse().unwrap(),
                    i.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        Self {
            _runtime_id_to_legacy: legacy_to_runtime_id.iter().map(|(x, y)| (*y, *x)).collect(),
            legacy_to_runtime_id,
        }
    }
}
