//! Binding shim for libz80e-core
extern crate libc;
use self::libc::{ c_int, c_void, pthread_mutex_t };

#[allow(dead_code)]
#[repr(C)]
pub struct Registers {
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
pub struct IoPort {
    pub device: *const c_void,
    pub read_in: extern fn(*const c_void) -> u8,
    pub write_out: extern fn(*mut c_void, u8),
}

#[allow(dead_code)]
#[repr(C)]
pub struct Cpu {
    pub devices: [IoPort; 0x100],
    pub registers: Registers,
    pub iff1: bool,
    pub iff2: bool,
    int_mode: u8,
    iff_wait: bool,
    pub halted: bool,
    prefix: u16,
    pub memory: *const c_void,
    pub read_byte: extern fn(*const c_void, u16) -> u8,
    pub write_byte: extern fn(*mut c_void, u16, u8),
    bus_lock: pthread_mutex_t,
    interrupt: bool,
    bus: u8,
}

#[allow(dead_code)]
#[link(name = "z80e-core")]
extern {
    pub fn cpu_init() -> *mut Cpu;
    pub fn cpu_free(cpu: *const Cpu);
    pub fn cpu_interrupt(cpu: *const Cpu, bus: u8) -> c_int;
    pub fn cpu_clear_interrupt(cpu: *const Cpu) -> c_int;
    pub fn cpu_try_interrupt(cpu: *const Cpu, bus: u8) -> c_int;
    pub fn cpu_try_clear_interrupt(cpu: *const Cpu) -> c_int;
    pub fn cpu_execute(cpu: *const Cpu, cycles: i32) -> i32;
}
