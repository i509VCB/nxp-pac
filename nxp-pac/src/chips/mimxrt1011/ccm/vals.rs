#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcAclkEn {
    #[doc = "ADC alt_clk source is disabled."]
    AdcAclkEn0 = 0x0,
    #[doc = "ADC alt_clk source is enabled."]
    AdcAclkEn1 = 0x01,
}
impl AdcAclkEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcAclkEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcAclkEn {
    #[inline(always)]
    fn from(val: u8) -> AdcAclkEn {
        AdcAclkEn::from_bits(val)
    }
}
impl From<AdcAclkEn> for u8 {
    #[inline(always)]
    fn from(val: AdcAclkEn) -> u8 {
        AdcAclkEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AdcAclkPodf {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "pll3_sw_clk / 8."]
    AdcAclkPodf7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "pll3_sw_clk / 12."]
    AdcAclkPodf11 = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "pll3_sw_clk / 16."]
    AdcAclkPodf15 = 0x0f,
}
impl AdcAclkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AdcAclkPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AdcAclkPodf {
    #[inline(always)]
    fn from(val: u8) -> AdcAclkPodf {
        AdcAclkPodf::from_bits(val)
    }
}
impl From<AdcAclkPodf> for u8 {
    #[inline(always)]
    fn from(val: AdcAclkPodf) -> u8 {
        AdcAclkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodf {
    #[doc = "divide by 1."]
    AhbPodf0 = 0x0,
    #[doc = "divide by 2."]
    AhbPodf1 = 0x01,
    #[doc = "divide by 3."]
    AhbPodf2 = 0x02,
    #[doc = "divide by 4."]
    AhbPodf3 = 0x03,
    #[doc = "divide by 5."]
    AhbPodf4 = 0x04,
    #[doc = "divide by 6."]
    AhbPodf5 = 0x05,
    #[doc = "divide by 7."]
    AhbPodf6 = 0x06,
    #[doc = "divide by 8."]
    AhbPodf7 = 0x07,
}
impl AhbPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodf {
    #[inline(always)]
    fn from(val: u8) -> AhbPodf {
        AhbPodf::from_bits(val)
    }
}
impl From<AhbPodf> for u8 {
    #[inline(always)]
    fn from(val: AhbPodf) -> u8 {
        AhbPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    AhbPodfBusy0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the ahb_podf will be applied."]
    AhbPodfBusy1 = 0x01,
}
impl AhbPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> AhbPodfBusy {
        AhbPodfBusy::from_bits(val)
    }
}
impl From<AhbPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: AhbPodfBusy) -> u8 {
        AhbPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of ahb_podf."]
    AhbPodfLoaded0 = 0x0,
    #[doc = "interrupt generated due to frequency change of ahb_podf."]
    AhbPodfLoaded1 = 0x01,
}
impl AhbPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> AhbPodfLoaded {
        AhbPodfLoaded::from_bits(val)
    }
}
impl From<AhbPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: AhbPodfLoaded) -> u8 {
        AhbPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArmClkDisOnLpm {
    #[doc = "Arm clock enabled on wait mode."]
    ArmClkDisOnLpm0 = 0x0,
    #[doc = "Arm clock disabled on wait mode.."]
    ArmClkDisOnLpm1 = 0x01,
}
impl ArmClkDisOnLpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArmClkDisOnLpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArmClkDisOnLpm {
    #[inline(always)]
    fn from(val: u8) -> ArmClkDisOnLpm {
        ArmClkDisOnLpm::from_bits(val)
    }
}
impl From<ArmClkDisOnLpm> for u8 {
    #[inline(always)]
    fn from(val: ArmClkDisOnLpm) -> u8 {
        ArmClkDisOnLpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Camp2Ready {
    #[doc = "CAMP2 is not ready."]
    Camp2Ready0 = 0x0,
    #[doc = "CAMP2 is ready."]
    Camp2Ready1 = 0x01,
}
impl Camp2Ready {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Camp2Ready {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Camp2Ready {
    #[inline(always)]
    fn from(val: u8) -> Camp2Ready {
        Camp2Ready::from_bits(val)
    }
}
impl From<Camp2Ready> for u8 {
    #[inline(always)]
    fn from(val: Camp2Ready) -> u8 {
        Camp2Ready::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CisrCoscReady {
    #[doc = "interrupt is not generated due to on board oscillator ready."]
    CoscReady0 = 0x0,
    #[doc = "interrupt generated due to on board oscillator ready."]
    CoscReady1 = 0x01,
}
impl CisrCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CisrCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CisrCoscReady {
    #[inline(always)]
    fn from(val: u8) -> CisrCoscReady {
        CisrCoscReady::from_bits(val)
    }
}
impl From<CisrCoscReady> for u8 {
    #[inline(always)]
    fn from(val: CisrCoscReady) -> u8 {
        CisrCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkOutSel {
    #[doc = "CCM_CLKO1 output drives CCM_CLKO1 clock."]
    ClkOutSel0 = 0x0,
    #[doc = "CCM_CLKO1 output drives CCM_CLKO2 clock."]
    ClkOutSel1 = 0x01,
}
impl ClkOutSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkOutSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkOutSel {
    #[inline(always)]
    fn from(val: u8) -> ClkOutSel {
        ClkOutSel::from_bits(val)
    }
}
impl From<ClkOutSel> for u8 {
    #[inline(always)]
    fn from(val: ClkOutSel) -> u8 {
        ClkOutSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1Div {
    #[doc = "divide by 1."]
    Clko1Div0 = 0x0,
    #[doc = "divide by 2."]
    Clko1Div1 = 0x01,
    #[doc = "divide by 3."]
    Clko1Div2 = 0x02,
    #[doc = "divide by 4."]
    Clko1Div3 = 0x03,
    #[doc = "divide by 5."]
    Clko1Div4 = 0x04,
    #[doc = "divide by 6."]
    Clko1Div5 = 0x05,
    #[doc = "divide by 7."]
    Clko1Div6 = 0x06,
    #[doc = "divide by 8."]
    Clko1Div7 = 0x07,
}
impl Clko1Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1Div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1Div {
    #[inline(always)]
    fn from(val: u8) -> Clko1Div {
        Clko1Div::from_bits(val)
    }
}
impl From<Clko1Div> for u8 {
    #[inline(always)]
    fn from(val: Clko1Div) -> u8 {
        Clko1Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1En {
    #[doc = "CCM_CLKO1 disabled."]
    Clko1En0 = 0x0,
    #[doc = "CCM_CLKO1 enabled."]
    Clko1En1 = 0x01,
}
impl Clko1En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1En {
    #[inline(always)]
    fn from(val: u8) -> Clko1En {
        Clko1En::from_bits(val)
    }
}
impl From<Clko1En> for u8 {
    #[inline(always)]
    fn from(val: Clko1En) -> u8 {
        Clko1En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko1Sel {
    #[doc = "pll3_sw_clk (divided by 2)."]
    Clko1Sel0 = 0x0,
    #[doc = "PLL2 (divided by 2)."]
    Clko1Sel1 = 0x01,
    #[doc = "ENET PLL (divided by 2)."]
    Clko1Sel2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    #[doc = "core_clk_root."]
    Clko1Sel11 = 0x0b,
    #[doc = "ipg_clk_root."]
    Clko1Sel12 = 0x0c,
    #[doc = "perclk_root."]
    Clko1Sel13 = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "pll4_main_clk."]
    Clko1Sel15 = 0x0f,
}
impl Clko1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko1Sel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko1Sel {
    #[inline(always)]
    fn from(val: u8) -> Clko1Sel {
        Clko1Sel::from_bits(val)
    }
}
impl From<Clko1Sel> for u8 {
    #[inline(always)]
    fn from(val: Clko1Sel) -> u8 {
        Clko1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2Div {
    #[doc = "divide by 1."]
    Clko2Div0 = 0x0,
    #[doc = "divide by 2."]
    Clko2Div1 = 0x01,
    #[doc = "divide by 3."]
    Clko2Div2 = 0x02,
    #[doc = "divide by 4."]
    Clko2Div3 = 0x03,
    #[doc = "divide by 5."]
    Clko2Div4 = 0x04,
    #[doc = "divide by 6."]
    Clko2Div5 = 0x05,
    #[doc = "divide by 7."]
    Clko2Div6 = 0x06,
    #[doc = "divide by 8."]
    Clko2Div7 = 0x07,
}
impl Clko2Div {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2Div {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2Div {
    #[inline(always)]
    fn from(val: u8) -> Clko2Div {
        Clko2Div::from_bits(val)
    }
}
impl From<Clko2Div> for u8 {
    #[inline(always)]
    fn from(val: Clko2Div) -> u8 {
        Clko2Div::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2En {
    #[doc = "CCM_CLKO2 disabled."]
    Clko2En0 = 0x0,
    #[doc = "CCM_CLKO2 enabled."]
    Clko2En1 = 0x01,
}
impl Clko2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2En {
    #[inline(always)]
    fn from(val: u8) -> Clko2En {
        Clko2En::from_bits(val)
    }
}
impl From<Clko2En> for u8 {
    #[inline(always)]
    fn from(val: Clko2En) -> u8 {
        Clko2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clko2Sel {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "lpi2c_clk_root."]
    Clko2Sel6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "osc_clk."]
    Clko2Sel14 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "lpspi_clk_root."]
    Clko2Sel16 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "sai1_clk_root."]
    Clko2Sel18 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "sai3_clk_root."]
    Clko2Sel20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "trace_clk_root."]
    Clko2Sel22 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "flexspi_clk_root."]
    Clko2Sel27 = 0x1b,
    #[doc = "uart_clk_root."]
    Clko2Sel28 = 0x1c,
    #[doc = "spdif0_clk_root."]
    Clko2Sel29 = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl Clko2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clko2Sel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clko2Sel {
    #[inline(always)]
    fn from(val: u8) -> Clko2Sel {
        Clko2Sel::from_bits(val)
    }
}
impl From<Clko2Sel> for u8 {
    #[inline(always)]
    fn from(val: Clko2Sel) -> u8 {
        Clko2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoscEn {
    #[doc = "disable on chip oscillator."]
    CoscEn0 = 0x0,
    #[doc = "enable on chip oscillator."]
    CoscEn1 = 0x01,
}
impl CoscEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoscEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoscEn {
    #[inline(always)]
    fn from(val: u8) -> CoscEn {
        CoscEn::from_bits(val)
    }
}
impl From<CoscEn> for u8 {
    #[inline(always)]
    fn from(val: CoscEn) -> u8 {
        CoscEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoscPwrdown {
    #[doc = "On chip oscillator will not be powered down, i.e. cosc_pwrdown = '0'."]
    CoscPwrdown0 = 0x0,
    #[doc = "On chip oscillator will be powered down, i.e. cosc_pwrdown = '1'."]
    CoscPwrdown1 = 0x01,
}
impl CoscPwrdown {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoscPwrdown {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoscPwrdown {
    #[inline(always)]
    fn from(val: u8) -> CoscPwrdown {
        CoscPwrdown::from_bits(val)
    }
}
impl From<CoscPwrdown> for u8 {
    #[inline(always)]
    fn from(val: CoscPwrdown) -> u8 {
        CoscPwrdown::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CsrCoscReady {
    #[doc = "on board oscillator is not ready."]
    CoscReady0 = 0x0,
    #[doc = "on board oscillator is ready."]
    CoscReady1 = 0x01,
}
impl CsrCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CsrCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CsrCoscReady {
    #[inline(always)]
    fn from(val: u8) -> CsrCoscReady {
        CsrCoscReady::from_bits(val)
    }
}
impl From<CsrCoscReady> for u8 {
    #[inline(always)]
    fn from(val: CsrCoscReady) -> u8 {
        CsrCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisRefOsc {
    #[doc = "external high frequency oscillator will be enabled, i.e. CCM_REF_EN_B = '0'."]
    DisRefOsc0 = 0x0,
    #[doc = "external high frequency oscillator will be disabled, i.e. CCM_REF_EN_B = '1'."]
    DisRefOsc1 = 0x01,
}
impl DisRefOsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisRefOsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisRefOsc {
    #[inline(always)]
    fn from(val: u8) -> DisRefOsc {
        DisRefOsc::from_bits(val)
    }
}
impl From<DisRefOsc> for u8 {
    #[inline(always)]
    fn from(val: DisRefOsc) -> u8 {
        DisRefOsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EfuseProgSupplyGate {
    #[doc = "fuse programing supply voltage is gated off to the efuse module."]
    EfuseProgSupplyGate0 = 0x0,
    #[doc = "allow fuse programing."]
    EfuseProgSupplyGate1 = 0x01,
}
impl EfuseProgSupplyGate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EfuseProgSupplyGate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EfuseProgSupplyGate {
    #[inline(always)]
    fn from(val: u8) -> EfuseProgSupplyGate {
        EfuseProgSupplyGate::from_bits(val)
    }
}
impl From<EfuseProgSupplyGate> for u8 {
    #[inline(always)]
    fn from(val: EfuseProgSupplyGate) -> u8 {
        EfuseProgSupplyGate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
}
impl Flexio1ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkPodf {
        Flexio1ClkPodf::from_bits(val)
    }
}
impl From<Flexio1ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkPodf) -> u8 {
        Flexio1ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkPred {
    #[doc = "divide by 1."]
    Flexio1ClkPred0 = 0x0,
    #[doc = "divide by 2."]
    Flexio1ClkPred1 = 0x01,
    #[doc = "divide by 3."]
    Flexio1ClkPred2 = 0x02,
    #[doc = "divide by 4."]
    Flexio1ClkPred3 = 0x03,
    #[doc = "divide by 5."]
    Flexio1ClkPred4 = 0x04,
    #[doc = "divide by 6."]
    Flexio1ClkPred5 = 0x05,
    #[doc = "divide by 7."]
    Flexio1ClkPred6 = 0x06,
    #[doc = "divide by 8."]
    Flexio1ClkPred7 = 0x07,
}
impl Flexio1ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkPred {
        Flexio1ClkPred::from_bits(val)
    }
}
impl From<Flexio1ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkPred) -> u8 {
        Flexio1ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexio1ClkSel {
    #[doc = "derive clock from PLL4 divided clock."]
    Flexio1ClkSel0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2 clock."]
    Flexio1ClkSel1 = 0x01,
    #[doc = "derive from PLL2."]
    Flexio1ClkSel2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk."]
    Flexio1ClkSel3 = 0x03,
}
impl Flexio1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexio1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexio1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Flexio1ClkSel {
        Flexio1ClkSel::from_bits(val)
    }
}
impl From<Flexio1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Flexio1ClkSel) -> u8 {
        Flexio1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkSel {
    #[doc = "derive clock from PLL2."]
    FlexspiClkSel0 = 0x0,
    #[doc = "derive clock from pll3_sw_clk."]
    FlexspiClkSel1 = 0x01,
    #[doc = "derive clock from PLL2 PFD2."]
    FlexspiClkSel2 = 0x02,
    #[doc = "derive clock from PLL3 PFD0."]
    FlexspiClkSel3 = 0x03,
}
impl FlexspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkSel {
        FlexspiClkSel::from_bits(val)
    }
}
impl From<FlexspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkSel) -> u8 {
        FlexspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiClkSrc {
    #[doc = "derive clock selected by CCM_CSCMR1\\[FLEXSPI_CLK_SEL\\]."]
    FlexspiClkSrc0 = 0x0,
    #[doc = "derive clock selected by CCM_CBCMR\\[PERIPH_CLK2_ SEL\\]."]
    FlexspiClkSrc1 = 0x01,
}
impl FlexspiClkSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiClkSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiClkSrc {
    #[inline(always)]
    fn from(val: u8) -> FlexspiClkSrc {
        FlexspiClkSrc::from_bits(val)
    }
}
impl From<FlexspiClkSrc> for u8 {
    #[inline(always)]
    fn from(val: FlexspiClkSrc) -> u8 {
        FlexspiClkSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiPodf {
    #[doc = "divide by 1."]
    FlexspiPodf0 = 0x0,
    #[doc = "divide by 2."]
    FlexspiPodf1 = 0x01,
    #[doc = "divide by 3."]
    FlexspiPodf2 = 0x02,
    #[doc = "divide by 4."]
    FlexspiPodf3 = 0x03,
    #[doc = "divide by 5."]
    FlexspiPodf4 = 0x04,
    #[doc = "divide by 6."]
    FlexspiPodf5 = 0x05,
    #[doc = "divide by 7."]
    FlexspiPodf6 = 0x06,
    #[doc = "divide by 8."]
    FlexspiPodf7 = 0x07,
}
impl FlexspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiPodf {
    #[inline(always)]
    fn from(val: u8) -> FlexspiPodf {
        FlexspiPodf::from_bits(val)
    }
}
impl From<FlexspiPodf> for u8 {
    #[inline(always)]
    fn from(val: FlexspiPodf) -> u8 {
        FlexspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    FlexspiPodfBusy0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the flexspi_podf will be applied."]
    FlexspiPodfBusy1 = 0x01,
}
impl FlexspiPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> FlexspiPodfBusy {
        FlexspiPodfBusy::from_bits(val)
    }
}
impl From<FlexspiPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: FlexspiPodfBusy) -> u8 {
        FlexspiPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of flexspi_podf."]
    FlexspiPodfLoaded0 = 0x0,
    #[doc = "interrupt generated due to frequency change of flexspi_podf."]
    FlexspiPodfLoaded1 = 0x01,
}
impl FlexspiPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> FlexspiPodfLoaded {
        FlexspiPodfLoaded::from_bits(val)
    }
}
impl From<FlexspiPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: FlexspiPodfLoaded) -> u8 {
        FlexspiPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fpl {
    #[doc = "Engage PLL enable default way."]
    Fpl0 = 0x0,
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    Fpl1 = 0x01,
}
impl Fpl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fpl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fpl {
    #[inline(always)]
    fn from(val: u8) -> Fpl {
        Fpl::from_bits(val)
    }
}
impl From<Fpl> for u8 {
    #[inline(always)]
    fn from(val: Fpl) -> u8 {
        Fpl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IntMemClkLpm {
    #[doc = "Disable the clock to the Arm platform memories when entering Low Power Mode."]
    IntMemClkLpm0 = 0x0,
    #[doc = "Keep the clocks to the Arm platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)."]
    IntMemClkLpm1 = 0x01,
}
impl IntMemClkLpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntMemClkLpm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntMemClkLpm {
    #[inline(always)]
    fn from(val: u8) -> IntMemClkLpm {
        IntMemClkLpm::from_bits(val)
    }
}
impl From<IntMemClkLpm> for u8 {
    #[inline(always)]
    fn from(val: IntMemClkLpm) -> u8 {
        IntMemClkLpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpgPodf {
    #[doc = "divide by 1."]
    IpgPodf0 = 0x0,
    #[doc = "divide by 2."]
    IpgPodf1 = 0x01,
    #[doc = "divide by 3."]
    IpgPodf2 = 0x02,
    #[doc = "divide by 4."]
    IpgPodf3 = 0x03,
}
impl IpgPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpgPodf {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpgPodf {
    #[inline(always)]
    fn from(val: u8) -> IpgPodf {
        IpgPodf::from_bits(val)
    }
}
impl From<IpgPodf> for u8 {
    #[inline(always)]
    fn from(val: IpgPodf) -> u8 {
        IpgPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
    #[doc = "Divide by 17."]
    Divide17 = 0x10,
    #[doc = "Divide by 18."]
    Divide18 = 0x11,
    #[doc = "Divide by 19."]
    Divide19 = 0x12,
    #[doc = "Divide by 20."]
    Divide20 = 0x13,
    #[doc = "Divide by 21."]
    Divide21 = 0x14,
    #[doc = "Divide by 22."]
    Divide22 = 0x15,
    #[doc = "Divide by 23."]
    Divide23 = 0x16,
    #[doc = "Divide by 24."]
    Divide24 = 0x17,
    #[doc = "Divide by 25."]
    Divide25 = 0x18,
    #[doc = "Divide by 26."]
    Divide26 = 0x19,
    #[doc = "Divide by 27."]
    Divide27 = 0x1a,
    #[doc = "Divide by 28."]
    Divide28 = 0x1b,
    #[doc = "Divide by 29."]
    Divide29 = 0x1c,
    #[doc = "Divide by 30."]
    Divide30 = 0x1d,
    #[doc = "Divide by 31."]
    Divide31 = 0x1e,
    #[doc = "Divide by 32."]
    Divide32 = 0x1f,
    #[doc = "Divide by 33."]
    Divide33 = 0x20,
    #[doc = "Divide by 34."]
    Divide34 = 0x21,
    #[doc = "Divide by 35."]
    Divide35 = 0x22,
    #[doc = "Divide by 36."]
    Divide36 = 0x23,
    #[doc = "Divide by 37."]
    Divide37 = 0x24,
    #[doc = "Divide by 38."]
    Divide38 = 0x25,
    #[doc = "Divide by 39."]
    Divide39 = 0x26,
    #[doc = "Divide by 40."]
    Divide40 = 0x27,
    #[doc = "Divide by 41."]
    Divide41 = 0x28,
    #[doc = "Divide by 42."]
    Divide42 = 0x29,
    #[doc = "Divide by 43."]
    Divide43 = 0x2a,
    #[doc = "Divide by 44."]
    Divide44 = 0x2b,
    #[doc = "Divide by 45."]
    Divide45 = 0x2c,
    #[doc = "Divide by 46."]
    Divide46 = 0x2d,
    #[doc = "Divide by 47."]
    Divide47 = 0x2e,
    #[doc = "Divide by 48."]
    Divide48 = 0x2f,
    #[doc = "Divide by 49."]
    Divide49 = 0x30,
    #[doc = "Divide by 50."]
    Divide50 = 0x31,
    #[doc = "Divide by 51."]
    Divide51 = 0x32,
    #[doc = "Divide by 52."]
    Divide52 = 0x33,
    #[doc = "Divide by 53."]
    Divide53 = 0x34,
    #[doc = "Divide by 54."]
    Divide54 = 0x35,
    #[doc = "Divide by 55."]
    Divide55 = 0x36,
    #[doc = "Divide by 56."]
    Divide56 = 0x37,
    #[doc = "Divide by 57."]
    Divide57 = 0x38,
    #[doc = "Divide by 58."]
    Divide58 = 0x39,
    #[doc = "Divide by 59."]
    Divide59 = 0x3a,
    #[doc = "Divide by 60."]
    Divide60 = 0x3b,
    #[doc = "Divide by 61."]
    Divide61 = 0x3c,
    #[doc = "Divide by 62."]
    Divide62 = 0x3d,
    #[doc = "Divide by 63."]
    Divide63 = 0x3e,
    #[doc = "Divide by 64."]
    Divide64 = 0x3f,
}
impl Lpi2cClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkPodf {
        Lpi2cClkPodf::from_bits(val)
    }
}
impl From<Lpi2cClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkPodf) -> u8 {
        Lpi2cClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpi2cClkSel {
    #[doc = "derive clock from pll3_60m."]
    Lpi2cClkSel0 = 0x0,
    #[doc = "derive clock from osc_clk."]
    Lpi2cClkSel1 = 0x01,
}
impl Lpi2cClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpi2cClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpi2cClkSel {
    #[inline(always)]
    fn from(val: u8) -> Lpi2cClkSel {
        Lpi2cClkSel::from_bits(val)
    }
}
impl From<Lpi2cClkSel> for u8 {
    #[inline(always)]
    fn from(val: Lpi2cClkSel) -> u8 {
        Lpi2cClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpm {
    #[doc = "Remain in run mode."]
    Lpm0 = 0x0,
    #[doc = "Transfer to wait mode."]
    Lpm1 = 0x01,
    #[doc = "Transfer to stop mode."]
    Lpm2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Lpm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpm {
    #[inline(always)]
    fn from(val: u8) -> Lpm {
        Lpm::from_bits(val)
    }
}
impl From<Lpm> for u8 {
    #[inline(always)]
    fn from(val: Lpm) -> u8 {
        Lpm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiClkSel {
    #[doc = "derive clock from PLL3 PFD1 clk."]
    LpspiClkSel0 = 0x0,
    #[doc = "derive clock from PLL3 PFD0."]
    LpspiClkSel1 = 0x01,
    #[doc = "derive clock from PLL2."]
    LpspiClkSel2 = 0x02,
    #[doc = "derive clock from PLL2 PFD2."]
    LpspiClkSel3 = 0x03,
}
impl LpspiClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiClkSel {
    #[inline(always)]
    fn from(val: u8) -> LpspiClkSel {
        LpspiClkSel::from_bits(val)
    }
}
impl From<LpspiClkSel> for u8 {
    #[inline(always)]
    fn from(val: LpspiClkSel) -> u8 {
        LpspiClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpspiPodf {
    #[doc = "divide by 1."]
    LpspiPodf0 = 0x0,
    #[doc = "divide by 2."]
    LpspiPodf1 = 0x01,
    #[doc = "divide by 3."]
    LpspiPodf2 = 0x02,
    #[doc = "divide by 4."]
    LpspiPodf3 = 0x03,
    #[doc = "divide by 5."]
    LpspiPodf4 = 0x04,
    #[doc = "divide by 6."]
    LpspiPodf5 = 0x05,
    #[doc = "divide by 7."]
    LpspiPodf6 = 0x06,
    #[doc = "divide by 8."]
    LpspiPodf7 = 0x07,
    #[doc = "divide by 9."]
    LpspiPodf8 = 0x08,
    #[doc = "divide by 10."]
    LpspiPodf9 = 0x09,
    #[doc = "divide by 11."]
    LpspiPodf10 = 0x0a,
    #[doc = "divide by 12."]
    LpspiPodf11 = 0x0b,
    #[doc = "divide by 13."]
    LpspiPodf12 = 0x0c,
    #[doc = "divide by 14."]
    LpspiPodf13 = 0x0d,
    #[doc = "divide by 15."]
    LpspiPodf14 = 0x0e,
    #[doc = "divide by 16."]
    LpspiPodf15 = 0x0f,
}
impl LpspiPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpspiPodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpspiPodf {
    #[inline(always)]
    fn from(val: u8) -> LpspiPodf {
        LpspiPodf::from_bits(val)
    }
}
impl From<LpspiPodf> for u8 {
    #[inline(always)]
    fn from(val: LpspiPodf) -> u8 {
        LpspiPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LrfPll {
    #[doc = "interrupt is not generated due to lock ready of all enabled and not bypaseed PLLs."]
    LrfPll0 = 0x0,
    #[doc = "interrupt generated due to lock ready of all enabled and not bypaseed PLLs."]
    LrfPll1 = 0x01,
}
impl LrfPll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LrfPll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LrfPll {
    #[inline(always)]
    fn from(val: u8) -> LrfPll {
        LrfPll::from_bits(val)
    }
}
impl From<LrfPll> for u8 {
    #[inline(always)]
    fn from(val: LrfPll) -> u8 {
        LrfPll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskAhbPodfLoaded {
    #[doc = "don't mask interrupt due to frequency change of ahb_podf - interrupt will be created."]
    MaskAhbPodfLoaded0 = 0x0,
    #[doc = "mask interrupt due to frequency change of ahb_podf."]
    MaskAhbPodfLoaded1 = 0x01,
}
impl MaskAhbPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskAhbPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskAhbPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskAhbPodfLoaded {
        MaskAhbPodfLoaded::from_bits(val)
    }
}
impl From<MaskAhbPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskAhbPodfLoaded) -> u8 {
        MaskAhbPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskCore0Wfi {
    #[doc = "WFI of core0 is not masked."]
    MaskCore0Wfi0 = 0x0,
    #[doc = "WFI of core0 is masked."]
    MaskCore0Wfi1 = 0x01,
}
impl MaskCore0Wfi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskCore0Wfi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskCore0Wfi {
    #[inline(always)]
    fn from(val: u8) -> MaskCore0Wfi {
        MaskCore0Wfi::from_bits(val)
    }
}
impl From<MaskCore0Wfi> for u8 {
    #[inline(always)]
    fn from(val: MaskCore0Wfi) -> u8 {
        MaskCore0Wfi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskCoscReady {
    #[doc = "don't mask interrupt due to on board oscillator ready - interrupt will be created."]
    MaskCoscReady0 = 0x0,
    #[doc = "mask interrupt due to on board oscillator ready."]
    MaskCoscReady1 = 0x01,
}
impl MaskCoscReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskCoscReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskCoscReady {
    #[inline(always)]
    fn from(val: u8) -> MaskCoscReady {
        MaskCoscReady::from_bits(val)
    }
}
impl From<MaskCoscReady> for u8 {
    #[inline(always)]
    fn from(val: MaskCoscReady) -> u8 {
        MaskCoscReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskFlexspiPodfLoaded {
    #[doc = "don't mask interrupt due to update of flexspi_podf."]
    MaskFlexspiPodfLoaded0 = 0x0,
    #[doc = "mask interrupt due to update of flexspi_podf."]
    MaskFlexspiPodfLoaded1 = 0x01,
}
impl MaskFlexspiPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskFlexspiPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskFlexspiPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskFlexspiPodfLoaded {
        MaskFlexspiPodfLoaded::from_bits(val)
    }
}
impl From<MaskFlexspiPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskFlexspiPodfLoaded) -> u8 {
        MaskFlexspiPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskL2ccIdle {
    #[doc = "L2CC IDLE is not masked."]
    MaskL2ccIdle0 = 0x0,
    #[doc = "L2CC IDLE is masked."]
    MaskL2ccIdle1 = 0x01,
}
impl MaskL2ccIdle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskL2ccIdle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskL2ccIdle {
    #[inline(always)]
    fn from(val: u8) -> MaskL2ccIdle {
        MaskL2ccIdle::from_bits(val)
    }
}
impl From<MaskL2ccIdle> for u8 {
    #[inline(always)]
    fn from(val: MaskL2ccIdle) -> u8 {
        MaskL2ccIdle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskLrfPll {
    #[doc = "don't mask interrupt due to lrf of PLLs - interrupt will be created."]
    MaskLrfPll0 = 0x0,
    #[doc = "mask interrupt due to lrf of PLLs."]
    MaskLrfPll1 = 0x01,
}
impl MaskLrfPll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskLrfPll {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskLrfPll {
    #[inline(always)]
    fn from(val: u8) -> MaskLrfPll {
        MaskLrfPll::from_bits(val)
    }
}
impl From<MaskLrfPll> for u8 {
    #[inline(always)]
    fn from(val: MaskLrfPll) -> u8 {
        MaskLrfPll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskPerclkPodfLoaded {
    #[doc = "don't mask interrupt due to update of perclk_podf."]
    MaskPerclkPodfLoaded0 = 0x0,
    #[doc = "mask interrupt due to update of perclk_podf."]
    MaskPerclkPodfLoaded1 = 0x01,
}
impl MaskPerclkPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskPerclkPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskPerclkPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskPerclkPodfLoaded {
        MaskPerclkPodfLoaded::from_bits(val)
    }
}
impl From<MaskPerclkPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskPerclkPodfLoaded) -> u8 {
        MaskPerclkPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskPeriphClkSelLoaded {
    #[doc = "don't mask interrupt due to update of periph_clk_sel - interrupt will be created."]
    MaskPeriphClkSelLoaded0 = 0x0,
    #[doc = "mask interrupt due to update of periph_clk_sel."]
    MaskPeriphClkSelLoaded1 = 0x01,
}
impl MaskPeriphClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskPeriphClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskPeriphClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> MaskPeriphClkSelLoaded {
        MaskPeriphClkSelLoaded::from_bits(val)
    }
}
impl From<MaskPeriphClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: MaskPeriphClkSelLoaded) -> u8 {
        MaskPeriphClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MaskScuIdle {
    #[doc = "SCU IDLE is not masked."]
    MaskScuIdle0 = 0x0,
    #[doc = "SCU IDLE is masked."]
    MaskScuIdle1 = 0x01,
}
impl MaskScuIdle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MaskScuIdle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MaskScuIdle {
    #[inline(always)]
    fn from(val: u8) -> MaskScuIdle {
        MaskScuIdle::from_bits(val)
    }
}
impl From<MaskScuIdle> for u8 {
    #[inline(always)]
    fn from(val: MaskScuIdle) -> u8 {
        MaskScuIdle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvGpt {
    #[doc = "don't override module enable signal."]
    ModEnOvGpt0 = 0x0,
    #[doc = "override module enable signal."]
    ModEnOvGpt1 = 0x01,
}
impl ModEnOvGpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvGpt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvGpt {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvGpt {
        ModEnOvGpt::from_bits(val)
    }
}
impl From<ModEnOvGpt> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvGpt) -> u8 {
        ModEnOvGpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvPit {
    #[doc = "don't override module enable signal."]
    ModEnOvPit0 = 0x0,
    #[doc = "override module enable signal."]
    ModEnOvPit1 = 0x01,
}
impl ModEnOvPit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvPit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvPit {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvPit {
        ModEnOvPit::from_bits(val)
    }
}
impl From<ModEnOvPit> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvPit) -> u8 {
        ModEnOvPit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModEnOvTrng {
    #[doc = "don't override module enable signal."]
    ModEnOvTrng0 = 0x0,
    #[doc = "override module enable signal."]
    ModEnOvTrng1 = 0x01,
}
impl ModEnOvTrng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModEnOvTrng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModEnOvTrng {
    #[inline(always)]
    fn from(val: u8) -> ModEnOvTrng {
        ModEnOvTrng::from_bits(val)
    }
}
impl From<ModEnOvTrng> for u8 {
    #[inline(always)]
    fn from(val: ModEnOvTrng) -> u8 {
        ModEnOvTrng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkClkSel {
    #[doc = "derive clock from ipg clk root."]
    PerclkClkSel0 = 0x0,
    #[doc = "derive clock from osc_clk."]
    PerclkClkSel1 = 0x01,
}
impl PerclkClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkClkSel {
    #[inline(always)]
    fn from(val: u8) -> PerclkClkSel {
        PerclkClkSel::from_bits(val)
    }
}
impl From<PerclkClkSel> for u8 {
    #[inline(always)]
    fn from(val: PerclkClkSel) -> u8 {
        PerclkClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
    #[doc = "Divide by 17."]
    Divide17 = 0x10,
    #[doc = "Divide by 18."]
    Divide18 = 0x11,
    #[doc = "Divide by 19."]
    Divide19 = 0x12,
    #[doc = "Divide by 20."]
    Divide20 = 0x13,
    #[doc = "Divide by 21."]
    Divide21 = 0x14,
    #[doc = "Divide by 22."]
    Divide22 = 0x15,
    #[doc = "Divide by 23."]
    Divide23 = 0x16,
    #[doc = "Divide by 24."]
    Divide24 = 0x17,
    #[doc = "Divide by 25."]
    Divide25 = 0x18,
    #[doc = "Divide by 26."]
    Divide26 = 0x19,
    #[doc = "Divide by 27."]
    Divide27 = 0x1a,
    #[doc = "Divide by 28."]
    Divide28 = 0x1b,
    #[doc = "Divide by 29."]
    Divide29 = 0x1c,
    #[doc = "Divide by 30."]
    Divide30 = 0x1d,
    #[doc = "Divide by 31."]
    Divide31 = 0x1e,
    #[doc = "Divide by 32."]
    Divide32 = 0x1f,
    #[doc = "Divide by 33."]
    Divide33 = 0x20,
    #[doc = "Divide by 34."]
    Divide34 = 0x21,
    #[doc = "Divide by 35."]
    Divide35 = 0x22,
    #[doc = "Divide by 36."]
    Divide36 = 0x23,
    #[doc = "Divide by 37."]
    Divide37 = 0x24,
    #[doc = "Divide by 38."]
    Divide38 = 0x25,
    #[doc = "Divide by 39."]
    Divide39 = 0x26,
    #[doc = "Divide by 40."]
    Divide40 = 0x27,
    #[doc = "Divide by 41."]
    Divide41 = 0x28,
    #[doc = "Divide by 42."]
    Divide42 = 0x29,
    #[doc = "Divide by 43."]
    Divide43 = 0x2a,
    #[doc = "Divide by 44."]
    Divide44 = 0x2b,
    #[doc = "Divide by 45."]
    Divide45 = 0x2c,
    #[doc = "Divide by 46."]
    Divide46 = 0x2d,
    #[doc = "Divide by 47."]
    Divide47 = 0x2e,
    #[doc = "Divide by 48."]
    Divide48 = 0x2f,
    #[doc = "Divide by 49."]
    Divide49 = 0x30,
    #[doc = "Divide by 50."]
    Divide50 = 0x31,
    #[doc = "Divide by 51."]
    Divide51 = 0x32,
    #[doc = "Divide by 52."]
    Divide52 = 0x33,
    #[doc = "Divide by 53."]
    Divide53 = 0x34,
    #[doc = "Divide by 54."]
    Divide54 = 0x35,
    #[doc = "Divide by 55."]
    Divide55 = 0x36,
    #[doc = "Divide by 56."]
    Divide56 = 0x37,
    #[doc = "Divide by 57."]
    Divide57 = 0x38,
    #[doc = "Divide by 58."]
    Divide58 = 0x39,
    #[doc = "Divide by 59."]
    Divide59 = 0x3a,
    #[doc = "Divide by 60."]
    Divide60 = 0x3b,
    #[doc = "Divide by 61."]
    Divide61 = 0x3c,
    #[doc = "Divide by 62."]
    Divide62 = 0x3d,
    #[doc = "Divide by 63."]
    Divide63 = 0x3e,
    #[doc = "Divide by 64."]
    Divide64 = 0x3f,
}
impl PerclkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkPodf {
    #[inline(always)]
    fn from(val: u8) -> PerclkPodf {
        PerclkPodf::from_bits(val)
    }
}
impl From<PerclkPodf> for u8 {
    #[inline(always)]
    fn from(val: PerclkPodf) -> u8 {
        PerclkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkPodfBusy {
    #[doc = "divider is not busy and its value represents the actual division."]
    PerclkPodfBusy0 = 0x0,
    #[doc = "divider is busy with handshake process with module. The value read in the divider represents the previous value of the division factor, and after the handshake the written value of the perclk_podf will be applied."]
    PerclkPodfBusy1 = 0x01,
}
impl PerclkPodfBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkPodfBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkPodfBusy {
    #[inline(always)]
    fn from(val: u8) -> PerclkPodfBusy {
        PerclkPodfBusy::from_bits(val)
    }
}
impl From<PerclkPodfBusy> for u8 {
    #[inline(always)]
    fn from(val: PerclkPodfBusy) -> u8 {
        PerclkPodfBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PerclkPodfLoaded {
    #[doc = "interrupt is not generated due to frequency change of perclk_podf."]
    PerclkPodfLoaded0 = 0x0,
    #[doc = "interrupt generated due to frequency change of perclk_podf."]
    PerclkPodfLoaded1 = 0x01,
}
impl PerclkPodfLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PerclkPodfLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PerclkPodfLoaded {
    #[inline(always)]
    fn from(val: u8) -> PerclkPodfLoaded {
        PerclkPodfLoaded::from_bits(val)
    }
}
impl From<PerclkPodfLoaded> for u8 {
    #[inline(always)]
    fn from(val: PerclkPodfLoaded) -> u8 {
        PerclkPodfLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClk2Sel {
    #[doc = "derive clock from pll3_sw_clk."]
    PeriphClk2Sel0 = 0x0,
    #[doc = "derive clock from osc_clk."]
    PeriphClk2Sel1 = 0x01,
    #[doc = "derive clock from pll2_bypass_clk."]
    PeriphClk2Sel2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl PeriphClk2Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClk2Sel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClk2Sel {
    #[inline(always)]
    fn from(val: u8) -> PeriphClk2Sel {
        PeriphClk2Sel::from_bits(val)
    }
}
impl From<PeriphClk2Sel> for u8 {
    #[inline(always)]
    fn from(val: PeriphClk2Sel) -> u8 {
        PeriphClk2Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSel {
    #[doc = "derive clock selected by CCM_CBCMR\\[CORE_CLK_PRE_SEL\\]."]
    PeriphClkSel0 = 0x0,
    #[doc = "derive clock selected by CCM_CBCMR\\[PERIPH_CLK2_SEL\\]."]
    PeriphClkSel1 = 0x01,
}
impl PeriphClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSel {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSel {
        PeriphClkSel::from_bits(val)
    }
}
impl From<PeriphClkSel> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSel) -> u8 {
        PeriphClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSelBusy {
    #[doc = "mux is not busy and its value represents the actual division."]
    PeriphClkSelBusy0 = 0x0,
    #[doc = "mux is busy with handshake process with module. The value read in the periph_clk_sel represents the previous value of select, and after the handshake periph_clk_sel value will be applied."]
    PeriphClkSelBusy1 = 0x01,
}
impl PeriphClkSelBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSelBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSelBusy {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSelBusy {
        PeriphClkSelBusy::from_bits(val)
    }
}
impl From<PeriphClkSelBusy> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSelBusy) -> u8 {
        PeriphClkSelBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PeriphClkSelLoaded {
    #[doc = "interrupt is not generated due to update of periph_clk_sel."]
    PeriphClkSelLoaded0 = 0x0,
    #[doc = "interrupt generated due to update of periph_clk_sel."]
    PeriphClkSelLoaded1 = 0x01,
}
impl PeriphClkSelLoaded {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PeriphClkSelLoaded {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PeriphClkSelLoaded {
    #[inline(always)]
    fn from(val: u8) -> PeriphClkSelLoaded {
        PeriphClkSelLoaded::from_bits(val)
    }
}
impl From<PeriphClkSelLoaded> for u8 {
    #[inline(always)]
    fn from(val: PeriphClkSelLoaded) -> u8 {
        PeriphClkSelLoaded::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll3SwClkSel {
    #[doc = "pll3_main_clk."]
    Pll3SwClkSel0 = 0x0,
    #[doc = "pll3 bypass clock."]
    Pll3SwClkSel1 = 0x01,
}
impl Pll3SwClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll3SwClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll3SwClkSel {
    #[inline(always)]
    fn from(val: u8) -> Pll3SwClkSel {
        Pll3SwClkSel::from_bits(val)
    }
}
impl From<Pll3SwClkSel> for u8 {
    #[inline(always)]
    fn from(val: Pll3SwClkSel) -> u8 {
        Pll3SwClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicDelayScaler {
    #[doc = "clock is not divided."]
    PmicDelayScaler0 = 0x0,
    #[doc = "clock is divided /8."]
    PmicDelayScaler1 = 0x01,
}
impl PmicDelayScaler {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicDelayScaler {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicDelayScaler {
    #[inline(always)]
    fn from(val: u8) -> PmicDelayScaler {
        PmicDelayScaler::from_bits(val)
    }
}
impl From<PmicDelayScaler> for u8 {
    #[inline(always)]
    fn from(val: PmicDelayScaler) -> u8 {
        PmicDelayScaler::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrePeriphClkSel {
    #[doc = "derive clock from PLL2."]
    PrePeriphClkSel0 = 0x0,
    #[doc = "derive clock from PLL3 PFD3."]
    PrePeriphClkSel1 = 0x01,
    #[doc = "derive clock from PLL2 PFD3."]
    PrePeriphClkSel2 = 0x02,
    #[doc = "derive clock from PLL6."]
    PrePeriphClkSel3 = 0x03,
}
impl PrePeriphClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrePeriphClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrePeriphClkSel {
    #[inline(always)]
    fn from(val: u8) -> PrePeriphClkSel {
        PrePeriphClkSel::from_bits(val)
    }
}
impl From<PrePeriphClkSel> for u8 {
    #[inline(always)]
    fn from(val: PrePeriphClkSel) -> u8 {
        PrePeriphClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RbcEn {
    #[doc = "REG_BYPASS_COUNTER disabled."]
    RbcEn0 = 0x0,
    #[doc = "REG_BYPASS_COUNTER enabled."]
    RbcEn1 = 0x01,
}
impl RbcEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RbcEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RbcEn {
    #[inline(always)]
    fn from(val: u8) -> RbcEn {
        RbcEn::from_bits(val)
    }
}
impl From<RbcEn> for u8 {
    #[inline(always)]
    fn from(val: RbcEn) -> u8 {
        RbcEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RefEnB {
    #[doc = "value of CCM_REF_EN_B is '0'."]
    RefEnB0 = 0x0,
    #[doc = "value of CCM_REF_EN_B is '1'."]
    RefEnB1 = 0x01,
}
impl RefEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RefEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RefEnB {
    #[inline(always)]
    fn from(val: u8) -> RefEnB {
        RefEnB::from_bits(val)
    }
}
impl From<RefEnB> for u8 {
    #[inline(always)]
    fn from(val: RefEnB) -> u8 {
        RefEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RegBypassCount {
    #[doc = "no delay."]
    RegBypassCount0 = 0x0,
    #[doc = "1 CKIL clock period delay."]
    RegBypassCount1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    #[doc = "63 CKIL clock periods delay."]
    RegBypassCount63 = 0x3f,
}
impl RegBypassCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RegBypassCount {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RegBypassCount {
    #[inline(always)]
    fn from(val: u8) -> RegBypassCount {
        RegBypassCount::from_bits(val)
    }
}
impl From<RegBypassCount> for u8 {
    #[inline(always)]
    fn from(val: RegBypassCount) -> u8 {
        RegBypassCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
    #[doc = "Divide by 17."]
    Divide17 = 0x10,
    #[doc = "Divide by 18."]
    Divide18 = 0x11,
    #[doc = "Divide by 19."]
    Divide19 = 0x12,
    #[doc = "Divide by 20."]
    Divide20 = 0x13,
    #[doc = "Divide by 21."]
    Divide21 = 0x14,
    #[doc = "Divide by 22."]
    Divide22 = 0x15,
    #[doc = "Divide by 23."]
    Divide23 = 0x16,
    #[doc = "Divide by 24."]
    Divide24 = 0x17,
    #[doc = "Divide by 25."]
    Divide25 = 0x18,
    #[doc = "Divide by 26."]
    Divide26 = 0x19,
    #[doc = "Divide by 27."]
    Divide27 = 0x1a,
    #[doc = "Divide by 28."]
    Divide28 = 0x1b,
    #[doc = "Divide by 29."]
    Divide29 = 0x1c,
    #[doc = "Divide by 30."]
    Divide30 = 0x1d,
    #[doc = "Divide by 31."]
    Divide31 = 0x1e,
    #[doc = "Divide by 32."]
    Divide32 = 0x1f,
    #[doc = "Divide by 33."]
    Divide33 = 0x20,
    #[doc = "Divide by 34."]
    Divide34 = 0x21,
    #[doc = "Divide by 35."]
    Divide35 = 0x22,
    #[doc = "Divide by 36."]
    Divide36 = 0x23,
    #[doc = "Divide by 37."]
    Divide37 = 0x24,
    #[doc = "Divide by 38."]
    Divide38 = 0x25,
    #[doc = "Divide by 39."]
    Divide39 = 0x26,
    #[doc = "Divide by 40."]
    Divide40 = 0x27,
    #[doc = "Divide by 41."]
    Divide41 = 0x28,
    #[doc = "Divide by 42."]
    Divide42 = 0x29,
    #[doc = "Divide by 43."]
    Divide43 = 0x2a,
    #[doc = "Divide by 44."]
    Divide44 = 0x2b,
    #[doc = "Divide by 45."]
    Divide45 = 0x2c,
    #[doc = "Divide by 46."]
    Divide46 = 0x2d,
    #[doc = "Divide by 47."]
    Divide47 = 0x2e,
    #[doc = "Divide by 48."]
    Divide48 = 0x2f,
    #[doc = "Divide by 49."]
    Divide49 = 0x30,
    #[doc = "Divide by 50."]
    Divide50 = 0x31,
    #[doc = "Divide by 51."]
    Divide51 = 0x32,
    #[doc = "Divide by 52."]
    Divide52 = 0x33,
    #[doc = "Divide by 53."]
    Divide53 = 0x34,
    #[doc = "Divide by 54."]
    Divide54 = 0x35,
    #[doc = "Divide by 55."]
    Divide55 = 0x36,
    #[doc = "Divide by 56."]
    Divide56 = 0x37,
    #[doc = "Divide by 57."]
    Divide57 = 0x38,
    #[doc = "Divide by 58."]
    Divide58 = 0x39,
    #[doc = "Divide by 59."]
    Divide59 = 0x3a,
    #[doc = "Divide by 60."]
    Divide60 = 0x3b,
    #[doc = "Divide by 61."]
    Divide61 = 0x3c,
    #[doc = "Divide by 62."]
    Divide62 = 0x3d,
    #[doc = "Divide by 63."]
    Divide63 = 0x3e,
    #[doc = "Divide by 64."]
    Divide64 = 0x3f,
}
impl Sai1ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkPodf {
        Sai1ClkPodf::from_bits(val)
    }
}
impl From<Sai1ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkPodf) -> u8 {
        Sai1ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkPred {
    #[doc = "divide by 1."]
    Sai1ClkPred0 = 0x0,
    #[doc = "divide by 2."]
    Sai1ClkPred1 = 0x01,
    #[doc = "divide by 3."]
    Sai1ClkPred2 = 0x02,
    #[doc = "divide by 4."]
    Sai1ClkPred3 = 0x03,
    #[doc = "divide by 5."]
    Sai1ClkPred4 = 0x04,
    #[doc = "divide by 6."]
    Sai1ClkPred5 = 0x05,
    #[doc = "divide by 7."]
    Sai1ClkPred6 = 0x06,
    #[doc = "divide by 8."]
    Sai1ClkPred7 = 0x07,
}
impl Sai1ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkPred {
        Sai1ClkPred::from_bits(val)
    }
}
impl From<Sai1ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkPred) -> u8 {
        Sai1ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1ClkSel {
    #[doc = "derive clock from PLL3 PFD2."]
    Sai1ClkSel0 = 0x0,
    #[doc = "derive from pll3_sw_clk."]
    Sai1ClkSel1 = 0x01,
    #[doc = "derive clock from PLL4."]
    Sai1ClkSel2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai1ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1ClkSel {
        Sai1ClkSel::from_bits(val)
    }
}
impl From<Sai1ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1ClkSel) -> u8 {
        Sai1ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
    #[doc = "Divide by 17."]
    Divide17 = 0x10,
    #[doc = "Divide by 18."]
    Divide18 = 0x11,
    #[doc = "Divide by 19."]
    Divide19 = 0x12,
    #[doc = "Divide by 20."]
    Divide20 = 0x13,
    #[doc = "Divide by 21."]
    Divide21 = 0x14,
    #[doc = "Divide by 22."]
    Divide22 = 0x15,
    #[doc = "Divide by 23."]
    Divide23 = 0x16,
    #[doc = "Divide by 24."]
    Divide24 = 0x17,
    #[doc = "Divide by 25."]
    Divide25 = 0x18,
    #[doc = "Divide by 26."]
    Divide26 = 0x19,
    #[doc = "Divide by 27."]
    Divide27 = 0x1a,
    #[doc = "Divide by 28."]
    Divide28 = 0x1b,
    #[doc = "Divide by 29."]
    Divide29 = 0x1c,
    #[doc = "Divide by 30."]
    Divide30 = 0x1d,
    #[doc = "Divide by 31."]
    Divide31 = 0x1e,
    #[doc = "Divide by 32."]
    Divide32 = 0x1f,
    #[doc = "Divide by 33."]
    Divide33 = 0x20,
    #[doc = "Divide by 34."]
    Divide34 = 0x21,
    #[doc = "Divide by 35."]
    Divide35 = 0x22,
    #[doc = "Divide by 36."]
    Divide36 = 0x23,
    #[doc = "Divide by 37."]
    Divide37 = 0x24,
    #[doc = "Divide by 38."]
    Divide38 = 0x25,
    #[doc = "Divide by 39."]
    Divide39 = 0x26,
    #[doc = "Divide by 40."]
    Divide40 = 0x27,
    #[doc = "Divide by 41."]
    Divide41 = 0x28,
    #[doc = "Divide by 42."]
    Divide42 = 0x29,
    #[doc = "Divide by 43."]
    Divide43 = 0x2a,
    #[doc = "Divide by 44."]
    Divide44 = 0x2b,
    #[doc = "Divide by 45."]
    Divide45 = 0x2c,
    #[doc = "Divide by 46."]
    Divide46 = 0x2d,
    #[doc = "Divide by 47."]
    Divide47 = 0x2e,
    #[doc = "Divide by 48."]
    Divide48 = 0x2f,
    #[doc = "Divide by 49."]
    Divide49 = 0x30,
    #[doc = "Divide by 50."]
    Divide50 = 0x31,
    #[doc = "Divide by 51."]
    Divide51 = 0x32,
    #[doc = "Divide by 52."]
    Divide52 = 0x33,
    #[doc = "Divide by 53."]
    Divide53 = 0x34,
    #[doc = "Divide by 54."]
    Divide54 = 0x35,
    #[doc = "Divide by 55."]
    Divide55 = 0x36,
    #[doc = "Divide by 56."]
    Divide56 = 0x37,
    #[doc = "Divide by 57."]
    Divide57 = 0x38,
    #[doc = "Divide by 58."]
    Divide58 = 0x39,
    #[doc = "Divide by 59."]
    Divide59 = 0x3a,
    #[doc = "Divide by 60."]
    Divide60 = 0x3b,
    #[doc = "Divide by 61."]
    Divide61 = 0x3c,
    #[doc = "Divide by 62."]
    Divide62 = 0x3d,
    #[doc = "Divide by 63."]
    Divide63 = 0x3e,
    #[doc = "Divide by 64."]
    Divide64 = 0x3f,
}
impl Sai3ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkPodf {
        Sai3ClkPodf::from_bits(val)
    }
}
impl From<Sai3ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkPodf) -> u8 {
        Sai3ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkPred {
    #[doc = "divide by 1."]
    Sai3ClkPred0 = 0x0,
    #[doc = "divide by 2."]
    Sai3ClkPred1 = 0x01,
    #[doc = "divide by 3."]
    Sai3ClkPred2 = 0x02,
    #[doc = "divide by 4."]
    Sai3ClkPred3 = 0x03,
    #[doc = "divide by 5."]
    Sai3ClkPred4 = 0x04,
    #[doc = "divide by 6."]
    Sai3ClkPred5 = 0x05,
    #[doc = "divide by 7."]
    Sai3ClkPred6 = 0x06,
    #[doc = "divide by 8."]
    Sai3ClkPred7 = 0x07,
}
impl Sai3ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkPred {
        Sai3ClkPred::from_bits(val)
    }
}
impl From<Sai3ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkPred) -> u8 {
        Sai3ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai3ClkSel {
    #[doc = "derive clock from PLL3 PFD2."]
    Sai3ClkSel0 = 0x0,
    #[doc = "derive from pll3_sw_clk."]
    Sai3ClkSel1 = 0x01,
    #[doc = "derive clock from PLL4."]
    Sai3ClkSel2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sai3ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai3ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai3ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Sai3ClkSel {
        Sai3ClkSel::from_bits(val)
    }
}
impl From<Sai3ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Sai3ClkSel) -> u8 {
        Sai3ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbyos {
    #[doc = "On-chip oscillator will not be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will remain asserted - '0' and cosc_pwrdown will remain de asserted - '0')."]
    Sbyos0 = 0x0,
    #[doc = "On-chip oscillator will be powered down, after next entrance to STOP mode. (CCM_REF_EN_B will be deasserted - '1' and cosc_pwrdown will be asserted - '1'). When returning from STOP mode, external oscillator will be enabled again, on-chip oscillator will return to oscillator mode, and after oscnt count, CCM will continue with the exit from the STOP mode process."]
    Sbyos1 = 0x01,
}
impl Sbyos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbyos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbyos {
    #[inline(always)]
    fn from(val: u8) -> Sbyos {
        Sbyos::from_bits(val)
    }
}
impl From<Sbyos> for u8 {
    #[inline(always)]
    fn from(val: Sbyos) -> u8 {
        Sbyos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
}
impl Spdif0ClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkPodf {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkPodf {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkPodf {
        Spdif0ClkPodf::from_bits(val)
    }
}
impl From<Spdif0ClkPodf> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkPodf) -> u8 {
        Spdif0ClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkPred {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
}
impl Spdif0ClkPred {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkPred {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkPred {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkPred {
        Spdif0ClkPred::from_bits(val)
    }
}
impl From<Spdif0ClkPred> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkPred) -> u8 {
        Spdif0ClkPred::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Spdif0ClkSel {
    #[doc = "derive clock from PLL4."]
    Spdif0ClkSel0 = 0x0,
    #[doc = "derive clock from PLL3 PFD2."]
    Spdif0ClkSel1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "derive clock from pll3_sw_clk."]
    Spdif0ClkSel3 = 0x03,
}
impl Spdif0ClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Spdif0ClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Spdif0ClkSel {
    #[inline(always)]
    fn from(val: u8) -> Spdif0ClkSel {
        Spdif0ClkSel::from_bits(val)
    }
}
impl From<Spdif0ClkSel> for u8 {
    #[inline(always)]
    fn from(val: Spdif0ClkSel) -> u8 {
        Spdif0ClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StbyCount {
    #[doc = "CCM will wait (1*pmic_delay_scaler)+1 ckil clock cycles."]
    StbyCount0 = 0x0,
    #[doc = "CCM will wait (3*pmic_delay_scaler)+1 ckil clock cycles."]
    StbyCount1 = 0x01,
    #[doc = "CCM will wait (7*pmic_delay_scaler)+1 ckil clock cycles."]
    StbyCount2 = 0x02,
    #[doc = "CCM will wait (15*pmic_delay_scaler)+1 ckil clock cycles."]
    StbyCount3 = 0x03,
}
impl StbyCount {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StbyCount {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StbyCount {
    #[inline(always)]
    fn from(val: u8) -> StbyCount {
        StbyCount::from_bits(val)
    }
}
impl From<StbyCount> for u8 {
    #[inline(always)]
    fn from(val: StbyCount) -> u8 {
        StbyCount::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SysMemDsCtrl {
    #[doc = "Disable memory DS mode always."]
    SysMemDsCtrl0 = 0x0,
    #[doc = "Enable memory (outside Arm platform) DS mode when system STOP and PLL are disabled."]
    SysMemDsCtrl1 = 0x01,
    #[doc = "enable memory (outside Arm platform) DS mode when system is in STOP mode."]
    SysMemDsCtrl2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SysMemDsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysMemDsCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysMemDsCtrl {
    #[inline(always)]
    fn from(val: u8) -> SysMemDsCtrl {
        SysMemDsCtrl::from_bits(val)
    }
}
impl From<SysMemDsCtrl> for u8 {
    #[inline(always)]
    fn from(val: SysMemDsCtrl) -> u8 {
        SysMemDsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceClkSel {
    #[doc = "derive clock from PLL2."]
    TraceClkSel0 = 0x0,
    #[doc = "derive clock from PLL2 PFD2."]
    TraceClkSel1 = 0x01,
    #[doc = "derive clock from PLL2 PFD0."]
    TraceClkSel2 = 0x02,
    #[doc = "derive clock from PLL2 PFD1."]
    TraceClkSel3 = 0x03,
}
impl TraceClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceClkSel {
    #[inline(always)]
    fn from(val: u8) -> TraceClkSel {
        TraceClkSel::from_bits(val)
    }
}
impl From<TraceClkSel> for u8 {
    #[inline(always)]
    fn from(val: TraceClkSel) -> u8 {
        TraceClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TracePodf {
    #[doc = "divide by 1."]
    TracePodf0 = 0x0,
    #[doc = "divide by 2."]
    TracePodf1 = 0x01,
    #[doc = "divide by 3."]
    TracePodf2 = 0x02,
    #[doc = "divide by 4."]
    TracePodf3 = 0x03,
    #[doc = "divide by 5."]
    TracePodf4 = 0x04,
    #[doc = "divide by 6."]
    TracePodf5 = 0x05,
    #[doc = "divide by 7."]
    TracePodf6 = 0x06,
    #[doc = "divide by 8."]
    TracePodf7 = 0x07,
    #[doc = "divide by 9."]
    TracePodf8 = 0x08,
    #[doc = "divide by 10."]
    TracePodf9 = 0x09,
    #[doc = "divide by 11."]
    TracePodf10 = 0x0a,
    #[doc = "divide by 12."]
    TracePodf11 = 0x0b,
    #[doc = "divide by 13."]
    TracePodf12 = 0x0c,
    #[doc = "divide by 14."]
    TracePodf13 = 0x0d,
    #[doc = "divide by 15."]
    TracePodf14 = 0x0e,
    #[doc = "divide by 16."]
    TracePodf15 = 0x0f,
}
impl TracePodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TracePodf {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TracePodf {
    #[inline(always)]
    fn from(val: u8) -> TracePodf {
        TracePodf::from_bits(val)
    }
}
impl From<TracePodf> for u8 {
    #[inline(always)]
    fn from(val: TracePodf) -> u8 {
        TracePodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClkPodf {
    #[doc = "Divide by 1."]
    Divide1 = 0x0,
    #[doc = "Divide by 2."]
    Divide2 = 0x01,
    #[doc = "Divide by 3."]
    Divide3 = 0x02,
    #[doc = "Divide by 4."]
    Divide4 = 0x03,
    #[doc = "Divide by 5."]
    Divide5 = 0x04,
    #[doc = "Divide by 6."]
    Divide6 = 0x05,
    #[doc = "Divide by 7."]
    Divide7 = 0x06,
    #[doc = "Divide by 8."]
    Divide8 = 0x07,
    #[doc = "Divide by 9."]
    Divide9 = 0x08,
    #[doc = "Divide by 10."]
    Divide10 = 0x09,
    #[doc = "Divide by 11."]
    Divide11 = 0x0a,
    #[doc = "Divide by 12."]
    Divide12 = 0x0b,
    #[doc = "Divide by 13."]
    Divide13 = 0x0c,
    #[doc = "Divide by 14."]
    Divide14 = 0x0d,
    #[doc = "Divide by 15."]
    Divide15 = 0x0e,
    #[doc = "Divide by 16."]
    Divide16 = 0x0f,
    #[doc = "Divide by 17."]
    Divide17 = 0x10,
    #[doc = "Divide by 18."]
    Divide18 = 0x11,
    #[doc = "Divide by 19."]
    Divide19 = 0x12,
    #[doc = "Divide by 20."]
    Divide20 = 0x13,
    #[doc = "Divide by 21."]
    Divide21 = 0x14,
    #[doc = "Divide by 22."]
    Divide22 = 0x15,
    #[doc = "Divide by 23."]
    Divide23 = 0x16,
    #[doc = "Divide by 24."]
    Divide24 = 0x17,
    #[doc = "Divide by 25."]
    Divide25 = 0x18,
    #[doc = "Divide by 26."]
    Divide26 = 0x19,
    #[doc = "Divide by 27."]
    Divide27 = 0x1a,
    #[doc = "Divide by 28."]
    Divide28 = 0x1b,
    #[doc = "Divide by 29."]
    Divide29 = 0x1c,
    #[doc = "Divide by 30."]
    Divide30 = 0x1d,
    #[doc = "Divide by 31."]
    Divide31 = 0x1e,
    #[doc = "Divide by 32."]
    Divide32 = 0x1f,
    #[doc = "Divide by 33."]
    Divide33 = 0x20,
    #[doc = "Divide by 34."]
    Divide34 = 0x21,
    #[doc = "Divide by 35."]
    Divide35 = 0x22,
    #[doc = "Divide by 36."]
    Divide36 = 0x23,
    #[doc = "Divide by 37."]
    Divide37 = 0x24,
    #[doc = "Divide by 38."]
    Divide38 = 0x25,
    #[doc = "Divide by 39."]
    Divide39 = 0x26,
    #[doc = "Divide by 40."]
    Divide40 = 0x27,
    #[doc = "Divide by 41."]
    Divide41 = 0x28,
    #[doc = "Divide by 42."]
    Divide42 = 0x29,
    #[doc = "Divide by 43."]
    Divide43 = 0x2a,
    #[doc = "Divide by 44."]
    Divide44 = 0x2b,
    #[doc = "Divide by 45."]
    Divide45 = 0x2c,
    #[doc = "Divide by 46."]
    Divide46 = 0x2d,
    #[doc = "Divide by 47."]
    Divide47 = 0x2e,
    #[doc = "Divide by 48."]
    Divide48 = 0x2f,
    #[doc = "Divide by 49."]
    Divide49 = 0x30,
    #[doc = "Divide by 50."]
    Divide50 = 0x31,
    #[doc = "Divide by 51."]
    Divide51 = 0x32,
    #[doc = "Divide by 52."]
    Divide52 = 0x33,
    #[doc = "Divide by 53."]
    Divide53 = 0x34,
    #[doc = "Divide by 54."]
    Divide54 = 0x35,
    #[doc = "Divide by 55."]
    Divide55 = 0x36,
    #[doc = "Divide by 56."]
    Divide56 = 0x37,
    #[doc = "Divide by 57."]
    Divide57 = 0x38,
    #[doc = "Divide by 58."]
    Divide58 = 0x39,
    #[doc = "Divide by 59."]
    Divide59 = 0x3a,
    #[doc = "Divide by 60."]
    Divide60 = 0x3b,
    #[doc = "Divide by 61."]
    Divide61 = 0x3c,
    #[doc = "Divide by 62."]
    Divide62 = 0x3d,
    #[doc = "Divide by 63."]
    Divide63 = 0x3e,
    #[doc = "Divide by 64."]
    Divide64 = 0x3f,
}
impl UartClkPodf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkPodf {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkPodf {
    #[inline(always)]
    fn from(val: u8) -> UartClkPodf {
        UartClkPodf::from_bits(val)
    }
}
impl From<UartClkPodf> for u8 {
    #[inline(always)]
    fn from(val: UartClkPodf) -> u8 {
        UartClkPodf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UartClkSel {
    #[doc = "derive clock from pll3_80m."]
    UartClkSel0 = 0x0,
    #[doc = "derive clock from osc_clk."]
    UartClkSel1 = 0x01,
    #[doc = "derive clock from per_clk_root."]
    UartClkSel2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl UartClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UartClkSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UartClkSel {
    #[inline(always)]
    fn from(val: u8) -> UartClkSel {
        UartClkSel::from_bits(val)
    }
}
impl From<UartClkSel> for u8 {
    #[inline(always)]
    fn from(val: UartClkSel) -> u8 {
        UartClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vstby {
    #[doc = "Voltage will not be changed to standby voltage after next entrance to STOP mode. ( PMIC_STBY_REQ will remain negated - '0')."]
    Vstby0 = 0x0,
    #[doc = "Voltage will be requested to change to standby voltage after next entrance to stop mode. ( PMIC_STBY_REQ will be asserted - '1')."]
    Vstby1 = 0x01,
}
impl Vstby {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vstby {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vstby {
    #[inline(always)]
    fn from(val: u8) -> Vstby {
        Vstby::from_bits(val)
    }
}
impl From<Vstby> for u8 {
    #[inline(always)]
    fn from(val: Vstby) -> u8 {
        Vstby::to_bits(val)
    }
}
