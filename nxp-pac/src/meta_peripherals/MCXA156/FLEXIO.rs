#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Flexible I/O."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexio {
    ptr: *mut u8,
}
unsafe impl Send for Flexio {}
unsafe impl Sync for Flexio {}
impl Flexio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FLEXIO Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin State."]
    #[inline(always)]
    pub const fn pin(self) -> crate::pac::common::Reg<Pin, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Shifter Status."]
    #[inline(always)]
    pub const fn shiftstat(self) -> crate::pac::common::Reg<Shiftstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Shifter Error."]
    #[inline(always)]
    pub const fn shifterr(self) -> crate::pac::common::Reg<Shifterr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Timer Status Flag."]
    #[inline(always)]
    pub const fn timstat(self) -> crate::pac::common::Reg<Timstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Shifter Status Interrupt Enable."]
    #[inline(always)]
    pub const fn shiftsien(self) -> crate::pac::common::Reg<Shiftsien, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Shifter Error Interrupt Enable."]
    #[inline(always)]
    pub const fn shifteien(self) -> crate::pac::common::Reg<Shifteien, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Timer Interrupt Enable."]
    #[inline(always)]
    pub const fn timien(self) -> crate::pac::common::Reg<Timien, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Shifter Status DMA Enable."]
    #[inline(always)]
    pub const fn shiftsden(self) -> crate::pac::common::Reg<Shiftsden, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Timer Status DMA Enable."]
    #[inline(always)]
    pub const fn timersden(self) -> crate::pac::common::Reg<Timersden, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Shifter State."]
    #[inline(always)]
    pub const fn shiftstate(self) -> crate::pac::common::Reg<Shiftstate, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Trigger Status."]
    #[inline(always)]
    pub const fn trgstat(self) -> crate::pac::common::Reg<Trgstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "External Trigger Interrupt Enable."]
    #[inline(always)]
    pub const fn trigien(self) -> crate::pac::common::Reg<Trigien, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Pin Status."]
    #[inline(always)]
    pub const fn pinstat(self) -> crate::pac::common::Reg<Pinstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Pin Interrupt Enable."]
    #[inline(always)]
    pub const fn pinien(self) -> crate::pac::common::Reg<Pinien, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Pin Rising Edge Enable."]
    #[inline(always)]
    pub const fn pinren(self) -> crate::pac::common::Reg<Pinren, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Falling Edge Enable."]
    #[inline(always)]
    pub const fn pinfen(self) -> crate::pac::common::Reg<Pinfen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Pin Output Data."]
    #[inline(always)]
    pub const fn pinoutd(self) -> crate::pac::common::Reg<Pinoutd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Pin Output Enable."]
    #[inline(always)]
    pub const fn pinoute(self) -> crate::pac::common::Reg<Pinoute, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Pin Output Disable."]
    #[inline(always)]
    pub const fn pinoutdis(self) -> crate::pac::common::Reg<Pinoutdis, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Pin Output Clear."]
    #[inline(always)]
    pub const fn pinoutclr(self) -> crate::pac::common::Reg<Pinoutclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Pin Output Set."]
    #[inline(always)]
    pub const fn pinoutset(self) -> crate::pac::common::Reg<Pinoutset, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Pin Output Toggle."]
    #[inline(always)]
    pub const fn pinouttog(self) -> crate::pac::common::Reg<Pinouttog, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Shifter Control."]
    #[inline(always)]
    pub const fn shiftctl(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftctl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Configuration."]
    #[inline(always)]
    pub const fn shiftcfg(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftcfg, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer."]
    #[inline(always)]
    pub const fn shiftbuf(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbuf, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Bit Swapped."]
    #[inline(always)]
    pub const fn shiftbufbis(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufbis, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Byte Swapped."]
    #[inline(always)]
    pub const fn shiftbufbys(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufbys, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Bit Byte Swapped."]
    #[inline(always)]
    pub const fn shiftbufbbs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufbbs, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Control."]
    #[inline(always)]
    pub const fn timctl(self, n: usize) -> crate::pac::common::Reg<Timctl, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Configuration."]
    #[inline(always)]
    pub const fn timcfg(self, n: usize) -> crate::pac::common::Reg<Timcfg, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize + n * 4usize) as _)
        }
    }
    #[doc = "Timer Compare."]
    #[inline(always)]
    pub const fn timcmp(self, n: usize) -> crate::pac::common::Reg<Timcmp, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Nibble Byte Swapped."]
    #[inline(always)]
    pub const fn shiftbufnbs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufnbs, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Halfword Swapped."]
    #[inline(always)]
    pub const fn shiftbufhws(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufhws, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Nibble Swapped."]
    #[inline(always)]
    pub const fn shiftbufnis(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufnis, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Odd Even Swapped."]
    #[inline(always)]
    pub const fn shiftbufoes(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufoes, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Even Odd Swapped."]
    #[inline(always)]
    pub const fn shiftbufeos(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufeos, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize + n * 4usize) as _)
        }
    }
    #[doc = "Shifter Buffer Halfword Byte Swapped."]
    #[inline(always)]
    pub const fn shiftbufhbs(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Shiftbufhbs, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize + n * 4usize) as _)
        }
    }
}
#[doc = "FLEXIO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "FLEXIO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn flexen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO Enable."]
    #[inline(always)]
    pub const fn set_flexen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fast Access."]
    #[must_use]
    #[inline(always)]
    pub const fn fastacc(&self) -> Fastacc {
        let val = (self.0 >> 2usize) & 0x01;
        Fastacc::from_bits(val as u8)
    }
    #[doc = "Fast Access."]
    #[inline(always)]
    pub const fn set_fastacc(&mut self, val: Fastacc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> Dbge {
        let val = (self.0 >> 30usize) & 0x01;
        Dbge::from_bits(val as u8)
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: Dbge) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Doze Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> Dozen {
        let val = (self.0 >> 31usize) & 0x01;
        Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: Dozen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("flexen", &self.flexen())
            .field("swrst", &self.swrst())
            .field("fastacc", &self.fastacc())
            .field("dbge", &self.dbge())
            .field("dozen", &self.dozen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ flexen: {=bool:?}, swrst: {=bool:?}, fastacc: {:?}, dbge: {:?}, dozen: {:?} }}",
            self.flexen(),
            self.swrst(),
            self.fastacc(),
            self.dbge(),
            self.dozen()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Shifter Number."]
    #[must_use]
    #[inline(always)]
    pub const fn shifter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number."]
    #[inline(always)]
    pub const fn set_shifter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Number."]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Number."]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pin Number."]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number."]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Trigger Number."]
    #[must_use]
    #[inline(always)]
    pub const fn trigger(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number."]
    #[inline(always)]
    pub const fn set_trigger(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("shifter", &self.shifter())
            .field("timer", &self.timer())
            .field("pin", &self.pin())
            .field("trigger", &self.trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ shifter: {=u8:?}, timer: {=u8:?}, pin: {=u8:?}, trigger: {=u8:?} }}",
            self.shifter(),
            self.timer(),
            self.pin(),
            self.trigger()
        )
    }
}
#[doc = "Pin State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin(pub u32);
impl Pin {
    #[doc = "Pin Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Data Input."]
    #[inline(always)]
    pub const fn set_pdi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pin {
    #[inline(always)]
    fn default() -> Pin {
        Pin(0)
    }
}
impl core::fmt::Debug for Pin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pin").field("pdi", &self.pdi()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pin {{ pdi: {=u32:?} }}", self.pdi())
    }
}
#[doc = "Pin Falling Edge Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinfen(pub u32);
impl Pinfen {
    #[doc = "Pin Falling Edge."]
    #[must_use]
    #[inline(always)]
    pub const fn pfe(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Falling Edge."]
    #[inline(always)]
    pub const fn set_pfe(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinfen {
    #[inline(always)]
    fn default() -> Pinfen {
        Pinfen(0)
    }
}
impl core::fmt::Debug for Pinfen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinfen").field("pfe", &self.pfe()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinfen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinfen {{ pfe: {=u32:?} }}", self.pfe())
    }
}
#[doc = "Pin Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinien(pub u32);
impl Pinien {
    #[doc = "Pin Status Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn psie(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Status Interrupt Enable."]
    #[inline(always)]
    pub const fn set_psie(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinien {
    #[inline(always)]
    fn default() -> Pinien {
        Pinien(0)
    }
}
impl core::fmt::Debug for Pinien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinien")
            .field("psie", &self.psie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinien {{ psie: {=u32:?} }}", self.psie())
    }
}
#[doc = "Pin Output Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinoutclr(pub u32);
impl Pinoutclr {
    #[doc = "Output Clear."]
    #[must_use]
    #[inline(always)]
    pub const fn outclr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Clear."]
    #[inline(always)]
    pub const fn set_outclr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinoutclr {
    #[inline(always)]
    fn default() -> Pinoutclr {
        Pinoutclr(0)
    }
}
impl core::fmt::Debug for Pinoutclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinoutclr")
            .field("outclr", &self.outclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinoutclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinoutclr {{ outclr: {=u32:?} }}", self.outclr())
    }
}
#[doc = "Pin Output Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinoutd(pub u32);
impl Pinoutd {
    #[doc = "Output Data."]
    #[must_use]
    #[inline(always)]
    pub const fn outd(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Data."]
    #[inline(always)]
    pub const fn set_outd(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinoutd {
    #[inline(always)]
    fn default() -> Pinoutd {
        Pinoutd(0)
    }
}
impl core::fmt::Debug for Pinoutd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinoutd")
            .field("outd", &self.outd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinoutd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinoutd {{ outd: {=u32:?} }}", self.outd())
    }
}
#[doc = "Pin Output Disable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinoutdis(pub u32);
impl Pinoutdis {
    #[doc = "Output Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn outdis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Disable."]
    #[inline(always)]
    pub const fn set_outdis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinoutdis {
    #[inline(always)]
    fn default() -> Pinoutdis {
        Pinoutdis(0)
    }
}
impl core::fmt::Debug for Pinoutdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinoutdis")
            .field("outdis", &self.outdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinoutdis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinoutdis {{ outdis: {=u32:?} }}", self.outdis())
    }
}
#[doc = "Pin Output Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinoute(pub u32);
impl Pinoute {
    #[doc = "Output Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn oute(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Enable."]
    #[inline(always)]
    pub const fn set_oute(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinoute {
    #[inline(always)]
    fn default() -> Pinoute {
        Pinoute(0)
    }
}
impl core::fmt::Debug for Pinoute {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinoute")
            .field("oute", &self.oute())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinoute {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinoute {{ oute: {=u32:?} }}", self.oute())
    }
}
#[doc = "Pin Output Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinoutset(pub u32);
impl Pinoutset {
    #[doc = "Output Set."]
    #[must_use]
    #[inline(always)]
    pub const fn outset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Set."]
    #[inline(always)]
    pub const fn set_outset(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinoutset {
    #[inline(always)]
    fn default() -> Pinoutset {
        Pinoutset(0)
    }
}
impl core::fmt::Debug for Pinoutset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinoutset")
            .field("outset", &self.outset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinoutset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinoutset {{ outset: {=u32:?} }}", self.outset())
    }
}
#[doc = "Pin Output Toggle."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinouttog(pub u32);
impl Pinouttog {
    #[doc = "Output Toggle."]
    #[must_use]
    #[inline(always)]
    pub const fn outtog(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Output Toggle."]
    #[inline(always)]
    pub const fn set_outtog(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinouttog {
    #[inline(always)]
    fn default() -> Pinouttog {
        Pinouttog(0)
    }
}
impl core::fmt::Debug for Pinouttog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinouttog")
            .field("outtog", &self.outtog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinouttog {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinouttog {{ outtog: {=u32:?} }}", self.outtog())
    }
}
#[doc = "Pin Rising Edge Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinren(pub u32);
impl Pinren {
    #[doc = "Pin Rising Edge."]
    #[must_use]
    #[inline(always)]
    pub const fn pre(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pin Rising Edge."]
    #[inline(always)]
    pub const fn set_pre(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinren {
    #[inline(always)]
    fn default() -> Pinren {
        Pinren(0)
    }
}
impl core::fmt::Debug for Pinren {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinren").field("pre", &self.pre()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinren {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinren {{ pre: {=u32:?} }}", self.pre())
    }
}
#[doc = "Pin Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinstat(pub u32);
impl Pinstat {
    #[doc = "Pin Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn psf(&self) -> Psf {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        Psf::from_bits(val as u32)
    }
    #[doc = "Pin Status Flag."]
    #[inline(always)]
    pub const fn set_psf(&mut self, val: Psf) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pinstat {
    #[inline(always)]
    fn default() -> Pinstat {
        Pinstat(0)
    }
}
impl core::fmt::Debug for Pinstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinstat").field("psf", &self.psf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pinstat {{ psf: {:?} }}", self.psf())
    }
}
#[doc = "Shifter Buffer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbuf(pub u32);
impl Shiftbuf {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbuf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbuf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbuf {
    #[inline(always)]
    fn default() -> Shiftbuf {
        Shiftbuf(0)
    }
}
impl core::fmt::Debug for Shiftbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbuf")
            .field("shiftbuf", &self.shiftbuf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftbuf {{ shiftbuf: {=u32:?} }}", self.shiftbuf())
    }
}
#[doc = "Shifter Buffer Bit Byte Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbbs(pub u32);
impl Shiftbufbbs {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufbbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbbs {
    #[inline(always)]
    fn default() -> Shiftbufbbs {
        Shiftbufbbs(0)
    }
}
impl core::fmt::Debug for Shiftbufbbs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbbs")
            .field("shiftbufbbs", &self.shiftbufbbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbbs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbbs {{ shiftbufbbs: {=u32:?} }}",
            self.shiftbufbbs()
        )
    }
}
#[doc = "Shifter Buffer Bit Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbis(pub u32);
impl Shiftbufbis {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufbis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbis {
    #[inline(always)]
    fn default() -> Shiftbufbis {
        Shiftbufbis(0)
    }
}
impl core::fmt::Debug for Shiftbufbis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbis")
            .field("shiftbufbis", &self.shiftbufbis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbis {{ shiftbufbis: {=u32:?} }}",
            self.shiftbufbis()
        )
    }
}
#[doc = "Shifter Buffer Byte Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufbys(pub u32);
impl Shiftbufbys {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufbys(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufbys(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufbys {
    #[inline(always)]
    fn default() -> Shiftbufbys {
        Shiftbufbys(0)
    }
}
impl core::fmt::Debug for Shiftbufbys {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufbys")
            .field("shiftbufbys", &self.shiftbufbys())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufbys {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufbys {{ shiftbufbys: {=u32:?} }}",
            self.shiftbufbys()
        )
    }
}
#[doc = "Shifter Buffer Even Odd Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufeos(pub u32);
impl Shiftbufeos {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufeos(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufeos(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufeos {
    #[inline(always)]
    fn default() -> Shiftbufeos {
        Shiftbufeos(0)
    }
}
impl core::fmt::Debug for Shiftbufeos {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufeos")
            .field("shiftbufeos", &self.shiftbufeos())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufeos {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufeos {{ shiftbufeos: {=u32:?} }}",
            self.shiftbufeos()
        )
    }
}
#[doc = "Shifter Buffer Halfword Byte Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufhbs(pub u32);
impl Shiftbufhbs {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufhbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufhbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufhbs {
    #[inline(always)]
    fn default() -> Shiftbufhbs {
        Shiftbufhbs(0)
    }
}
impl core::fmt::Debug for Shiftbufhbs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufhbs")
            .field("shiftbufhbs", &self.shiftbufhbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufhbs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufhbs {{ shiftbufhbs: {=u32:?} }}",
            self.shiftbufhbs()
        )
    }
}
#[doc = "Shifter Buffer Halfword Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufhws(pub u32);
impl Shiftbufhws {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufhws(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufhws(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufhws {
    #[inline(always)]
    fn default() -> Shiftbufhws {
        Shiftbufhws(0)
    }
}
impl core::fmt::Debug for Shiftbufhws {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufhws")
            .field("shiftbufhws", &self.shiftbufhws())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufhws {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufhws {{ shiftbufhws: {=u32:?} }}",
            self.shiftbufhws()
        )
    }
}
#[doc = "Shifter Buffer Nibble Byte Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufnbs(pub u32);
impl Shiftbufnbs {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufnbs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufnbs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufnbs {
    #[inline(always)]
    fn default() -> Shiftbufnbs {
        Shiftbufnbs(0)
    }
}
impl core::fmt::Debug for Shiftbufnbs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufnbs")
            .field("shiftbufnbs", &self.shiftbufnbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufnbs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufnbs {{ shiftbufnbs: {=u32:?} }}",
            self.shiftbufnbs()
        )
    }
}
#[doc = "Shifter Buffer Nibble Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufnis(pub u32);
impl Shiftbufnis {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufnis(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufnis(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufnis {
    #[inline(always)]
    fn default() -> Shiftbufnis {
        Shiftbufnis(0)
    }
}
impl core::fmt::Debug for Shiftbufnis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufnis")
            .field("shiftbufnis", &self.shiftbufnis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufnis {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufnis {{ shiftbufnis: {=u32:?} }}",
            self.shiftbufnis()
        )
    }
}
#[doc = "Shifter Buffer Odd Even Swapped."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftbufoes(pub u32);
impl Shiftbufoes {
    #[doc = "Shift Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn shiftbufoes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Shift Buffer."]
    #[inline(always)]
    pub const fn set_shiftbufoes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Shiftbufoes {
    #[inline(always)]
    fn default() -> Shiftbufoes {
        Shiftbufoes(0)
    }
}
impl core::fmt::Debug for Shiftbufoes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftbufoes")
            .field("shiftbufoes", &self.shiftbufoes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftbufoes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftbufoes {{ shiftbufoes: {=u32:?} }}",
            self.shiftbufoes()
        )
    }
}
#[doc = "Shifter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftcfg(pub u32);
impl Shiftcfg {
    #[doc = "Shifter Start."]
    #[must_use]
    #[inline(always)]
    pub const fn sstart(&self) -> Sstart {
        let val = (self.0 >> 0usize) & 0x03;
        Sstart::from_bits(val as u8)
    }
    #[doc = "Shifter Start."]
    #[inline(always)]
    pub const fn set_sstart(&mut self, val: Sstart) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Shifter Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> Sstop {
        let val = (self.0 >> 4usize) & 0x03;
        Sstop::from_bits(val as u8)
    }
    #[doc = "Shifter Stop."]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: Sstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Source."]
    #[must_use]
    #[inline(always)]
    pub const fn insrc(&self) -> Insrc {
        let val = (self.0 >> 8usize) & 0x01;
        Insrc::from_bits(val as u8)
    }
    #[doc = "Input Source."]
    #[inline(always)]
    pub const fn set_insrc(&mut self, val: Insrc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Late Store."]
    #[must_use]
    #[inline(always)]
    pub const fn latst(&self) -> Latst {
        let val = (self.0 >> 9usize) & 0x01;
        Latst::from_bits(val as u8)
    }
    #[doc = "Late Store."]
    #[inline(always)]
    pub const fn set_latst(&mut self, val: Latst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Shifter Size."]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> Ssize {
        let val = (self.0 >> 12usize) & 0x01;
        Ssize::from_bits(val as u8)
    }
    #[doc = "Shifter Size."]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: Ssize) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Parallel Width."]
    #[must_use]
    #[inline(always)]
    pub const fn pwidth(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parallel Width."]
    #[inline(always)]
    pub const fn set_pwidth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Shiftcfg {
    #[inline(always)]
    fn default() -> Shiftcfg {
        Shiftcfg(0)
    }
}
impl core::fmt::Debug for Shiftcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftcfg")
            .field("sstart", &self.sstart())
            .field("sstop", &self.sstop())
            .field("insrc", &self.insrc())
            .field("latst", &self.latst())
            .field("ssize", &self.ssize())
            .field("pwidth", &self.pwidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftcfg {{ sstart: {:?}, sstop: {:?}, insrc: {:?}, latst: {:?}, ssize: {:?}, pwidth: {=u8:?} }}",
            self.sstart(),
            self.sstop(),
            self.insrc(),
            self.latst(),
            self.ssize(),
            self.pwidth()
        )
    }
}
#[doc = "Shifter Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftctl(pub u32);
impl Shiftctl {
    #[doc = "Shifter Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> Smod {
        let val = (self.0 >> 0usize) & 0x07;
        Smod::from_bits(val as u8)
    }
    #[doc = "Shifter Mode."]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: Smod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Shifter Pin Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> ShiftctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        ShiftctlPinpol::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Polarity."]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: ShiftctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Shifter Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shifter Pin Select."]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Shifter Pin Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> ShiftctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        ShiftctlPincfg::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Configuration."]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: ShiftctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Timer Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn timpol(&self) -> Timpol {
        let val = (self.0 >> 23usize) & 0x01;
        Timpol::from_bits(val as u8)
    }
    #[doc = "Timer Polarity."]
    #[inline(always)]
    pub const fn set_timpol(&mut self, val: Timpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Timer Select."]
    #[must_use]
    #[inline(always)]
    pub const fn timsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Timer Select."]
    #[inline(always)]
    pub const fn set_timsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Shiftctl {
    #[inline(always)]
    fn default() -> Shiftctl {
        Shiftctl(0)
    }
}
impl core::fmt::Debug for Shiftctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftctl")
            .field("smod", &self.smod())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("timpol", &self.timpol())
            .field("timsel", &self.timsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Shiftctl {{ smod: {:?}, pinpol: {:?}, pinsel: {=u8:?}, pincfg: {:?}, timpol: {:?}, timsel: {=u8:?} }}",
            self.smod(),
            self.pinpol(),
            self.pinsel(),
            self.pincfg(),
            self.timpol(),
            self.timsel()
        )
    }
}
#[doc = "Shifter Error Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifteien(pub u32);
impl Shifteien {
    #[doc = "Shifter Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Shifter Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Shifteien {
    #[inline(always)]
    fn default() -> Shifteien {
        Shifteien(0)
    }
}
impl core::fmt::Debug for Shifteien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifteien")
            .field("seie", &self.seie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifteien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shifteien {{ seie: {=u8:?} }}", self.seie())
    }
}
#[doc = "Shifter Error."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifterr(pub u32);
impl Shifterr {
    #[doc = "Shifter Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> Sef {
        let val = (self.0 >> 0usize) & 0x0f;
        Sef::from_bits(val as u8)
    }
    #[doc = "Shifter Error Flag."]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: Sef) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Shifterr {
    #[inline(always)]
    fn default() -> Shifterr {
        Shifterr(0)
    }
}
impl core::fmt::Debug for Shifterr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifterr")
            .field("sef", &self.sef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifterr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shifterr {{ sef: {:?} }}", self.sef())
    }
}
#[doc = "Shifter Status DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsden(pub u32);
impl Shiftsden {
    #[doc = "Shifter Status DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ssde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Shifter Status DMA Enable."]
    #[inline(always)]
    pub const fn set_ssde(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Shiftsden {
    #[inline(always)]
    fn default() -> Shiftsden {
        Shiftsden(0)
    }
}
impl core::fmt::Debug for Shiftsden {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsden")
            .field("ssde", &self.ssde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsden {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftsden {{ ssde: {=u8:?} }}", self.ssde())
    }
}
#[doc = "Shifter Status Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsien(pub u32);
impl Shiftsien {
    #[doc = "Shifter Status Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ssie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Shifter Status Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ssie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Shiftsien {
    #[inline(always)]
    fn default() -> Shiftsien {
        Shiftsien(0)
    }
}
impl core::fmt::Debug for Shiftsien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsien")
            .field("ssie", &self.ssie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftsien {{ ssie: {=u8:?} }}", self.ssie())
    }
}
#[doc = "Shifter Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstat(pub u32);
impl Shiftstat {
    #[doc = "Shifter Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ssf(&self) -> Ssf {
        let val = (self.0 >> 0usize) & 0x0f;
        Ssf::from_bits(val as u8)
    }
    #[doc = "Shifter Status Flag."]
    #[inline(always)]
    pub const fn set_ssf(&mut self, val: Ssf) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Shiftstat {
    #[inline(always)]
    fn default() -> Shiftstat {
        Shiftstat(0)
    }
}
impl core::fmt::Debug for Shiftstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstat")
            .field("ssf", &self.ssf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftstat {{ ssf: {:?} }}", self.ssf())
    }
}
#[doc = "Shifter State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstate(pub u32);
impl Shiftstate {
    #[doc = "Current State Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current State Pointer."]
    #[inline(always)]
    pub const fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Shiftstate {
    #[inline(always)]
    fn default() -> Shiftstate {
        Shiftstate(0)
    }
}
impl core::fmt::Debug for Shiftstate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstate")
            .field("state", &self.state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Shiftstate {{ state: {=u8:?} }}", self.state())
    }
}
#[doc = "Timer Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcfg(pub u32);
impl Timcfg {
    #[doc = "Timer Start."]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Start."]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> Tstop {
        let val = (self.0 >> 4usize) & 0x03;
        Tstop::from_bits(val as u8)
    }
    #[doc = "Timer Stop."]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: Tstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn timena(&self) -> Timena {
        let val = (self.0 >> 8usize) & 0x07;
        Timena::from_bits(val as u8)
    }
    #[doc = "Timer Enable."]
    #[inline(always)]
    pub const fn set_timena(&mut self, val: Timena) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Timer Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn timdis(&self) -> Timdis {
        let val = (self.0 >> 12usize) & 0x07;
        Timdis::from_bits(val as u8)
    }
    #[doc = "Timer Disable."]
    #[inline(always)]
    pub const fn set_timdis(&mut self, val: Timdis) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn timrst(&self) -> Timrst {
        let val = (self.0 >> 16usize) & 0x07;
        Timrst::from_bits(val as u8)
    }
    #[doc = "Timer Reset."]
    #[inline(always)]
    pub const fn set_timrst(&mut self, val: Timrst) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Timer Decrement."]
    #[must_use]
    #[inline(always)]
    pub const fn timdec(&self) -> Timdec {
        let val = (self.0 >> 20usize) & 0x07;
        Timdec::from_bits(val as u8)
    }
    #[doc = "Timer Decrement."]
    #[inline(always)]
    pub const fn set_timdec(&mut self, val: Timdec) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Timer Output."]
    #[must_use]
    #[inline(always)]
    pub const fn timout(&self) -> Timout {
        let val = (self.0 >> 24usize) & 0x03;
        Timout::from_bits(val as u8)
    }
    #[doc = "Timer Output."]
    #[inline(always)]
    pub const fn set_timout(&mut self, val: Timout) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Timcfg {
    #[inline(always)]
    fn default() -> Timcfg {
        Timcfg(0)
    }
}
impl core::fmt::Debug for Timcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcfg")
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("timena", &self.timena())
            .field("timdis", &self.timdis())
            .field("timrst", &self.timrst())
            .field("timdec", &self.timdec())
            .field("timout", &self.timout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timcfg {{ tstart: {=bool:?}, tstop: {:?}, timena: {:?}, timdis: {:?}, timrst: {:?}, timdec: {:?}, timout: {:?} }}",
            self.tstart(),
            self.tstop(),
            self.timena(),
            self.timdis(),
            self.timrst(),
            self.timdec(),
            self.timout()
        )
    }
}
#[doc = "Timer Compare."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcmp(pub u32);
impl Timcmp {
    #[doc = "Timer Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Compare Value."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timcmp {
    #[inline(always)]
    fn default() -> Timcmp {
        Timcmp(0)
    }
}
impl core::fmt::Debug for Timcmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcmp").field("cmp", &self.cmp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcmp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timcmp {{ cmp: {=u16:?} }}", self.cmp())
    }
}
#[doc = "Timer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timctl(pub u32);
impl Timctl {
    #[doc = "Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn timod(&self) -> Timod {
        let val = (self.0 >> 0usize) & 0x07;
        Timod::from_bits(val as u8)
    }
    #[doc = "Timer Mode."]
    #[inline(always)]
    pub const fn set_timod(&mut self, val: Timod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Timer One Time Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn onetim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer One Time Operation."]
    #[inline(always)]
    pub const fn set_onetim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Timer Pin Input Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pinins(&self) -> Pinins {
        let val = (self.0 >> 6usize) & 0x01;
        Pinins::from_bits(val as u8)
    }
    #[doc = "Timer Pin Input Select."]
    #[inline(always)]
    pub const fn set_pinins(&mut self, val: Pinins) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Pin Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> TimctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        TimctlPinpol::from_bits(val as u8)
    }
    #[doc = "Timer Pin Polarity."]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: TimctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Pin Select."]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Timer Pin Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> TimctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        TimctlPincfg::from_bits(val as u8)
    }
    #[doc = "Timer Pin Configuration."]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: TimctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Trigger Source."]
    #[must_use]
    #[inline(always)]
    pub const fn trgsrc(&self) -> Trgsrc {
        let val = (self.0 >> 22usize) & 0x01;
        Trgsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source."]
    #[inline(always)]
    pub const fn set_trgsrc(&mut self, val: Trgsrc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Trigger Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn trgpol(&self) -> Trgpol {
        let val = (self.0 >> 23usize) & 0x01;
        Trgpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity."]
    #[inline(always)]
    pub const fn set_trgpol(&mut self, val: Trgpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select."]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trigger Select."]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Timctl {
    #[inline(always)]
    fn default() -> Timctl {
        Timctl(0)
    }
}
impl core::fmt::Debug for Timctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timctl")
            .field("timod", &self.timod())
            .field("onetim", &self.onetim())
            .field("pinins", &self.pinins())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("trgsrc", &self.trgsrc())
            .field("trgpol", &self.trgpol())
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Timctl {{ timod: {:?}, onetim: {=bool:?}, pinins: {:?}, pinpol: {:?}, pinsel: {=u8:?}, pincfg: {:?}, trgsrc: {:?}, trgpol: {:?}, trgsel: {=u8:?} }}",
            self.timod(),
            self.onetim(),
            self.pinins(),
            self.pinpol(),
            self.pinsel(),
            self.pincfg(),
            self.trgsrc(),
            self.trgpol(),
            self.trgsel()
        )
    }
}
#[doc = "Timer Status DMA Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timersden(pub u32);
impl Timersden {
    #[doc = "Timer Status DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tsde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timer Status DMA Enable."]
    #[inline(always)]
    pub const fn set_tsde(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Timersden {
    #[inline(always)]
    fn default() -> Timersden {
        Timersden(0)
    }
}
impl core::fmt::Debug for Timersden {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timersden")
            .field("tsde", &self.tsde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timersden {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timersden {{ tsde: {=u8:?} }}", self.tsde())
    }
}
#[doc = "Timer Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timien(pub u32);
impl Timien {
    #[doc = "Timer Status Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Timer Status Interrupt Enable."]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Timien {
    #[inline(always)]
    fn default() -> Timien {
        Timien(0)
    }
}
impl core::fmt::Debug for Timien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timien")
            .field("teie", &self.teie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timien {{ teie: {=u8:?} }}", self.teie())
    }
}
#[doc = "Timer Status Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timstat(pub u32);
impl Timstat {
    #[doc = "Timer Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> Tsf {
        let val = (self.0 >> 0usize) & 0x0f;
        Tsf::from_bits(val as u8)
    }
    #[doc = "Timer Status Flag."]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: Tsf) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Timstat {
    #[inline(always)]
    fn default() -> Timstat {
        Timstat(0)
    }
}
impl core::fmt::Debug for Timstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timstat").field("tsf", &self.tsf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timstat {{ tsf: {:?} }}", self.tsf())
    }
}
#[doc = "Trigger Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgstat(pub u32);
impl Trgstat {
    #[doc = "External Trigger Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn etsf(&self) -> Etsf {
        let val = (self.0 >> 0usize) & 0x0f;
        Etsf::from_bits(val as u8)
    }
    #[doc = "External Trigger Status Flag."]
    #[inline(always)]
    pub const fn set_etsf(&mut self, val: Etsf) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Trgstat {
    #[inline(always)]
    fn default() -> Trgstat {
        Trgstat(0)
    }
}
impl core::fmt::Debug for Trgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trgstat")
            .field("etsf", &self.etsf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trgstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Trgstat {{ etsf: {:?} }}", self.etsf())
    }
}
#[doc = "External Trigger Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigien(pub u32);
impl Trigien {
    #[doc = "External Trigger Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn trie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "External Trigger Interrupt Enable."]
    #[inline(always)]
    pub const fn set_trie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Trigien {
    #[inline(always)]
    fn default() -> Trigien {
        Trigien(0)
    }
}
impl core::fmt::Debug for Trigien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trigien")
            .field("trie", &self.trie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trigien {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Trigien {{ trie: {=u8:?} }}", self.trie())
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbge {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable."]
    Emable = 0x01,
}
impl Dbge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbge {
    #[inline(always)]
    fn from(val: u8) -> Dbge {
        Dbge::from_bits(val)
    }
}
impl From<Dbge> for u8 {
    #[inline(always)]
    fn from(val: Dbge) -> u8 {
        Dbge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozen {
    #[doc = "Enable."]
    Enable = 0x0,
    #[doc = "Disable."]
    Disable = 0x01,
}
impl Dozen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozen {
    #[inline(always)]
    fn from(val: u8) -> Dozen {
        Dozen::from_bits(val)
    }
}
impl From<Dozen> for u8 {
    #[inline(always)]
    fn from(val: Dozen) -> u8 {
        Dozen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Etsf {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Etsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Etsf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Etsf {
    #[inline(always)]
    fn from(val: u8) -> Etsf {
        Etsf::from_bits(val)
    }
}
impl From<Etsf> for u8 {
    #[inline(always)]
    fn from(val: Etsf) -> u8 {
        Etsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fastacc {
    #[doc = "Normal."]
    Normal = 0x0,
    #[doc = "Fast."]
    Fast = 0x01,
}
impl Fastacc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fastacc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fastacc {
    #[inline(always)]
    fn from(val: u8) -> Fastacc {
        Fastacc::from_bits(val)
    }
}
impl From<Fastacc> for u8 {
    #[inline(always)]
    fn from(val: Fastacc) -> u8 {
        Fastacc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features implemented."]
    pub const Standard: Self = Self(0x0);
    #[doc = "State, logic, and parallel modes supported."]
    pub const StateLogicParallel: Self = Self(0x01);
    #[doc = "Pin control registers supported."]
    pub const Pinctrl: Self = Self(0x02);
    #[doc = "State, logic, and parallel modes, plus pin control registers supported."]
    pub const StateLogicParallelPinctrl: Self = Self(0x03);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Standard"),
            0x01 => f.write_str("StateLogicParallel"),
            0x02 => f.write_str("Pinctrl"),
            0x03 => f.write_str("StateLogicParallelPinctrl"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Standard"),
            0x01 => defmt::write!(f, "StateLogicParallel"),
            0x02 => defmt::write!(f, "Pinctrl"),
            0x03 => defmt::write!(f, "StateLogicParallelPinctrl"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Insrc {
    #[doc = "Pin."]
    Pin = 0x0,
    #[doc = "Shifter n+1 output."]
    ShifterNplus1 = 0x01,
}
impl Insrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Insrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Insrc {
    #[inline(always)]
    fn from(val: u8) -> Insrc {
        Insrc::from_bits(val)
    }
}
impl From<Insrc> for u8 {
    #[inline(always)]
    fn from(val: Insrc) -> u8 {
        Insrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Latst {
    #[doc = "Store the pre-shift register state."]
    Preshift = 0x0,
    #[doc = "Store the post-shift register state."]
    Postshift = 0x01,
}
impl Latst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Latst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Latst {
    #[inline(always)]
    fn from(val: u8) -> Latst {
        Latst::from_bits(val)
    }
}
impl From<Latst> for u8 {
    #[inline(always)]
    fn from(val: Latst) -> u8 {
        Latst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pinins {
    #[doc = "PINSEL selects timer pin input and output."]
    Pinsel = 0x0,
    #[doc = "PINSEL + 1 selects the timer pin input; timer pin output remains selected by PINSEL."]
    Pinselplus1 = 0x01,
}
impl Pinins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pinins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pinins {
    #[inline(always)]
    fn from(val: u8) -> Pinins {
        Pinins::from_bits(val)
    }
}
impl From<Pinins> for u8 {
    #[inline(always)]
    fn from(val: Pinins) -> u8 {
        Pinins::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Psf(u32);
impl Psf {
    #[doc = "Clear."]
    pub const Clr: Self = Self(0x0);
    #[doc = "Set."]
    pub const Set: Self = Self(0x01);
}
impl Psf {
    pub const fn from_bits(val: u32) -> Psf {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Psf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Clr"),
            0x01 => f.write_str("Set"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psf {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Clr"),
            0x01 => defmt::write!(f, "Set"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Psf {
    #[inline(always)]
    fn from(val: u32) -> Psf {
        Psf::from_bits(val)
    }
}
impl From<Psf> for u32 {
    #[inline(always)]
    fn from(val: Psf) -> u32 {
        Psf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sef {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Sef {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sef {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sef {
    #[inline(always)]
    fn from(val: u8) -> Sef {
        Sef::from_bits(val)
    }
}
impl From<Sef> for u8 {
    #[inline(always)]
    fn from(val: Sef) -> u8 {
        Sef::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPincfg {
    #[doc = "Shifter pin output disabled."]
    Disable = 0x0,
    #[doc = "Shifter pin open-drain or bidirectional output enable."]
    OpendBidirouten = 0x01,
    #[doc = "Shifter pin bidirectional output data."]
    BidirOutdata = 0x02,
    #[doc = "Shifter pin output."]
    Output = 0x03,
}
impl ShiftctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPincfg {
        ShiftctlPincfg::from_bits(val)
    }
}
impl From<ShiftctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPincfg) -> u8 {
        ShiftctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ShiftctlPinpol {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl ShiftctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ShiftctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ShiftctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> ShiftctlPinpol {
        ShiftctlPinpol::from_bits(val)
    }
}
impl From<ShiftctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: ShiftctlPinpol) -> u8 {
        ShiftctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smod {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Receive mode; capture the current shifter content into SHIFTBUF on expiration of the timer."]
    Receive = 0x01,
    #[doc = "Transmit mode; load SHIFTBUF contents into the shifter on expiration of the timer."]
    Transmit = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Match Store mode; shifter data is compared to SHIFTBUF content on expiration of the timer."]
    Matchstore = 0x04,
    #[doc = "Match Continuous mode; shifter data is continuously compared to SHIFTBUF contents."]
    Matchcont = 0x05,
    #[doc = "State mode; SHIFTBUF contents store programmable state attributes."]
    State = 0x06,
    #[doc = "Logic mode; SHIFTBUF contents implement programmable logic lookup table."]
    Logic = 0x07,
}
impl Smod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssf {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ssf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssf {
    #[inline(always)]
    fn from(val: u8) -> Ssf {
        Ssf::from_bits(val)
    }
}
impl From<Ssf> for u8 {
    #[inline(always)]
    fn from(val: Ssf) -> u8 {
        Ssf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ssize {
    #[doc = "32-bit."]
    Width32 = 0x0,
    #[doc = "24-bit."]
    Width24 = 0x01,
}
impl Ssize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ssize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ssize {
    #[inline(always)]
    fn from(val: u8) -> Ssize {
        Ssize::from_bits(val)
    }
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(val: Ssize) -> u8 {
        Ssize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstart {
    #[doc = "Start bit disabled for Transmitter, Receiver, and Match Store modes; Transmitter mode loads data on enable."]
    Value00 = 0x0,
    #[doc = "Start bit disabled for Transmitter, Receiver, and Match Store modes; Transmitter mode loads data on first shift."]
    Value01 = 0x01,
    #[doc = "Transmitter mode outputs start bit value 0 before loading data on first shift; if start bit is not 0, Receiver and Match Store modes set error flag."]
    Value10 = 0x02,
    #[doc = "Transmitter mode outputs start bit value 1 before loading data on first shift; if start bit is not 1, Receiver and Match Store modes set error flag."]
    Value11 = 0x03,
}
impl Sstart {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstart {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstart {
    #[inline(always)]
    fn from(val: u8) -> Sstart {
        Sstart::from_bits(val)
    }
}
impl From<Sstart> for u8 {
    #[inline(always)]
    fn from(val: Sstart) -> u8 {
        Sstart::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sstop {
    #[doc = "Stop bit disabled for Transmitter, Receiver, and Match Store modes."]
    Value00 = 0x0,
    #[doc = "Stop bit disabled for Transmitter, Receiver, and Match Store modes; when timer is in stop condition, Receiver and Match Store modes store receive data on the configured shift edge."]
    Value01 = 0x01,
    #[doc = "Transmitter mode outputs stop bit value 0 in Match Store mode; if stop bit is not 0, Receiver and Match Store modes set error flag (when timer is in stop condition, these modes also store receive data on the configured shift edge)."]
    Value10 = 0x02,
    #[doc = "Transmitter mode outputs stop bit value 1 in Match Store mode; if stop bit is not 1, Receiver and Match Store modes set error flag (when timer is in stop condition, these modes also store receive data on the configured shift edge)."]
    Value11 = 0x03,
}
impl Sstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sstop {
    #[inline(always)]
    fn from(val: u8) -> Sstop {
        Sstop::from_bits(val)
    }
}
impl From<Sstop> for u8 {
    #[inline(always)]
    fn from(val: Sstop) -> u8 {
        Sstop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPincfg {
    #[doc = "Timer pin output disabled."]
    Outdisable = 0x0,
    #[doc = "Timer pin open-drain or bidirectional output enable."]
    OpendBidirouten = 0x01,
    #[doc = "Timer pin bidirectional output data."]
    BidirOutdata = 0x02,
    #[doc = "Timer pin output."]
    Output = 0x03,
}
impl TimctlPincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPincfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPincfg {
    #[inline(always)]
    fn from(val: u8) -> TimctlPincfg {
        TimctlPincfg::from_bits(val)
    }
}
impl From<TimctlPincfg> for u8 {
    #[inline(always)]
    fn from(val: TimctlPincfg) -> u8 {
        TimctlPincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimctlPinpol {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl TimctlPinpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TimctlPinpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TimctlPinpol {
    #[inline(always)]
    fn from(val: u8) -> TimctlPinpol {
        TimctlPinpol::from_bits(val)
    }
}
impl From<TimctlPinpol> for u8 {
    #[inline(always)]
    fn from(val: TimctlPinpol) -> u8 {
        TimctlPinpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdec {
    #[doc = "Decrement counter on FLEXIO clock; shift clock equals timer output."]
    FlexioClkShiftclkTmrOut = 0x0,
    #[doc = "Decrement counter on trigger input (both edges); shift clock equals timer output."]
    TrigEdgeShiftclkTmrOut = 0x01,
    #[doc = "Decrement counter on pin input (both edges); shift clock equals pin input."]
    PinEdgeShiftclkTmrOut = 0x02,
    #[doc = "Decrement counter on trigger input (both edges); shift clock equals trigger input."]
    TrigEdgeShiftclkTrigIn = 0x03,
    #[doc = "Decrement counter on FLEXIO clock divided by 16; shift clock equals timer output."]
    FlexioClkDiv16ShiftclkTmrOut = 0x04,
    #[doc = "Decrement counter on FLEXIO clock divided by 256; shift clock equals timer output."]
    FlexioClkDiv256ShiftclkTmrOut = 0x05,
    #[doc = "Decrement counter on pin input (rising edge); shift clock equals pin input."]
    PinRiseShiftclkPinIn = 0x06,
    #[doc = "Decrement counter on trigger input (rising edge); shift clock equals trigger input."]
    TrigRiseShiftclkTrigIn = 0x07,
}
impl Timdec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdec {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdec {
    #[inline(always)]
    fn from(val: u8) -> Timdec {
        Timdec::from_bits(val)
    }
}
impl From<Timdec> for u8 {
    #[inline(always)]
    fn from(val: Timdec) -> u8 {
        Timdec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timdis {
    #[doc = "Timer never disabled."]
    Never = 0x0,
    #[doc = "Timer disabled on timer n-1 disable."]
    TmrNminus1 = 0x01,
    #[doc = "Timer disabled on timer compare (upper 8 bits match and decrement)."]
    TmrCmp = 0x02,
    #[doc = "Timer disabled on timer compare (upper 8 bits match and decrement) and trigger low."]
    TmrCmpTriglow = 0x03,
    #[doc = "Timer disabled on pin rising or falling edge."]
    PinEdge = 0x04,
    #[doc = "Timer disabled on pin rising or falling edge provided trigger is high."]
    PinEdgeTrighi = 0x05,
    #[doc = "Timer disabled on trigger falling edge."]
    TrigFalledge = 0x06,
    _RESERVED_7 = 0x07,
}
impl Timdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timdis {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timdis {
    #[inline(always)]
    fn from(val: u8) -> Timdis {
        Timdis::from_bits(val)
    }
}
impl From<Timdis> for u8 {
    #[inline(always)]
    fn from(val: Timdis) -> u8 {
        Timdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timena {
    #[doc = "Timer always enabled."]
    Enable = 0x0,
    #[doc = "Timer enabled on timer n-1 enable."]
    TmrNminus1En = 0x01,
    #[doc = "Timer enabled on trigger high."]
    TmrTrighiEn = 0x02,
    #[doc = "Timer enabled on trigger high and pin high."]
    TmrTrigPinHiEn = 0x03,
    #[doc = "Timer enabled on pin rising edge."]
    TmrPinriseEn = 0x04,
    #[doc = "Timer enabled on pin rising edge and trigger high."]
    TmrPinriseTrighiEn = 0x05,
    #[doc = "Timer enabled on trigger rising edge."]
    TmrTrigriseEn = 0x06,
    #[doc = "Timer enabled on trigger rising or falling edge."]
    TmrTrigedgeEn = 0x07,
}
impl Timena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timena {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timena {
    #[inline(always)]
    fn from(val: u8) -> Timena {
        Timena::from_bits(val)
    }
}
impl From<Timena> for u8 {
    #[inline(always)]
    fn from(val: Timena) -> u8 {
        Timena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timod {
    #[doc = "Timer disabled."]
    Disable = 0x0,
    #[doc = "Dual 8-bit counters baud mode."]
    Dual8bitBaud = 0x01,
    #[doc = "Dual 8-bit counters PWM high mode."]
    Dual8bitPwmH = 0x02,
    #[doc = "Single 16-bit counter mode."]
    Single16bit = 0x03,
    #[doc = "Single 16-bit counter disable mode."]
    Single16bitDisable = 0x04,
    #[doc = "Dual 8-bit counters word mode."]
    Dual8bitWord = 0x05,
    #[doc = "Dual 8-bit counters PWM low mode."]
    Dual8bitPwmL = 0x06,
    #[doc = "Single 16-bit input capture mode."]
    Single16bitInCapture = 0x07,
}
impl Timod {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timod {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timod {
    #[inline(always)]
    fn from(val: u8) -> Timod {
        Timod::from_bits(val)
    }
}
impl From<Timod> for u8 {
    #[inline(always)]
    fn from(val: Timod) -> u8 {
        Timod::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timout {
    #[doc = "Logic one when enabled; not affected by timer reset."]
    One = 0x0,
    #[doc = "Logic zero when enabled; not affected by timer reset."]
    Zero = 0x01,
    #[doc = "Logic one when enabled and on timer reset."]
    OneTmrreset = 0x02,
    #[doc = "Logic zero when enabled and on timer reset."]
    ZeroTmrreset = 0x03,
}
impl Timout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timout {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timout {
    #[inline(always)]
    fn from(val: u8) -> Timout {
        Timout::from_bits(val)
    }
}
impl From<Timout> for u8 {
    #[inline(always)]
    fn from(val: Timout) -> u8 {
        Timout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timpol {
    #[doc = "Positive edge."]
    Posedge = 0x0,
    #[doc = "Negative edge."]
    Negedge = 0x01,
}
impl Timpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timpol {
    #[inline(always)]
    fn from(val: u8) -> Timpol {
        Timpol::from_bits(val)
    }
}
impl From<Timpol> for u8 {
    #[inline(always)]
    fn from(val: Timpol) -> u8 {
        Timpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timrst {
    #[doc = "Never reset timer."]
    Never = 0x0,
    #[doc = "Timer reset on timer output high."]
    TmrOutHi = 0x01,
    #[doc = "Timer reset on timer pin equal to timer output."]
    PinEqTmrOut = 0x02,
    #[doc = "Timer reset on timer trigger equal to timer output."]
    TrigEqTmrOut = 0x03,
    #[doc = "Timer reset on timer pin rising edge."]
    PinRiseEdge = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Timer reset on trigger rising edge."]
    TrigRiseEdge = 0x06,
    #[doc = "Timer reset on trigger rising or falling edge."]
    TrigEdge = 0x07,
}
impl Timrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timrst {
    #[inline(always)]
    fn from(val: u8) -> Timrst {
        Timrst::from_bits(val)
    }
}
impl From<Timrst> for u8 {
    #[inline(always)]
    fn from(val: Timrst) -> u8 {
        Timrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgpol {
    #[doc = "Active high."]
    ActiveHigh = 0x0,
    #[doc = "Active low."]
    ActiveLow = 0x01,
}
impl Trgpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgpol {
    #[inline(always)]
    fn from(val: u8) -> Trgpol {
        Trgpol::from_bits(val)
    }
}
impl From<Trgpol> for u8 {
    #[inline(always)]
    fn from(val: Trgpol) -> u8 {
        Trgpol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trgsrc {
    #[doc = "External."]
    ExtTrig = 0x0,
    #[doc = "Internal."]
    InternalTrig = 0x01,
}
impl Trgsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trgsrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trgsrc {
    #[inline(always)]
    fn from(val: u8) -> Trgsrc {
        Trgsrc::from_bits(val)
    }
}
impl From<Trgsrc> for u8 {
    #[inline(always)]
    fn from(val: Trgsrc) -> u8 {
        Trgsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tsf {
    #[doc = "Clear."]
    Clr = 0x0,
    #[doc = "Set."]
    Set = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Tsf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tsf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tsf {
    #[inline(always)]
    fn from(val: u8) -> Tsf {
        Tsf::from_bits(val)
    }
}
impl From<Tsf> for u8 {
    #[inline(always)]
    fn from(val: Tsf) -> u8 {
        Tsf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstop {
    #[doc = "Disabled."]
    StopDisable = 0x0,
    #[doc = "Enabled on timer compare."]
    EnableTmrcmp = 0x01,
    #[doc = "Enabled on timer disable."]
    EnableTmrdisable = 0x02,
    #[doc = "Enabled on timer compare and timer disable."]
    EnableTmrCmpDis = 0x03,
}
impl Tstop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstop {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstop {
    #[inline(always)]
    fn from(val: u8) -> Tstop {
        Tstop::from_bits(val)
    }
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(val: Tstop) -> u8 {
        Tstop::to_bits(val)
    }
}
