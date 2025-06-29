use bitintr::Tzcnt;
// I know there are crates for this
// I just really like implementing it...

pub struct BitSet {
    bytes: Vec<u8>,
    num_bits: usize,
}

impl BitSet {
    pub fn with_bits(bits: usize) -> Self {
        BitSet { bytes: vec![0; bits / 8 + 1], num_bits: bits }
    }
}

impl BitSet {
    pub fn set(&mut self, pos: usize) {
        self.bytes[pos >> 3] |= 1u8 << (pos & 7)
    }
    pub fn unset(&mut self, pos: usize) {
        self.bytes[pos >> 3] &= !(1u8 << (pos & 7))
    }

    pub fn get(&self, pos: usize) -> u8 {
        (self.bytes[pos >> 3] >> (pos & 7)) & 1
    }
}

impl BitSet {
    pub fn find_first_unset(&self) -> Option<usize> {
        for (by_pos, &byte) in self.bytes.iter().enumerate() {
            if byte == 0xFF { continue; }
            
            let in_pos = (!byte).tzcnt() as usize;
            if by_pos * 8 + in_pos < self.num_bits {
                return Some(by_pos * 8 + in_pos);
            } else {
                break;
            }
        }

        return None;
    }

    // also returns all unset
    pub fn find_all_unset(&mut self) -> Vec<usize> {
        let mut unset = vec![];

        for (by_pos, &byte) in self.bytes.iter().enumerate() {
            // TODO: check by u64 for a maybe 8x speedup
            let mut mbyte = byte;
            
            while mbyte != 0xFF {
                let in_pos = (!mbyte).tzcnt() as usize;
                mbyte |= 1 << in_pos;

                if by_pos * 8 + in_pos < self.num_bits {
                    unset.push(by_pos * 8 + in_pos);
                } else {
                    break;
                }
            }
        }

        return unset;
    }
}

impl ToString for BitSet {
    fn to_string(&self) -> String {
        format!("{:?}", self.bytes)
    }
}