#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datainsel {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
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
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
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
pub enum Fc0dataouten {
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC0 does contribute to this shared set."]
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
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC1 does contribute to this shared set."]
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
pub enum Fc2dataouten {
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC2 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl Fc2dataouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fc2dataouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fc2dataouten {
    #[inline(always)]
    fn from(val: u8) -> Fc2dataouten {
        Fc2dataouten::from_bits(val)
    }
}
impl From<Fc2dataouten> for u8 {
    #[inline(always)]
    fn from(val: Fc2dataouten) -> u8 {
        Fc2dataouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fc4dataouten {
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC4 does contribute to this shared set."]
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
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC5 does contribute to this shared set."]
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
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC6 does contribute to this shared set."]
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
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC7 does contribute to this shared set."]
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
pub enum Sckinsel {
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
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
    #[doc = "DATA input for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0x0,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 0x01,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 0x02,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 0x03,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 0x04,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 0x05,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 0x06,
    #[doc = "DATA input for this shared signal set comes from Flexcomm 7."]
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
    #[doc = "SCK for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0x0,
    #[doc = "SCK for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 0x01,
    #[doc = "SCK for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 0x02,
    #[doc = "SCK for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 0x03,
    #[doc = "SCK for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 0x04,
    #[doc = "SCK for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 0x05,
    #[doc = "SCK for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 0x06,
    #[doc = "SCK for this shared signal set comes from Flexcomm 7."]
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
    #[doc = "WS for this shared signal set comes from Flexcomm 0."]
    FLEXCOMM0 = 0x0,
    #[doc = "WS for this shared signal set comes from Flexcomm 1."]
    FLEXCOMM1 = 0x01,
    #[doc = "WS for this shared signal set comes from Flexcomm 2."]
    FLEXCOMM2 = 0x02,
    #[doc = "WS for this shared signal set comes from Flexcomm 3."]
    FLEXCOMM3 = 0x03,
    #[doc = "WS for this shared signal set comes from Flexcomm 4."]
    FLEXCOMM4 = 0x04,
    #[doc = "WS for this shared signal set comes from Flexcomm 5."]
    FLEXCOMM5 = 0x05,
    #[doc = "WS for this shared signal set comes from Flexcomm 6."]
    FLEXCOMM6 = 0x06,
    #[doc = "WS for this shared signal set comes from Flexcomm 7."]
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
pub enum Updatelckout {
    #[doc = "Normal Mode. Can be written to."]
    NORMAL_MODE = 0x0,
    #[doc = "Protected Mode. Cannot be written to."]
    PROTECTED_MODE = 0x01,
}
impl Updatelckout {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Updatelckout {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Updatelckout {
    #[inline(always)]
    fn from(val: u8) -> Updatelckout {
        Updatelckout::from_bits(val)
    }
}
impl From<Updatelckout> for u8 {
    #[inline(always)]
    fn from(val: Updatelckout) -> u8 {
        Updatelckout::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhs3vNok {
    #[doc = "3v3 supply is good."]
    SUPPLY_3V_OK = 0x0,
    #[doc = "3v3 supply is too low."]
    SUPPLY_3V_LOW = 0x01,
}
impl Usbhs3vNok {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhs3vNok {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhs3vNok {
    #[inline(always)]
    fn from(val: u8) -> Usbhs3vNok {
        Usbhs3vNok::from_bits(val)
    }
}
impl From<Usbhs3vNok> for u8 {
    #[inline(always)]
    fn from(val: Usbhs3vNok) -> u8 {
        Usbhs3vNok::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wsinsel {
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
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
