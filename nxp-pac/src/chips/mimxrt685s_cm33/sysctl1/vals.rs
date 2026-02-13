#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datainsel {
    #[doc = "Original FLEXCOMM I2S signals"]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Shared Set0 I2S signals."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Shared Set1 I2S signals."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl Datainsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datainsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datainsel {
    #[inline(always)]
    fn from(val: u8) -> Datainsel {
        Datainsel::from_bits(val)
    }
}
impl From<Datainsel> for u8 {
    #[inline(always)]
    fn from(val: Datainsel) -> u8 {
        Datainsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dataoutsel {
    #[doc = "Original FLEXCOMM I2S signals"]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Shared Set0 I2S signals."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Shared Set1 I2S signals."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dataoutsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dataoutsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dataoutsel {
    #[inline(always)]
    fn from(val: u8) -> Dataoutsel {
        Dataoutsel::from_bits(val)
    }
}
impl From<Dataoutsel> for u8 {
    #[inline(always)]
    fn from(val: Dataoutsel) -> u8 {
        Dataoutsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum F20dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl F20dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> F20dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for F20dataouten {
    #[inline(always)]
    fn from(val: u8) -> F20dataouten {
        F20dataouten::from_bits(val)
    }
}
impl From<F20dataouten> for u8 {
    #[inline(always)]
    fn from(val: F20dataouten) -> u8 {
        F20dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc0dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc0dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc0dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc0dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc0dataouten {
        Fc0dataouten::from_bits(val)
    }
}
impl From<Fc0dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc0dataouten) -> u8 {
        Fc0dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc1dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc1dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc1dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc1dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc1dataouten {
        Fc1dataouten::from_bits(val)
    }
}
impl From<Fc1dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc1dataouten) -> u8 {
        Fc1dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc3dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc3dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc3dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc3dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc3dataouten {
        Fc3dataouten::from_bits(val)
    }
}
impl From<Fc3dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc3dataouten) -> u8 {
        Fc3dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc4dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc4dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc4dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc4dataouten {
        Fc4dataouten::from_bits(val)
    }
}
impl From<Fc4dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc4dataouten) -> u8 {
        Fc4dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc5dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc5dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc5dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc5dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc5dataouten {
        Fc5dataouten::from_bits(val)
    }
}
impl From<Fc5dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc5dataouten) -> u8 {
        Fc5dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc6dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc6dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc6dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc6dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc6dataouten {
        Fc6dataouten::from_bits(val)
    }
}
impl From<Fc6dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc6dataouten) -> u8 {
        Fc6dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc7dataouten {
    #[doc = "Input"]
    INPUT = 0x0,
    #[doc = "Output"]
    OUTPUT = 0x01,
}
impl Fc7dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc7dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc7dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc7dataouten {
        Fc7dataouten::from_bits(val)
    }
}
impl From<Fc7dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc7dataouten) -> u8 {
        Fc7dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mclkpindir {
    #[doc = "MCLK is in input direction."]
    INPUT_DIRECTION = 0x0,
    #[doc = "MCLK is in the output direction."]
    OUTPUT_DIRECTION = 0x01,
}
impl Mclkpindir {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mclkpindir {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mclkpindir {
    #[inline(always)]
    fn from(val: u8) -> Mclkpindir {
        Mclkpindir::from_bits(val)
    }
}
impl From<Mclkpindir> for u8 {
    #[inline(always)]
    fn from(val: Mclkpindir) -> u8 {
        Mclkpindir::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxevpulsegen {
    #[doc = "No effect."]
    NO_EFFECT = 0x0,
    #[doc = "Pulse RXEV High for one PSCLK cycle."]
    PULSE_RXEV_HIGH = 0x01,
}
impl Rxevpulsegen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxevpulsegen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxevpulsegen {
    #[inline(always)]
    fn from(val: u8) -> Rxevpulsegen {
        Rxevpulsegen::from_bits(val)
    }
}
impl From<Rxevpulsegen> for u8 {
    #[inline(always)]
    fn from(val: Rxevpulsegen) -> u8 {
        Rxevpulsegen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckinsel {
    #[doc = "Original FLEXCOMM I2S signals"]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Shared Set0 I2S signals."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Shared Set1 I2S signals."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sckinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckinsel {
    #[inline(always)]
    fn from(val: u8) -> Sckinsel {
        Sckinsel::from_bits(val)
    }
}
impl From<Sckinsel> for u8 {
    #[inline(always)]
    fn from(val: Sckinsel) -> u8 {
        Sckinsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Shareddatasel {
    #[doc = "FLEXCOMM0"]
    FLEXCOMM0 = 0x0,
    #[doc = "FLEXCOMM1"]
    FLEXCOMM1 = 0x01,
    #[doc = "FLEXCOMM2"]
    FLEXCOMM2 = 0x02,
    #[doc = "FLEXCOMM3"]
    FLEXCOMM3 = 0x03,
    #[doc = "FLEXCOMM4"]
    FLEXCOMM4 = 0x04,
    #[doc = "FLEXCOMM5"]
    FLEXCOMM5 = 0x05,
    #[doc = "FLEXCOMM6"]
    FLEXCOMM6 = 0x06,
    #[doc = "FLEXCOMM7"]
    FLEXCOMM7 = 0x07,
}
impl Shareddatasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Shareddatasel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Shareddatasel {
    #[inline(always)]
    fn from(val: u8) -> Shareddatasel {
        Shareddatasel::from_bits(val)
    }
}
impl From<Shareddatasel> for u8 {
    #[inline(always)]
    fn from(val: Shareddatasel) -> u8 {
        Shareddatasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sharedscksel {
    #[doc = "FLEXCOMM0"]
    FLEXCOMM0 = 0x0,
    #[doc = "FLEXCOMM1"]
    FLEXCOMM1 = 0x01,
    #[doc = "FLEXCOMM2"]
    FLEXCOMM2 = 0x02,
    #[doc = "FLEXCOMM3"]
    FLEXCOMM3 = 0x03,
    #[doc = "FLEXCOMM4"]
    FLEXCOMM4 = 0x04,
    #[doc = "FLEXCOMM5"]
    FLEXCOMM5 = 0x05,
    #[doc = "FLEXCOMM6"]
    FLEXCOMM6 = 0x06,
    #[doc = "FLEXCOMM7"]
    FLEXCOMM7 = 0x07,
}
impl Sharedscksel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sharedscksel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sharedscksel {
    #[inline(always)]
    fn from(val: u8) -> Sharedscksel {
        Sharedscksel::from_bits(val)
    }
}
impl From<Sharedscksel> for u8 {
    #[inline(always)]
    fn from(val: Sharedscksel) -> u8 {
        Sharedscksel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sharedwssel {
    #[doc = "FLEXCOMM0"]
    FLEXCOMM0 = 0x0,
    #[doc = "FLEXCOMM1"]
    FLEXCOMM1 = 0x01,
    #[doc = "FLEXCOMM2"]
    FLEXCOMM2 = 0x02,
    #[doc = "FLEXCOMM3"]
    FLEXCOMM3 = 0x03,
    #[doc = "FLEXCOMM4"]
    FLEXCOMM4 = 0x04,
    #[doc = "FLEXCOMM5"]
    FLEXCOMM5 = 0x05,
    #[doc = "FLEXCOMM6"]
    FLEXCOMM6 = 0x06,
    #[doc = "FLEXCOMM7"]
    FLEXCOMM7 = 0x07,
}
impl Sharedwssel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sharedwssel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sharedwssel {
    #[inline(always)]
    fn from(val: u8) -> Sharedwssel {
        Sharedwssel::from_bits(val)
    }
}
impl From<Sharedwssel> for u8 {
    #[inline(always)]
    fn from(val: Sharedwssel) -> u8 {
        Sharedwssel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wsinsel {
    #[doc = "Original FLEXCOMM I2S signals"]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Shared Set0 I2S signals."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Shared Set1 I2S signals."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl Wsinsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wsinsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wsinsel {
    #[inline(always)]
    fn from(val: u8) -> Wsinsel {
        Wsinsel::from_bits(val)
    }
}
impl From<Wsinsel> for u8 {
    #[inline(always)]
    fn from(val: Wsinsel) -> u8 {
        Wsinsel::to_bits(val)
    }
}
