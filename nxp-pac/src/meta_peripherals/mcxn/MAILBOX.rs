#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: IRQ, IRQSET, IRQCLR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq {
    ptr: *mut u8,
}
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "name (CPUn) Interrupt."]
    #[inline(always)]
    pub const fn irq(self) -> crate::pac::common::Reg<IrqReg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "name (CPUn) Interrupt Set."]
    #[inline(always)]
    pub const fn irqset(self) -> crate::pac::common::Reg<Irqset, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "name (CPUn) Interrupt Clear."]
    #[inline(always)]
    pub const fn irqclr(self) -> crate::pac::common::Reg<Irqclr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
#[doc = "MAILBOX."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mailbox {
    ptr: *mut u8,
}
unsafe impl Send for Mailbox {}
unsafe impl Sync for Mailbox {}
impl Mailbox {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: IRQ, IRQSET, IRQCLR."]
    #[inline(always)]
    pub const fn irq(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "Mutual Exclusion."]
    #[inline(always)]
    pub const fn mutex(self) -> crate::pac::common::Reg<Mutex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
}
#[doc = "name (CPUn) Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqReg(pub u32);
impl IrqReg {
    #[doc = "Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn intreq(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Request."]
    #[inline(always)]
    pub const fn set_intreq(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for IrqReg {
    #[inline(always)]
    fn default() -> IrqReg {
        IrqReg(0)
    }
}
impl core::fmt::Debug for IrqReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrqReg")
            .field("intreq", &self.intreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrqReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IrqReg {{ intreq: {=u32:?} }}", self.intreq())
    }
}
#[doc = "name (CPUn) Interrupt Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqclr(pub u32);
impl Irqclr {
    #[doc = "Interrupt Request Clear 0."]
    #[must_use]
    #[inline(always)]
    pub const fn intreqclr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Request Clear 0."]
    #[inline(always)]
    pub const fn set_intreqclr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Irqclr {
    #[inline(always)]
    fn default() -> Irqclr {
        Irqclr(0)
    }
}
impl core::fmt::Debug for Irqclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqclr")
            .field("intreqclr", &self.intreqclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqclr {{ intreqclr: {=u32:?} }}", self.intreqclr())
    }
}
#[doc = "name (CPUn) Interrupt Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqset(pub u32);
impl Irqset {
    #[doc = "Interrupt Request Set 0."]
    #[must_use]
    #[inline(always)]
    pub const fn intreqset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt Request Set 0."]
    #[inline(always)]
    pub const fn set_intreqset(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Irqset {
    #[inline(always)]
    fn default() -> Irqset {
        Irqset(0)
    }
}
impl core::fmt::Debug for Irqset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irqset")
            .field("intreqset", &self.intreqset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irqset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Irqset {{ intreqset: {=u32:?} }}", self.intreqset())
    }
}
#[doc = "Mutual Exclusion."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mutex(pub u32);
impl Mutex {
    #[doc = "Mutual Exclusion Request."]
    #[must_use]
    #[inline(always)]
    pub const fn ex(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mutual Exclusion Request."]
    #[inline(always)]
    pub const fn set_ex(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Mutex {
    #[inline(always)]
    fn default() -> Mutex {
        Mutex(0)
    }
}
impl core::fmt::Debug for Mutex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mutex").field("ex", &self.ex()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mutex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Mutex {{ ex: {=bool:?} }}", self.ex())
    }
}
