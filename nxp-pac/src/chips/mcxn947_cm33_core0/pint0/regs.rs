#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienf(pub u32);
impl Cienf {
    #[doc = "Writes 0 to IENF"]
    #[must_use]
    #[inline(always)]
    pub const fn cenaf(&self) -> super::vals::Cenaf {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Cenaf::from_bits(val as u8)
    }
    #[doc = "Writes 0 to IENF"]
    #[inline(always)]
    pub const fn set_cenaf(&mut self, val: super::vals::Cenaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cienf {
    #[inline(always)]
    fn default() -> Cienf {
        Cienf(0)
    }
}
impl core::fmt::Debug for Cienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cienf")
            .field("cenaf", &self.cenaf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cienf {{ cenaf: {:?} }}", self.cenaf())
    }
}
#[doc = "Pin Interrupt Level (Rising-Edge Interrupt) Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienr(pub u32);
impl Cienr {
    #[doc = "Clear bits in IENR"]
    #[must_use]
    #[inline(always)]
    pub const fn cenrl(&self) -> super::vals::Cenrl {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Cenrl::from_bits(val as u8)
    }
    #[doc = "Clear bits in IENR"]
    #[inline(always)]
    pub const fn set_cenrl(&mut self, val: super::vals::Cenrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Cienr {
    #[inline(always)]
    fn default() -> Cienr {
        Cienr(0)
    }
}
impl core::fmt::Debug for Cienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cienr")
            .field("cenrl", &self.cenrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cienr {{ cenrl: {:?} }}", self.cenrl())
    }
}
#[doc = "Pin Interrupt Falling Edge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fall(pub u32);
impl Fall {
    #[doc = "Falling-Edge Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn fdet(&self) -> super::vals::Fdet {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Fdet::from_bits(val as u8)
    }
    #[doc = "Falling-Edge Detect"]
    #[inline(always)]
    pub const fn set_fdet(&mut self, val: super::vals::Fdet) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Fall {
    #[inline(always)]
    fn default() -> Fall {
        Fall(0)
    }
}
impl core::fmt::Debug for Fall {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fall").field("fdet", &self.fdet()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fall {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fall {{ fdet: {:?} }}", self.fdet())
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienf(pub u32);
impl Ienf {
    #[doc = "Enables Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn enaf(&self) -> super::vals::Enaf {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Enaf::from_bits(val as u8)
    }
    #[doc = "Enables Interrupt"]
    #[inline(always)]
    pub const fn set_enaf(&mut self, val: super::vals::Enaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ienf {
    #[inline(always)]
    fn default() -> Ienf {
        Ienf(0)
    }
}
impl core::fmt::Debug for Ienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienf").field("enaf", &self.enaf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ienf {{ enaf: {:?} }}", self.enaf())
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienr(pub u32);
impl Ienr {
    #[doc = "Enables Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn enrl(&self) -> super::vals::Enrl {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Enrl::from_bits(val as u8)
    }
    #[doc = "Enables Interrupt"]
    #[inline(always)]
    pub const fn set_enrl(&mut self, val: super::vals::Enrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ienr {
    #[inline(always)]
    fn default() -> Ienr {
        Ienr(0)
    }
}
impl core::fmt::Debug for Ienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ienr").field("enrl", &self.enrl()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ienr {{ enrl: {:?} }}", self.enrl())
    }
}
#[doc = "Pin Interrupt Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isel(pub u32);
impl Isel {
    #[doc = "Interrupt mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pmode(&self) -> super::vals::Pmode {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Pmode::from_bits(val as u8)
    }
    #[doc = "Interrupt mode"]
    #[inline(always)]
    pub const fn set_pmode(&mut self, val: super::vals::Pmode) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Isel {
    #[inline(always)]
    fn default() -> Isel {
        Isel(0)
    }
}
impl core::fmt::Debug for Isel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isel")
            .field("pmode", &self.pmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Isel {{ pmode: {:?} }}", self.pmode())
    }
}
#[doc = "Pin Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ist(pub u32);
impl Ist {
    #[doc = "Pin Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn pstat(&self) -> super::vals::Pstat {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Pstat::from_bits(val as u8)
    }
    #[doc = "Pin Interrupt Status"]
    #[inline(always)]
    pub const fn set_pstat(&mut self, val: super::vals::Pstat) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Ist {
    #[inline(always)]
    fn default() -> Ist {
        Ist(0)
    }
}
impl core::fmt::Debug for Ist {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ist").field("pstat", &self.pstat()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ist {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ist {{ pstat: {:?} }}", self.pstat())
    }
}
#[doc = "Pattern-Match Interrupt Bit Slice Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcfg(pub u32);
impl Pmcfg {
    #[doc = "Determines whether slice 0 is an endpoint. Slice 0 is not an endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts0(&self) -> super::vals::ProdEndpts0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ProdEndpts0::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 0 is an endpoint. Slice 0 is not an endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts0(&mut self, val: super::vals::ProdEndpts0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether slice 1 is an endpoint. Slice 1 is not an endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts1(&self) -> super::vals::ProdEndpts1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ProdEndpts1::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 1 is an endpoint. Slice 1 is not an endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts1(&mut self, val: super::vals::ProdEndpts1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether slice 2 is an endpoint. Slice 2 is not an endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts2(&self) -> super::vals::ProdEndpts2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ProdEndpts2::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 2 is an endpoint. Slice 2 is not an endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts2(&mut self, val: super::vals::ProdEndpts2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether slice 3 is an endpoint. Slice 3 is not an endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts3(&self) -> super::vals::ProdEndpts3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ProdEndpts3::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 3 is an endpoint. Slice 3 is not an endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts3(&mut self, val: super::vals::ProdEndpts3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Determines whether slice 4 is an endpoint. Slice 4 is not an endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts4(&self) -> super::vals::ProdEndpts4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ProdEndpts4::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 4 is an endpoint. Slice 4 is not an endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts4(&mut self, val: super::vals::ProdEndpts4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Determines whether slice 5 is an endpoint. Slice 5 is not an endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts5(&self) -> super::vals::ProdEndpts5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ProdEndpts5::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 5 is an endpoint. Slice 5 is not an endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts5(&mut self, val: super::vals::ProdEndpts5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Determines whether slice 6 is an endpoint. Slice 6 is not an endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts6(&self) -> super::vals::ProdEndpts6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ProdEndpts6::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 6 is an endpoint. Slice 6 is not an endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub const fn set_prod_endpts6(&mut self, val: super::vals::ProdEndpts6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg0(&self) -> super::vals::Cfg0 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cfg0::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg0(&mut self, val: super::vals::Cfg0) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg1(&self) -> super::vals::Cfg1 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Cfg1::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg1(&mut self, val: super::vals::Cfg1) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg2(&self) -> super::vals::Cfg2 {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Cfg2::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg2(&mut self, val: super::vals::Cfg2) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg3(&self) -> super::vals::Cfg3 {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Cfg3::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg3(&mut self, val: super::vals::Cfg3) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg4(&self) -> super::vals::Cfg4 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Cfg4::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg4(&mut self, val: super::vals::Cfg4) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg5(&self) -> super::vals::Cfg5 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Cfg5::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg5(&mut self, val: super::vals::Cfg5) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg6(&self) -> super::vals::Cfg6 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Cfg6::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg6(&mut self, val: super::vals::Cfg6) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg7(&self) -> super::vals::Cfg7 {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::Cfg7::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_cfg7(&mut self, val: super::vals::Cfg7) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for Pmcfg {
    #[inline(always)]
    fn default() -> Pmcfg {
        Pmcfg(0)
    }
}
impl core::fmt::Debug for Pmcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcfg")
            .field("prod_endpts0", &self.prod_endpts0())
            .field("prod_endpts1", &self.prod_endpts1())
            .field("prod_endpts2", &self.prod_endpts2())
            .field("prod_endpts3", &self.prod_endpts3())
            .field("prod_endpts4", &self.prod_endpts4())
            .field("prod_endpts5", &self.prod_endpts5())
            .field("prod_endpts6", &self.prod_endpts6())
            .field("cfg0", &self.cfg0())
            .field("cfg1", &self.cfg1())
            .field("cfg2", &self.cfg2())
            .field("cfg3", &self.cfg3())
            .field("cfg4", &self.cfg4())
            .field("cfg5", &self.cfg5())
            .field("cfg6", &self.cfg6())
            .field("cfg7", &self.cfg7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmcfg {{ prod_endpts0: {:?}, prod_endpts1: {:?}, prod_endpts2: {:?}, prod_endpts3: {:?}, prod_endpts4: {:?}, prod_endpts5: {:?}, prod_endpts6: {:?}, cfg0: {:?}, cfg1: {:?}, cfg2: {:?}, cfg3: {:?}, cfg4: {:?}, cfg5: {:?}, cfg6: {:?}, cfg7: {:?} }}",
            self.prod_endpts0(),
            self.prod_endpts1(),
            self.prod_endpts2(),
            self.prod_endpts3(),
            self.prod_endpts4(),
            self.prod_endpts5(),
            self.prod_endpts6(),
            self.cfg0(),
            self.cfg1(),
            self.cfg2(),
            self.cfg3(),
            self.cfg4(),
            self.cfg5(),
            self.cfg6(),
            self.cfg7()
        )
    }
}
#[doc = "Pattern-Match Interrupt Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrl(pub u32);
impl Pmctrl {
    #[doc = "Specifies whether the pin interrupts are controlled by the pin interrupt function or by the pattern-match function. If this value is 0b, interrupts are driven in response to the standard pin interrupt function. If this value is 1b, interrupts are driven in response to pattern matches."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_pmatch(&self) -> super::vals::SelPmatch {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SelPmatch::from_bits(val as u8)
    }
    #[doc = "Specifies whether the pin interrupts are controlled by the pin interrupt function or by the pattern-match function. If this value is 0b, interrupts are driven in response to the standard pin interrupt function. If this value is 1b, interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub const fn set_sel_pmatch(&mut self, val: super::vals::SelPmatch) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output, when the specified Boolean expression evaluates to true. If this value is 0b, RXEV output to the CPU is disabled. If this value is 1b, RXEV output to the CPU is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_rxev(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output, when the specified Boolean expression evaluates to true. If this value is 0b, RXEV output to the CPU is disabled. If this value is 1b, RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub const fn set_ena_rxev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pattern Matches"]
    #[must_use]
    #[inline(always)]
    pub const fn pmat(&self) -> super::vals::Pmat {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Pmat::from_bits(val as u8)
    }
    #[doc = "Pattern Matches"]
    #[inline(always)]
    pub const fn set_pmat(&mut self, val: super::vals::Pmat) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Pmctrl {
    #[inline(always)]
    fn default() -> Pmctrl {
        Pmctrl(0)
    }
}
impl core::fmt::Debug for Pmctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmctrl")
            .field("sel_pmatch", &self.sel_pmatch())
            .field("ena_rxev", &self.ena_rxev())
            .field("pmat", &self.pmat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmctrl {{ sel_pmatch: {:?}, ena_rxev: {=bool:?}, pmat: {:?} }}",
            self.sel_pmatch(),
            self.ena_rxev(),
            self.pmat()
        )
    }
}
#[doc = "Pattern-Match Interrupt Bit-Slice Source"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmsrc(pub u32);
impl Pmsrc {
    #[doc = "Selects the input source for bit slice 0"]
    #[must_use]
    #[inline(always)]
    pub const fn src0(&self) -> super::vals::Src0 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Src0::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 0"]
    #[inline(always)]
    pub const fn set_src0(&mut self, val: super::vals::Src0) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Selects the input source for bit slice 1"]
    #[must_use]
    #[inline(always)]
    pub const fn src1(&self) -> super::vals::Src1 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Src1::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 1"]
    #[inline(always)]
    pub const fn set_src1(&mut self, val: super::vals::Src1) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Selects the input source for bit slice 2"]
    #[must_use]
    #[inline(always)]
    pub const fn src2(&self) -> super::vals::Src2 {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Src2::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 2"]
    #[inline(always)]
    pub const fn set_src2(&mut self, val: super::vals::Src2) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Selects the input source for bit slice 3"]
    #[must_use]
    #[inline(always)]
    pub const fn src3(&self) -> super::vals::Src3 {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Src3::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 3"]
    #[inline(always)]
    pub const fn set_src3(&mut self, val: super::vals::Src3) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Selects the input source for bit slice 4"]
    #[must_use]
    #[inline(always)]
    pub const fn src4(&self) -> super::vals::Src4 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Src4::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 4"]
    #[inline(always)]
    pub const fn set_src4(&mut self, val: super::vals::Src4) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Selects the input source for bit slice 5"]
    #[must_use]
    #[inline(always)]
    pub const fn src5(&self) -> super::vals::Src5 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Src5::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 5"]
    #[inline(always)]
    pub const fn set_src5(&mut self, val: super::vals::Src5) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Selects the input source for bit slice 6"]
    #[must_use]
    #[inline(always)]
    pub const fn src6(&self) -> super::vals::Src6 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Src6::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 6"]
    #[inline(always)]
    pub const fn set_src6(&mut self, val: super::vals::Src6) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Selects the input source for bit slice 7"]
    #[must_use]
    #[inline(always)]
    pub const fn src7(&self) -> super::vals::Src7 {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::Src7::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 7"]
    #[inline(always)]
    pub const fn set_src7(&mut self, val: super::vals::Src7) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for Pmsrc {
    #[inline(always)]
    fn default() -> Pmsrc {
        Pmsrc(0)
    }
}
impl core::fmt::Debug for Pmsrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmsrc")
            .field("src0", &self.src0())
            .field("src1", &self.src1())
            .field("src2", &self.src2())
            .field("src3", &self.src3())
            .field("src4", &self.src4())
            .field("src5", &self.src5())
            .field("src6", &self.src6())
            .field("src7", &self.src7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmsrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmsrc {{ src0: {:?}, src1: {:?}, src2: {:?}, src3: {:?}, src4: {:?}, src5: {:?}, src6: {:?}, src7: {:?} }}",
            self.src0(),
            self.src1(),
            self.src2(),
            self.src3(),
            self.src4(),
            self.src5(),
            self.src6(),
            self.src7()
        )
    }
}
#[doc = "Pin Interrupt Rising Edge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rise(pub u32);
impl Rise {
    #[doc = "Rising-Edge Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn rdet(&self) -> super::vals::Rdet {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Rdet::from_bits(val as u8)
    }
    #[doc = "Rising-Edge Detect"]
    #[inline(always)]
    pub const fn set_rdet(&mut self, val: super::vals::Rdet) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Rise {
    #[inline(always)]
    fn default() -> Rise {
        Rise(0)
    }
}
impl core::fmt::Debug for Rise {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rise").field("rdet", &self.rdet()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rise {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rise {{ rdet: {:?} }}", self.rdet())
    }
}
#[doc = "Pin Interrupt Active Level or Falling-Edge Interrupt Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienf(pub u32);
impl Sienf {
    #[doc = "Write 1 to this address to clear to disable interrupts. Bit a sets bit n in IENF."]
    #[must_use]
    #[inline(always)]
    pub const fn setenaf(&self) -> super::vals::Setenaf {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Setenaf::from_bits(val as u8)
    }
    #[doc = "Write 1 to this address to clear to disable interrupts. Bit a sets bit n in IENF."]
    #[inline(always)]
    pub const fn set_setenaf(&mut self, val: super::vals::Setenaf) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Sienf {
    #[inline(always)]
    fn default() -> Sienf {
        Sienf(0)
    }
}
impl core::fmt::Debug for Sienf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sienf")
            .field("setenaf", &self.setenaf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sienf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sienf {{ setenaf: {:?} }}", self.setenaf())
    }
}
#[doc = "Pin Interrupt Level or Rising-Edge Interrupt Set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienr(pub u32);
impl Sienr {
    #[doc = "Configures IENR"]
    #[must_use]
    #[inline(always)]
    pub const fn setenrl(&self) -> super::vals::Setenrl {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Setenrl::from_bits(val as u8)
    }
    #[doc = "Configures IENR"]
    #[inline(always)]
    pub const fn set_setenrl(&mut self, val: super::vals::Setenrl) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Sienr {
    #[inline(always)]
    fn default() -> Sienr {
        Sienr(0)
    }
}
impl core::fmt::Debug for Sienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sienr")
            .field("setenrl", &self.setenrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sienr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sienr {{ setenrl: {:?} }}", self.setenrl())
    }
}
