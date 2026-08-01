#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "UTICK."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utick {
    ptr: *mut u8,
}
unsafe impl Send for Utick {}
unsafe impl Sync for Utick {}
impl Utick {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::pac::common::Reg<Ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Capture Configuration."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::pac::common::Reg<Cfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Capture Clear."]
    #[inline(always)]
    pub const fn capclr(self) -> crate::pac::common::Reg<Capclr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Capture."]
    #[inline(always)]
    pub const fn cap(self, n: usize) -> crate::pac::common::Reg<Cap, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
}
#[doc = "Capture."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cap(pub u32);
impl Cap {
    #[doc = "Captured Value for the Related Capture Event."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Captured Value for the Related Capture Event."]
    #[inline(always)]
    pub const fn set_cap_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Captured Value Valid Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Captured Value Valid Flag."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("cap_value", &self.cap_value())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cap {{ cap_value: {=u32:?}, valid: {=bool:?} }}",
            self.cap_value(),
            self.valid()
        )
    }
}
#[doc = "Capture Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capclr(pub u32);
impl Capclr {
    #[doc = "Clear Capture."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr(&self, n: usize) -> CapclrVal {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        CapclrVal::from_bits(val as u8)
    }
    #[doc = "Clear Capture."]
    #[inline(always)]
    pub const fn set_capclr(&mut self, n: usize, val: CapclrVal) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Capclr {
    #[inline(always)]
    fn default() -> Capclr {
        Capclr(0)
    }
}
impl core::fmt::Debug for Capclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capclr")
            .field("capclr[0]", &self.capclr(0usize))
            .field("capclr[1]", &self.capclr(1usize))
            .field("capclr[2]", &self.capclr(2usize))
            .field("capclr[3]", &self.capclr(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capclr {{ capclr[0]: {:?}, capclr[1]: {:?}, capclr[2]: {:?}, capclr[3]: {:?} }}",
            self.capclr(0usize),
            self.capclr(1usize),
            self.capclr(2usize),
            self.capclr(3usize)
        )
    }
}
#[doc = "Capture Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Enable Capture."]
    #[must_use]
    #[inline(always)]
    pub const fn capen(&self, n: usize) -> Capen {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Capen::from_bits(val as u8)
    }
    #[doc = "Enable Capture."]
    #[inline(always)]
    pub const fn set_capen(&mut self, n: usize, val: Capen) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Capture Polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol(&self, n: usize) -> Cappol {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Cappol::from_bits(val as u8)
    }
    #[doc = "Capture Polarity."]
    #[inline(always)]
    pub const fn set_cappol(&mut self, n: usize, val: Cappol) {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("capen[0]", &self.capen(0usize))
            .field("capen[1]", &self.capen(1usize))
            .field("capen[2]", &self.capen(2usize))
            .field("capen[3]", &self.capen(3usize))
            .field("cappol[0]", &self.cappol(0usize))
            .field("cappol[1]", &self.cappol(1usize))
            .field("cappol[2]", &self.cappol(2usize))
            .field("cappol[3]", &self.cappol(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ capen[0]: {:?}, capen[1]: {:?}, capen[2]: {:?}, capen[3]: {:?}, cappol[0]: {:?}, cappol[1]: {:?}, cappol[2]: {:?}, cappol[3]: {:?} }}",
            self.capen(0usize),
            self.capen(1usize),
            self.capen(2usize),
            self.capen(3usize),
            self.cappol(0usize),
            self.cappol(1usize),
            self.cappol(2usize),
            self.cappol(3usize)
        )
    }
}
#[doc = "Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Tick Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn delayval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "Tick Interval."]
    #[inline(always)]
    pub const fn set_delayval(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "Repeat Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn repeat(&self) -> Repeat {
        let val = (self.0 >> 31usize) & 0x01;
        Repeat::from_bits(val as u8)
    }
    #[doc = "Repeat Delay."]
    #[inline(always)]
    pub const fn set_repeat(&mut self, val: Repeat) {
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
            .field("delayval", &self.delayval())
            .field("repeat", &self.repeat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl {{ delayval: {=u32:?}, repeat: {:?} }}",
            self.delayval(),
            self.repeat()
        )
    }
}
#[doc = "Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn intr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Flag."]
    #[inline(always)]
    pub const fn set_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Active Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 1usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "Timer Active Flag."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("intr", &self.intr())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ intr: {=bool:?}, active: {:?} }}",
            self.intr(),
            self.active()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "Inactive (stopped)."]
    Timerisnotactive = 0x0,
    #[doc = "Active."]
    Timerisactive = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapclrVal {
    #[doc = "Does nothing."]
    Capclr0 = 0x0,
    #[doc = "Clears the CAP register value."]
    Capclr1 = 0x01,
}
impl CapclrVal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CapclrVal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CapclrVal {
    #[inline(always)]
    fn from(val: u8) -> CapclrVal {
        CapclrVal::from_bits(val)
    }
}
impl From<CapclrVal> for u8 {
    #[inline(always)]
    fn from(val: CapclrVal) -> u8 {
        CapclrVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen {
    #[doc = "Disable."]
    Capen0 = 0x0,
    #[doc = "Enable."]
    Capen1 = 0x01,
}
impl Capen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen {
    #[inline(always)]
    fn from(val: u8) -> Capen {
        Capen::from_bits(val)
    }
}
impl From<Capen> for u8 {
    #[inline(always)]
    fn from(val: Capen) -> u8 {
        Capen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol {
    #[doc = "Positive."]
    Cappol0 = 0x0,
    #[doc = "Negative."]
    Cappol1 = 0x01,
}
impl Cappol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol {
    #[inline(always)]
    fn from(val: u8) -> Cappol {
        Cappol::from_bits(val)
    }
}
impl From<Cappol> for u8 {
    #[inline(always)]
    fn from(val: Cappol) -> u8 {
        Cappol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repeat {
    #[doc = "One-time delay."]
    Delayonce = 0x0,
    #[doc = "Delay repeats continuously."]
    Delayrepeats = 0x01,
}
impl Repeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repeat {
    #[inline(always)]
    fn from(val: u8) -> Repeat {
        Repeat::from_bits(val)
    }
}
impl From<Repeat> for u8 {
    #[inline(always)]
    fn from(val: Repeat) -> u8 {
        Repeat::to_bits(val)
    }
}
