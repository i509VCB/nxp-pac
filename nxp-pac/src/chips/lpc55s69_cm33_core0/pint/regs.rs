#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienf(pub u32);
impl Cienf {
    #[doc = "Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn cenaf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    pub const fn set_cenaf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Cienf {{ cenaf: {=u8:?} }}", self.cenaf())
    }
}
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cienr(pub u32);
impl Cienr {
    #[doc = "Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cenrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_cenrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Cienr {{ cenrl: {=u8:?} }}", self.cenrl())
    }
}
#[doc = "Pin interrupt falling edge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fall(pub u32);
impl Fall {
    #[doc = "Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[must_use]
    #[inline(always)]
    pub const fn fdet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub const fn set_fdet(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Fall {{ fdet: {=u8:?} }}", self.fdet())
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienf(pub u32);
impl Ienf {
    #[doc = "Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn enaf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub const fn set_enaf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Ienf {{ enaf: {=u8:?} }}", self.enaf())
    }
}
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ienr(pub u32);
impl Ienr {
    #[doc = "Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn enrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_enrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Ienr {{ enrl: {=u8:?} }}", self.enrl())
    }
}
#[doc = "Pin Interrupt Mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isel(pub u32);
impl Isel {
    #[doc = "Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[must_use]
    #[inline(always)]
    pub const fn pmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive"]
    #[inline(always)]
    pub const fn set_pmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Isel {{ pmode: {=u8:?} }}", self.pmode())
    }
}
#[doc = "Pin interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ist(pub u32);
impl Ist {
    #[doc = "Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[must_use]
    #[inline(always)]
    pub const fn pstat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub const fn set_pstat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Ist {{ pstat: {=u8:?} }}", self.pstat())
    }
}
#[doc = "Pattern match interrupt bit slice configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcfg(pub u32);
impl Pmcfg {
    #[doc = "Determines whether slice 0 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts0(&self) -> super::vals::ProdEndpts0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ProdEndpts0::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts0(&mut self, val: super::vals::ProdEndpts0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether slice 1 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts1(&self) -> super::vals::ProdEndpts1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ProdEndpts1::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts1(&mut self, val: super::vals::ProdEndpts1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether slice 2 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts2(&self) -> super::vals::ProdEndpts2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ProdEndpts2::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts2(&mut self, val: super::vals::ProdEndpts2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether slice 3 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts3(&self) -> super::vals::ProdEndpts3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ProdEndpts3::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts3(&mut self, val: super::vals::ProdEndpts3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Determines whether slice 4 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts4(&self) -> super::vals::ProdEndpts4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::ProdEndpts4::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts4(&mut self, val: super::vals::ProdEndpts4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Determines whether slice 5 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts5(&self) -> super::vals::ProdEndpts5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::ProdEndpts5::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts5(&mut self, val: super::vals::ProdEndpts5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Determines whether slice 6 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn prod_endpts6(&self) -> super::vals::ProdEndpts6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::ProdEndpts6::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub const fn set_prod_endpts6(&mut self, val: super::vals::ProdEndpts6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg0(&self) -> super::vals::Cfg0 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Cfg0::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub const fn set_cfg0(&mut self, val: super::vals::Cfg0) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg1(&self) -> super::vals::Cfg1 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::Cfg1::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub const fn set_cfg1(&mut self, val: super::vals::Cfg1) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg2(&self) -> super::vals::Cfg2 {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Cfg2::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub const fn set_cfg2(&mut self, val: super::vals::Cfg2) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 3."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg3(&self) -> super::vals::Cfg3 {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::Cfg3::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub const fn set_cfg3(&mut self, val: super::vals::Cfg3) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 4."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg4(&self) -> super::vals::Cfg4 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Cfg4::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub const fn set_cfg4(&mut self, val: super::vals::Cfg4) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 5."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg5(&self) -> super::vals::Cfg5 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Cfg5::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub const fn set_cfg5(&mut self, val: super::vals::Cfg5) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 6."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg6(&self) -> super::vals::Cfg6 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::Cfg6::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub const fn set_cfg6(&mut self, val: super::vals::Cfg6) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 7."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg7(&self) -> super::vals::Cfg7 {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::Cfg7::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 7."]
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
#[doc = "Pattern match interrupt control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmctrl(pub u32);
impl Pmctrl {
    #[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_pmatch(&self) -> super::vals::SelPmatch {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SelPmatch::from_bits(val as u8)
    }
    #[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub const fn set_sel_pmatch(&mut self, val: super::vals::SelPmatch) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[must_use]
    #[inline(always)]
    pub const fn ena_rxev(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub const fn set_ena_rxev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn pmat(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub const fn set_pmat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
            "Pmctrl {{ sel_pmatch: {:?}, ena_rxev: {=bool:?}, pmat: {=u8:?} }}",
            self.sel_pmatch(),
            self.ena_rxev(),
            self.pmat()
        )
    }
}
#[doc = "Pattern match interrupt bit-slice source register"]
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
#[doc = "Pin interrupt rising edge register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rise(pub u32);
impl Rise {
    #[doc = "Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[must_use]
    #[inline(always)]
    pub const fn rdet(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub const fn set_rdet(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Rise {{ rdet: {=u8:?} }}", self.rdet())
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienf(pub u32);
impl Sienf {
    #[doc = "Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn setenaf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    pub const fn set_setenaf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Sienf {{ setenaf: {=u8:?} }}", self.setenaf())
    }
}
#[doc = "Pin interrupt level or rising edge interrupt set register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sienr(pub u32);
impl Sienr {
    #[doc = "Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn setenrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_setenrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
        defmt::write!(f, "Sienr {{ setenrl: {=u8:?} }}", self.setenrl())
    }
}
