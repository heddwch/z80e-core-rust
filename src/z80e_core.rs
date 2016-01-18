//! Binding shim for libz80e-core
extern crate libc;
use self::libc::{ c_int, c_void, pthread_mutex_t };

#[allow(dead_code)]
#[repr(C)]
pub struct z80e_registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    _af: u16,
    _bc: u16,
    _de: u16,
    _hl: u16,
    pc: u16,
    sp: u16,
    ix: u16,
    iy: u16,
    i: u8,
    r: u8,
    wz: u16,
}

#[allow(dead_code)]
#[repr(C)]
pub struct z80e_io_device {
    pub device: *mut c_void,
    pub read_in: extern fn(*mut c_void) -> u8,
    pub write_out: extern fn(*mut c_void, u8),
}

#[allow(dead_code)]
#[repr(C)]
pub struct z80e_cpu {
    pub devices: [z80e_io_device; 100],
    registers: z80e_registers,
    iff1: bool,
    iff2: bool,
    int_mode: u8,
    iff_wait: bool,
    halted: bool,
    prefix: u16,
    pub memory: *mut c_void,
    pub read_byte: extern fn(*mut c_void, u16) -> u8,
    pub write_byte: extern fn(*mut c_void, u16, u8),
    bus_lock: pthread_mutex_t,
    interrupt: bool,
    bus: u8,
}

#[allow(dead_code)]
#[link(name = "z80e-core")]
extern {
    pub fn cpu_init() -> *mut z80e_cpu;
    pub fn cpu_free(cpu: *mut z80e_cpu);
    pub fn cpu_interrupt(cpu: *mut z80e_cpu, bus: u8) -> c_int;
    pub fn cpu_try_interrupt(cpu: *mut z80e_cpu, bus: u8) -> c_int;
//    pub fn cpu_clear_interrupt(cpu: *mut z80e_cpu) -> c_int;
//    pub fn cpu_try_clear_interrupt(cpu: *mut z80e_cpu) -> c_int;
    pub fn cpu_execute(cpu: *mut z80e_cpu, cycles: c_int) -> c_int;
}
