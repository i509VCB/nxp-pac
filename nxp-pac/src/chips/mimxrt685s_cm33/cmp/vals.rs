#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acph1tc {
    #[doc = "Phase1 active time in one sampling period equals to T"]
    ACPH1TC_0 = 0x0,
    #[doc = "Phase1 active time in one sampling period equals to 2*T"]
    ACPH1TC_1 = 0x01,
    #[doc = "Phase1 active time in one sampling period equals to 4*T"]
    ACPH1TC_2 = 0x02,
    #[doc = "Phase1 active time in one sampling period equals to 8*T"]
    ACPH1TC_3 = 0x03,
    #[doc = "Phase1 active time in one sampling period equals to T"]
    ACPH1TC_4 = 0x04,
    #[doc = "Phase1 active time in one sampling period equals to T"]
    ACPH1TC_5 = 0x05,
    #[doc = "Phase1 active time in one sampling period equals to T"]
    ACPH1TC_6 = 0x06,
    #[doc = "Phase1 active time in one sampling period equals to 0"]
    ACPH1TC_7 = 0x07,
}
impl Acph1tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acph1tc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acph1tc {
    #[inline(always)]
    fn from(val: u8) -> Acph1tc {
        Acph1tc::from_bits(val)
    }
}
impl From<Acph1tc> for u8 {
    #[inline(always)]
    fn from(val: Acph1tc) -> u8 {
        Acph1tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acph2tc {
    #[doc = "Phase2 active time in one sampling period equals to T"]
    ACPH2TC_0 = 0x0,
    #[doc = "Phase2 active time in one sampling period equals to 2*T"]
    ACPH2TC_1 = 0x01,
    #[doc = "Phase2 active time in one sampling period equals to 4*T"]
    ACPH2TC_2 = 0x02,
    #[doc = "Phase2 active time in one sampling period equals to 8*T"]
    ACPH2TC_3 = 0x03,
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    ACPH2TC_4 = 0x04,
    #[doc = "Phase2 active time in one sampling period equals to 32*T"]
    ACPH2TC_5 = 0x05,
    #[doc = "Phase2 active time in one sampling period equals to 64*T"]
    ACPH2TC_6 = 0x06,
    #[doc = "Phase2 active time in one sampling period equals to 16*T"]
    ACPH2TC_7 = 0x07,
}
impl Acph2tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acph2tc {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acph2tc {
    #[inline(always)]
    fn from(val: u8) -> Acph2tc {
        Acph2tc::from_bits(val)
    }
}
impl From<Acph2tc> for u8 {
    #[inline(always)]
    fn from(val: Acph2tc) -> u8 {
        Acph2tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acsat {
    #[doc = "The sampling time equals to T"]
    ACSAT_0 = 0x0,
    #[doc = "The sampling time equasl to 2*T"]
    ACSAT_1 = 0x01,
    #[doc = "The sampling time equasl to 4*T"]
    ACSAT_2 = 0x02,
    #[doc = "The sampling time equasl to 8*T"]
    ACSAT_3 = 0x03,
    #[doc = "The sampling time equasl to 16*T"]
    ACSAT_4 = 0x04,
    #[doc = "The sampling time equasl to 32*T"]
    ACSAT_5 = 0x05,
    #[doc = "The sampling time equasl to 64*T"]
    ACSAT_6 = 0x06,
    #[doc = "The sampling time equasl to 256*T"]
    ACSAT_7 = 0x07,
}
impl Acsat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acsat {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acsat {
    #[inline(always)]
    fn from(val: u8) -> Acsat {
        Acsat::from_bits(val)
    }
}
impl From<Acsat> for u8 {
    #[inline(always)]
    fn from(val: Acsat) -> u8 {
        Acsat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cff {
    #[doc = "A falling edge has not been detected on COUT."]
    CFF_0 = 0x0,
    #[doc = "A falling edge on COUT has occurred."]
    CFF_1 = 0x01,
}
impl Cff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cff {
    #[inline(always)]
    fn from(val: u8) -> Cff {
        Cff::from_bits(val)
    }
}
impl From<Cff> for u8 {
    #[inline(always)]
    fn from(val: Cff) -> u8 {
        Cff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cfr {
    #[doc = "A rising edge has not been detected on COUT."]
    CFR_0 = 0x0,
    #[doc = "A rising edge on COUT has occurred."]
    CFR_1 = 0x01,
}
impl Cfr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cfr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cfr {
    #[inline(always)]
    fn from(val: u8) -> Cfr {
        Cfr::from_bits(val)
    }
}
impl From<Cfr> for u8 {
    #[inline(always)]
    fn from(val: Cfr) -> u8 {
        Cfr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cos {
    #[doc = "Set CMPO to equal COUT (filtered comparator output)."]
    COS_0 = 0x0,
    #[doc = "Set CMPO to equal COUTA (unfiltered comparator output)."]
    COS_1 = 0x01,
}
impl Cos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cos {
    #[inline(always)]
    fn from(val: u8) -> Cos {
        Cos::from_bits(val)
    }
}
impl From<Cos> for u8 {
    #[inline(always)]
    fn from(val: Cos) -> u8 {
        Cos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dacen {
    #[doc = "DAC is disabled."]
    DACEN_0 = 0x0,
    #[doc = "DAC is enabled."]
    DACEN_1 = 0x01,
}
impl Dacen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dacen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dacen {
    #[inline(always)]
    fn from(val: u8) -> Dacen {
        Dacen::from_bits(val)
    }
}
impl From<Dacen> for u8 {
    #[inline(always)]
    fn from(val: Dacen) -> u8 {
        Dacen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "DMA is disabled."]
    DMAEN_0 = 0x0,
    #[doc = "DMA is enabled."]
    DMAEN_1 = 0x01,
}
impl Dmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaen {
    #[inline(always)]
    fn from(val: u8) -> Dmaen {
        Dmaen::from_bits(val)
    }
}
impl From<Dmaen> for u8 {
    #[inline(always)]
    fn from(val: Dmaen) -> u8 {
        Dmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmcs {
    #[doc = "Slow clock is selected for the timing generation."]
    DMCS_0 = 0x0,
    #[doc = "Fast clock is selected for the timing generation."]
    DMCS_1 = 0x01,
}
impl Dmcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmcs {
    #[inline(always)]
    fn from(val: u8) -> Dmcs {
        Dmcs::from_bits(val)
    }
}
impl From<Dmcs> for u8 {
    #[inline(always)]
    fn from(val: Dmcs) -> u8 {
        Dmcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmode {
    #[doc = "DAC is selected to work in low speed and low power mode."]
    DMODE_0 = 0x0,
    #[doc = "DAC is selected to work in high speed high power mode."]
    DMODE_1 = 0x01,
}
impl Dmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmode {
    #[inline(always)]
    fn from(val: u8) -> Dmode {
        Dmode::from_bits(val)
    }
}
impl From<Dmode> for u8 {
    #[inline(always)]
    fn from(val: Dmode) -> u8 {
        Dmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Analog Comparator is disabled."]
    EN_0 = 0x0,
    #[doc = "Analog Comparator is enabled."]
    EN_1 = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilterCnt {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic zero (this is not a legal state, and is not recommended). If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0x0,
    #[doc = "1 consecutive sample must agree (comparator output is simply sampled)."]
    FILTER_CNT_1 = 0x01,
    #[doc = "2 consecutive samples must agree."]
    FILTER_CNT_2 = 0x02,
    #[doc = "3 consecutive samples must agree."]
    FILTER_CNT_3 = 0x03,
    #[doc = "4 consecutive samples must agree."]
    FILTER_CNT_4 = 0x04,
    #[doc = "5 consecutive samples must agree."]
    FILTER_CNT_5 = 0x05,
    #[doc = "6 consecutive samples must agree."]
    FILTER_CNT_6 = 0x06,
    #[doc = "7 consecutive samples must agree."]
    FILTER_CNT_7 = 0x07,
}
impl FilterCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilterCnt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilterCnt {
    #[inline(always)]
    fn from(val: u8) -> FilterCnt {
        FilterCnt::from_bits(val)
    }
}
impl From<FilterCnt> for u8 {
    #[inline(always)]
    fn from(val: FilterCnt) -> u8 {
        FilterCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxmp {
    #[doc = "The Plus port is fixed. Only the inputs to the Minus port are swept in each round."]
    FXMP_0 = 0x0,
    #[doc = "The Minus port is fixed. Only the inputs to the Plus port are swept in each round."]
    FXMP_1 = 0x01,
}
impl Fxmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxmp {
    #[inline(always)]
    fn from(val: u8) -> Fxmp {
        Fxmp::from_bits(val)
    }
}
impl From<Fxmp> for u8 {
    #[inline(always)]
    fn from(val: Fxmp) -> u8 {
        Fxmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fxmxch {
    #[doc = "External Reference Input 0 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_0 = 0x0,
    #[doc = "External Reference Input 1 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_1 = 0x01,
    #[doc = "External Reference Input 2 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_2 = 0x02,
    #[doc = "External Reference Input 3 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_3 = 0x03,
    #[doc = "External Reference Input 4 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_4 = 0x04,
    #[doc = "External Reference Input 5 is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "The 8bit DAC is selected as the fixed reference input for the fixed mux port."]
    FXMXCH_7 = 0x07,
}
impl Fxmxch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fxmxch {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fxmxch {
    #[inline(always)]
    fn from(val: u8) -> Fxmxch {
        Fxmxch::from_bits(val)
    }
}
impl From<Fxmxch> for u8 {
    #[inline(always)]
    fn from(val: Fxmxch) -> u8 {
        Fxmxch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hystctr {
    #[doc = "The hard block output has level 0 hysteresis internally."]
    HYSTCTR_0 = 0x0,
    #[doc = "The hard block output has level 1 hysteresis internally."]
    HYSTCTR_1 = 0x01,
    #[doc = "The hard block output has level 2 hysteresis internally."]
    HYSTCTR_2 = 0x02,
    #[doc = "The hard block output has level 3 hysteresis internally."]
    HYSTCTR_3 = 0x03,
}
impl Hystctr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hystctr {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hystctr {
    #[inline(always)]
    fn from(val: u8) -> Hystctr {
        Hystctr::from_bits(val)
    }
}
impl From<Hystctr> for u8 {
    #[inline(always)]
    fn from(val: Hystctr) -> u8 {
        Hystctr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ief {
    #[doc = "Interrupt is disabled."]
    IEF_0 = 0x0,
    #[doc = "Interrupt is enabled."]
    IEF_1 = 0x01,
}
impl Ief {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ief {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ief {
    #[inline(always)]
    fn from(val: u8) -> Ief {
        Ief::from_bits(val)
    }
}
impl From<Ief> for u8 {
    #[inline(always)]
    fn from(val: Ief) -> u8 {
        Ief::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ier {
    #[doc = "Interrupt is disabled."]
    IER_0 = 0x0,
    #[doc = "Interrupt is enabled."]
    IER_1 = 0x01,
}
impl Ier {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ier {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ier {
    #[inline(always)]
    fn from(val: u8) -> Ier {
        Ier::from_bits(val)
    }
}
impl From<Ier> for u8 {
    #[inline(always)]
    fn from(val: Ier) -> u8 {
        Ier::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Invt {
    #[doc = "Does not invert the comparator output."]
    INVT_0 = 0x0,
    #[doc = "Inverts the comparator output."]
    INVT_1 = 0x01,
}
impl Invt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Invt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Invt {
    #[inline(always)]
    fn from(val: u8) -> Invt {
        Invt::from_bits(val)
    }
}
impl From<Invt> for u8 {
    #[inline(always)]
    fn from(val: Invt) -> u8 {
        Invt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Linken {
    #[doc = "CMP to DAC link is disabled"]
    LINKEN_0 = 0x0,
    #[doc = "CMP to DAC link is enabled."]
    LINKEN_1 = 0x01,
}
impl Linken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Linken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Linken {
    #[inline(always)]
    fn from(val: u8) -> Linken {
        Linken::from_bits(val)
    }
}
impl From<Linken> for u8 {
    #[inline(always)]
    fn from(val: Linken) -> u8 {
        Linken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Msel {
    #[doc = "Internal Negative Input 0 for Minus Channel -- Internal Minus Input"]
    MSEL_0 = 0x0,
    #[doc = "External Input 1 for Minus Channel -- Reference Input 0"]
    MSEL_1 = 0x01,
    #[doc = "External Input 2 for Minus Channel -- Reference Input 1"]
    MSEL_2 = 0x02,
    #[doc = "External Input 3 for Minus Channel -- Reference Input 2"]
    MSEL_3 = 0x03,
    #[doc = "External Input 4 for Minus Channel -- Reference Input 3"]
    MSEL_4 = 0x04,
    #[doc = "External Input 5 for Minus Channel -- Reference Input 4"]
    MSEL_5 = 0x05,
    #[doc = "External Input 6 for Minus Channel -- Reference Input 5"]
    MSEL_6 = 0x06,
    #[doc = "Internal 8b DAC output"]
    MSEL_7 = 0x07,
}
impl Msel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Msel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Msel {
    #[inline(always)]
    fn from(val: u8) -> Msel {
        Msel::from_bits(val)
    }
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(val: Msel) -> u8 {
        Msel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nchcten {
    #[doc = "Negative channel is in Discrete Mode and special timing needs to be configured."]
    NCHCTEN_0 = 0x0,
    #[doc = "Negative channel is in Continuous Mode and no special timing is requried."]
    NCHCTEN_1 = 0x01,
}
impl Nchcten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nchcten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nchcten {
    #[inline(always)]
    fn from(val: u8) -> Nchcten {
        Nchcten::from_bits(val)
    }
}
impl From<Nchcten> for u8 {
    #[inline(always)]
    fn from(val: Nchcten) -> u8 {
        Nchcten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nsam {
    #[doc = "The comparison result is sampled as soon as the active channel is scanned in one round-robin clock."]
    NSAM_0 = 0x0,
    #[doc = "The sampling takes place 1 round-robin clock cycle after the next cycle of the round-robin clock."]
    NSAM_1 = 0x01,
    #[doc = "The sampling takes place 2 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_2 = 0x02,
    #[doc = "The sampling takes place 3 round-robin clock cycles after the next cycle of the round-robin clock."]
    NSAM_3 = 0x03,
}
impl Nsam {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nsam {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nsam {
    #[inline(always)]
    fn from(val: u8) -> Nsam {
        Nsam::from_bits(val)
    }
}
impl From<Nsam> for u8 {
    #[inline(always)]
    fn from(val: Nsam) -> u8 {
        Nsam::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ope {
    #[doc = "When OPE is 0, the comparator output (after window/filter settings dependent on software configuration) is not available to a packaged pin."]
    OPE_0 = 0x0,
    #[doc = "When OPE is 1, and if the software has configured the comparator to own a packaged pin, the comparator is available in a packaged pin."]
    OPE_1 = 0x01,
}
impl Ope {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ope {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ope {
    #[inline(always)]
    fn from(val: u8) -> Ope {
        Ope::from_bits(val)
    }
}
impl From<Ope> for u8 {
    #[inline(always)]
    fn from(val: Ope) -> u8 {
        Ope::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pchcten {
    #[doc = "Positive channel is in Discrete Mode and special timing needs to be configured."]
    PCHCTEN_0 = 0x0,
    #[doc = "Positive channel is in Continuous Mode and no special timing is requried."]
    PCHCTEN_1 = 0x01,
}
impl Pchcten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pchcten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pchcten {
    #[inline(always)]
    fn from(val: u8) -> Pchcten {
        Pchcten::from_bits(val)
    }
}
impl From<Pchcten> for u8 {
    #[inline(always)]
    fn from(val: Pchcten) -> u8 {
        Pchcten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmode {
    #[doc = "Low Speed (LS) comparison mode is selected."]
    PMODE_0 = 0x0,
    #[doc = "High Speed (HS) comparison mode is selected."]
    PMODE_1 = 0x01,
}
impl Pmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmode {
    #[inline(always)]
    fn from(val: u8) -> Pmode {
        Pmode::from_bits(val)
    }
}
impl From<Pmode> for u8 {
    #[inline(always)]
    fn from(val: Pmode) -> u8 {
        Pmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psel {
    #[doc = "Internal Posivite Input 0 for Plus Channel -- Internal Minus Input"]
    PSEL_0 = 0x0,
    #[doc = "External Input 1 for Plus Channel -- Reference Input 0"]
    PSEL_1 = 0x01,
    #[doc = "External Input 2 for Plus Channel -- Reference Input 1"]
    PSEL_2 = 0x02,
    #[doc = "External Input 3 for Plus Channel -- Reference Input 2"]
    PSEL_3 = 0x03,
    #[doc = "External Input 4 for Plus Channel -- Reference Input 3"]
    PSEL_4 = 0x04,
    #[doc = "External Input 4 for Plus Channel -- Reference Input 4"]
    PSEL_5 = 0x05,
    #[doc = "External Input 4 for Plus Channel -- Reference Input 5"]
    PSEL_6 = 0x06,
    #[doc = "Internal 8b DAC output"]
    PSEL_7 = 0x07,
}
impl Psel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psel {
    #[inline(always)]
    fn from(val: u8) -> Psel {
        Psel::from_bits(val)
    }
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(val: Psel) -> u8 {
        Psel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rdive {
    #[doc = "The resistor is not enabled even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 - 1.8v."]
    RDIVE_0 = 0x0,
    #[doc = "The resistor is enabled because the inputs are above 1.8v."]
    RDIVE_1 = 0x01,
}
impl Rdive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rdive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rdive {
    #[inline(always)]
    fn from(val: u8) -> Rdive {
        Rdive::from_bits(val)
    }
}
impl From<Rdive> for u8 {
    #[inline(always)]
    fn from(val: Rdive) -> u8 {
        Rdive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rrie {
    #[doc = "The round-robin interrupt is disabled."]
    RRIE_0 = 0x0,
    #[doc = "The round-robin interrupt is enabled when a comparison result changes from the last sample."]
    RRIE_1 = 0x01,
}
impl Rrie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rrie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rrie {
    #[inline(always)]
    fn from(val: u8) -> Rrie {
        Rrie::from_bits(val)
    }
}
impl From<Rrie> for u8 {
    #[inline(always)]
    fn from(val: Rrie) -> u8 {
        Rrie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Se {
    #[doc = "Sampling mode is not selected."]
    SE_0 = 0x0,
    #[doc = "Sampling mode is selected."]
    SE_1 = 0x01,
}
impl Se {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Se {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Se {
    #[inline(always)]
    fn from(val: u8) -> Se {
        Se::from_bits(val)
    }
}
impl From<Se> for u8 {
    #[inline(always)]
    fn from(val: Se) -> u8 {
        Se::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrsel {
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin. Vin1 is from internal PMC."]
    VRSEL_0 = 0x0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin. Vin2 is from PAD."]
    VRSEL_1 = 0x01,
}
impl Vrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrsel {
    #[inline(always)]
    fn from(val: u8) -> Vrsel {
        Vrsel::from_bits(val)
    }
}
impl From<Vrsel> for u8 {
    #[inline(always)]
    fn from(val: Vrsel) -> u8 {
        Vrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum We {
    #[doc = "Windowing mode is not selected."]
    WE_0 = 0x0,
    #[doc = "Windowing mode is selected."]
    WE_1 = 0x01,
}
impl We {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> We {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for We {
    #[inline(always)]
    fn from(val: u8) -> We {
        We::from_bits(val)
    }
}
impl From<We> for u8 {
    #[inline(always)]
    fn from(val: We) -> u8 {
        We::to_bits(val)
    }
}
