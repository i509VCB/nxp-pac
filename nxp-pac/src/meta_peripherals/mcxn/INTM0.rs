#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "INTM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intm0 {
    ptr: *mut u8,
}
unsafe impl Send for Intm0 {}
unsafe impl Sync for Intm0 {}
impl Intm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Monitor Mode."]
    #[inline(always)]
    pub const fn intm_mm(self) -> crate::pac::common::Reg<IntmMm, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Acknowledge."]
    #[inline(always)]
    pub const fn intm_iack(self) -> crate::pac::common::Reg<IntmIack, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Monitoring."]
    #[inline(always)]
    pub const fn mon(self, n: usize) -> Mon {
        assert!(n < 4usize);
        unsafe { Mon::from_ptr(self.ptr.wrapping_add(0x08usize + n * 16usize) as _) }
    }
}
#[doc = "Monitoring."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mon {
    ptr: *mut u8,
}
unsafe impl Send for Mon {}
unsafe impl Sync for Mon {}
impl Mon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Request Select for Monitor mon_index."]
    #[inline(always)]
    pub const fn intm_irqsel(self) -> crate::pac::common::Reg<IntmIrqsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt Latency for Monitor mon_index."]
    #[inline(always)]
    pub const fn intm_latency(
        self,
    ) -> crate::pac::common::Reg<IntmLatency, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Timer for Monitor mon_index."]
    #[inline(always)]
    pub const fn intm_timer(self) -> crate::pac::common::Reg<IntmTimer, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status for Monitor mon_index."]
    #[inline(always)]
    pub const fn intm_status(self) -> crate::pac::common::Reg<IntmStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Interrupt Acknowledge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmIack(pub u32);
impl IntmIack {
    #[doc = "Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interrupt Request."]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IntmIack {
    #[inline(always)]
    fn default() -> IntmIack {
        IntmIack(0)
    }
}
impl core::fmt::Debug for IntmIack {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmIack")
            .field("irq", &self.irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmIack {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmIack {{ irq: {=u16:?} }}", self.irq())
    }
}
#[doc = "Interrupt Request Select for Monitor mon_index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmIrqsel(pub u32);
impl IntmIrqsel {
    #[doc = "Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Interrupt Request."]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for IntmIrqsel {
    #[inline(always)]
    fn default() -> IntmIrqsel {
        IntmIrqsel(0)
    }
}
impl core::fmt::Debug for IntmIrqsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmIrqsel")
            .field("irq", &self.irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmIrqsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmIrqsel {{ irq: {=u16:?} }}", self.irq())
    }
}
#[doc = "Interrupt Latency for Monitor mon_index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmLatency(pub u32);
impl IntmLatency {
    #[doc = "Latency."]
    #[must_use]
    #[inline(always)]
    pub const fn lat(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Latency."]
    #[inline(always)]
    pub const fn set_lat(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for IntmLatency {
    #[inline(always)]
    fn default() -> IntmLatency {
        IntmLatency(0)
    }
}
impl core::fmt::Debug for IntmLatency {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmLatency")
            .field("lat", &self.lat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmLatency {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmLatency {{ lat: {=u32:?} }}", self.lat())
    }
}
#[doc = "Monitor Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmMm(pub u32);
impl IntmMm {
    #[doc = "Monitor Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn mm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Mode."]
    #[inline(always)]
    pub const fn set_mm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IntmMm {
    #[inline(always)]
    fn default() -> IntmMm {
        IntmMm(0)
    }
}
impl core::fmt::Debug for IntmMm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmMm").field("mm", &self.mm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmMm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmMm {{ mm: {=bool:?} }}", self.mm())
    }
}
#[doc = "Status for Monitor mon_index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmStatus(pub u32);
impl IntmStatus {
    #[doc = "Monitor status."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> Status {
        let val = (self.0 >> 0usize) & 0x01;
        Status::from_bits(val as u8)
    }
    #[doc = "Monitor status."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: Status) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for IntmStatus {
    #[inline(always)]
    fn default() -> IntmStatus {
        IntmStatus(0)
    }
}
impl core::fmt::Debug for IntmStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmStatus")
            .field("status", &self.status())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmStatus {{ status: {:?} }}", self.status())
    }
}
#[doc = "Timer for Monitor mon_index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntmTimer(pub u32);
impl IntmTimer {
    #[doc = "Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timer."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for IntmTimer {
    #[inline(always)]
    fn default() -> IntmTimer {
        IntmTimer(0)
    }
}
impl core::fmt::Debug for IntmTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntmTimer")
            .field("timer", &self.timer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntmTimer {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IntmTimer {{ timer: {=u32:?} }}", self.timer())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status {
    #[doc = "Did not exceed."]
    St02 = 0x0,
    #[doc = "Exceeded."]
    St01 = 0x01,
}
impl Status {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status {
    #[inline(always)]
    fn from(val: u8) -> Status {
        Status::from_bits(val)
    }
}
impl From<Status> for u8 {
    #[inline(always)]
    fn from(val: Status) -> u8 {
        Status::to_bits(val)
    }
}
