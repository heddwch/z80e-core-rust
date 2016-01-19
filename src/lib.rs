//! Z80 emulation library
extern crate libc;
mod z80e_core;
use libc::{ c_void, c_int };

/// An interface for a 16-bit addressed container of bytes.
pub trait Z80Memory {
    fn read_byte(&mut self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

impl Z80Memory for Vec<u8> {
    fn read_byte(&mut self, address: u16) -> u8 {
        self[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }
}

impl Z80Memory for [u8; 0x10000] {
    fn read_byte(&mut self, address: u16) -> u8 {
        self[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }
}

extern fn read_z80_memory<T: Z80Memory>(memory: *mut c_void, address: u16) -> u8 {
    let memory: &mut T = unsafe { &mut *(memory as *mut T) };
    memory.read_byte(address)
}

extern fn write_z80_memory<T: Z80Memory>(memory: *mut c_void, address: u16, value: u8) {
    let memory: &mut T = unsafe { &mut *(memory as *mut T) };
    memory.write_byte(address, value)
}

/// An interface for implementing one Z80 I/O port.
pub trait Z80IODevice {
    fn read_in(&mut self) -> u8;
    fn write_out(&mut self, value: u8);
}

extern fn read_z80_device<T: Z80IODevice>(device: *mut c_void) -> u8 {
    let device: &mut T = unsafe { &mut *(device as *mut T) };
    device.read_in()
}

extern fn write_z80_device<T: Z80IODevice>(device: *mut c_void, value: u8) {
    let device: &mut T = unsafe { &mut *(device as *mut T) };
    device.write_out(value)
}

/// Z80 CPU implementation.
pub struct Z80 {
    core: *mut z80e_core::z80e_cpu,
}

impl Z80 {
    /// Allocate a new Z80 core and install its memory.
    pub fn new<T: Z80Memory>(memory: &mut T) -> Self {
        unsafe {
            let cpu = Z80 {
                core: z80e_core::cpu_init(),
            };
            (*cpu.core).memory = (memory as *mut _) as *mut c_void;
            (*cpu.core).read_byte = read_z80_memory::<T>;
            (*cpu.core).write_byte = write_z80_memory::<T>;
            cpu
        }
    }
    /// Install a device on a Z80 I/O port.
    pub fn install_device<T: Z80IODevice>(&mut self, port: u8, device: &mut T) {
        let port: usize = port as usize;
        unsafe {
            (*self.core).devices[port].device = (device as *mut _) as *mut c_void;
            (*self.core).devices[port].read_in = read_z80_device::<T>;
            (*self.core).devices[port].write_out = write_z80_device::<T>;
        }
    }
    pub fn execute(&mut self, cycles: u32) -> u32 {
        unsafe {
            z80e_core::cpu_execute(self.core, cycles as c_int) as u32
        }
    }
}

impl Drop for Z80 {
    fn drop(&mut self) {
        unsafe {
            z80e_core::cpu_free(self.core);
        }
    }
}

#[test]
fn it_works() {
}
