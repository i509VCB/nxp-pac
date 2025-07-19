#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adiclk {
    #[doc = "IPG clock"]
    ADICLK_0 = 0x0,
    #[doc = "IPG clock divided by 2"]
    ADICLK_1 = 0x01,
    #[doc = "Alternate clock (ALTCLK)"]
    ADICLK_2 = 0x02,
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
pub enum Hc0Adch {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
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
pub enum HcAdch {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
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
