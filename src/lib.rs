//! Z80 emulation library
extern crate libc;
mod z80e_core;
use libc::{ c_void };

pub enum Error {
    InvalidMutex,
    WouldBlock,
    Deadlock,
    Other(&'static str)
}
pub type Result<T> = std::result::Result<T, Error>;

/// An interface for a 16-bit addressed container of bytes.
pub trait Memory {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
}

impl Memory for Vec<u8> {
    fn read_byte(&self, address: u16) -> u8 {
        let address = address as usize;
        if address < self.len() {
            self[address]
        } else {
            0
        }
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        if address < self.len() {
            self[address] = value;
        }
    }
}

impl Memory for [u8; 0x10000] {
    fn read_byte(&self, address: u16) -> u8 {
        self[address as usize]
    }
    fn write_byte(&mut self, address: u16, value: u8) {
        self[address as usize] = value;
    }
}

extern fn read_memory<T: Memory>(memory: *const c_void, address: u16) -> u8 {
    let memory: &T = unsafe { &*(memory as *mut T) };
    memory.read_byte(address)
}

extern fn write_memory<T: Memory>(memory: *mut c_void, address: u16, value: u8) {
    let memory: &mut T = unsafe { &mut *(memory as *mut T) };
    memory.write_byte(address, value)
}

/// An interface for implementing one Z80 I/O port.
pub trait IoDevice {
    fn read_in(&self) -> u8;
    fn write_out(&mut self, value: u8);
}

extern fn read_device<T: IoDevice>(device: *const c_void) -> u8 {
    let device: &mut T = unsafe { &mut *(device as *mut T) };
    device.read_in()
}

extern fn write_device<T: IoDevice>(device: *mut c_void, value: u8) {
    let device: &mut T = unsafe { &mut *(device as *mut T) };
    device.write_out(value)
}

pub enum StopReason {
    Done,
    Halted(i32),
    Hung(i32),
    Error(Error),
}

/// Z80 CPU implementation.
pub struct Cpu {
    core: *mut z80e_core::Cpu,
}

impl Cpu {
    /// Allocate a new Z80 core and install its memory.
    pub fn new<T: Memory>(memory: &mut T) -> Self {
        unsafe {
            let cpu = Cpu {
                core: z80e_core::cpu_init(),
            };
            (*cpu.core).memory = (memory as *mut _) as *mut c_void;
            (*cpu.core).read_byte = read_memory::<T>;
            (*cpu.core).write_byte = write_memory::<T>;
            cpu
        }
    }
    /// Install a device on a Z80 I/O port.
    pub fn install_device<T: IoDevice>(&mut self, port: u8, device: &mut T) {
        let port: usize = port as usize;
        unsafe {
            (*self.core).devices[port].device = (device as *mut _) as *mut c_void;
            (*self.core).devices[port].read_in = read_device::<T>;
            (*self.core).devices[port].write_out = write_device::<T>;
        }
    }
    pub fn execute(&self, cycles: i32) -> StopReason {
        unsafe {
            let cycles = z80e_core::cpu_execute(self.core, cycles);
            if cycles < 0 {
                match cycles {
                    x if x == -libc::EINVAL => StopReason::Error(Error::InvalidMutex),
                    _ => StopReason::Error(Error::Other("Unknown error")),
                }
            } else {
                if (*self.core).halted {
                    if (*self.core).iff2 {
                        StopReason::Halted(cycles)
                    } else {
                        StopReason::Hung(cycles)
                    }
                } else {
                    StopReason::Done
                }
            }
        }
    }
    pub fn interrupt(&self, bus: u8) -> Result<()> {
        let status = unsafe {
            z80e_core::cpu_interrupt(self.core, bus)
        };
        match status {
            0 => Ok(()),
            libc::EINVAL => Err(Error::InvalidMutex),
            libc::EDEADLK => Err(Error::Deadlock),
            _ => Err(Error::Other("Unknown error")),
        }
    }
    pub fn clear_interrupt(&self) -> Result<()> {
        let status = unsafe {
            z80e_core::cpu_clear_interrupt(self.core)
        };
        match status {
            0 => Ok(()),
            libc::EINVAL => Err(Error::InvalidMutex),
            libc::EDEADLK => Err(Error::Deadlock),
            _ => Err(Error::Other("Unknown error")),
        }
    }
    pub fn try_interrupt(&self, bus: u8) -> Result<()> {
        let status = unsafe {
            z80e_core::cpu_try_interrupt(self.core, bus)
        };
        match status {
            0 => Ok(()),
            libc::EINVAL => Err(Error::InvalidMutex),
            libc::EBUSY => Err(Error::WouldBlock),
            _ => Err(Error::Other("Unknown error")),
        }
    }
    pub fn try_clear_interrupt(&self) -> Result<()> {
        let status = unsafe {
            z80e_core::cpu_try_clear_interrupt(self.core)
        };
        match status {
            0 => Ok(()),
            libc::EINVAL => Err(Error::InvalidMutex),
            libc::EBUSY => Err(Error::WouldBlock),
            _ => Err(Error::Other("Unknown error")),
        }
    }
}

impl Drop for Cpu {
    fn drop(&mut self) {
        unsafe {
            z80e_core::cpu_free(self.core);
        }
    }
}

#[test]
fn it_works() {
}
