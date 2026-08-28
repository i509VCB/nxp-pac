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
    #[doc = "Clear Capture 0."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr0(&self) -> Capclr0 {
        let val = (self.0 >> 0usize) & 0x01;
        Capclr0::from_bits(val as u8)
    }
    #[doc = "Clear Capture 0."]
    #[inline(always)]
    pub const fn set_capclr0(&mut self, val: Capclr0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear Capture 1."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr1(&self) -> Capclr1 {
        let val = (self.0 >> 1usize) & 0x01;
        Capclr1::from_bits(val as u8)
    }
    #[doc = "Clear Capture 1."]
    #[inline(always)]
    pub const fn set_capclr1(&mut self, val: Capclr1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear Capture 2."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr2(&self) -> Capclr2 {
        let val = (self.0 >> 2usize) & 0x01;
        Capclr2::from_bits(val as u8)
    }
    #[doc = "Clear Capture 2."]
    #[inline(always)]
    pub const fn set_capclr2(&mut self, val: Capclr2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Capture 3."]
    #[must_use]
    #[inline(always)]
    pub const fn capclr3(&self) -> Capclr3 {
        let val = (self.0 >> 3usize) & 0x01;
        Capclr3::from_bits(val as u8)
    }
    #[doc = "Clear Capture 3."]
    #[inline(always)]
    pub const fn set_capclr3(&mut self, val: Capclr3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
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
            .field("capclr0", &self.capclr0())
            .field("capclr1", &self.capclr1())
            .field("capclr2", &self.capclr2())
            .field("capclr3", &self.capclr3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Capclr {{ capclr0: {:?}, capclr1: {:?}, capclr2: {:?}, capclr3: {:?} }}",
            self.capclr0(),
            self.capclr1(),
            self.capclr2(),
            self.capclr3()
        )
    }
}
#[doc = "Capture Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Enable Capture 0."]
    #[must_use]
    #[inline(always)]
    pub const fn capen0(&self) -> Capen0 {
        let val = (self.0 >> 0usize) & 0x01;
        Capen0::from_bits(val as u8)
    }
    #[doc = "Enable Capture 0."]
    #[inline(always)]
    pub const fn set_capen0(&mut self, val: Capen0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Capture 1."]
    #[must_use]
    #[inline(always)]
    pub const fn capen1(&self) -> Capen1 {
        let val = (self.0 >> 1usize) & 0x01;
        Capen1::from_bits(val as u8)
    }
    #[doc = "Enable Capture 1."]
    #[inline(always)]
    pub const fn set_capen1(&mut self, val: Capen1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Capture 2."]
    #[must_use]
    #[inline(always)]
    pub const fn capen2(&self) -> Capen2 {
        let val = (self.0 >> 2usize) & 0x01;
        Capen2::from_bits(val as u8)
    }
    #[doc = "Enable Capture 2."]
    #[inline(always)]
    pub const fn set_capen2(&mut self, val: Capen2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Capture 3."]
    #[must_use]
    #[inline(always)]
    pub const fn capen3(&self) -> Capen3 {
        let val = (self.0 >> 3usize) & 0x01;
        Capen3::from_bits(val as u8)
    }
    #[doc = "Enable Capture 3."]
    #[inline(always)]
    pub const fn set_capen3(&mut self, val: Capen3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Capture Polarity 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol0(&self) -> Cappol0 {
        let val = (self.0 >> 8usize) & 0x01;
        Cappol0::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 0."]
    #[inline(always)]
    pub const fn set_cappol0(&mut self, val: Cappol0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Capture-Polarity 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol1(&self) -> Cappol1 {
        let val = (self.0 >> 9usize) & 0x01;
        Cappol1::from_bits(val as u8)
    }
    #[doc = "Capture-Polarity 1."]
    #[inline(always)]
    pub const fn set_cappol1(&mut self, val: Cappol1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Capture Polarity 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol2(&self) -> Cappol2 {
        let val = (self.0 >> 10usize) & 0x01;
        Cappol2::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 2."]
    #[inline(always)]
    pub const fn set_cappol2(&mut self, val: Cappol2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Capture Polarity 3."]
    #[must_use]
    #[inline(always)]
    pub const fn cappol3(&self) -> Cappol3 {
        let val = (self.0 >> 11usize) & 0x01;
        Cappol3::from_bits(val as u8)
    }
    #[doc = "Capture Polarity 3."]
    #[inline(always)]
    pub const fn set_cappol3(&mut self, val: Cappol3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
            .field("capen0", &self.capen0())
            .field("capen1", &self.capen1())
            .field("capen2", &self.capen2())
            .field("capen3", &self.capen3())
            .field("cappol0", &self.cappol0())
            .field("cappol1", &self.cappol1())
            .field("cappol2", &self.cappol2())
            .field("cappol3", &self.cappol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg {{ capen0: {:?}, capen1: {:?}, capen2: {:?}, capen3: {:?}, cappol0: {:?}, cappol1: {:?}, cappol2: {:?}, cappol3: {:?} }}",
            self.capen0(),
            self.capen1(),
            self.capen2(),
            self.capen3(),
            self.cappol0(),
            self.cappol1(),
            self.cappol2(),
            self.cappol3()
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
pub enum Capclr0 {
    #[doc = "Does nothing."]
    Capclr0nothing = 0x0,
    #[doc = "Clears the CAP0 register value."]
    Capclr0cleared = 0x01,
}
impl Capclr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr0 {
    #[inline(always)]
    fn from(val: u8) -> Capclr0 {
        Capclr0::from_bits(val)
    }
}
impl From<Capclr0> for u8 {
    #[inline(always)]
    fn from(val: Capclr0) -> u8 {
        Capclr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr1 {
    #[doc = "Does nothing."]
    Capclr1nothing = 0x0,
    #[doc = "Clears the CAP1 register value."]
    Capclr1cleared = 0x01,
}
impl Capclr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr1 {
    #[inline(always)]
    fn from(val: u8) -> Capclr1 {
        Capclr1::from_bits(val)
    }
}
impl From<Capclr1> for u8 {
    #[inline(always)]
    fn from(val: Capclr1) -> u8 {
        Capclr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr2 {
    #[doc = "Does nothing."]
    Capclr2nothing = 0x0,
    #[doc = "Clears the CAP2 register value."]
    Capclr2cleared = 0x01,
}
impl Capclr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr2 {
    #[inline(always)]
    fn from(val: u8) -> Capclr2 {
        Capclr2::from_bits(val)
    }
}
impl From<Capclr2> for u8 {
    #[inline(always)]
    fn from(val: Capclr2) -> u8 {
        Capclr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capclr3 {
    #[doc = "Does nothing."]
    Capclr3nothing = 0x0,
    #[doc = "Clears the CAP3 register value."]
    Capclr3cleared = 0x01,
}
impl Capclr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capclr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capclr3 {
    #[inline(always)]
    fn from(val: u8) -> Capclr3 {
        Capclr3::from_bits(val)
    }
}
impl From<Capclr3> for u8 {
    #[inline(always)]
    fn from(val: Capclr3) -> u8 {
        Capclr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen0 {
    #[doc = "Disable."]
    Capen0isdisabled = 0x0,
    #[doc = "Enable."]
    Capen0isenabled = 0x01,
}
impl Capen0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen0 {
    #[inline(always)]
    fn from(val: u8) -> Capen0 {
        Capen0::from_bits(val)
    }
}
impl From<Capen0> for u8 {
    #[inline(always)]
    fn from(val: Capen0) -> u8 {
        Capen0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen1 {
    #[doc = "Disable."]
    Capen1isdisabled = 0x0,
    #[doc = "Enable."]
    Capen1isenabled = 0x01,
}
impl Capen1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen1 {
    #[inline(always)]
    fn from(val: u8) -> Capen1 {
        Capen1::from_bits(val)
    }
}
impl From<Capen1> for u8 {
    #[inline(always)]
    fn from(val: Capen1) -> u8 {
        Capen1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen2 {
    #[doc = "Disable."]
    Capen2isdisabled = 0x0,
    #[doc = "Enable."]
    Capen2isenabled = 0x01,
}
impl Capen2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen2 {
    #[inline(always)]
    fn from(val: u8) -> Capen2 {
        Capen2::from_bits(val)
    }
}
impl From<Capen2> for u8 {
    #[inline(always)]
    fn from(val: Capen2) -> u8 {
        Capen2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Capen3 {
    #[doc = "Disable."]
    Capen3isdisabled = 0x0,
    #[doc = "Enable."]
    Capen3isenabled = 0x01,
}
impl Capen3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Capen3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Capen3 {
    #[inline(always)]
    fn from(val: u8) -> Capen3 {
        Capen3::from_bits(val)
    }
}
impl From<Capen3> for u8 {
    #[inline(always)]
    fn from(val: Capen3) -> u8 {
        Capen3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol0 {
    #[doc = "Positive."]
    Cappol0posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol0negedgecapture = 0x01,
}
impl Cappol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol0 {
    #[inline(always)]
    fn from(val: u8) -> Cappol0 {
        Cappol0::from_bits(val)
    }
}
impl From<Cappol0> for u8 {
    #[inline(always)]
    fn from(val: Cappol0) -> u8 {
        Cappol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol1 {
    #[doc = "Positive."]
    Cappol1posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol1negedgecapture = 0x01,
}
impl Cappol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol1 {
    #[inline(always)]
    fn from(val: u8) -> Cappol1 {
        Cappol1::from_bits(val)
    }
}
impl From<Cappol1> for u8 {
    #[inline(always)]
    fn from(val: Cappol1) -> u8 {
        Cappol1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol2 {
    #[doc = "Positive."]
    Cappol2posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol2negedgecapture = 0x01,
}
impl Cappol2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol2 {
    #[inline(always)]
    fn from(val: u8) -> Cappol2 {
        Cappol2::from_bits(val)
    }
}
impl From<Cappol2> for u8 {
    #[inline(always)]
    fn from(val: Cappol2) -> u8 {
        Cappol2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cappol3 {
    #[doc = "Positive."]
    Cappol3posedgecapture = 0x0,
    #[doc = "Negative."]
    Cappol3negedgecapture = 0x01,
}
impl Cappol3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cappol3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cappol3 {
    #[inline(always)]
    fn from(val: u8) -> Cappol3 {
        Cappol3::from_bits(val)
    }
}
impl From<Cappol3> for u8 {
    #[inline(always)]
    fn from(val: Cappol3) -> u8 {
        Cappol3::to_bits(val)
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
