#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Adc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc {
    #[inline(always)]
    fn from(val: u8) -> Adc {
        Adc::from_bits(val)
    }
}
impl From<Adc> for u8 {
    #[inline(always)]
    fn from(val: Adc) -> u8 {
        Adc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahb2apb0 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl Ahb2apb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahb2apb0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahb2apb0 {
    #[inline(always)]
    fn from(val: u8) -> Ahb2apb0 {
        Ahb2apb0::from_bits(val)
    }
}
impl From<Ahb2apb0> for u8 {
    #[inline(always)]
    fn from(val: Ahb2apb0) -> u8 {
        Ahb2apb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahb2apb1 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl Ahb2apb1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahb2apb1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahb2apb1 {
    #[inline(always)]
    fn from(val: u8) -> Ahb2apb1 {
        Ahb2apb1::from_bits(val)
    }
}
impl From<Ahb2apb1> for u8 {
    #[inline(always)]
    fn from(val: Ahb2apb1) -> u8 {
        Ahb2apb1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbFlexspiAccessDisable {
    #[doc = "Enable AHB access to FLEXSPI"]
    ENABLE = 0x0,
    #[doc = "Disable AHB access to FLEXSPI"]
    DISABLE = 0x01,
}
impl AhbFlexspiAccessDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbFlexspiAccessDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbFlexspiAccessDisable {
    #[inline(always)]
    fn from(val: u8) -> AhbFlexspiAccessDisable {
        AhbFlexspiAccessDisable::from_bits(val)
    }
}
impl From<AhbFlexspiAccessDisable> for u8 {
    #[inline(always)]
    fn from(val: AhbFlexspiAccessDisable) -> u8 {
        AhbFlexspiAccessDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram00If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram00If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram00If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram00If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram00If {
        AhbSramAccessDisableSram00If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram00If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram00If) -> u8 {
        AhbSramAccessDisableSram00If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram01If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram01If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram01If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram01If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram01If {
        AhbSramAccessDisableSram01If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram01If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram01If) -> u8 {
        AhbSramAccessDisableSram01If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram02If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram02If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram02If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram02If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram02If {
        AhbSramAccessDisableSram02If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram02If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram02If) -> u8 {
        AhbSramAccessDisableSram02If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram03If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram03If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram03If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram03If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram03If {
        AhbSramAccessDisableSram03If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram03If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram03If) -> u8 {
        AhbSramAccessDisableSram03If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram04If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram04If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram04If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram04If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram04If {
        AhbSramAccessDisableSram04If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram04If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram04If) -> u8 {
        AhbSramAccessDisableSram04If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram05If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram05If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram05If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram05If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram05If {
        AhbSramAccessDisableSram05If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram05If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram05If) -> u8 {
        AhbSramAccessDisableSram05If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram06If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram06If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram06If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram06If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram06If {
        AhbSramAccessDisableSram06If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram06If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram06If) -> u8 {
        AhbSramAccessDisableSram06If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram07If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram07If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram07If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram07If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram07If {
        AhbSramAccessDisableSram07If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram07If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram07If) -> u8 {
        AhbSramAccessDisableSram07If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram08If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram08If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram08If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram08If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram08If {
        AhbSramAccessDisableSram08If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram08If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram08If) -> u8 {
        AhbSramAccessDisableSram08If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram09If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram09If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram09If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram09If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram09If {
        AhbSramAccessDisableSram09If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram09If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram09If) -> u8 {
        AhbSramAccessDisableSram09If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram10If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram10If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram10If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram10If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram10If {
        AhbSramAccessDisableSram10If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram10If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram10If) -> u8 {
        AhbSramAccessDisableSram10If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram11If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram11If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram11If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram11If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram11If {
        AhbSramAccessDisableSram11If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram11If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram11If) -> u8 {
        AhbSramAccessDisableSram11If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram12If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram12If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram12If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram12If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram12If {
        AhbSramAccessDisableSram12If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram12If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram12If) -> u8 {
        AhbSramAccessDisableSram12If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram13If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram13If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram13If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram13If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram13If {
        AhbSramAccessDisableSram13If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram13If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram13If) -> u8 {
        AhbSramAccessDisableSram13If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram14If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram14If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram14If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram14If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram14If {
        AhbSramAccessDisableSram14If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram14If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram14If) -> u8 {
        AhbSramAccessDisableSram14If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram15If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram15If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram15If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram15If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram15If {
        AhbSramAccessDisableSram15If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram15If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram15If) -> u8 {
        AhbSramAccessDisableSram15If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram16If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram16If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram16If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram16If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram16If {
        AhbSramAccessDisableSram16If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram16If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram16If) -> u8 {
        AhbSramAccessDisableSram16If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram17If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram17If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram17If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram17If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram17If {
        AhbSramAccessDisableSram17If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram17If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram17If) -> u8 {
        AhbSramAccessDisableSram17If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram18If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram18If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram18If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram18If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram18If {
        AhbSramAccessDisableSram18If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram18If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram18If) -> u8 {
        AhbSramAccessDisableSram18If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram19If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram19If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram19If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram19If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram19If {
        AhbSramAccessDisableSram19If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram19If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram19If) -> u8 {
        AhbSramAccessDisableSram19If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram20If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram20If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram20If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram20If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram20If {
        AhbSramAccessDisableSram20If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram20If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram20If) -> u8 {
        AhbSramAccessDisableSram20If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram21If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram21If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram21If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram21If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram21If {
        AhbSramAccessDisableSram21If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram21If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram21If) -> u8 {
        AhbSramAccessDisableSram21If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram22If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram22If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram22If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram22If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram22If {
        AhbSramAccessDisableSram22If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram22If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram22If) -> u8 {
        AhbSramAccessDisableSram22If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram23If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram23If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram23If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram23If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram23If {
        AhbSramAccessDisableSram23If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram23If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram23If) -> u8 {
        AhbSramAccessDisableSram23If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram24If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram24If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram24If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram24If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram24If {
        AhbSramAccessDisableSram24If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram24If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram24If) -> u8 {
        AhbSramAccessDisableSram24If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram25If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram25If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram25If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram25If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram25If {
        AhbSramAccessDisableSram25If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram25If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram25If) -> u8 {
        AhbSramAccessDisableSram25If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram26If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram26If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram26If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram26If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram26If {
        AhbSramAccessDisableSram26If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram26If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram26If) -> u8 {
        AhbSramAccessDisableSram26If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram27If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram27If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram27If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram27If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram27If {
        AhbSramAccessDisableSram27If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram27If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram27If) -> u8 {
        AhbSramAccessDisableSram27If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram28If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram28If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram28If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram28If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram28If {
        AhbSramAccessDisableSram28If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram28If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram28If) -> u8 {
        AhbSramAccessDisableSram28If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbSramAccessDisableSram29If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl AhbSramAccessDisableSram29If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbSramAccessDisableSram29If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbSramAccessDisableSram29If {
    #[inline(always)]
    fn from(val: u8) -> AhbSramAccessDisableSram29If {
        AhbSramAccessDisableSram29If::from_bits(val)
    }
}
impl From<AhbSramAccessDisableSram29If> for u8 {
    #[inline(always)]
    fn from(val: AhbSramAccessDisableSram29If) -> u8 {
        AhbSramAccessDisableSram29If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApDevClk {
    #[doc = "Under hardware control."]
    UNDER_HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED_HIGH = 0x01,
}
impl ApDevClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApDevClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApDevClk {
    #[inline(always)]
    fn from(val: u8) -> ApDevClk {
        ApDevClk::from_bits(val)
    }
}
impl From<ApDevClk> for u8 {
    #[inline(always)]
    fn from(val: ApDevClk) -> u8 {
        ApDevClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ApHostClk {
    #[doc = "Under hardware control."]
    UNDER_HW_CTRL = 0x0,
    #[doc = "Forced high."]
    FORCED_HIGH = 0x01,
}
impl ApHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ApHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ApHostClk {
    #[inline(always)]
    fn from(val: u8) -> ApHostClk {
        ApHostClk::from_bits(val)
    }
}
impl From<ApHostClk> for u8 {
    #[inline(always)]
    fn from(val: ApHostClk) -> u8 {
        ApHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autoclkgateoverride0Dmac0 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl Autoclkgateoverride0Dmac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autoclkgateoverride0Dmac0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autoclkgateoverride0Dmac0 {
    #[inline(always)]
    fn from(val: u8) -> Autoclkgateoverride0Dmac0 {
        Autoclkgateoverride0Dmac0::from_bits(val)
    }
}
impl From<Autoclkgateoverride0Dmac0> for u8 {
    #[inline(always)]
    fn from(val: Autoclkgateoverride0Dmac0) -> u8 {
        Autoclkgateoverride0Dmac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autoclkgateoverride0Dmac1 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl Autoclkgateoverride0Dmac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autoclkgateoverride0Dmac1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autoclkgateoverride0Dmac1 {
    #[inline(always)]
    fn from(val: u8) -> Autoclkgateoverride0Dmac1 {
        Autoclkgateoverride0Dmac1::from_bits(val)
    }
}
impl From<Autoclkgateoverride0Dmac1> for u8 {
    #[inline(always)]
    fn from(val: Autoclkgateoverride0Dmac1) -> u8 {
        Autoclkgateoverride0Dmac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Casper {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl Casper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Casper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Casper {
    #[inline(always)]
    fn from(val: u8) -> Casper {
        Casper::from_bits(val)
    }
}
impl From<Casper> for u8 {
    #[inline(always)]
    fn from(val: Casper) -> u8 {
        Casper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkgateoverride0Acmp {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Clkgateoverride0Acmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkgateoverride0Acmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkgateoverride0Acmp {
    #[inline(always)]
    fn from(val: u8) -> Clkgateoverride0Acmp {
        Clkgateoverride0Acmp::from_bits(val)
    }
}
impl From<Clkgateoverride0Acmp> for u8 {
    #[inline(always)]
    fn from(val: Clkgateoverride0Acmp) -> u8 {
        Clkgateoverride0Acmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkgateoverride0Mu {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Clkgateoverride0Mu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkgateoverride0Mu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkgateoverride0Mu {
    #[inline(always)]
    fn from(val: u8) -> Clkgateoverride0Mu {
        Clkgateoverride0Mu::from_bits(val)
    }
}
impl From<Clkgateoverride0Mu> for u8 {
    #[inline(always)]
    fn from(val: Clkgateoverride0Mu) -> u8 {
        Clkgateoverride0Mu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcEngine {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl CrcEngine {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcEngine {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcEngine {
    #[inline(always)]
    fn from(val: u8) -> CrcEngine {
        CrcEngine::from_bits(val)
    }
}
impl From<CrcEngine> for u8 {
    #[inline(always)]
    fn from(val: CrcEngine) -> u8 {
        CrcEngine::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesDbgen {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesDbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesDbgen {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesDbgen {
        DbgFeaturesDbgen::from_bits(val)
    }
}
impl From<DbgFeaturesDbgen> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesDbgen) -> u8 {
        DbgFeaturesDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesDpDbgen {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesDpDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesDpDbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesDpDbgen {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesDpDbgen {
        DbgFeaturesDpDbgen::from_bits(val)
    }
}
impl From<DbgFeaturesDpDbgen> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesDpDbgen) -> u8 {
        DbgFeaturesDpDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesDpNiden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesDpNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesDpNiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesDpNiden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesDpNiden {
        DbgFeaturesDpNiden::from_bits(val)
    }
}
impl From<DbgFeaturesDpNiden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesDpNiden) -> u8 {
        DbgFeaturesDpNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesDpSpiden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesDpSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesDpSpiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesDpSpiden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesDpSpiden {
        DbgFeaturesDpSpiden::from_bits(val)
    }
}
impl From<DbgFeaturesDpSpiden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesDpSpiden) -> u8 {
        DbgFeaturesDpSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesDpSpniden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesDpSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesDpSpniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesDpSpniden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesDpSpniden {
        DbgFeaturesDpSpniden::from_bits(val)
    }
}
impl From<DbgFeaturesDpSpniden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesDpSpniden) -> u8 {
        DbgFeaturesDpSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesNiden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesNiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesNiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesNiden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesNiden {
        DbgFeaturesNiden::from_bits(val)
    }
}
impl From<DbgFeaturesNiden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesNiden) -> u8 {
        DbgFeaturesNiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesSpiden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesSpiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesSpiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesSpiden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesSpiden {
        DbgFeaturesSpiden::from_bits(val)
    }
}
impl From<DbgFeaturesSpiden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesSpiden) -> u8 {
        DbgFeaturesSpiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DbgFeaturesSpniden {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "disabled"]
    DISBALED1 = 0x01,
    #[doc = "disabled"]
    DISABLED2 = 0x02,
    #[doc = "disabled"]
    DISABLED3 = 0x03,
}
impl DbgFeaturesSpniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DbgFeaturesSpniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DbgFeaturesSpniden {
    #[inline(always)]
    fn from(val: u8) -> DbgFeaturesSpniden {
        DbgFeaturesSpniden::from_bits(val)
    }
}
impl From<DbgFeaturesSpniden> for u8 {
    #[inline(always)]
    fn from(val: DbgFeaturesSpniden) -> u8 {
        DbgFeaturesSpniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeepPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl DeepPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeepPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeepPd {
    #[inline(always)]
    fn from(val: u8) -> DeepPd {
        DeepPd::from_bits(val)
    }
}
impl From<DeepPd> for u8 {
    #[inline(always)]
    fn from(val: DeepPd) -> u8 {
        DeepPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DevNeedClkst {
    #[doc = "low"]
    LOW = 0x0,
    #[doc = "high"]
    HIGH = 0x01,
}
impl DevNeedClkst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DevNeedClkst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DevNeedClkst {
    #[inline(always)]
    fn from(val: u8) -> DevNeedClkst {
        DevNeedClkst::from_bits(val)
    }
}
impl From<DevNeedClkst> for u8 {
    #[inline(always)]
    fn from(val: DevNeedClkst) -> u8 {
        DevNeedClkst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspFlexspiAccessDisable {
    #[doc = "Enable DSP access to FLEXSPI"]
    ENABLE = 0x0,
    #[doc = "Disable DSP access to FLEXSPI"]
    DISABLE = 0x01,
}
impl DspFlexspiAccessDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspFlexspiAccessDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspFlexspiAccessDisable {
    #[inline(always)]
    fn from(val: u8) -> DspFlexspiAccessDisable {
        DspFlexspiAccessDisable::from_bits(val)
    }
}
impl From<DspFlexspiAccessDisable> for u8 {
    #[inline(always)]
    fn from(val: DspFlexspiAccessDisable) -> u8 {
        DspFlexspiAccessDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram00If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram00If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram00If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram00If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram00If {
        DspSramAccessDisableSram00If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram00If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram00If) -> u8 {
        DspSramAccessDisableSram00If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram01If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram01If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram01If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram01If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram01If {
        DspSramAccessDisableSram01If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram01If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram01If) -> u8 {
        DspSramAccessDisableSram01If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram02If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram02If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram02If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram02If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram02If {
        DspSramAccessDisableSram02If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram02If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram02If) -> u8 {
        DspSramAccessDisableSram02If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram03If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram03If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram03If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram03If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram03If {
        DspSramAccessDisableSram03If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram03If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram03If) -> u8 {
        DspSramAccessDisableSram03If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram04If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram04If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram04If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram04If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram04If {
        DspSramAccessDisableSram04If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram04If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram04If) -> u8 {
        DspSramAccessDisableSram04If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram05If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram05If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram05If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram05If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram05If {
        DspSramAccessDisableSram05If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram05If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram05If) -> u8 {
        DspSramAccessDisableSram05If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram06If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram06If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram06If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram06If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram06If {
        DspSramAccessDisableSram06If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram06If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram06If) -> u8 {
        DspSramAccessDisableSram06If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram07If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram07If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram07If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram07If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram07If {
        DspSramAccessDisableSram07If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram07If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram07If) -> u8 {
        DspSramAccessDisableSram07If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram08If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram08If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram08If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram08If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram08If {
        DspSramAccessDisableSram08If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram08If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram08If) -> u8 {
        DspSramAccessDisableSram08If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram09If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram09If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram09If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram09If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram09If {
        DspSramAccessDisableSram09If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram09If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram09If) -> u8 {
        DspSramAccessDisableSram09If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram10If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram10If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram10If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram10If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram10If {
        DspSramAccessDisableSram10If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram10If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram10If) -> u8 {
        DspSramAccessDisableSram10If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram11If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram11If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram11If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram11If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram11If {
        DspSramAccessDisableSram11If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram11If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram11If) -> u8 {
        DspSramAccessDisableSram11If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram12If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram12If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram12If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram12If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram12If {
        DspSramAccessDisableSram12If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram12If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram12If) -> u8 {
        DspSramAccessDisableSram12If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram13If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram13If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram13If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram13If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram13If {
        DspSramAccessDisableSram13If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram13If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram13If) -> u8 {
        DspSramAccessDisableSram13If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram14If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram14If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram14If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram14If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram14If {
        DspSramAccessDisableSram14If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram14If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram14If) -> u8 {
        DspSramAccessDisableSram14If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram15If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram15If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram15If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram15If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram15If {
        DspSramAccessDisableSram15If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram15If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram15If) -> u8 {
        DspSramAccessDisableSram15If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram16If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram16If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram16If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram16If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram16If {
        DspSramAccessDisableSram16If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram16If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram16If) -> u8 {
        DspSramAccessDisableSram16If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram17If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram17If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram17If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram17If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram17If {
        DspSramAccessDisableSram17If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram17If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram17If) -> u8 {
        DspSramAccessDisableSram17If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram18If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram18If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram18If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram18If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram18If {
        DspSramAccessDisableSram18If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram18If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram18If) -> u8 {
        DspSramAccessDisableSram18If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram19If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram19If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram19If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram19If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram19If {
        DspSramAccessDisableSram19If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram19If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram19If) -> u8 {
        DspSramAccessDisableSram19If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram20If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram20If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram20If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram20If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram20If {
        DspSramAccessDisableSram20If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram20If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram20If) -> u8 {
        DspSramAccessDisableSram20If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram21If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram21If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram21If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram21If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram21If {
        DspSramAccessDisableSram21If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram21If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram21If) -> u8 {
        DspSramAccessDisableSram21If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram22If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram22If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram22If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram22If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram22If {
        DspSramAccessDisableSram22If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram22If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram22If) -> u8 {
        DspSramAccessDisableSram22If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram23If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram23If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram23If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram23If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram23If {
        DspSramAccessDisableSram23If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram23If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram23If) -> u8 {
        DspSramAccessDisableSram23If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram24If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram24If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram24If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram24If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram24If {
        DspSramAccessDisableSram24If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram24If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram24If) -> u8 {
        DspSramAccessDisableSram24If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram25If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram25If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram25If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram25If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram25If {
        DspSramAccessDisableSram25If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram25If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram25If) -> u8 {
        DspSramAccessDisableSram25If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram26If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram26If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram26If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram26If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram26If {
        DspSramAccessDisableSram26If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram26If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram26If) -> u8 {
        DspSramAccessDisableSram26If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram27If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram27If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram27If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram27If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram27If {
        DspSramAccessDisableSram27If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram27If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram27If) -> u8 {
        DspSramAccessDisableSram27If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram28If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram28If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram28If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram28If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram28If {
        DspSramAccessDisableSram28If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram28If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram28If) -> u8 {
        DspSramAccessDisableSram28If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspSramAccessDisableSram29If {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "disable"]
    DISABLED = 0x01,
}
impl DspSramAccessDisableSram29If {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspSramAccessDisableSram29If {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspSramAccessDisableSram29If {
    #[inline(always)]
    fn from(val: u8) -> DspSramAccessDisableSram29If {
        DspSramAccessDisableSram29If::from_bits(val)
    }
}
impl From<DspSramAccessDisableSram29If> for u8 {
    #[inline(always)]
    fn from(val: DspSramAccessDisableSram29If) -> u8 {
        DspSramAccessDisableSram29If::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dspstall {
    #[doc = "Run (Normal) Mode."]
    RUN_MODE = 0x0,
    #[doc = "Stall Mode."]
    STALL_MODE = 0x01,
}
impl Dspstall {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dspstall {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dspstall {
    #[inline(always)]
    fn from(val: u8) -> Dspstall {
        Dspstall::from_bits(val)
    }
}
impl From<Dspstall> for u8 {
    #[inline(always)]
    fn from(val: Dspstall) -> u8 {
        Dspstall::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fbbkeepst {
    #[doc = "Use value of FBB_PD in PDRUNCFG on wakeup"]
    FBBKEEPST_0 = 0x0,
    #[doc = "Copy PDSLEEPCFG FBB_PD value to PDRUNCFG FBB_PD on wakeup to keep FBB state"]
    FBBKEEPST_1 = 0x01,
}
impl Fbbkeepst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fbbkeepst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fbbkeepst {
    #[inline(always)]
    fn from(val: u8) -> Fbbkeepst {
        Fbbkeepst::from_bits(val)
    }
}
impl From<Fbbkeepst> for u8 {
    #[inline(always)]
    fn from(val: Fbbkeepst) -> u8 {
        Fbbkeepst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HostNeedClkst {
    #[doc = "low"]
    LOW = 0x0,
    #[doc = "high"]
    HIGH = 0x01,
}
impl HostNeedClkst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HostNeedClkst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HostNeedClkst {
    #[inline(always)]
    fn from(val: u8) -> HostNeedClkst {
        HostNeedClkst::from_bits(val)
    }
}
impl From<HostNeedClkst> for u8 {
    #[inline(always)]
    fn from(val: HostNeedClkst) -> u8 {
        HostNeedClkst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HsDevWakeupN {
    #[doc = "Forces USB0 PHY to wake-up."]
    FORCE_TO_WAKEUP = 0x0,
    #[doc = "Normal USB0 PHY behavior."]
    NORMAL = 0x01,
}
impl HsDevWakeupN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HsDevWakeupN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HsDevWakeupN {
    #[inline(always)]
    fn from(val: u8) -> HsDevWakeupN {
        HsDevWakeupN::from_bits(val)
    }
}
impl From<HsDevWakeupN> for u8 {
    #[inline(always)]
    fn from(val: HsDevWakeupN) -> u8 {
        HsDevWakeupN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MainclkShutoff {
    #[doc = "Clocks enabled"]
    CLOCK_ENABLED = 0x0,
    #[doc = "Clocks gated"]
    CLOCK_GATED = 0x01,
}
impl MainclkShutoff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkShutoff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkShutoff {
    #[inline(always)]
    fn from(val: u8) -> MainclkShutoff {
        MainclkShutoff::from_bits(val)
    }
}
impl From<MainclkShutoff> for u8 {
    #[inline(always)]
    fn from(val: MainclkShutoff) -> u8 {
        MainclkShutoff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AcmpPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AcmpPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AcmpPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AcmpPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AcmpPd {
        Pdruncfg0AcmpPd::from_bits(val)
    }
}
impl From<Pdruncfg0AcmpPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AcmpPd) -> u8 {
        Pdruncfg0AcmpPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AdcLp {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AdcLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AdcLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AdcLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AdcLp {
        Pdruncfg0AdcLp::from_bits(val)
    }
}
impl From<Pdruncfg0AdcLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AdcLp) -> u8 {
        Pdruncfg0AdcLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AdcPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AdcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AdcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AdcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AdcPd {
        Pdruncfg0AdcPd::from_bits(val)
    }
}
impl From<Pdruncfg0AdcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AdcPd) -> u8 {
        Pdruncfg0AdcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AdctempsnsPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AdctempsnsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AdctempsnsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AdctempsnsPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AdctempsnsPd {
        Pdruncfg0AdctempsnsPd::from_bits(val)
    }
}
impl From<Pdruncfg0AdctempsnsPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AdctempsnsPd) -> u8 {
        Pdruncfg0AdctempsnsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AudpllanaPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AudpllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AudpllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AudpllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AudpllanaPd {
        Pdruncfg0AudpllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0AudpllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AudpllanaPd) -> u8 {
        Pdruncfg0AudpllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0AudpllldoPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0AudpllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0AudpllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0AudpllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0AudpllldoPd {
        Pdruncfg0AudpllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0AudpllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0AudpllldoPd) -> u8 {
        Pdruncfg0AudpllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAcmpPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAcmpPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAcmpPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAcmpPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAcmpPd {
        Pdruncfg0ClrAcmpPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAcmpPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAcmpPd) -> u8 {
        Pdruncfg0ClrAcmpPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAdcLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAdcLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAdcLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAdcLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAdcLp {
        Pdruncfg0ClrAdcLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAdcLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAdcLp) -> u8 {
        Pdruncfg0ClrAdcLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAdcPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAdcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAdcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAdcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAdcPd {
        Pdruncfg0ClrAdcPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAdcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAdcPd) -> u8 {
        Pdruncfg0ClrAdcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAdctempsnsPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAdctempsnsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAdctempsnsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAdctempsnsPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAdctempsnsPd {
        Pdruncfg0ClrAdctempsnsPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAdctempsnsPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAdctempsnsPd) -> u8 {
        Pdruncfg0ClrAdctempsnsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAudpllanaPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAudpllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAudpllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAudpllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAudpllanaPd {
        Pdruncfg0ClrAudpllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAudpllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAudpllanaPd) -> u8 {
        Pdruncfg0ClrAudpllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrAudpllldoPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrAudpllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrAudpllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrAudpllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrAudpllldoPd {
        Pdruncfg0ClrAudpllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrAudpllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrAudpllldoPd) -> u8 {
        Pdruncfg0ClrAudpllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrFbbPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrFbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrFbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrFbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrFbbPd {
        Pdruncfg0ClrFbbPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrFbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrFbbPd) -> u8 {
        Pdruncfg0ClrFbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrFfroPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrFfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrFfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrFfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrFfroPd {
        Pdruncfg0ClrFfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrFfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrFfroPd) -> u8 {
        Pdruncfg0ClrFfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHspad0RefPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHspad0RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHspad0RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHspad0RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHspad0RefPd {
        Pdruncfg0ClrHspad0RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHspad0RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHspad0RefPd) -> u8 {
        Pdruncfg0ClrHspad0RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHspad0VdetLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHspad0VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHspad0VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHspad0VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHspad0VdetLp {
        Pdruncfg0ClrHspad0VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHspad0VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHspad0VdetLp) -> u8 {
        Pdruncfg0ClrHspad0VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHspad2RefPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHspad2RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHspad2RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHspad2RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHspad2RefPd {
        Pdruncfg0ClrHspad2RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHspad2RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHspad2RefPd) -> u8 {
        Pdruncfg0ClrHspad2RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHspad2VdetLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHspad2VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHspad2VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHspad2VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHspad2VdetLp {
        Pdruncfg0ClrHspad2VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHspad2VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHspad2VdetLp) -> u8 {
        Pdruncfg0ClrHspad2VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHvd1v8Pd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHvd1v8Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHvd1v8Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHvd1v8Pd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHvd1v8Pd {
        Pdruncfg0ClrHvd1v8Pd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHvd1v8Pd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHvd1v8Pd) -> u8 {
        Pdruncfg0ClrHvd1v8Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrHvdcorePd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrHvdcorePd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrHvdcorePd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrHvdcorePd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrHvdcorePd {
        Pdruncfg0ClrHvdcorePd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrHvdcorePd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrHvdcorePd) -> u8 {
        Pdruncfg0ClrHvdcorePd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrLposcPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrLposcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrLposcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrLposcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrLposcPd {
        Pdruncfg0ClrLposcPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrLposcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrLposcPd) -> u8 {
        Pdruncfg0ClrLposcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrLvdcoreLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrLvdcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrLvdcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrLvdcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrLvdcoreLp {
        Pdruncfg0ClrLvdcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrLvdcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrLvdcoreLp) -> u8 {
        Pdruncfg0ClrLvdcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrPmcrefLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrPmcrefLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrPmcrefLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrPmcrefLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrPmcrefLp {
        Pdruncfg0ClrPmcrefLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrPmcrefLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrPmcrefLp) -> u8 {
        Pdruncfg0ClrPmcrefLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrPmicMode0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrPmicMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrPmicMode0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrPmicMode0 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrPmicMode0 {
        Pdruncfg0ClrPmicMode0::from_bits(val)
    }
}
impl From<Pdruncfg0ClrPmicMode0> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrPmicMode0) -> u8 {
        Pdruncfg0ClrPmicMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrPmicMode1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrPmicMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrPmicMode1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrPmicMode1 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrPmicMode1 {
        Pdruncfg0ClrPmicMode1::from_bits(val)
    }
}
impl From<Pdruncfg0ClrPmicMode1> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrPmicMode1) -> u8 {
        Pdruncfg0ClrPmicMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrPorcoreLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrPorcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrPorcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrPorcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrPorcoreLp {
        Pdruncfg0ClrPorcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrPorcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrPorcoreLp) -> u8 {
        Pdruncfg0ClrPorcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrRbbPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrRbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrRbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrRbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrRbbPd {
        Pdruncfg0ClrRbbPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrRbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrRbbPd) -> u8 {
        Pdruncfg0ClrRbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrSfroPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrSfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrSfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrSfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrSfroPd {
        Pdruncfg0ClrSfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrSfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrSfroPd) -> u8 {
        Pdruncfg0ClrSfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrSyspllanaPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrSyspllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrSyspllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrSyspllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrSyspllanaPd {
        Pdruncfg0ClrSyspllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrSyspllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrSyspllanaPd) -> u8 {
        Pdruncfg0ClrSyspllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrSyspllldoPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrSyspllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrSyspllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrSyspllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrSyspllldoPd {
        Pdruncfg0ClrSyspllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrSyspllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrSyspllldoPd) -> u8 {
        Pdruncfg0ClrSyspllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrSysxtalPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrSysxtalPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrSysxtalPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrSysxtalPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrSysxtalPd {
        Pdruncfg0ClrSysxtalPd::from_bits(val)
    }
}
impl From<Pdruncfg0ClrSysxtalPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrSysxtalPd) -> u8 {
        Pdruncfg0ClrSysxtalPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0ClrVddcoreregLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG0 Bit"]
    CLR_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0ClrVddcoreregLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0ClrVddcoreregLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0ClrVddcoreregLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0ClrVddcoreregLp {
        Pdruncfg0ClrVddcoreregLp::from_bits(val)
    }
}
impl From<Pdruncfg0ClrVddcoreregLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0ClrVddcoreregLp) -> u8 {
        Pdruncfg0ClrVddcoreregLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0FfroPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0FfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0FfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0FfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0FfroPd {
        Pdruncfg0FfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0FfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0FfroPd) -> u8 {
        Pdruncfg0FfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0Hspad0RefPd {
    #[doc = "High Speed Pad VREF Enabled"]
    ENABLED = 0x0,
    #[doc = "High Speed Pad VREF in Power Down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0Hspad0RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0Hspad0RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0Hspad0RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0Hspad0RefPd {
        Pdruncfg0Hspad0RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0Hspad0RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0Hspad0RefPd) -> u8 {
        Pdruncfg0Hspad0RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0Hspad0VdetLp {
    #[doc = "High Speed Pad VDET in Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    SLEEP_MODE = 0x01,
}
impl Pdruncfg0Hspad0VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0Hspad0VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0Hspad0VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0Hspad0VdetLp {
        Pdruncfg0Hspad0VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0Hspad0VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0Hspad0VdetLp) -> u8 {
        Pdruncfg0Hspad0VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0Hspad2RefPd {
    #[doc = "High Speed Pad VREF Enabled"]
    ENABLED = 0x0,
    #[doc = "High Speed Pad VREF in Power Down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0Hspad2RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0Hspad2RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0Hspad2RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0Hspad2RefPd {
        Pdruncfg0Hspad2RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0Hspad2RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0Hspad2RefPd) -> u8 {
        Pdruncfg0Hspad2RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0Hspad2VdetLp {
    #[doc = "High Speed Pad VDET in Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    SLEEP_MODE = 0x01,
}
impl Pdruncfg0Hspad2VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0Hspad2VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0Hspad2VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0Hspad2VdetLp {
        Pdruncfg0Hspad2VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0Hspad2VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0Hspad2VdetLp) -> u8 {
        Pdruncfg0Hspad2VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0Hvd1v8Pd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0Hvd1v8Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0Hvd1v8Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0Hvd1v8Pd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0Hvd1v8Pd {
        Pdruncfg0Hvd1v8Pd::from_bits(val)
    }
}
impl From<Pdruncfg0Hvd1v8Pd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0Hvd1v8Pd) -> u8 {
        Pdruncfg0Hvd1v8Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0HvdcorePd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0HvdcorePd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0HvdcorePd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0HvdcorePd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0HvdcorePd {
        Pdruncfg0HvdcorePd::from_bits(val)
    }
}
impl From<Pdruncfg0HvdcorePd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0HvdcorePd) -> u8 {
        Pdruncfg0HvdcorePd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0LposcPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0LposcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0LposcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0LposcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0LposcPd {
        Pdruncfg0LposcPd::from_bits(val)
    }
}
impl From<Pdruncfg0LposcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0LposcPd) -> u8 {
        Pdruncfg0LposcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0LvdcoreLp {
    #[doc = "LVD0V85 HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LVD0V85 LP Mode."]
    LP_MODE = 0x01,
}
impl Pdruncfg0LvdcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0LvdcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0LvdcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0LvdcoreLp {
        Pdruncfg0LvdcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0LvdcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0LvdcoreLp) -> u8 {
        Pdruncfg0LvdcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0PmcrefLp {
    #[doc = "PMCREF HP Mode"]
    HP_MODE = 0x0,
    #[doc = "PMCREF LP Mode"]
    LP_MODE = 0x01,
}
impl Pdruncfg0PmcrefLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0PmcrefLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0PmcrefLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0PmcrefLp {
        Pdruncfg0PmcrefLp::from_bits(val)
    }
}
impl From<Pdruncfg0PmcrefLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0PmcrefLp) -> u8 {
        Pdruncfg0PmcrefLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0PmicMode0 {
    #[doc = "Set Mode0 to 0."]
    SET_MODE0_0 = 0x0,
    #[doc = "Set Mode0 to 1."]
    SET_MODE0_1 = 0x01,
}
impl Pdruncfg0PmicMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0PmicMode0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0PmicMode0 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0PmicMode0 {
        Pdruncfg0PmicMode0::from_bits(val)
    }
}
impl From<Pdruncfg0PmicMode0> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0PmicMode0) -> u8 {
        Pdruncfg0PmicMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0PmicMode1 {
    #[doc = "Set Mode1 to 0."]
    SET_MODE1_0 = 0x0,
    #[doc = "Set Mode1 to 1."]
    SET_MODE1_1 = 0x01,
}
impl Pdruncfg0PmicMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0PmicMode1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0PmicMode1 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0PmicMode1 {
        Pdruncfg0PmicMode1::from_bits(val)
    }
}
impl From<Pdruncfg0PmicMode1> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0PmicMode1) -> u8 {
        Pdruncfg0PmicMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0PorcoreLp {
    #[doc = "LVD0V6 HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LVD0V6 LP Mode"]
    LP_MODE = 0x01,
}
impl Pdruncfg0PorcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0PorcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0PorcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0PorcoreLp {
        Pdruncfg0PorcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0PorcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0PorcoreLp) -> u8 {
        Pdruncfg0PorcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAcmpPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAcmpPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAcmpPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAcmpPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAcmpPd {
        Pdruncfg0SetAcmpPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetAcmpPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAcmpPd) -> u8 {
        Pdruncfg0SetAcmpPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAdcLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAdcLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAdcLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAdcLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAdcLp {
        Pdruncfg0SetAdcLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetAdcLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAdcLp) -> u8 {
        Pdruncfg0SetAdcLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAdcPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAdcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAdcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAdcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAdcPd {
        Pdruncfg0SetAdcPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetAdcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAdcPd) -> u8 {
        Pdruncfg0SetAdcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAdctempsnsPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAdctempsnsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAdctempsnsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAdctempsnsPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAdctempsnsPd {
        Pdruncfg0SetAdctempsnsPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetAdctempsnsPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAdctempsnsPd) -> u8 {
        Pdruncfg0SetAdctempsnsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAudpllanaPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAudpllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAudpllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAudpllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAudpllanaPd {
        Pdruncfg0SetAudpllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetAudpllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAudpllanaPd) -> u8 {
        Pdruncfg0SetAudpllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetAudpllldoPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetAudpllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetAudpllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetAudpllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetAudpllldoPd {
        Pdruncfg0SetAudpllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetAudpllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetAudpllldoPd) -> u8 {
        Pdruncfg0SetAudpllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetFbbPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetFbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetFbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetFbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetFbbPd {
        Pdruncfg0SetFbbPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetFbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetFbbPd) -> u8 {
        Pdruncfg0SetFbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetFfroPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetFfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetFfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetFfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetFfroPd {
        Pdruncfg0SetFfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetFfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetFfroPd) -> u8 {
        Pdruncfg0SetFfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHspad0RefPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHspad0RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHspad0RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHspad0RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHspad0RefPd {
        Pdruncfg0SetHspad0RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetHspad0RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHspad0RefPd) -> u8 {
        Pdruncfg0SetHspad0RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHspad0VdetLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHspad0VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHspad0VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHspad0VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHspad0VdetLp {
        Pdruncfg0SetHspad0VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetHspad0VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHspad0VdetLp) -> u8 {
        Pdruncfg0SetHspad0VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHspad2RefPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHspad2RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHspad2RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHspad2RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHspad2RefPd {
        Pdruncfg0SetHspad2RefPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetHspad2RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHspad2RefPd) -> u8 {
        Pdruncfg0SetHspad2RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHspad2VdetLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHspad2VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHspad2VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHspad2VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHspad2VdetLp {
        Pdruncfg0SetHspad2VdetLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetHspad2VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHspad2VdetLp) -> u8 {
        Pdruncfg0SetHspad2VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHvd1v8Pd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHvd1v8Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHvd1v8Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHvd1v8Pd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHvd1v8Pd {
        Pdruncfg0SetHvd1v8Pd::from_bits(val)
    }
}
impl From<Pdruncfg0SetHvd1v8Pd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHvd1v8Pd) -> u8 {
        Pdruncfg0SetHvd1v8Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetHvdcorePd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetHvdcorePd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetHvdcorePd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetHvdcorePd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetHvdcorePd {
        Pdruncfg0SetHvdcorePd::from_bits(val)
    }
}
impl From<Pdruncfg0SetHvdcorePd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetHvdcorePd) -> u8 {
        Pdruncfg0SetHvdcorePd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetLposcPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetLposcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetLposcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetLposcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetLposcPd {
        Pdruncfg0SetLposcPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetLposcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetLposcPd) -> u8 {
        Pdruncfg0SetLposcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetLvdcoreLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetLvdcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetLvdcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetLvdcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetLvdcoreLp {
        Pdruncfg0SetLvdcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetLvdcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetLvdcoreLp) -> u8 {
        Pdruncfg0SetLvdcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetPmcrefLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetPmcrefLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetPmcrefLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetPmcrefLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetPmcrefLp {
        Pdruncfg0SetPmcrefLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetPmcrefLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetPmcrefLp) -> u8 {
        Pdruncfg0SetPmcrefLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetPmicMode0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetPmicMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetPmicMode0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetPmicMode0 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetPmicMode0 {
        Pdruncfg0SetPmicMode0::from_bits(val)
    }
}
impl From<Pdruncfg0SetPmicMode0> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetPmicMode0) -> u8 {
        Pdruncfg0SetPmicMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetPmicMode1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetPmicMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetPmicMode1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetPmicMode1 {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetPmicMode1 {
        Pdruncfg0SetPmicMode1::from_bits(val)
    }
}
impl From<Pdruncfg0SetPmicMode1> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetPmicMode1) -> u8 {
        Pdruncfg0SetPmicMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetPorcoreLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetPorcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetPorcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetPorcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetPorcoreLp {
        Pdruncfg0SetPorcoreLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetPorcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetPorcoreLp) -> u8 {
        Pdruncfg0SetPorcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetRbbPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetRbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetRbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetRbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetRbbPd {
        Pdruncfg0SetRbbPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetRbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetRbbPd) -> u8 {
        Pdruncfg0SetRbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetSfroPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetSfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetSfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetSfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetSfroPd {
        Pdruncfg0SetSfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetSfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetSfroPd) -> u8 {
        Pdruncfg0SetSfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetSyspllanaPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetSyspllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetSyspllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetSyspllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetSyspllanaPd {
        Pdruncfg0SetSyspllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetSyspllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetSyspllanaPd) -> u8 {
        Pdruncfg0SetSyspllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetSyspllldoPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetSyspllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetSyspllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetSyspllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetSyspllldoPd {
        Pdruncfg0SetSyspllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetSyspllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetSyspllldoPd) -> u8 {
        Pdruncfg0SetSyspllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetSysxtalPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetSysxtalPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetSysxtalPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetSysxtalPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetSysxtalPd {
        Pdruncfg0SetSysxtalPd::from_bits(val)
    }
}
impl From<Pdruncfg0SetSysxtalPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetSysxtalPd) -> u8 {
        Pdruncfg0SetSysxtalPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SetVddcoreregLp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG0 Bit"]
    SET_PDRUNCFG0 = 0x01,
}
impl Pdruncfg0SetVddcoreregLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SetVddcoreregLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SetVddcoreregLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SetVddcoreregLp {
        Pdruncfg0SetVddcoreregLp::from_bits(val)
    }
}
impl From<Pdruncfg0SetVddcoreregLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SetVddcoreregLp) -> u8 {
        Pdruncfg0SetVddcoreregLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SfroPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0SfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SfroPd {
        Pdruncfg0SfroPd::from_bits(val)
    }
}
impl From<Pdruncfg0SfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SfroPd) -> u8 {
        Pdruncfg0SfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SyspllanaPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0SyspllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SyspllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SyspllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SyspllanaPd {
        Pdruncfg0SyspllanaPd::from_bits(val)
    }
}
impl From<Pdruncfg0SyspllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SyspllanaPd) -> u8 {
        Pdruncfg0SyspllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SyspllldoPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0SyspllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SyspllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SyspllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SyspllldoPd {
        Pdruncfg0SyspllldoPd::from_bits(val)
    }
}
impl From<Pdruncfg0SyspllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SyspllldoPd) -> u8 {
        Pdruncfg0SyspllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0SysxtalPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg0SysxtalPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0SysxtalPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0SysxtalPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0SysxtalPd {
        Pdruncfg0SysxtalPd::from_bits(val)
    }
}
impl From<Pdruncfg0SysxtalPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0SysxtalPd) -> u8 {
        Pdruncfg0SysxtalPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg0VddcoreregLp {
    #[doc = "VDDCOREREG HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LP Mode"]
    LP_MODE = 0x01,
}
impl Pdruncfg0VddcoreregLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg0VddcoreregLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg0VddcoreregLp {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg0VddcoreregLp {
        Pdruncfg0VddcoreregLp::from_bits(val)
    }
}
impl From<Pdruncfg0VddcoreregLp> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg0VddcoreregLp) -> u8 {
        Pdruncfg0VddcoreregLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1CasperSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1CasperSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1CasperSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1CasperSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1CasperSramApd {
        Pdruncfg1CasperSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1CasperSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1CasperSramApd) -> u8 {
        Pdruncfg1CasperSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1CasperSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1CasperSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1CasperSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1CasperSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1CasperSramPpd {
        Pdruncfg1CasperSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1CasperSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1CasperSramPpd) -> u8 {
        Pdruncfg1CasperSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrCasperSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrCasperSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrCasperSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrCasperSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrCasperSramApd {
        Pdruncfg1ClrCasperSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrCasperSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrCasperSramApd) -> u8 {
        Pdruncfg1ClrCasperSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrCasperSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrCasperSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrCasperSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrCasperSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrCasperSramPpd {
        Pdruncfg1ClrCasperSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrCasperSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrCasperSramPpd) -> u8 {
        Pdruncfg1ClrCasperSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrDspcacheRegfApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrDspcacheRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrDspcacheRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrDspcacheRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrDspcacheRegfApd {
        Pdruncfg1ClrDspcacheRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrDspcacheRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrDspcacheRegfApd) -> u8 {
        Pdruncfg1ClrDspcacheRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrDspcacheRegfPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrDspcacheRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrDspcacheRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrDspcacheRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrDspcacheRegfPpd {
        Pdruncfg1ClrDspcacheRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrDspcacheRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrDspcacheRegfPpd) -> u8 {
        Pdruncfg1ClrDspcacheRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrDsptcmRegfApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrDsptcmRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrDsptcmRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrDsptcmRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrDsptcmRegfApd {
        Pdruncfg1ClrDsptcmRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrDsptcmRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrDsptcmRegfApd) -> u8 {
        Pdruncfg1ClrDsptcmRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrDsptcmRegfPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrDsptcmRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrDsptcmRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrDsptcmRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrDsptcmRegfPpd {
        Pdruncfg1ClrDsptcmRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrDsptcmRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrDsptcmRegfPpd) -> u8 {
        Pdruncfg1ClrDsptcmRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrFlexspiSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrFlexspiSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrFlexspiSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrFlexspiSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrFlexspiSramApd {
        Pdruncfg1ClrFlexspiSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrFlexspiSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrFlexspiSramApd) -> u8 {
        Pdruncfg1ClrFlexspiSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrFlexspiSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrFlexspiSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrFlexspiSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrFlexspiSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrFlexspiSramPpd {
        Pdruncfg1ClrFlexspiSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrFlexspiSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrFlexspiSramPpd) -> u8 {
        Pdruncfg1ClrFlexspiSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrPqSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrPqSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrPqSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrPqSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrPqSramApd {
        Pdruncfg1ClrPqSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrPqSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrPqSramApd) -> u8 {
        Pdruncfg1ClrPqSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrPqSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrPqSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrPqSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrPqSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrPqSramPpd {
        Pdruncfg1ClrPqSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrPqSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrPqSramPpd) -> u8 {
        Pdruncfg1ClrPqSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrRomPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrRomPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrRomPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrRomPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrRomPd {
        Pdruncfg1ClrRomPd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrRomPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrRomPd) -> u8 {
        Pdruncfg1ClrRomPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrSramSleep {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrSramSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrSramSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrSramSleep {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrSramSleep {
        Pdruncfg1ClrSramSleep::from_bits(val)
    }
}
impl From<Pdruncfg1ClrSramSleep> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrSramSleep) -> u8 {
        Pdruncfg1ClrSramSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsbhsSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsbhsSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsbhsSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsbhsSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsbhsSramApd {
        Pdruncfg1ClrUsbhsSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsbhsSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsbhsSramApd) -> u8 {
        Pdruncfg1ClrUsbhsSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsbhsSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsbhsSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsbhsSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsbhsSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsbhsSramPpd {
        Pdruncfg1ClrUsbhsSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsbhsSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsbhsSramPpd) -> u8 {
        Pdruncfg1ClrUsbhsSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsdhc0SramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsdhc0SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsdhc0SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsdhc0SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsdhc0SramApd {
        Pdruncfg1ClrUsdhc0SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsdhc0SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsdhc0SramApd) -> u8 {
        Pdruncfg1ClrUsdhc0SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsdhc0SramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsdhc0SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsdhc0SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsdhc0SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsdhc0SramPpd {
        Pdruncfg1ClrUsdhc0SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsdhc0SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsdhc0SramPpd) -> u8 {
        Pdruncfg1ClrUsdhc0SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsdhc1SramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsdhc1SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsdhc1SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsdhc1SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsdhc1SramApd {
        Pdruncfg1ClrUsdhc1SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsdhc1SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsdhc1SramApd) -> u8 {
        Pdruncfg1ClrUsdhc1SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1ClrUsdhc1SramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG1 Bit"]
    CLR_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1ClrUsdhc1SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1ClrUsdhc1SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1ClrUsdhc1SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1ClrUsdhc1SramPpd {
        Pdruncfg1ClrUsdhc1SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1ClrUsdhc1SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1ClrUsdhc1SramPpd) -> u8 {
        Pdruncfg1ClrUsdhc1SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1DspcacheRegfApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1DspcacheRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1DspcacheRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1DspcacheRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1DspcacheRegfApd {
        Pdruncfg1DspcacheRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1DspcacheRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1DspcacheRegfApd) -> u8 {
        Pdruncfg1DspcacheRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1DspcacheRegfPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1DspcacheRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1DspcacheRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1DspcacheRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1DspcacheRegfPpd {
        Pdruncfg1DspcacheRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1DspcacheRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1DspcacheRegfPpd) -> u8 {
        Pdruncfg1DspcacheRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1DsptcmRegfApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1DsptcmRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1DsptcmRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1DsptcmRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1DsptcmRegfApd {
        Pdruncfg1DsptcmRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1DsptcmRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1DsptcmRegfApd) -> u8 {
        Pdruncfg1DsptcmRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1DsptcmRegfPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1DsptcmRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1DsptcmRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1DsptcmRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1DsptcmRegfPpd {
        Pdruncfg1DsptcmRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1DsptcmRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1DsptcmRegfPpd) -> u8 {
        Pdruncfg1DsptcmRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1FlexspiSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1FlexspiSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1FlexspiSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1FlexspiSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1FlexspiSramApd {
        Pdruncfg1FlexspiSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1FlexspiSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1FlexspiSramApd) -> u8 {
        Pdruncfg1FlexspiSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1FlexspiSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1FlexspiSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1FlexspiSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1FlexspiSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1FlexspiSramPpd {
        Pdruncfg1FlexspiSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1FlexspiSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1FlexspiSramPpd) -> u8 {
        Pdruncfg1FlexspiSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1PqSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1PqSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1PqSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1PqSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1PqSramApd {
        Pdruncfg1PqSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1PqSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1PqSramApd) -> u8 {
        Pdruncfg1PqSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1PqSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1PqSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1PqSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1PqSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1PqSramPpd {
        Pdruncfg1PqSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1PqSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1PqSramPpd) -> u8 {
        Pdruncfg1PqSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1RomPd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1RomPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1RomPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1RomPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1RomPd {
        Pdruncfg1RomPd::from_bits(val)
    }
}
impl From<Pdruncfg1RomPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1RomPd) -> u8 {
        Pdruncfg1RomPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetCasperSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetCasperSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetCasperSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetCasperSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetCasperSramApd {
        Pdruncfg1SetCasperSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetCasperSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetCasperSramApd) -> u8 {
        Pdruncfg1SetCasperSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetCasperSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetCasperSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetCasperSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetCasperSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetCasperSramPpd {
        Pdruncfg1SetCasperSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetCasperSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetCasperSramPpd) -> u8 {
        Pdruncfg1SetCasperSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetDspcacheRegfApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetDspcacheRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetDspcacheRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetDspcacheRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetDspcacheRegfApd {
        Pdruncfg1SetDspcacheRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetDspcacheRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetDspcacheRegfApd) -> u8 {
        Pdruncfg1SetDspcacheRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetDspcacheRegfPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetDspcacheRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetDspcacheRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetDspcacheRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetDspcacheRegfPpd {
        Pdruncfg1SetDspcacheRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetDspcacheRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetDspcacheRegfPpd) -> u8 {
        Pdruncfg1SetDspcacheRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetDsptcmRegfApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetDsptcmRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetDsptcmRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetDsptcmRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetDsptcmRegfApd {
        Pdruncfg1SetDsptcmRegfApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetDsptcmRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetDsptcmRegfApd) -> u8 {
        Pdruncfg1SetDsptcmRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetDsptcmRegfPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetDsptcmRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetDsptcmRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetDsptcmRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetDsptcmRegfPpd {
        Pdruncfg1SetDsptcmRegfPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetDsptcmRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetDsptcmRegfPpd) -> u8 {
        Pdruncfg1SetDsptcmRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetFlexspiSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetFlexspiSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetFlexspiSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetFlexspiSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetFlexspiSramApd {
        Pdruncfg1SetFlexspiSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetFlexspiSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetFlexspiSramApd) -> u8 {
        Pdruncfg1SetFlexspiSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetFlexspiSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetFlexspiSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetFlexspiSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetFlexspiSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetFlexspiSramPpd {
        Pdruncfg1SetFlexspiSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetFlexspiSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetFlexspiSramPpd) -> u8 {
        Pdruncfg1SetFlexspiSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetPqSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetPqSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetPqSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetPqSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetPqSramApd {
        Pdruncfg1SetPqSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetPqSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetPqSramApd) -> u8 {
        Pdruncfg1SetPqSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetPqSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetPqSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetPqSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetPqSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetPqSramPpd {
        Pdruncfg1SetPqSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetPqSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetPqSramPpd) -> u8 {
        Pdruncfg1SetPqSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetRomPd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetRomPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetRomPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetRomPd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetRomPd {
        Pdruncfg1SetRomPd::from_bits(val)
    }
}
impl From<Pdruncfg1SetRomPd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetRomPd) -> u8 {
        Pdruncfg1SetRomPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetSramSleep {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetSramSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetSramSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetSramSleep {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetSramSleep {
        Pdruncfg1SetSramSleep::from_bits(val)
    }
}
impl From<Pdruncfg1SetSramSleep> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetSramSleep) -> u8 {
        Pdruncfg1SetSramSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsbhsSramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsbhsSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsbhsSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsbhsSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsbhsSramApd {
        Pdruncfg1SetUsbhsSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsbhsSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsbhsSramApd) -> u8 {
        Pdruncfg1SetUsbhsSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsbhsSramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsbhsSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsbhsSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsbhsSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsbhsSramPpd {
        Pdruncfg1SetUsbhsSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsbhsSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsbhsSramPpd) -> u8 {
        Pdruncfg1SetUsbhsSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsdhc0SramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsdhc0SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsdhc0SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsdhc0SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsdhc0SramApd {
        Pdruncfg1SetUsdhc0SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsdhc0SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsdhc0SramApd) -> u8 {
        Pdruncfg1SetUsdhc0SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsdhc0SramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsdhc0SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsdhc0SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsdhc0SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsdhc0SramPpd {
        Pdruncfg1SetUsdhc0SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsdhc0SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsdhc0SramPpd) -> u8 {
        Pdruncfg1SetUsdhc0SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsdhc1SramApd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsdhc1SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsdhc1SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsdhc1SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsdhc1SramApd {
        Pdruncfg1SetUsdhc1SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsdhc1SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsdhc1SramApd) -> u8 {
        Pdruncfg1SetUsdhc1SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SetUsdhc1SramPpd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG1 Bit"]
    SET_PDRUNCFG1 = 0x01,
}
impl Pdruncfg1SetUsdhc1SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SetUsdhc1SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SetUsdhc1SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SetUsdhc1SramPpd {
        Pdruncfg1SetUsdhc1SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1SetUsdhc1SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SetUsdhc1SramPpd) -> u8 {
        Pdruncfg1SetUsdhc1SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1SramSleep {
    #[doc = "RAM Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "RAM Sleep Mode."]
    SLEEP_MODE = 0x01,
}
impl Pdruncfg1SramSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1SramSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1SramSleep {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1SramSleep {
        Pdruncfg1SramSleep::from_bits(val)
    }
}
impl From<Pdruncfg1SramSleep> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1SramSleep) -> u8 {
        Pdruncfg1SramSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1UsbhsSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1UsbhsSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1UsbhsSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1UsbhsSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1UsbhsSramApd {
        Pdruncfg1UsbhsSramApd::from_bits(val)
    }
}
impl From<Pdruncfg1UsbhsSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1UsbhsSramApd) -> u8 {
        Pdruncfg1UsbhsSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1UsbhsSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1UsbhsSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1UsbhsSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1UsbhsSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1UsbhsSramPpd {
        Pdruncfg1UsbhsSramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1UsbhsSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1UsbhsSramPpd) -> u8 {
        Pdruncfg1UsbhsSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1Usdhc0SramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1Usdhc0SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1Usdhc0SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1Usdhc0SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1Usdhc0SramApd {
        Pdruncfg1Usdhc0SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1Usdhc0SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1Usdhc0SramApd) -> u8 {
        Pdruncfg1Usdhc0SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1Usdhc0SramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1Usdhc0SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1Usdhc0SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1Usdhc0SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1Usdhc0SramPpd {
        Pdruncfg1Usdhc0SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1Usdhc0SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1Usdhc0SramPpd) -> u8 {
        Pdruncfg1Usdhc0SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1Usdhc1SramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1Usdhc1SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1Usdhc1SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1Usdhc1SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1Usdhc1SramApd {
        Pdruncfg1Usdhc1SramApd::from_bits(val)
    }
}
impl From<Pdruncfg1Usdhc1SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1Usdhc1SramApd) -> u8 {
        Pdruncfg1Usdhc1SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg1Usdhc1SramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg1Usdhc1SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg1Usdhc1SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg1Usdhc1SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg1Usdhc1SramPpd {
        Pdruncfg1Usdhc1SramPpd::from_bits(val)
    }
}
impl From<Pdruncfg1Usdhc1SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg1Usdhc1SramPpd) -> u8 {
        Pdruncfg1Usdhc1SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf0Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf0Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf0Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf0Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf0Apd {
        Pdruncfg2ClrSramIf0Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf0Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf0Apd) -> u8 {
        Pdruncfg2ClrSramIf0Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf10Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf10Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf10Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf10Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf10Apd {
        Pdruncfg2ClrSramIf10Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf10Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf10Apd) -> u8 {
        Pdruncfg2ClrSramIf10Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf11Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf11Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf11Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf11Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf11Apd {
        Pdruncfg2ClrSramIf11Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf11Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf11Apd) -> u8 {
        Pdruncfg2ClrSramIf11Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf12Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf12Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf12Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf12Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf12Apd {
        Pdruncfg2ClrSramIf12Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf12Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf12Apd) -> u8 {
        Pdruncfg2ClrSramIf12Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf13Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf13Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf13Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf13Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf13Apd {
        Pdruncfg2ClrSramIf13Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf13Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf13Apd) -> u8 {
        Pdruncfg2ClrSramIf13Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf14Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf14Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf14Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf14Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf14Apd {
        Pdruncfg2ClrSramIf14Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf14Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf14Apd) -> u8 {
        Pdruncfg2ClrSramIf14Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf15Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf15Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf15Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf15Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf15Apd {
        Pdruncfg2ClrSramIf15Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf15Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf15Apd) -> u8 {
        Pdruncfg2ClrSramIf15Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf16Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf16Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf16Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf16Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf16Apd {
        Pdruncfg2ClrSramIf16Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf16Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf16Apd) -> u8 {
        Pdruncfg2ClrSramIf16Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf17Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf17Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf17Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf17Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf17Apd {
        Pdruncfg2ClrSramIf17Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf17Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf17Apd) -> u8 {
        Pdruncfg2ClrSramIf17Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf18Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf18Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf18Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf18Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf18Apd {
        Pdruncfg2ClrSramIf18Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf18Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf18Apd) -> u8 {
        Pdruncfg2ClrSramIf18Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf19Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf19Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf19Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf19Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf19Apd {
        Pdruncfg2ClrSramIf19Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf19Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf19Apd) -> u8 {
        Pdruncfg2ClrSramIf19Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf1Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf1Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf1Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf1Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf1Apd {
        Pdruncfg2ClrSramIf1Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf1Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf1Apd) -> u8 {
        Pdruncfg2ClrSramIf1Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf20Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf20Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf20Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf20Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf20Apd {
        Pdruncfg2ClrSramIf20Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf20Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf20Apd) -> u8 {
        Pdruncfg2ClrSramIf20Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf21Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf21Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf21Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf21Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf21Apd {
        Pdruncfg2ClrSramIf21Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf21Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf21Apd) -> u8 {
        Pdruncfg2ClrSramIf21Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf22Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf22Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf22Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf22Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf22Apd {
        Pdruncfg2ClrSramIf22Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf22Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf22Apd) -> u8 {
        Pdruncfg2ClrSramIf22Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf23Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf23Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf23Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf23Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf23Apd {
        Pdruncfg2ClrSramIf23Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf23Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf23Apd) -> u8 {
        Pdruncfg2ClrSramIf23Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf24Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf24Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf24Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf24Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf24Apd {
        Pdruncfg2ClrSramIf24Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf24Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf24Apd) -> u8 {
        Pdruncfg2ClrSramIf24Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf25Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf25Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf25Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf25Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf25Apd {
        Pdruncfg2ClrSramIf25Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf25Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf25Apd) -> u8 {
        Pdruncfg2ClrSramIf25Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf26Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf26Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf26Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf26Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf26Apd {
        Pdruncfg2ClrSramIf26Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf26Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf26Apd) -> u8 {
        Pdruncfg2ClrSramIf26Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf27Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf27Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf27Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf27Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf27Apd {
        Pdruncfg2ClrSramIf27Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf27Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf27Apd) -> u8 {
        Pdruncfg2ClrSramIf27Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf28Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf28Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf28Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf28Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf28Apd {
        Pdruncfg2ClrSramIf28Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf28Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf28Apd) -> u8 {
        Pdruncfg2ClrSramIf28Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf29Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf29Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf29Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf29Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf29Apd {
        Pdruncfg2ClrSramIf29Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf29Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf29Apd) -> u8 {
        Pdruncfg2ClrSramIf29Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf2Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf2Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf2Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf2Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf2Apd {
        Pdruncfg2ClrSramIf2Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf2Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf2Apd) -> u8 {
        Pdruncfg2ClrSramIf2Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf3Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf3Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf3Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf3Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf3Apd {
        Pdruncfg2ClrSramIf3Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf3Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf3Apd) -> u8 {
        Pdruncfg2ClrSramIf3Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf4Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf4Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf4Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf4Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf4Apd {
        Pdruncfg2ClrSramIf4Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf4Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf4Apd) -> u8 {
        Pdruncfg2ClrSramIf4Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf5Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf5Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf5Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf5Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf5Apd {
        Pdruncfg2ClrSramIf5Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf5Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf5Apd) -> u8 {
        Pdruncfg2ClrSramIf5Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf6Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf6Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf6Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf6Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf6Apd {
        Pdruncfg2ClrSramIf6Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf6Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf6Apd) -> u8 {
        Pdruncfg2ClrSramIf6Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf7Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf7Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf7Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf7Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf7Apd {
        Pdruncfg2ClrSramIf7Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf7Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf7Apd) -> u8 {
        Pdruncfg2ClrSramIf7Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf8Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf8Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf8Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf8Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf8Apd {
        Pdruncfg2ClrSramIf8Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf8Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf8Apd) -> u8 {
        Pdruncfg2ClrSramIf8Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2ClrSramIf9Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG2 Bit"]
    CLR_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2ClrSramIf9Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2ClrSramIf9Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2ClrSramIf9Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2ClrSramIf9Apd {
        Pdruncfg2ClrSramIf9Apd::from_bits(val)
    }
}
impl From<Pdruncfg2ClrSramIf9Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2ClrSramIf9Apd) -> u8 {
        Pdruncfg2ClrSramIf9Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf0Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf0Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf0Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf0Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf0Apd {
        Pdruncfg2SetSramIf0Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf0Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf0Apd) -> u8 {
        Pdruncfg2SetSramIf0Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf10Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf10Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf10Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf10Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf10Apd {
        Pdruncfg2SetSramIf10Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf10Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf10Apd) -> u8 {
        Pdruncfg2SetSramIf10Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf11Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf11Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf11Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf11Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf11Apd {
        Pdruncfg2SetSramIf11Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf11Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf11Apd) -> u8 {
        Pdruncfg2SetSramIf11Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf12Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf12Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf12Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf12Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf12Apd {
        Pdruncfg2SetSramIf12Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf12Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf12Apd) -> u8 {
        Pdruncfg2SetSramIf12Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf13Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf13Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf13Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf13Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf13Apd {
        Pdruncfg2SetSramIf13Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf13Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf13Apd) -> u8 {
        Pdruncfg2SetSramIf13Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf14Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf14Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf14Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf14Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf14Apd {
        Pdruncfg2SetSramIf14Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf14Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf14Apd) -> u8 {
        Pdruncfg2SetSramIf14Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf15Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf15Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf15Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf15Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf15Apd {
        Pdruncfg2SetSramIf15Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf15Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf15Apd) -> u8 {
        Pdruncfg2SetSramIf15Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf16Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf16Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf16Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf16Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf16Apd {
        Pdruncfg2SetSramIf16Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf16Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf16Apd) -> u8 {
        Pdruncfg2SetSramIf16Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf17Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf17Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf17Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf17Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf17Apd {
        Pdruncfg2SetSramIf17Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf17Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf17Apd) -> u8 {
        Pdruncfg2SetSramIf17Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf18Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf18Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf18Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf18Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf18Apd {
        Pdruncfg2SetSramIf18Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf18Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf18Apd) -> u8 {
        Pdruncfg2SetSramIf18Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf19Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf19Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf19Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf19Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf19Apd {
        Pdruncfg2SetSramIf19Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf19Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf19Apd) -> u8 {
        Pdruncfg2SetSramIf19Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf1Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf1Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf1Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf1Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf1Apd {
        Pdruncfg2SetSramIf1Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf1Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf1Apd) -> u8 {
        Pdruncfg2SetSramIf1Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf20Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf20Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf20Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf20Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf20Apd {
        Pdruncfg2SetSramIf20Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf20Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf20Apd) -> u8 {
        Pdruncfg2SetSramIf20Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf21Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf21Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf21Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf21Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf21Apd {
        Pdruncfg2SetSramIf21Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf21Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf21Apd) -> u8 {
        Pdruncfg2SetSramIf21Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf22Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf22Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf22Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf22Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf22Apd {
        Pdruncfg2SetSramIf22Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf22Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf22Apd) -> u8 {
        Pdruncfg2SetSramIf22Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf23Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf23Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf23Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf23Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf23Apd {
        Pdruncfg2SetSramIf23Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf23Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf23Apd) -> u8 {
        Pdruncfg2SetSramIf23Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf24Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf24Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf24Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf24Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf24Apd {
        Pdruncfg2SetSramIf24Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf24Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf24Apd) -> u8 {
        Pdruncfg2SetSramIf24Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf25Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf25Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf25Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf25Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf25Apd {
        Pdruncfg2SetSramIf25Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf25Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf25Apd) -> u8 {
        Pdruncfg2SetSramIf25Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf26Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf26Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf26Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf26Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf26Apd {
        Pdruncfg2SetSramIf26Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf26Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf26Apd) -> u8 {
        Pdruncfg2SetSramIf26Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf27Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf27Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf27Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf27Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf27Apd {
        Pdruncfg2SetSramIf27Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf27Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf27Apd) -> u8 {
        Pdruncfg2SetSramIf27Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf28Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf28Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf28Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf28Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf28Apd {
        Pdruncfg2SetSramIf28Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf28Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf28Apd) -> u8 {
        Pdruncfg2SetSramIf28Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf29Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf29Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf29Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf29Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf29Apd {
        Pdruncfg2SetSramIf29Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf29Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf29Apd) -> u8 {
        Pdruncfg2SetSramIf29Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf2Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf2Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf2Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf2Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf2Apd {
        Pdruncfg2SetSramIf2Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf2Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf2Apd) -> u8 {
        Pdruncfg2SetSramIf2Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf3Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf3Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf3Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf3Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf3Apd {
        Pdruncfg2SetSramIf3Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf3Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf3Apd) -> u8 {
        Pdruncfg2SetSramIf3Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf4Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf4Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf4Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf4Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf4Apd {
        Pdruncfg2SetSramIf4Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf4Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf4Apd) -> u8 {
        Pdruncfg2SetSramIf4Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf5Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf5Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf5Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf5Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf5Apd {
        Pdruncfg2SetSramIf5Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf5Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf5Apd) -> u8 {
        Pdruncfg2SetSramIf5Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf6Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf6Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf6Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf6Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf6Apd {
        Pdruncfg2SetSramIf6Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf6Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf6Apd) -> u8 {
        Pdruncfg2SetSramIf6Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf7Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf7Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf7Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf7Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf7Apd {
        Pdruncfg2SetSramIf7Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf7Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf7Apd) -> u8 {
        Pdruncfg2SetSramIf7Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf8Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf8Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf8Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf8Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf8Apd {
        Pdruncfg2SetSramIf8Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf8Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf8Apd) -> u8 {
        Pdruncfg2SetSramIf8Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SetSramIf9Apd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG2 Bit"]
    SET_PDRUNCFG2 = 0x01,
}
impl Pdruncfg2SetSramIf9Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SetSramIf9Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SetSramIf9Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SetSramIf9Apd {
        Pdruncfg2SetSramIf9Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SetSramIf9Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SetSramIf9Apd) -> u8 {
        Pdruncfg2SetSramIf9Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf0Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf0Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf0Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf0Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf0Apd {
        Pdruncfg2SramIf0Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf0Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf0Apd) -> u8 {
        Pdruncfg2SramIf0Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf10Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf10Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf10Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf10Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf10Apd {
        Pdruncfg2SramIf10Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf10Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf10Apd) -> u8 {
        Pdruncfg2SramIf10Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf11Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf11Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf11Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf11Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf11Apd {
        Pdruncfg2SramIf11Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf11Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf11Apd) -> u8 {
        Pdruncfg2SramIf11Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf12Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf12Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf12Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf12Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf12Apd {
        Pdruncfg2SramIf12Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf12Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf12Apd) -> u8 {
        Pdruncfg2SramIf12Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf13Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf13Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf13Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf13Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf13Apd {
        Pdruncfg2SramIf13Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf13Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf13Apd) -> u8 {
        Pdruncfg2SramIf13Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf14Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf14Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf14Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf14Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf14Apd {
        Pdruncfg2SramIf14Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf14Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf14Apd) -> u8 {
        Pdruncfg2SramIf14Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf15Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf15Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf15Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf15Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf15Apd {
        Pdruncfg2SramIf15Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf15Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf15Apd) -> u8 {
        Pdruncfg2SramIf15Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf16Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf16Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf16Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf16Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf16Apd {
        Pdruncfg2SramIf16Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf16Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf16Apd) -> u8 {
        Pdruncfg2SramIf16Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf17Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf17Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf17Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf17Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf17Apd {
        Pdruncfg2SramIf17Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf17Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf17Apd) -> u8 {
        Pdruncfg2SramIf17Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf18Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf18Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf18Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf18Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf18Apd {
        Pdruncfg2SramIf18Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf18Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf18Apd) -> u8 {
        Pdruncfg2SramIf18Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf19Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf19Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf19Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf19Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf19Apd {
        Pdruncfg2SramIf19Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf19Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf19Apd) -> u8 {
        Pdruncfg2SramIf19Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf1Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf1Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf1Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf1Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf1Apd {
        Pdruncfg2SramIf1Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf1Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf1Apd) -> u8 {
        Pdruncfg2SramIf1Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf20Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf20Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf20Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf20Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf20Apd {
        Pdruncfg2SramIf20Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf20Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf20Apd) -> u8 {
        Pdruncfg2SramIf20Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf21Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf21Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf21Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf21Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf21Apd {
        Pdruncfg2SramIf21Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf21Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf21Apd) -> u8 {
        Pdruncfg2SramIf21Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf22Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf22Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf22Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf22Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf22Apd {
        Pdruncfg2SramIf22Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf22Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf22Apd) -> u8 {
        Pdruncfg2SramIf22Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf23Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf23Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf23Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf23Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf23Apd {
        Pdruncfg2SramIf23Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf23Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf23Apd) -> u8 {
        Pdruncfg2SramIf23Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf24Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf24Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf24Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf24Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf24Apd {
        Pdruncfg2SramIf24Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf24Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf24Apd) -> u8 {
        Pdruncfg2SramIf24Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf25Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf25Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf25Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf25Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf25Apd {
        Pdruncfg2SramIf25Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf25Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf25Apd) -> u8 {
        Pdruncfg2SramIf25Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf26Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf26Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf26Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf26Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf26Apd {
        Pdruncfg2SramIf26Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf26Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf26Apd) -> u8 {
        Pdruncfg2SramIf26Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf27Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf27Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf27Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf27Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf27Apd {
        Pdruncfg2SramIf27Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf27Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf27Apd) -> u8 {
        Pdruncfg2SramIf27Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf28Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf28Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf28Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf28Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf28Apd {
        Pdruncfg2SramIf28Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf28Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf28Apd) -> u8 {
        Pdruncfg2SramIf28Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf29Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf29Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf29Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf29Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf29Apd {
        Pdruncfg2SramIf29Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf29Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf29Apd) -> u8 {
        Pdruncfg2SramIf29Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf2Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf2Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf2Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf2Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf2Apd {
        Pdruncfg2SramIf2Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf2Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf2Apd) -> u8 {
        Pdruncfg2SramIf2Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf3Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf3Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf3Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf3Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf3Apd {
        Pdruncfg2SramIf3Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf3Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf3Apd) -> u8 {
        Pdruncfg2SramIf3Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf4Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf4Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf4Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf4Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf4Apd {
        Pdruncfg2SramIf4Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf4Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf4Apd) -> u8 {
        Pdruncfg2SramIf4Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf5Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf5Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf5Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf5Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf5Apd {
        Pdruncfg2SramIf5Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf5Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf5Apd) -> u8 {
        Pdruncfg2SramIf5Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf6Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf6Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf6Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf6Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf6Apd {
        Pdruncfg2SramIf6Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf6Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf6Apd) -> u8 {
        Pdruncfg2SramIf6Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf7Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf7Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf7Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf7Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf7Apd {
        Pdruncfg2SramIf7Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf7Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf7Apd) -> u8 {
        Pdruncfg2SramIf7Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf8Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf8Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf8Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf8Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf8Apd {
        Pdruncfg2SramIf8Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf8Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf8Apd) -> u8 {
        Pdruncfg2SramIf8Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg2SramIf9Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg2SramIf9Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg2SramIf9Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg2SramIf9Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg2SramIf9Apd {
        Pdruncfg2SramIf9Apd::from_bits(val)
    }
}
impl From<Pdruncfg2SramIf9Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg2SramIf9Apd) -> u8 {
        Pdruncfg2SramIf9Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf0Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf0Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf0Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf0Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf0Ppd {
        Pdruncfg3ClrSramIf0Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf0Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf0Ppd) -> u8 {
        Pdruncfg3ClrSramIf0Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf10Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf10Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf10Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf10Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf10Ppd {
        Pdruncfg3ClrSramIf10Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf10Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf10Ppd) -> u8 {
        Pdruncfg3ClrSramIf10Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf11Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf11Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf11Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf11Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf11Ppd {
        Pdruncfg3ClrSramIf11Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf11Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf11Ppd) -> u8 {
        Pdruncfg3ClrSramIf11Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf12Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf12Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf12Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf12Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf12Ppd {
        Pdruncfg3ClrSramIf12Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf12Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf12Ppd) -> u8 {
        Pdruncfg3ClrSramIf12Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf13Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf13Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf13Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf13Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf13Ppd {
        Pdruncfg3ClrSramIf13Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf13Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf13Ppd) -> u8 {
        Pdruncfg3ClrSramIf13Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf14Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf14Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf14Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf14Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf14Ppd {
        Pdruncfg3ClrSramIf14Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf14Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf14Ppd) -> u8 {
        Pdruncfg3ClrSramIf14Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf15Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf15Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf15Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf15Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf15Ppd {
        Pdruncfg3ClrSramIf15Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf15Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf15Ppd) -> u8 {
        Pdruncfg3ClrSramIf15Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf16Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf16Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf16Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf16Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf16Ppd {
        Pdruncfg3ClrSramIf16Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf16Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf16Ppd) -> u8 {
        Pdruncfg3ClrSramIf16Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf17Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf17Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf17Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf17Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf17Ppd {
        Pdruncfg3ClrSramIf17Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf17Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf17Ppd) -> u8 {
        Pdruncfg3ClrSramIf17Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf18Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf18Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf18Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf18Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf18Ppd {
        Pdruncfg3ClrSramIf18Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf18Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf18Ppd) -> u8 {
        Pdruncfg3ClrSramIf18Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf19Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf19Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf19Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf19Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf19Ppd {
        Pdruncfg3ClrSramIf19Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf19Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf19Ppd) -> u8 {
        Pdruncfg3ClrSramIf19Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf1Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf1Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf1Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf1Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf1Ppd {
        Pdruncfg3ClrSramIf1Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf1Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf1Ppd) -> u8 {
        Pdruncfg3ClrSramIf1Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf20Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf20Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf20Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf20Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf20Ppd {
        Pdruncfg3ClrSramIf20Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf20Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf20Ppd) -> u8 {
        Pdruncfg3ClrSramIf20Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf21Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf21Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf21Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf21Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf21Ppd {
        Pdruncfg3ClrSramIf21Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf21Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf21Ppd) -> u8 {
        Pdruncfg3ClrSramIf21Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf22Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf22Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf22Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf22Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf22Ppd {
        Pdruncfg3ClrSramIf22Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf22Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf22Ppd) -> u8 {
        Pdruncfg3ClrSramIf22Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf23Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf23Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf23Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf23Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf23Ppd {
        Pdruncfg3ClrSramIf23Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf23Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf23Ppd) -> u8 {
        Pdruncfg3ClrSramIf23Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf24Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf24Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf24Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf24Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf24Ppd {
        Pdruncfg3ClrSramIf24Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf24Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf24Ppd) -> u8 {
        Pdruncfg3ClrSramIf24Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf25Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf25Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf25Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf25Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf25Ppd {
        Pdruncfg3ClrSramIf25Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf25Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf25Ppd) -> u8 {
        Pdruncfg3ClrSramIf25Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf26Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf26Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf26Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf26Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf26Ppd {
        Pdruncfg3ClrSramIf26Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf26Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf26Ppd) -> u8 {
        Pdruncfg3ClrSramIf26Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf27Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf27Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf27Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf27Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf27Ppd {
        Pdruncfg3ClrSramIf27Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf27Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf27Ppd) -> u8 {
        Pdruncfg3ClrSramIf27Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf28Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf28Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf28Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf28Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf28Ppd {
        Pdruncfg3ClrSramIf28Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf28Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf28Ppd) -> u8 {
        Pdruncfg3ClrSramIf28Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf29Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf29Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf29Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf29Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf29Ppd {
        Pdruncfg3ClrSramIf29Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf29Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf29Ppd) -> u8 {
        Pdruncfg3ClrSramIf29Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf2Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf2Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf2Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf2Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf2Ppd {
        Pdruncfg3ClrSramIf2Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf2Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf2Ppd) -> u8 {
        Pdruncfg3ClrSramIf2Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf3Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf3Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf3Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf3Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf3Ppd {
        Pdruncfg3ClrSramIf3Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf3Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf3Ppd) -> u8 {
        Pdruncfg3ClrSramIf3Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf4Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf4Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf4Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf4Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf4Ppd {
        Pdruncfg3ClrSramIf4Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf4Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf4Ppd) -> u8 {
        Pdruncfg3ClrSramIf4Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf5Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf5Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf5Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf5Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf5Ppd {
        Pdruncfg3ClrSramIf5Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf5Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf5Ppd) -> u8 {
        Pdruncfg3ClrSramIf5Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf6Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf6Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf6Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf6Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf6Ppd {
        Pdruncfg3ClrSramIf6Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf6Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf6Ppd) -> u8 {
        Pdruncfg3ClrSramIf6Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf7Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf7Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf7Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf7Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf7Ppd {
        Pdruncfg3ClrSramIf7Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf7Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf7Ppd) -> u8 {
        Pdruncfg3ClrSramIf7Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf8Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf8Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf8Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf8Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf8Ppd {
        Pdruncfg3ClrSramIf8Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf8Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf8Ppd) -> u8 {
        Pdruncfg3ClrSramIf8Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3ClrSramIf9Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PDRUNCFG3 Bit"]
    CLR_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3ClrSramIf9Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3ClrSramIf9Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3ClrSramIf9Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3ClrSramIf9Ppd {
        Pdruncfg3ClrSramIf9Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3ClrSramIf9Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3ClrSramIf9Ppd) -> u8 {
        Pdruncfg3ClrSramIf9Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf0Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf0Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf0Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf0Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf0Ppd {
        Pdruncfg3SetSramIf0Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf0Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf0Ppd) -> u8 {
        Pdruncfg3SetSramIf0Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf10Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf10Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf10Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf10Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf10Ppd {
        Pdruncfg3SetSramIf10Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf10Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf10Ppd) -> u8 {
        Pdruncfg3SetSramIf10Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf11Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf11Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf11Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf11Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf11Ppd {
        Pdruncfg3SetSramIf11Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf11Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf11Ppd) -> u8 {
        Pdruncfg3SetSramIf11Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf12Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf12Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf12Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf12Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf12Ppd {
        Pdruncfg3SetSramIf12Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf12Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf12Ppd) -> u8 {
        Pdruncfg3SetSramIf12Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf13Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf13Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf13Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf13Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf13Ppd {
        Pdruncfg3SetSramIf13Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf13Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf13Ppd) -> u8 {
        Pdruncfg3SetSramIf13Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf14Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf14Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf14Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf14Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf14Ppd {
        Pdruncfg3SetSramIf14Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf14Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf14Ppd) -> u8 {
        Pdruncfg3SetSramIf14Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf15Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf15Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf15Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf15Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf15Ppd {
        Pdruncfg3SetSramIf15Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf15Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf15Ppd) -> u8 {
        Pdruncfg3SetSramIf15Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf16Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf16Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf16Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf16Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf16Ppd {
        Pdruncfg3SetSramIf16Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf16Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf16Ppd) -> u8 {
        Pdruncfg3SetSramIf16Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf17Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf17Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf17Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf17Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf17Ppd {
        Pdruncfg3SetSramIf17Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf17Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf17Ppd) -> u8 {
        Pdruncfg3SetSramIf17Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf18Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf18Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf18Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf18Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf18Ppd {
        Pdruncfg3SetSramIf18Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf18Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf18Ppd) -> u8 {
        Pdruncfg3SetSramIf18Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf19Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf19Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf19Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf19Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf19Ppd {
        Pdruncfg3SetSramIf19Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf19Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf19Ppd) -> u8 {
        Pdruncfg3SetSramIf19Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf1Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf1Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf1Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf1Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf1Ppd {
        Pdruncfg3SetSramIf1Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf1Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf1Ppd) -> u8 {
        Pdruncfg3SetSramIf1Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf20Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf20Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf20Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf20Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf20Ppd {
        Pdruncfg3SetSramIf20Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf20Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf20Ppd) -> u8 {
        Pdruncfg3SetSramIf20Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf21Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf21Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf21Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf21Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf21Ppd {
        Pdruncfg3SetSramIf21Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf21Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf21Ppd) -> u8 {
        Pdruncfg3SetSramIf21Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf22Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf22Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf22Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf22Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf22Ppd {
        Pdruncfg3SetSramIf22Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf22Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf22Ppd) -> u8 {
        Pdruncfg3SetSramIf22Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf23Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf23Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf23Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf23Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf23Ppd {
        Pdruncfg3SetSramIf23Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf23Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf23Ppd) -> u8 {
        Pdruncfg3SetSramIf23Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf24Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf24Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf24Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf24Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf24Ppd {
        Pdruncfg3SetSramIf24Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf24Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf24Ppd) -> u8 {
        Pdruncfg3SetSramIf24Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf25Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf25Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf25Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf25Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf25Ppd {
        Pdruncfg3SetSramIf25Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf25Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf25Ppd) -> u8 {
        Pdruncfg3SetSramIf25Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf26Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf26Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf26Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf26Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf26Ppd {
        Pdruncfg3SetSramIf26Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf26Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf26Ppd) -> u8 {
        Pdruncfg3SetSramIf26Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf27Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf27Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf27Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf27Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf27Ppd {
        Pdruncfg3SetSramIf27Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf27Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf27Ppd) -> u8 {
        Pdruncfg3SetSramIf27Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf28Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf28Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf28Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf28Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf28Ppd {
        Pdruncfg3SetSramIf28Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf28Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf28Ppd) -> u8 {
        Pdruncfg3SetSramIf28Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf29Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf29Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf29Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf29Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf29Ppd {
        Pdruncfg3SetSramIf29Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf29Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf29Ppd) -> u8 {
        Pdruncfg3SetSramIf29Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf2Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf2Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf2Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf2Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf2Ppd {
        Pdruncfg3SetSramIf2Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf2Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf2Ppd) -> u8 {
        Pdruncfg3SetSramIf2Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf3Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf3Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf3Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf3Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf3Ppd {
        Pdruncfg3SetSramIf3Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf3Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf3Ppd) -> u8 {
        Pdruncfg3SetSramIf3Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf4Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf4Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf4Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf4Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf4Ppd {
        Pdruncfg3SetSramIf4Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf4Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf4Ppd) -> u8 {
        Pdruncfg3SetSramIf4Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf5Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf5Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf5Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf5Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf5Ppd {
        Pdruncfg3SetSramIf5Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf5Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf5Ppd) -> u8 {
        Pdruncfg3SetSramIf5Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf6Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf6Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf6Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf6Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf6Ppd {
        Pdruncfg3SetSramIf6Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf6Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf6Ppd) -> u8 {
        Pdruncfg3SetSramIf6Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf7Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf7Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf7Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf7Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf7Ppd {
        Pdruncfg3SetSramIf7Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf7Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf7Ppd) -> u8 {
        Pdruncfg3SetSramIf7Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf8Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf8Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf8Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf8Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf8Ppd {
        Pdruncfg3SetSramIf8Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf8Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf8Ppd) -> u8 {
        Pdruncfg3SetSramIf8Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SetSramIf9Ppd {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PDRUNCFG3 Bit"]
    SET_PDRUNCFG3 = 0x01,
}
impl Pdruncfg3SetSramIf9Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SetSramIf9Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SetSramIf9Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SetSramIf9Ppd {
        Pdruncfg3SetSramIf9Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SetSramIf9Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SetSramIf9Ppd) -> u8 {
        Pdruncfg3SetSramIf9Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf0Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf0Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf0Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf0Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf0Ppd {
        Pdruncfg3SramIf0Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf0Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf0Ppd) -> u8 {
        Pdruncfg3SramIf0Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf10Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf10Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf10Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf10Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf10Ppd {
        Pdruncfg3SramIf10Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf10Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf10Ppd) -> u8 {
        Pdruncfg3SramIf10Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf11Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf11Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf11Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf11Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf11Ppd {
        Pdruncfg3SramIf11Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf11Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf11Ppd) -> u8 {
        Pdruncfg3SramIf11Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf12Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf12Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf12Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf12Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf12Ppd {
        Pdruncfg3SramIf12Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf12Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf12Ppd) -> u8 {
        Pdruncfg3SramIf12Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf13Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf13Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf13Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf13Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf13Ppd {
        Pdruncfg3SramIf13Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf13Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf13Ppd) -> u8 {
        Pdruncfg3SramIf13Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf14Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf14Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf14Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf14Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf14Ppd {
        Pdruncfg3SramIf14Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf14Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf14Ppd) -> u8 {
        Pdruncfg3SramIf14Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf15Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf15Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf15Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf15Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf15Ppd {
        Pdruncfg3SramIf15Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf15Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf15Ppd) -> u8 {
        Pdruncfg3SramIf15Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf16Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf16Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf16Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf16Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf16Ppd {
        Pdruncfg3SramIf16Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf16Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf16Ppd) -> u8 {
        Pdruncfg3SramIf16Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf17Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf17Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf17Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf17Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf17Ppd {
        Pdruncfg3SramIf17Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf17Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf17Ppd) -> u8 {
        Pdruncfg3SramIf17Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf18Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf18Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf18Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf18Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf18Ppd {
        Pdruncfg3SramIf18Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf18Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf18Ppd) -> u8 {
        Pdruncfg3SramIf18Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf19Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf19Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf19Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf19Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf19Ppd {
        Pdruncfg3SramIf19Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf19Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf19Ppd) -> u8 {
        Pdruncfg3SramIf19Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf1Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf1Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf1Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf1Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf1Ppd {
        Pdruncfg3SramIf1Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf1Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf1Ppd) -> u8 {
        Pdruncfg3SramIf1Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf20Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf20Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf20Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf20Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf20Ppd {
        Pdruncfg3SramIf20Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf20Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf20Ppd) -> u8 {
        Pdruncfg3SramIf20Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf21Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf21Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf21Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf21Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf21Ppd {
        Pdruncfg3SramIf21Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf21Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf21Ppd) -> u8 {
        Pdruncfg3SramIf21Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf22Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf22Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf22Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf22Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf22Ppd {
        Pdruncfg3SramIf22Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf22Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf22Ppd) -> u8 {
        Pdruncfg3SramIf22Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf23Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf23Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf23Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf23Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf23Ppd {
        Pdruncfg3SramIf23Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf23Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf23Ppd) -> u8 {
        Pdruncfg3SramIf23Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf24Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf24Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf24Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf24Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf24Ppd {
        Pdruncfg3SramIf24Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf24Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf24Ppd) -> u8 {
        Pdruncfg3SramIf24Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf25Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf25Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf25Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf25Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf25Ppd {
        Pdruncfg3SramIf25Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf25Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf25Ppd) -> u8 {
        Pdruncfg3SramIf25Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf26Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf26Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf26Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf26Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf26Ppd {
        Pdruncfg3SramIf26Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf26Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf26Ppd) -> u8 {
        Pdruncfg3SramIf26Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf27Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf27Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf27Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf27Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf27Ppd {
        Pdruncfg3SramIf27Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf27Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf27Ppd) -> u8 {
        Pdruncfg3SramIf27Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf28Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf28Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf28Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf28Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf28Ppd {
        Pdruncfg3SramIf28Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf28Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf28Ppd) -> u8 {
        Pdruncfg3SramIf28Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf29Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf29Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf29Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf29Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf29Ppd {
        Pdruncfg3SramIf29Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf29Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf29Ppd) -> u8 {
        Pdruncfg3SramIf29Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf2Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf2Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf2Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf2Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf2Ppd {
        Pdruncfg3SramIf2Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf2Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf2Ppd) -> u8 {
        Pdruncfg3SramIf2Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf3Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf3Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf3Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf3Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf3Ppd {
        Pdruncfg3SramIf3Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf3Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf3Ppd) -> u8 {
        Pdruncfg3SramIf3Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf4Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf4Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf4Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf4Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf4Ppd {
        Pdruncfg3SramIf4Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf4Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf4Ppd) -> u8 {
        Pdruncfg3SramIf4Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf5Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf5Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf5Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf5Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf5Ppd {
        Pdruncfg3SramIf5Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf5Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf5Ppd) -> u8 {
        Pdruncfg3SramIf5Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf6Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf6Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf6Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf6Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf6Ppd {
        Pdruncfg3SramIf6Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf6Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf6Ppd) -> u8 {
        Pdruncfg3SramIf6Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf7Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf7Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf7Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf7Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf7Ppd {
        Pdruncfg3SramIf7Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf7Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf7Ppd) -> u8 {
        Pdruncfg3SramIf7Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf8Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf8Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf8Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf8Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf8Ppd {
        Pdruncfg3SramIf8Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf8Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf8Ppd) -> u8 {
        Pdruncfg3SramIf8Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdruncfg3SramIf9Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdruncfg3SramIf9Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdruncfg3SramIf9Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdruncfg3SramIf9Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdruncfg3SramIf9Ppd {
        Pdruncfg3SramIf9Ppd::from_bits(val)
    }
}
impl From<Pdruncfg3SramIf9Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdruncfg3SramIf9Ppd) -> u8 {
        Pdruncfg3SramIf9Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AcmpPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AcmpPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AcmpPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AcmpPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AcmpPd {
        Pdsleepcfg0AcmpPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0AcmpPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AcmpPd) -> u8 {
        Pdsleepcfg0AcmpPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AdcLp {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AdcLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AdcLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AdcLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AdcLp {
        Pdsleepcfg0AdcLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0AdcLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AdcLp) -> u8 {
        Pdsleepcfg0AdcLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AdcPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AdcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AdcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AdcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AdcPd {
        Pdsleepcfg0AdcPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0AdcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AdcPd) -> u8 {
        Pdsleepcfg0AdcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AdctempsnsPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AdctempsnsPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AdctempsnsPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AdctempsnsPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AdctempsnsPd {
        Pdsleepcfg0AdctempsnsPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0AdctempsnsPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AdctempsnsPd) -> u8 {
        Pdsleepcfg0AdctempsnsPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AudpllanaPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AudpllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AudpllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AudpllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AudpllanaPd {
        Pdsleepcfg0AudpllanaPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0AudpllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AudpllanaPd) -> u8 {
        Pdsleepcfg0AudpllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0AudpllldoPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0AudpllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0AudpllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0AudpllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0AudpllldoPd {
        Pdsleepcfg0AudpllldoPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0AudpllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0AudpllldoPd) -> u8 {
        Pdsleepcfg0AudpllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0FbbPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0FbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0FbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0FbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0FbbPd {
        Pdsleepcfg0FbbPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0FbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0FbbPd) -> u8 {
        Pdsleepcfg0FbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0FfroPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0FfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0FfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0FfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0FfroPd {
        Pdsleepcfg0FfroPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0FfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0FfroPd) -> u8 {
        Pdsleepcfg0FfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0Hspad0RefPd {
    #[doc = "High Speed Pad VREF Enabled"]
    ENABLED = 0x0,
    #[doc = "High Speed Pad VREF in Power Down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0Hspad0RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0Hspad0RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0Hspad0RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0Hspad0RefPd {
        Pdsleepcfg0Hspad0RefPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0Hspad0RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0Hspad0RefPd) -> u8 {
        Pdsleepcfg0Hspad0RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0Hspad0VdetLp {
    #[doc = "High Speed Pad VDET in Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    SLEEP_MODE = 0x01,
}
impl Pdsleepcfg0Hspad0VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0Hspad0VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0Hspad0VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0Hspad0VdetLp {
        Pdsleepcfg0Hspad0VdetLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0Hspad0VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0Hspad0VdetLp) -> u8 {
        Pdsleepcfg0Hspad0VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0Hspad2RefPd {
    #[doc = "High Speed Pad VREF Enabled"]
    ENABLED = 0x0,
    #[doc = "High Speed Pad VREF in Power Down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0Hspad2RefPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0Hspad2RefPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0Hspad2RefPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0Hspad2RefPd {
        Pdsleepcfg0Hspad2RefPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0Hspad2RefPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0Hspad2RefPd) -> u8 {
        Pdsleepcfg0Hspad2RefPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0Hspad2VdetLp {
    #[doc = "High Speed Pad VDET in Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "High Speed Pad VDET in Sleep Mode"]
    SLEEP_MODE = 0x01,
}
impl Pdsleepcfg0Hspad2VdetLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0Hspad2VdetLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0Hspad2VdetLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0Hspad2VdetLp {
        Pdsleepcfg0Hspad2VdetLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0Hspad2VdetLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0Hspad2VdetLp) -> u8 {
        Pdsleepcfg0Hspad2VdetLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0Hvd1v8Pd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0Hvd1v8Pd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0Hvd1v8Pd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0Hvd1v8Pd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0Hvd1v8Pd {
        Pdsleepcfg0Hvd1v8Pd::from_bits(val)
    }
}
impl From<Pdsleepcfg0Hvd1v8Pd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0Hvd1v8Pd) -> u8 {
        Pdsleepcfg0Hvd1v8Pd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0HvdcorePd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0HvdcorePd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0HvdcorePd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0HvdcorePd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0HvdcorePd {
        Pdsleepcfg0HvdcorePd::from_bits(val)
    }
}
impl From<Pdsleepcfg0HvdcorePd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0HvdcorePd) -> u8 {
        Pdsleepcfg0HvdcorePd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0LposcPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0LposcPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0LposcPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0LposcPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0LposcPd {
        Pdsleepcfg0LposcPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0LposcPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0LposcPd) -> u8 {
        Pdsleepcfg0LposcPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0LvdcoreLp {
    #[doc = "LVD0V85 HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LVD0V85 LP Mode."]
    LP_MODE = 0x01,
}
impl Pdsleepcfg0LvdcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0LvdcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0LvdcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0LvdcoreLp {
        Pdsleepcfg0LvdcoreLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0LvdcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0LvdcoreLp) -> u8 {
        Pdsleepcfg0LvdcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0PmcrefLp {
    #[doc = "PMCREF HP Mode"]
    HP_MODE = 0x0,
    #[doc = "PMCREF LP Mode"]
    LP_MODE = 0x01,
}
impl Pdsleepcfg0PmcrefLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0PmcrefLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0PmcrefLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0PmcrefLp {
        Pdsleepcfg0PmcrefLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0PmcrefLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0PmcrefLp) -> u8 {
        Pdsleepcfg0PmcrefLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0PmicMode0 {
    #[doc = "Set Mode0 to '0'."]
    SET_MODE0_0 = 0x0,
    #[doc = "Set Mode0 to '1'."]
    SET_MODE0_1 = 0x01,
}
impl Pdsleepcfg0PmicMode0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0PmicMode0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0PmicMode0 {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0PmicMode0 {
        Pdsleepcfg0PmicMode0::from_bits(val)
    }
}
impl From<Pdsleepcfg0PmicMode0> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0PmicMode0) -> u8 {
        Pdsleepcfg0PmicMode0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0PmicMode1 {
    #[doc = "Set Mode1 to 0."]
    SET_MODE1_0 = 0x0,
    #[doc = "Set Mode1 to 1."]
    SET_MODE1_1 = 0x01,
}
impl Pdsleepcfg0PmicMode1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0PmicMode1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0PmicMode1 {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0PmicMode1 {
        Pdsleepcfg0PmicMode1::from_bits(val)
    }
}
impl From<Pdsleepcfg0PmicMode1> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0PmicMode1) -> u8 {
        Pdsleepcfg0PmicMode1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0PorcoreLp {
    #[doc = "LVD0V6 HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LVD0V6 LP Mode"]
    LP_MODE = 0x01,
}
impl Pdsleepcfg0PorcoreLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0PorcoreLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0PorcoreLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0PorcoreLp {
        Pdsleepcfg0PorcoreLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0PorcoreLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0PorcoreLp) -> u8 {
        Pdsleepcfg0PorcoreLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0RbbPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0RbbPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0RbbPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0RbbPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0RbbPd {
        Pdsleepcfg0RbbPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0RbbPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0RbbPd) -> u8 {
        Pdsleepcfg0RbbPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0SfroPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0SfroPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0SfroPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0SfroPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0SfroPd {
        Pdsleepcfg0SfroPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0SfroPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0SfroPd) -> u8 {
        Pdsleepcfg0SfroPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0SyspllanaPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0SyspllanaPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0SyspllanaPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0SyspllanaPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0SyspllanaPd {
        Pdsleepcfg0SyspllanaPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0SyspllanaPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0SyspllanaPd) -> u8 {
        Pdsleepcfg0SyspllanaPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0SyspllldoPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0SyspllldoPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0SyspllldoPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0SyspllldoPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0SyspllldoPd {
        Pdsleepcfg0SyspllldoPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0SyspllldoPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0SyspllldoPd) -> u8 {
        Pdsleepcfg0SyspllldoPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0SysxtalPd {
    #[doc = "enabled"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg0SysxtalPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0SysxtalPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0SysxtalPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0SysxtalPd {
        Pdsleepcfg0SysxtalPd::from_bits(val)
    }
}
impl From<Pdsleepcfg0SysxtalPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0SysxtalPd) -> u8 {
        Pdsleepcfg0SysxtalPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg0VddcoreregLp {
    #[doc = "VDDCOREREG HP Mode"]
    HP_MODE = 0x0,
    #[doc = "LP Mode"]
    LP_MODE = 0x01,
}
impl Pdsleepcfg0VddcoreregLp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg0VddcoreregLp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg0VddcoreregLp {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg0VddcoreregLp {
        Pdsleepcfg0VddcoreregLp::from_bits(val)
    }
}
impl From<Pdsleepcfg0VddcoreregLp> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg0VddcoreregLp) -> u8 {
        Pdsleepcfg0VddcoreregLp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1CasperSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1CasperSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1CasperSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1CasperSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1CasperSramApd {
        Pdsleepcfg1CasperSramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1CasperSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1CasperSramApd) -> u8 {
        Pdsleepcfg1CasperSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1CasperSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1CasperSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1CasperSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1CasperSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1CasperSramPpd {
        Pdsleepcfg1CasperSramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1CasperSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1CasperSramPpd) -> u8 {
        Pdsleepcfg1CasperSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1DspcacheRegfApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1DspcacheRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1DspcacheRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1DspcacheRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1DspcacheRegfApd {
        Pdsleepcfg1DspcacheRegfApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1DspcacheRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1DspcacheRegfApd) -> u8 {
        Pdsleepcfg1DspcacheRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1DspcacheRegfPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1DspcacheRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1DspcacheRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1DspcacheRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1DspcacheRegfPpd {
        Pdsleepcfg1DspcacheRegfPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1DspcacheRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1DspcacheRegfPpd) -> u8 {
        Pdsleepcfg1DspcacheRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1DsptcmRegfApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1DsptcmRegfApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1DsptcmRegfApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1DsptcmRegfApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1DsptcmRegfApd {
        Pdsleepcfg1DsptcmRegfApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1DsptcmRegfApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1DsptcmRegfApd) -> u8 {
        Pdsleepcfg1DsptcmRegfApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1DsptcmRegfPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1DsptcmRegfPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1DsptcmRegfPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1DsptcmRegfPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1DsptcmRegfPpd {
        Pdsleepcfg1DsptcmRegfPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1DsptcmRegfPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1DsptcmRegfPpd) -> u8 {
        Pdsleepcfg1DsptcmRegfPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1FlexspiSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1FlexspiSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1FlexspiSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1FlexspiSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1FlexspiSramApd {
        Pdsleepcfg1FlexspiSramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1FlexspiSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1FlexspiSramApd) -> u8 {
        Pdsleepcfg1FlexspiSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1FlexspiSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1FlexspiSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1FlexspiSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1FlexspiSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1FlexspiSramPpd {
        Pdsleepcfg1FlexspiSramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1FlexspiSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1FlexspiSramPpd) -> u8 {
        Pdsleepcfg1FlexspiSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1PqSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1PqSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1PqSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1PqSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1PqSramApd {
        Pdsleepcfg1PqSramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1PqSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1PqSramApd) -> u8 {
        Pdsleepcfg1PqSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1PqSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1PqSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1PqSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1PqSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1PqSramPpd {
        Pdsleepcfg1PqSramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1PqSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1PqSramPpd) -> u8 {
        Pdsleepcfg1PqSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1RomPd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1RomPd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1RomPd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1RomPd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1RomPd {
        Pdsleepcfg1RomPd::from_bits(val)
    }
}
impl From<Pdsleepcfg1RomPd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1RomPd) -> u8 {
        Pdsleepcfg1RomPd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1SramSleep {
    #[doc = "RAM Normal Mode"]
    NORMAL_MODE = 0x0,
    #[doc = "RAM Sleep Mode."]
    SLEEP_MODE = 0x01,
}
impl Pdsleepcfg1SramSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1SramSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1SramSleep {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1SramSleep {
        Pdsleepcfg1SramSleep::from_bits(val)
    }
}
impl From<Pdsleepcfg1SramSleep> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1SramSleep) -> u8 {
        Pdsleepcfg1SramSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1UsbhsSramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1UsbhsSramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1UsbhsSramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1UsbhsSramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1UsbhsSramApd {
        Pdsleepcfg1UsbhsSramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1UsbhsSramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1UsbhsSramApd) -> u8 {
        Pdsleepcfg1UsbhsSramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1UsbhsSramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1UsbhsSramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1UsbhsSramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1UsbhsSramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1UsbhsSramPpd {
        Pdsleepcfg1UsbhsSramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1UsbhsSramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1UsbhsSramPpd) -> u8 {
        Pdsleepcfg1UsbhsSramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1Usdhc0SramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1Usdhc0SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1Usdhc0SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1Usdhc0SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1Usdhc0SramApd {
        Pdsleepcfg1Usdhc0SramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1Usdhc0SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1Usdhc0SramApd) -> u8 {
        Pdsleepcfg1Usdhc0SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1Usdhc0SramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1Usdhc0SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1Usdhc0SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1Usdhc0SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1Usdhc0SramPpd {
        Pdsleepcfg1Usdhc0SramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1Usdhc0SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1Usdhc0SramPpd) -> u8 {
        Pdsleepcfg1Usdhc0SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1Usdhc1SramApd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1Usdhc1SramApd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1Usdhc1SramApd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1Usdhc1SramApd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1Usdhc1SramApd {
        Pdsleepcfg1Usdhc1SramApd::from_bits(val)
    }
}
impl From<Pdsleepcfg1Usdhc1SramApd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1Usdhc1SramApd) -> u8 {
        Pdsleepcfg1Usdhc1SramApd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg1Usdhc1SramPpd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg1Usdhc1SramPpd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg1Usdhc1SramPpd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg1Usdhc1SramPpd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg1Usdhc1SramPpd {
        Pdsleepcfg1Usdhc1SramPpd::from_bits(val)
    }
}
impl From<Pdsleepcfg1Usdhc1SramPpd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg1Usdhc1SramPpd) -> u8 {
        Pdsleepcfg1Usdhc1SramPpd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf0Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf0Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf0Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf0Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf0Apd {
        Pdsleepcfg2SramIf0Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf0Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf0Apd) -> u8 {
        Pdsleepcfg2SramIf0Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf10Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf10Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf10Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf10Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf10Apd {
        Pdsleepcfg2SramIf10Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf10Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf10Apd) -> u8 {
        Pdsleepcfg2SramIf10Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf11Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf11Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf11Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf11Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf11Apd {
        Pdsleepcfg2SramIf11Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf11Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf11Apd) -> u8 {
        Pdsleepcfg2SramIf11Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf12Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf12Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf12Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf12Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf12Apd {
        Pdsleepcfg2SramIf12Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf12Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf12Apd) -> u8 {
        Pdsleepcfg2SramIf12Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf13Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf13Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf13Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf13Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf13Apd {
        Pdsleepcfg2SramIf13Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf13Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf13Apd) -> u8 {
        Pdsleepcfg2SramIf13Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf14Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf14Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf14Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf14Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf14Apd {
        Pdsleepcfg2SramIf14Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf14Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf14Apd) -> u8 {
        Pdsleepcfg2SramIf14Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf15Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf15Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf15Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf15Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf15Apd {
        Pdsleepcfg2SramIf15Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf15Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf15Apd) -> u8 {
        Pdsleepcfg2SramIf15Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf16Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf16Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf16Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf16Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf16Apd {
        Pdsleepcfg2SramIf16Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf16Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf16Apd) -> u8 {
        Pdsleepcfg2SramIf16Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf17Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf17Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf17Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf17Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf17Apd {
        Pdsleepcfg2SramIf17Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf17Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf17Apd) -> u8 {
        Pdsleepcfg2SramIf17Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf18Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf18Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf18Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf18Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf18Apd {
        Pdsleepcfg2SramIf18Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf18Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf18Apd) -> u8 {
        Pdsleepcfg2SramIf18Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf19Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf19Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf19Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf19Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf19Apd {
        Pdsleepcfg2SramIf19Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf19Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf19Apd) -> u8 {
        Pdsleepcfg2SramIf19Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf1Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf1Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf1Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf1Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf1Apd {
        Pdsleepcfg2SramIf1Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf1Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf1Apd) -> u8 {
        Pdsleepcfg2SramIf1Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf20Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf20Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf20Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf20Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf20Apd {
        Pdsleepcfg2SramIf20Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf20Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf20Apd) -> u8 {
        Pdsleepcfg2SramIf20Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf21Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf21Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf21Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf21Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf21Apd {
        Pdsleepcfg2SramIf21Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf21Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf21Apd) -> u8 {
        Pdsleepcfg2SramIf21Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf22Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf22Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf22Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf22Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf22Apd {
        Pdsleepcfg2SramIf22Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf22Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf22Apd) -> u8 {
        Pdsleepcfg2SramIf22Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf23Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf23Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf23Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf23Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf23Apd {
        Pdsleepcfg2SramIf23Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf23Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf23Apd) -> u8 {
        Pdsleepcfg2SramIf23Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf24Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf24Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf24Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf24Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf24Apd {
        Pdsleepcfg2SramIf24Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf24Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf24Apd) -> u8 {
        Pdsleepcfg2SramIf24Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf25Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf25Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf25Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf25Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf25Apd {
        Pdsleepcfg2SramIf25Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf25Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf25Apd) -> u8 {
        Pdsleepcfg2SramIf25Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf26Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf26Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf26Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf26Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf26Apd {
        Pdsleepcfg2SramIf26Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf26Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf26Apd) -> u8 {
        Pdsleepcfg2SramIf26Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf27Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf27Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf27Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf27Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf27Apd {
        Pdsleepcfg2SramIf27Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf27Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf27Apd) -> u8 {
        Pdsleepcfg2SramIf27Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf28Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf28Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf28Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf28Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf28Apd {
        Pdsleepcfg2SramIf28Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf28Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf28Apd) -> u8 {
        Pdsleepcfg2SramIf28Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf29Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf29Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf29Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf29Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf29Apd {
        Pdsleepcfg2SramIf29Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf29Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf29Apd) -> u8 {
        Pdsleepcfg2SramIf29Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf2Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf2Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf2Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf2Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf2Apd {
        Pdsleepcfg2SramIf2Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf2Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf2Apd) -> u8 {
        Pdsleepcfg2SramIf2Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf3Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf3Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf3Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf3Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf3Apd {
        Pdsleepcfg2SramIf3Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf3Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf3Apd) -> u8 {
        Pdsleepcfg2SramIf3Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf4Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf4Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf4Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf4Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf4Apd {
        Pdsleepcfg2SramIf4Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf4Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf4Apd) -> u8 {
        Pdsleepcfg2SramIf4Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf5Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf5Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf5Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf5Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf5Apd {
        Pdsleepcfg2SramIf5Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf5Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf5Apd) -> u8 {
        Pdsleepcfg2SramIf5Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf6Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf6Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf6Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf6Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf6Apd {
        Pdsleepcfg2SramIf6Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf6Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf6Apd) -> u8 {
        Pdsleepcfg2SramIf6Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf7Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf7Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf7Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf7Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf7Apd {
        Pdsleepcfg2SramIf7Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf7Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf7Apd) -> u8 {
        Pdsleepcfg2SramIf7Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf8Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf8Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf8Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf8Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf8Apd {
        Pdsleepcfg2SramIf8Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf8Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf8Apd) -> u8 {
        Pdsleepcfg2SramIf8Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg2SramIf9Apd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg2SramIf9Apd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg2SramIf9Apd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg2SramIf9Apd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg2SramIf9Apd {
        Pdsleepcfg2SramIf9Apd::from_bits(val)
    }
}
impl From<Pdsleepcfg2SramIf9Apd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg2SramIf9Apd) -> u8 {
        Pdsleepcfg2SramIf9Apd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf0Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf0Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf0Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf0Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf0Ppd {
        Pdsleepcfg3SramIf0Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf0Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf0Ppd) -> u8 {
        Pdsleepcfg3SramIf0Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf10Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf10Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf10Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf10Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf10Ppd {
        Pdsleepcfg3SramIf10Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf10Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf10Ppd) -> u8 {
        Pdsleepcfg3SramIf10Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf11Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf11Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf11Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf11Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf11Ppd {
        Pdsleepcfg3SramIf11Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf11Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf11Ppd) -> u8 {
        Pdsleepcfg3SramIf11Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf12Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf12Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf12Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf12Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf12Ppd {
        Pdsleepcfg3SramIf12Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf12Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf12Ppd) -> u8 {
        Pdsleepcfg3SramIf12Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf13Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf13Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf13Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf13Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf13Ppd {
        Pdsleepcfg3SramIf13Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf13Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf13Ppd) -> u8 {
        Pdsleepcfg3SramIf13Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf14Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf14Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf14Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf14Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf14Ppd {
        Pdsleepcfg3SramIf14Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf14Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf14Ppd) -> u8 {
        Pdsleepcfg3SramIf14Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf15Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf15Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf15Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf15Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf15Ppd {
        Pdsleepcfg3SramIf15Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf15Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf15Ppd) -> u8 {
        Pdsleepcfg3SramIf15Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf16Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf16Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf16Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf16Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf16Ppd {
        Pdsleepcfg3SramIf16Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf16Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf16Ppd) -> u8 {
        Pdsleepcfg3SramIf16Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf17Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf17Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf17Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf17Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf17Ppd {
        Pdsleepcfg3SramIf17Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf17Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf17Ppd) -> u8 {
        Pdsleepcfg3SramIf17Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf18Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf18Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf18Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf18Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf18Ppd {
        Pdsleepcfg3SramIf18Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf18Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf18Ppd) -> u8 {
        Pdsleepcfg3SramIf18Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf19Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf19Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf19Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf19Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf19Ppd {
        Pdsleepcfg3SramIf19Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf19Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf19Ppd) -> u8 {
        Pdsleepcfg3SramIf19Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf1Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf1Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf1Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf1Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf1Ppd {
        Pdsleepcfg3SramIf1Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf1Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf1Ppd) -> u8 {
        Pdsleepcfg3SramIf1Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf20Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf20Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf20Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf20Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf20Ppd {
        Pdsleepcfg3SramIf20Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf20Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf20Ppd) -> u8 {
        Pdsleepcfg3SramIf20Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf21Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf21Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf21Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf21Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf21Ppd {
        Pdsleepcfg3SramIf21Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf21Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf21Ppd) -> u8 {
        Pdsleepcfg3SramIf21Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf22Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf22Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf22Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf22Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf22Ppd {
        Pdsleepcfg3SramIf22Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf22Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf22Ppd) -> u8 {
        Pdsleepcfg3SramIf22Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf23Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf23Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf23Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf23Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf23Ppd {
        Pdsleepcfg3SramIf23Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf23Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf23Ppd) -> u8 {
        Pdsleepcfg3SramIf23Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf24Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf24Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf24Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf24Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf24Ppd {
        Pdsleepcfg3SramIf24Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf24Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf24Ppd) -> u8 {
        Pdsleepcfg3SramIf24Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf25Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf25Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf25Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf25Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf25Ppd {
        Pdsleepcfg3SramIf25Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf25Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf25Ppd) -> u8 {
        Pdsleepcfg3SramIf25Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf26Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf26Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf26Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf26Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf26Ppd {
        Pdsleepcfg3SramIf26Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf26Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf26Ppd) -> u8 {
        Pdsleepcfg3SramIf26Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf27Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf27Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf27Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf27Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf27Ppd {
        Pdsleepcfg3SramIf27Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf27Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf27Ppd) -> u8 {
        Pdsleepcfg3SramIf27Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf28Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf28Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf28Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf28Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf28Ppd {
        Pdsleepcfg3SramIf28Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf28Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf28Ppd) -> u8 {
        Pdsleepcfg3SramIf28Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf29Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf29Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf29Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf29Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf29Ppd {
        Pdsleepcfg3SramIf29Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf29Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf29Ppd) -> u8 {
        Pdsleepcfg3SramIf29Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf2Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf2Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf2Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf2Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf2Ppd {
        Pdsleepcfg3SramIf2Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf2Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf2Ppd) -> u8 {
        Pdsleepcfg3SramIf2Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf3Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf3Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf3Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf3Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf3Ppd {
        Pdsleepcfg3SramIf3Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf3Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf3Ppd) -> u8 {
        Pdsleepcfg3SramIf3Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf4Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf4Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf4Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf4Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf4Ppd {
        Pdsleepcfg3SramIf4Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf4Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf4Ppd) -> u8 {
        Pdsleepcfg3SramIf4Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf5Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf5Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf5Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf5Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf5Ppd {
        Pdsleepcfg3SramIf5Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf5Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf5Ppd) -> u8 {
        Pdsleepcfg3SramIf5Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf6Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf6Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf6Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf6Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf6Ppd {
        Pdsleepcfg3SramIf6Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf6Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf6Ppd) -> u8 {
        Pdsleepcfg3SramIf6Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf7Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf7Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf7Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf7Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf7Ppd {
        Pdsleepcfg3SramIf7Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf7Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf7Ppd) -> u8 {
        Pdsleepcfg3SramIf7Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf8Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf8Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf8Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf8Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf8Ppd {
        Pdsleepcfg3SramIf8Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf8Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf8Ppd) -> u8 {
        Pdsleepcfg3SramIf8Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdsleepcfg3SramIf9Ppd {
    #[doc = "enable"]
    ENABLED = 0x0,
    #[doc = "power down"]
    POWER_DOWN = 0x01,
}
impl Pdsleepcfg3SramIf9Ppd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdsleepcfg3SramIf9Ppd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdsleepcfg3SramIf9Ppd {
    #[inline(always)]
    fn from(val: u8) -> Pdsleepcfg3SramIf9Ppd {
        Pdsleepcfg3SramIf9Ppd::from_bits(val)
    }
}
impl From<Pdsleepcfg3SramIf9Ppd> for u8 {
    #[inline(always)]
    fn from(val: Pdsleepcfg3SramIf9Ppd) -> u8 {
        Pdsleepcfg3SramIf9Ppd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pmc {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Pmc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pmc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pmc {
    #[inline(always)]
    fn from(val: u8) -> Pmc {
        Pmc::from_bits(val)
    }
}
impl From<Pmc> for u8 {
    #[inline(always)]
    fn from(val: Pmc) -> u8 {
        Pmc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolDevClk {
    #[doc = "Falling edge of device need_clock triggers wake-up."]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge of device need_clock triggers wake-up."]
    RISING_EDGE = 0x01,
}
impl PolDevClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolDevClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolDevClk {
    #[inline(always)]
    fn from(val: u8) -> PolDevClk {
        PolDevClk::from_bits(val)
    }
}
impl From<PolDevClk> for u8 {
    #[inline(always)]
    fn from(val: PolDevClk) -> u8 {
        PolDevClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PolHostClk {
    #[doc = "Falling edge of host need_clock triggers wake-up."]
    FALLING_EDGE = 0x0,
    #[doc = "Rising edge of host need_clock triggers wake-up."]
    RISING_EDGE = 0x01,
}
impl PolHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PolHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PolHostClk {
    #[inline(always)]
    fn from(val: u8) -> PolHostClk {
        PolHostClk::from_bits(val)
    }
}
impl From<PolHostClk> for u8 {
    #[inline(always)]
    fn from(val: PolHostClk) -> u8 {
        PolHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rbbkeepst {
    #[doc = "Use value of RBB_PD in PDRUNCFG on wakeup."]
    RBBKEEPST_0 = 0x0,
    #[doc = "Copy PDSLEEPCFG RBB_PD value to PDRUNCFG RBB_PD on wakeup to keep RBB state."]
    RBBKEEPST_1 = 0x01,
}
impl Rbbkeepst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rbbkeepst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rbbkeepst {
    #[inline(always)]
    fn from(val: u8) -> Rbbkeepst {
        Rbbkeepst::from_bits(val)
    }
}
impl From<Rbbkeepst> for u8 {
    #[inline(always)]
    fn from(val: Rbbkeepst) -> u8 {
        Rbbkeepst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdio0 {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Sdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio0 {
    #[inline(always)]
    fn from(val: u8) -> Sdio0 {
        Sdio0::from_bits(val)
    }
}
impl From<Sdio0> for u8 {
    #[inline(always)]
    fn from(val: Sdio0) -> u8 {
        Sdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdio1 {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Sdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio1 {
    #[inline(always)]
    fn from(val: u8) -> Sdio1 {
        Sdio1::from_bits(val)
    }
}
impl From<Sdio1> for u8 {
    #[inline(always)]
    fn from(val: Sdio1) -> u8 {
        Sdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf0 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf0 {
    #[inline(always)]
    fn from(val: u8) -> SramIf0 {
        SramIf0::from_bits(val)
    }
}
impl From<SramIf0> for u8 {
    #[inline(always)]
    fn from(val: SramIf0) -> u8 {
        SramIf0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf1 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf1 {
    #[inline(always)]
    fn from(val: u8) -> SramIf1 {
        SramIf1::from_bits(val)
    }
}
impl From<SramIf1> for u8 {
    #[inline(always)]
    fn from(val: SramIf1) -> u8 {
        SramIf1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf10 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf10 {
    #[inline(always)]
    fn from(val: u8) -> SramIf10 {
        SramIf10::from_bits(val)
    }
}
impl From<SramIf10> for u8 {
    #[inline(always)]
    fn from(val: SramIf10) -> u8 {
        SramIf10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf11 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf11 {
    #[inline(always)]
    fn from(val: u8) -> SramIf11 {
        SramIf11::from_bits(val)
    }
}
impl From<SramIf11> for u8 {
    #[inline(always)]
    fn from(val: SramIf11) -> u8 {
        SramIf11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf12 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf12 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf12 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf12 {
    #[inline(always)]
    fn from(val: u8) -> SramIf12 {
        SramIf12::from_bits(val)
    }
}
impl From<SramIf12> for u8 {
    #[inline(always)]
    fn from(val: SramIf12) -> u8 {
        SramIf12::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf13 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf13 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf13 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf13 {
    #[inline(always)]
    fn from(val: u8) -> SramIf13 {
        SramIf13::from_bits(val)
    }
}
impl From<SramIf13> for u8 {
    #[inline(always)]
    fn from(val: SramIf13) -> u8 {
        SramIf13::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf14 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf14 {
    #[inline(always)]
    fn from(val: u8) -> SramIf14 {
        SramIf14::from_bits(val)
    }
}
impl From<SramIf14> for u8 {
    #[inline(always)]
    fn from(val: SramIf14) -> u8 {
        SramIf14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf15 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf15 {
    #[inline(always)]
    fn from(val: u8) -> SramIf15 {
        SramIf15::from_bits(val)
    }
}
impl From<SramIf15> for u8 {
    #[inline(always)]
    fn from(val: SramIf15) -> u8 {
        SramIf15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf16 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf16 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf16 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf16 {
    #[inline(always)]
    fn from(val: u8) -> SramIf16 {
        SramIf16::from_bits(val)
    }
}
impl From<SramIf16> for u8 {
    #[inline(always)]
    fn from(val: SramIf16) -> u8 {
        SramIf16::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf17 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf17 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf17 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf17 {
    #[inline(always)]
    fn from(val: u8) -> SramIf17 {
        SramIf17::from_bits(val)
    }
}
impl From<SramIf17> for u8 {
    #[inline(always)]
    fn from(val: SramIf17) -> u8 {
        SramIf17::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf19 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf19 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf19 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf19 {
    #[inline(always)]
    fn from(val: u8) -> SramIf19 {
        SramIf19::from_bits(val)
    }
}
impl From<SramIf19> for u8 {
    #[inline(always)]
    fn from(val: SramIf19) -> u8 {
        SramIf19::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf2 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf2 {
    #[inline(always)]
    fn from(val: u8) -> SramIf2 {
        SramIf2::from_bits(val)
    }
}
impl From<SramIf2> for u8 {
    #[inline(always)]
    fn from(val: SramIf2) -> u8 {
        SramIf2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf20 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf20 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf20 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf20 {
    #[inline(always)]
    fn from(val: u8) -> SramIf20 {
        SramIf20::from_bits(val)
    }
}
impl From<SramIf20> for u8 {
    #[inline(always)]
    fn from(val: SramIf20) -> u8 {
        SramIf20::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf21 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf21 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf21 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf21 {
    #[inline(always)]
    fn from(val: u8) -> SramIf21 {
        SramIf21::from_bits(val)
    }
}
impl From<SramIf21> for u8 {
    #[inline(always)]
    fn from(val: SramIf21) -> u8 {
        SramIf21::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf22 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf22 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf22 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf22 {
    #[inline(always)]
    fn from(val: u8) -> SramIf22 {
        SramIf22::from_bits(val)
    }
}
impl From<SramIf22> for u8 {
    #[inline(always)]
    fn from(val: SramIf22) -> u8 {
        SramIf22::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf23 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf23 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf23 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf23 {
    #[inline(always)]
    fn from(val: u8) -> SramIf23 {
        SramIf23::from_bits(val)
    }
}
impl From<SramIf23> for u8 {
    #[inline(always)]
    fn from(val: SramIf23) -> u8 {
        SramIf23::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf24 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf24 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf24 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf24 {
    #[inline(always)]
    fn from(val: u8) -> SramIf24 {
        SramIf24::from_bits(val)
    }
}
impl From<SramIf24> for u8 {
    #[inline(always)]
    fn from(val: SramIf24) -> u8 {
        SramIf24::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf25 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf25 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf25 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf25 {
    #[inline(always)]
    fn from(val: u8) -> SramIf25 {
        SramIf25::from_bits(val)
    }
}
impl From<SramIf25> for u8 {
    #[inline(always)]
    fn from(val: SramIf25) -> u8 {
        SramIf25::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf26 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf26 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf26 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf26 {
    #[inline(always)]
    fn from(val: u8) -> SramIf26 {
        SramIf26::from_bits(val)
    }
}
impl From<SramIf26> for u8 {
    #[inline(always)]
    fn from(val: SramIf26) -> u8 {
        SramIf26::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf27 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf27 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf27 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf27 {
    #[inline(always)]
    fn from(val: u8) -> SramIf27 {
        SramIf27::from_bits(val)
    }
}
impl From<SramIf27> for u8 {
    #[inline(always)]
    fn from(val: SramIf27) -> u8 {
        SramIf27::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf28 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf28 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf28 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf28 {
    #[inline(always)]
    fn from(val: u8) -> SramIf28 {
        SramIf28::from_bits(val)
    }
}
impl From<SramIf28> for u8 {
    #[inline(always)]
    fn from(val: SramIf28) -> u8 {
        SramIf28::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf29 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf29 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf29 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf29 {
    #[inline(always)]
    fn from(val: u8) -> SramIf29 {
        SramIf29::from_bits(val)
    }
}
impl From<SramIf29> for u8 {
    #[inline(always)]
    fn from(val: SramIf29) -> u8 {
        SramIf29::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf3 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf3 {
    #[inline(always)]
    fn from(val: u8) -> SramIf3 {
        SramIf3::from_bits(val)
    }
}
impl From<SramIf3> for u8 {
    #[inline(always)]
    fn from(val: SramIf3) -> u8 {
        SramIf3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf4 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf4 {
    #[inline(always)]
    fn from(val: u8) -> SramIf4 {
        SramIf4::from_bits(val)
    }
}
impl From<SramIf4> for u8 {
    #[inline(always)]
    fn from(val: SramIf4) -> u8 {
        SramIf4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf5 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf5 {
    #[inline(always)]
    fn from(val: u8) -> SramIf5 {
        SramIf5::from_bits(val)
    }
}
impl From<SramIf5> for u8 {
    #[inline(always)]
    fn from(val: SramIf5) -> u8 {
        SramIf5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf6 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf6 {
    #[inline(always)]
    fn from(val: u8) -> SramIf6 {
        SramIf6::from_bits(val)
    }
}
impl From<SramIf6> for u8 {
    #[inline(always)]
    fn from(val: SramIf6) -> u8 {
        SramIf6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf7 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf7 {
    #[inline(always)]
    fn from(val: u8) -> SramIf7 {
        SramIf7::from_bits(val)
    }
}
impl From<SramIf7> for u8 {
    #[inline(always)]
    fn from(val: SramIf7) -> u8 {
        SramIf7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf8 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf8 {
    #[inline(always)]
    fn from(val: u8) -> SramIf8 {
        SramIf8::from_bits(val)
    }
}
impl From<SramIf8> for u8 {
    #[inline(always)]
    fn from(val: SramIf8) -> u8 {
        SramIf8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramIf9 {
    #[doc = "Enable Auto-Clk"]
    ENABLED = 0x0,
    #[doc = "Disable Auto-Clk"]
    DISABLED = 0x01,
}
impl SramIf9 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramIf9 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramIf9 {
    #[inline(always)]
    fn from(val: u8) -> SramIf9 {
        SramIf9::from_bits(val)
    }
}
impl From<SramIf9> for u8 {
    #[inline(always)]
    fn from(val: SramIf9) -> u8 {
        SramIf9::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrAcmp {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrAcmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrAcmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrAcmp {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrAcmp {
        Starten0ClrAcmp::from_bits(val)
    }
}
impl From<Starten0ClrAcmp> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrAcmp) -> u8 {
        Starten0ClrAcmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrAdc0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrAdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrAdc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrAdc0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrAdc0 {
        Starten0ClrAdc0::from_bits(val)
    }
}
impl From<Starten0ClrAdc0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrAdc0) -> u8 {
        Starten0ClrAdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrCt32bit0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrCt32bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrCt32bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrCt32bit0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrCt32bit0 {
        Starten0ClrCt32bit0::from_bits(val)
    }
}
impl From<Starten0ClrCt32bit0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrCt32bit0) -> u8 {
        Starten0ClrCt32bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrCt32bit1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrCt32bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrCt32bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrCt32bit1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrCt32bit1 {
        Starten0ClrCt32bit1::from_bits(val)
    }
}
impl From<Starten0ClrCt32bit1> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrCt32bit1) -> u8 {
        Starten0ClrCt32bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrCt32bit3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrCt32bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrCt32bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrCt32bit3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrCt32bit3 {
        Starten0ClrCt32bit3::from_bits(val)
    }
}
impl From<Starten0ClrCt32bit3> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrCt32bit3) -> u8 {
        Starten0ClrCt32bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrDmac0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrDmac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrDmac0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrDmac0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrDmac0 {
        Starten0ClrDmac0::from_bits(val)
    }
}
impl From<Starten0ClrDmac0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrDmac0) -> u8 {
        Starten0ClrDmac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrDmic0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrDmic0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrDmic0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrDmic0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrDmic0 {
        Starten0ClrDmic0::from_bits(val)
    }
}
impl From<Starten0ClrDmic0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrDmic0) -> u8 {
        Starten0ClrDmic0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm0 {
        Starten0ClrFlexcomm0::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm0) -> u8 {
        Starten0ClrFlexcomm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm1 {
        Starten0ClrFlexcomm1::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm1> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm1) -> u8 {
        Starten0ClrFlexcomm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm14 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm14 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm14 {
        Starten0ClrFlexcomm14::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm14> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm14) -> u8 {
        Starten0ClrFlexcomm14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm15 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm15 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm15 {
        Starten0ClrFlexcomm15::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm15> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm15) -> u8 {
        Starten0ClrFlexcomm15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm2 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm2 {
        Starten0ClrFlexcomm2::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm2> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm2) -> u8 {
        Starten0ClrFlexcomm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm3 {
        Starten0ClrFlexcomm3::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm3> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm3) -> u8 {
        Starten0ClrFlexcomm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm4 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm4 {
        Starten0ClrFlexcomm4::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm4> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm4) -> u8 {
        Starten0ClrFlexcomm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrFlexcomm5 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrFlexcomm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrFlexcomm5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrFlexcomm5 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrFlexcomm5 {
        Starten0ClrFlexcomm5::from_bits(val)
    }
}
impl From<Starten0ClrFlexcomm5> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrFlexcomm5) -> u8 {
        Starten0ClrFlexcomm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrGpioInt0Irq0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrGpioInt0Irq0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrGpioInt0Irq0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrGpioInt0Irq0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrGpioInt0Irq0 {
        Starten0ClrGpioInt0Irq0::from_bits(val)
    }
}
impl From<Starten0ClrGpioInt0Irq0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrGpioInt0Irq0) -> u8 {
        Starten0ClrGpioInt0Irq0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrGpioInt0Irq1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrGpioInt0Irq1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrGpioInt0Irq1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrGpioInt0Irq1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrGpioInt0Irq1 {
        Starten0ClrGpioInt0Irq1::from_bits(val)
    }
}
impl From<Starten0ClrGpioInt0Irq1> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrGpioInt0Irq1) -> u8 {
        Starten0ClrGpioInt0Irq1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrGpioInt0Irq2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrGpioInt0Irq2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrGpioInt0Irq2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrGpioInt0Irq2 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrGpioInt0Irq2 {
        Starten0ClrGpioInt0Irq2::from_bits(val)
    }
}
impl From<Starten0ClrGpioInt0Irq2> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrGpioInt0Irq2) -> u8 {
        Starten0ClrGpioInt0Irq2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrGpioInt0Irq3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrGpioInt0Irq3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrGpioInt0Irq3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrGpioInt0Irq3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrGpioInt0Irq3 {
        Starten0ClrGpioInt0Irq3::from_bits(val)
    }
}
impl From<Starten0ClrGpioInt0Irq3> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrGpioInt0Irq3) -> u8 {
        Starten0ClrGpioInt0Irq3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrHwvad0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrHwvad0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrHwvad0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrHwvad0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrHwvad0 {
        Starten0ClrHwvad0::from_bits(val)
    }
}
impl From<Starten0ClrHwvad0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrHwvad0) -> u8 {
        Starten0ClrHwvad0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrHypervisor {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrHypervisor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrHypervisor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrHypervisor {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrHypervisor {
        Starten0ClrHypervisor::from_bits(val)
    }
}
impl From<Starten0ClrHypervisor> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrHypervisor) -> u8 {
        Starten0ClrHypervisor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrMrt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrMrt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrMrt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrMrt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrMrt0 {
        Starten0ClrMrt0::from_bits(val)
    }
}
impl From<Starten0ClrMrt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrMrt0) -> u8 {
        Starten0ClrMrt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrNshsgpioInt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrNshsgpioInt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrNshsgpioInt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrNshsgpioInt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrNshsgpioInt0 {
        Starten0ClrNshsgpioInt0::from_bits(val)
    }
}
impl From<Starten0ClrNshsgpioInt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrNshsgpioInt0) -> u8 {
        Starten0ClrNshsgpioInt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrNshsgpioInt1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrNshsgpioInt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrNshsgpioInt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrNshsgpioInt1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrNshsgpioInt1 {
        Starten0ClrNshsgpioInt1::from_bits(val)
    }
}
impl From<Starten0ClrNshsgpioInt1> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrNshsgpioInt1) -> u8 {
        Starten0ClrNshsgpioInt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrRng {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrRng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrRng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrRng {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrRng {
        Starten0ClrRng::from_bits(val)
    }
}
impl From<Starten0ClrRng> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrRng) -> u8 {
        Starten0ClrRng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrSct0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrSct0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrSct0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrSct0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrSct0 {
        Starten0ClrSct0::from_bits(val)
    }
}
impl From<Starten0ClrSct0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrSct0) -> u8 {
        Starten0ClrSct0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrSecureviolation {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrSecureviolation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrSecureviolation {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrSecureviolation {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrSecureviolation {
        Starten0ClrSecureviolation::from_bits(val)
    }
}
impl From<Starten0ClrSecureviolation> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrSecureviolation) -> u8 {
        Starten0ClrSecureviolation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrUtick0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrUtick0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrUtick0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrUtick0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrUtick0 {
        Starten0ClrUtick0::from_bits(val)
    }
}
impl From<Starten0ClrUtick0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrUtick0) -> u8 {
        Starten0ClrUtick0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0ClrWdt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN0 Bit"]
    CLR_START_EN0 = 0x01,
}
impl Starten0ClrWdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0ClrWdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0ClrWdt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0ClrWdt0 {
        Starten0ClrWdt0::from_bits(val)
    }
}
impl From<Starten0ClrWdt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0ClrWdt0) -> u8 {
        Starten0ClrWdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetAcmp {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetAcmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetAcmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetAcmp {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetAcmp {
        Starten0SetAcmp::from_bits(val)
    }
}
impl From<Starten0SetAcmp> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetAcmp) -> u8 {
        Starten0SetAcmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetAdc0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetAdc0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetAdc0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetAdc0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetAdc0 {
        Starten0SetAdc0::from_bits(val)
    }
}
impl From<Starten0SetAdc0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetAdc0) -> u8 {
        Starten0SetAdc0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetCt32bit0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetCt32bit0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetCt32bit0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetCt32bit0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetCt32bit0 {
        Starten0SetCt32bit0::from_bits(val)
    }
}
impl From<Starten0SetCt32bit0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetCt32bit0) -> u8 {
        Starten0SetCt32bit0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetCt32bit1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetCt32bit1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetCt32bit1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetCt32bit1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetCt32bit1 {
        Starten0SetCt32bit1::from_bits(val)
    }
}
impl From<Starten0SetCt32bit1> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetCt32bit1) -> u8 {
        Starten0SetCt32bit1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetCt32bit3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetCt32bit3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetCt32bit3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetCt32bit3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetCt32bit3 {
        Starten0SetCt32bit3::from_bits(val)
    }
}
impl From<Starten0SetCt32bit3> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetCt32bit3) -> u8 {
        Starten0SetCt32bit3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetDmac0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetDmac0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetDmac0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetDmac0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetDmac0 {
        Starten0SetDmac0::from_bits(val)
    }
}
impl From<Starten0SetDmac0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetDmac0) -> u8 {
        Starten0SetDmac0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetDmic0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetDmic0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetDmic0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetDmic0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetDmic0 {
        Starten0SetDmic0::from_bits(val)
    }
}
impl From<Starten0SetDmic0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetDmic0) -> u8 {
        Starten0SetDmic0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm0 {
        Starten0SetFlexcomm0::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm0) -> u8 {
        Starten0SetFlexcomm0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm1 {
        Starten0SetFlexcomm1::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm1> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm1) -> u8 {
        Starten0SetFlexcomm1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm14 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm14 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm14 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm14 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm14 {
        Starten0SetFlexcomm14::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm14> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm14) -> u8 {
        Starten0SetFlexcomm14::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm15 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm15 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm15 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm15 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm15 {
        Starten0SetFlexcomm15::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm15> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm15) -> u8 {
        Starten0SetFlexcomm15::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm2 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm2 {
        Starten0SetFlexcomm2::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm2> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm2) -> u8 {
        Starten0SetFlexcomm2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm3 {
        Starten0SetFlexcomm3::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm3> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm3) -> u8 {
        Starten0SetFlexcomm3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm4 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm4 {
        Starten0SetFlexcomm4::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm4> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm4) -> u8 {
        Starten0SetFlexcomm4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetFlexcomm5 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetFlexcomm5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetFlexcomm5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetFlexcomm5 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetFlexcomm5 {
        Starten0SetFlexcomm5::from_bits(val)
    }
}
impl From<Starten0SetFlexcomm5> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetFlexcomm5) -> u8 {
        Starten0SetFlexcomm5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetGpioInt0Irq0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetGpioInt0Irq0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetGpioInt0Irq0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetGpioInt0Irq0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetGpioInt0Irq0 {
        Starten0SetGpioInt0Irq0::from_bits(val)
    }
}
impl From<Starten0SetGpioInt0Irq0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetGpioInt0Irq0) -> u8 {
        Starten0SetGpioInt0Irq0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetGpioInt0Irq1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetGpioInt0Irq1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetGpioInt0Irq1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetGpioInt0Irq1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetGpioInt0Irq1 {
        Starten0SetGpioInt0Irq1::from_bits(val)
    }
}
impl From<Starten0SetGpioInt0Irq1> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetGpioInt0Irq1) -> u8 {
        Starten0SetGpioInt0Irq1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetGpioInt0Irq2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetGpioInt0Irq2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetGpioInt0Irq2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetGpioInt0Irq2 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetGpioInt0Irq2 {
        Starten0SetGpioInt0Irq2::from_bits(val)
    }
}
impl From<Starten0SetGpioInt0Irq2> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetGpioInt0Irq2) -> u8 {
        Starten0SetGpioInt0Irq2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetGpioInt0Irq3 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetGpioInt0Irq3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetGpioInt0Irq3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetGpioInt0Irq3 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetGpioInt0Irq3 {
        Starten0SetGpioInt0Irq3::from_bits(val)
    }
}
impl From<Starten0SetGpioInt0Irq3> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetGpioInt0Irq3) -> u8 {
        Starten0SetGpioInt0Irq3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetHwvad0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetHwvad0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetHwvad0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetHwvad0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetHwvad0 {
        Starten0SetHwvad0::from_bits(val)
    }
}
impl From<Starten0SetHwvad0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetHwvad0) -> u8 {
        Starten0SetHwvad0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetHypervisor {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetHypervisor {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetHypervisor {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetHypervisor {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetHypervisor {
        Starten0SetHypervisor::from_bits(val)
    }
}
impl From<Starten0SetHypervisor> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetHypervisor) -> u8 {
        Starten0SetHypervisor::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetMrt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetMrt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetMrt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetMrt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetMrt0 {
        Starten0SetMrt0::from_bits(val)
    }
}
impl From<Starten0SetMrt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetMrt0) -> u8 {
        Starten0SetMrt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetNshsgpioInt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetNshsgpioInt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetNshsgpioInt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetNshsgpioInt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetNshsgpioInt0 {
        Starten0SetNshsgpioInt0::from_bits(val)
    }
}
impl From<Starten0SetNshsgpioInt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetNshsgpioInt0) -> u8 {
        Starten0SetNshsgpioInt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetNshsgpioInt1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetNshsgpioInt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetNshsgpioInt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetNshsgpioInt1 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetNshsgpioInt1 {
        Starten0SetNshsgpioInt1::from_bits(val)
    }
}
impl From<Starten0SetNshsgpioInt1> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetNshsgpioInt1) -> u8 {
        Starten0SetNshsgpioInt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetRng {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetRng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetRng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetRng {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetRng {
        Starten0SetRng::from_bits(val)
    }
}
impl From<Starten0SetRng> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetRng) -> u8 {
        Starten0SetRng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetSct0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetSct0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetSct0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetSct0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetSct0 {
        Starten0SetSct0::from_bits(val)
    }
}
impl From<Starten0SetSct0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetSct0) -> u8 {
        Starten0SetSct0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetSecureviolation {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetSecureviolation {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetSecureviolation {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetSecureviolation {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetSecureviolation {
        Starten0SetSecureviolation::from_bits(val)
    }
}
impl From<Starten0SetSecureviolation> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetSecureviolation) -> u8 {
        Starten0SetSecureviolation::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetUtick0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetUtick0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetUtick0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetUtick0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetUtick0 {
        Starten0SetUtick0::from_bits(val)
    }
}
impl From<Starten0SetUtick0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetUtick0) -> u8 {
        Starten0SetUtick0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten0SetWdt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN0 Bit"]
    SET_START_EN0 = 0x01,
}
impl Starten0SetWdt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten0SetWdt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten0SetWdt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten0SetWdt0 {
        Starten0SetWdt0::from_bits(val)
    }
}
impl From<Starten0SetWdt0> for u8 {
    #[inline(always)]
    fn from(val: Starten0SetWdt0) -> u8 {
        Starten0SetWdt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrCasper {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrCasper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrCasper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrCasper {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrCasper {
        Starten1ClrCasper::from_bits(val)
    }
}
impl From<Starten1ClrCasper> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrCasper) -> u8 {
        Starten1ClrCasper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrCt32bit2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrCt32bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrCt32bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrCt32bit2 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrCt32bit2 {
        Starten1ClrCt32bit2::from_bits(val)
    }
}
impl From<Starten1ClrCt32bit2> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrCt32bit2) -> u8 {
        Starten1ClrCt32bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrCt32bit4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrCt32bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrCt32bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrCt32bit4 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrCt32bit4 {
        Starten1ClrCt32bit4::from_bits(val)
    }
}
impl From<Starten1ClrCt32bit4> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrCt32bit4) -> u8 {
        Starten1ClrCt32bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrDmac1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrDmac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrDmac1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrDmac1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrDmac1 {
        Starten1ClrDmac1::from_bits(val)
    }
}
impl From<Starten1ClrDmac1> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrDmac1) -> u8 {
        Starten1ClrDmac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrFlexcomm6 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrFlexcomm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrFlexcomm6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrFlexcomm6 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrFlexcomm6 {
        Starten1ClrFlexcomm6::from_bits(val)
    }
}
impl From<Starten1ClrFlexcomm6> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrFlexcomm6) -> u8 {
        Starten1ClrFlexcomm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrFlexcomm7 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrFlexcomm7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrFlexcomm7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrFlexcomm7 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrFlexcomm7 {
        Starten1ClrFlexcomm7::from_bits(val)
    }
}
impl From<Starten1ClrFlexcomm7> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrFlexcomm7) -> u8 {
        Starten1ClrFlexcomm7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrFlexspi {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrFlexspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrFlexspi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrFlexspi {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrFlexspi {
        Starten1ClrFlexspi::from_bits(val)
    }
}
impl From<Starten1ClrFlexspi> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrFlexspi) -> u8 {
        Starten1ClrFlexspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrGpioInt0Irq4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrGpioInt0Irq4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrGpioInt0Irq4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrGpioInt0Irq4 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrGpioInt0Irq4 {
        Starten1ClrGpioInt0Irq4::from_bits(val)
    }
}
impl From<Starten1ClrGpioInt0Irq4> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrGpioInt0Irq4) -> u8 {
        Starten1ClrGpioInt0Irq4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrGpioInt0Irq5 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrGpioInt0Irq5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrGpioInt0Irq5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrGpioInt0Irq5 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrGpioInt0Irq5 {
        Starten1ClrGpioInt0Irq5::from_bits(val)
    }
}
impl From<Starten1ClrGpioInt0Irq5> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrGpioInt0Irq5) -> u8 {
        Starten1ClrGpioInt0Irq5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrGpioInt0Irq6 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrGpioInt0Irq6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrGpioInt0Irq6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrGpioInt0Irq6 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrGpioInt0Irq6 {
        Starten1ClrGpioInt0Irq6::from_bits(val)
    }
}
impl From<Starten1ClrGpioInt0Irq6> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrGpioInt0Irq6) -> u8 {
        Starten1ClrGpioInt0Irq6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrGpioInt0Irq7 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrGpioInt0Irq7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrGpioInt0Irq7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrGpioInt0Irq7 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrGpioInt0Irq7 {
        Starten1ClrGpioInt0Irq7::from_bits(val)
    }
}
impl From<Starten1ClrGpioInt0Irq7> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrGpioInt0Irq7) -> u8 {
        Starten1ClrGpioInt0Irq7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrI3c0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrI3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrI3c0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrI3c0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrI3c0 {
        Starten1ClrI3c0::from_bits(val)
    }
}
impl From<Starten1ClrI3c0> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrI3c0) -> u8 {
        Starten1ClrI3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrMu {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrMu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrMu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrMu {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrMu {
        Starten1ClrMu::from_bits(val)
    }
}
impl From<Starten1ClrMu> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrMu) -> u8 {
        Starten1ClrMu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrOsEventTimerWu {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrOsEventTimerWu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrOsEventTimerWu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrOsEventTimerWu {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrOsEventTimerWu {
        Starten1ClrOsEventTimerWu::from_bits(val)
    }
}
impl From<Starten1ClrOsEventTimerWu> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrOsEventTimerWu) -> u8 {
        Starten1ClrOsEventTimerWu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrPmic {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrPmic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrPmic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrPmic {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrPmic {
        Starten1ClrPmic::from_bits(val)
    }
}
impl From<Starten1ClrPmic> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrPmic) -> u8 {
        Starten1ClrPmic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrPowerquad {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrPowerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrPowerquad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrPowerquad {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrPowerquad {
        Starten1ClrPowerquad::from_bits(val)
    }
}
impl From<Starten1ClrPowerquad> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrPowerquad) -> u8 {
        Starten1ClrPowerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrPuf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrPuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrPuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrPuf {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrPuf {
        Starten1ClrPuf::from_bits(val)
    }
}
impl From<Starten1ClrPuf> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrPuf) -> u8 {
        Starten1ClrPuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrRtcLite0AlarmOrWakeup {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrRtcLite0AlarmOrWakeup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrRtcLite0AlarmOrWakeup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrRtcLite0AlarmOrWakeup {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrRtcLite0AlarmOrWakeup {
        Starten1ClrRtcLite0AlarmOrWakeup::from_bits(val)
    }
}
impl From<Starten1ClrRtcLite0AlarmOrWakeup> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrRtcLite0AlarmOrWakeup) -> u8 {
        Starten1ClrRtcLite0AlarmOrWakeup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrSdio0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrSdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrSdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrSdio0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrSdio0 {
        Starten1ClrSdio0::from_bits(val)
    }
}
impl From<Starten1ClrSdio0> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrSdio0) -> u8 {
        Starten1ClrSdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrSdio1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrSdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrSdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrSdio1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrSdio1 {
        Starten1ClrSdio1::from_bits(val)
    }
}
impl From<Starten1ClrSdio1> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrSdio1) -> u8 {
        Starten1ClrSdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrSha {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrSha {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrSha {
        Starten1ClrSha::from_bits(val)
    }
}
impl From<Starten1ClrSha> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrSha) -> u8 {
        Starten1ClrSha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrShsgpioInt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrShsgpioInt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrShsgpioInt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrShsgpioInt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrShsgpioInt0 {
        Starten1ClrShsgpioInt0::from_bits(val)
    }
}
impl From<Starten1ClrShsgpioInt0> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrShsgpioInt0) -> u8 {
        Starten1ClrShsgpioInt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrShsgpioInt1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrShsgpioInt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrShsgpioInt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrShsgpioInt1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrShsgpioInt1 {
        Starten1ClrShsgpioInt1::from_bits(val)
    }
}
impl From<Starten1ClrShsgpioInt1> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrShsgpioInt1) -> u8 {
        Starten1ClrShsgpioInt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrUsbIrq {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrUsbIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrUsbIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrUsbIrq {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrUsbIrq {
        Starten1ClrUsbIrq::from_bits(val)
    }
}
impl From<Starten1ClrUsbIrq> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrUsbIrq) -> u8 {
        Starten1ClrUsbIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1ClrUsbNeedclk {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the START_EN1 Bit"]
    CLR_START_EN1 = 0x01,
}
impl Starten1ClrUsbNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1ClrUsbNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1ClrUsbNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Starten1ClrUsbNeedclk {
        Starten1ClrUsbNeedclk::from_bits(val)
    }
}
impl From<Starten1ClrUsbNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Starten1ClrUsbNeedclk) -> u8 {
        Starten1ClrUsbNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetCasper {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetCasper {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetCasper {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetCasper {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetCasper {
        Starten1SetCasper::from_bits(val)
    }
}
impl From<Starten1SetCasper> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetCasper) -> u8 {
        Starten1SetCasper::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetCt32bit2 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetCt32bit2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetCt32bit2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetCt32bit2 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetCt32bit2 {
        Starten1SetCt32bit2::from_bits(val)
    }
}
impl From<Starten1SetCt32bit2> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetCt32bit2) -> u8 {
        Starten1SetCt32bit2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetCt32bit4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetCt32bit4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetCt32bit4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetCt32bit4 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetCt32bit4 {
        Starten1SetCt32bit4::from_bits(val)
    }
}
impl From<Starten1SetCt32bit4> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetCt32bit4) -> u8 {
        Starten1SetCt32bit4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetDmac1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetDmac1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetDmac1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetDmac1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetDmac1 {
        Starten1SetDmac1::from_bits(val)
    }
}
impl From<Starten1SetDmac1> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetDmac1) -> u8 {
        Starten1SetDmac1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetFlexcomm6 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetFlexcomm6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetFlexcomm6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetFlexcomm6 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetFlexcomm6 {
        Starten1SetFlexcomm6::from_bits(val)
    }
}
impl From<Starten1SetFlexcomm6> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetFlexcomm6) -> u8 {
        Starten1SetFlexcomm6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetFlexcomm7 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetFlexcomm7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetFlexcomm7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetFlexcomm7 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetFlexcomm7 {
        Starten1SetFlexcomm7::from_bits(val)
    }
}
impl From<Starten1SetFlexcomm7> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetFlexcomm7) -> u8 {
        Starten1SetFlexcomm7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetFlexspi {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetFlexspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetFlexspi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetFlexspi {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetFlexspi {
        Starten1SetFlexspi::from_bits(val)
    }
}
impl From<Starten1SetFlexspi> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetFlexspi) -> u8 {
        Starten1SetFlexspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetGpioInt0Irq4 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetGpioInt0Irq4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetGpioInt0Irq4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetGpioInt0Irq4 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetGpioInt0Irq4 {
        Starten1SetGpioInt0Irq4::from_bits(val)
    }
}
impl From<Starten1SetGpioInt0Irq4> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetGpioInt0Irq4) -> u8 {
        Starten1SetGpioInt0Irq4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetGpioInt0Irq5 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetGpioInt0Irq5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetGpioInt0Irq5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetGpioInt0Irq5 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetGpioInt0Irq5 {
        Starten1SetGpioInt0Irq5::from_bits(val)
    }
}
impl From<Starten1SetGpioInt0Irq5> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetGpioInt0Irq5) -> u8 {
        Starten1SetGpioInt0Irq5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetGpioInt0Irq6 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetGpioInt0Irq6 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetGpioInt0Irq6 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetGpioInt0Irq6 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetGpioInt0Irq6 {
        Starten1SetGpioInt0Irq6::from_bits(val)
    }
}
impl From<Starten1SetGpioInt0Irq6> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetGpioInt0Irq6) -> u8 {
        Starten1SetGpioInt0Irq6::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetGpioInt0Irq7 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetGpioInt0Irq7 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetGpioInt0Irq7 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetGpioInt0Irq7 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetGpioInt0Irq7 {
        Starten1SetGpioInt0Irq7::from_bits(val)
    }
}
impl From<Starten1SetGpioInt0Irq7> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetGpioInt0Irq7) -> u8 {
        Starten1SetGpioInt0Irq7::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetI3c0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetI3c0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetI3c0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetI3c0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetI3c0 {
        Starten1SetI3c0::from_bits(val)
    }
}
impl From<Starten1SetI3c0> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetI3c0) -> u8 {
        Starten1SetI3c0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetMu {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetMu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetMu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetMu {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetMu {
        Starten1SetMu::from_bits(val)
    }
}
impl From<Starten1SetMu> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetMu) -> u8 {
        Starten1SetMu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetOsEventTimerWu {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetOsEventTimerWu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetOsEventTimerWu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetOsEventTimerWu {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetOsEventTimerWu {
        Starten1SetOsEventTimerWu::from_bits(val)
    }
}
impl From<Starten1SetOsEventTimerWu> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetOsEventTimerWu) -> u8 {
        Starten1SetOsEventTimerWu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetPmic {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetPmic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetPmic {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetPmic {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetPmic {
        Starten1SetPmic::from_bits(val)
    }
}
impl From<Starten1SetPmic> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetPmic) -> u8 {
        Starten1SetPmic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetPowerquad {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetPowerquad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetPowerquad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetPowerquad {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetPowerquad {
        Starten1SetPowerquad::from_bits(val)
    }
}
impl From<Starten1SetPowerquad> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetPowerquad) -> u8 {
        Starten1SetPowerquad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetPuf {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetPuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetPuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetPuf {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetPuf {
        Starten1SetPuf::from_bits(val)
    }
}
impl From<Starten1SetPuf> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetPuf) -> u8 {
        Starten1SetPuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetRtcLite0AlarmOrWakeup {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetRtcLite0AlarmOrWakeup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetRtcLite0AlarmOrWakeup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetRtcLite0AlarmOrWakeup {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetRtcLite0AlarmOrWakeup {
        Starten1SetRtcLite0AlarmOrWakeup::from_bits(val)
    }
}
impl From<Starten1SetRtcLite0AlarmOrWakeup> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetRtcLite0AlarmOrWakeup) -> u8 {
        Starten1SetRtcLite0AlarmOrWakeup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetSdio0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetSdio0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetSdio0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetSdio0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetSdio0 {
        Starten1SetSdio0::from_bits(val)
    }
}
impl From<Starten1SetSdio0> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetSdio0) -> u8 {
        Starten1SetSdio0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetSdio1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetSdio1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetSdio1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetSdio1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetSdio1 {
        Starten1SetSdio1::from_bits(val)
    }
}
impl From<Starten1SetSdio1> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetSdio1) -> u8 {
        Starten1SetSdio1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetSha {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetSha {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetSha {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetSha {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetSha {
        Starten1SetSha::from_bits(val)
    }
}
impl From<Starten1SetSha> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetSha) -> u8 {
        Starten1SetSha::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetShsgpioInt0 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetShsgpioInt0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetShsgpioInt0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetShsgpioInt0 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetShsgpioInt0 {
        Starten1SetShsgpioInt0::from_bits(val)
    }
}
impl From<Starten1SetShsgpioInt0> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetShsgpioInt0) -> u8 {
        Starten1SetShsgpioInt0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetShsgpioInt1 {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetShsgpioInt1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetShsgpioInt1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetShsgpioInt1 {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetShsgpioInt1 {
        Starten1SetShsgpioInt1::from_bits(val)
    }
}
impl From<Starten1SetShsgpioInt1> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetShsgpioInt1) -> u8 {
        Starten1SetShsgpioInt1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetUsbIrq {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetUsbIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetUsbIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetUsbIrq {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetUsbIrq {
        Starten1SetUsbIrq::from_bits(val)
    }
}
impl From<Starten1SetUsbIrq> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetUsbIrq) -> u8 {
        Starten1SetUsbIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Starten1SetUsbNeedclk {
    #[doc = "No effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the START_EN1 Bit"]
    SET_START_EN1 = 0x01,
}
impl Starten1SetUsbNeedclk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Starten1SetUsbNeedclk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Starten1SetUsbNeedclk {
    #[inline(always)]
    fn from(val: u8) -> Starten1SetUsbNeedclk {
        Starten1SetUsbNeedclk::from_bits(val)
    }
}
impl From<Starten1SetUsbNeedclk> for u8 {
    #[inline(always)]
    fn from(val: Starten1SetUsbNeedclk) -> u8 {
        Starten1SetUsbNeedclk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usbhsphy {
    #[doc = "no effect"]
    NO_EFFECT = 0x0,
    #[doc = "override"]
    OVERRIDE = 0x01,
}
impl Usbhsphy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usbhsphy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usbhsphy {
    #[inline(always)]
    fn from(val: u8) -> Usbhsphy {
        Usbhsphy::from_bits(val)
    }
}
impl From<Usbhsphy> for u8 {
    #[inline(always)]
    fn from(val: Usbhsphy) -> u8 {
        Usbhsphy::to_bits(val)
    }
}
