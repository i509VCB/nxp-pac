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
    #[doc = "DMA/Trigger Wake-up Enable for Module 4."]
    #[must_use]
    #[inline(always)]
    pub const fn wude4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module 4."]
    #[inline(always)]
    pub const fn set_wude4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module 6."]
    #[must_use]
    #[inline(always)]
    pub const fn wude6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module 6."]
    #[inline(always)]
    pub const fn set_wude6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module 8."]
    #[must_use]
    #[inline(always)]
    pub const fn wude8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA/Trigger Wake-up Enable for Module 8."]
    #[inline(always)]
    pub const fn set_wude8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("wude4", &self.wude4())
            .field("wude6", &self.wude6())
            .field("wude8", &self.wude8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for De {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "De {{ wude4: {=bool:?}, wude6: {=bool:?}, wude8: {=bool:?} }}",
            self.wude4(),
            self.wude6(),
            self.wude8()
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
    #[doc = "Module Interrupt Wake-up Enable for Module 0."]
    #[must_use]
    #[inline(always)]
    pub const fn wume0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 0."]
    #[inline(always)]
    pub const fn set_wume0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 2."]
    #[must_use]
    #[inline(always)]
    pub const fn wume2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 2."]
    #[inline(always)]
    pub const fn set_wume2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 6."]
    #[must_use]
    #[inline(always)]
    pub const fn wume6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 6."]
    #[inline(always)]
    pub const fn set_wume6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 8."]
    #[must_use]
    #[inline(always)]
    pub const fn wume8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Module Interrupt Wake-up Enable for Module 8."]
    #[inline(always)]
    pub const fn set_wume8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
            .field("wume0", &self.wume0())
            .field("wume2", &self.wume2())
            .field("wume6", &self.wume6())
            .field("wume8", &self.wume8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Me {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Me {{ wume0: {=bool:?}, wume2: {=bool:?}, wume6: {=bool:?}, wume8: {=bool:?} }}",
            self.wume0(),
            self.wume2(),
            self.wume6(),
            self.wume8()
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
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> Pdc1Reserved0 {
        let val = (self.0 >> 0usize) & 0x03;
        Pdc1Reserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: Pdc1Reserved0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> Pdc1Reserved1 {
        let val = (self.0 >> 2usize) & 0x03;
        Pdc1Reserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: Pdc1Reserved1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc2(&self) -> Wupdc2 {
        let val = (self.0 >> 4usize) & 0x03;
        Wupdc2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc2(&mut self, val: Wupdc2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> Pdc1Reserved3 {
        let val = (self.0 >> 6usize) & 0x03;
        Pdc1Reserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: Pdc1Reserved3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> Pdc1Reserved4 {
        let val = (self.0 >> 8usize) & 0x03;
        Pdc1Reserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: Pdc1Reserved4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> Pdc1Reserved5 {
        let val = (self.0 >> 10usize) & 0x03;
        Pdc1Reserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: Pdc1Reserved5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc6(&self) -> Wupdc6 {
        let val = (self.0 >> 12usize) & 0x03;
        Wupdc6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc6(&mut self, val: Wupdc6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc7(&self) -> Wupdc7 {
        let val = (self.0 >> 14usize) & 0x03;
        Wupdc7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc7(&mut self, val: Wupdc7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc8(&self) -> Wupdc8 {
        let val = (self.0 >> 16usize) & 0x03;
        Wupdc8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc8(&mut self, val: Wupdc8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc9(&self) -> Wupdc9 {
        let val = (self.0 >> 18usize) & 0x03;
        Wupdc9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc9(&mut self, val: Wupdc9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc10(&self) -> Wupdc10 {
        let val = (self.0 >> 20usize) & 0x03;
        Wupdc10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc10(&mut self, val: Wupdc10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc11(&self) -> Wupdc11 {
        let val = (self.0 >> 22usize) & 0x03;
        Wupdc11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc11(&mut self, val: Wupdc11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc12(&self) -> Wupdc12 {
        let val = (self.0 >> 24usize) & 0x03;
        Wupdc12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc12(&mut self, val: Wupdc12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> Pdc1Reserved13 {
        let val = (self.0 >> 26usize) & 0x03;
        Pdc1Reserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: Pdc1Reserved13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> Pdc1Reserved14 {
        let val = (self.0 >> 28usize) & 0x03;
        Pdc1Reserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: Pdc1Reserved14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> Pdc1Reserved15 {
        let val = (self.0 >> 30usize) & 0x03;
        Pdc1Reserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: Pdc1Reserved15) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("wupdc2", &self.wupdc2())
            .field("reserved3", &self.reserved3())
            .field("reserved4", &self.reserved4())
            .field("reserved5", &self.reserved5())
            .field("wupdc6", &self.wupdc6())
            .field("wupdc7", &self.wupdc7())
            .field("wupdc8", &self.wupdc8())
            .field("wupdc9", &self.wupdc9())
            .field("wupdc10", &self.wupdc10())
            .field("wupdc11", &self.wupdc11())
            .field("wupdc12", &self.wupdc12())
            .field("reserved13", &self.reserved13())
            .field("reserved14", &self.reserved14())
            .field("reserved15", &self.reserved15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdc1 {{ reserved0: {:?}, reserved1: {:?}, wupdc2: {:?}, reserved3: {:?}, reserved4: {:?}, reserved5: {:?}, wupdc6: {:?}, wupdc7: {:?}, wupdc8: {:?}, wupdc9: {:?}, wupdc10: {:?}, wupdc11: {:?}, wupdc12: {:?}, reserved13: {:?}, reserved14: {:?}, reserved15: {:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.wupdc2(),
            self.reserved3(),
            self.reserved4(),
            self.reserved5(),
            self.wupdc6(),
            self.wupdc7(),
            self.wupdc8(),
            self.wupdc9(),
            self.wupdc10(),
            self.wupdc11(),
            self.wupdc12(),
            self.reserved13(),
            self.reserved14(),
            self.reserved15()
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
    pub const fn wupdc16(&self) -> Wupdc16 {
        let val = (self.0 >> 0usize) & 0x03;
        Wupdc16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc16(&mut self, val: Wupdc16) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc17(&self) -> Wupdc17 {
        let val = (self.0 >> 2usize) & 0x03;
        Wupdc17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc17(&mut self, val: Wupdc17) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc18(&self) -> Wupdc18 {
        let val = (self.0 >> 4usize) & 0x03;
        Wupdc18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc18(&mut self, val: Wupdc18) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc19(&self) -> Wupdc19 {
        let val = (self.0 >> 6usize) & 0x03;
        Wupdc19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc19(&mut self, val: Wupdc19) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc20(&self) -> Wupdc20 {
        let val = (self.0 >> 8usize) & 0x03;
        Wupdc20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc20(&mut self, val: Wupdc20) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> Pdc2Reserved21 {
        let val = (self.0 >> 10usize) & 0x03;
        Pdc2Reserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: Pdc2Reserved21) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc22(&self) -> Wupdc22 {
        let val = (self.0 >> 12usize) & 0x03;
        Wupdc22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc22(&mut self, val: Wupdc22) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc23(&self) -> Wupdc23 {
        let val = (self.0 >> 14usize) & 0x03;
        Wupdc23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc23(&mut self, val: Wupdc23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc24(&self) -> Wupdc24 {
        let val = (self.0 >> 16usize) & 0x03;
        Wupdc24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc24(&mut self, val: Wupdc24) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc25(&self) -> Wupdc25 {
        let val = (self.0 >> 18usize) & 0x03;
        Wupdc25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc25(&mut self, val: Wupdc25) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc26(&self) -> Wupdc26 {
        let val = (self.0 >> 20usize) & 0x03;
        Wupdc26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc26(&mut self, val: Wupdc26) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc27(&self) -> Wupdc27 {
        let val = (self.0 >> 22usize) & 0x03;
        Wupdc27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc27(&mut self, val: Wupdc27) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc28(&self) -> Wupdc28 {
        let val = (self.0 >> 24usize) & 0x03;
        Wupdc28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc28(&mut self, val: Wupdc28) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc29(&self) -> Wupdc29 {
        let val = (self.0 >> 26usize) & 0x03;
        Wupdc29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc29(&mut self, val: Wupdc29) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc30(&self) -> Wupdc30 {
        let val = (self.0 >> 28usize) & 0x03;
        Wupdc30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc30(&mut self, val: Wupdc30) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc31(&self) -> Wupdc31 {
        let val = (self.0 >> 30usize) & 0x03;
        Wupdc31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc31(&mut self, val: Wupdc31) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
            .field("wupdc16", &self.wupdc16())
            .field("wupdc17", &self.wupdc17())
            .field("wupdc18", &self.wupdc18())
            .field("wupdc19", &self.wupdc19())
            .field("wupdc20", &self.wupdc20())
            .field("reserved21", &self.reserved21())
            .field("wupdc22", &self.wupdc22())
            .field("wupdc23", &self.wupdc23())
            .field("wupdc24", &self.wupdc24())
            .field("wupdc25", &self.wupdc25())
            .field("wupdc26", &self.wupdc26())
            .field("wupdc27", &self.wupdc27())
            .field("wupdc28", &self.wupdc28())
            .field("wupdc29", &self.wupdc29())
            .field("wupdc30", &self.wupdc30())
            .field("wupdc31", &self.wupdc31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdc2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pdc2 {{ wupdc16: {:?}, wupdc17: {:?}, wupdc18: {:?}, wupdc19: {:?}, wupdc20: {:?}, reserved21: {:?}, wupdc22: {:?}, wupdc23: {:?}, wupdc24: {:?}, wupdc25: {:?}, wupdc26: {:?}, wupdc27: {:?}, wupdc28: {:?}, wupdc29: {:?}, wupdc30: {:?}, wupdc31: {:?} }}",
            self.wupdc16(),
            self.wupdc17(),
            self.wupdc18(),
            self.wupdc19(),
            self.wupdc20(),
            self.reserved21(),
            self.wupdc22(),
            self.wupdc23(),
            self.wupdc24(),
            self.wupdc25(),
            self.wupdc26(),
            self.wupdc27(),
            self.wupdc28(),
            self.wupdc29(),
            self.wupdc30(),
            self.wupdc31()
        )
    }
}
#[doc = "Pin Enable 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pe1(pub u32);
impl Pe1 {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> Pe1Reserved0 {
        let val = (self.0 >> 0usize) & 0x03;
        Pe1Reserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: Pe1Reserved0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> Pe1Reserved1 {
        let val = (self.0 >> 2usize) & 0x03;
        Pe1Reserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: Pe1Reserved1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe2(&self) -> Wupe2 {
        let val = (self.0 >> 4usize) & 0x03;
        Wupe2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe2(&mut self, val: Wupe2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> Pe1Reserved3 {
        let val = (self.0 >> 6usize) & 0x03;
        Pe1Reserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: Pe1Reserved3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> Pe1Reserved4 {
        let val = (self.0 >> 8usize) & 0x03;
        Pe1Reserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: Pe1Reserved4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> Pe1Reserved5 {
        let val = (self.0 >> 10usize) & 0x03;
        Pe1Reserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: Pe1Reserved5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe6(&self) -> Wupe6 {
        let val = (self.0 >> 12usize) & 0x03;
        Wupe6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe6(&mut self, val: Wupe6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe7(&self) -> Wupe7 {
        let val = (self.0 >> 14usize) & 0x03;
        Wupe7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe7(&mut self, val: Wupe7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe8(&self) -> Wupe8 {
        let val = (self.0 >> 16usize) & 0x03;
        Wupe8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe8(&mut self, val: Wupe8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe9(&self) -> Wupe9 {
        let val = (self.0 >> 18usize) & 0x03;
        Wupe9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe9(&mut self, val: Wupe9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe10(&self) -> Wupe10 {
        let val = (self.0 >> 20usize) & 0x03;
        Wupe10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe10(&mut self, val: Wupe10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe11(&self) -> Wupe11 {
        let val = (self.0 >> 22usize) & 0x03;
        Wupe11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe11(&mut self, val: Wupe11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe12(&self) -> Wupe12 {
        let val = (self.0 >> 24usize) & 0x03;
        Wupe12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe12(&mut self, val: Wupe12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> Pe1Reserved13 {
        let val = (self.0 >> 26usize) & 0x03;
        Pe1Reserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: Pe1Reserved13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> Pe1Reserved14 {
        let val = (self.0 >> 28usize) & 0x03;
        Pe1Reserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: Pe1Reserved14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> Pe1Reserved15 {
        let val = (self.0 >> 30usize) & 0x03;
        Pe1Reserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: Pe1Reserved15) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("wupe2", &self.wupe2())
            .field("reserved3", &self.reserved3())
            .field("reserved4", &self.reserved4())
            .field("reserved5", &self.reserved5())
            .field("wupe6", &self.wupe6())
            .field("wupe7", &self.wupe7())
            .field("wupe8", &self.wupe8())
            .field("wupe9", &self.wupe9())
            .field("wupe10", &self.wupe10())
            .field("wupe11", &self.wupe11())
            .field("wupe12", &self.wupe12())
            .field("reserved13", &self.reserved13())
            .field("reserved14", &self.reserved14())
            .field("reserved15", &self.reserved15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pe1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pe1 {{ reserved0: {:?}, reserved1: {:?}, wupe2: {:?}, reserved3: {:?}, reserved4: {:?}, reserved5: {:?}, wupe6: {:?}, wupe7: {:?}, wupe8: {:?}, wupe9: {:?}, wupe10: {:?}, wupe11: {:?}, wupe12: {:?}, reserved13: {:?}, reserved14: {:?}, reserved15: {:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.wupe2(),
            self.reserved3(),
            self.reserved4(),
            self.reserved5(),
            self.wupe6(),
            self.wupe7(),
            self.wupe8(),
            self.wupe9(),
            self.wupe10(),
            self.wupe11(),
            self.wupe12(),
            self.reserved13(),
            self.reserved14(),
            self.reserved15()
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
    pub const fn wupe16(&self) -> Wupe16 {
        let val = (self.0 >> 0usize) & 0x03;
        Wupe16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe16(&mut self, val: Wupe16) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe17(&self) -> Wupe17 {
        let val = (self.0 >> 2usize) & 0x03;
        Wupe17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe17(&mut self, val: Wupe17) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe18(&self) -> Wupe18 {
        let val = (self.0 >> 4usize) & 0x03;
        Wupe18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe18(&mut self, val: Wupe18) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe19(&self) -> Wupe19 {
        let val = (self.0 >> 6usize) & 0x03;
        Wupe19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe19(&mut self, val: Wupe19) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe20(&self) -> Wupe20 {
        let val = (self.0 >> 8usize) & 0x03;
        Wupe20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe20(&mut self, val: Wupe20) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> Pe2Reserved21 {
        let val = (self.0 >> 10usize) & 0x03;
        Pe2Reserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: Pe2Reserved21) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe22(&self) -> Wupe22 {
        let val = (self.0 >> 12usize) & 0x03;
        Wupe22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe22(&mut self, val: Wupe22) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe23(&self) -> Wupe23 {
        let val = (self.0 >> 14usize) & 0x03;
        Wupe23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe23(&mut self, val: Wupe23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe24(&self) -> Wupe24 {
        let val = (self.0 >> 16usize) & 0x03;
        Wupe24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe24(&mut self, val: Wupe24) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe25(&self) -> Wupe25 {
        let val = (self.0 >> 18usize) & 0x03;
        Wupe25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe25(&mut self, val: Wupe25) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe26(&self) -> Wupe26 {
        let val = (self.0 >> 20usize) & 0x03;
        Wupe26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe26(&mut self, val: Wupe26) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe27(&self) -> Wupe27 {
        let val = (self.0 >> 22usize) & 0x03;
        Wupe27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe27(&mut self, val: Wupe27) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe28(&self) -> Wupe28 {
        let val = (self.0 >> 24usize) & 0x03;
        Wupe28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe28(&mut self, val: Wupe28) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe29(&self) -> Wupe29 {
        let val = (self.0 >> 26usize) & 0x03;
        Wupe29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe29(&mut self, val: Wupe29) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe30(&self) -> Wupe30 {
        let val = (self.0 >> 28usize) & 0x03;
        Wupe30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe30(&mut self, val: Wupe30) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe31(&self) -> Wupe31 {
        let val = (self.0 >> 30usize) & 0x03;
        Wupe31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe31(&mut self, val: Wupe31) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
            .field("wupe16", &self.wupe16())
            .field("wupe17", &self.wupe17())
            .field("wupe18", &self.wupe18())
            .field("wupe19", &self.wupe19())
            .field("wupe20", &self.wupe20())
            .field("reserved21", &self.reserved21())
            .field("wupe22", &self.wupe22())
            .field("wupe23", &self.wupe23())
            .field("wupe24", &self.wupe24())
            .field("wupe25", &self.wupe25())
            .field("wupe26", &self.wupe26())
            .field("wupe27", &self.wupe27())
            .field("wupe28", &self.wupe28())
            .field("wupe29", &self.wupe29())
            .field("wupe30", &self.wupe30())
            .field("wupe31", &self.wupe31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pe2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pe2 {{ wupe16: {:?}, wupe17: {:?}, wupe18: {:?}, wupe19: {:?}, wupe20: {:?}, reserved21: {:?}, wupe22: {:?}, wupe23: {:?}, wupe24: {:?}, wupe25: {:?}, wupe26: {:?}, wupe27: {:?}, wupe28: {:?}, wupe29: {:?}, wupe30: {:?}, wupe31: {:?} }}",
            self.wupe16(),
            self.wupe17(),
            self.wupe18(),
            self.wupe19(),
            self.wupe20(),
            self.reserved21(),
            self.wupe22(),
            self.wupe23(),
            self.wupe24(),
            self.wupe25(),
            self.wupe26(),
            self.wupe27(),
            self.wupe28(),
            self.wupe29(),
            self.wupe30(),
            self.wupe31()
        )
    }
}
#[doc = "Pin Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pf(pub u32);
impl Pf {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf24(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf24(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf25(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf25(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf26(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf26(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf27(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf27(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf29(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf29(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf30(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf30(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wuf31(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-up Flag for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wuf31(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("wuf2", &self.wuf2())
            .field("reserved3", &self.reserved3())
            .field("reserved4", &self.reserved4())
            .field("reserved5", &self.reserved5())
            .field("wuf6", &self.wuf6())
            .field("wuf7", &self.wuf7())
            .field("wuf8", &self.wuf8())
            .field("wuf9", &self.wuf9())
            .field("wuf10", &self.wuf10())
            .field("wuf11", &self.wuf11())
            .field("wuf12", &self.wuf12())
            .field("reserved13", &self.reserved13())
            .field("reserved14", &self.reserved14())
            .field("reserved15", &self.reserved15())
            .field("wuf16", &self.wuf16())
            .field("wuf17", &self.wuf17())
            .field("wuf18", &self.wuf18())
            .field("wuf19", &self.wuf19())
            .field("wuf20", &self.wuf20())
            .field("reserved21", &self.reserved21())
            .field("wuf22", &self.wuf22())
            .field("wuf23", &self.wuf23())
            .field("wuf24", &self.wuf24())
            .field("wuf25", &self.wuf25())
            .field("wuf26", &self.wuf26())
            .field("wuf27", &self.wuf27())
            .field("wuf28", &self.wuf28())
            .field("wuf29", &self.wuf29())
            .field("wuf30", &self.wuf30())
            .field("wuf31", &self.wuf31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pf {{ reserved0: {=bool:?}, reserved1: {=bool:?}, wuf2: {=bool:?}, reserved3: {=bool:?}, reserved4: {=bool:?}, reserved5: {=bool:?}, wuf6: {=bool:?}, wuf7: {=bool:?}, wuf8: {=bool:?}, wuf9: {=bool:?}, wuf10: {=bool:?}, wuf11: {=bool:?}, wuf12: {=bool:?}, reserved13: {=bool:?}, reserved14: {=bool:?}, reserved15: {=bool:?}, wuf16: {=bool:?}, wuf17: {=bool:?}, wuf18: {=bool:?}, wuf19: {=bool:?}, wuf20: {=bool:?}, reserved21: {=bool:?}, wuf22: {=bool:?}, wuf23: {=bool:?}, wuf24: {=bool:?}, wuf25: {=bool:?}, wuf26: {=bool:?}, wuf27: {=bool:?}, wuf28: {=bool:?}, wuf29: {=bool:?}, wuf30: {=bool:?}, wuf31: {=bool:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.wuf2(),
            self.reserved3(),
            self.reserved4(),
            self.reserved5(),
            self.wuf6(),
            self.wuf7(),
            self.wuf8(),
            self.wuf9(),
            self.wuf10(),
            self.wuf11(),
            self.wuf12(),
            self.reserved13(),
            self.reserved14(),
            self.reserved15(),
            self.wuf16(),
            self.wuf17(),
            self.wuf18(),
            self.wuf19(),
            self.wuf20(),
            self.reserved21(),
            self.wuf22(),
            self.wuf23(),
            self.wuf24(),
            self.wuf25(),
            self.wuf26(),
            self.wuf27(),
            self.wuf28(),
            self.wuf29(),
            self.wuf30(),
            self.wuf31()
        )
    }
}
#[doc = "Pin Mode Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmc(pub u32);
impl Pmc {
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> PmcReserved0 {
        let val = (self.0 >> 0usize) & 0x01;
        PmcReserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: PmcReserved0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> PmcReserved1 {
        let val = (self.0 >> 1usize) & 0x01;
        PmcReserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: PmcReserved1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc2(&self) -> Wupmc2 {
        let val = (self.0 >> 2usize) & 0x01;
        Wupmc2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc2(&mut self, val: Wupmc2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> PmcReserved3 {
        let val = (self.0 >> 3usize) & 0x01;
        PmcReserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: PmcReserved3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> PmcReserved4 {
        let val = (self.0 >> 4usize) & 0x01;
        PmcReserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: PmcReserved4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> PmcReserved5 {
        let val = (self.0 >> 5usize) & 0x01;
        PmcReserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: PmcReserved5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc6(&self) -> Wupmc6 {
        let val = (self.0 >> 6usize) & 0x01;
        Wupmc6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc6(&mut self, val: Wupmc6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc7(&self) -> Wupmc7 {
        let val = (self.0 >> 7usize) & 0x01;
        Wupmc7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc7(&mut self, val: Wupmc7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc8(&self) -> Wupmc8 {
        let val = (self.0 >> 8usize) & 0x01;
        Wupmc8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc8(&mut self, val: Wupmc8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc9(&self) -> Wupmc9 {
        let val = (self.0 >> 9usize) & 0x01;
        Wupmc9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc9(&mut self, val: Wupmc9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc10(&self) -> Wupmc10 {
        let val = (self.0 >> 10usize) & 0x01;
        Wupmc10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc10(&mut self, val: Wupmc10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc11(&self) -> Wupmc11 {
        let val = (self.0 >> 11usize) & 0x01;
        Wupmc11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc11(&mut self, val: Wupmc11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc12(&self) -> Wupmc12 {
        let val = (self.0 >> 12usize) & 0x01;
        Wupmc12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc12(&mut self, val: Wupmc12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> PmcReserved13 {
        let val = (self.0 >> 13usize) & 0x01;
        PmcReserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: PmcReserved13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> PmcReserved14 {
        let val = (self.0 >> 14usize) & 0x01;
        PmcReserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: PmcReserved14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> PmcReserved15 {
        let val = (self.0 >> 15usize) & 0x01;
        PmcReserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: PmcReserved15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc16(&self) -> Wupmc16 {
        let val = (self.0 >> 16usize) & 0x01;
        Wupmc16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc16(&mut self, val: Wupmc16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc17(&self) -> Wupmc17 {
        let val = (self.0 >> 17usize) & 0x01;
        Wupmc17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc17(&mut self, val: Wupmc17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc18(&self) -> Wupmc18 {
        let val = (self.0 >> 18usize) & 0x01;
        Wupmc18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc18(&mut self, val: Wupmc18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc19(&self) -> Wupmc19 {
        let val = (self.0 >> 19usize) & 0x01;
        Wupmc19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc19(&mut self, val: Wupmc19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc20(&self) -> Wupmc20 {
        let val = (self.0 >> 20usize) & 0x01;
        Wupmc20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc20(&mut self, val: Wupmc20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> PmcReserved21 {
        let val = (self.0 >> 21usize) & 0x01;
        PmcReserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: PmcReserved21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc22(&self) -> Wupmc22 {
        let val = (self.0 >> 22usize) & 0x01;
        Wupmc22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc22(&mut self, val: Wupmc22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc23(&self) -> Wupmc23 {
        let val = (self.0 >> 23usize) & 0x01;
        Wupmc23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc23(&mut self, val: Wupmc23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc24(&self) -> Wupmc24 {
        let val = (self.0 >> 24usize) & 0x01;
        Wupmc24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc24(&mut self, val: Wupmc24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc25(&self) -> Wupmc25 {
        let val = (self.0 >> 25usize) & 0x01;
        Wupmc25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc25(&mut self, val: Wupmc25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc26(&self) -> Wupmc26 {
        let val = (self.0 >> 26usize) & 0x01;
        Wupmc26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc26(&mut self, val: Wupmc26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc27(&self) -> Wupmc27 {
        let val = (self.0 >> 27usize) & 0x01;
        Wupmc27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc27(&mut self, val: Wupmc27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc28(&self) -> Wupmc28 {
        let val = (self.0 >> 28usize) & 0x01;
        Wupmc28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc28(&mut self, val: Wupmc28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc29(&self) -> Wupmc29 {
        let val = (self.0 >> 29usize) & 0x01;
        Wupmc29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc29(&mut self, val: Wupmc29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc30(&self) -> Wupmc30 {
        let val = (self.0 >> 30usize) & 0x01;
        Wupmc30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc30(&mut self, val: Wupmc30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc31(&self) -> Wupmc31 {
        let val = (self.0 >> 31usize) & 0x01;
        Wupmc31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc31(&mut self, val: Wupmc31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("wupmc2", &self.wupmc2())
            .field("reserved3", &self.reserved3())
            .field("reserved4", &self.reserved4())
            .field("reserved5", &self.reserved5())
            .field("wupmc6", &self.wupmc6())
            .field("wupmc7", &self.wupmc7())
            .field("wupmc8", &self.wupmc8())
            .field("wupmc9", &self.wupmc9())
            .field("wupmc10", &self.wupmc10())
            .field("wupmc11", &self.wupmc11())
            .field("wupmc12", &self.wupmc12())
            .field("reserved13", &self.reserved13())
            .field("reserved14", &self.reserved14())
            .field("reserved15", &self.reserved15())
            .field("wupmc16", &self.wupmc16())
            .field("wupmc17", &self.wupmc17())
            .field("wupmc18", &self.wupmc18())
            .field("wupmc19", &self.wupmc19())
            .field("wupmc20", &self.wupmc20())
            .field("reserved21", &self.reserved21())
            .field("wupmc22", &self.wupmc22())
            .field("wupmc23", &self.wupmc23())
            .field("wupmc24", &self.wupmc24())
            .field("wupmc25", &self.wupmc25())
            .field("wupmc26", &self.wupmc26())
            .field("wupmc27", &self.wupmc27())
            .field("wupmc28", &self.wupmc28())
            .field("wupmc29", &self.wupmc29())
            .field("wupmc30", &self.wupmc30())
            .field("wupmc31", &self.wupmc31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmc {{ reserved0: {:?}, reserved1: {:?}, wupmc2: {:?}, reserved3: {:?}, reserved4: {:?}, reserved5: {:?}, wupmc6: {:?}, wupmc7: {:?}, wupmc8: {:?}, wupmc9: {:?}, wupmc10: {:?}, wupmc11: {:?}, wupmc12: {:?}, reserved13: {:?}, reserved14: {:?}, reserved15: {:?}, wupmc16: {:?}, wupmc17: {:?}, wupmc18: {:?}, wupmc19: {:?}, wupmc20: {:?}, reserved21: {:?}, wupmc22: {:?}, wupmc23: {:?}, wupmc24: {:?}, wupmc25: {:?}, wupmc26: {:?}, wupmc27: {:?}, wupmc28: {:?}, wupmc29: {:?}, wupmc30: {:?}, wupmc31: {:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.wupmc2(),
            self.reserved3(),
            self.reserved4(),
            self.reserved5(),
            self.wupmc6(),
            self.wupmc7(),
            self.wupmc8(),
            self.wupmc9(),
            self.wupmc10(),
            self.wupmc11(),
            self.wupmc12(),
            self.reserved13(),
            self.reserved14(),
            self.reserved15(),
            self.wupmc16(),
            self.wupmc17(),
            self.wupmc18(),
            self.wupmc19(),
            self.wupmc20(),
            self.reserved21(),
            self.wupmc22(),
            self.wupmc23(),
            self.wupmc24(),
            self.wupmc25(),
            self.wupmc26(),
            self.wupmc27(),
            self.wupmc28(),
            self.wupmc29(),
            self.wupmc30(),
            self.wupmc31()
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
pub enum Pdc1Reserved0 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved0 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved0 {
        Pdc1Reserved0::from_bits(val)
    }
}
impl From<Pdc1Reserved0> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved0) -> u8 {
        Pdc1Reserved0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved1 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved1 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved1 {
        Pdc1Reserved1::from_bits(val)
    }
}
impl From<Pdc1Reserved1> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved1) -> u8 {
        Pdc1Reserved1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved13 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved13 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved13 {
        Pdc1Reserved13::from_bits(val)
    }
}
impl From<Pdc1Reserved13> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved13) -> u8 {
        Pdc1Reserved13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved14 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved14 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved14 {
        Pdc1Reserved14::from_bits(val)
    }
}
impl From<Pdc1Reserved14> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved14) -> u8 {
        Pdc1Reserved14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved15 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved15 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved15 {
        Pdc1Reserved15::from_bits(val)
    }
}
impl From<Pdc1Reserved15> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved15) -> u8 {
        Pdc1Reserved15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved3 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved3 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved3 {
        Pdc1Reserved3::from_bits(val)
    }
}
impl From<Pdc1Reserved3> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved3) -> u8 {
        Pdc1Reserved3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved4 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved4 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved4 {
        Pdc1Reserved4::from_bits(val)
    }
}
impl From<Pdc1Reserved4> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved4) -> u8 {
        Pdc1Reserved4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc1Reserved5 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc1Reserved5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc1Reserved5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc1Reserved5 {
    #[inline(always)]
    fn from(val: u8) -> Pdc1Reserved5 {
        Pdc1Reserved5::from_bits(val)
    }
}
impl From<Pdc1Reserved5> for u8 {
    #[inline(always)]
    fn from(val: Pdc1Reserved5) -> u8 {
        Pdc1Reserved5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdc2Reserved21 {
    #[doc = "Not supported."]
    Interrupt = 0x0,
    #[doc = "Not supported."]
    DmaReq = 0x01,
    #[doc = "Not supported."]
    Trigger = 0x02,
    #[doc = "Not supported."]
    Res = 0x03,
}
impl Pdc2Reserved21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdc2Reserved21 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdc2Reserved21 {
    #[inline(always)]
    fn from(val: u8) -> Pdc2Reserved21 {
        Pdc2Reserved21::from_bits(val)
    }
}
impl From<Pdc2Reserved21> for u8 {
    #[inline(always)]
    fn from(val: Pdc2Reserved21) -> u8 {
        Pdc2Reserved21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved0 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved0 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved0 {
        Pe1Reserved0::from_bits(val)
    }
}
impl From<Pe1Reserved0> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved0) -> u8 {
        Pe1Reserved0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved1 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved1 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved1 {
        Pe1Reserved1::from_bits(val)
    }
}
impl From<Pe1Reserved1> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved1) -> u8 {
        Pe1Reserved1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved13 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved13 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved13 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved13 {
        Pe1Reserved13::from_bits(val)
    }
}
impl From<Pe1Reserved13> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved13) -> u8 {
        Pe1Reserved13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved14 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved14 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved14 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved14 {
        Pe1Reserved14::from_bits(val)
    }
}
impl From<Pe1Reserved14> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved14) -> u8 {
        Pe1Reserved14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved15 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved15 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved15 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved15 {
        Pe1Reserved15::from_bits(val)
    }
}
impl From<Pe1Reserved15> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved15) -> u8 {
        Pe1Reserved15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved3 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved3 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved3 {
        Pe1Reserved3::from_bits(val)
    }
}
impl From<Pe1Reserved3> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved3) -> u8 {
        Pe1Reserved3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved4 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved4 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved4 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved4 {
        Pe1Reserved4::from_bits(val)
    }
}
impl From<Pe1Reserved4> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved4) -> u8 {
        Pe1Reserved4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe1Reserved5 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe1Reserved5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe1Reserved5 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe1Reserved5 {
    #[inline(always)]
    fn from(val: u8) -> Pe1Reserved5 {
        Pe1Reserved5::from_bits(val)
    }
}
impl From<Pe1Reserved5> for u8 {
    #[inline(always)]
    fn from(val: Pe1Reserved5) -> u8 {
        Pe1Reserved5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pe2Reserved21 {
    #[doc = "Not supported."]
    Disable = 0x0,
    #[doc = "Not supported."]
    EnRiseHi = 0x01,
    #[doc = "Not supported."]
    EnFallLo = 0x02,
    #[doc = "Not supported."]
    EnAny = 0x03,
}
impl Pe2Reserved21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pe2Reserved21 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pe2Reserved21 {
    #[inline(always)]
    fn from(val: u8) -> Pe2Reserved21 {
        Pe2Reserved21::from_bits(val)
    }
}
impl From<Pe2Reserved21> for u8 {
    #[inline(always)]
    fn from(val: Pe2Reserved21) -> u8 {
        Pe2Reserved21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved0 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved0 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved0 {
        PmcReserved0::from_bits(val)
    }
}
impl From<PmcReserved0> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved0) -> u8 {
        PmcReserved0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved1 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved1 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved1 {
        PmcReserved1::from_bits(val)
    }
}
impl From<PmcReserved1> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved1) -> u8 {
        PmcReserved1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved13 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved13 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved13 {
        PmcReserved13::from_bits(val)
    }
}
impl From<PmcReserved13> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved13) -> u8 {
        PmcReserved13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved14 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved14 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved14 {
        PmcReserved14::from_bits(val)
    }
}
impl From<PmcReserved14> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved14) -> u8 {
        PmcReserved14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved15 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved15 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved15 {
        PmcReserved15::from_bits(val)
    }
}
impl From<PmcReserved15> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved15) -> u8 {
        PmcReserved15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved21 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved21 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved21 {
        PmcReserved21::from_bits(val)
    }
}
impl From<PmcReserved21> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved21) -> u8 {
        PmcReserved21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved3 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved3 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved3 {
        PmcReserved3::from_bits(val)
    }
}
impl From<PmcReserved3> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved3) -> u8 {
        PmcReserved3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved4 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved4 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved4 {
        PmcReserved4::from_bits(val)
    }
}
impl From<PmcReserved4> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved4) -> u8 {
        PmcReserved4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmcReserved5 {
    #[doc = "Not supported."]
    LowPwrOnly = 0x0,
    #[doc = "Not supported."]
    AnyPwr = 0x01,
}
impl PmcReserved5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmcReserved5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmcReserved5 {
    #[inline(always)]
    fn from(val: u8) -> PmcReserved5 {
        PmcReserved5::from_bits(val)
    }
}
impl From<PmcReserved5> for u8 {
    #[inline(always)]
    fn from(val: PmcReserved5) -> u8 {
        PmcReserved5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc10 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc10 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc10 {
        Wupdc10::from_bits(val)
    }
}
impl From<Wupdc10> for u8 {
    #[inline(always)]
    fn from(val: Wupdc10) -> u8 {
        Wupdc10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc11 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc11 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc11 {
        Wupdc11::from_bits(val)
    }
}
impl From<Wupdc11> for u8 {
    #[inline(always)]
    fn from(val: Wupdc11) -> u8 {
        Wupdc11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc12 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc12 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc12 {
        Wupdc12::from_bits(val)
    }
}
impl From<Wupdc12> for u8 {
    #[inline(always)]
    fn from(val: Wupdc12) -> u8 {
        Wupdc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc16 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc16 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc16 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc16 {
        Wupdc16::from_bits(val)
    }
}
impl From<Wupdc16> for u8 {
    #[inline(always)]
    fn from(val: Wupdc16) -> u8 {
        Wupdc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc17 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc17 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc17 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc17 {
        Wupdc17::from_bits(val)
    }
}
impl From<Wupdc17> for u8 {
    #[inline(always)]
    fn from(val: Wupdc17) -> u8 {
        Wupdc17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc18 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc18 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc18 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc18 {
        Wupdc18::from_bits(val)
    }
}
impl From<Wupdc18> for u8 {
    #[inline(always)]
    fn from(val: Wupdc18) -> u8 {
        Wupdc18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc19 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc19 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc19 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc19 {
        Wupdc19::from_bits(val)
    }
}
impl From<Wupdc19> for u8 {
    #[inline(always)]
    fn from(val: Wupdc19) -> u8 {
        Wupdc19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc2 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc2 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc2 {
        Wupdc2::from_bits(val)
    }
}
impl From<Wupdc2> for u8 {
    #[inline(always)]
    fn from(val: Wupdc2) -> u8 {
        Wupdc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc20 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc20 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc20 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc20 {
        Wupdc20::from_bits(val)
    }
}
impl From<Wupdc20> for u8 {
    #[inline(always)]
    fn from(val: Wupdc20) -> u8 {
        Wupdc20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc22 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc22 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc22 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc22 {
        Wupdc22::from_bits(val)
    }
}
impl From<Wupdc22> for u8 {
    #[inline(always)]
    fn from(val: Wupdc22) -> u8 {
        Wupdc22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc23 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc23 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc23 {
        Wupdc23::from_bits(val)
    }
}
impl From<Wupdc23> for u8 {
    #[inline(always)]
    fn from(val: Wupdc23) -> u8 {
        Wupdc23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc24 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc24 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc24 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc24 {
        Wupdc24::from_bits(val)
    }
}
impl From<Wupdc24> for u8 {
    #[inline(always)]
    fn from(val: Wupdc24) -> u8 {
        Wupdc24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc25 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc25 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc25 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc25 {
        Wupdc25::from_bits(val)
    }
}
impl From<Wupdc25> for u8 {
    #[inline(always)]
    fn from(val: Wupdc25) -> u8 {
        Wupdc25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc26 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc26 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc26 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc26 {
        Wupdc26::from_bits(val)
    }
}
impl From<Wupdc26> for u8 {
    #[inline(always)]
    fn from(val: Wupdc26) -> u8 {
        Wupdc26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc27 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc27 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc27 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc27 {
        Wupdc27::from_bits(val)
    }
}
impl From<Wupdc27> for u8 {
    #[inline(always)]
    fn from(val: Wupdc27) -> u8 {
        Wupdc27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc28 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc28 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc28 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc28 {
        Wupdc28::from_bits(val)
    }
}
impl From<Wupdc28> for u8 {
    #[inline(always)]
    fn from(val: Wupdc28) -> u8 {
        Wupdc28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc29 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc29 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc29 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc29 {
        Wupdc29::from_bits(val)
    }
}
impl From<Wupdc29> for u8 {
    #[inline(always)]
    fn from(val: Wupdc29) -> u8 {
        Wupdc29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc30 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc30 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc30 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc30 {
        Wupdc30::from_bits(val)
    }
}
impl From<Wupdc30> for u8 {
    #[inline(always)]
    fn from(val: Wupdc30) -> u8 {
        Wupdc30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc31 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc31 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc31 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc31 {
        Wupdc31::from_bits(val)
    }
}
impl From<Wupdc31> for u8 {
    #[inline(always)]
    fn from(val: Wupdc31) -> u8 {
        Wupdc31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc6 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc6 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc6 {
        Wupdc6::from_bits(val)
    }
}
impl From<Wupdc6> for u8 {
    #[inline(always)]
    fn from(val: Wupdc6) -> u8 {
        Wupdc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc7 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc7 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc7 {
        Wupdc7::from_bits(val)
    }
}
impl From<Wupdc7> for u8 {
    #[inline(always)]
    fn from(val: Wupdc7) -> u8 {
        Wupdc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc8 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc8 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc8 {
        Wupdc8::from_bits(val)
    }
}
impl From<Wupdc8> for u8 {
    #[inline(always)]
    fn from(val: Wupdc8) -> u8 {
        Wupdc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupdc9 {
    #[doc = "Interrupt."]
    Interrupt = 0x0,
    #[doc = "DMA request."]
    DmaReq = 0x01,
    #[doc = "Trigger event."]
    Trigger = 0x02,
    #[doc = "Reserved."]
    Res = 0x03,
}
impl Wupdc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupdc9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupdc9 {
    #[inline(always)]
    fn from(val: u8) -> Wupdc9 {
        Wupdc9::from_bits(val)
    }
}
impl From<Wupdc9> for u8 {
    #[inline(always)]
    fn from(val: Wupdc9) -> u8 {
        Wupdc9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe10 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe10 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe10 {
    #[inline(always)]
    fn from(val: u8) -> Wupe10 {
        Wupe10::from_bits(val)
    }
}
impl From<Wupe10> for u8 {
    #[inline(always)]
    fn from(val: Wupe10) -> u8 {
        Wupe10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe11 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe11 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe11 {
    #[inline(always)]
    fn from(val: u8) -> Wupe11 {
        Wupe11::from_bits(val)
    }
}
impl From<Wupe11> for u8 {
    #[inline(always)]
    fn from(val: Wupe11) -> u8 {
        Wupe11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe12 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe12 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe12 {
    #[inline(always)]
    fn from(val: u8) -> Wupe12 {
        Wupe12::from_bits(val)
    }
}
impl From<Wupe12> for u8 {
    #[inline(always)]
    fn from(val: Wupe12) -> u8 {
        Wupe12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe16 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe16 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe16 {
    #[inline(always)]
    fn from(val: u8) -> Wupe16 {
        Wupe16::from_bits(val)
    }
}
impl From<Wupe16> for u8 {
    #[inline(always)]
    fn from(val: Wupe16) -> u8 {
        Wupe16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe17 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe17 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe17 {
    #[inline(always)]
    fn from(val: u8) -> Wupe17 {
        Wupe17::from_bits(val)
    }
}
impl From<Wupe17> for u8 {
    #[inline(always)]
    fn from(val: Wupe17) -> u8 {
        Wupe17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe18 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe18 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe18 {
    #[inline(always)]
    fn from(val: u8) -> Wupe18 {
        Wupe18::from_bits(val)
    }
}
impl From<Wupe18> for u8 {
    #[inline(always)]
    fn from(val: Wupe18) -> u8 {
        Wupe18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe19 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe19 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe19 {
    #[inline(always)]
    fn from(val: u8) -> Wupe19 {
        Wupe19::from_bits(val)
    }
}
impl From<Wupe19> for u8 {
    #[inline(always)]
    fn from(val: Wupe19) -> u8 {
        Wupe19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe2 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe2 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe2 {
    #[inline(always)]
    fn from(val: u8) -> Wupe2 {
        Wupe2::from_bits(val)
    }
}
impl From<Wupe2> for u8 {
    #[inline(always)]
    fn from(val: Wupe2) -> u8 {
        Wupe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe20 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe20 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe20 {
    #[inline(always)]
    fn from(val: u8) -> Wupe20 {
        Wupe20::from_bits(val)
    }
}
impl From<Wupe20> for u8 {
    #[inline(always)]
    fn from(val: Wupe20) -> u8 {
        Wupe20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe22 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe22 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe22 {
    #[inline(always)]
    fn from(val: u8) -> Wupe22 {
        Wupe22::from_bits(val)
    }
}
impl From<Wupe22> for u8 {
    #[inline(always)]
    fn from(val: Wupe22) -> u8 {
        Wupe22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe23 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe23 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe23 {
    #[inline(always)]
    fn from(val: u8) -> Wupe23 {
        Wupe23::from_bits(val)
    }
}
impl From<Wupe23> for u8 {
    #[inline(always)]
    fn from(val: Wupe23) -> u8 {
        Wupe23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe24 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe24 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe24 {
    #[inline(always)]
    fn from(val: u8) -> Wupe24 {
        Wupe24::from_bits(val)
    }
}
impl From<Wupe24> for u8 {
    #[inline(always)]
    fn from(val: Wupe24) -> u8 {
        Wupe24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe25 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe25 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe25 {
    #[inline(always)]
    fn from(val: u8) -> Wupe25 {
        Wupe25::from_bits(val)
    }
}
impl From<Wupe25> for u8 {
    #[inline(always)]
    fn from(val: Wupe25) -> u8 {
        Wupe25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe26 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe26 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe26 {
    #[inline(always)]
    fn from(val: u8) -> Wupe26 {
        Wupe26::from_bits(val)
    }
}
impl From<Wupe26> for u8 {
    #[inline(always)]
    fn from(val: Wupe26) -> u8 {
        Wupe26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe27 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe27 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe27 {
    #[inline(always)]
    fn from(val: u8) -> Wupe27 {
        Wupe27::from_bits(val)
    }
}
impl From<Wupe27> for u8 {
    #[inline(always)]
    fn from(val: Wupe27) -> u8 {
        Wupe27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe28 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe28 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe28 {
    #[inline(always)]
    fn from(val: u8) -> Wupe28 {
        Wupe28::from_bits(val)
    }
}
impl From<Wupe28> for u8 {
    #[inline(always)]
    fn from(val: Wupe28) -> u8 {
        Wupe28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe29 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe29 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe29 {
    #[inline(always)]
    fn from(val: u8) -> Wupe29 {
        Wupe29::from_bits(val)
    }
}
impl From<Wupe29> for u8 {
    #[inline(always)]
    fn from(val: Wupe29) -> u8 {
        Wupe29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe30 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe30 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe30 {
    #[inline(always)]
    fn from(val: u8) -> Wupe30 {
        Wupe30::from_bits(val)
    }
}
impl From<Wupe30> for u8 {
    #[inline(always)]
    fn from(val: Wupe30) -> u8 {
        Wupe30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe31 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe31 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe31 {
    #[inline(always)]
    fn from(val: u8) -> Wupe31 {
        Wupe31::from_bits(val)
    }
}
impl From<Wupe31> for u8 {
    #[inline(always)]
    fn from(val: Wupe31) -> u8 {
        Wupe31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe6 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe6 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe6 {
    #[inline(always)]
    fn from(val: u8) -> Wupe6 {
        Wupe6::from_bits(val)
    }
}
impl From<Wupe6> for u8 {
    #[inline(always)]
    fn from(val: Wupe6) -> u8 {
        Wupe6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe7 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe7 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe7 {
    #[inline(always)]
    fn from(val: u8) -> Wupe7 {
        Wupe7::from_bits(val)
    }
}
impl From<Wupe7> for u8 {
    #[inline(always)]
    fn from(val: Wupe7) -> u8 {
        Wupe7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe8 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe8 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe8 {
    #[inline(always)]
    fn from(val: u8) -> Wupe8 {
        Wupe8::from_bits(val)
    }
}
impl From<Wupe8> for u8 {
    #[inline(always)]
    fn from(val: Wupe8) -> u8 {
        Wupe8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupe9 {
    #[doc = "Disable."]
    Disable = 0x0,
    #[doc = "Enable (detect on rising edge or high level)."]
    EnRiseHi = 0x01,
    #[doc = "Enable (detect on falling edge or low level)."]
    EnFallLo = 0x02,
    #[doc = "Enable (detect on any edge)."]
    EnAny = 0x03,
}
impl Wupe9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupe9 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupe9 {
    #[inline(always)]
    fn from(val: u8) -> Wupe9 {
        Wupe9::from_bits(val)
    }
}
impl From<Wupe9> for u8 {
    #[inline(always)]
    fn from(val: Wupe9) -> u8 {
        Wupe9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc10 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc10 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc10 {
        Wupmc10::from_bits(val)
    }
}
impl From<Wupmc10> for u8 {
    #[inline(always)]
    fn from(val: Wupmc10) -> u8 {
        Wupmc10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc11 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc11 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc11 {
        Wupmc11::from_bits(val)
    }
}
impl From<Wupmc11> for u8 {
    #[inline(always)]
    fn from(val: Wupmc11) -> u8 {
        Wupmc11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc12 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc12 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc12 {
        Wupmc12::from_bits(val)
    }
}
impl From<Wupmc12> for u8 {
    #[inline(always)]
    fn from(val: Wupmc12) -> u8 {
        Wupmc12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc16 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc16 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc16 {
        Wupmc16::from_bits(val)
    }
}
impl From<Wupmc16> for u8 {
    #[inline(always)]
    fn from(val: Wupmc16) -> u8 {
        Wupmc16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc17 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc17 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc17 {
        Wupmc17::from_bits(val)
    }
}
impl From<Wupmc17> for u8 {
    #[inline(always)]
    fn from(val: Wupmc17) -> u8 {
        Wupmc17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc18 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc18 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc18 {
        Wupmc18::from_bits(val)
    }
}
impl From<Wupmc18> for u8 {
    #[inline(always)]
    fn from(val: Wupmc18) -> u8 {
        Wupmc18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc19 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc19 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc19 {
        Wupmc19::from_bits(val)
    }
}
impl From<Wupmc19> for u8 {
    #[inline(always)]
    fn from(val: Wupmc19) -> u8 {
        Wupmc19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc2 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc2 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc2 {
        Wupmc2::from_bits(val)
    }
}
impl From<Wupmc2> for u8 {
    #[inline(always)]
    fn from(val: Wupmc2) -> u8 {
        Wupmc2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc20 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc20 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc20 {
        Wupmc20::from_bits(val)
    }
}
impl From<Wupmc20> for u8 {
    #[inline(always)]
    fn from(val: Wupmc20) -> u8 {
        Wupmc20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc22 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc22 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc22 {
        Wupmc22::from_bits(val)
    }
}
impl From<Wupmc22> for u8 {
    #[inline(always)]
    fn from(val: Wupmc22) -> u8 {
        Wupmc22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc23 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc23 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc23 {
        Wupmc23::from_bits(val)
    }
}
impl From<Wupmc23> for u8 {
    #[inline(always)]
    fn from(val: Wupmc23) -> u8 {
        Wupmc23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc24 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc24 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc24 {
        Wupmc24::from_bits(val)
    }
}
impl From<Wupmc24> for u8 {
    #[inline(always)]
    fn from(val: Wupmc24) -> u8 {
        Wupmc24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc25 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc25 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc25 {
        Wupmc25::from_bits(val)
    }
}
impl From<Wupmc25> for u8 {
    #[inline(always)]
    fn from(val: Wupmc25) -> u8 {
        Wupmc25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc26 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc26 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc26 {
        Wupmc26::from_bits(val)
    }
}
impl From<Wupmc26> for u8 {
    #[inline(always)]
    fn from(val: Wupmc26) -> u8 {
        Wupmc26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc27 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc27 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc27 {
        Wupmc27::from_bits(val)
    }
}
impl From<Wupmc27> for u8 {
    #[inline(always)]
    fn from(val: Wupmc27) -> u8 {
        Wupmc27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc28 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc28 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc28 {
        Wupmc28::from_bits(val)
    }
}
impl From<Wupmc28> for u8 {
    #[inline(always)]
    fn from(val: Wupmc28) -> u8 {
        Wupmc28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc29 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc29 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc29 {
        Wupmc29::from_bits(val)
    }
}
impl From<Wupmc29> for u8 {
    #[inline(always)]
    fn from(val: Wupmc29) -> u8 {
        Wupmc29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc30 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc30 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc30 {
        Wupmc30::from_bits(val)
    }
}
impl From<Wupmc30> for u8 {
    #[inline(always)]
    fn from(val: Wupmc30) -> u8 {
        Wupmc30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc31 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc31 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc31 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc31 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc31 {
        Wupmc31::from_bits(val)
    }
}
impl From<Wupmc31> for u8 {
    #[inline(always)]
    fn from(val: Wupmc31) -> u8 {
        Wupmc31::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc6 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc6 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc6 {
        Wupmc6::from_bits(val)
    }
}
impl From<Wupmc6> for u8 {
    #[inline(always)]
    fn from(val: Wupmc6) -> u8 {
        Wupmc6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc7 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc7 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc7 {
        Wupmc7::from_bits(val)
    }
}
impl From<Wupmc7> for u8 {
    #[inline(always)]
    fn from(val: Wupmc7) -> u8 {
        Wupmc7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc8 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc8 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc8 {
        Wupmc8::from_bits(val)
    }
}
impl From<Wupmc8> for u8 {
    #[inline(always)]
    fn from(val: Wupmc8) -> u8 {
        Wupmc8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wupmc9 {
    #[doc = "Active only during a low-leakage mode. You can modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    LowPwrOnly = 0x0,
    #[doc = "Active during all power modes. Do not modify the corresponding fields within Pin Enable (PEn) or Pin DMA/Trigger Configuration (PDCn)."]
    AnyPwr = 0x01,
}
impl Wupmc9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wupmc9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wupmc9 {
    #[inline(always)]
    fn from(val: u8) -> Wupmc9 {
        Wupmc9::from_bits(val)
    }
}
impl From<Wupmc9> for u8 {
    #[inline(always)]
    fn from(val: Wupmc9) -> u8 {
        Wupmc9::to_bits(val)
    }
}
