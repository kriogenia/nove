use crate::exception::NoveError;
use crate::Program;

const NES_TAG: [u8; 4] = [b'N', b'E', b'S', 0x1a];
const HEADER_SIZE: usize = 16;
const TRAINER_SIZE: usize = 512;
const PRG_ROM_PAGE_SIZE: usize = 16384; // 16kB
const CHR_ROM_PAGE_SIZE: usize = 8192; //  8kB

#[derive(Debug, PartialEq)]
enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

impl From<u8> for Mirroring {
    fn from(value: u8) -> Self {
        match (value & 0b1000 != 0, value & 0b1 != 0) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rom {
    pub(crate) prg_rom: Program,
    chr_rom: Program,
    mapper: u8,
    screen_mirroring: Mirroring,
}

impl Rom {
    pub fn new(raw: &Program) -> Result<Rom, NoveError> {
        if raw[0..4] != NES_TAG {
            return Err(NoveError::WrongRomFormat);
        }

        if ((raw[7] >> 2) & 0b11) != 0 {
            return Err(NoveError::WrongRomFormat);
        }

        let prg_rom_size = raw[4] as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = raw[5] as usize * CHR_ROM_PAGE_SIZE;

        let trainer = raw[6] & 0b100 != 0;

        let prg_rom_start = HEADER_SIZE + if trainer { TRAINER_SIZE } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;
        let chr_rom_end = chr_rom_start + chr_rom_size;

        Ok(Rom {
            prg_rom: raw[prg_rom_start..chr_rom_start].to_vec(),
            chr_rom: raw[chr_rom_start..chr_rom_end].to_vec(),
            mapper: (raw[7] & 0b1111_0000) | (raw[6] >> 4),
            screen_mirroring: raw[6].into(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::exception::NoveError;

    #[test]
    fn invalid_format() {
        assert_eq!(
            Err(NoveError::WrongRomFormat),
            Rom::new(&vec![0, 1, 2, 3, 4])
        );
        assert_eq!(
            Err(NoveError::WrongRomFormat),
            Rom::new(&vec![b'N', b'E', b'S', 0x1a, 0, 0, 0, 4])
        );
    }
}
