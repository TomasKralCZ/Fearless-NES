use super::super::cartridge::Cartridge;
use super::super::cartridge::Mirroring;

use super::Mapper;

pub struct Nrom {
    pub prg_1: usize,
    pub prg_2: usize,
    pub chr: usize,
    pub ram: [u8; 0x2000],
    pub nt_ram: [u8; 0x1000],
    pub cartridge: Cartridge,
}

impl Nrom {
    pub fn new(cartridge: Cartridge) -> Nrom {
        let mut mapper = Nrom {
            prg_1: 0,
            prg_2: 0,
            chr: 0,
            ram: [0; 0x2000],
            nt_ram: [0; 0x1000],
            cartridge,
        };

        mapper.initialize_banks();
        mapper
    }

    fn initialize_banks(&mut self) {
        let prg_count = self.cartridge.header.prg_rom_size as usize;
        match prg_count {
            1 => {
                self.prg_1 = 0;
                self.prg_2 = 0;
            }
            2 => {
                self.prg_1 = 0;
                self.prg_2 = 0x4000;
            }
            _ => panic!("NROM shouldn't have more than 2 PRG banks"),
        }

        let chr_start = 0x4000 * prg_count;
        match self.cartridge.header.chr_rom_size {
            0 => (),
            1 => self.chr = chr_start,
            _ => panic!("NROM shouldn't have more than 1 CHR bank"),
        }
    }
}

impl Mapper for Nrom {
    fn cpu_read_direct(&mut self, addr: usize) -> u8 {
        match addr {
            0x6000..=0x7FFF => self.ram[addr - 0x6000],
            0x8000..=0xBFFF => self.cartridge.rom[self.prg_1 + (addr - 0x8000)],
            0xC000..=0xFFFF => self.cartridge.rom[self.prg_2 + (addr - 0xC000)],
            _ => 0,
        }
    }

    fn cpu_read(&mut self, addr: usize) -> Option<u8> {
        match addr {
            0x6000..=0x7FFF => Some(self.ram[addr - 0x6000]),
            0x8000..=0xBFFF => Some(self.cartridge.rom[self.prg_1 + (addr - 0x8000)]),
            0xC000..=0xFFFF => Some(self.cartridge.rom[self.prg_2 + (addr - 0xC000)]),
            _ => None,
        }
    }

    fn cpu_write(&mut self, addr: usize, val: u8) {
        match addr {
            0x6000..=0x7FFF => self.ram[addr - 0x6000] = val,
            _ => (),
        }
    }

    fn read_chr(&mut self, addr: usize) -> u8 {
        self.cartridge.rom[self.chr + addr]
    }

    fn write_chr(&mut self, addr: usize, val: u8) {
        if self.cartridge.header.chr_rom_size == 0 {
            self.cartridge.rom[self.chr + addr] = val;
        }
    }

    fn read_nametable(&mut self, addr: usize) -> u8 {
        self.nt_ram[addr]
    }

    fn write_nametable(&mut self, addr: usize, val: u8) {
        self.nt_ram[addr] = val;
    }

    fn mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}