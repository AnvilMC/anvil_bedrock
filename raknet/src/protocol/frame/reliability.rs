#[derive(Debug)]
pub struct Reliability {
    pub id: u8,
    pub reliable: bool,
    pub ordered: bool,
    pub sequenced: bool,
    pub requires_ack: bool,
}

impl Reliability {
    pub(crate) const fn new(
        id: u8,
        reliable: bool,
        ordered: bool,
        sequenced: bool,
        requires_ack: bool,
    ) -> Self {
        Self {
            id,
            reliable,
            ordered,
            sequenced,
            requires_ack,
        }
    }

    pub(crate) fn lookup(id: u8) -> Option<Self> {
        Some(match id {
            0 => UNRELIABLE,
            1 => UNRELIABLE_SEQUENCED,
            2 => RELIABLE,
            3 => RELIABLE_ORDERED,
            4 => RELIABLE_SEQUENCED,
            5 => UNRELIABLE_WITH_ACK_RECEIPT,
            6 => RELIABLE_WITH_ACK_RECEIPT,
            7 => RELIABLE_ORDERED_WITH_ACK_RECEIPT,
            _ => return None,
        })
    }

    pub(crate) fn compute_flag(&self, is_split: bool) -> u8 {
        (self.id << FLAG_RELIABILITY_INDEX) | if is_split { FLAG_SPLIT } else { 0 }
    }
}

pub const UNRELIABLE: Reliability = Reliability::new(0, false, false, false, false);
pub const UNRELIABLE_SEQUENCED: Reliability = Reliability::new(1, false, false, true, false);
pub const RELIABLE: Reliability = Reliability::new(2, true, false, false, false);
pub const RELIABLE_ORDERED: Reliability = Reliability::new(3, true, true, false, false);
pub const RELIABLE_SEQUENCED: Reliability = Reliability::new(4, true, false, true, false);
pub const UNRELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(5, false, false, false, true);
pub const RELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(6, true, false, false, true);
pub const RELIABLE_ORDERED_WITH_ACK_RECEIPT: Reliability =
    Reliability::new(7, true, true, false, true);

pub(crate) const FLAG_RELIABILITY_INDEX: u8 = 5;
pub(crate) const FLAG_RELIABILITY: u8 = 0b11100000;
pub(crate) const FLAG_SPLIT: u8 = 0b00010000;
