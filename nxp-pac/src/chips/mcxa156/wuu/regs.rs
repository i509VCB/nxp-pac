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
    pub const fn filtc1(&self) -> super::vals::Filtc1 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Filtc1::from_bits(val as u8)
    }
    #[doc = "Filter Configuration for FILTn."]
    #[inline(always)]
    pub const fn set_filtc1(&mut self, val: super::vals::Filtc1) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Filter Configuration for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtc2(&self) -> super::vals::Filtc2 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Filtc2::from_bits(val as u8)
    }
    #[doc = "Filter Configuration for FILTn."]
    #[inline(always)]
    pub const fn set_filtc2(&mut self, val: super::vals::Filtc2) {
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
    pub const fn filte1(&self) -> super::vals::Filte1 {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::Filte1::from_bits(val as u8)
    }
    #[doc = "Filter 1 Enable."]
    #[inline(always)]
    pub const fn set_filte1(&mut self, val: super::vals::Filte1) {
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
    pub const fn filte2(&self) -> super::vals::Filte2 {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Filte2::from_bits(val as u8)
    }
    #[doc = "Filter 2 Enable."]
    #[inline(always)]
    pub const fn set_filte2(&mut self, val: super::vals::Filte2) {
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
    pub const fn filtm1(&self) -> super::vals::Filtm1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Filtm1::from_bits(val as u8)
    }
    #[doc = "Filter Mode for FILTn."]
    #[inline(always)]
    pub const fn set_filtm1(&mut self, val: super::vals::Filtm1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Filter Mode for FILTn."]
    #[must_use]
    #[inline(always)]
    pub const fn filtm2(&self) -> super::vals::Filtm2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Filtm2::from_bits(val as u8)
    }
    #[doc = "Filter Mode for FILTn."]
    #[inline(always)]
    pub const fn set_filtm2(&mut self, val: super::vals::Filtm2) {
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
    pub const fn reserved0(&self) -> super::vals::Pdc1Reserved0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pdc1Reserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: super::vals::Pdc1Reserved0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> super::vals::Pdc1Reserved1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pdc1Reserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: super::vals::Pdc1Reserved1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc2(&self) -> super::vals::Wupdc2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Wupdc2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc2(&mut self, val: super::vals::Wupdc2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> super::vals::Pdc1Reserved3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pdc1Reserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: super::vals::Pdc1Reserved3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> super::vals::Pdc1Reserved4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pdc1Reserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: super::vals::Pdc1Reserved4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> super::vals::Pdc1Reserved5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pdc1Reserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: super::vals::Pdc1Reserved5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc6(&self) -> super::vals::Wupdc6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Wupdc6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc6(&mut self, val: super::vals::Wupdc6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc7(&self) -> super::vals::Wupdc7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Wupdc7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc7(&mut self, val: super::vals::Wupdc7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc8(&self) -> super::vals::Wupdc8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Wupdc8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc8(&mut self, val: super::vals::Wupdc8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc9(&self) -> super::vals::Wupdc9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Wupdc9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc9(&mut self, val: super::vals::Wupdc9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc10(&self) -> super::vals::Wupdc10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Wupdc10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc10(&mut self, val: super::vals::Wupdc10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc11(&self) -> super::vals::Wupdc11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Wupdc11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc11(&mut self, val: super::vals::Wupdc11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc12(&self) -> super::vals::Wupdc12 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wupdc12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc12(&mut self, val: super::vals::Wupdc12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> super::vals::Pdc1Reserved13 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Pdc1Reserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: super::vals::Pdc1Reserved13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> super::vals::Pdc1Reserved14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Pdc1Reserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: super::vals::Pdc1Reserved14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> super::vals::Pdc1Reserved15 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Pdc1Reserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: super::vals::Pdc1Reserved15) {
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
    pub const fn wupdc16(&self) -> super::vals::Wupdc16 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Wupdc16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc16(&mut self, val: super::vals::Wupdc16) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc17(&self) -> super::vals::Wupdc17 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wupdc17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc17(&mut self, val: super::vals::Wupdc17) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc18(&self) -> super::vals::Wupdc18 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Wupdc18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc18(&mut self, val: super::vals::Wupdc18) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc19(&self) -> super::vals::Wupdc19 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Wupdc19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc19(&mut self, val: super::vals::Wupdc19) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc20(&self) -> super::vals::Wupdc20 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Wupdc20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc20(&mut self, val: super::vals::Wupdc20) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> super::vals::Pdc2Reserved21 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pdc2Reserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: super::vals::Pdc2Reserved21) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc22(&self) -> super::vals::Wupdc22 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Wupdc22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc22(&mut self, val: super::vals::Wupdc22) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc23(&self) -> super::vals::Wupdc23 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Wupdc23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc23(&mut self, val: super::vals::Wupdc23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc24(&self) -> super::vals::Wupdc24 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Wupdc24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc24(&mut self, val: super::vals::Wupdc24) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc25(&self) -> super::vals::Wupdc25 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Wupdc25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc25(&mut self, val: super::vals::Wupdc25) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc26(&self) -> super::vals::Wupdc26 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Wupdc26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc26(&mut self, val: super::vals::Wupdc26) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc27(&self) -> super::vals::Wupdc27 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Wupdc27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc27(&mut self, val: super::vals::Wupdc27) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc28(&self) -> super::vals::Wupdc28 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wupdc28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc28(&mut self, val: super::vals::Wupdc28) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc29(&self) -> super::vals::Wupdc29 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Wupdc29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc29(&mut self, val: super::vals::Wupdc29) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc30(&self) -> super::vals::Wupdc30 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Wupdc30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc30(&mut self, val: super::vals::Wupdc30) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupdc31(&self) -> super::vals::Wupdc31 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Wupdc31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupdc31(&mut self, val: super::vals::Wupdc31) {
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
    pub const fn reserved0(&self) -> super::vals::Pe1Reserved0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pe1Reserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: super::vals::Pe1Reserved0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> super::vals::Pe1Reserved1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Pe1Reserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: super::vals::Pe1Reserved1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe2(&self) -> super::vals::Wupe2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Wupe2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe2(&mut self, val: super::vals::Wupe2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> super::vals::Pe1Reserved3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Pe1Reserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: super::vals::Pe1Reserved3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> super::vals::Pe1Reserved4 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Pe1Reserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: super::vals::Pe1Reserved4) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> super::vals::Pe1Reserved5 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pe1Reserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: super::vals::Pe1Reserved5) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe6(&self) -> super::vals::Wupe6 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Wupe6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe6(&mut self, val: super::vals::Wupe6) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe7(&self) -> super::vals::Wupe7 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Wupe7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe7(&mut self, val: super::vals::Wupe7) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe8(&self) -> super::vals::Wupe8 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Wupe8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe8(&mut self, val: super::vals::Wupe8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe9(&self) -> super::vals::Wupe9 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Wupe9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe9(&mut self, val: super::vals::Wupe9) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe10(&self) -> super::vals::Wupe10 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Wupe10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe10(&mut self, val: super::vals::Wupe10) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe11(&self) -> super::vals::Wupe11 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Wupe11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe11(&mut self, val: super::vals::Wupe11) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe12(&self) -> super::vals::Wupe12 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wupe12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe12(&mut self, val: super::vals::Wupe12) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> super::vals::Pe1Reserved13 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Pe1Reserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: super::vals::Pe1Reserved13) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> super::vals::Pe1Reserved14 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Pe1Reserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: super::vals::Pe1Reserved14) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> super::vals::Pe1Reserved15 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Pe1Reserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: super::vals::Pe1Reserved15) {
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
    pub const fn wupe16(&self) -> super::vals::Wupe16 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Wupe16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe16(&mut self, val: super::vals::Wupe16) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe17(&self) -> super::vals::Wupe17 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wupe17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe17(&mut self, val: super::vals::Wupe17) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe18(&self) -> super::vals::Wupe18 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Wupe18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe18(&mut self, val: super::vals::Wupe18) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe19(&self) -> super::vals::Wupe19 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Wupe19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe19(&mut self, val: super::vals::Wupe19) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe20(&self) -> super::vals::Wupe20 {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Wupe20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe20(&mut self, val: super::vals::Wupe20) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> super::vals::Pe2Reserved21 {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Pe2Reserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: super::vals::Pe2Reserved21) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe22(&self) -> super::vals::Wupe22 {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Wupe22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe22(&mut self, val: super::vals::Wupe22) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe23(&self) -> super::vals::Wupe23 {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Wupe23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe23(&mut self, val: super::vals::Wupe23) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe24(&self) -> super::vals::Wupe24 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Wupe24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe24(&mut self, val: super::vals::Wupe24) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe25(&self) -> super::vals::Wupe25 {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Wupe25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe25(&mut self, val: super::vals::Wupe25) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe26(&self) -> super::vals::Wupe26 {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Wupe26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe26(&mut self, val: super::vals::Wupe26) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe27(&self) -> super::vals::Wupe27 {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Wupe27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe27(&mut self, val: super::vals::Wupe27) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe28(&self) -> super::vals::Wupe28 {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Wupe28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe28(&mut self, val: super::vals::Wupe28) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe29(&self) -> super::vals::Wupe29 {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Wupe29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe29(&mut self, val: super::vals::Wupe29) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe30(&self) -> super::vals::Wupe30 {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Wupe30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe30(&mut self, val: super::vals::Wupe30) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupe31(&self) -> super::vals::Wupe31 {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Wupe31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Enable for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupe31(&mut self, val: super::vals::Wupe31) {
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
    pub const fn reserved0(&self) -> super::vals::PmcReserved0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PmcReserved0::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: super::vals::PmcReserved0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> super::vals::PmcReserved1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PmcReserved1::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: super::vals::PmcReserved1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc2(&self) -> super::vals::Wupmc2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Wupmc2::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc2(&mut self, val: super::vals::Wupmc2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved3(&self) -> super::vals::PmcReserved3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PmcReserved3::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved3(&mut self, val: super::vals::PmcReserved3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved4(&self) -> super::vals::PmcReserved4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PmcReserved4::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved4(&mut self, val: super::vals::PmcReserved4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved5(&self) -> super::vals::PmcReserved5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PmcReserved5::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved5(&mut self, val: super::vals::PmcReserved5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc6(&self) -> super::vals::Wupmc6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wupmc6::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc6(&mut self, val: super::vals::Wupmc6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc7(&self) -> super::vals::Wupmc7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wupmc7::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc7(&mut self, val: super::vals::Wupmc7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc8(&self) -> super::vals::Wupmc8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wupmc8::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc8(&mut self, val: super::vals::Wupmc8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc9(&self) -> super::vals::Wupmc9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Wupmc9::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc9(&mut self, val: super::vals::Wupmc9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc10(&self) -> super::vals::Wupmc10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wupmc10::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc10(&mut self, val: super::vals::Wupmc10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc11(&self) -> super::vals::Wupmc11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Wupmc11::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc11(&mut self, val: super::vals::Wupmc11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc12(&self) -> super::vals::Wupmc12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Wupmc12::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc12(&mut self, val: super::vals::Wupmc12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved13(&self) -> super::vals::PmcReserved13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::PmcReserved13::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved13(&mut self, val: super::vals::PmcReserved13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved14(&self) -> super::vals::PmcReserved14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::PmcReserved14::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved14(&mut self, val: super::vals::PmcReserved14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved15(&self) -> super::vals::PmcReserved15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::PmcReserved15::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved15(&mut self, val: super::vals::PmcReserved15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc16(&self) -> super::vals::Wupmc16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Wupmc16::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc16(&mut self, val: super::vals::Wupmc16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc17(&self) -> super::vals::Wupmc17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Wupmc17::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc17(&mut self, val: super::vals::Wupmc17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc18(&self) -> super::vals::Wupmc18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Wupmc18::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc18(&mut self, val: super::vals::Wupmc18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc19(&self) -> super::vals::Wupmc19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Wupmc19::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc19(&mut self, val: super::vals::Wupmc19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc20(&self) -> super::vals::Wupmc20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Wupmc20::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc20(&mut self, val: super::vals::Wupmc20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Reserved."]
    #[must_use]
    #[inline(always)]
    pub const fn reserved21(&self) -> super::vals::PmcReserved21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::PmcReserved21::from_bits(val as u8)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub const fn set_reserved21(&mut self, val: super::vals::PmcReserved21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc22(&self) -> super::vals::Wupmc22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Wupmc22::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc22(&mut self, val: super::vals::Wupmc22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc23(&self) -> super::vals::Wupmc23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Wupmc23::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc23(&mut self, val: super::vals::Wupmc23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc24(&self) -> super::vals::Wupmc24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Wupmc24::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc24(&mut self, val: super::vals::Wupmc24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc25(&self) -> super::vals::Wupmc25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Wupmc25::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc25(&mut self, val: super::vals::Wupmc25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc26(&self) -> super::vals::Wupmc26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Wupmc26::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc26(&mut self, val: super::vals::Wupmc26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc27(&self) -> super::vals::Wupmc27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Wupmc27::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc27(&mut self, val: super::vals::Wupmc27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc28(&self) -> super::vals::Wupmc28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Wupmc28::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc28(&mut self, val: super::vals::Wupmc28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc29(&self) -> super::vals::Wupmc29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Wupmc29::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc29(&mut self, val: super::vals::Wupmc29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc30(&self) -> super::vals::Wupmc30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Wupmc30::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc30(&mut self, val: super::vals::Wupmc30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[must_use]
    #[inline(always)]
    pub const fn wupmc31(&self) -> super::vals::Wupmc31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Wupmc31::from_bits(val as u8)
    }
    #[doc = "Wake-up Pin Mode Configuration for WUU_Pn."]
    #[inline(always)]
    pub const fn set_wupmc31(&mut self, val: super::vals::Wupmc31) {
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
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
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
