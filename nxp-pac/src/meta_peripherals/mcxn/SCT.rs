#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: EV_STATE, EV_CTRL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Event {
    ptr: *mut u8,
}
unsafe impl Send for Event {}
unsafe impl Sync for Event {}
impl Event {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Event n State."]
    #[inline(always)]
    pub const fn ev_state(self) -> crate::pac::common::Reg<EvState, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event n Control."]
    #[inline(always)]
    pub const fn ev_ctrl(self) -> crate::pac::common::Reg<EvCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "Array of registers: OUT_SET, OUT_CLR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out {
    ptr: *mut u8,
}
unsafe impl Send for Out {}
unsafe impl Sync for Out {}
impl Out {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output n Set."]
    #[inline(always)]
    pub const fn out_set(self) -> crate::pac::common::Reg<OutSet, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output n Clear."]
    #[inline(always)]
    pub const fn out_clr(self) -> crate::pac::common::Reg<OutClr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "SCT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct {
    ptr: *mut u8,
}
unsafe impl Send for Sct {}
unsafe impl Sync for Sct {}
impl Sct {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT Configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::pac::common::Reg<Config, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SCT Limit Event Select."]
    #[inline(always)]
    pub const fn limit(self) -> crate::pac::common::Reg<Limit, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Halt Event Select."]
    #[inline(always)]
    pub const fn halt(self) -> crate::pac::common::Reg<Halt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Stop Event Select."]
    #[inline(always)]
    pub const fn stop(self) -> crate::pac::common::Reg<Stop, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Start Event Select."]
    #[inline(always)]
    pub const fn start(self) -> crate::pac::common::Reg<Start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Dither Condition."]
    #[inline(always)]
    pub const fn dither(self) -> crate::pac::common::Reg<Dither, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Counter Value."]
    #[inline(always)]
    pub const fn count(self) -> crate::pac::common::Reg<Count, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "State Variable."]
    #[inline(always)]
    pub const fn state(self) -> crate::pac::common::Reg<State, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Input State."]
    #[inline(always)]
    pub const fn input(self) -> crate::pac::common::Reg<Input, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Match and Capture Register Mode."]
    #[inline(always)]
    pub const fn regmode(self) -> crate::pac::common::Reg<Regmode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Output State."]
    #[inline(always)]
    pub const fn output(self) -> crate::pac::common::Reg<Output, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Output Counter Direction Control."]
    #[inline(always)]
    pub const fn outputdirctrl(
        self,
    ) -> crate::pac::common::Reg<Outputdirctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Output Conflict Resolution."]
    #[inline(always)]
    pub const fn res(self) -> crate::pac::common::Reg<Res, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DMA Request 0."]
    #[inline(always)]
    pub const fn dmareq0(self) -> crate::pac::common::Reg<Dmareq0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "DMA Request 1."]
    #[inline(always)]
    pub const fn dmareq1(self) -> crate::pac::common::Reg<Dmareq1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Event Interrupt Enable."]
    #[inline(always)]
    pub const fn even(self) -> crate::pac::common::Reg<Even, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Event Flag."]
    #[inline(always)]
    pub const fn evflag(self) -> crate::pac::common::Reg<Evflag, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Conflict Interrupt Enable."]
    #[inline(always)]
    pub const fn conen(self) -> crate::pac::common::Reg<Conen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Conflict Flag."]
    #[inline(always)]
    pub const fn conflag(self) -> crate::pac::common::Reg<Conflag, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Value."]
    #[inline(always)]
    pub const fn cap(self, n: usize) -> crate::pac::common::Reg<Cap, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Match Value."]
    #[inline(always)]
    pub const fn match_(self, n: usize) -> crate::pac::common::Reg<Match, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Fractional Match."]
    #[inline(always)]
    pub const fn fracmat(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Fracmat, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "Capture Control."]
    #[inline(always)]
    pub const fn capctrl(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Capctrl, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Match Reload Value."]
    #[inline(always)]
    pub const fn matchrel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Matchrel, crate::pac::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Fractional Match Reload."]
    #[inline(always)]
    pub const fn fracmatrel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Fracmatrel, crate::pac::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: EV_STATE, EV_CTRL."]
    #[inline(always)]
    pub const fn event(self, n: usize) -> Event {
        assert!(n < 16usize);
        unsafe { Event::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 8usize) as _) }
    }
    #[doc = "Array of registers: OUT_SET, OUT_CLR."]
    #[inline(always)]
    pub const fn out(self, n: usize) -> Out {
        assert!(n < 10usize);
        unsafe { Out::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 8usize) as _) }
    }
}
#[doc = "Capture Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Capture Low."]
    #[must_use]
    #[inline(always)]
    pub const fn ca_pn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Low."]
    #[inline(always)]
    pub const fn set_ca_pn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture High."]
    #[must_use]
    #[inline(always)]
    pub const fn ca_pn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture High."]
    #[inline(always)]
    pub const fn set_ca_pn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        Cap(0)
    }
}
impl core::fmt::Debug for Cap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cap")
            .field("ca_pn_l", &self.ca_pn_l())
            .field("ca_pn_h", &self.ca_pn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap {{ ca_pn_l: {=u16:?}, ca_pn_h: {=u16:?} }}",
            self.ca_pn_l(),
            self.ca_pn_h()
        )
    }
}
#[doc = "Capture Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capctrl(pub u32);
impl Capctrl {
    #[doc = "Capture Control Low."]
    #[must_use]
    #[inline(always)]
    pub const fn capco_nn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control Low."]
    #[inline(always)]
    pub const fn set_capco_nn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capture Control High."]
    #[must_use]
    #[inline(always)]
    pub const fn capco_nn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Capture Control High."]
    #[inline(always)]
    pub const fn set_capco_nn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Capctrl {
    #[inline(always)]
    fn default() -> Capctrl {
        Capctrl(0)
    }
}
impl core::fmt::Debug for Capctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capctrl")
            .field("capco_nn_l", &self.capco_nn_l())
            .field("capco_nn_h", &self.capco_nn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capctrl {{ capco_nn_l: {=u16:?}, capco_nn_h: {=u16:?} }}",
            self.capco_nn_l(),
            self.capco_nn_h()
        )
    }
}
#[doc = "Conflict Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conen(pub u32);
impl Conen {
    #[doc = "No Change Conflict Event and Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ncen(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event and Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ncen(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Conen {
    #[inline(always)]
    fn default() -> Conen {
        Conen(0)
    }
}
impl core::fmt::Debug for Conen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conen")
            .field("ncen[0]", &self.ncen(0usize))
            .field("ncen[1]", &self.ncen(1usize))
            .field("ncen[2]", &self.ncen(2usize))
            .field("ncen[3]", &self.ncen(3usize))
            .field("ncen[4]", &self.ncen(4usize))
            .field("ncen[5]", &self.ncen(5usize))
            .field("ncen[6]", &self.ncen(6usize))
            .field("ncen[7]", &self.ncen(7usize))
            .field("ncen[8]", &self.ncen(8usize))
            .field("ncen[9]", &self.ncen(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Conen {{ ncen[0]: {=bool:?}, ncen[1]: {=bool:?}, ncen[2]: {=bool:?}, ncen[3]: {=bool:?}, ncen[4]: {=bool:?}, ncen[5]: {=bool:?}, ncen[6]: {=bool:?}, ncen[7]: {=bool:?}, ncen[8]: {=bool:?}, ncen[9]: {=bool:?} }}",
            self.ncen(0usize),
            self.ncen(1usize),
            self.ncen(2usize),
            self.ncen(3usize),
            self.ncen(4usize),
            self.ncen(5usize),
            self.ncen(6usize),
            self.ncen(7usize),
            self.ncen(8usize),
            self.ncen(9usize)
        )
    }
}
#[doc = "SCT Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Config(pub u32);
impl Config {
    #[doc = "SCT Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn unify(&self) -> Unify {
        let val = (self.0 >> 0usize) & 0x01;
        Unify::from_bits(val as u8)
    }
    #[doc = "SCT Operation."]
    #[inline(always)]
    pub const fn set_unify(&mut self, val: Unify) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SCT Clock Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn clkmode(&self) -> Clkmode {
        let val = (self.0 >> 1usize) & 0x03;
        Clkmode::from_bits(val as u8)
    }
    #[doc = "SCT Clock Mode."]
    #[inline(always)]
    pub const fn set_clkmode(&mut self, val: Clkmode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "SCT Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn cksel(&self) -> Cksel {
        let val = (self.0 >> 3usize) & 0x0f;
        Cksel::from_bits(val as u8)
    }
    #[doc = "SCT Clock Select."]
    #[inline(always)]
    pub const fn set_cksel(&mut self, val: Cksel) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
    #[doc = "No Reload Lower Match."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_l(&self) -> NoreloadL {
        let val = (self.0 >> 7usize) & 0x01;
        NoreloadL::from_bits(val as u8)
    }
    #[doc = "No Reload Lower Match."]
    #[inline(always)]
    pub const fn set_noreload_l(&mut self, val: NoreloadL) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "No Reload Higher Match."]
    #[must_use]
    #[inline(always)]
    pub const fn noreload_h(&self) -> NoreloadH {
        let val = (self.0 >> 8usize) & 0x01;
        NoreloadH::from_bits(val as u8)
    }
    #[doc = "No Reload Higher Match."]
    #[inline(always)]
    pub const fn set_noreload_h(&mut self, val: NoreloadH) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Input Synchronization."]
    #[must_use]
    #[inline(always)]
    pub const fn insync(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0xff;
        val as u8
    }
    #[doc = "Input Synchronization."]
    #[inline(always)]
    pub const fn set_insync(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 9usize)) | (((val as u32) & 0xff) << 9usize);
    }
    #[doc = "Auto Limit Lower."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_l(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Limit Lower."]
    #[inline(always)]
    pub const fn set_autolimit_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Auto Limit Higher."]
    #[must_use]
    #[inline(always)]
    pub const fn autolimit_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Auto Limit Higher."]
    #[inline(always)]
    pub const fn set_autolimit_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Config {
    #[inline(always)]
    fn default() -> Config {
        Config(0)
    }
}
impl core::fmt::Debug for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("unify", &self.unify())
            .field("clkmode", &self.clkmode())
            .field("cksel", &self.cksel())
            .field("noreload_l", &self.noreload_l())
            .field("noreload_h", &self.noreload_h())
            .field("insync", &self.insync())
            .field("autolimit_l", &self.autolimit_l())
            .field("autolimit_h", &self.autolimit_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Config {{ unify: {:?}, clkmode: {:?}, cksel: {:?}, noreload_l: {:?}, noreload_h: {:?}, insync: {=u8:?}, autolimit_l: {=bool:?}, autolimit_h: {=bool:?} }}",
            self.unify(),
            self.clkmode(),
            self.cksel(),
            self.noreload_l(),
            self.noreload_h(),
            self.insync(),
            self.autolimit_l(),
            self.autolimit_h()
        )
    }
}
#[doc = "Conflict Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conflag(pub u32);
impl Conflag {
    #[doc = "No Change Conflict Event Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ncflag(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "No Change Conflict Event Flag."]
    #[inline(always)]
    pub const fn set_ncflag(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Bus Error Low or Unified."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrl(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error Low or Unified."]
    #[inline(always)]
    pub const fn set_buserrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Bus Error High."]
    #[must_use]
    #[inline(always)]
    pub const fn buserrh(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Error High."]
    #[inline(always)]
    pub const fn set_buserrh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Conflag {
    #[inline(always)]
    fn default() -> Conflag {
        Conflag(0)
    }
}
impl core::fmt::Debug for Conflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conflag")
            .field("ncflag[0]", &self.ncflag(0usize))
            .field("ncflag[1]", &self.ncflag(1usize))
            .field("ncflag[2]", &self.ncflag(2usize))
            .field("ncflag[3]", &self.ncflag(3usize))
            .field("ncflag[4]", &self.ncflag(4usize))
            .field("ncflag[5]", &self.ncflag(5usize))
            .field("ncflag[6]", &self.ncflag(6usize))
            .field("ncflag[7]", &self.ncflag(7usize))
            .field("ncflag[8]", &self.ncflag(8usize))
            .field("ncflag[9]", &self.ncflag(9usize))
            .field("buserrl", &self.buserrl())
            .field("buserrh", &self.buserrh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Conflag {{ ncflag[0]: {=bool:?}, ncflag[1]: {=bool:?}, ncflag[2]: {=bool:?}, ncflag[3]: {=bool:?}, ncflag[4]: {=bool:?}, ncflag[5]: {=bool:?}, ncflag[6]: {=bool:?}, ncflag[7]: {=bool:?}, ncflag[8]: {=bool:?}, ncflag[9]: {=bool:?}, buserrl: {=bool:?}, buserrh: {=bool:?} }}",
            self.ncflag(0usize),
            self.ncflag(1usize),
            self.ncflag(2usize),
            self.ncflag(3usize),
            self.ncflag(4usize),
            self.ncflag(5usize),
            self.ncflag(6usize),
            self.ncflag(7usize),
            self.ncflag(8usize),
            self.ncflag(9usize),
            self.buserrl(),
            self.buserrh()
        )
    }
}
#[doc = "Counter Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[doc = "Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter Low."]
    #[inline(always)]
    pub const fn set_ctr_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn ctr_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Counter High."]
    #[inline(always)]
    pub const fn set_ctr_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("ctr_l", &self.ctr_l())
            .field("ctr_h", &self.ctr_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Count {{ ctr_l: {=u16:?}, ctr_h: {=u16:?} }}",
            self.ctr_l(),
            self.ctr_h()
        )
    }
}
#[doc = "SCT Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Down Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn down_l(&self) -> DownL {
        let val = (self.0 >> 0usize) & 0x01;
        DownL::from_bits(val as u8)
    }
    #[doc = "Down Counter Low."]
    #[inline(always)]
    pub const fn set_down_l(&mut self, val: DownL) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Stop Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Counter Low."]
    #[inline(always)]
    pub const fn set_stop_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Halt Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_l(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Halt Counter Low."]
    #[inline(always)]
    pub const fn set_halt_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_l(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Counter Low."]
    #[inline(always)]
    pub const fn set_clrctr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bidirectional Select Low."]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_l(&self) -> BidirL {
        let val = (self.0 >> 4usize) & 0x01;
        BidirL::from_bits(val as u8)
    }
    #[doc = "Bidirectional Select Low."]
    #[inline(always)]
    pub const fn set_bidir_l(&mut self, val: BidirL) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Prescaler for Low Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_l(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler for Low Counter."]
    #[inline(always)]
    pub const fn set_pre_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 5usize)) | (((val as u32) & 0xff) << 5usize);
    }
    #[doc = "Down Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn down_h(&self) -> DownH {
        let val = (self.0 >> 16usize) & 0x01;
        DownH::from_bits(val as u8)
    }
    #[doc = "Down Counter High."]
    #[inline(always)]
    pub const fn set_down_h(&mut self, val: DownH) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Stop Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn stop_h(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Counter High."]
    #[inline(always)]
    pub const fn set_stop_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Halt Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn halt_h(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Halt Counter High."]
    #[inline(always)]
    pub const fn set_halt_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Clear Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn clrctr_h(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Counter High."]
    #[inline(always)]
    pub const fn set_clrctr_h(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bidirectional Select High."]
    #[must_use]
    #[inline(always)]
    pub const fn bidir_h(&self) -> BidirH {
        let val = (self.0 >> 20usize) & 0x01;
        BidirH::from_bits(val as u8)
    }
    #[doc = "Bidirectional Select High."]
    #[inline(always)]
    pub const fn set_bidir_h(&mut self, val: BidirH) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Prescaler for High Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_h(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[doc = "Prescaler for High Counter."]
    #[inline(always)]
    pub const fn set_pre_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("down_l", &self.down_l())
            .field("stop_l", &self.stop_l())
            .field("halt_l", &self.halt_l())
            .field("clrctr_l", &self.clrctr_l())
            .field("bidir_l", &self.bidir_l())
            .field("pre_l", &self.pre_l())
            .field("down_h", &self.down_h())
            .field("stop_h", &self.stop_h())
            .field("halt_h", &self.halt_h())
            .field("clrctr_h", &self.clrctr_h())
            .field("bidir_h", &self.bidir_h())
            .field("pre_h", &self.pre_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ down_l: {:?}, stop_l: {=bool:?}, halt_l: {=bool:?}, clrctr_l: {=bool:?}, bidir_l: {:?}, pre_l: {=u8:?}, down_h: {:?}, stop_h: {=bool:?}, halt_h: {=bool:?}, clrctr_h: {=bool:?}, bidir_h: {:?}, pre_h: {=u8:?} }}",
            self.down_l(),
            self.stop_l(),
            self.halt_l(),
            self.clrctr_l(),
            self.bidir_l(),
            self.pre_l(),
            self.down_h(),
            self.stop_h(),
            self.halt_h(),
            self.clrctr_h(),
            self.bidir_h(),
            self.pre_h()
        )
    }
}
#[doc = "Dither Condition."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dither(pub u32);
impl Dither {
    #[doc = "Dither Low."]
    #[must_use]
    #[inline(always)]
    pub const fn dither_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Dither Low."]
    #[inline(always)]
    pub const fn set_dither_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Dither High."]
    #[must_use]
    #[inline(always)]
    pub const fn dither_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Dither High."]
    #[inline(always)]
    pub const fn set_dither_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dither {
    #[inline(always)]
    fn default() -> Dither {
        Dither(0)
    }
}
impl core::fmt::Debug for Dither {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dither")
            .field("dither_l", &self.dither_l())
            .field("dither_h", &self.dither_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dither {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dither {{ dither_l: {=u16:?}, dither_h: {=u16:?} }}",
            self.dither_l(),
            self.dither_h()
        )
    }
}
#[doc = "DMA Request 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq0(pub u32);
impl Dmareq0 {
    #[doc = "DMA Request Event."]
    #[must_use]
    #[inline(always)]
    pub const fn dev(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event."]
    #[inline(always)]
    pub const fn set_dev(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "DMA Request Low 0."]
    #[must_use]
    #[inline(always)]
    pub const fn drl0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Low 0."]
    #[inline(always)]
    pub const fn set_drl0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Request 0 State."]
    #[must_use]
    #[inline(always)]
    pub const fn drq0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request 0 State."]
    #[inline(always)]
    pub const fn set_drq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq0 {
    #[inline(always)]
    fn default() -> Dmareq0 {
        Dmareq0(0)
    }
}
impl core::fmt::Debug for Dmareq0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq0")
            .field("dev[0]", &self.dev(0usize))
            .field("dev[1]", &self.dev(1usize))
            .field("dev[2]", &self.dev(2usize))
            .field("dev[3]", &self.dev(3usize))
            .field("dev[4]", &self.dev(4usize))
            .field("dev[5]", &self.dev(5usize))
            .field("dev[6]", &self.dev(6usize))
            .field("dev[7]", &self.dev(7usize))
            .field("dev[8]", &self.dev(8usize))
            .field("dev[9]", &self.dev(9usize))
            .field("dev[10]", &self.dev(10usize))
            .field("dev[11]", &self.dev(11usize))
            .field("dev[12]", &self.dev(12usize))
            .field("dev[13]", &self.dev(13usize))
            .field("dev[14]", &self.dev(14usize))
            .field("dev[15]", &self.dev(15usize))
            .field("drl0", &self.drl0())
            .field("drq0", &self.drq0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq0 {{ dev[0]: {=bool:?}, dev[1]: {=bool:?}, dev[2]: {=bool:?}, dev[3]: {=bool:?}, dev[4]: {=bool:?}, dev[5]: {=bool:?}, dev[6]: {=bool:?}, dev[7]: {=bool:?}, dev[8]: {=bool:?}, dev[9]: {=bool:?}, dev[10]: {=bool:?}, dev[11]: {=bool:?}, dev[12]: {=bool:?}, dev[13]: {=bool:?}, dev[14]: {=bool:?}, dev[15]: {=bool:?}, drl0: {=bool:?}, drq0: {=bool:?} }}",
            self.dev(0usize),
            self.dev(1usize),
            self.dev(2usize),
            self.dev(3usize),
            self.dev(4usize),
            self.dev(5usize),
            self.dev(6usize),
            self.dev(7usize),
            self.dev(8usize),
            self.dev(9usize),
            self.dev(10usize),
            self.dev(11usize),
            self.dev(12usize),
            self.dev(13usize),
            self.dev(14usize),
            self.dev(15usize),
            self.drl0(),
            self.drq0()
        )
    }
}
#[doc = "DMA Request 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmareq1(pub u32);
impl Dmareq1 {
    #[doc = "DMA Request Event."]
    #[must_use]
    #[inline(always)]
    pub const fn dev(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Event."]
    #[inline(always)]
    pub const fn set_dev(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "DMA Request Low 1."]
    #[must_use]
    #[inline(always)]
    pub const fn drl1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request Low 1."]
    #[inline(always)]
    pub const fn set_drl1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA Request 1 State."]
    #[must_use]
    #[inline(always)]
    pub const fn drq1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Request 1 State."]
    #[inline(always)]
    pub const fn set_drq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmareq1 {
    #[inline(always)]
    fn default() -> Dmareq1 {
        Dmareq1(0)
    }
}
impl core::fmt::Debug for Dmareq1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmareq1")
            .field("dev[0]", &self.dev(0usize))
            .field("dev[1]", &self.dev(1usize))
            .field("dev[2]", &self.dev(2usize))
            .field("dev[3]", &self.dev(3usize))
            .field("dev[4]", &self.dev(4usize))
            .field("dev[5]", &self.dev(5usize))
            .field("dev[6]", &self.dev(6usize))
            .field("dev[7]", &self.dev(7usize))
            .field("dev[8]", &self.dev(8usize))
            .field("dev[9]", &self.dev(9usize))
            .field("dev[10]", &self.dev(10usize))
            .field("dev[11]", &self.dev(11usize))
            .field("dev[12]", &self.dev(12usize))
            .field("dev[13]", &self.dev(13usize))
            .field("dev[14]", &self.dev(14usize))
            .field("dev[15]", &self.dev(15usize))
            .field("drl1", &self.drl1())
            .field("drq1", &self.drq1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmareq1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmareq1 {{ dev[0]: {=bool:?}, dev[1]: {=bool:?}, dev[2]: {=bool:?}, dev[3]: {=bool:?}, dev[4]: {=bool:?}, dev[5]: {=bool:?}, dev[6]: {=bool:?}, dev[7]: {=bool:?}, dev[8]: {=bool:?}, dev[9]: {=bool:?}, dev[10]: {=bool:?}, dev[11]: {=bool:?}, dev[12]: {=bool:?}, dev[13]: {=bool:?}, dev[14]: {=bool:?}, dev[15]: {=bool:?}, drl1: {=bool:?}, drq1: {=bool:?} }}",
            self.dev(0usize),
            self.dev(1usize),
            self.dev(2usize),
            self.dev(3usize),
            self.dev(4usize),
            self.dev(5usize),
            self.dev(6usize),
            self.dev(7usize),
            self.dev(8usize),
            self.dev(9usize),
            self.dev(10usize),
            self.dev(11usize),
            self.dev(12usize),
            self.dev(13usize),
            self.dev(14usize),
            self.dev(15usize),
            self.drl1(),
            self.drq1()
        )
    }
}
#[doc = "Event n Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvCtrl(pub u32);
impl EvCtrl {
    #[doc = "Match Select."]
    #[must_use]
    #[inline(always)]
    pub const fn matchsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Match Select."]
    #[inline(always)]
    pub const fn set_matchsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "High Event."]
    #[must_use]
    #[inline(always)]
    pub const fn hevent(&self) -> Hevent {
        let val = (self.0 >> 4usize) & 0x01;
        Hevent::from_bits(val as u8)
    }
    #[doc = "High Event."]
    #[inline(always)]
    pub const fn set_hevent(&mut self, val: Hevent) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Input and Output Select."]
    #[must_use]
    #[inline(always)]
    pub const fn outsel(&self) -> Outsel {
        let val = (self.0 >> 5usize) & 0x01;
        Outsel::from_bits(val as u8)
    }
    #[doc = "Input and Output Select."]
    #[inline(always)]
    pub const fn set_outsel(&mut self, val: Outsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Input or Output Signal Select."]
    #[must_use]
    #[inline(always)]
    pub const fn iosel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Input or Output Signal Select."]
    #[inline(always)]
    pub const fn set_iosel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Input or Output Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn iocond(&self) -> Iocond {
        let val = (self.0 >> 10usize) & 0x03;
        Iocond::from_bits(val as u8)
    }
    #[doc = "Input or Output Condition."]
    #[inline(always)]
    pub const fn set_iocond(&mut self, val: Iocond) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Combination Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn combmode(&self) -> Combmode {
        let val = (self.0 >> 12usize) & 0x03;
        Combmode::from_bits(val as u8)
    }
    #[doc = "Combination Mode."]
    #[inline(always)]
    pub const fn set_combmode(&mut self, val: Combmode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "State Load."]
    #[must_use]
    #[inline(always)]
    pub const fn stateld(&self) -> Stateld {
        let val = (self.0 >> 14usize) & 0x01;
        Stateld::from_bits(val as u8)
    }
    #[doc = "State Load."]
    #[inline(always)]
    pub const fn set_stateld(&mut self, val: Stateld) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "State Value."]
    #[must_use]
    #[inline(always)]
    pub const fn statev(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "State Value."]
    #[inline(always)]
    pub const fn set_statev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "Match Mem."]
    #[must_use]
    #[inline(always)]
    pub const fn matchmem(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Match Mem."]
    #[inline(always)]
    pub const fn set_matchmem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn direction(&self) -> Direction {
        let val = (self.0 >> 21usize) & 0x03;
        Direction::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_direction(&mut self, val: Direction) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
}
impl Default for EvCtrl {
    #[inline(always)]
    fn default() -> EvCtrl {
        EvCtrl(0)
    }
}
impl core::fmt::Debug for EvCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvCtrl")
            .field("matchsel", &self.matchsel())
            .field("hevent", &self.hevent())
            .field("outsel", &self.outsel())
            .field("iosel", &self.iosel())
            .field("iocond", &self.iocond())
            .field("combmode", &self.combmode())
            .field("stateld", &self.stateld())
            .field("statev", &self.statev())
            .field("matchmem", &self.matchmem())
            .field("direction", &self.direction())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvCtrl {{ matchsel: {=u8:?}, hevent: {:?}, outsel: {:?}, iosel: {=u8:?}, iocond: {:?}, combmode: {:?}, stateld: {:?}, statev: {=u8:?}, matchmem: {=bool:?}, direction: {:?} }}",
            self.matchsel(),
            self.hevent(),
            self.outsel(),
            self.iosel(),
            self.iocond(),
            self.combmode(),
            self.stateld(),
            self.statev(),
            self.matchmem(),
            self.direction()
        )
    }
}
#[doc = "Event n State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvState(pub u32);
impl EvState {
    #[doc = "Event State Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn statems_kn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Event State Mask."]
    #[inline(always)]
    pub const fn set_statems_kn(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for EvState {
    #[inline(always)]
    fn default() -> EvState {
        EvState(0)
    }
}
impl core::fmt::Debug for EvState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvState")
            .field("statems_kn", &self.statems_kn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EvState {{ statems_kn: {=u32:?} }}", self.statems_kn())
    }
}
#[doc = "Event Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Even(pub u32);
impl Even {
    #[doc = "Event Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ien(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Event Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ien(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Even {
    #[inline(always)]
    fn default() -> Even {
        Even(0)
    }
}
impl core::fmt::Debug for Even {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Even")
            .field("ien[0]", &self.ien(0usize))
            .field("ien[1]", &self.ien(1usize))
            .field("ien[2]", &self.ien(2usize))
            .field("ien[3]", &self.ien(3usize))
            .field("ien[4]", &self.ien(4usize))
            .field("ien[5]", &self.ien(5usize))
            .field("ien[6]", &self.ien(6usize))
            .field("ien[7]", &self.ien(7usize))
            .field("ien[8]", &self.ien(8usize))
            .field("ien[9]", &self.ien(9usize))
            .field("ien[10]", &self.ien(10usize))
            .field("ien[11]", &self.ien(11usize))
            .field("ien[12]", &self.ien(12usize))
            .field("ien[13]", &self.ien(13usize))
            .field("ien[14]", &self.ien(14usize))
            .field("ien[15]", &self.ien(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Even {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Even {{ ien[0]: {=bool:?}, ien[1]: {=bool:?}, ien[2]: {=bool:?}, ien[3]: {=bool:?}, ien[4]: {=bool:?}, ien[5]: {=bool:?}, ien[6]: {=bool:?}, ien[7]: {=bool:?}, ien[8]: {=bool:?}, ien[9]: {=bool:?}, ien[10]: {=bool:?}, ien[11]: {=bool:?}, ien[12]: {=bool:?}, ien[13]: {=bool:?}, ien[14]: {=bool:?}, ien[15]: {=bool:?} }}",
            self.ien(0usize),
            self.ien(1usize),
            self.ien(2usize),
            self.ien(3usize),
            self.ien(4usize),
            self.ien(5usize),
            self.ien(6usize),
            self.ien(7usize),
            self.ien(8usize),
            self.ien(9usize),
            self.ien(10usize),
            self.ien(11usize),
            self.ien(12usize),
            self.ien(13usize),
            self.ien(14usize),
            self.ien(15usize)
        )
    }
}
#[doc = "Event Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflag(pub u32);
impl Evflag {
    #[doc = "Event Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn flag(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Event Flag."]
    #[inline(always)]
    pub const fn set_flag(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Evflag {
    #[inline(always)]
    fn default() -> Evflag {
        Evflag(0)
    }
}
impl core::fmt::Debug for Evflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Evflag")
            .field("flag[0]", &self.flag(0usize))
            .field("flag[1]", &self.flag(1usize))
            .field("flag[2]", &self.flag(2usize))
            .field("flag[3]", &self.flag(3usize))
            .field("flag[4]", &self.flag(4usize))
            .field("flag[5]", &self.flag(5usize))
            .field("flag[6]", &self.flag(6usize))
            .field("flag[7]", &self.flag(7usize))
            .field("flag[8]", &self.flag(8usize))
            .field("flag[9]", &self.flag(9usize))
            .field("flag[10]", &self.flag(10usize))
            .field("flag[11]", &self.flag(11usize))
            .field("flag[12]", &self.flag(12usize))
            .field("flag[13]", &self.flag(13usize))
            .field("flag[14]", &self.flag(14usize))
            .field("flag[15]", &self.flag(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Evflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Evflag {{ flag[0]: {=bool:?}, flag[1]: {=bool:?}, flag[2]: {=bool:?}, flag[3]: {=bool:?}, flag[4]: {=bool:?}, flag[5]: {=bool:?}, flag[6]: {=bool:?}, flag[7]: {=bool:?}, flag[8]: {=bool:?}, flag[9]: {=bool:?}, flag[10]: {=bool:?}, flag[11]: {=bool:?}, flag[12]: {=bool:?}, flag[13]: {=bool:?}, flag[14]: {=bool:?}, flag[15]: {=bool:?} }}",
            self.flag(0usize),
            self.flag(1usize),
            self.flag(2usize),
            self.flag(3usize),
            self.flag(4usize),
            self.flag(5usize),
            self.flag(6usize),
            self.flag(7usize),
            self.flag(8usize),
            self.flag(9usize),
            self.flag(10usize),
            self.flag(11usize),
            self.flag(12usize),
            self.flag(13usize),
            self.flag(14usize),
            self.flag(15usize)
        )
    }
}
#[doc = "Fractional Match."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fracmat(pub u32);
impl Fracmat {
    #[doc = "Fractional Match Low."]
    #[must_use]
    #[inline(always)]
    pub const fn fracmat_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional Match Low."]
    #[inline(always)]
    pub const fn set_fracmat_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Fractional Match High."]
    #[must_use]
    #[inline(always)]
    pub const fn fracmat_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Fractional Match High."]
    #[inline(always)]
    pub const fn set_fracmat_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fracmat {
    #[inline(always)]
    fn default() -> Fracmat {
        Fracmat(0)
    }
}
impl core::fmt::Debug for Fracmat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fracmat")
            .field("fracmat_l", &self.fracmat_l())
            .field("fracmat_h", &self.fracmat_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fracmat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fracmat {{ fracmat_l: {=u8:?}, fracmat_h: {=u8:?} }}",
            self.fracmat_l(),
            self.fracmat_h()
        )
    }
}
#[doc = "Fractional Match Reload."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fracmatrel(pub u32);
impl Fracmatrel {
    #[doc = "Reload Fractional Match Low."]
    #[must_use]
    #[inline(always)]
    pub const fn relfrac_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Reload Fractional Match Low."]
    #[inline(always)]
    pub const fn set_relfrac_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reload Fractional Match High."]
    #[must_use]
    #[inline(always)]
    pub const fn relfrac_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Reload Fractional Match High."]
    #[inline(always)]
    pub const fn set_relfrac_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fracmatrel {
    #[inline(always)]
    fn default() -> Fracmatrel {
        Fracmatrel(0)
    }
}
impl core::fmt::Debug for Fracmatrel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fracmatrel")
            .field("relfrac_l", &self.relfrac_l())
            .field("relfrac_h", &self.relfrac_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fracmatrel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fracmatrel {{ relfrac_l: {=u8:?}, relfrac_h: {=u8:?} }}",
            self.relfrac_l(),
            self.relfrac_h()
        )
    }
}
#[doc = "Halt Event Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Halt(pub u32);
impl Halt {
    #[doc = "Halt Event Low."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Halt Event Low."]
    #[inline(always)]
    pub const fn set_haltmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Halt Event High."]
    #[must_use]
    #[inline(always)]
    pub const fn haltmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Halt Event High."]
    #[inline(always)]
    pub const fn set_haltmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Halt {
    #[inline(always)]
    fn default() -> Halt {
        Halt(0)
    }
}
impl core::fmt::Debug for Halt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Halt")
            .field("haltmsk_l", &self.haltmsk_l())
            .field("haltmsk_h", &self.haltmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Halt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Halt {{ haltmsk_l: {=u16:?}, haltmsk_h: {=u16:?} }}",
            self.haltmsk_l(),
            self.haltmsk_h()
        )
    }
}
#[doc = "Input State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Input(pub u32);
impl Input {
    #[doc = "Input state."]
    #[must_use]
    #[inline(always)]
    pub const fn ain(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Input state."]
    #[inline(always)]
    pub const fn set_ain(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Input state."]
    #[must_use]
    #[inline(always)]
    pub const fn sin(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Input state."]
    #[inline(always)]
    pub const fn set_sin(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Input {
    #[inline(always)]
    fn default() -> Input {
        Input(0)
    }
}
impl core::fmt::Debug for Input {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Input")
            .field("ain[0]", &self.ain(0usize))
            .field("ain[1]", &self.ain(1usize))
            .field("ain[2]", &self.ain(2usize))
            .field("ain[3]", &self.ain(3usize))
            .field("ain[4]", &self.ain(4usize))
            .field("ain[5]", &self.ain(5usize))
            .field("ain[6]", &self.ain(6usize))
            .field("ain[7]", &self.ain(7usize))
            .field("ain[8]", &self.ain(8usize))
            .field("ain[9]", &self.ain(9usize))
            .field("ain[10]", &self.ain(10usize))
            .field("ain[11]", &self.ain(11usize))
            .field("ain[12]", &self.ain(12usize))
            .field("ain[13]", &self.ain(13usize))
            .field("ain[14]", &self.ain(14usize))
            .field("ain[15]", &self.ain(15usize))
            .field("sin[0]", &self.sin(0usize))
            .field("sin[1]", &self.sin(1usize))
            .field("sin[2]", &self.sin(2usize))
            .field("sin[3]", &self.sin(3usize))
            .field("sin[4]", &self.sin(4usize))
            .field("sin[5]", &self.sin(5usize))
            .field("sin[6]", &self.sin(6usize))
            .field("sin[7]", &self.sin(7usize))
            .field("sin[8]", &self.sin(8usize))
            .field("sin[9]", &self.sin(9usize))
            .field("sin[10]", &self.sin(10usize))
            .field("sin[11]", &self.sin(11usize))
            .field("sin[12]", &self.sin(12usize))
            .field("sin[13]", &self.sin(13usize))
            .field("sin[14]", &self.sin(14usize))
            .field("sin[15]", &self.sin(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Input {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Input {{ ain[0]: {=bool:?}, ain[1]: {=bool:?}, ain[2]: {=bool:?}, ain[3]: {=bool:?}, ain[4]: {=bool:?}, ain[5]: {=bool:?}, ain[6]: {=bool:?}, ain[7]: {=bool:?}, ain[8]: {=bool:?}, ain[9]: {=bool:?}, ain[10]: {=bool:?}, ain[11]: {=bool:?}, ain[12]: {=bool:?}, ain[13]: {=bool:?}, ain[14]: {=bool:?}, ain[15]: {=bool:?}, sin[0]: {=bool:?}, sin[1]: {=bool:?}, sin[2]: {=bool:?}, sin[3]: {=bool:?}, sin[4]: {=bool:?}, sin[5]: {=bool:?}, sin[6]: {=bool:?}, sin[7]: {=bool:?}, sin[8]: {=bool:?}, sin[9]: {=bool:?}, sin[10]: {=bool:?}, sin[11]: {=bool:?}, sin[12]: {=bool:?}, sin[13]: {=bool:?}, sin[14]: {=bool:?}, sin[15]: {=bool:?} }}",
            self.ain(0usize),
            self.ain(1usize),
            self.ain(2usize),
            self.ain(3usize),
            self.ain(4usize),
            self.ain(5usize),
            self.ain(6usize),
            self.ain(7usize),
            self.ain(8usize),
            self.ain(9usize),
            self.ain(10usize),
            self.ain(11usize),
            self.ain(12usize),
            self.ain(13usize),
            self.ain(14usize),
            self.ain(15usize),
            self.sin(0usize),
            self.sin(1usize),
            self.sin(2usize),
            self.sin(3usize),
            self.sin(4usize),
            self.sin(5usize),
            self.sin(6usize),
            self.sin(7usize),
            self.sin(8usize),
            self.sin(9usize),
            self.sin(10usize),
            self.sin(11usize),
            self.sin(12usize),
            self.sin(13usize),
            self.sin(14usize),
            self.sin(15usize)
        )
    }
}
#[doc = "SCT Limit Event Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Limit(pub u32);
impl Limit {
    #[doc = "Limit Event Counter Low."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Limit Event Counter Low."]
    #[inline(always)]
    pub const fn set_limmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Limit Event Counter High."]
    #[must_use]
    #[inline(always)]
    pub const fn limmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Limit Event Counter High."]
    #[inline(always)]
    pub const fn set_limmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Limit {
    #[inline(always)]
    fn default() -> Limit {
        Limit(0)
    }
}
impl core::fmt::Debug for Limit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Limit")
            .field("limmsk_l", &self.limmsk_l())
            .field("limmsk_h", &self.limmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Limit {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Limit {{ limmsk_l: {=u16:?}, limmsk_h: {=u16:?} }}",
            self.limmsk_l(),
            self.limmsk_h()
        )
    }
}
#[doc = "Match Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match(pub u32);
impl Match {
    #[doc = "Match Low."]
    #[must_use]
    #[inline(always)]
    pub const fn matc_hn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Match Low."]
    #[inline(always)]
    pub const fn set_matc_hn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Match High."]
    #[must_use]
    #[inline(always)]
    pub const fn matc_hn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Match High."]
    #[inline(always)]
    pub const fn set_matc_hn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0)
    }
}
impl core::fmt::Debug for Match {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("matc_hn_l", &self.matc_hn_l())
            .field("matc_hn_h", &self.matc_hn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Match {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Match {{ matc_hn_l: {=u16:?}, matc_hn_h: {=u16:?} }}",
            self.matc_hn_l(),
            self.matc_hn_h()
        )
    }
}
#[doc = "Match Reload Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Matchrel(pub u32);
impl Matchrel {
    #[doc = "Reload Low."]
    #[must_use]
    #[inline(always)]
    pub const fn reloa_dn_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload Low."]
    #[inline(always)]
    pub const fn set_reloa_dn_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Reload High."]
    #[must_use]
    #[inline(always)]
    pub const fn reloa_dn_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Reload High."]
    #[inline(always)]
    pub const fn set_reloa_dn_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Matchrel {
    #[inline(always)]
    fn default() -> Matchrel {
        Matchrel(0)
    }
}
impl core::fmt::Debug for Matchrel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Matchrel")
            .field("reloa_dn_l", &self.reloa_dn_l())
            .field("reloa_dn_h", &self.reloa_dn_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Matchrel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Matchrel {{ reloa_dn_l: {=u16:?}, reloa_dn_h: {=u16:?} }}",
            self.reloa_dn_l(),
            self.reloa_dn_h()
        )
    }
}
#[doc = "Output n Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutClr(pub u32);
impl OutClr {
    #[doc = "Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Clear Output."]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OutClr {
    #[inline(always)]
    fn default() -> OutClr {
        OutClr(0)
    }
}
impl core::fmt::Debug for OutClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutClr").field("clr", &self.clr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutClr {{ clr: {=u16:?} }}", self.clr())
    }
}
#[doc = "Output n Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OutSet(pub u32);
impl OutSet {
    #[doc = "Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set Output."]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OutSet {
    #[inline(always)]
    fn default() -> OutSet {
        OutSet(0)
    }
}
impl core::fmt::Debug for OutSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OutSet").field("set", &self.set()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OutSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OutSet {{ set: {=u16:?} }}", self.set())
    }
}
#[doc = "Output State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Output(pub u32);
impl Output {
    #[doc = "Output Low and High."]
    #[must_use]
    #[inline(always)]
    pub const fn out(&self, n: usize) -> OutVal {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        OutVal::from_bits(val as u8)
    }
    #[doc = "Output Low and High."]
    #[inline(always)]
    pub const fn set_out(&mut self, n: usize, val: OutVal) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Output {
    #[inline(always)]
    fn default() -> Output {
        Output(0)
    }
}
impl core::fmt::Debug for Output {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Output")
            .field("out[0]", &self.out(0usize))
            .field("out[1]", &self.out(1usize))
            .field("out[2]", &self.out(2usize))
            .field("out[3]", &self.out(3usize))
            .field("out[4]", &self.out(4usize))
            .field("out[5]", &self.out(5usize))
            .field("out[6]", &self.out(6usize))
            .field("out[7]", &self.out(7usize))
            .field("out[8]", &self.out(8usize))
            .field("out[9]", &self.out(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Output {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Output {{ out[0]: {:?}, out[1]: {:?}, out[2]: {:?}, out[3]: {:?}, out[4]: {:?}, out[5]: {:?}, out[6]: {:?}, out[7]: {:?}, out[8]: {:?}, out[9]: {:?} }}",
            self.out(0usize),
            self.out(1usize),
            self.out(2usize),
            self.out(3usize),
            self.out(4usize),
            self.out(5usize),
            self.out(6usize),
            self.out(7usize),
            self.out(8usize),
            self.out(9usize)
        )
    }
}
#[doc = "Output Counter Direction Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outputdirctrl(pub u32);
impl Outputdirctrl {
    #[doc = "Set and Clear Operation on Output."]
    #[must_use]
    #[inline(always)]
    pub const fn setclr(&self, n: usize) -> Setclr {
        assert!(n < 10usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Setclr::from_bits(val as u8)
    }
    #[doc = "Set and Clear Operation on Output."]
    #[inline(always)]
    pub const fn set_setclr(&mut self, n: usize, val: Setclr) {
        assert!(n < 10usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Outputdirctrl {
    #[inline(always)]
    fn default() -> Outputdirctrl {
        Outputdirctrl(0)
    }
}
impl core::fmt::Debug for Outputdirctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Outputdirctrl")
            .field("setclr[0]", &self.setclr(0usize))
            .field("setclr[1]", &self.setclr(1usize))
            .field("setclr[2]", &self.setclr(2usize))
            .field("setclr[3]", &self.setclr(3usize))
            .field("setclr[4]", &self.setclr(4usize))
            .field("setclr[5]", &self.setclr(5usize))
            .field("setclr[6]", &self.setclr(6usize))
            .field("setclr[7]", &self.setclr(7usize))
            .field("setclr[8]", &self.setclr(8usize))
            .field("setclr[9]", &self.setclr(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Outputdirctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Outputdirctrl {{ setclr[0]: {:?}, setclr[1]: {:?}, setclr[2]: {:?}, setclr[3]: {:?}, setclr[4]: {:?}, setclr[5]: {:?}, setclr[6]: {:?}, setclr[7]: {:?}, setclr[8]: {:?}, setclr[9]: {:?} }}",
            self.setclr(0usize),
            self.setclr(1usize),
            self.setclr(2usize),
            self.setclr(3usize),
            self.setclr(4usize),
            self.setclr(5usize),
            self.setclr(6usize),
            self.setclr(7usize),
            self.setclr(8usize),
            self.setclr(9usize)
        )
    }
}
#[doc = "Match and Capture Register Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regmode(pub u32);
impl Regmode {
    #[doc = "Register Mode Low."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_l(&self, n: usize) -> RegmodL {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        RegmodL::from_bits(val as u8)
    }
    #[doc = "Register Mode Low."]
    #[inline(always)]
    pub const fn set_regmod_l(&mut self, n: usize, val: RegmodL) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Register Mode High."]
    #[must_use]
    #[inline(always)]
    pub const fn regmod_h(&self, n: usize) -> RegmodH {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        RegmodH::from_bits(val as u8)
    }
    #[doc = "Register Mode High."]
    #[inline(always)]
    pub const fn set_regmod_h(&mut self, n: usize, val: RegmodH) {
        assert!(n < 16usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Regmode {
    #[inline(always)]
    fn default() -> Regmode {
        Regmode(0)
    }
}
impl core::fmt::Debug for Regmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Regmode")
            .field("regmod_l[0]", &self.regmod_l(0usize))
            .field("regmod_l[1]", &self.regmod_l(1usize))
            .field("regmod_l[2]", &self.regmod_l(2usize))
            .field("regmod_l[3]", &self.regmod_l(3usize))
            .field("regmod_l[4]", &self.regmod_l(4usize))
            .field("regmod_l[5]", &self.regmod_l(5usize))
            .field("regmod_l[6]", &self.regmod_l(6usize))
            .field("regmod_l[7]", &self.regmod_l(7usize))
            .field("regmod_l[8]", &self.regmod_l(8usize))
            .field("regmod_l[9]", &self.regmod_l(9usize))
            .field("regmod_l[10]", &self.regmod_l(10usize))
            .field("regmod_l[11]", &self.regmod_l(11usize))
            .field("regmod_l[12]", &self.regmod_l(12usize))
            .field("regmod_l[13]", &self.regmod_l(13usize))
            .field("regmod_l[14]", &self.regmod_l(14usize))
            .field("regmod_l[15]", &self.regmod_l(15usize))
            .field("regmod_h[0]", &self.regmod_h(0usize))
            .field("regmod_h[1]", &self.regmod_h(1usize))
            .field("regmod_h[2]", &self.regmod_h(2usize))
            .field("regmod_h[3]", &self.regmod_h(3usize))
            .field("regmod_h[4]", &self.regmod_h(4usize))
            .field("regmod_h[5]", &self.regmod_h(5usize))
            .field("regmod_h[6]", &self.regmod_h(6usize))
            .field("regmod_h[7]", &self.regmod_h(7usize))
            .field("regmod_h[8]", &self.regmod_h(8usize))
            .field("regmod_h[9]", &self.regmod_h(9usize))
            .field("regmod_h[10]", &self.regmod_h(10usize))
            .field("regmod_h[11]", &self.regmod_h(11usize))
            .field("regmod_h[12]", &self.regmod_h(12usize))
            .field("regmod_h[13]", &self.regmod_h(13usize))
            .field("regmod_h[14]", &self.regmod_h(14usize))
            .field("regmod_h[15]", &self.regmod_h(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Regmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Regmode {{ regmod_l[0]: {:?}, regmod_l[1]: {:?}, regmod_l[2]: {:?}, regmod_l[3]: {:?}, regmod_l[4]: {:?}, regmod_l[5]: {:?}, regmod_l[6]: {:?}, regmod_l[7]: {:?}, regmod_l[8]: {:?}, regmod_l[9]: {:?}, regmod_l[10]: {:?}, regmod_l[11]: {:?}, regmod_l[12]: {:?}, regmod_l[13]: {:?}, regmod_l[14]: {:?}, regmod_l[15]: {:?}, regmod_h[0]: {:?}, regmod_h[1]: {:?}, regmod_h[2]: {:?}, regmod_h[3]: {:?}, regmod_h[4]: {:?}, regmod_h[5]: {:?}, regmod_h[6]: {:?}, regmod_h[7]: {:?}, regmod_h[8]: {:?}, regmod_h[9]: {:?}, regmod_h[10]: {:?}, regmod_h[11]: {:?}, regmod_h[12]: {:?}, regmod_h[13]: {:?}, regmod_h[14]: {:?}, regmod_h[15]: {:?} }}",
            self.regmod_l(0usize),
            self.regmod_l(1usize),
            self.regmod_l(2usize),
            self.regmod_l(3usize),
            self.regmod_l(4usize),
            self.regmod_l(5usize),
            self.regmod_l(6usize),
            self.regmod_l(7usize),
            self.regmod_l(8usize),
            self.regmod_l(9usize),
            self.regmod_l(10usize),
            self.regmod_l(11usize),
            self.regmod_l(12usize),
            self.regmod_l(13usize),
            self.regmod_l(14usize),
            self.regmod_l(15usize),
            self.regmod_h(0usize),
            self.regmod_h(1usize),
            self.regmod_h(2usize),
            self.regmod_h(3usize),
            self.regmod_h(4usize),
            self.regmod_h(5usize),
            self.regmod_h(6usize),
            self.regmod_h(7usize),
            self.regmod_h(8usize),
            self.regmod_h(9usize),
            self.regmod_h(10usize),
            self.regmod_h(11usize),
            self.regmod_h(12usize),
            self.regmod_h(13usize),
            self.regmod_h(14usize),
            self.regmod_h(15usize)
        )
    }
}
#[doc = "Output Conflict Resolution."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Res(pub u32);
impl Res {
    #[doc = "Output Resolution."]
    #[must_use]
    #[inline(always)]
    pub const fn ores(&self, n: usize) -> Ores {
        assert!(n < 10usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Ores::from_bits(val as u8)
    }
    #[doc = "Output Resolution."]
    #[inline(always)]
    pub const fn set_ores(&mut self, n: usize, val: Ores) {
        assert!(n < 10usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Res {
    #[inline(always)]
    fn default() -> Res {
        Res(0)
    }
}
impl core::fmt::Debug for Res {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Res")
            .field("ores[0]", &self.ores(0usize))
            .field("ores[1]", &self.ores(1usize))
            .field("ores[2]", &self.ores(2usize))
            .field("ores[3]", &self.ores(3usize))
            .field("ores[4]", &self.ores(4usize))
            .field("ores[5]", &self.ores(5usize))
            .field("ores[6]", &self.ores(6usize))
            .field("ores[7]", &self.ores(7usize))
            .field("ores[8]", &self.ores(8usize))
            .field("ores[9]", &self.ores(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Res {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Res {{ ores[0]: {:?}, ores[1]: {:?}, ores[2]: {:?}, ores[3]: {:?}, ores[4]: {:?}, ores[5]: {:?}, ores[6]: {:?}, ores[7]: {:?}, ores[8]: {:?}, ores[9]: {:?} }}",
            self.ores(0usize),
            self.ores(1usize),
            self.ores(2usize),
            self.ores(3usize),
            self.ores(4usize),
            self.ores(5usize),
            self.ores(6usize),
            self.ores(7usize),
            self.ores(8usize),
            self.ores(9usize)
        )
    }
}
#[doc = "Start Event Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Start(pub u32);
impl Start {
    #[doc = "Start Event Low."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start Event Low."]
    #[inline(always)]
    pub const fn set_startmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start Event High."]
    #[must_use]
    #[inline(always)]
    pub const fn startmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start Event High."]
    #[inline(always)]
    pub const fn set_startmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Start {
    #[inline(always)]
    fn default() -> Start {
        Start(0)
    }
}
impl core::fmt::Debug for Start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Start")
            .field("startmsk_l", &self.startmsk_l())
            .field("startmsk_h", &self.startmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Start {{ startmsk_l: {=u16:?}, startmsk_h: {=u16:?} }}",
            self.startmsk_l(),
            self.startmsk_h()
        )
    }
}
#[doc = "State Variable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[doc = "State Variable Low."]
    #[must_use]
    #[inline(always)]
    pub const fn state_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "State Variable Low."]
    #[inline(always)]
    pub const fn set_state_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "State Variable High."]
    #[must_use]
    #[inline(always)]
    pub const fn state_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "State Variable High."]
    #[inline(always)]
    pub const fn set_state_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("State")
            .field("state_l", &self.state_l())
            .field("state_h", &self.state_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for State {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "State {{ state_l: {=u8:?}, state_h: {=u8:?} }}",
            self.state_l(),
            self.state_h()
        )
    }
}
#[doc = "Stop Event Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stop(pub u32);
impl Stop {
    #[doc = "Stop Event Low."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop Event Low."]
    #[inline(always)]
    pub const fn set_stopmsk_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Stop Event High."]
    #[must_use]
    #[inline(always)]
    pub const fn stopmsk_h(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop Event High."]
    #[inline(always)]
    pub const fn set_stopmsk_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Stop {
    #[inline(always)]
    fn default() -> Stop {
        Stop(0)
    }
}
impl core::fmt::Debug for Stop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stop")
            .field("stopmsk_l", &self.stopmsk_l())
            .field("stopmsk_h", &self.stopmsk_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stop {{ stopmsk_l: {=u16:?}, stopmsk_h: {=u16:?} }}",
            self.stopmsk_l(),
            self.stopmsk_h()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BidirH {
    #[doc = "Up."]
    Up = 0x0,
    #[doc = "Up-down."]
    UpDown = 0x01,
}
impl BidirH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BidirH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BidirH {
    #[inline(always)]
    fn from(val: u8) -> BidirH {
        BidirH::from_bits(val)
    }
}
impl From<BidirH> for u8 {
    #[inline(always)]
    fn from(val: BidirH) -> u8 {
        BidirH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BidirL {
    #[doc = "Up."]
    Up = 0x0,
    #[doc = "Up-down."]
    UpDown = 0x01,
}
impl BidirL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BidirL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BidirL {
    #[inline(always)]
    fn from(val: u8) -> BidirL {
        BidirL::from_bits(val)
    }
}
impl From<BidirL> for u8 {
    #[inline(always)]
    fn from(val: BidirL) -> u8 {
        BidirL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cksel {
    #[doc = "Rising edges on input 0."]
    Input0RisingEdges = 0x0,
    #[doc = "Falling edges on input 0."]
    Input0FallingEdges = 0x01,
    #[doc = "Rising edges on input 1."]
    Input1RisingEdges = 0x02,
    #[doc = "Falling edges on input 1."]
    Input1FallingEdges = 0x03,
    #[doc = "Rising edges on input 2."]
    Input2RisingEdges = 0x04,
    #[doc = "Falling edges on input 2."]
    Input2FallingEdges = 0x05,
    #[doc = "Rising edges on input 3."]
    Input3RisingEdges = 0x06,
    #[doc = "Falling edges on input 3."]
    Input3FallingEdges = 0x07,
    #[doc = "Rising edges on input 4."]
    Input4RisingEdges = 0x08,
    #[doc = "Falling edges on input 4."]
    Input4FallingEdges = 0x09,
    #[doc = "Rising edges on input 5."]
    Input5RisingEdges = 0x0a,
    #[doc = "Falling edges on input 5."]
    Input5FallingEdges = 0x0b,
    #[doc = "Rising edges on input 6."]
    Input6RisingEdges = 0x0c,
    #[doc = "Falling edges on input 6."]
    Input6FallingEdges = 0x0d,
    #[doc = "Rising edges on input 7."]
    Input7RisingEdges = 0x0e,
    #[doc = "Falling edges on input 7."]
    Input7FallingEdges = 0x0f,
}
impl Cksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cksel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cksel {
    #[inline(always)]
    fn from(val: u8) -> Cksel {
        Cksel::from_bits(val)
    }
}
impl From<Cksel> for u8 {
    #[inline(always)]
    fn from(val: Cksel) -> u8 {
        Cksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkmode {
    #[doc = "System Clock mode."]
    SystemClockMode = 0x0,
    #[doc = "Sampled System Clock mode."]
    SampledSystemClockMode = 0x01,
    #[doc = "SCT Input Clock mode."]
    SctInputClockMode = 0x02,
    #[doc = "Asynchronous mode."]
    AsynchronousMode = 0x03,
}
impl Clkmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkmode {
    #[inline(always)]
    fn from(val: u8) -> Clkmode {
        Clkmode::from_bits(val)
    }
}
impl From<Clkmode> for u8 {
    #[inline(always)]
    fn from(val: Clkmode) -> u8 {
        Clkmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Combmode {
    #[doc = "OR (the event occurs when either the specified match or I/O condition occurs)."]
    Or = 0x0,
    #[doc = "MATCH (uses the specified match only)."]
    Match = 0x01,
    #[doc = "IO (uses the specified I/O condition only)."]
    Io = 0x02,
    #[doc = "AND (the event occurs when the specified match and I/O condition occur simultaneously)."]
    And = 0x03,
}
impl Combmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Combmode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Combmode {
    #[inline(always)]
    fn from(val: u8) -> Combmode {
        Combmode::from_bits(val)
    }
}
impl From<Combmode> for u8 {
    #[inline(always)]
    fn from(val: Combmode) -> u8 {
        Combmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Direction {
    #[doc = "Direction independent (event triggered regardless of the count direction)."]
    DirectionIndependent = 0x0,
    #[doc = "Counting up (event triggered only during up-counting when CTRL\\[BIDIR\\] = 1)."]
    CountingUp = 0x01,
    #[doc = "Counting down (event triggered only during down-counting when CTRL\\[BIDIR\\] = 1)."]
    CountingDown = 0x02,
    _RESERVED_3 = 0x03,
}
impl Direction {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Direction {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Direction {
    #[inline(always)]
    fn from(val: u8) -> Direction {
        Direction::from_bits(val)
    }
}
impl From<Direction> for u8 {
    #[inline(always)]
    fn from(val: Direction) -> u8 {
        Direction::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DownH {
    #[doc = "Up."]
    Up = 0x0,
    #[doc = "Down."]
    Down = 0x01,
}
impl DownH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DownH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DownH {
    #[inline(always)]
    fn from(val: u8) -> DownH {
        DownH::from_bits(val)
    }
}
impl From<DownH> for u8 {
    #[inline(always)]
    fn from(val: DownH) -> u8 {
        DownH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DownL {
    #[doc = "Up."]
    Up = 0x0,
    #[doc = "Down."]
    Down = 0x01,
}
impl DownL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DownL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DownL {
    #[inline(always)]
    fn from(val: u8) -> DownL {
        DownL::from_bits(val)
    }
}
impl From<DownL> for u8 {
    #[inline(always)]
    fn from(val: DownL) -> u8 {
        DownL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hevent {
    #[doc = "Low counter (selects the L state and the L match register that the MATCHSEL field specifies)."]
    LCounter = 0x0,
    #[doc = "High counter (selects the H state and the H match register that the MATCHSEL field specifies)."]
    HCounter = 0x01,
}
impl Hevent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hevent {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hevent {
    #[inline(always)]
    fn from(val: u8) -> Hevent {
        Hevent::from_bits(val)
    }
}
impl From<Hevent> for u8 {
    #[inline(always)]
    fn from(val: Hevent) -> u8 {
        Hevent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iocond {
    #[doc = "Low."]
    Low = 0x0,
    #[doc = "Rise."]
    Rise = 0x01,
    #[doc = "Fall."]
    Fall = 0x02,
    #[doc = "High."]
    High = 0x03,
}
impl Iocond {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iocond {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iocond {
    #[inline(always)]
    fn from(val: u8) -> Iocond {
        Iocond::from_bits(val)
    }
}
impl From<Iocond> for u8 {
    #[inline(always)]
    fn from(val: Iocond) -> u8 {
        Iocond::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoreloadH {
    #[doc = "Reloaded."]
    ReloadH = 0x0,
    #[doc = "Not reloaded."]
    NoReloadH = 0x01,
}
impl NoreloadH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoreloadH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoreloadH {
    #[inline(always)]
    fn from(val: u8) -> NoreloadH {
        NoreloadH::from_bits(val)
    }
}
impl From<NoreloadH> for u8 {
    #[inline(always)]
    fn from(val: NoreloadH) -> u8 {
        NoreloadH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NoreloadL {
    #[doc = "Reloaded."]
    Reload = 0x0,
    #[doc = "Not reloaded."]
    NoReload = 0x01,
}
impl NoreloadL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NoreloadL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NoreloadL {
    #[inline(always)]
    fn from(val: u8) -> NoreloadL {
        NoreloadL::from_bits(val)
    }
}
impl From<NoreloadL> for u8 {
    #[inline(always)]
    fn from(val: NoreloadL) -> u8 {
        NoreloadL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ores {
    #[doc = "No change."]
    NoChange = 0x0,
    #[doc = "Set output (or clear, based on OUTPUTDIRCTRL\\[SETCLRn\\])."]
    Set = 0x01,
    #[doc = "Clear output (or set, based on OUTPUTDIRCTRL\\[SETCLRn\\])."]
    Clear = 0x02,
    #[doc = "Toggle output."]
    ToggleOutput = 0x03,
}
impl Ores {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ores {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ores {
    #[inline(always)]
    fn from(val: u8) -> Ores {
        Ores::from_bits(val)
    }
}
impl From<Ores> for u8 {
    #[inline(always)]
    fn from(val: Ores) -> u8 {
        Ores::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutVal {
    #[doc = "Forces the corresponding output low."]
    Low = 0x0,
    #[doc = "Forces the corresponding output high."]
    High = 0x01,
}
impl OutVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutVal {
    #[inline(always)]
    fn from(val: u8) -> OutVal {
        OutVal::from_bits(val)
    }
}
impl From<OutVal> for u8 {
    #[inline(always)]
    fn from(val: OutVal) -> u8 {
        OutVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Outsel {
    #[doc = "Inputs."]
    Input = 0x0,
    #[doc = "Outputs."]
    Output = 0x01,
}
impl Outsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outsel {
    #[inline(always)]
    fn from(val: u8) -> Outsel {
        Outsel::from_bits(val)
    }
}
impl From<Outsel> for u8 {
    #[inline(always)]
    fn from(val: Outsel) -> u8 {
        Outsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodH {
    #[doc = "Match."]
    Match = 0x0,
    #[doc = "Capture."]
    Capture = 0x01,
}
impl RegmodH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodH {
    #[inline(always)]
    fn from(val: u8) -> RegmodH {
        RegmodH::from_bits(val)
    }
}
impl From<RegmodH> for u8 {
    #[inline(always)]
    fn from(val: RegmodH) -> u8 {
        RegmodH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegmodL {
    #[doc = "Match."]
    Match = 0x0,
    #[doc = "Capture."]
    Capture = 0x01,
}
impl RegmodL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegmodL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegmodL {
    #[inline(always)]
    fn from(val: u8) -> RegmodL {
        RegmodL::from_bits(val)
    }
}
impl From<RegmodL> for u8 {
    #[inline(always)]
    fn from(val: RegmodL) -> u8 {
        RegmodL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Setclr {
    #[doc = "Not dependent on the direction of any counter."]
    Independent = 0x0,
    #[doc = "Reversed when counter L or the unified counter is counting down."]
    LReversed = 0x01,
    #[doc = "Reversed when counter H is counting down (do not use this value when CONFIG\\[UNIFY\\] = 1)."]
    HReversed = 0x02,
    _RESERVED_3 = 0x03,
}
impl Setclr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Setclr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Setclr {
    #[inline(always)]
    fn from(val: u8) -> Setclr {
        Setclr::from_bits(val)
    }
}
impl From<Setclr> for u8 {
    #[inline(always)]
    fn from(val: Setclr) -> u8 {
        Setclr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stateld {
    #[doc = "Value of STATEV added to that of STATE (the carry out is ignored)."]
    Add = 0x0,
    #[doc = "Value of STATEV loaded into that of STATE."]
    Load = 0x01,
}
impl Stateld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stateld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stateld {
    #[inline(always)]
    fn from(val: u8) -> Stateld {
        Stateld::from_bits(val)
    }
}
impl From<Stateld> for u8 {
    #[inline(always)]
    fn from(val: Stateld) -> u8 {
        Stateld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unify {
    #[doc = "Dual counters, COUNTER_L and COUNTER_H."]
    DualCounter = 0x0,
    #[doc = "Unified counter."]
    UnifiedCounter = 0x01,
}
impl Unify {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unify {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unify {
    #[inline(always)]
    fn from(val: u8) -> Unify {
        Unify::from_bits(val)
    }
}
impl From<Unify> for u8 {
    #[inline(always)]
    fn from(val: Unify) -> u8 {
        Unify::to_bits(val)
    }
}
