#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Low-Leakage Wakeup Unit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wuu {
    ptr: *mut u8,
}
unsafe impl Send for Wuu {}
unsafe impl Sync for Wuu {}
impl Wuu {
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
    #[doc = "Pin Enable 1."]
    #[inline(always)]
    pub const fn pe1(self) -> crate::pac::common::Reg<Pe1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pin Enable 2."]
    #[inline(always)]
    pub const fn pe2(self) -> crate::pac::common::Reg<Pe2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Module Interrupt Enable."]
    #[inline(always)]
    pub const fn me(self) -> crate::pac::common::Reg<Me, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Module DMA/Trigger Enable."]
    #[inline(always)]
    pub const fn de(self) -> crate::pac::common::Reg<De, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pin Flag."]
    #[inline(always)]
    pub const fn pf(self) -> crate::pac::common::Reg<Pf, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pin Filter."]
    #[inline(always)]
    pub const fn filt(self) -> crate::pac::common::Reg<Filt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Pin DMA/Trigger Configuration 1."]
    #[inline(always)]
    pub const fn pdc1(self) -> crate::pac::common::Reg<Pdc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Pin DMA/Trigger Configuration 2."]
    #[inline(always)]
    pub const fn pdc2(self) -> crate::pac::common::Reg<Pdc2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Pin Filter DMA/Trigger Configuration."]
    #[inline(always)]
    pub const fn fdc(self) -> crate::pac::common::Reg<Fdc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Pin Mode Configuration."]
    #[inline(always)]
    pub const fn pmc(self) -> crate::pac::common::Reg<Pmc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Pin Filter Mode Configuration."]
    #[inline(always)]
    pub const fn fmc(self) -> crate::pac::common::Reg<Fmc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
}
#[doc = "Module DMA/Trigger Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct De(pub u32);
impl De {
    #[doc = "DMA/Trigger Wake-up Enable for Module n."]
    #[must_use]
    #[inline(always)]
    pub const fn wude(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module n."]
    #[inline(always)]
    pub const fn set_wude(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for De {
    #[inline(always)]
    fn default() -> De {
        De(0)
    }
}
impl core::fmt::Debug for De {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("De")
            .field("wude[0]", &self.wude(0usize))
            .field("wude[1]", &self.wude(1usize))
            .field("wude[2]", &self.wude(2usize))
            .field("wude[3]", &self.wude(3usize))
            .field("wude[4]", &self.wude(4usize))
            .field("wude[5]", &self.wude(5usize))
            .field("wude[6]", &self.wude(6usize))
            .field("wude[7]", &self.wude(7usize))
            .field("wude[8]", &self.wude(8usize))
            .field("wude[9]", &self.wude(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "De {{ wude[0]: {=bool:?}, wude[1]: {=bool:?}, wude[2]: {=bool:?}, wude[3]: {=bool:?}, wude[4]: {=bool:?}, wude[5]: {=bool:?}, wude[6]: {=bool:?}, wude[7]: {=bool:?}, wude[8]: {=bool:?}, wude[9]: {=bool:?} }}",
            self.wude(0usize),
            self.wude(1usize),
            self.wude(2usize),
            self.wude(3usize),
            self.wude(4usize),
            self.wude(5usize),
            self.wude(6usize),
            self.wude(7usize),
            self.wude(8usize),
            self.wude(9usize)
        )
    }
}
#[doc = "Pin Filter DMA/Trigger Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdc(pub u32);
impl Fdc {
    #[doc = "Filter Configuration for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtc1(&self) -> Filtc1 {
        let val = (self.0 >> 0usize) & 0x03;
        Filtc1::from_bits(val as u8)
    }
    #[doc = "Filter Configuration for FILTn."]
    #[inline(always)]
    pub const fn set_filtc1(&mut self, val: Filtc1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Filter Configuration for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtc2(&self) -> Filtc2 {
        let val = (self.0 >> 2usize) & 0x03;
        Filtc2::from_bits(val as u8)
    }
    #[doc = "Filter Configuration for FILTn."]
    #[inline(always)]
    pub const fn set_filtc2(&mut self, val: Filtc2) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for Fdc {
    #[inline(always)]
    fn default() -> Fdc {
        Fdc(0)
    }
}
impl core::fmt::Debug for Fdc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdc")
            .field("filtc1", &self.filtc1())
            .field("filtc2", &self.filtc2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fdc {{ filtc1: {:?}, filtc2: {:?} }}",
            self.filtc1(),
            self.filtc2()
        )
    }
}
#[doc = "Pin Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filt(pub u32);
impl Filt {
    #[doc = "Filter 1 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn filtsel1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Filter 1 Pin Select."]
    #[inline(always)]
    pub const fn set_filtsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Filter 1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn filte1(&self) -> Filte1 {
        let val = (self.0 >> 5usize) & 0x03;
        Filte1::from_bits(val as u8)
    }
    #[doc = "Filter 1 Enable."]
    #[inline(always)]
    pub const fn set_filte1(&mut self, val: Filte1) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "Filter 1 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn filtf1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Filter 1 Flag."]
    #[inline(always)]
    pub const fn set_filtf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Filter 2 Pin Select."]
    #[must_use]
    #[inline(always)]
    pub const fn filtsel2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Filter 2 Pin Select."]
    #[inline(always)]
    pub const fn set_filtsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Filter 2 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn filte2(&self) -> Filte2 {
        let val = (self.0 >> 13usize) & 0x03;
        Filte2::from_bits(val as u8)
    }
    #[doc = "Filter 2 Enable."]
    #[inline(always)]
    pub const fn set_filte2(&mut self, val: Filte2) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
    #[doc = "Filter 2 Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn filtf2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Filter 2 Flag."]
    #[inline(always)]
    pub const fn set_filtf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Filt {
    #[inline(always)]
    fn default() -> Filt {
        Filt(0)
    }
}
impl core::fmt::Debug for Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filt")
            .field("filtsel1", &self.filtsel1())
            .field("filte1", &self.filte1())
            .field("filtf1", &self.filtf1())
            .field("filtsel2", &self.filtsel2())
            .field("filte2", &self.filte2())
            .field("filtf2", &self.filtf2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Filt {{ filtsel1: {=u8:?}, filte1: {:?}, filtf1: {=bool:?}, filtsel2: {=u8:?}, filte2: {:?}, filtf2: {=bool:?} }}",
            self.filtsel1(),
            self.filte1(),
            self.filtf1(),
            self.filtsel2(),
            self.filte2(),
            self.filtf2()
        )
    }
}
#[doc = "Pin Filter Mode Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc(pub u32);
impl Fmc {
    #[doc = "Filter Mode for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtm1(&self) -> Filtm1 {
        let val = (self.0 >> 0usize) & 0x01;
        Filtm1::from_bits(val as u8)
    }
    #[doc = "Filter Mode for FILTn."]
    #[inline(always)]
    pub const fn set_filtm1(&mut self, val: Filtm1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Filter Mode for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtm2(&self) -> Filtm2 {
        let val = (self.0 >> 1usize) & 0x01;
        Filtm2::from_bits(val as u8)
    }
    #[doc = "Filter Mode for FILTn."]
    #[inline(always)]
    pub const fn set_filtm2(&mut self, val: Filtm2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Fmc {
    #[inline(always)]
    fn default() -> Fmc {
        Fmc(0)
    }
}
impl core::fmt::Debug for Fmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmc")
            .field("filtm1", &self.filtm1())
            .field("filtm2", &self.filtm2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fmc {{ filtm1: {:?}, filtm2: {:?} }}",
            self.filtm1(),
            self.filtm2()
        )
    }
}
#[doc = "Module Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me(pub u32);
impl Me {
    #[doc = "Module Interrupt Wake-up Enable for Module n."]
    #[must_use]
    #[inline(always)]
    pub const fn wume(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Wake-up Enable for Module n."]
    #[inline(always)]
    pub const fn set_wume(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Me {
    #[inline(always)]
    fn default() -> Me {
        Me(0)
    }
}
impl core::fmt::Debug for Me {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Me")
            .field("wume[0]", &self.wume(0usize))
            .field("wume[1]", &self.wume(1usize))
            .field("wume[2]", &self.wume(2usize))
            .field("wume[3]", &self.wume(3usize))
            .field("wume[4]", &self.wume(4usize))
            .field("wume[5]", &self.wume(5usize))
            .field("wume[6]", &self.wume(6usize))
            .field("wume[7]", &self.wume(7usize))
            .field("wume[8]", &self.wume(8usize))
            .field("wume[9]", &self.wume(9usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Me {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Me {{ wume[0]: {=bool:?}, wume[1]: {=bool:?}, wume[2]: {=bool:?}, wume[3]: {=bool:?}, wume[4]: {=bool:?}, wume[5]: {=bool:?}, wume[6]: {=bool:?}, wume[7]: {=bool:?}, wume[8]: {=bool:?}, wume[9]: {=bool:?} }}",
            self.wume(0usize),
            self.wume(1usize),
            self.wume(2usize),
            self.wume(3usize),
            self.wume(4usize),
            self.wume(5usize),
            self.wume(6usize),
            self.wume(7usize),
            self.wume(8usize),
            self.wume(9usize)
        )
    }
}
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Filter Number."]
    #[must_use]
    #[inline(always)]
    pub const fn filters(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Filter Number."]
    #[inline(always)]
    pub const fn set_filters(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DMA Number."]
    #[must_use]
    #[inline(always)]
    pub const fn dmas(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "DMA Number."]
    #[inline(always)]
    pub const fn set_dmas(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Module Number."]
    #[must_use]
    #[inline(always)]
    pub const fn modules(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Module Number."]
    #[inline(always)]
    pub const fn set_modules(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Pin Number."]
    #[must_use]
    #[inline(always)]
    pub const fn pins(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number."]
    #[inline(always)]
    pub const fn set_pins(&mut self, val: u8) {
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
            .field("filters", &self.filters())
            .field("dmas", &self.dmas())
            .field("modules", &self.modules())
            .field("pins", &self.pins())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ filters: {=u8:?}, dmas: {=u8:?}, modules: {=u8:?}, pins: {=u8:?} }}",
            self.filters(),
            self.dmas(),
            self.modules(),
            self.pins()
        )
    }
}
#[doc = "Pin DMA/Trigger Configuration 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdc1(pub u32);
impl Pdc1 {
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc(&self, n: usize) -> Wupdc {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Wupdc::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc(&mut self, n: usize, val: Wupdc) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Pdc1 {
    #[inline(always)]
    fn default() -> Pdc1 {
        Pdc1(0)
    }
}
impl core::fmt::Debug for Pdc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdc1")
            .field("wupdc[0]", &self.wupdc(0usize))
            .field("wupdc[1]", &self.wupdc(1usize))
            .field("wupdc[2]", &self.wupdc(2usize))
            .field("wupdc[3]", &self.wupdc(3usize))
            .field("wupdc[4]", &self.wupdc(4usize))
            .field("wupdc[5]", &self.wupdc(5usize))
            .field("wupdc[6]", &self.wupdc(6usize))
            .field("wupdc[7]", &self.wupdc(7usize))
            .field("wupdc[8]", &self.wupdc(8usize))
            .field("wupdc[9]", &self.wupdc(9usize))
            .field("wupdc[10]", &self.wupdc(10usize))
            .field("wupdc[11]", &self.wupdc(11usize))
            .field("wupdc[12]", &self.wupdc(12usize))
            .field("wupdc[13]", &self.wupdc(13usize))
            .field("wupdc[14]", &self.wupdc(14usize))
            .field("wupdc[15]", &self.wupdc(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdc1 {{ wupdc[0]: {:?}, wupdc[1]: {:?}, wupdc[2]: {:?}, wupdc[3]: {:?}, wupdc[4]: {:?}, wupdc[5]: {:?}, wupdc[6]: {:?}, wupdc[7]: {:?}, wupdc[8]: {:?}, wupdc[9]: {:?}, wupdc[10]: {:?}, wupdc[11]: {:?}, wupdc[12]: {:?}, wupdc[13]: {:?}, wupdc[14]: {:?}, wupdc[15]: {:?} }}",
            self.wupdc(0usize),
            self.wupdc(1usize),
            self.wupdc(2usize),
            self.wupdc(3usize),
            self.wupdc(4usize),
            self.wupdc(5usize),
            self.wupdc(6usize),
            self.wupdc(7usize),
            self.wupdc(8usize),
            self.wupdc(9usize),
            self.wupdc(10usize),
            self.wupdc(11usize),
            self.wupdc(12usize),
            self.wupdc(13usize),
            self.wupdc(14usize),
            self.wupdc(15usize)
        )
    }
}
#[doc = "Pin DMA/Trigger Configuration 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdc2(pub u32);
impl Pdc2 {
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc(&self, n: usize) -> Wupdc {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Wupdc::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc(&mut self, n: usize, val: Wupdc) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Pdc2 {
    #[inline(always)]
    fn default() -> Pdc2 {
        Pdc2(0)
    }
}
impl core::fmt::Debug for Pdc2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdc2")
            .field("wupdc[0]", &self.wupdc(0usize))
            .field("wupdc[1]", &self.wupdc(1usize))
            .field("wupdc[2]", &self.wupdc(2usize))
            .field("wupdc[3]", &self.wupdc(3usize))
            .field("wupdc[4]", &self.wupdc(4usize))
            .field("wupdc[5]", &self.wupdc(5usize))
            .field("wupdc[6]", &self.wupdc(6usize))
            .field("wupdc[7]", &self.wupdc(7usize))
            .field("wupdc[8]", &self.wupdc(8usize))
            .field("wupdc[9]", &self.wupdc(9usize))
            .field("wupdc[10]", &self.wupdc(10usize))
            .field("wupdc[11]", &self.wupdc(11usize))
            .field("wupdc[12]", &self.wupdc(12usize))
            .field("wupdc[13]", &self.wupdc(13usize))
            .field("wupdc[14]", &self.wupdc(14usize))
            .field("wupdc[15]", &self.wupdc(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdc2 {{ wupdc[0]: {:?}, wupdc[1]: {:?}, wupdc[2]: {:?}, wupdc[3]: {:?}, wupdc[4]: {:?}, wupdc[5]: {:?}, wupdc[6]: {:?}, wupdc[7]: {:?}, wupdc[8]: {:?}, wupdc[9]: {:?}, wupdc[10]: {:?}, wupdc[11]: {:?}, wupdc[12]: {:?}, wupdc[13]: {:?}, wupdc[14]: {:?}, wupdc[15]: {:?} }}",
            self.wupdc(0usize),
            self.wupdc(1usize),
            self.wupdc(2usize),
            self.wupdc(3usize),
            self.wupdc(4usize),
            self.wupdc(5usize),
            self.wupdc(6usize),
            self.wupdc(7usize),
            self.wupdc(8usize),
            self.wupdc(9usize),
            self.wupdc(10usize),
            self.wupdc(11usize),
            self.wupdc(12usize),
            self.wupdc(13usize),
            self.wupdc(14usize),
            self.wupdc(15usize)
        )
    }
}
#[doc = "Pin Enable 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe1(pub u32);
impl Pe1 {
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe(&self, n: usize) -> Wupe {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Wupe::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe(&mut self, n: usize, val: Wupe) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Pe1 {
    #[inline(always)]
    fn default() -> Pe1 {
        Pe1(0)
    }
}
impl core::fmt::Debug for Pe1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pe1")
            .field("wupe[0]", &self.wupe(0usize))
            .field("wupe[1]", &self.wupe(1usize))
            .field("wupe[2]", &self.wupe(2usize))
            .field("wupe[3]", &self.wupe(3usize))
            .field("wupe[4]", &self.wupe(4usize))
            .field("wupe[5]", &self.wupe(5usize))
            .field("wupe[6]", &self.wupe(6usize))
            .field("wupe[7]", &self.wupe(7usize))
            .field("wupe[8]", &self.wupe(8usize))
            .field("wupe[9]", &self.wupe(9usize))
            .field("wupe[10]", &self.wupe(10usize))
            .field("wupe[11]", &self.wupe(11usize))
            .field("wupe[12]", &self.wupe(12usize))
            .field("wupe[13]", &self.wupe(13usize))
            .field("wupe[14]", &self.wupe(14usize))
            .field("wupe[15]", &self.wupe(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pe1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pe1 {{ wupe[0]: {:?}, wupe[1]: {:?}, wupe[2]: {:?}, wupe[3]: {:?}, wupe[4]: {:?}, wupe[5]: {:?}, wupe[6]: {:?}, wupe[7]: {:?}, wupe[8]: {:?}, wupe[9]: {:?}, wupe[10]: {:?}, wupe[11]: {:?}, wupe[12]: {:?}, wupe[13]: {:?}, wupe[14]: {:?}, wupe[15]: {:?} }}",
            self.wupe(0usize),
            self.wupe(1usize),
            self.wupe(2usize),
            self.wupe(3usize),
            self.wupe(4usize),
            self.wupe(5usize),
            self.wupe(6usize),
            self.wupe(7usize),
            self.wupe(8usize),
            self.wupe(9usize),
            self.wupe(10usize),
            self.wupe(11usize),
            self.wupe(12usize),
            self.wupe(13usize),
            self.wupe(14usize),
            self.wupe(15usize)
        )
    }
}
#[doc = "Pin Enable 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe2(pub u32);
impl Pe2 {
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe(&self, n: usize) -> Wupe {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        Wupe::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe(&mut self, n: usize, val: Wupe) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
    }
}
impl Default for Pe2 {
    #[inline(always)]
    fn default() -> Pe2 {
        Pe2(0)
    }
}
impl core::fmt::Debug for Pe2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pe2")
            .field("wupe[0]", &self.wupe(0usize))
            .field("wupe[1]", &self.wupe(1usize))
            .field("wupe[2]", &self.wupe(2usize))
            .field("wupe[3]", &self.wupe(3usize))
            .field("wupe[4]", &self.wupe(4usize))
            .field("wupe[5]", &self.wupe(5usize))
            .field("wupe[6]", &self.wupe(6usize))
            .field("wupe[7]", &self.wupe(7usize))
            .field("wupe[8]", &self.wupe(8usize))
            .field("wupe[9]", &self.wupe(9usize))
            .field("wupe[10]", &self.wupe(10usize))
            .field("wupe[11]", &self.wupe(11usize))
            .field("wupe[12]", &self.wupe(12usize))
            .field("wupe[13]", &self.wupe(13usize))
            .field("wupe[14]", &self.wupe(14usize))
            .field("wupe[15]", &self.wupe(15usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pe2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pe2 {{ wupe[0]: {:?}, wupe[1]: {:?}, wupe[2]: {:?}, wupe[3]: {:?}, wupe[4]: {:?}, wupe[5]: {:?}, wupe[6]: {:?}, wupe[7]: {:?}, wupe[8]: {:?}, wupe[9]: {:?}, wupe[10]: {:?}, wupe[11]: {:?}, wupe[12]: {:?}, wupe[13]: {:?}, wupe[14]: {:?}, wupe[15]: {:?} }}",
            self.wupe(0usize),
            self.wupe(1usize),
            self.wupe(2usize),
            self.wupe(3usize),
            self.wupe(4usize),
            self.wupe(5usize),
            self.wupe(6usize),
            self.wupe(7usize),
            self.wupe(8usize),
            self.wupe(9usize),
            self.wupe(10usize),
            self.wupe(11usize),
            self.wupe(12usize),
            self.wupe(13usize),
            self.wupe(14usize),
            self.wupe(15usize)
        )
    }
}
#[doc = "Pin Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pf(pub u32);
impl Pf {
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf(&self, n: usize) -> bool {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf(&mut self, n: usize, val: bool) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pf {
    #[inline(always)]
    fn default() -> Pf {
        Pf(0)
    }
}
impl core::fmt::Debug for Pf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pf")
            .field("wuf[0]", &self.wuf(0usize))
            .field("wuf[1]", &self.wuf(1usize))
            .field("wuf[2]", &self.wuf(2usize))
            .field("wuf[3]", &self.wuf(3usize))
            .field("wuf[4]", &self.wuf(4usize))
            .field("wuf[5]", &self.wuf(5usize))
            .field("wuf[6]", &self.wuf(6usize))
            .field("wuf[7]", &self.wuf(7usize))
            .field("wuf[8]", &self.wuf(8usize))
            .field("wuf[9]", &self.wuf(9usize))
            .field("wuf[10]", &self.wuf(10usize))
            .field("wuf[11]", &self.wuf(11usize))
            .field("wuf[12]", &self.wuf(12usize))
            .field("wuf[13]", &self.wuf(13usize))
            .field("wuf[14]", &self.wuf(14usize))
            .field("wuf[15]", &self.wuf(15usize))
            .field("wuf[16]", &self.wuf(16usize))
            .field("wuf[17]", &self.wuf(17usize))
            .field("wuf[18]", &self.wuf(18usize))
            .field("wuf[19]", &self.wuf(19usize))
            .field("wuf[20]", &self.wuf(20usize))
            .field("wuf[21]", &self.wuf(21usize))
            .field("wuf[22]", &self.wuf(22usize))
            .field("wuf[23]", &self.wuf(23usize))
            .field("wuf[24]", &self.wuf(24usize))
            .field("wuf[25]", &self.wuf(25usize))
            .field("wuf[26]", &self.wuf(26usize))
            .field("wuf[27]", &self.wuf(27usize))
            .field("wuf[28]", &self.wuf(28usize))
            .field("wuf[29]", &self.wuf(29usize))
            .field("wuf[30]", &self.wuf(30usize))
            .field("wuf[31]", &self.wuf(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pf {{ wuf[0]: {=bool:?}, wuf[1]: {=bool:?}, wuf[2]: {=bool:?}, wuf[3]: {=bool:?}, wuf[4]: {=bool:?}, wuf[5]: {=bool:?}, wuf[6]: {=bool:?}, wuf[7]: {=bool:?}, wuf[8]: {=bool:?}, wuf[9]: {=bool:?}, wuf[10]: {=bool:?}, wuf[11]: {=bool:?}, wuf[12]: {=bool:?}, wuf[13]: {=bool:?}, wuf[14]: {=bool:?}, wuf[15]: {=bool:?}, wuf[16]: {=bool:?}, wuf[17]: {=bool:?}, wuf[18]: {=bool:?}, wuf[19]: {=bool:?}, wuf[20]: {=bool:?}, wuf[21]: {=bool:?}, wuf[22]: {=bool:?}, wuf[23]: {=bool:?}, wuf[24]: {=bool:?}, wuf[25]: {=bool:?}, wuf[26]: {=bool:?}, wuf[27]: {=bool:?}, wuf[28]: {=bool:?}, wuf[29]: {=bool:?}, wuf[30]: {=bool:?}, wuf[31]: {=bool:?} }}",
            self.wuf(0usize),
            self.wuf(1usize),
            self.wuf(2usize),
            self.wuf(3usize),
            self.wuf(4usize),
            self.wuf(5usize),
            self.wuf(6usize),
            self.wuf(7usize),
            self.wuf(8usize),
            self.wuf(9usize),
            self.wuf(10usize),
            self.wuf(11usize),
            self.wuf(12usize),
            self.wuf(13usize),
            self.wuf(14usize),
            self.wuf(15usize),
            self.wuf(16usize),
            self.wuf(17usize),
            self.wuf(18usize),
            self.wuf(19usize),
            self.wuf(20usize),
            self.wuf(21usize),
            self.wuf(22usize),
            self.wuf(23usize),
            self.wuf(24usize),
            self.wuf(25usize),
            self.wuf(26usize),
            self.wuf(27usize),
            self.wuf(28usize),
            self.wuf(29usize),
            self.wuf(30usize),
            self.wuf(31usize)
        )
    }
}
#[doc = "Pin Mode Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmc(pub u32);
impl Pmc {
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc(&self, n: usize) -> Wupmc {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Wupmc::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc(&mut self, n: usize, val: Wupmc) {
        assert!(n < 32usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
}
impl Default for Pmc {
    #[inline(always)]
    fn default() -> Pmc {
        Pmc(0)
    }
}
impl core::fmt::Debug for Pmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmc")
            .field("wupmc[0]", &self.wupmc(0usize))
            .field("wupmc[1]", &self.wupmc(1usize))
            .field("wupmc[2]", &self.wupmc(2usize))
            .field("wupmc[3]", &self.wupmc(3usize))
            .field("wupmc[4]", &self.wupmc(4usize))
            .field("wupmc[5]", &self.wupmc(5usize))
            .field("wupmc[6]", &self.wupmc(6usize))
            .field("wupmc[7]", &self.wupmc(7usize))
            .field("wupmc[8]", &self.wupmc(8usize))
            .field("wupmc[9]", &self.wupmc(9usize))
            .field("wupmc[10]", &self.wupmc(10usize))
            .field("wupmc[11]", &self.wupmc(11usize))
            .field("wupmc[12]", &self.wupmc(12usize))
            .field("wupmc[13]", &self.wupmc(13usize))
            .field("wupmc[14]", &self.wupmc(14usize))
            .field("wupmc[15]", &self.wupmc(15usize))
            .field("wupmc[16]", &self.wupmc(16usize))
            .field("wupmc[17]", &self.wupmc(17usize))
            .field("wupmc[18]", &self.wupmc(18usize))
            .field("wupmc[19]", &self.wupmc(19usize))
            .field("wupmc[20]", &self.wupmc(20usize))
            .field("wupmc[21]", &self.wupmc(21usize))
            .field("wupmc[22]", &self.wupmc(22usize))
            .field("wupmc[23]", &self.wupmc(23usize))
            .field("wupmc[24]", &self.wupmc(24usize))
            .field("wupmc[25]", &self.wupmc(25usize))
            .field("wupmc[26]", &self.wupmc(26usize))
            .field("wupmc[27]", &self.wupmc(27usize))
            .field("wupmc[28]", &self.wupmc(28usize))
            .field("wupmc[29]", &self.wupmc(29usize))
            .field("wupmc[30]", &self.wupmc(30usize))
            .field("wupmc[31]", &self.wupmc(31usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmc {{ wupmc[0]: {:?}, wupmc[1]: {:?}, wupmc[2]: {:?}, wupmc[3]: {:?}, wupmc[4]: {:?}, wupmc[5]: {:?}, wupmc[6]: {:?}, wupmc[7]: {:?}, wupmc[8]: {:?}, wupmc[9]: {:?}, wupmc[10]: {:?}, wupmc[11]: {:?}, wupmc[12]: {:?}, wupmc[13]: {:?}, wupmc[14]: {:?}, wupmc[15]: {:?}, wupmc[16]: {:?}, wupmc[17]: {:?}, wupmc[18]: {:?}, wupmc[19]: {:?}, wupmc[20]: {:?}, wupmc[21]: {:?}, wupmc[22]: {:?}, wupmc[23]: {:?}, wupmc[24]: {:?}, wupmc[25]: {:?}, wupmc[26]: {:?}, wupmc[27]: {:?}, wupmc[28]: {:?}, wupmc[29]: {:?}, wupmc[30]: {:?}, wupmc[31]: {:?} }}",
            self.wupmc(0usize),
            self.wupmc(1usize),
            self.wupmc(2usize),
            self.wupmc(3usize),
            self.wupmc(4usize),
            self.wupmc(5usize),
            self.wupmc(6usize),
            self.wupmc(7usize),
            self.wupmc(8usize),
            self.wupmc(9usize),
            self.wupmc(10usize),
            self.wupmc(11usize),
            self.wupmc(12usize),
            self.wupmc(13usize),
            self.wupmc(14usize),
            self.wupmc(15usize),
            self.wupmc(16usize),
            self.wupmc(17usize),
            self.wupmc(18usize),
            self.wupmc(19usize),
            self.wupmc(20usize),
            self.wupmc(21usize),
            self.wupmc(22usize),
            self.wupmc(23usize),
            self.wupmc(24usize),
            self.wupmc(25usize),
            self.wupmc(26usize),
            self.wupmc(27usize),
            self.wupmc(28usize),
            self.wupmc(29usize),
            self.wupmc(30usize),
            self.wupmc(31usize)
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(u16);
impl Feature {
    #[doc = "Standard features implemented."]
    pub const Standard: Self = Self(0x0);
    #[doc = "Support for DMA/Trigger generation from wake-up pins and filters enabled. Support for external pin/filter detection during all power modes enabled."]
    pub const FiltAllPwr: Self = Self(0x01);
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
            0x01 => f.write_str("FiltAllPwr"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Standard"),
            0x01 => defmt::write!(f, "FiltAllPwr"),
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
pub enum Filtc1 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    _RESERVED_3 = 0x03,
}
impl Filtc1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtc1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtc1 {
    #[inline(always)]
    fn from(val: u8) -> Filtc1 {
        Filtc1::from_bits(val)
    }
}
impl From<Filtc1> for u8 {
    #[inline(always)]
    fn from(val: Filtc1) -> u8 {
        Filtc1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtc2 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    _RESERVED_3 = 0x03,
}
impl Filtc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtc2 {
    #[inline(always)]
    fn from(val: u8) -> Filtc2 {
        Filtc2::from_bits(val)
    }
}
impl From<Filtc2> for u8 {
    #[inline(always)]
    fn from(val: Filtc2) -> u8 {
        Filtc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filte1 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (Detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (Detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (Detect on any edge)."]
    EnAny = 0x03,
}
impl Filte1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filte1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filte1 {
    #[inline(always)]
    fn from(val: u8) -> Filte1 {
        Filte1::from_bits(val)
    }
}
impl From<Filte1> for u8 {
    #[inline(always)]
    fn from(val: Filte1) -> u8 {
        Filte1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filte2 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (Detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (Detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (Detect on any edge)."]
    EnAny = 0x03,
}
impl Filte2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filte2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filte2 {
    #[inline(always)]
    fn from(val: u8) -> Filte2 {
        Filte2::from_bits(val)
    }
}
impl From<Filte2> for u8 {
    #[inline(always)]
    fn from(val: Filte2) -> u8 {
        Filte2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtm1 {
    #[doc = "Active only during Power Down/Deep Power Down mode."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes."]
    AnyPwr = 0x01,
}
impl Filtm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtm1 {
    #[inline(always)]
    fn from(val: u8) -> Filtm1 {
        Filtm1::from_bits(val)
    }
}
impl From<Filtm1> for u8 {
    #[inline(always)]
    fn from(val: Filtm1) -> u8 {
        Filtm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Filtm2 {
    #[doc = "Active only during Power Down/Deep Power Down mode."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes."]
    AnyPwr = 0x01,
}
impl Filtm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Filtm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Filtm2 {
    #[inline(always)]
    fn from(val: u8) -> Filtm2 {
        Filtm2::from_bits(val)
    }
}
impl From<Filtm2> for u8 {
    #[inline(always)]
    fn from(val: Filtm2) -> u8 {
        Filtm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc {
    #[inline(always)]
    fn from(val: u8) -> Wupdc {
        Wupdc::from_bits(val)
    }
}
impl From<Wupdc> for u8 {
    #[inline(always)]
    fn from(val: Wupdc) -> u8 {
        Wupdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe {
    #[inline(always)]
    fn from(val: u8) -> Wupe {
        Wupe::from_bits(val)
    }
}
impl From<Wupe> for u8 {
    #[inline(always)]
    fn from(val: Wupe) -> u8 {
        Wupe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc {
    #[inline(always)]
    fn from(val: u8) -> Wupmc {
        Wupmc::from_bits(val)
    }
}
impl From<Wupmc> for u8 {
    #[inline(always)]
    fn from(val: Wupmc) -> u8 {
        Wupmc::to_bits(val)
    }
}
