#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acfe {
    #[doc = "Compare function disabled"]
    ACFE_0 = 0x0,
    #[doc = "Compare function enabled"]
    ACFE_1 = 0x01,
}
impl Acfe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acfe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acfe {
    #[inline(always)]
    fn from(val: u8) -> Acfe {
        Acfe::from_bits(val)
    }
}
impl From<Acfe> for u8 {
    #[inline(always)]
    fn from(val: Acfe) -> u8 {
        Acfe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acfgt {
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    ACFGT_0 = 0x0,
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    ACFGT_1 = 0x01,
}
impl Acfgt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acfgt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acfgt {
    #[inline(always)]
    fn from(val: u8) -> Acfgt {
        Acfgt::from_bits(val)
    }
}
impl From<Acfgt> for u8 {
    #[inline(always)]
    fn from(val: Acfgt) -> u8 {
        Acfgt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acren {
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    ACREN_0 = 0x0,
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    ACREN_1 = 0x01,
}
impl Acren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acren {
    #[inline(always)]
    fn from(val: u8) -> Acren {
        Acren::from_bits(val)
    }
}
impl From<Acren> for u8 {
    #[inline(always)]
    fn from(val: Acren) -> u8 {
        Acren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adacken {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    ADACKEN_0 = 0x0,
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    ADACKEN_1 = 0x01,
}
impl Adacken {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adacken {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adacken {
    #[inline(always)]
    fn from(val: u8) -> Adacken {
        Adacken::from_bits(val)
    }
}
impl From<Adacken> for u8 {
    #[inline(always)]
    fn from(val: Adacken) -> u8 {
        Adacken::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adact {
    #[doc = "Conversion not in progress."]
    ADACT_0 = 0x0,
    #[doc = "Conversion in progress."]
    ADACT_1 = 0x01,
}
impl Adact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adact {
    #[inline(always)]
    fn from(val: u8) -> Adact {
        Adact::from_bits(val)
    }
}
impl From<Adact> for u8 {
    #[inline(always)]
    fn from(val: Adact) -> u8 {
        Adact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adco {
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_0 = 0x0,
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_1 = 0x01,
}
impl Adco {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adco {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adco {
    #[inline(always)]
    fn from(val: u8) -> Adco {
        Adco::from_bits(val)
    }
}
impl From<Adco> for u8 {
    #[inline(always)]
    fn from(val: Adco) -> u8 {
        Adco::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adhsc {
    #[doc = "Normal conversion selected."]
    ADHSC_0 = 0x0,
    #[doc = "High speed conversion selected."]
    ADHSC_1 = 0x01,
}
impl Adhsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adhsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adhsc {
    #[inline(always)]
    fn from(val: u8) -> Adhsc {
        Adhsc::from_bits(val)
    }
}
impl From<Adhsc> for u8 {
    #[inline(always)]
    fn from(val: Adhsc) -> u8 {
        Adhsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adiclk {
    #[doc = "IPG clock"]
    ADICLK_0 = 0x0,
    #[doc = "IPG clock divided by 2"]
    ADICLK_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Asynchronous clock (ADACK)"]
    ADICLK_3 = 0x03,
}
impl Adiclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adiclk {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adiclk {
    #[inline(always)]
    fn from(val: u8) -> Adiclk {
        Adiclk::from_bits(val)
    }
}
impl From<Adiclk> for u8 {
    #[inline(always)]
    fn from(val: Adiclk) -> u8 {
        Adiclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adiv {
    #[doc = "Input clock"]
    ADIV_0 = 0x0,
    #[doc = "Input clock / 2"]
    ADIV_1 = 0x01,
    #[doc = "Input clock / 4"]
    ADIV_2 = 0x02,
    #[doc = "Input clock / 8"]
    ADIV_3 = 0x03,
}
impl Adiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adiv {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adiv {
    #[inline(always)]
    fn from(val: u8) -> Adiv {
        Adiv::from_bits(val)
    }
}
impl From<Adiv> for u8 {
    #[inline(always)]
    fn from(val: Adiv) -> u8 {
        Adiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adlpc {
    #[doc = "ADC hard block not in low power mode."]
    ADLPC_0 = 0x0,
    #[doc = "ADC hard block in low power mode."]
    ADLPC_1 = 0x01,
}
impl Adlpc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adlpc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adlpc {
    #[inline(always)]
    fn from(val: u8) -> Adlpc {
        Adlpc::from_bits(val)
    }
}
impl From<Adlpc> for u8 {
    #[inline(always)]
    fn from(val: Adlpc) -> u8 {
        Adlpc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adlsmp {
    #[doc = "Short sample mode."]
    ADLSMP_0 = 0x0,
    #[doc = "Long sample mode."]
    ADLSMP_1 = 0x01,
}
impl Adlsmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adlsmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adlsmp {
    #[inline(always)]
    fn from(val: u8) -> Adlsmp {
        Adlsmp::from_bits(val)
    }
}
impl From<Adlsmp> for u8 {
    #[inline(always)]
    fn from(val: Adlsmp) -> u8 {
        Adlsmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adsts {
    #[doc = "Sample period (ADC clocks) = 3 if ADLSMP=0b Sample period (ADC clocks) = 13 if ADLSMP=1b"]
    ADSTS_0 = 0x0,
    #[doc = "Sample period (ADC clocks) = 5 if ADLSMP=0b Sample period (ADC clocks) = 17 if ADLSMP=1b"]
    ADSTS_1 = 0x01,
    #[doc = "Sample period (ADC clocks) = 7 if ADLSMP=0b Sample period (ADC clocks) = 21 if ADLSMP=1b"]
    ADSTS_2 = 0x02,
    #[doc = "Sample period (ADC clocks) = 9 if ADLSMP=0b Sample period (ADC clocks) = 25 if ADLSMP=1b"]
    ADSTS_3 = 0x03,
}
impl Adsts {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adsts {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adsts {
    #[inline(always)]
    fn from(val: u8) -> Adsts {
        Adsts::from_bits(val)
    }
}
impl From<Adsts> for u8 {
    #[inline(always)]
    fn from(val: Adsts) -> u8 {
        Adsts::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adtrg {
    #[doc = "Software trigger selected"]
    ADTRG_0 = 0x0,
    #[doc = "Hardware trigger selected"]
    ADTRG_1 = 0x01,
}
impl Adtrg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adtrg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adtrg {
    #[inline(always)]
    fn from(val: u8) -> Adtrg {
        Adtrg::from_bits(val)
    }
}
impl From<Adtrg> for u8 {
    #[inline(always)]
    fn from(val: Adtrg) -> u8 {
        Adtrg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avge {
    #[doc = "Hardware average function disabled"]
    AVGE_0 = 0x0,
    #[doc = "Hardware average function enabled"]
    AVGE_1 = 0x01,
}
impl Avge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avge {
    #[inline(always)]
    fn from(val: u8) -> Avge {
        Avge::from_bits(val)
    }
}
impl From<Avge> for u8 {
    #[inline(always)]
    fn from(val: Avge) -> u8 {
        Avge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Avgs {
    #[doc = "4 samples averaged"]
    AVGS_0 = 0x0,
    #[doc = "8 samples averaged"]
    AVGS_1 = 0x01,
    #[doc = "16 samples averaged"]
    AVGS_2 = 0x02,
    #[doc = "32 samples averaged"]
    AVGS_3 = 0x03,
}
impl Avgs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Avgs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Avgs {
    #[inline(always)]
    fn from(val: u8) -> Avgs {
        Avgs::from_bits(val)
    }
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(val: Avgs) -> u8 {
        Avgs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Awkst {
    #[doc = "No asynchronous interrupt."]
    AWKST_0 = 0x0,
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    AWKST_1 = 0x01,
}
impl Awkst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awkst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awkst {
    #[inline(always)]
    fn from(val: u8) -> Awkst {
        Awkst::from_bits(val)
    }
}
impl From<Awkst> for u8 {
    #[inline(always)]
    fn from(val: Awkst) -> u8 {
        Awkst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Calf {
    #[doc = "Calibration completed normally."]
    CALF_0 = 0x0,
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    CALF_1 = 0x01,
}
impl Calf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Calf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Calf {
    #[inline(always)]
    fn from(val: u8) -> Calf {
        Calf::from_bits(val)
    }
}
impl From<Calf> for u8 {
    #[inline(always)]
    fn from(val: Calf) -> u8 {
        Calf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaen {
    #[doc = "DMA disabled (default)"]
    DMAEN_0 = 0x0,
    #[doc = "DMA enabled"]
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
pub enum Hc0Adch {
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_0 = 0x0,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_1 = 0x01,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_2 = 0x02,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_3 = 0x03,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_4 = 0x04,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_5 = 0x05,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_6 = 0x06,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_7 = 0x07,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_8 = 0x08,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "External channel selection from ADC_ETC"]
    ADCH_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31 = 0x1f,
}
impl Hc0Adch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc0Adch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc0Adch {
    #[inline(always)]
    fn from(val: u8) -> Hc0Adch {
        Hc0Adch::from_bits(val)
    }
}
impl From<Hc0Adch> for u8 {
    #[inline(always)]
    fn from(val: Hc0Adch) -> u8 {
        Hc0Adch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc0Aien {
    #[doc = "Conversion complete interrupt disabled"]
    AIEN_0 = 0x0,
    #[doc = "Conversion complete interrupt enabled"]
    AIEN_1 = 0x01,
}
impl Hc0Aien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc0Aien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc0Aien {
    #[inline(always)]
    fn from(val: u8) -> Hc0Aien {
        Hc0Aien::from_bits(val)
    }
}
impl From<Hc0Aien> for u8 {
    #[inline(always)]
    fn from(val: Hc0Aien) -> u8 {
        Hc0Aien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HcAdch {
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_0 = 0x0,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_1 = 0x01,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_2 = 0x02,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_3 = 0x03,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_4 = 0x04,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_5 = 0x05,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_6 = 0x06,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_7 = 0x07,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_8 = 0x08,
    #[doc = "External channels 0 to 15 See External Signals for more information"]
    ADCH_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "External channel selection from ADC_ETC"]
    ADCH_16 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    #[doc = "VREFSH = internal channel, for ADC self-test, hard connected to VRH internally"]
    ADCH_25 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    #[doc = "Conversion Disabled. Hardware Triggers will not initiate any conversion."]
    ADCH_31 = 0x1f,
}
impl HcAdch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HcAdch {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HcAdch {
    #[inline(always)]
    fn from(val: u8) -> HcAdch {
        HcAdch::from_bits(val)
    }
}
impl From<HcAdch> for u8 {
    #[inline(always)]
    fn from(val: HcAdch) -> u8 {
        HcAdch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HcAien {
    #[doc = "Conversion complete interrupt disabled"]
    AIEN_0 = 0x0,
    #[doc = "Conversion complete interrupt enabled"]
    AIEN_1 = 0x01,
}
impl HcAien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HcAien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HcAien {
    #[inline(always)]
    fn from(val: u8) -> HcAien {
        HcAien::from_bits(val)
    }
}
impl From<HcAien> for u8 {
    #[inline(always)]
    fn from(val: HcAien) -> u8 {
        HcAien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mode {
    #[doc = "8-bit conversion"]
    MODE_0 = 0x0,
    #[doc = "10-bit conversion"]
    MODE_1 = 0x01,
    #[doc = "12-bit conversion"]
    MODE_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mode {
    #[inline(always)]
    fn from(val: u8) -> Mode {
        Mode::from_bits(val)
    }
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(val: Mode) -> u8 {
        Mode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovwren {
    #[doc = "Disable the overwriting. Existing Data in Data result register will not be overwritten by subsequent converted data."]
    OVWREN_0 = 0x0,
    #[doc = "Enable the overwriting."]
    OVWREN_1 = 0x01,
}
impl Ovwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovwren {
    #[inline(always)]
    fn from(val: u8) -> Ovwren {
        Ovwren::from_bits(val)
    }
}
impl From<Ovwren> for u8 {
    #[inline(always)]
    fn from(val: Ovwren) -> u8 {
        Ovwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Refsel {
    #[doc = "Selects VREFH/VREFL as reference voltage."]
    REFSEL_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Refsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Refsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Refsel {
    #[inline(always)]
    fn from(val: u8) -> Refsel {
        Refsel::from_bits(val)
    }
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(val: Refsel) -> u8 {
        Refsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sign {
    #[doc = "The offset value is added with the raw result"]
    SIGN_0 = 0x0,
    #[doc = "The offset value is subtracted from the raw converted value"]
    SIGN_1 = 0x01,
}
impl Sign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sign {
    #[inline(always)]
    fn from(val: u8) -> Sign {
        Sign::from_bits(val)
    }
}
impl From<Sign> for u8 {
    #[inline(always)]
    fn from(val: Sign) -> u8 {
        Sign::to_bits(val)
    }
}
