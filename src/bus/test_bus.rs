#![allow(dead_code)]

use crate::cpu::CpuInterface;
use crate::cpu::Interface;
use crate::savable::Savable;

/// Bus used for easier cpu testing
pub struct TestBus {
    ram: [u8; 0x800],
    program: Vec<u8>,
}

impl Interface for TestBus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize],
            _ => self.program[(addr - 0x2000) as usize],
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram[(addr & 0x7FF) as usize] = data,
            _ => self.program[(addr - 0x2000) as usize] = data,
        }
    }
}

impl CpuInterface for TestBus {}

impl TestBus {
    pub fn new(program: Vec<u8>) -> Self {
        Self {
            ram: [0; 0x800],
            program,
        }
    }

    /// Sets a value at a RAM address
    pub fn set_ram(&mut self, addr: u16, data: u8) {
        self.ram[(addr & 0x7FF) as usize] = data;
    }
}

impl Savable for TestBus {}
