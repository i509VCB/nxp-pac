#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "SEMA42."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sema42 {
    ptr: *mut u8,
}
unsafe impl Send for Sema42 {}
unsafe impl Sync for Sema42 {}
impl Sema42 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate3(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate2(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate1(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate0(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate7(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate6(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate5(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate4(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate11(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate10(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate9(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate8(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate15(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate14(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate13(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Gate."]
    #[inline(always)]
    pub const fn gate12(self) -> crate::pac::common::Reg<Gate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "Reset Gate Read."]
    #[inline(always)]
    pub const fn rstgt_r(self) -> crate::pac::common::Reg<RstgtR, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Reset Gate Write."]
    #[inline(always)]
    pub const fn rstgt_w(self) -> crate::pac::common::Reg<RstgtW, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
}
#[doc = "Gate."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gate(pub u8);
impl Gate {
    #[doc = "Gate Finite State Machine."]
    #[must_use]
    #[inline(always)]
    pub const fn gtfsm(&self) -> GateGtfsm {
        let val = (self.0 >> 0usize) & 0x0f;
        GateGtfsm::from_bits(val as u8)
    }
    #[doc = "Gate Finite State Machine."]
    #[inline(always)]
    pub const fn set_gtfsm(&mut self, val: GateGtfsm) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u8) & 0x0f) << 0usize);
    }
}
impl Default for Gate {
    #[inline(always)]
    fn default() -> Gate {
        Gate(0)
    }
}
impl core::fmt::Debug for Gate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gate")
            .field("gtfsm", &self.gtfsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gate {{ gtfsm: {:?} }}", self.gtfsm())
    }
}
#[doc = "Reset Gate Read."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtR(pub u16);
impl RstgtR {
    #[doc = "Reset Gate Number."]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number."]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Domain."]
    #[must_use]
    #[inline(always)]
    pub const fn rstgms(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Reset Gate Domain."]
    #[inline(always)]
    pub const fn set_rstgms(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Reset Gate Finite State Machine."]
    #[must_use]
    #[inline(always)]
    pub const fn rstgsm(&self) -> Rstgsm {
        let val = (self.0 >> 12usize) & 0x03;
        Rstgsm::from_bits(val as u8)
    }
    #[doc = "Reset Gate Finite State Machine."]
    #[inline(always)]
    pub const fn set_rstgsm(&mut self, val: Rstgsm) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
}
impl Default for RstgtR {
    #[inline(always)]
    fn default() -> RstgtR {
        RstgtR(0)
    }
}
impl core::fmt::Debug for RstgtR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtR")
            .field("rstgtn", &self.rstgtn())
            .field("rstgms", &self.rstgms())
            .field("rstgsm", &self.rstgsm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtR {{ rstgtn: {=u8:?}, rstgms: {=u8:?}, rstgsm: {:?} }}",
            self.rstgtn(),
            self.rstgms(),
            self.rstgsm()
        )
    }
}
#[doc = "Reset Gate Write."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstgtW(pub u16);
impl RstgtW {
    #[doc = "Reset Gate Number."]
    #[must_use]
    #[inline(always)]
    pub const fn rstgtn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Number."]
    #[inline(always)]
    pub const fn set_rstgtn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Reset Gate Data Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn rstgdp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Reset Gate Data Pattern."]
    #[inline(always)]
    pub const fn set_rstgdp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for RstgtW {
    #[inline(always)]
    fn default() -> RstgtW {
        RstgtW(0)
    }
}
impl core::fmt::Debug for RstgtW {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstgtW")
            .field("rstgtn", &self.rstgtn())
            .field("rstgdp", &self.rstgdp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstgtW {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RstgtW {{ rstgtn: {=u8:?}, rstgdp: {=u8:?} }}",
            self.rstgtn(),
            self.rstgdp()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GateGtfsm {
    #[doc = "The gate is unlocked (free)."]
    Unlocked = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LockedByD0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LockedByD1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LockedByD2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LockedByD3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LockedByD4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LockedByD5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LockedByD6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LockedByD7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LockedByD8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LockedByD9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LockedByD10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LockedByD11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LockedByD12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LockedByD13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LockedByD14 = 0x0f,
}
impl GateGtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GateGtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GateGtfsm {
    #[inline(always)]
    fn from(val: u8) -> GateGtfsm {
        GateGtfsm::from_bits(val)
    }
}
impl From<GateGtfsm> for u8 {
    #[inline(always)]
    fn from(val: GateGtfsm) -> u8 {
        GateGtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstgsm {
    #[doc = "Idle, waiting for the first data pattern write."]
    Idle = 0x0,
    #[doc = "Waiting for the second data pattern write."]
    Waiting = 0x01,
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
    TwoWriteDone = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rstgsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstgsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstgsm {
    #[inline(always)]
    fn from(val: u8) -> Rstgsm {
        Rstgsm::from_bits(val)
    }
}
impl From<Rstgsm> for u8 {
    #[inline(always)]
    fn from(val: Rstgsm) -> u8 {
        Rstgsm::to_bits(val)
    }
}
