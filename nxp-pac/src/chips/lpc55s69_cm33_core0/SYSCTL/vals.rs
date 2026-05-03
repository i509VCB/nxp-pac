#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATAINSEL {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl DATAINSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATAINSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATAINSEL {
    #[inline(always)]
    fn from(val: u8) -> DATAINSEL {
        DATAINSEL::from_bits(val)
    }
}
impl From<DATAINSEL> for u8 {
    #[inline(always)]
    fn from(val: DATAINSEL) -> u8 {
        DATAINSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DATAOUTSEL {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl DATAOUTSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DATAOUTSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DATAOUTSEL {
    #[inline(always)]
    fn from(val: u8) -> DATAOUTSEL {
        DATAOUTSEL::from_bits(val)
    }
}
impl From<DATAOUTSEL> for u8 {
    #[inline(always)]
    fn from(val: DATAOUTSEL) -> u8 {
        DATAOUTSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC0DATAOUTEN {
    #[doc = "Data output from FC0 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC0 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC0DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC0DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC0DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC0DATAOUTEN {
        FC0DATAOUTEN::from_bits(val)
    }
}
impl From<FC0DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC0DATAOUTEN) -> u8 {
        FC0DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC1DATAOUTEN {
    #[doc = "Data output from FC1 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC1 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC1DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC1DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC1DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC1DATAOUTEN {
        FC1DATAOUTEN::from_bits(val)
    }
}
impl From<FC1DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC1DATAOUTEN) -> u8 {
        FC1DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC2DATAOUTEN {
    #[doc = "Data output from FC2 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC2 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC2DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC2DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC2DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC2DATAOUTEN {
        FC2DATAOUTEN::from_bits(val)
    }
}
impl From<FC2DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC2DATAOUTEN) -> u8 {
        FC2DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC4DATAOUTEN {
    #[doc = "Data output from FC4 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC4 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC4DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC4DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC4DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC4DATAOUTEN {
        FC4DATAOUTEN::from_bits(val)
    }
}
impl From<FC4DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC4DATAOUTEN) -> u8 {
        FC4DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC5DATAOUTEN {
    #[doc = "Data output from FC5 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC5 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC5DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC5DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC5DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC5DATAOUTEN {
        FC5DATAOUTEN::from_bits(val)
    }
}
impl From<FC5DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC5DATAOUTEN) -> u8 {
        FC5DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC6DATAOUTEN {
    #[doc = "Data output from FC6 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC6 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC6DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC6DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC6DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC6DATAOUTEN {
        FC6DATAOUTEN::from_bits(val)
    }
}
impl From<FC6DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC6DATAOUTEN) -> u8 {
        FC6DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FC7DATAOUTEN {
    #[doc = "Data output from FC7 does not contribute to this shared set."]
    INPUT = 0x0,
    #[doc = "Data output from FC7 does contribute to this shared set."]
    OUTPUT = 0x01,
}
impl FC7DATAOUTEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FC7DATAOUTEN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FC7DATAOUTEN {
    #[inline(always)]
    fn from(val: u8) -> FC7DATAOUTEN {
        FC7DATAOUTEN::from_bits(val)
    }
}
impl From<FC7DATAOUTEN> for u8 {
    #[inline(always)]
    fn from(val: FC7DATAOUTEN) -> u8 {
        FC7DATAOUTEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SCKINSEL {
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl SCKINSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SCKINSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SCKINSEL {
    #[inline(always)]
    fn from(val: u8) -> SCKINSEL {
        SCKINSEL::from_bits(val)
    }
}
impl From<SCKINSEL> for u8 {
    #[inline(always)]
    fn from(val: SCKINSEL) -> u8 {
        SCKINSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SHAREDDATASEL {
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
impl SHAREDDATASEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SHAREDDATASEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SHAREDDATASEL {
    #[inline(always)]
    fn from(val: u8) -> SHAREDDATASEL {
        SHAREDDATASEL::from_bits(val)
    }
}
impl From<SHAREDDATASEL> for u8 {
    #[inline(always)]
    fn from(val: SHAREDDATASEL) -> u8 {
        SHAREDDATASEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SHAREDSCKSEL {
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
impl SHAREDSCKSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SHAREDSCKSEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SHAREDSCKSEL {
    #[inline(always)]
    fn from(val: u8) -> SHAREDSCKSEL {
        SHAREDSCKSEL::from_bits(val)
    }
}
impl From<SHAREDSCKSEL> for u8 {
    #[inline(always)]
    fn from(val: SHAREDSCKSEL) -> u8 {
        SHAREDSCKSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SHAREDWSSEL {
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
impl SHAREDWSSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SHAREDWSSEL {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SHAREDWSSEL {
    #[inline(always)]
    fn from(val: u8) -> SHAREDWSSEL {
        SHAREDWSSEL::from_bits(val)
    }
}
impl From<SHAREDWSSEL> for u8 {
    #[inline(always)]
    fn from(val: SHAREDWSSEL) -> u8 {
        SHAREDWSSEL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UPDATELCKOUT {
    #[doc = "Normal Mode. Can be written to."]
    NORMAL_MODE = 0x0,
    #[doc = "Protected Mode. Cannot be written to."]
    PROTECTED_MODE = 0x01,
}
impl UPDATELCKOUT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UPDATELCKOUT {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UPDATELCKOUT {
    #[inline(always)]
    fn from(val: u8) -> UPDATELCKOUT {
        UPDATELCKOUT::from_bits(val)
    }
}
impl From<UPDATELCKOUT> for u8 {
    #[inline(always)]
    fn from(val: UPDATELCKOUT) -> u8 {
        UPDATELCKOUT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USBHS_3V_NOK {
    #[doc = "3v3 supply is good."]
    SUPPLY_3V_OK = 0x0,
    #[doc = "3v3 supply is too low."]
    SUPPLY_3V_LOW = 0x01,
}
impl USBHS_3V_NOK {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USBHS_3V_NOK {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USBHS_3V_NOK {
    #[inline(always)]
    fn from(val: u8) -> USBHS_3V_NOK {
        USBHS_3V_NOK::from_bits(val)
    }
}
impl From<USBHS_3V_NOK> for u8 {
    #[inline(always)]
    fn from(val: USBHS_3V_NOK) -> u8 {
        USBHS_3V_NOK::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WSINSEL {
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0x0,
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 0x01,
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 0x02,
    _RESERVED_3 = 0x03,
}
impl WSINSEL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WSINSEL {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WSINSEL {
    #[inline(always)]
    fn from(val: u8) -> WSINSEL {
        WSINSEL::from_bits(val)
    }
}
impl From<WSINSEL> for u8 {
    #[inline(always)]
    fn from(val: WSINSEL) -> u8 {
        WSINSEL::to_bits(val)
    }
}
