#[doc = "Pin interrupt active level or falling edge interrupt clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CIENF(pub u32);
impl CIENF {
    #[doc = "Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn CENAF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address clears bits in the IENF, thus disabling interrupts. Bit n clears bit n in the IENF register. 0 = No operation. 1 = LOW-active interrupt selected or falling edge interrupt disabled."]
    #[inline(always)]
    pub const fn set_CENAF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CIENF {
    #[inline(always)]
    fn default() -> CIENF {
        CIENF(0)
    }
}
impl core::fmt::Debug for CIENF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIENF")
            .field("CENAF", &self.CENAF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CIENF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CIENF {{ CENAF: {=u8:?} }}", self.CENAF())
    }
}
#[doc = "Pin interrupt level (rising edge interrupt) clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CIENR(pub u32);
impl CIENR {
    #[doc = "Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn CENRL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address clear bits in the IENR, thus disabling the interrupts. Bit n clears bit n in the IENR register. 0 = No operation. 1 = Disable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_CENRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CIENR {
    #[inline(always)]
    fn default() -> CIENR {
        CIENR(0)
    }
}
impl core::fmt::Debug for CIENR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIENR")
            .field("CENRL", &self.CENRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CIENR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CIENR {{ CENRL: {=u8:?} }}", self.CENRL())
    }
}
#[doc = "Pin interrupt falling edge register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FALL(pub u32);
impl FALL {
    #[doc = "Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[must_use]
    #[inline(always)]
    pub const fn FDET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Falling edge detect. Bit n detects the falling edge of the pin selected in PINTSELn. Read 0: No falling edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a falling edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear falling edge detection for this pin."]
    #[inline(always)]
    pub const fn set_FDET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FALL {
    #[inline(always)]
    fn default() -> FALL {
        FALL(0)
    }
}
impl core::fmt::Debug for FALL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FALL").field("FDET", &self.FDET()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FALL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FALL {{ FDET: {=u8:?} }}", self.FDET())
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IENF(pub u32);
impl IENF {
    #[doc = "Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[must_use]
    #[inline(always)]
    pub const fn ENAF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enables the falling edge or configures the active level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable falling edge interrupt or set active interrupt level LOW. 1 = Enable falling edge interrupt enabled or set active interrupt level HIGH."]
    #[inline(always)]
    pub const fn set_ENAF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IENF {
    #[inline(always)]
    fn default() -> IENF {
        IENF(0)
    }
}
impl core::fmt::Debug for IENF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IENF").field("ENAF", &self.ENAF()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IENF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IENF {{ ENAF: {=u8:?} }}", self.ENAF())
    }
}
#[doc = "Pin interrupt level or rising edge interrupt enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IENR(pub u32);
impl IENR {
    #[doc = "Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ENRL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Enables the rising edge or level interrupt for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Disable rising edge or level interrupt. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_ENRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IENR {
    #[inline(always)]
    fn default() -> IENR {
        IENR(0)
    }
}
impl core::fmt::Debug for IENR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IENR").field("ENRL", &self.ENRL()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IENR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IENR {{ ENRL: {=u8:?} }}", self.ENRL())
    }
}
#[doc = "Pin Interrupt Mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ISEL(pub u32);
impl ISEL {
    #[doc = "Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive."]
    #[must_use]
    #[inline(always)]
    pub const fn PMODE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the interrupt mode for each pin interrupt. Bit n configures the pin interrupt selected in PINTSELn. 0 = Edge sensitive 1 = Level sensitive."]
    #[inline(always)]
    pub const fn set_PMODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ISEL {
    #[inline(always)]
    fn default() -> ISEL {
        ISEL(0)
    }
}
impl core::fmt::Debug for ISEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISEL")
            .field("PMODE", &self.PMODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ISEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ISEL {{ PMODE: {=u8:?} }}", self.PMODE())
    }
}
#[doc = "Pin interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IST(pub u32);
impl IST {
    #[doc = "Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[must_use]
    #[inline(always)]
    pub const fn PSTAT(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Pin interrupt status. Bit n returns the status, clears the edge interrupt, or inverts the active level of the pin selected in PINTSELn. Read 0: interrupt is not being requested for this interrupt pin. Write 0: no operation. Read 1: interrupt is being requested for this interrupt pin. Write 1 (edge-sensitive): clear rising- and falling-edge detection for this pin. Write 1 (level-sensitive): switch the active level for this pin (in the IENF register)."]
    #[inline(always)]
    pub const fn set_PSTAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IST {
    #[inline(always)]
    fn default() -> IST {
        IST(0)
    }
}
impl core::fmt::Debug for IST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IST").field("PSTAT", &self.PSTAT()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IST {{ PSTAT: {=u8:?} }}", self.PSTAT())
    }
}
#[doc = "Pattern match interrupt bit slice configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMCFG(pub u32);
impl PMCFG {
    #[doc = "Determines whether slice 0 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS0(&self) -> super::vals::PROD_ENDPTS0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PROD_ENDPTS0::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS0(&mut self, val: super::vals::PROD_ENDPTS0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether slice 1 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS1(&self) -> super::vals::PROD_ENDPTS1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PROD_ENDPTS1::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS1(&mut self, val: super::vals::PROD_ENDPTS1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether slice 2 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS2(&self) -> super::vals::PROD_ENDPTS2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PROD_ENDPTS2::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS2(&mut self, val: super::vals::PROD_ENDPTS2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether slice 3 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS3(&self) -> super::vals::PROD_ENDPTS3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PROD_ENDPTS3::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS3(&mut self, val: super::vals::PROD_ENDPTS3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Determines whether slice 4 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS4(&self) -> super::vals::PROD_ENDPTS4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PROD_ENDPTS4::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS4(&mut self, val: super::vals::PROD_ENDPTS4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Determines whether slice 5 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS5(&self) -> super::vals::PROD_ENDPTS5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PROD_ENDPTS5::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS5(&mut self, val: super::vals::PROD_ENDPTS5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Determines whether slice 6 is an endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn PROD_ENDPTS6(&self) -> super::vals::PROD_ENDPTS6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PROD_ENDPTS6::from_bits(val as u8)
    }
    #[doc = "Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub const fn set_PROD_ENDPTS6(&mut self, val: super::vals::PROD_ENDPTS6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 0."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG0(&self) -> super::vals::CFG0 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::CFG0::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub const fn set_CFG0(&mut self, val: super::vals::CFG0) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 1."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG1(&self) -> super::vals::CFG1 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::CFG1::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub const fn set_CFG1(&mut self, val: super::vals::CFG1) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 2."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG2(&self) -> super::vals::CFG2 {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::CFG2::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub const fn set_CFG2(&mut self, val: super::vals::CFG2) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 3."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG3(&self) -> super::vals::CFG3 {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::CFG3::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub const fn set_CFG3(&mut self, val: super::vals::CFG3) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG4(&self) -> super::vals::CFG4 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::CFG4::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub const fn set_CFG4(&mut self, val: super::vals::CFG4) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 5."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG5(&self) -> super::vals::CFG5 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::CFG5::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub const fn set_CFG5(&mut self, val: super::vals::CFG5) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 6."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG6(&self) -> super::vals::CFG6 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::CFG6::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub const fn set_CFG6(&mut self, val: super::vals::CFG6) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Specifies the match contribution condition for bit slice 7."]
    #[must_use]
    #[inline(always)]
    pub const fn CFG7(&self) -> super::vals::CFG7 {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::CFG7::from_bits(val as u8)
    }
    #[doc = "Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub const fn set_CFG7(&mut self, val: super::vals::CFG7) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for PMCFG {
    #[inline(always)]
    fn default() -> PMCFG {
        PMCFG(0)
    }
}
impl core::fmt::Debug for PMCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCFG")
            .field("PROD_ENDPTS0", &self.PROD_ENDPTS0())
            .field("PROD_ENDPTS1", &self.PROD_ENDPTS1())
            .field("PROD_ENDPTS2", &self.PROD_ENDPTS2())
            .field("PROD_ENDPTS3", &self.PROD_ENDPTS3())
            .field("PROD_ENDPTS4", &self.PROD_ENDPTS4())
            .field("PROD_ENDPTS5", &self.PROD_ENDPTS5())
            .field("PROD_ENDPTS6", &self.PROD_ENDPTS6())
            .field("CFG0", &self.CFG0())
            .field("CFG1", &self.CFG1())
            .field("CFG2", &self.CFG2())
            .field("CFG3", &self.CFG3())
            .field("CFG4", &self.CFG4())
            .field("CFG5", &self.CFG5())
            .field("CFG6", &self.CFG6())
            .field("CFG7", &self.CFG7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMCFG {{ PROD_ENDPTS0: {:?}, PROD_ENDPTS1: {:?}, PROD_ENDPTS2: {:?}, PROD_ENDPTS3: {:?}, PROD_ENDPTS4: {:?}, PROD_ENDPTS5: {:?}, PROD_ENDPTS6: {:?}, CFG0: {:?}, CFG1: {:?}, CFG2: {:?}, CFG3: {:?}, CFG4: {:?}, CFG5: {:?}, CFG6: {:?}, CFG7: {:?} }}",
            self.PROD_ENDPTS0(),
            self.PROD_ENDPTS1(),
            self.PROD_ENDPTS2(),
            self.PROD_ENDPTS3(),
            self.PROD_ENDPTS4(),
            self.PROD_ENDPTS5(),
            self.PROD_ENDPTS6(),
            self.CFG0(),
            self.CFG1(),
            self.CFG2(),
            self.CFG3(),
            self.CFG4(),
            self.CFG5(),
            self.CFG6(),
            self.CFG7()
        )
    }
}
#[doc = "Pattern match interrupt control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMCTRL(pub u32);
impl PMCTRL {
    #[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[must_use]
    #[inline(always)]
    pub const fn SEL_PMATCH(&self) -> super::vals::SEL_PMATCH {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SEL_PMATCH::from_bits(val as u8)
    }
    #[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub const fn set_SEL_PMATCH(&mut self, val: super::vals::SEL_PMATCH) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[must_use]
    #[inline(always)]
    pub const fn ENA_RXEV(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub const fn set_ENA_RXEV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[must_use]
    #[inline(always)]
    pub const fn PMAT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub const fn set_PMAT(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PMCTRL {
    #[inline(always)]
    fn default() -> PMCTRL {
        PMCTRL(0)
    }
}
impl core::fmt::Debug for PMCTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMCTRL")
            .field("SEL_PMATCH", &self.SEL_PMATCH())
            .field("ENA_RXEV", &self.ENA_RXEV())
            .field("PMAT", &self.PMAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMCTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMCTRL {{ SEL_PMATCH: {:?}, ENA_RXEV: {=bool:?}, PMAT: {=u8:?} }}",
            self.SEL_PMATCH(),
            self.ENA_RXEV(),
            self.PMAT()
        )
    }
}
#[doc = "Pattern match interrupt bit-slice source register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PMSRC(pub u32);
impl PMSRC {
    #[doc = "Selects the input source for bit slice 0."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC0(&self) -> super::vals::SRC0 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::SRC0::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 0."]
    #[inline(always)]
    pub const fn set_SRC0(&mut self, val: super::vals::SRC0) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Selects the input source for bit slice 1."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC1(&self) -> super::vals::SRC1 {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::SRC1::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 1."]
    #[inline(always)]
    pub const fn set_SRC1(&mut self, val: super::vals::SRC1) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Selects the input source for bit slice 2."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC2(&self) -> super::vals::SRC2 {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::SRC2::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 2."]
    #[inline(always)]
    pub const fn set_SRC2(&mut self, val: super::vals::SRC2) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Selects the input source for bit slice 3."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC3(&self) -> super::vals::SRC3 {
        let val = (self.0 >> 17usize) & 0x07;
        super::vals::SRC3::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 3."]
    #[inline(always)]
    pub const fn set_SRC3(&mut self, val: super::vals::SRC3) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
    }
    #[doc = "Selects the input source for bit slice 4."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC4(&self) -> super::vals::SRC4 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::SRC4::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 4."]
    #[inline(always)]
    pub const fn set_SRC4(&mut self, val: super::vals::SRC4) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Selects the input source for bit slice 5."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC5(&self) -> super::vals::SRC5 {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::SRC5::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 5."]
    #[inline(always)]
    pub const fn set_SRC5(&mut self, val: super::vals::SRC5) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "Selects the input source for bit slice 6."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC6(&self) -> super::vals::SRC6 {
        let val = (self.0 >> 26usize) & 0x07;
        super::vals::SRC6::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 6."]
    #[inline(always)]
    pub const fn set_SRC6(&mut self, val: super::vals::SRC6) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val.to_bits() as u32) & 0x07) << 26usize);
    }
    #[doc = "Selects the input source for bit slice 7."]
    #[must_use]
    #[inline(always)]
    pub const fn SRC7(&self) -> super::vals::SRC7 {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::SRC7::from_bits(val as u8)
    }
    #[doc = "Selects the input source for bit slice 7."]
    #[inline(always)]
    pub const fn set_SRC7(&mut self, val: super::vals::SRC7) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for PMSRC {
    #[inline(always)]
    fn default() -> PMSRC {
        PMSRC(0)
    }
}
impl core::fmt::Debug for PMSRC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMSRC")
            .field("SRC0", &self.SRC0())
            .field("SRC1", &self.SRC1())
            .field("SRC2", &self.SRC2())
            .field("SRC3", &self.SRC3())
            .field("SRC4", &self.SRC4())
            .field("SRC5", &self.SRC5())
            .field("SRC6", &self.SRC6())
            .field("SRC7", &self.SRC7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PMSRC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PMSRC {{ SRC0: {:?}, SRC1: {:?}, SRC2: {:?}, SRC3: {:?}, SRC4: {:?}, SRC5: {:?}, SRC6: {:?}, SRC7: {:?} }}",
            self.SRC0(),
            self.SRC1(),
            self.SRC2(),
            self.SRC3(),
            self.SRC4(),
            self.SRC5(),
            self.SRC6(),
            self.SRC7()
        )
    }
}
#[doc = "Pin interrupt rising edge register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RISE(pub u32);
impl RISE {
    #[doc = "Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[must_use]
    #[inline(always)]
    pub const fn RDET(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Rising edge detect. Bit n detects the rising edge of the pin selected in PINTSELn. Read 0: No rising edge has been detected on this pin since Reset or the last time a one was written to this bit. Write 0: no operation. Read 1: a rising edge has been detected since Reset or the last time a one was written to this bit. Write 1: clear rising edge detection for this pin."]
    #[inline(always)]
    pub const fn set_RDET(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RISE {
    #[inline(always)]
    fn default() -> RISE {
        RISE(0)
    }
}
impl core::fmt::Debug for RISE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RISE").field("RDET", &self.RDET()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RISE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RISE {{ RDET: {=u8:?} }}", self.RDET())
    }
}
#[doc = "Pin interrupt active level or falling edge interrupt set register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SIENF(pub u32);
impl SIENF {
    #[doc = "Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENAF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address set bits in the IENF, thus enabling interrupts. Bit n sets bit n in the IENF register. 0 = No operation. 1 = Select HIGH-active interrupt or enable falling edge interrupt."]
    #[inline(always)]
    pub const fn set_SETENAF(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SIENF {
    #[inline(always)]
    fn default() -> SIENF {
        SIENF(0)
    }
}
impl core::fmt::Debug for SIENF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIENF")
            .field("SETENAF", &self.SETENAF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SIENF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SIENF {{ SETENAF: {=u8:?} }}", self.SETENAF())
    }
}
#[doc = "Pin interrupt level or rising edge interrupt set register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SIENR(pub u32);
impl SIENR {
    #[doc = "Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn SETENRL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Ones written to this address set bits in the IENR, thus enabling interrupts. Bit n sets bit n in the IENR register. 0 = No operation. 1 = Enable rising edge or level interrupt."]
    #[inline(always)]
    pub const fn set_SETENRL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for SIENR {
    #[inline(always)]
    fn default() -> SIENR {
        SIENR(0)
    }
}
impl core::fmt::Debug for SIENR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIENR")
            .field("SETENRL", &self.SETENRL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SIENR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SIENR {{ SETENRL: {=u8:?} }}", self.SETENRL())
    }
}
