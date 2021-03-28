pub trait BitInformation {
    
    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize;
    
    /// Checks to see if bit X is set
    fn has_x_bit(&self, position: usize) -> bool;
    
    /// Checks to see if the requested bit position is in bounds
    fn is_bit_in_bounds(&self, position: usize) -> bool {
        position <= self.number_of_bits()
    }
    
    /// Checks to see if the most signifigant bit is set
    fn has_most_signifigant_bit(&self) -> bool {
        self.has_x_bit(self.number_of_bits() - 1) //8 bits? position 7.
    }
    
}

impl BitInformation for u8 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        8
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i8 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        8
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u16 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        16
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i16 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        16
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u32 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        32
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i32 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        32
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b00000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for u64 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        64
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for i64 {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        64
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for usize {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        (self.count_ones() + self.count_zeros()) as usize
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}

impl BitInformation for isize {

    /// Gets the number of bits contained in this type
    fn number_of_bits(&self) -> usize {
        (self.count_ones() + self.count_zeros()) as usize
    }
    
    fn has_x_bit(&self, position: usize) -> bool {
        if self.is_bit_in_bounds(position) {
            return (self & (0b0000000000000000000000000000000000000000000000000000000000000001 << position)) != 0;
        } else {
            return false; //Huh
        }
    }

}