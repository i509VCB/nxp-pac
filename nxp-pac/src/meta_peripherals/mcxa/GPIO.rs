#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "GPIO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
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
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn lock(self) -> crate::pac::common::Reg<Lock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pin Control Nonsecure."]
    #[inline(always)]
    pub const fn pcns(self) -> crate::pac::common::Reg<Pcns, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Control Nonsecure."]
    #[inline(always)]
    pub const fn icns(self) -> crate::pac::common::Reg<Icns, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pin Control Nonprivilege."]
    #[inline(always)]
    pub const fn pcnp(self) -> crate::pac::common::Reg<Pcnp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Interrupt Control Nonprivilege."]
    #[inline(always)]
    pub const fn icnp(self) -> crate::pac::common::Reg<Icnp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn pdor(self) -> crate::pac::common::Reg<Pdor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn psor(self) -> crate::pac::common::Reg<Psor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn pcor(self) -> crate::pac::common::Reg<Pcor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn ptor(self) -> crate::pac::common::Reg<Ptor, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn pdir(self) -> crate::pac::common::Reg<Pdir, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn pddr(self) -> crate::pac::common::Reg<Pddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn pidr(self) -> crate::pac::common::Reg<Pidr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Pin Data."]
    #[inline(always)]
    pub const fn pdr(self, n: usize) -> crate::pac::common::Reg<Pdr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 1usize) as _)
        }
    }
    #[doc = "Interrupt Control index."]
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::pac::common::Reg<Icr, crate::pac::common::RW> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "Global Interrupt Control Low."]
    #[inline(always)]
    pub const fn giclr(self) -> crate::pac::common::Reg<Giclr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Global Interrupt Control High."]
    #[inline(always)]
    pub const fn gichr(self) -> crate::pac::common::Reg<Gichr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn isfr(self, n: usize) -> crate::pac::common::Reg<Isfr, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
}
#[doc = "Global Interrupt Control High."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gichr(pub u32);
impl Gichr {
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Global Interrupt Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data."]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Gichr {
    #[inline(always)]
    fn default() -> Gichr {
        Gichr(0)
    }
}
impl core::fmt::Debug for Gichr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gichr")
            .field("giwe[0]", &self.giwe(0usize))
            .field("giwe[1]", &self.giwe(1usize))
            .field("giwe[2]", &self.giwe(2usize))
            .field("giwe[3]", &self.giwe(3usize))
            .field("giwe[4]", &self.giwe(4usize))
            .field("giwe[5]", &self.giwe(5usize))
            .field("giwe[6]", &self.giwe(6usize))
            .field("giwe[7]", &self.giwe(7usize))
            .field("giwe[8]", &self.giwe(8usize))
            .field("giwe[9]", &self.giwe(9usize))
            .field("giwe[10]", &self.giwe(10usize))
            .field("giwe[11]", &self.giwe(11usize))
            .field("giwe[12]", &self.giwe(12usize))
            .field("giwe[13]", &self.giwe(13usize))
            .field("giwe[14]", &self.giwe(14usize))
            .field("giwe[15]", &self.giwe(15usize))
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gichr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gichr {{ giwe[0]: {=bool:?}, giwe[1]: {=bool:?}, giwe[2]: {=bool:?}, giwe[3]: {=bool:?}, giwe[4]: {=bool:?}, giwe[5]: {=bool:?}, giwe[6]: {=bool:?}, giwe[7]: {=bool:?}, giwe[8]: {=bool:?}, giwe[9]: {=bool:?}, giwe[10]: {=bool:?}, giwe[11]: {=bool:?}, giwe[12]: {=bool:?}, giwe[13]: {=bool:?}, giwe[14]: {=bool:?}, giwe[15]: {=bool:?}, giwd: {=u16:?} }}",
            self.giwe(0usize),
            self.giwe(1usize),
            self.giwe(2usize),
            self.giwe(3usize),
            self.giwe(4usize),
            self.giwe(5usize),
            self.giwe(6usize),
            self.giwe(7usize),
            self.giwe(8usize),
            self.giwe(9usize),
            self.giwe(10usize),
            self.giwe(11usize),
            self.giwe(12usize),
            self.giwe(13usize),
            self.giwe(14usize),
            self.giwe(15usize),
            self.giwd()
        )
    }
}
#[doc = "Global Interrupt Control Low."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Giclr(pub u32);
impl Giclr {
    #[doc = "Global Interrupt Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn giwe(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt Write Enable."]
    #[inline(always)]
    pub const fn set_giwe(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Global Interrupt Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn giwd(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Global Interrupt Write Data."]
    #[inline(always)]
    pub const fn set_giwd(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Giclr {
    #[inline(always)]
    fn default() -> Giclr {
        Giclr(0)
    }
}
impl core::fmt::Debug for Giclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Giclr")
            .field("giwe[0]", &self.giwe(0usize))
            .field("giwe[1]", &self.giwe(1usize))
            .field("giwe[2]", &self.giwe(2usize))
            .field("giwe[3]", &self.giwe(3usize))
            .field("giwe[4]", &self.giwe(4usize))
            .field("giwe[5]", &self.giwe(5usize))
            .field("giwe[6]", &self.giwe(6usize))
            .field("giwe[7]", &self.giwe(7usize))
            .field("giwe[8]", &self.giwe(8usize))
            .field("giwe[9]", &self.giwe(9usize))
            .field("giwe[10]", &self.giwe(10usize))
            .field("giwe[11]", &self.giwe(11usize))
            .field("giwe[12]", &self.giwe(12usize))
            .field("giwe[13]", &self.giwe(13usize))
            .field("giwe[14]", &self.giwe(14usize))
            .field("giwe[15]", &self.giwe(15usize))
            .field("giwd", &self.giwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Giclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Giclr {{ giwe[0]: {=bool:?}, giwe[1]: {=bool:?}, giwe[2]: {=bool:?}, giwe[3]: {=bool:?}, giwe[4]: {=bool:?}, giwe[5]: {=bool:?}, giwe[6]: {=bool:?}, giwe[7]: {=bool:?}, giwe[8]: {=bool:?}, giwe[9]: {=bool:?}, giwe[10]: {=bool:?}, giwe[11]: {=bool:?}, giwe[12]: {=bool:?}, giwe[13]: {=bool:?}, giwe[14]: {=bool:?}, giwe[15]: {=bool:?}, giwd: {=u16:?} }}",
            self.giwe(0usize),
            self.giwe(1usize),
            self.giwe(2usize),
            self.giwe(3usize),
            self.giwe(4usize),
            self.giwe(5usize),
            self.giwe(6usize),
            self.giwe(7usize),
            self.giwe(8usize),
            self.giwe(9usize),
            self.giwe(10usize),
            self.giwe(11usize),
            self.giwe(12usize),
            self.giwe(13usize),
            self.giwe(14usize),
            self.giwe(15usize),
            self.giwd()
        )
    }
}
#[doc = "Interrupt Control Nonprivilege."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnp(pub u32);
impl Icnp {
    #[doc = "Nonprivilege Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn npe0(&self) -> IcnpNpe0 {
        let val = (self.0 >> 0usize) & 0x01;
        IcnpNpe0::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable."]
    #[inline(always)]
    pub const fn set_npe0(&mut self, val: IcnpNpe0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonprivilege Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn npe1(&self) -> IcnpNpe1 {
        let val = (self.0 >> 1usize) & 0x01;
        IcnpNpe1::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable."]
    #[inline(always)]
    pub const fn set_npe1(&mut self, val: IcnpNpe1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Icnp {
    #[inline(always)]
    fn default() -> Icnp {
        Icnp(0)
    }
}
impl core::fmt::Debug for Icnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icnp")
            .field("npe0", &self.npe0())
            .field("npe1", &self.npe1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icnp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icnp {{ npe0: {:?}, npe1: {:?} }}",
            self.npe0(),
            self.npe1()
        )
    }
}
#[doc = "Interrupt Control Nonsecure."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icns(pub u32);
impl Icns {
    #[doc = "Nonsecure Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nse0(&self) -> IcnsNse0 {
        let val = (self.0 >> 0usize) & 0x01;
        IcnsNse0::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable."]
    #[inline(always)]
    pub const fn set_nse0(&mut self, val: IcnsNse0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Nonsecure Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nse1(&self) -> IcnsNse1 {
        let val = (self.0 >> 1usize) & 0x01;
        IcnsNse1::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable."]
    #[inline(always)]
    pub const fn set_nse1(&mut self, val: IcnsNse1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Icns {
    #[inline(always)]
    fn default() -> Icns {
        Icns(0)
    }
}
impl core::fmt::Debug for Icns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icns")
            .field("nse0", &self.nse0())
            .field("nse1", &self.nse1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icns {{ nse0: {:?}, nse1: {:?} }}",
            self.nse0(),
            self.nse1()
        )
    }
}
#[doc = "Interrupt Control index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Interrupt Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn irqc(&self) -> Irqc {
        let val = (self.0 >> 16usize) & 0x0f;
        Irqc::from_bits(val as u8)
    }
    #[doc = "Interrupt Configuration."]
    #[inline(always)]
    pub const fn set_irqc(&mut self, val: Irqc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Interrupt Select."]
    #[must_use]
    #[inline(always)]
    pub const fn irqs(&self) -> Irqs {
        let val = (self.0 >> 20usize) & 0x01;
        Irqs::from_bits(val as u8)
    }
    #[doc = "Interrupt Select."]
    #[inline(always)]
    pub const fn set_irqs(&mut self, val: Irqs) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn lk(&self) -> Lk {
        let val = (self.0 >> 23usize) & 0x01;
        Lk::from_bits(val as u8)
    }
    #[doc = "Lock."]
    #[inline(always)]
    pub const fn set_lk(&mut self, val: Lk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf(&self) -> Isf {
        let val = (self.0 >> 24usize) & 0x01;
        Isf::from_bits(val as u8)
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf(&mut self, val: Isf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr")
            .field("irqc", &self.irqc())
            .field("irqs", &self.irqs())
            .field("lk", &self.lk())
            .field("isf", &self.isf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Icr {{ irqc: {:?}, irqs: {:?}, lk: {:?}, isf: {:?} }}",
            self.irqc(),
            self.irqs(),
            self.lk(),
            self.isf()
        )
    }
}
#[doc = "Interrupt Status Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfr(pub u32);
impl Isfr {
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Interrupt Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn isf31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status Flag."]
    #[inline(always)]
    pub const fn set_isf31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isfr {
    #[inline(always)]
    fn default() -> Isfr {
        Isfr(0)
    }
}
impl core::fmt::Debug for Isfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfr")
            .field("isf0", &self.isf0())
            .field("isf1", &self.isf1())
            .field("isf2", &self.isf2())
            .field("isf3", &self.isf3())
            .field("isf4", &self.isf4())
            .field("isf5", &self.isf5())
            .field("isf6", &self.isf6())
            .field("isf7", &self.isf7())
            .field("isf8", &self.isf8())
            .field("isf9", &self.isf9())
            .field("isf10", &self.isf10())
            .field("isf11", &self.isf11())
            .field("isf12", &self.isf12())
            .field("isf13", &self.isf13())
            .field("isf14", &self.isf14())
            .field("isf15", &self.isf15())
            .field("isf16", &self.isf16())
            .field("isf17", &self.isf17())
            .field("isf18", &self.isf18())
            .field("isf19", &self.isf19())
            .field("isf20", &self.isf20())
            .field("isf21", &self.isf21())
            .field("isf22", &self.isf22())
            .field("isf23", &self.isf23())
            .field("isf24", &self.isf24())
            .field("isf25", &self.isf25())
            .field("isf26", &self.isf26())
            .field("isf27", &self.isf27())
            .field("isf28", &self.isf28())
            .field("isf29", &self.isf29())
            .field("isf30", &self.isf30())
            .field("isf31", &self.isf31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Isfr {{ isf0: {=bool:?}, isf1: {=bool:?}, isf2: {=bool:?}, isf3: {=bool:?}, isf4: {=bool:?}, isf5: {=bool:?}, isf6: {=bool:?}, isf7: {=bool:?}, isf8: {=bool:?}, isf9: {=bool:?}, isf10: {=bool:?}, isf11: {=bool:?}, isf12: {=bool:?}, isf13: {=bool:?}, isf14: {=bool:?}, isf15: {=bool:?}, isf16: {=bool:?}, isf17: {=bool:?}, isf18: {=bool:?}, isf19: {=bool:?}, isf20: {=bool:?}, isf21: {=bool:?}, isf22: {=bool:?}, isf23: {=bool:?}, isf24: {=bool:?}, isf25: {=bool:?}, isf26: {=bool:?}, isf27: {=bool:?}, isf28: {=bool:?}, isf29: {=bool:?}, isf30: {=bool:?}, isf31: {=bool:?} }}",
            self.isf0(),
            self.isf1(),
            self.isf2(),
            self.isf3(),
            self.isf4(),
            self.isf5(),
            self.isf6(),
            self.isf7(),
            self.isf8(),
            self.isf9(),
            self.isf10(),
            self.isf11(),
            self.isf12(),
            self.isf13(),
            self.isf14(),
            self.isf15(),
            self.isf16(),
            self.isf17(),
            self.isf18(),
            self.isf19(),
            self.isf20(),
            self.isf21(),
            self.isf22(),
            self.isf23(),
            self.isf24(),
            self.isf25(),
            self.isf26(),
            self.isf27(),
            self.isf28(),
            self.isf29(),
            self.isf30(),
            self.isf31()
        )
    }
}
#[doc = "Lock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Lock PCNS."]
    #[must_use]
    #[inline(always)]
    pub const fn pcns(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Lock PCNS."]
    #[inline(always)]
    pub const fn set_pcns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock ICNS."]
    #[must_use]
    #[inline(always)]
    pub const fn icns(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock ICNS."]
    #[inline(always)]
    pub const fn set_icns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Lock PCNP."]
    #[must_use]
    #[inline(always)]
    pub const fn pcnp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Lock PCNP."]
    #[inline(always)]
    pub const fn set_pcnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock ICNP."]
    #[must_use]
    #[inline(always)]
    pub const fn icnp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Lock ICNP."]
    #[inline(always)]
    pub const fn set_icnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("pcns", &self.pcns())
            .field("icns", &self.icns())
            .field("pcnp", &self.pcnp())
            .field("icnp", &self.icnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lock {{ pcns: {=bool:?}, icns: {=bool:?}, pcnp: {=bool:?}, icnp: {=bool:?} }}",
            self.pcns(),
            self.icns(),
            self.pcnp(),
            self.icnp()
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Interrupt Number."]
    #[must_use]
    #[inline(always)]
    pub const fn irqnum(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Interrupt Number."]
    #[inline(always)]
    pub const fn set_irqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
            .field("irqnum", &self.irqnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Param {{ irqnum: {=u8:?} }}", self.irqnum())
    }
}
#[doc = "Pin Control Nonprivilege."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcnp(pub u32);
impl Pcnp {
    #[doc = "Nonprivilege Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn npe(&self, n: usize) -> PcnpNpe0 {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        PcnpNpe0::from_bits(val as u8)
    }
    #[doc = "Nonprivilege Enable."]
    #[inline(always)]
    pub const fn set_npe(&mut self, n: usize, val: PcnpNpe0) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pcnp {
    #[inline(always)]
    fn default() -> Pcnp {
        Pcnp(0)
    }
}
impl core::fmt::Debug for Pcnp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcnp")
            .field("npe[0]", &self.npe(0usize))
            .field("npe[1]", &self.npe(1usize))
            .field("npe[2]", &self.npe(2usize))
            .field("npe[3]", &self.npe(3usize))
            .field("npe[4]", &self.npe(4usize))
            .field("npe[5]", &self.npe(5usize))
            .field("npe[6]", &self.npe(6usize))
            .field("npe[7]", &self.npe(7usize))
            .field("npe[8]", &self.npe(8usize))
            .field("npe[9]", &self.npe(9usize))
            .field("npe[10]", &self.npe(10usize))
            .field("npe[11]", &self.npe(11usize))
            .field("npe[12]", &self.npe(12usize))
            .field("npe[13]", &self.npe(13usize))
            .field("npe[14]", &self.npe(14usize))
            .field("npe[15]", &self.npe(15usize))
            .field("npe[16]", &self.npe(16usize))
            .field("npe[17]", &self.npe(17usize))
            .field("npe[18]", &self.npe(18usize))
            .field("npe[19]", &self.npe(19usize))
            .field("npe[20]", &self.npe(20usize))
            .field("npe[21]", &self.npe(21usize))
            .field("npe[22]", &self.npe(22usize))
            .field("npe[23]", &self.npe(23usize))
            .field("npe[24]", &self.npe(24usize))
            .field("npe[25]", &self.npe(25usize))
            .field("npe[26]", &self.npe(26usize))
            .field("npe[27]", &self.npe(27usize))
            .field("npe[28]", &self.npe(28usize))
            .field("npe[29]", &self.npe(29usize))
            .field("npe[30]", &self.npe(30usize))
            .field("npe[31]", &self.npe(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcnp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcnp {{ npe[0]: {:?}, npe[1]: {:?}, npe[2]: {:?}, npe[3]: {:?}, npe[4]: {:?}, npe[5]: {:?}, npe[6]: {:?}, npe[7]: {:?}, npe[8]: {:?}, npe[9]: {:?}, npe[10]: {:?}, npe[11]: {:?}, npe[12]: {:?}, npe[13]: {:?}, npe[14]: {:?}, npe[15]: {:?}, npe[16]: {:?}, npe[17]: {:?}, npe[18]: {:?}, npe[19]: {:?}, npe[20]: {:?}, npe[21]: {:?}, npe[22]: {:?}, npe[23]: {:?}, npe[24]: {:?}, npe[25]: {:?}, npe[26]: {:?}, npe[27]: {:?}, npe[28]: {:?}, npe[29]: {:?}, npe[30]: {:?}, npe[31]: {:?} }}",
            self.npe(0usize),
            self.npe(1usize),
            self.npe(2usize),
            self.npe(3usize),
            self.npe(4usize),
            self.npe(5usize),
            self.npe(6usize),
            self.npe(7usize),
            self.npe(8usize),
            self.npe(9usize),
            self.npe(10usize),
            self.npe(11usize),
            self.npe(12usize),
            self.npe(13usize),
            self.npe(14usize),
            self.npe(15usize),
            self.npe(16usize),
            self.npe(17usize),
            self.npe(18usize),
            self.npe(19usize),
            self.npe(20usize),
            self.npe(21usize),
            self.npe(22usize),
            self.npe(23usize),
            self.npe(24usize),
            self.npe(25usize),
            self.npe(26usize),
            self.npe(27usize),
            self.npe(28usize),
            self.npe(29usize),
            self.npe(30usize),
            self.npe(31usize)
        )
    }
}
#[doc = "Pin Control Nonsecure."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcns(pub u32);
impl Pcns {
    #[doc = "Nonsecure Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nse(&self, n: usize) -> PcnsNse0 {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        PcnsNse0::from_bits(val as u8)
    }
    #[doc = "Nonsecure Enable."]
    #[inline(always)]
    pub const fn set_nse(&mut self, n: usize, val: PcnsNse0) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pcns {
    #[inline(always)]
    fn default() -> Pcns {
        Pcns(0)
    }
}
impl core::fmt::Debug for Pcns {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcns")
            .field("nse[0]", &self.nse(0usize))
            .field("nse[1]", &self.nse(1usize))
            .field("nse[2]", &self.nse(2usize))
            .field("nse[3]", &self.nse(3usize))
            .field("nse[4]", &self.nse(4usize))
            .field("nse[5]", &self.nse(5usize))
            .field("nse[6]", &self.nse(6usize))
            .field("nse[7]", &self.nse(7usize))
            .field("nse[8]", &self.nse(8usize))
            .field("nse[9]", &self.nse(9usize))
            .field("nse[10]", &self.nse(10usize))
            .field("nse[11]", &self.nse(11usize))
            .field("nse[12]", &self.nse(12usize))
            .field("nse[13]", &self.nse(13usize))
            .field("nse[14]", &self.nse(14usize))
            .field("nse[15]", &self.nse(15usize))
            .field("nse[16]", &self.nse(16usize))
            .field("nse[17]", &self.nse(17usize))
            .field("nse[18]", &self.nse(18usize))
            .field("nse[19]", &self.nse(19usize))
            .field("nse[20]", &self.nse(20usize))
            .field("nse[21]", &self.nse(21usize))
            .field("nse[22]", &self.nse(22usize))
            .field("nse[23]", &self.nse(23usize))
            .field("nse[24]", &self.nse(24usize))
            .field("nse[25]", &self.nse(25usize))
            .field("nse[26]", &self.nse(26usize))
            .field("nse[27]", &self.nse(27usize))
            .field("nse[28]", &self.nse(28usize))
            .field("nse[29]", &self.nse(29usize))
            .field("nse[30]", &self.nse(30usize))
            .field("nse[31]", &self.nse(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcns {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcns {{ nse[0]: {:?}, nse[1]: {:?}, nse[2]: {:?}, nse[3]: {:?}, nse[4]: {:?}, nse[5]: {:?}, nse[6]: {:?}, nse[7]: {:?}, nse[8]: {:?}, nse[9]: {:?}, nse[10]: {:?}, nse[11]: {:?}, nse[12]: {:?}, nse[13]: {:?}, nse[14]: {:?}, nse[15]: {:?}, nse[16]: {:?}, nse[17]: {:?}, nse[18]: {:?}, nse[19]: {:?}, nse[20]: {:?}, nse[21]: {:?}, nse[22]: {:?}, nse[23]: {:?}, nse[24]: {:?}, nse[25]: {:?}, nse[26]: {:?}, nse[27]: {:?}, nse[28]: {:?}, nse[29]: {:?}, nse[30]: {:?}, nse[31]: {:?} }}",
            self.nse(0usize),
            self.nse(1usize),
            self.nse(2usize),
            self.nse(3usize),
            self.nse(4usize),
            self.nse(5usize),
            self.nse(6usize),
            self.nse(7usize),
            self.nse(8usize),
            self.nse(9usize),
            self.nse(10usize),
            self.nse(11usize),
            self.nse(12usize),
            self.nse(13usize),
            self.nse(14usize),
            self.nse(15usize),
            self.nse(16usize),
            self.nse(17usize),
            self.nse(18usize),
            self.nse(19usize),
            self.nse(20usize),
            self.nse(21usize),
            self.nse(22usize),
            self.nse(23usize),
            self.nse(24usize),
            self.nse(25usize),
            self.nse(26usize),
            self.nse(27usize),
            self.nse(28usize),
            self.nse(29usize),
            self.nse(30usize),
            self.nse(31usize)
        )
    }
}
#[doc = "Port Clear Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc = "Port Clear Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptco(&self, n: usize) -> Ptco {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Ptco::from_bits(val as u8)
    }
    #[doc = "Port Clear Output."]
    #[inline(always)]
    pub const fn set_ptco(&mut self, n: usize, val: Ptco) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pcor {
    #[inline(always)]
    fn default() -> Pcor {
        Pcor(0)
    }
}
impl core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcor")
            .field("ptco[0]", &self.ptco(0usize))
            .field("ptco[1]", &self.ptco(1usize))
            .field("ptco[2]", &self.ptco(2usize))
            .field("ptco[3]", &self.ptco(3usize))
            .field("ptco[4]", &self.ptco(4usize))
            .field("ptco[5]", &self.ptco(5usize))
            .field("ptco[6]", &self.ptco(6usize))
            .field("ptco[7]", &self.ptco(7usize))
            .field("ptco[8]", &self.ptco(8usize))
            .field("ptco[9]", &self.ptco(9usize))
            .field("ptco[10]", &self.ptco(10usize))
            .field("ptco[11]", &self.ptco(11usize))
            .field("ptco[12]", &self.ptco(12usize))
            .field("ptco[13]", &self.ptco(13usize))
            .field("ptco[14]", &self.ptco(14usize))
            .field("ptco[15]", &self.ptco(15usize))
            .field("ptco[16]", &self.ptco(16usize))
            .field("ptco[17]", &self.ptco(17usize))
            .field("ptco[18]", &self.ptco(18usize))
            .field("ptco[19]", &self.ptco(19usize))
            .field("ptco[20]", &self.ptco(20usize))
            .field("ptco[21]", &self.ptco(21usize))
            .field("ptco[22]", &self.ptco(22usize))
            .field("ptco[23]", &self.ptco(23usize))
            .field("ptco[24]", &self.ptco(24usize))
            .field("ptco[25]", &self.ptco(25usize))
            .field("ptco[26]", &self.ptco(26usize))
            .field("ptco[27]", &self.ptco(27usize))
            .field("ptco[28]", &self.ptco(28usize))
            .field("ptco[29]", &self.ptco(29usize))
            .field("ptco[30]", &self.ptco(30usize))
            .field("ptco[31]", &self.ptco(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pcor {{ ptco[0]: {:?}, ptco[1]: {:?}, ptco[2]: {:?}, ptco[3]: {:?}, ptco[4]: {:?}, ptco[5]: {:?}, ptco[6]: {:?}, ptco[7]: {:?}, ptco[8]: {:?}, ptco[9]: {:?}, ptco[10]: {:?}, ptco[11]: {:?}, ptco[12]: {:?}, ptco[13]: {:?}, ptco[14]: {:?}, ptco[15]: {:?}, ptco[16]: {:?}, ptco[17]: {:?}, ptco[18]: {:?}, ptco[19]: {:?}, ptco[20]: {:?}, ptco[21]: {:?}, ptco[22]: {:?}, ptco[23]: {:?}, ptco[24]: {:?}, ptco[25]: {:?}, ptco[26]: {:?}, ptco[27]: {:?}, ptco[28]: {:?}, ptco[29]: {:?}, ptco[30]: {:?}, ptco[31]: {:?} }}",
            self.ptco(0usize),
            self.ptco(1usize),
            self.ptco(2usize),
            self.ptco(3usize),
            self.ptco(4usize),
            self.ptco(5usize),
            self.ptco(6usize),
            self.ptco(7usize),
            self.ptco(8usize),
            self.ptco(9usize),
            self.ptco(10usize),
            self.ptco(11usize),
            self.ptco(12usize),
            self.ptco(13usize),
            self.ptco(14usize),
            self.ptco(15usize),
            self.ptco(16usize),
            self.ptco(17usize),
            self.ptco(18usize),
            self.ptco(19usize),
            self.ptco(20usize),
            self.ptco(21usize),
            self.ptco(22usize),
            self.ptco(23usize),
            self.ptco(24usize),
            self.ptco(25usize),
            self.ptco(26usize),
            self.ptco(27usize),
            self.ptco(28usize),
            self.ptco(29usize),
            self.ptco(30usize),
            self.ptco(31usize)
        )
    }
}
#[doc = "Port Data Direction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc = "Port Data Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn pdd(&self, n: usize) -> Pdd {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Pdd::from_bits(val as u8)
    }
    #[doc = "Port Data Direction."]
    #[inline(always)]
    pub const fn set_pdd(&mut self, n: usize, val: Pdd) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pddr {
    #[inline(always)]
    fn default() -> Pddr {
        Pddr(0)
    }
}
impl core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pddr")
            .field("pdd[0]", &self.pdd(0usize))
            .field("pdd[1]", &self.pdd(1usize))
            .field("pdd[2]", &self.pdd(2usize))
            .field("pdd[3]", &self.pdd(3usize))
            .field("pdd[4]", &self.pdd(4usize))
            .field("pdd[5]", &self.pdd(5usize))
            .field("pdd[6]", &self.pdd(6usize))
            .field("pdd[7]", &self.pdd(7usize))
            .field("pdd[8]", &self.pdd(8usize))
            .field("pdd[9]", &self.pdd(9usize))
            .field("pdd[10]", &self.pdd(10usize))
            .field("pdd[11]", &self.pdd(11usize))
            .field("pdd[12]", &self.pdd(12usize))
            .field("pdd[13]", &self.pdd(13usize))
            .field("pdd[14]", &self.pdd(14usize))
            .field("pdd[15]", &self.pdd(15usize))
            .field("pdd[16]", &self.pdd(16usize))
            .field("pdd[17]", &self.pdd(17usize))
            .field("pdd[18]", &self.pdd(18usize))
            .field("pdd[19]", &self.pdd(19usize))
            .field("pdd[20]", &self.pdd(20usize))
            .field("pdd[21]", &self.pdd(21usize))
            .field("pdd[22]", &self.pdd(22usize))
            .field("pdd[23]", &self.pdd(23usize))
            .field("pdd[24]", &self.pdd(24usize))
            .field("pdd[25]", &self.pdd(25usize))
            .field("pdd[26]", &self.pdd(26usize))
            .field("pdd[27]", &self.pdd(27usize))
            .field("pdd[28]", &self.pdd(28usize))
            .field("pdd[29]", &self.pdd(29usize))
            .field("pdd[30]", &self.pdd(30usize))
            .field("pdd[31]", &self.pdd(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pddr {{ pdd[0]: {:?}, pdd[1]: {:?}, pdd[2]: {:?}, pdd[3]: {:?}, pdd[4]: {:?}, pdd[5]: {:?}, pdd[6]: {:?}, pdd[7]: {:?}, pdd[8]: {:?}, pdd[9]: {:?}, pdd[10]: {:?}, pdd[11]: {:?}, pdd[12]: {:?}, pdd[13]: {:?}, pdd[14]: {:?}, pdd[15]: {:?}, pdd[16]: {:?}, pdd[17]: {:?}, pdd[18]: {:?}, pdd[19]: {:?}, pdd[20]: {:?}, pdd[21]: {:?}, pdd[22]: {:?}, pdd[23]: {:?}, pdd[24]: {:?}, pdd[25]: {:?}, pdd[26]: {:?}, pdd[27]: {:?}, pdd[28]: {:?}, pdd[29]: {:?}, pdd[30]: {:?}, pdd[31]: {:?} }}",
            self.pdd(0usize),
            self.pdd(1usize),
            self.pdd(2usize),
            self.pdd(3usize),
            self.pdd(4usize),
            self.pdd(5usize),
            self.pdd(6usize),
            self.pdd(7usize),
            self.pdd(8usize),
            self.pdd(9usize),
            self.pdd(10usize),
            self.pdd(11usize),
            self.pdd(12usize),
            self.pdd(13usize),
            self.pdd(14usize),
            self.pdd(15usize),
            self.pdd(16usize),
            self.pdd(17usize),
            self.pdd(18usize),
            self.pdd(19usize),
            self.pdd(20usize),
            self.pdd(21usize),
            self.pdd(22usize),
            self.pdd(23usize),
            self.pdd(24usize),
            self.pdd(25usize),
            self.pdd(26usize),
            self.pdd(27usize),
            self.pdd(28usize),
            self.pdd(29usize),
            self.pdd(30usize),
            self.pdd(31usize)
        )
    }
}
#[doc = "Port Data Input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc = "Port Data Input."]
    #[must_use]
    #[inline(always)]
    pub const fn pdi(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Input."]
    #[inline(always)]
    pub const fn set_pdi(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdir {
    #[inline(always)]
    fn default() -> Pdir {
        Pdir(0)
    }
}
impl core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdir")
            .field("pdi[0]", &self.pdi(0usize))
            .field("pdi[1]", &self.pdi(1usize))
            .field("pdi[2]", &self.pdi(2usize))
            .field("pdi[3]", &self.pdi(3usize))
            .field("pdi[4]", &self.pdi(4usize))
            .field("pdi[5]", &self.pdi(5usize))
            .field("pdi[6]", &self.pdi(6usize))
            .field("pdi[7]", &self.pdi(7usize))
            .field("pdi[8]", &self.pdi(8usize))
            .field("pdi[9]", &self.pdi(9usize))
            .field("pdi[10]", &self.pdi(10usize))
            .field("pdi[11]", &self.pdi(11usize))
            .field("pdi[12]", &self.pdi(12usize))
            .field("pdi[13]", &self.pdi(13usize))
            .field("pdi[14]", &self.pdi(14usize))
            .field("pdi[15]", &self.pdi(15usize))
            .field("pdi[16]", &self.pdi(16usize))
            .field("pdi[17]", &self.pdi(17usize))
            .field("pdi[18]", &self.pdi(18usize))
            .field("pdi[19]", &self.pdi(19usize))
            .field("pdi[20]", &self.pdi(20usize))
            .field("pdi[21]", &self.pdi(21usize))
            .field("pdi[22]", &self.pdi(22usize))
            .field("pdi[23]", &self.pdi(23usize))
            .field("pdi[24]", &self.pdi(24usize))
            .field("pdi[25]", &self.pdi(25usize))
            .field("pdi[26]", &self.pdi(26usize))
            .field("pdi[27]", &self.pdi(27usize))
            .field("pdi[28]", &self.pdi(28usize))
            .field("pdi[29]", &self.pdi(29usize))
            .field("pdi[30]", &self.pdi(30usize))
            .field("pdi[31]", &self.pdi(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdir {{ pdi[0]: {=bool:?}, pdi[1]: {=bool:?}, pdi[2]: {=bool:?}, pdi[3]: {=bool:?}, pdi[4]: {=bool:?}, pdi[5]: {=bool:?}, pdi[6]: {=bool:?}, pdi[7]: {=bool:?}, pdi[8]: {=bool:?}, pdi[9]: {=bool:?}, pdi[10]: {=bool:?}, pdi[11]: {=bool:?}, pdi[12]: {=bool:?}, pdi[13]: {=bool:?}, pdi[14]: {=bool:?}, pdi[15]: {=bool:?}, pdi[16]: {=bool:?}, pdi[17]: {=bool:?}, pdi[18]: {=bool:?}, pdi[19]: {=bool:?}, pdi[20]: {=bool:?}, pdi[21]: {=bool:?}, pdi[22]: {=bool:?}, pdi[23]: {=bool:?}, pdi[24]: {=bool:?}, pdi[25]: {=bool:?}, pdi[26]: {=bool:?}, pdi[27]: {=bool:?}, pdi[28]: {=bool:?}, pdi[29]: {=bool:?}, pdi[30]: {=bool:?}, pdi[31]: {=bool:?} }}",
            self.pdi(0usize),
            self.pdi(1usize),
            self.pdi(2usize),
            self.pdi(3usize),
            self.pdi(4usize),
            self.pdi(5usize),
            self.pdi(6usize),
            self.pdi(7usize),
            self.pdi(8usize),
            self.pdi(9usize),
            self.pdi(10usize),
            self.pdi(11usize),
            self.pdi(12usize),
            self.pdi(13usize),
            self.pdi(14usize),
            self.pdi(15usize),
            self.pdi(16usize),
            self.pdi(17usize),
            self.pdi(18usize),
            self.pdi(19usize),
            self.pdi(20usize),
            self.pdi(21usize),
            self.pdi(22usize),
            self.pdi(23usize),
            self.pdi(24usize),
            self.pdi(25usize),
            self.pdi(26usize),
            self.pdi(27usize),
            self.pdi(28usize),
            self.pdi(29usize),
            self.pdi(30usize),
            self.pdi(31usize)
        )
    }
}
#[doc = "Port Data Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc = "Port Data Output."]
    #[must_use]
    #[inline(always)]
    pub const fn pdo(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Data Output."]
    #[inline(always)]
    pub const fn set_pdo(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pdor {
    #[inline(always)]
    fn default() -> Pdor {
        Pdor(0)
    }
}
impl core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdor")
            .field("pdo[0]", &self.pdo(0usize))
            .field("pdo[1]", &self.pdo(1usize))
            .field("pdo[2]", &self.pdo(2usize))
            .field("pdo[3]", &self.pdo(3usize))
            .field("pdo[4]", &self.pdo(4usize))
            .field("pdo[5]", &self.pdo(5usize))
            .field("pdo[6]", &self.pdo(6usize))
            .field("pdo[7]", &self.pdo(7usize))
            .field("pdo[8]", &self.pdo(8usize))
            .field("pdo[9]", &self.pdo(9usize))
            .field("pdo[10]", &self.pdo(10usize))
            .field("pdo[11]", &self.pdo(11usize))
            .field("pdo[12]", &self.pdo(12usize))
            .field("pdo[13]", &self.pdo(13usize))
            .field("pdo[14]", &self.pdo(14usize))
            .field("pdo[15]", &self.pdo(15usize))
            .field("pdo[16]", &self.pdo(16usize))
            .field("pdo[17]", &self.pdo(17usize))
            .field("pdo[18]", &self.pdo(18usize))
            .field("pdo[19]", &self.pdo(19usize))
            .field("pdo[20]", &self.pdo(20usize))
            .field("pdo[21]", &self.pdo(21usize))
            .field("pdo[22]", &self.pdo(22usize))
            .field("pdo[23]", &self.pdo(23usize))
            .field("pdo[24]", &self.pdo(24usize))
            .field("pdo[25]", &self.pdo(25usize))
            .field("pdo[26]", &self.pdo(26usize))
            .field("pdo[27]", &self.pdo(27usize))
            .field("pdo[28]", &self.pdo(28usize))
            .field("pdo[29]", &self.pdo(29usize))
            .field("pdo[30]", &self.pdo(30usize))
            .field("pdo[31]", &self.pdo(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdor {{ pdo[0]: {=bool:?}, pdo[1]: {=bool:?}, pdo[2]: {=bool:?}, pdo[3]: {=bool:?}, pdo[4]: {=bool:?}, pdo[5]: {=bool:?}, pdo[6]: {=bool:?}, pdo[7]: {=bool:?}, pdo[8]: {=bool:?}, pdo[9]: {=bool:?}, pdo[10]: {=bool:?}, pdo[11]: {=bool:?}, pdo[12]: {=bool:?}, pdo[13]: {=bool:?}, pdo[14]: {=bool:?}, pdo[15]: {=bool:?}, pdo[16]: {=bool:?}, pdo[17]: {=bool:?}, pdo[18]: {=bool:?}, pdo[19]: {=bool:?}, pdo[20]: {=bool:?}, pdo[21]: {=bool:?}, pdo[22]: {=bool:?}, pdo[23]: {=bool:?}, pdo[24]: {=bool:?}, pdo[25]: {=bool:?}, pdo[26]: {=bool:?}, pdo[27]: {=bool:?}, pdo[28]: {=bool:?}, pdo[29]: {=bool:?}, pdo[30]: {=bool:?}, pdo[31]: {=bool:?} }}",
            self.pdo(0usize),
            self.pdo(1usize),
            self.pdo(2usize),
            self.pdo(3usize),
            self.pdo(4usize),
            self.pdo(5usize),
            self.pdo(6usize),
            self.pdo(7usize),
            self.pdo(8usize),
            self.pdo(9usize),
            self.pdo(10usize),
            self.pdo(11usize),
            self.pdo(12usize),
            self.pdo(13usize),
            self.pdo(14usize),
            self.pdo(15usize),
            self.pdo(16usize),
            self.pdo(17usize),
            self.pdo(18usize),
            self.pdo(19usize),
            self.pdo(20usize),
            self.pdo(21usize),
            self.pdo(22usize),
            self.pdo(23usize),
            self.pdo(24usize),
            self.pdo(25usize),
            self.pdo(26usize),
            self.pdo(27usize),
            self.pdo(28usize),
            self.pdo(29usize),
            self.pdo(30usize),
            self.pdo(31usize)
        )
    }
}
#[doc = "Pin Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr(pub u8);
impl Pdr {
    #[doc = "Pin Data (I/O)."]
    #[must_use]
    #[inline(always)]
    pub const fn pd(&self) -> Pd {
        let val = (self.0 >> 0usize) & 0x01;
        Pd::from_bits(val as u8)
    }
    #[doc = "Pin Data (I/O)."]
    #[inline(always)]
    pub const fn set_pd(&mut self, val: Pd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u8) & 0x01) << 0usize);
    }
}
impl Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        Pdr(0)
    }
}
impl core::fmt::Debug for Pdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdr").field("pd", &self.pd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pdr {{ pd: {:?} }}", self.pd())
    }
}
#[doc = "Port Input Disable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc = "Port Input Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pid(&self, n: usize) -> Pid {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Pid::from_bits(val as u8)
    }
    #[doc = "Port Input Disable."]
    #[inline(always)]
    pub const fn set_pid(&mut self, n: usize, val: Pid) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pidr {
    #[inline(always)]
    fn default() -> Pidr {
        Pidr(0)
    }
}
impl core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pidr")
            .field("pid[0]", &self.pid(0usize))
            .field("pid[1]", &self.pid(1usize))
            .field("pid[2]", &self.pid(2usize))
            .field("pid[3]", &self.pid(3usize))
            .field("pid[4]", &self.pid(4usize))
            .field("pid[5]", &self.pid(5usize))
            .field("pid[6]", &self.pid(6usize))
            .field("pid[7]", &self.pid(7usize))
            .field("pid[8]", &self.pid(8usize))
            .field("pid[9]", &self.pid(9usize))
            .field("pid[10]", &self.pid(10usize))
            .field("pid[11]", &self.pid(11usize))
            .field("pid[12]", &self.pid(12usize))
            .field("pid[13]", &self.pid(13usize))
            .field("pid[14]", &self.pid(14usize))
            .field("pid[15]", &self.pid(15usize))
            .field("pid[16]", &self.pid(16usize))
            .field("pid[17]", &self.pid(17usize))
            .field("pid[18]", &self.pid(18usize))
            .field("pid[19]", &self.pid(19usize))
            .field("pid[20]", &self.pid(20usize))
            .field("pid[21]", &self.pid(21usize))
            .field("pid[22]", &self.pid(22usize))
            .field("pid[23]", &self.pid(23usize))
            .field("pid[24]", &self.pid(24usize))
            .field("pid[25]", &self.pid(25usize))
            .field("pid[26]", &self.pid(26usize))
            .field("pid[27]", &self.pid(27usize))
            .field("pid[28]", &self.pid(28usize))
            .field("pid[29]", &self.pid(29usize))
            .field("pid[30]", &self.pid(30usize))
            .field("pid[31]", &self.pid(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pidr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pidr {{ pid[0]: {:?}, pid[1]: {:?}, pid[2]: {:?}, pid[3]: {:?}, pid[4]: {:?}, pid[5]: {:?}, pid[6]: {:?}, pid[7]: {:?}, pid[8]: {:?}, pid[9]: {:?}, pid[10]: {:?}, pid[11]: {:?}, pid[12]: {:?}, pid[13]: {:?}, pid[14]: {:?}, pid[15]: {:?}, pid[16]: {:?}, pid[17]: {:?}, pid[18]: {:?}, pid[19]: {:?}, pid[20]: {:?}, pid[21]: {:?}, pid[22]: {:?}, pid[23]: {:?}, pid[24]: {:?}, pid[25]: {:?}, pid[26]: {:?}, pid[27]: {:?}, pid[28]: {:?}, pid[29]: {:?}, pid[30]: {:?}, pid[31]: {:?} }}",
            self.pid(0usize),
            self.pid(1usize),
            self.pid(2usize),
            self.pid(3usize),
            self.pid(4usize),
            self.pid(5usize),
            self.pid(6usize),
            self.pid(7usize),
            self.pid(8usize),
            self.pid(9usize),
            self.pid(10usize),
            self.pid(11usize),
            self.pid(12usize),
            self.pid(13usize),
            self.pid(14usize),
            self.pid(15usize),
            self.pid(16usize),
            self.pid(17usize),
            self.pid(18usize),
            self.pid(19usize),
            self.pid(20usize),
            self.pid(21usize),
            self.pid(22usize),
            self.pid(23usize),
            self.pid(24usize),
            self.pid(25usize),
            self.pid(26usize),
            self.pid(27usize),
            self.pid(28usize),
            self.pid(29usize),
            self.pid(30usize),
            self.pid(31usize)
        )
    }
}
#[doc = "Port Set Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc = "Port Set Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptso(&self, n: usize) -> Ptso {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Ptso::from_bits(val as u8)
    }
    #[doc = "Port Set Output."]
    #[inline(always)]
    pub const fn set_ptso(&mut self, n: usize, val: Ptso) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Psor {
    #[inline(always)]
    fn default() -> Psor {
        Psor(0)
    }
}
impl core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psor")
            .field("ptso[0]", &self.ptso(0usize))
            .field("ptso[1]", &self.ptso(1usize))
            .field("ptso[2]", &self.ptso(2usize))
            .field("ptso[3]", &self.ptso(3usize))
            .field("ptso[4]", &self.ptso(4usize))
            .field("ptso[5]", &self.ptso(5usize))
            .field("ptso[6]", &self.ptso(6usize))
            .field("ptso[7]", &self.ptso(7usize))
            .field("ptso[8]", &self.ptso(8usize))
            .field("ptso[9]", &self.ptso(9usize))
            .field("ptso[10]", &self.ptso(10usize))
            .field("ptso[11]", &self.ptso(11usize))
            .field("ptso[12]", &self.ptso(12usize))
            .field("ptso[13]", &self.ptso(13usize))
            .field("ptso[14]", &self.ptso(14usize))
            .field("ptso[15]", &self.ptso(15usize))
            .field("ptso[16]", &self.ptso(16usize))
            .field("ptso[17]", &self.ptso(17usize))
            .field("ptso[18]", &self.ptso(18usize))
            .field("ptso[19]", &self.ptso(19usize))
            .field("ptso[20]", &self.ptso(20usize))
            .field("ptso[21]", &self.ptso(21usize))
            .field("ptso[22]", &self.ptso(22usize))
            .field("ptso[23]", &self.ptso(23usize))
            .field("ptso[24]", &self.ptso(24usize))
            .field("ptso[25]", &self.ptso(25usize))
            .field("ptso[26]", &self.ptso(26usize))
            .field("ptso[27]", &self.ptso(27usize))
            .field("ptso[28]", &self.ptso(28usize))
            .field("ptso[29]", &self.ptso(29usize))
            .field("ptso[30]", &self.ptso(30usize))
            .field("ptso[31]", &self.ptso(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Psor {{ ptso[0]: {:?}, ptso[1]: {:?}, ptso[2]: {:?}, ptso[3]: {:?}, ptso[4]: {:?}, ptso[5]: {:?}, ptso[6]: {:?}, ptso[7]: {:?}, ptso[8]: {:?}, ptso[9]: {:?}, ptso[10]: {:?}, ptso[11]: {:?}, ptso[12]: {:?}, ptso[13]: {:?}, ptso[14]: {:?}, ptso[15]: {:?}, ptso[16]: {:?}, ptso[17]: {:?}, ptso[18]: {:?}, ptso[19]: {:?}, ptso[20]: {:?}, ptso[21]: {:?}, ptso[22]: {:?}, ptso[23]: {:?}, ptso[24]: {:?}, ptso[25]: {:?}, ptso[26]: {:?}, ptso[27]: {:?}, ptso[28]: {:?}, ptso[29]: {:?}, ptso[30]: {:?}, ptso[31]: {:?} }}",
            self.ptso(0usize),
            self.ptso(1usize),
            self.ptso(2usize),
            self.ptso(3usize),
            self.ptso(4usize),
            self.ptso(5usize),
            self.ptso(6usize),
            self.ptso(7usize),
            self.ptso(8usize),
            self.ptso(9usize),
            self.ptso(10usize),
            self.ptso(11usize),
            self.ptso(12usize),
            self.ptso(13usize),
            self.ptso(14usize),
            self.ptso(15usize),
            self.ptso(16usize),
            self.ptso(17usize),
            self.ptso(18usize),
            self.ptso(19usize),
            self.ptso(20usize),
            self.ptso(21usize),
            self.ptso(22usize),
            self.ptso(23usize),
            self.ptso(24usize),
            self.ptso(25usize),
            self.ptso(26usize),
            self.ptso(27usize),
            self.ptso(28usize),
            self.ptso(29usize),
            self.ptso(30usize),
            self.ptso(31usize)
        )
    }
}
#[doc = "Port Toggle Output."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc = "Port Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn ptto(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Port Toggle Output."]
    #[inline(always)]
    pub const fn set_ptto(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ptor {
    #[inline(always)]
    fn default() -> Ptor {
        Ptor(0)
    }
}
impl core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptor")
            .field("ptto[0]", &self.ptto(0usize))
            .field("ptto[1]", &self.ptto(1usize))
            .field("ptto[2]", &self.ptto(2usize))
            .field("ptto[3]", &self.ptto(3usize))
            .field("ptto[4]", &self.ptto(4usize))
            .field("ptto[5]", &self.ptto(5usize))
            .field("ptto[6]", &self.ptto(6usize))
            .field("ptto[7]", &self.ptto(7usize))
            .field("ptto[8]", &self.ptto(8usize))
            .field("ptto[9]", &self.ptto(9usize))
            .field("ptto[10]", &self.ptto(10usize))
            .field("ptto[11]", &self.ptto(11usize))
            .field("ptto[12]", &self.ptto(12usize))
            .field("ptto[13]", &self.ptto(13usize))
            .field("ptto[14]", &self.ptto(14usize))
            .field("ptto[15]", &self.ptto(15usize))
            .field("ptto[16]", &self.ptto(16usize))
            .field("ptto[17]", &self.ptto(17usize))
            .field("ptto[18]", &self.ptto(18usize))
            .field("ptto[19]", &self.ptto(19usize))
            .field("ptto[20]", &self.ptto(20usize))
            .field("ptto[21]", &self.ptto(21usize))
            .field("ptto[22]", &self.ptto(22usize))
            .field("ptto[23]", &self.ptto(23usize))
            .field("ptto[24]", &self.ptto(24usize))
            .field("ptto[25]", &self.ptto(25usize))
            .field("ptto[26]", &self.ptto(26usize))
            .field("ptto[27]", &self.ptto(27usize))
            .field("ptto[28]", &self.ptto(28usize))
            .field("ptto[29]", &self.ptto(29usize))
            .field("ptto[30]", &self.ptto(30usize))
            .field("ptto[31]", &self.ptto(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ptor {{ ptto[0]: {=bool:?}, ptto[1]: {=bool:?}, ptto[2]: {=bool:?}, ptto[3]: {=bool:?}, ptto[4]: {=bool:?}, ptto[5]: {=bool:?}, ptto[6]: {=bool:?}, ptto[7]: {=bool:?}, ptto[8]: {=bool:?}, ptto[9]: {=bool:?}, ptto[10]: {=bool:?}, ptto[11]: {=bool:?}, ptto[12]: {=bool:?}, ptto[13]: {=bool:?}, ptto[14]: {=bool:?}, ptto[15]: {=bool:?}, ptto[16]: {=bool:?}, ptto[17]: {=bool:?}, ptto[18]: {=bool:?}, ptto[19]: {=bool:?}, ptto[20]: {=bool:?}, ptto[21]: {=bool:?}, ptto[22]: {=bool:?}, ptto[23]: {=bool:?}, ptto[24]: {=bool:?}, ptto[25]: {=bool:?}, ptto[26]: {=bool:?}, ptto[27]: {=bool:?}, ptto[28]: {=bool:?}, ptto[29]: {=bool:?}, ptto[30]: {=bool:?}, ptto[31]: {=bool:?} }}",
            self.ptto(0usize),
            self.ptto(1usize),
            self.ptto(2usize),
            self.ptto(3usize),
            self.ptto(4usize),
            self.ptto(5usize),
            self.ptto(6usize),
            self.ptto(7usize),
            self.ptto(8usize),
            self.ptto(9usize),
            self.ptto(10usize),
            self.ptto(11usize),
            self.ptto(12usize),
            self.ptto(13usize),
            self.ptto(14usize),
            self.ptto(15usize),
            self.ptto(16usize),
            self.ptto(17usize),
            self.ptto(18usize),
            self.ptto(19usize),
            self.ptto(20usize),
            self.ptto(21usize),
            self.ptto(22usize),
            self.ptto(23usize),
            self.ptto(24usize),
            self.ptto(25usize),
            self.ptto(26usize),
            self.ptto(27usize),
            self.ptto(28usize),
            self.ptto(29usize),
            self.ptto(30usize),
            self.ptto(31usize)
        )
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnpNpe0 {
    #[doc = "Privilege access."]
    Npe0 = 0x0,
    #[doc = "Nonprivilege access."]
    Npe1 = 0x01,
}
impl IcnpNpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnpNpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnpNpe0 {
    #[inline(always)]
    fn from(val: u8) -> IcnpNpe0 {
        IcnpNpe0::from_bits(val)
    }
}
impl From<IcnpNpe0> for u8 {
    #[inline(always)]
    fn from(val: IcnpNpe0) -> u8 {
        IcnpNpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnpNpe1 {
    #[doc = "Privilege access."]
    Npe0 = 0x0,
    #[doc = "Nonprivilege access."]
    Npe1 = 0x01,
}
impl IcnpNpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnpNpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnpNpe1 {
    #[inline(always)]
    fn from(val: u8) -> IcnpNpe1 {
        IcnpNpe1::from_bits(val)
    }
}
impl From<IcnpNpe1> for u8 {
    #[inline(always)]
    fn from(val: IcnpNpe1) -> u8 {
        IcnpNpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnsNse0 {
    #[doc = "Secure access."]
    Nse0 = 0x0,
    #[doc = "Nonsecure access."]
    Nse1 = 0x01,
}
impl IcnsNse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnsNse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnsNse0 {
    #[inline(always)]
    fn from(val: u8) -> IcnsNse0 {
        IcnsNse0::from_bits(val)
    }
}
impl From<IcnsNse0> for u8 {
    #[inline(always)]
    fn from(val: IcnsNse0) -> u8 {
        IcnsNse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IcnsNse1 {
    #[doc = "Secure access."]
    Nse0 = 0x0,
    #[doc = "Nonsecure access."]
    Nse1 = 0x01,
}
impl IcnsNse1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IcnsNse1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IcnsNse1 {
    #[inline(always)]
    fn from(val: u8) -> IcnsNse1 {
        IcnsNse1::from_bits(val)
    }
}
impl From<IcnsNse1> for u8 {
    #[inline(always)]
    fn from(val: IcnsNse1) -> u8 {
        IcnsNse1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqc {
    #[doc = "ISF is disabled."]
    Irqc0 = 0x0,
    #[doc = "ISF and DMA request on rising edge."]
    Irqc1 = 0x01,
    #[doc = "ISF and DMA request on falling edge."]
    Irqc2 = 0x02,
    #[doc = "ISF and DMA request on either edge."]
    Irqc3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ISF sets on rising edge."]
    Irqc5 = 0x05,
    #[doc = "ISF sets on falling edge."]
    Irqc6 = 0x06,
    #[doc = "ISF sets on either edge."]
    Irqc7 = 0x07,
    #[doc = "ISF and interrupt when logic 0."]
    Irqc8 = 0x08,
    #[doc = "ISF and interrupt on rising edge."]
    Irqc9 = 0x09,
    #[doc = "ISF and interrupt on falling edge."]
    Irqc10 = 0x0a,
    #[doc = "ISF and Interrupt on either edge."]
    Irqc11 = 0x0b,
    #[doc = "ISF and interrupt when logic 1."]
    Irqc12 = 0x0c,
    #[doc = "Enable active-high trigger output; ISF on rising edge (pin state is ORed with other enabled triggers to generate the output trigger for use by other peripherals)."]
    Irqc13 = 0x0d,
    #[doc = "Enable active-low trigger output; ISF on falling edge (pin state is inverted and ORed with other enabled triggers to generate the output trigger for use by other peripherals)."]
    Irqc14 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqc {
    #[inline(always)]
    fn from(val: u8) -> Irqc {
        Irqc::from_bits(val)
    }
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(val: Irqc) -> u8 {
        Irqc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqs {
    #[doc = "Interrupt, trigger output, or DMA request 0."]
    Irqs0 = 0x0,
    #[doc = "Interrupt, trigger output, or DMA request 1."]
    Irqs1 = 0x01,
}
impl Irqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqs {
    #[inline(always)]
    fn from(val: u8) -> Irqs {
        Irqs::from_bits(val)
    }
}
impl From<Irqs> for u8 {
    #[inline(always)]
    fn from(val: Irqs) -> u8 {
        Irqs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Isf {
    #[doc = "Not detected."]
    Isf0 = 0x0,
    #[doc = "Detected."]
    Isf1 = 0x01,
}
impl Isf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Isf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Isf {
    #[inline(always)]
    fn from(val: u8) -> Isf {
        Isf::from_bits(val)
    }
}
impl From<Isf> for u8 {
    #[inline(always)]
    fn from(val: Isf) -> u8 {
        Isf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lk {
    #[doc = "Not locked."]
    Lk0 = 0x0,
    #[doc = "Locked."]
    Lk1 = 0x01,
}
impl Lk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lk {
    #[inline(always)]
    fn from(val: u8) -> Lk {
        Lk::from_bits(val)
    }
}
impl From<Lk> for u8 {
    #[inline(always)]
    fn from(val: Lk) -> u8 {
        Lk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnpNpe0 {
    #[doc = "Privilege access."]
    Npe0 = 0x0,
    #[doc = "Nonprivilege access."]
    Npe1 = 0x01,
}
impl PcnpNpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnpNpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnpNpe0 {
    #[inline(always)]
    fn from(val: u8) -> PcnpNpe0 {
        PcnpNpe0::from_bits(val)
    }
}
impl From<PcnpNpe0> for u8 {
    #[inline(always)]
    fn from(val: PcnpNpe0) -> u8 {
        PcnpNpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PcnsNse0 {
    #[doc = "Secure access."]
    Nse0 = 0x0,
    #[doc = "Nonsecure access."]
    Nse1 = 0x01,
}
impl PcnsNse0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PcnsNse0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PcnsNse0 {
    #[inline(always)]
    fn from(val: u8) -> PcnsNse0 {
        PcnsNse0::from_bits(val)
    }
}
impl From<PcnsNse0> for u8 {
    #[inline(always)]
    fn from(val: PcnsNse0) -> u8 {
        PcnsNse0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pd {
    #[doc = "Logic zero."]
    Pd0 = 0x0,
    #[doc = "Logic one."]
    Pd1 = 0x01,
}
impl Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pd {
    #[inline(always)]
    fn from(val: u8) -> Pd {
        Pd::from_bits(val)
    }
}
impl From<Pd> for u8 {
    #[inline(always)]
    fn from(val: Pd) -> u8 {
        Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdd {
    #[doc = "Input."]
    Pdd0 = 0x0,
    #[doc = "Output."]
    Pdd1 = 0x01,
}
impl Pdd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdd {
    #[inline(always)]
    fn from(val: u8) -> Pdd {
        Pdd::from_bits(val)
    }
}
impl From<Pdd> for u8 {
    #[inline(always)]
    fn from(val: Pdd) -> u8 {
        Pdd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pid {
    #[doc = "Configured for general-purpose input."]
    Pid0 = 0x0,
    #[doc = "Disabled for general-purpose input."]
    Pid1 = 0x01,
}
impl Pid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pid {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pid {
    #[inline(always)]
    fn from(val: u8) -> Pid {
        Pid::from_bits(val)
    }
}
impl From<Pid> for u8 {
    #[inline(always)]
    fn from(val: Pid) -> u8 {
        Pid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptco {
    #[doc = "No change."]
    Ptco0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 0."]
    Ptco1 = 0x01,
}
impl Ptco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptco {
    #[inline(always)]
    fn from(val: u8) -> Ptco {
        Ptco::from_bits(val)
    }
}
impl From<Ptco> for u8 {
    #[inline(always)]
    fn from(val: Ptco) -> u8 {
        Ptco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptso {
    #[doc = "No change."]
    Ptso0 = 0x0,
    #[doc = "Corresponding field in PDOR becomes 1."]
    Ptso1 = 0x01,
}
impl Ptso {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptso {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptso {
    #[inline(always)]
    fn from(val: u8) -> Ptso {
        Ptso::from_bits(val)
    }
}
impl From<Ptso> for u8 {
    #[inline(always)]
    fn from(val: Ptso) -> u8 {
        Ptso::to_bits(val)
    }
}
