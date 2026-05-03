#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BODVBAT_HYST {
    #[doc = "25 mV."]
    HYST_25MV = 0x0,
    #[doc = "50 mV."]
    HYST_50MV = 0x01,
    #[doc = "75 mV."]
    HYST_75MV = 0x02,
    #[doc = "100 mV."]
    HYST_100MV = 0x03,
}
impl BODVBAT_HYST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BODVBAT_HYST {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BODVBAT_HYST {
    #[inline(always)]
    fn from(val: u8) -> BODVBAT_HYST {
        BODVBAT_HYST::from_bits(val)
    }
}
impl From<BODVBAT_HYST> for u8 {
    #[inline(always)]
    fn from(val: BODVBAT_HYST) -> u8 {
        BODVBAT_HYST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BOOTMODE {
    #[doc = "Latest IC boot was a Full power cycle boot sequence (PoR, Pin Reset, Brown Out Detectors Reset, Software Reset)."]
    POWERUP = 0x0,
    #[doc = "Latest IC boot was from DEEP SLEEP low power mode."]
    DEEPSLEEP = 0x01,
    #[doc = "Latest IC boot was from POWER DOWN low power mode."]
    POWERDOWN = 0x02,
    #[doc = "Latest IC boot was from DEEP POWER DOWN low power mode."]
    DEEPPOWERDOWN = 0x03,
}
impl BOOTMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BOOTMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BOOTMODE {
    #[inline(always)]
    fn from(val: u8) -> BOOTMODE {
        BOOTMODE::from_bits(val)
    }
}
impl From<BOOTMODE> for u8 {
    #[inline(always)]
    fn from(val: BOOTMODE) -> u8 {
        BOOTMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAPTESTOSCINSEL {
    #[doc = "Oscillator output pin (osc_out)."]
    OSCOUT = 0x0,
    #[doc = "Oscillator input pin (osc_in)."]
    OSCIN = 0x01,
}
impl CAPTESTOSCINSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAPTESTOSCINSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAPTESTOSCINSEL {
    #[inline(always)]
    fn from(val: u8) -> CAPTESTOSCINSEL {
        CAPTESTOSCINSEL::from_bits(val)
    }
}
impl From<CAPTESTOSCINSEL> for u8 {
    #[inline(always)]
    fn from(val: CAPTESTOSCINSEL) -> u8 {
        CAPTESTOSCINSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CAPTESTSTARTSRCSEL {
    #[doc = "Sourced from CAPTESTSTART."]
    CAPSTART = 0x0,
    #[doc = "Sourced from calibration."]
    CALIB = 0x01,
}
impl CAPTESTSTARTSRCSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CAPTESTSTARTSRCSEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CAPTESTSTARTSRCSEL {
    #[inline(always)]
    fn from(val: u8) -> CAPTESTSTARTSRCSEL {
        CAPTESTSTARTSRCSEL::from_bits(val)
    }
}
impl From<CAPTESTSTARTSRCSEL> for u8 {
    #[inline(always)]
    fn from(val: CAPTESTSTARTSRCSEL) -> u8 {
        CAPTESTSTARTSRCSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DISABLE_BLEED {
    #[doc = "LDO_MEM bleed current is enabled."]
    BLEED_ENABLE = 0x0,
    #[doc = "LDO_MEM bleed current is disabled. Should be set before entering in Deep Sleep low power mode and cleared after wake up from Deep SLeep low power mode."]
    BLEED_DISABLE = 0x01,
}
impl DISABLE_BLEED {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DISABLE_BLEED {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DISABLE_BLEED {
    #[inline(always)]
    fn from(val: u8) -> DISABLE_BLEED {
        DISABLE_BLEED::from_bits(val)
    }
}
impl From<DISABLE_BLEED> for u8 {
    #[inline(always)]
    fn from(val: DISABLE_BLEED) -> u8 {
        DISABLE_BLEED::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FILTERCGF_CLKDIV {
    #[doc = "Filter clock period duration equals 1 Analog Comparator clock period."]
    FILTER_1CLK_PERIOD = 0x0,
    #[doc = "Filter clock period duration equals 2 Analog Comparator clock period."]
    FILTER_2CLK_PERIOD = 0x01,
    #[doc = "Filter clock period duration equals 4 Analog Comparator clock period."]
    FILTER_4CLK_PERIOD = 0x02,
    #[doc = "Filter clock period duration equals 8 Analog Comparator clock period."]
    FILTER_8CLK_PERIOD = 0x03,
    #[doc = "Filter clock period duration equals 16 Analog Comparator clock period."]
    FILTER_16CLK_PERIOD = 0x04,
    #[doc = "Filter clock period duration equals 32 Analog Comparator clock period."]
    FILTER_32CLK_PERIOD = 0x05,
    #[doc = "Filter clock period duration equals 64 Analog Comparator clock period."]
    FILTER_64CLK_PERIOD = 0x06,
    #[doc = "Filter clock period duration equals 128 Analog Comparator clock period."]
    FILTER_128CLK_PERIOD = 0x07,
}
impl FILTERCGF_CLKDIV {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FILTERCGF_CLKDIV {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FILTERCGF_CLKDIV {
    #[inline(always)]
    fn from(val: u8) -> FILTERCGF_CLKDIV {
        FILTERCGF_CLKDIV::from_bits(val)
    }
}
impl From<FILTERCGF_CLKDIV> for u8 {
    #[inline(always)]
    fn from(val: FILTERCGF_CLKDIV) -> u8 {
        FILTERCGF_CLKDIV::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FILTERCGF_SAMPLEMODE {
    #[doc = "Bypass mode."]
    BYPASS = 0x0,
    #[doc = "Filter 1 clock period."]
    FILTER1CLK = 0x01,
    #[doc = "Filter 2 clock period."]
    FILTER2CLK = 0x02,
    #[doc = "Filter 3 clock period."]
    FILTER3CLK = 0x03,
}
impl FILTERCGF_SAMPLEMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FILTERCGF_SAMPLEMODE {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FILTERCGF_SAMPLEMODE {
    #[inline(always)]
    fn from(val: u8) -> FILTERCGF_SAMPLEMODE {
        FILTERCGF_SAMPLEMODE::from_bits(val)
    }
}
impl From<FILTERCGF_SAMPLEMODE> for u8 {
    #[inline(always)]
    fn from(val: FILTERCGF_SAMPLEMODE) -> u8 {
        FILTERCGF_SAMPLEMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LDODEEPSLEEPREF {
    #[doc = "LDO DEEP Sleep uses Flash buffer biasing as reference."]
    FLASHBUFFER = 0x0,
    #[doc = "LDO DEEP Sleep uses Band Gap 0.8V as reference."]
    BGP0P8V = 0x01,
}
impl LDODEEPSLEEPREF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LDODEEPSLEEPREF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LDODEEPSLEEPREF {
    #[inline(always)]
    fn from(val: u8) -> LDODEEPSLEEPREF {
        LDODEEPSLEEPREF::from_bits(val)
    }
}
impl From<LDODEEPSLEEPREF> for u8 {
    #[inline(always)]
    fn from(val: LDODEEPSLEEPREF) -> u8 {
        LDODEEPSLEEPREF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LOWPOWER {
    #[doc = "High speed mode."]
    HIGHSPEED = 0x0,
    #[doc = "Low power mode (Low speed)."]
    LOWSPEED = 0x01,
}
impl LOWPOWER {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LOWPOWER {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LOWPOWER {
    #[inline(always)]
    fn from(val: u8) -> LOWPOWER {
        LOWPOWER::from_bits(val)
    }
}
impl From<LOWPOWER> for u8 {
    #[inline(always)]
    fn from(val: LOWPOWER) -> u8 {
        LOWPOWER::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NMUX {
    #[doc = "VREF (See field VREFINPUT)."]
    VREF = 0x0,
    #[doc = "Pin P0_0."]
    CMP0_A = 0x01,
    #[doc = "Pin P0_9."]
    CMP0_B = 0x02,
    #[doc = "Pin P0_18."]
    CMP0_C = 0x03,
    #[doc = "Pin P1_14."]
    CMP0_D = 0x04,
    #[doc = "Pin P2_23."]
    CMP0_E = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl NMUX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NMUX {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NMUX {
    #[inline(always)]
    fn from(val: u8) -> NMUX {
        NMUX::from_bits(val)
    }
}
impl From<NMUX> for u8 {
    #[inline(always)]
    fn from(val: NMUX) -> u8 {
        NMUX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_AUXBIAS {
    #[doc = "auxiliary biasing is powered."]
    POWEREDON = 0x0,
    #[doc = "auxiliary biasing is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_AUXBIAS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_AUXBIAS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_AUXBIAS {
    #[inline(always)]
    fn from(val: u8) -> PDEN_AUXBIAS {
        PDEN_AUXBIAS::from_bits(val)
    }
}
impl From<PDEN_AUXBIAS> for u8 {
    #[inline(always)]
    fn from(val: PDEN_AUXBIAS) -> u8 {
        PDEN_AUXBIAS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_BODVBAT {
    #[doc = "BOD VBAT is powered."]
    POWEREDON = 0x0,
    #[doc = "BOD VBAT is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_BODVBAT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_BODVBAT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_BODVBAT {
    #[inline(always)]
    fn from(val: u8) -> PDEN_BODVBAT {
        PDEN_BODVBAT::from_bits(val)
    }
}
impl From<PDEN_BODVBAT> for u8 {
    #[inline(always)]
    fn from(val: PDEN_BODVBAT) -> u8 {
        PDEN_BODVBAT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_COMP {
    #[doc = "Analog Comparator is powered."]
    POWEREDON = 0x0,
    #[doc = "Analog Comparator is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_COMP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_COMP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_COMP {
    #[inline(always)]
    fn from(val: u8) -> PDEN_COMP {
        PDEN_COMP::from_bits(val)
    }
}
impl From<PDEN_COMP> for u8 {
    #[inline(always)]
    fn from(val: PDEN_COMP) -> u8 {
        PDEN_COMP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_FRO32K {
    #[doc = "FRO32KHz is powered."]
    POWEREDON = 0x0,
    #[doc = "FRO32KHz is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_FRO32K {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_FRO32K {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_FRO32K {
    #[inline(always)]
    fn from(val: u8) -> PDEN_FRO32K {
        PDEN_FRO32K::from_bits(val)
    }
}
impl From<PDEN_FRO32K> for u8 {
    #[inline(always)]
    fn from(val: PDEN_FRO32K) -> u8 {
        PDEN_FRO32K::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_LDOUSBHS {
    #[doc = "USB high speed LDO is powered."]
    POWEREDON = 0x0,
    #[doc = "USB high speed LDO is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_LDOUSBHS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_LDOUSBHS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_LDOUSBHS {
    #[inline(always)]
    fn from(val: u8) -> PDEN_LDOUSBHS {
        PDEN_LDOUSBHS::from_bits(val)
    }
}
impl From<PDEN_LDOUSBHS> for u8 {
    #[inline(always)]
    fn from(val: PDEN_LDOUSBHS) -> u8 {
        PDEN_LDOUSBHS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_LDOXO32M {
    #[doc = "High speed crystal LDO is powered."]
    POWEREDON = 0x0,
    #[doc = "High speed crystal LDO is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_LDOXO32M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_LDOXO32M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_LDOXO32M {
    #[inline(always)]
    fn from(val: u8) -> PDEN_LDOXO32M {
        PDEN_LDOXO32M::from_bits(val)
    }
}
impl From<PDEN_LDOXO32M> for u8 {
    #[inline(always)]
    fn from(val: PDEN_LDOXO32M) -> u8 {
        PDEN_LDOXO32M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_PLL0 {
    #[doc = "PLL0 is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL0 is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_PLL0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_PLL0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_PLL0 {
    #[inline(always)]
    fn from(val: u8) -> PDEN_PLL0 {
        PDEN_PLL0::from_bits(val)
    }
}
impl From<PDEN_PLL0> for u8 {
    #[inline(always)]
    fn from(val: PDEN_PLL0) -> u8 {
        PDEN_PLL0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_PLL0_SSCG {
    #[doc = "PLL0 Sread spectrum module is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL0 Sread spectrum module is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_PLL0_SSCG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_PLL0_SSCG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_PLL0_SSCG {
    #[inline(always)]
    fn from(val: u8) -> PDEN_PLL0_SSCG {
        PDEN_PLL0_SSCG::from_bits(val)
    }
}
impl From<PDEN_PLL0_SSCG> for u8 {
    #[inline(always)]
    fn from(val: PDEN_PLL0_SSCG) -> u8 {
        PDEN_PLL0_SSCG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_PLL1 {
    #[doc = "PLL1 is powered."]
    POWEREDON = 0x0,
    #[doc = "PLL1 is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_PLL1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_PLL1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_PLL1 {
    #[inline(always)]
    fn from(val: u8) -> PDEN_PLL1 {
        PDEN_PLL1::from_bits(val)
    }
}
impl From<PDEN_PLL1> for u8 {
    #[inline(always)]
    fn from(val: PDEN_PLL1) -> u8 {
        PDEN_PLL1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_RNG {
    #[doc = "TRNG clocks are powered."]
    POWEREDON = 0x0,
    #[doc = "TRNG clocks are powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_RNG {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_RNG {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_RNG {
    #[inline(always)]
    fn from(val: u8) -> PDEN_RNG {
        PDEN_RNG::from_bits(val)
    }
}
impl From<PDEN_RNG> for u8 {
    #[inline(always)]
    fn from(val: PDEN_RNG) -> u8 {
        PDEN_RNG::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_USBFSPHY {
    #[doc = "USB Full Speed phy is powered."]
    POWEREDON = 0x0,
    #[doc = "USB Full Speed phy is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_USBFSPHY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_USBFSPHY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_USBFSPHY {
    #[inline(always)]
    fn from(val: u8) -> PDEN_USBFSPHY {
        PDEN_USBFSPHY::from_bits(val)
    }
}
impl From<PDEN_USBFSPHY> for u8 {
    #[inline(always)]
    fn from(val: PDEN_USBFSPHY) -> u8 {
        PDEN_USBFSPHY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_USBHSPHY {
    #[doc = "USB HS phy is powered."]
    POWEREDON = 0x0,
    #[doc = "USB HS phy is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_USBHSPHY {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_USBHSPHY {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_USBHSPHY {
    #[inline(always)]
    fn from(val: u8) -> PDEN_USBHSPHY {
        PDEN_USBHSPHY::from_bits(val)
    }
}
impl From<PDEN_USBHSPHY> for u8 {
    #[inline(always)]
    fn from(val: PDEN_USBHSPHY) -> u8 {
        PDEN_USBHSPHY::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_XTAL32K {
    #[doc = "Crystal 32KHz is powered."]
    POWEREDON = 0x0,
    #[doc = "Crystal 32KHz is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_XTAL32K {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_XTAL32K {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_XTAL32K {
    #[inline(always)]
    fn from(val: u8) -> PDEN_XTAL32K {
        PDEN_XTAL32K::from_bits(val)
    }
}
impl From<PDEN_XTAL32K> for u8 {
    #[inline(always)]
    fn from(val: PDEN_XTAL32K) -> u8 {
        PDEN_XTAL32K::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PDEN_XTAL32M {
    #[doc = "High speed crystal is powered."]
    POWEREDON = 0x0,
    #[doc = "High speed crystal is powered down."]
    POWEREDOFF = 0x01,
}
impl PDEN_XTAL32M {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PDEN_XTAL32M {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PDEN_XTAL32M {
    #[inline(always)]
    fn from(val: u8) -> PDEN_XTAL32M {
        PDEN_XTAL32M::from_bits(val)
    }
}
impl From<PDEN_XTAL32M> for u8 {
    #[inline(always)]
    fn from(val: PDEN_XTAL32M) -> u8 {
        PDEN_XTAL32M::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PMUX {
    #[doc = "VREF (See fiedl VREFINPUT)."]
    VREF = 0x0,
    #[doc = "Pin P0_0."]
    CMP0_A = 0x01,
    #[doc = "Pin P0_9."]
    CMP0_B = 0x02,
    #[doc = "Pin P0_18."]
    CMP0_C = 0x03,
    #[doc = "Pin P1_14."]
    CMP0_D = 0x04,
    #[doc = "Pin P2_23."]
    CMP0_E = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl PMUX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PMUX {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PMUX {
    #[inline(always)]
    fn from(val: u8) -> PMUX {
        PMUX::from_bits(val)
    }
}
impl From<PMUX> for u8 {
    #[inline(always)]
    fn from(val: PMUX) -> u8 {
        PMUX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEL {
    #[doc = "FRO 32 KHz."]
    FRO32K = 0x0,
    #[doc = "XTAL 32KHz."]
    XTAL32K = 0x01,
}
impl SEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEL {
    #[inline(always)]
    fn from(val: u8) -> SEL {
        SEL::from_bits(val)
    }
}
impl From<SEL> for u8 {
    #[inline(always)]
    fn from(val: SEL) -> u8 {
        SEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SMB {
    #[doc = "Low leakage."]
    LOW = 0x0,
    #[doc = "Medium leakage."]
    MEDIUM = 0x01,
    #[doc = "Highest leakage."]
    HIGHEST = 0x02,
    #[doc = "Disable."]
    DISABLE = 0x03,
}
impl SMB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SMB {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SMB {
    #[inline(always)]
    fn from(val: u8) -> SMB {
        SMB::from_bits(val)
    }
}
impl From<SMB> for u8 {
    #[inline(always)]
    fn from(val: SMB) -> u8 {
        SMB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TRIGLVL {
    #[doc = "1.00 V."]
    V_1P00 = 0x0,
    #[doc = "1.10 V."]
    V_1P10 = 0x01,
    #[doc = "1.20 V."]
    V_1P20 = 0x02,
    #[doc = "1.30 V."]
    V_1P30 = 0x03,
    #[doc = "1.40 V."]
    V_1P40 = 0x04,
    #[doc = "1.50 V."]
    V_1P50 = 0x05,
    #[doc = "1.60 V."]
    V_1P60 = 0x06,
    #[doc = "1.65 V."]
    V_1P65 = 0x07,
    #[doc = "1.70 V."]
    V_1P70 = 0x08,
    #[doc = "1.75 V."]
    V_1P75 = 0x09,
    #[doc = "1.80 V."]
    V_1P80 = 0x0a,
    #[doc = "1.90 V."]
    V_1P90 = 0x0b,
    #[doc = "2.00 V."]
    V_2P00 = 0x0c,
    #[doc = "2.10 V."]
    V_2P10 = 0x0d,
    #[doc = "2.20 V."]
    V_2P20 = 0x0e,
    #[doc = "2.30 V."]
    V_2P30 = 0x0f,
    #[doc = "2.40 V."]
    V_2P40 = 0x10,
    #[doc = "2.50 V."]
    V_2P50 = 0x11,
    #[doc = "2.60 V."]
    V_2P60 = 0x12,
    #[doc = "2.70 V."]
    V_2P70 = 0x13,
    #[doc = "2.806 V."]
    V_2P80 = 0x14,
    #[doc = "2.90 V."]
    V_2P90 = 0x15,
    #[doc = "3.00 V."]
    V_3P00 = 0x16,
    #[doc = "3.10 V."]
    V_3P10 = 0x17,
    #[doc = "3.20 V."]
    V_3P20 = 0x18,
    #[doc = "3.30 V."]
    V_3P30_2 = 0x19,
    #[doc = "3.30 V."]
    V_3P30_3 = 0x1a,
    #[doc = "3.30 V."]
    V_3P30_4 = 0x1b,
    #[doc = "3.30 V."]
    V_3P30_5 = 0x1c,
    #[doc = "3.30 V."]
    V_3P30_6 = 0x1d,
    #[doc = "3.30 V."]
    V_3P30_7 = 0x1e,
    #[doc = "3.30 V."]
    V_3P30_8 = 0x1f,
}
impl TRIGLVL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TRIGLVL {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TRIGLVL {
    #[inline(always)]
    fn from(val: u8) -> TRIGLVL {
        TRIGLVL::from_bits(val)
    }
}
impl From<TRIGLVL> for u8 {
    #[inline(always)]
    fn from(val: TRIGLVL) -> u8 {
        TRIGLVL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VADJ {
    #[doc = "1.22 V."]
    V_1P220 = 0x0,
    #[doc = "0.7 V."]
    V_0P700 = 0x01,
    #[doc = "0.725 V."]
    V_0P725 = 0x02,
    #[doc = "0.75 V."]
    V_0P750 = 0x03,
    #[doc = "0.775 V."]
    V_0P775 = 0x04,
    #[doc = "0.8 V."]
    V_0P800 = 0x05,
    #[doc = "0.825 V."]
    V_0P825 = 0x06,
    #[doc = "0.85 V."]
    V_0P850 = 0x07,
    #[doc = "0.875 V."]
    V_0P875 = 0x08,
    #[doc = "0.9 V."]
    V_0P900 = 0x09,
    #[doc = "0.96 V."]
    V_0P960 = 0x0a,
    #[doc = "0.97 V."]
    V_0P970 = 0x0b,
    #[doc = "0.98 V."]
    V_0P980 = 0x0c,
    #[doc = "0.99 V."]
    V_0P990 = 0x0d,
    #[doc = "1 V."]
    V_1P000 = 0x0e,
    #[doc = "1.01 V."]
    V_1P010 = 0x0f,
    #[doc = "1.02 V."]
    V_1P020 = 0x10,
    #[doc = "1.03 V."]
    V_1P030 = 0x11,
    #[doc = "1.04 V."]
    V_1P040 = 0x12,
    #[doc = "1.05 V."]
    V_1P050 = 0x13,
    #[doc = "1.06 V."]
    V_1P060 = 0x14,
    #[doc = "1.07 V."]
    V_1P070 = 0x15,
    #[doc = "1.08 V."]
    V_1P080 = 0x16,
    #[doc = "1.09 V."]
    V_1P090 = 0x17,
    #[doc = "1.1 V."]
    V_1P100 = 0x18,
    #[doc = "1.11 V."]
    V_1P110 = 0x19,
    #[doc = "1.12 V."]
    V_1P120 = 0x1a,
    #[doc = "1.13 V."]
    V_1P130 = 0x1b,
    #[doc = "1.14 V."]
    V_1P140 = 0x1c,
    #[doc = "1.15 V."]
    V_1P150 = 0x1d,
    #[doc = "1.16 V."]
    V_1P160 = 0x1e,
    #[doc = "1.22 V."]
    V_1P220_1 = 0x1f,
}
impl VADJ {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VADJ {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VADJ {
    #[inline(always)]
    fn from(val: u8) -> VADJ {
        VADJ::from_bits(val)
    }
}
impl From<VADJ> for u8 {
    #[inline(always)]
    fn from(val: VADJ) -> u8 {
        VADJ::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VOUT {
    #[doc = "0.95 V."]
    V_DCDC_0P950 = 0x0,
    #[doc = "0.975 V."]
    V_DCDC_0P975 = 0x01,
    #[doc = "1 V."]
    V_DCDC_1P000 = 0x02,
    #[doc = "1.025 V."]
    V_DCDC_1P025 = 0x03,
    #[doc = "1.05 V."]
    V_DCDC_1P050 = 0x04,
    #[doc = "1.075 V."]
    V_DCDC_1P075 = 0x05,
    #[doc = "1.1 V."]
    V_DCDC_1P100 = 0x06,
    #[doc = "1.125 V."]
    V_DCDC_1P125 = 0x07,
    #[doc = "1.15 V."]
    V_DCDC_1P150 = 0x08,
    #[doc = "1.175 V."]
    V_DCDC_1P175 = 0x09,
    #[doc = "1.2 V."]
    V_DCDC_1P200 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl VOUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VOUT {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VOUT {
    #[inline(always)]
    fn from(val: u8) -> VOUT {
        VOUT::from_bits(val)
    }
}
impl From<VOUT> for u8 {
    #[inline(always)]
    fn from(val: VOUT) -> u8 {
        VOUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VREFINPUT {
    #[doc = "Select internal VREF."]
    INTERNALREF = 0x0,
    #[doc = "Select VDDA."]
    VDDA = 0x01,
}
impl VREFINPUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VREFINPUT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VREFINPUT {
    #[inline(always)]
    fn from(val: u8) -> VREFINPUT {
        VREFINPUT::from_bits(val)
    }
}
impl From<VREFINPUT> for u8 {
    #[inline(always)]
    fn from(val: VREFINPUT) -> u8 {
        VREFINPUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WAKUPIO_RST {
    #[doc = "Wakeup IO is not reset."]
    RELEASED = 0x0,
    #[doc = "Wakeup IO is reset."]
    ASSERTED = 0x01,
}
impl WAKUPIO_RST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WAKUPIO_RST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WAKUPIO_RST {
    #[inline(always)]
    fn from(val: u8) -> WAKUPIO_RST {
        WAKUPIO_RST::from_bits(val)
    }
}
impl From<WAKUPIO_RST> for u8 {
    #[inline(always)]
    fn from(val: WAKUPIO_RST) -> u8 {
        WAKUPIO_RST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XTAL32KOSCFAILURE {
    #[doc = "No oscillation failure has been detetced since the last time this bit has been cleared."]
    NOFAIL = 0x0,
    #[doc = "At least one oscillation failure has been detetced since the last time this bit has been cleared."]
    FAILURE = 0x01,
}
impl XTAL32KOSCFAILURE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XTAL32KOSCFAILURE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XTAL32KOSCFAILURE {
    #[inline(always)]
    fn from(val: u8) -> XTAL32KOSCFAILURE {
        XTAL32KOSCFAILURE::from_bits(val)
    }
}
impl From<XTAL32KOSCFAILURE> for u8 {
    #[inline(always)]
    fn from(val: XTAL32KOSCFAILURE) -> u8 {
        XTAL32KOSCFAILURE::to_bits(val)
    }
}
