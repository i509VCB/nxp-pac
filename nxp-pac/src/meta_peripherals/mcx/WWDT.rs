#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Windowed Watchdog Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdt {
    ptr: *mut u8,
}
unsafe impl Send for Wwdt {}
unsafe impl Sync for Wwdt {}
impl Wwdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Mode."]
    #[inline(always)]
    pub const fn mod_(self) -> crate::pac::common::Reg<Mod, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Constant."]
    #[inline(always)]
    pub const fn tc(self) -> crate::pac::common::Reg<Tc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Feed Sequence."]
    #[inline(always)]
    pub const fn feed(self) -> crate::pac::common::Reg<Feed, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer Value."]
    #[inline(always)]
    pub const fn tv(self) -> crate::pac::common::Reg<Tv, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Warning Interrupt Compare Value."]
    #[inline(always)]
    pub const fn warnint(self) -> crate::pac::common::Reg<Warnint, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Window Compare Value."]
    #[inline(always)]
    pub const fn window(self) -> crate::pac::common::Reg<Window, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
#[doc = "Feed Sequence."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Feed(pub u32);
impl Feed {
    #[doc = "Feed Value."]
    #[must_use]
    #[inline(always)]
    pub const fn feed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Feed Value."]
    #[inline(always)]
    pub const fn set_feed(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Feed {
    #[inline(always)]
    fn default() -> Feed {
        Feed(0)
    }
}
impl core::fmt::Debug for Feed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Feed").field("feed", &self.feed()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feed {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Feed {{ feed: {=u8:?} }}", self.feed())
    }
}
#[doc = "Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc = "Watchdog Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wden(&self) -> Wden {
        let val = (self.0 >> 0usize) & 0x01;
        Wden::from_bits(val as u8)
    }
    #[doc = "Watchdog Enable."]
    #[inline(always)]
    pub const fn set_wden(&mut self, val: Wden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog Reset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wdreset(&self) -> Wdreset {
        let val = (self.0 >> 1usize) & 0x01;
        Wdreset::from_bits(val as u8)
    }
    #[doc = "Watchdog Reset Enable."]
    #[inline(always)]
    pub const fn set_wdreset(&mut self, val: Wdreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Watchdog Timeout Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wdtof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog Timeout Flag."]
    #[inline(always)]
    pub const fn set_wdtof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Warning Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn wdint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Warning Interrupt Flag."]
    #[inline(always)]
    pub const fn set_wdint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Watchdog Update Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn wdprotect(&self) -> Wdprotect {
        let val = (self.0 >> 4usize) & 0x01;
        Wdprotect::from_bits(val as u8)
    }
    #[doc = "Watchdog Update Mode."]
    #[inline(always)]
    pub const fn set_wdprotect(&mut self, val: Wdprotect) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Debug Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable."]
    #[inline(always)]
    pub const fn set_debug_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Mod {
    #[inline(always)]
    fn default() -> Mod {
        Mod(0)
    }
}
impl core::fmt::Debug for Mod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mod")
            .field("wden", &self.wden())
            .field("wdreset", &self.wdreset())
            .field("wdtof", &self.wdtof())
            .field("wdint", &self.wdint())
            .field("wdprotect", &self.wdprotect())
            .field("lock", &self.lock())
            .field("debug_en", &self.debug_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mod {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mod {{ wden: {:?}, wdreset: {:?}, wdtof: {=bool:?}, wdint: {=bool:?}, wdprotect: {:?}, lock: {=bool:?}, debug_en: {=bool:?} }}",
            self.wden(),
            self.wdreset(),
            self.wdtof(),
            self.wdint(),
            self.wdprotect(),
            self.lock(),
            self.debug_en()
        )
    }
}
#[doc = "Timer Constant."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tc(pub u32);
impl Tc {
    #[doc = "Watchdog Timeout Value."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog Timeout Value."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tc {
    #[inline(always)]
    fn default() -> Tc {
        Tc(0)
    }
}
impl core::fmt::Debug for Tc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tc").field("count", &self.count()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tc {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Timer Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tv(pub u32);
impl Tv {
    #[doc = "Counter Timer Value."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter Timer Value."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Tv {
    #[inline(always)]
    fn default() -> Tv {
        Tv(0)
    }
}
impl core::fmt::Debug for Tv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tv").field("count", &self.count()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tv {{ count: {=u32:?} }}", self.count())
    }
}
#[doc = "Warning Interrupt Compare Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Warnint(pub u32);
impl Warnint {
    #[doc = "Watchdog Warning Interrupt Compare Value."]
    #[must_use]
    #[inline(always)]
    pub const fn warnint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Watchdog Warning Interrupt Compare Value."]
    #[inline(always)]
    pub const fn set_warnint(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Warnint {
    #[inline(always)]
    fn default() -> Warnint {
        Warnint(0)
    }
}
impl core::fmt::Debug for Warnint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Warnint")
            .field("warnint", &self.warnint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Warnint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Warnint {{ warnint: {=u16:?} }}", self.warnint())
    }
}
#[doc = "Window Compare Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Window(pub u32);
impl Window {
    #[doc = "Watchdog Window Value."]
    #[must_use]
    #[inline(always)]
    pub const fn window(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Watchdog Window Value."]
    #[inline(always)]
    pub const fn set_window(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Window {
    #[inline(always)]
    fn default() -> Window {
        Window(0)
    }
}
impl core::fmt::Debug for Window {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Window")
            .field("window", &self.window())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Window {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Window {{ window: {=u32:?} }}", self.window())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wden {
    #[doc = "Timer stopped."]
    Stop = 0x0,
    #[doc = "Timer running."]
    Run = 0x01,
}
impl Wden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wden {
    #[inline(always)]
    fn from(val: u8) -> Wden {
        Wden::from_bits(val)
    }
}
impl From<Wden> for u8 {
    #[inline(always)]
    fn from(val: Wden) -> u8 {
        Wden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdprotect {
    #[doc = "Flexible."]
    Flexible = 0x0,
    #[doc = "Threshold."]
    Threshold = 0x01,
}
impl Wdprotect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdprotect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdprotect {
    #[inline(always)]
    fn from(val: u8) -> Wdprotect {
        Wdprotect::from_bits(val)
    }
}
impl From<Wdprotect> for u8 {
    #[inline(always)]
    fn from(val: Wdprotect) -> u8 {
        Wdprotect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdreset {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "Reset."]
    Reset = 0x01,
}
impl Wdreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdreset {
    #[inline(always)]
    fn from(val: u8) -> Wdreset {
        Wdreset::from_bits(val)
    }
}
impl From<Wdreset> for u8 {
    #[inline(always)]
    fn from(val: Wdreset) -> u8 {
        Wdreset::to_bits(val)
    }
}
