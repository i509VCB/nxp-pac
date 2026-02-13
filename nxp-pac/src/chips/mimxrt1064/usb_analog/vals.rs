#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SiliconRevision(u32);
impl SiliconRevision {
    #[doc = "Silicon revision 1.0"]
    pub const SILICON_REVISION_7077888: Self = Self(0x006c_0000);
}
impl SiliconRevision {
    pub const fn from_bits(val: u32) -> SiliconRevision {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SiliconRevision {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x006c_0000 => f.write_str("SILICON_REVISION_7077888"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SiliconRevision {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x006c_0000 => defmt::write!(f, "SILICON_REVISION_7077888"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SiliconRevision {
    #[inline(always)]
    fn from(val: u32) -> SiliconRevision {
        SiliconRevision::from_bits(val)
    }
}
impl From<SiliconRevision> for u32 {
    #[inline(always)]
    fn from(val: SiliconRevision) -> u32 {
        SiliconRevision::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb1ChrgDetectChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectChkChrgB {
        Usb1ChrgDetectChkChrgB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectChkChrgB) -> u8 {
        Usb1ChrgDetectChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectClrChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb1ChrgDetectClrChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectClrChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectClrChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectClrChkChrgB {
        Usb1ChrgDetectClrChkChrgB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectClrChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectClrChkChrgB) -> u8 {
        Usb1ChrgDetectClrChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectClrEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb1ChrgDetectClrEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectClrEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectClrEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectClrEnB {
        Usb1ChrgDetectClrEnB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectClrEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectClrEnB) -> u8 {
        Usb1ChrgDetectClrEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb1ChrgDetectEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectEnB {
        Usb1ChrgDetectEnB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectEnB) -> u8 {
        Usb1ChrgDetectEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectSetChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb1ChrgDetectSetChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectSetChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectSetChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectSetChkChrgB {
        Usb1ChrgDetectSetChkChrgB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectSetChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectSetChkChrgB) -> u8 {
        Usb1ChrgDetectSetChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectSetEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb1ChrgDetectSetEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectSetEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectSetEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectSetEnB {
        Usb1ChrgDetectSetEnB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectSetEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectSetEnB) -> u8 {
        Usb1ChrgDetectSetEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectStatChrgDetected {
    #[doc = "The USB port is not connected to a charger."]
    CHARGER_NOT_PRESENT = 0x0,
    #[doc = "A charger (either a dedicated charger or a host charger) is connected to the USB port."]
    CHARGER_PRESENT = 0x01,
}
impl Usb1ChrgDetectStatChrgDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectStatChrgDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectStatChrgDetected {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectStatChrgDetected {
        Usb1ChrgDetectStatChrgDetected::from_bits(val)
    }
}
impl From<Usb1ChrgDetectStatChrgDetected> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectStatChrgDetected) -> u8 {
        Usb1ChrgDetectStatChrgDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectStatPlugContact {
    #[doc = "The USB plug has not made contact."]
    NO_CONTACT = 0x0,
    #[doc = "The USB plug has made good contact."]
    GOOD_CONTACT = 0x01,
}
impl Usb1ChrgDetectStatPlugContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectStatPlugContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectStatPlugContact {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectStatPlugContact {
        Usb1ChrgDetectStatPlugContact::from_bits(val)
    }
}
impl From<Usb1ChrgDetectStatPlugContact> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectStatPlugContact) -> u8 {
        Usb1ChrgDetectStatPlugContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectTogChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb1ChrgDetectTogChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectTogChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectTogChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectTogChkChrgB {
        Usb1ChrgDetectTogChkChrgB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectTogChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectTogChkChrgB) -> u8 {
        Usb1ChrgDetectTogChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1ChrgDetectTogEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb1ChrgDetectTogEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1ChrgDetectTogEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1ChrgDetectTogEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb1ChrgDetectTogEnB {
        Usb1ChrgDetectTogEnB::from_bits(val)
    }
}
impl From<Usb1ChrgDetectTogEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb1ChrgDetectTogEnB) -> u8 {
        Usb1ChrgDetectTogEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectClrVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectClrVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectClrVbusvalidThresh {
        Usb1VbusDetectClrVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectClrVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectClrVbusvalidThresh) -> u8 {
        Usb1VbusDetectClrVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectSetVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectSetVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectSetVbusvalidThresh {
        Usb1VbusDetectSetVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectSetVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectSetVbusvalidThresh) -> u8 {
        Usb1VbusDetectSetVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectTogVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectTogVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectTogVbusvalidThresh {
        Usb1VbusDetectTogVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectTogVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectTogVbusvalidThresh) -> u8 {
        Usb1VbusDetectTogVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb1VbusDetectVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb1VbusDetectVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb1VbusDetectVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb1VbusDetectVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb1VbusDetectVbusvalidThresh {
        Usb1VbusDetectVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb1VbusDetectVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb1VbusDetectVbusvalidThresh) -> u8 {
        Usb1VbusDetectVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb2ChrgDetectChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectChkChrgB {
        Usb2ChrgDetectChkChrgB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectChkChrgB) -> u8 {
        Usb2ChrgDetectChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectClrChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb2ChrgDetectClrChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectClrChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectClrChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectClrChkChrgB {
        Usb2ChrgDetectClrChkChrgB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectClrChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectClrChkChrgB) -> u8 {
        Usb2ChrgDetectClrChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectClrEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb2ChrgDetectClrEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectClrEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectClrEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectClrEnB {
        Usb2ChrgDetectClrEnB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectClrEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectClrEnB) -> u8 {
        Usb2ChrgDetectClrEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb2ChrgDetectEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectEnB {
        Usb2ChrgDetectEnB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectEnB) -> u8 {
        Usb2ChrgDetectEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectSetChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb2ChrgDetectSetChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectSetChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectSetChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectSetChkChrgB {
        Usb2ChrgDetectSetChkChrgB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectSetChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectSetChkChrgB) -> u8 {
        Usb2ChrgDetectSetChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectSetEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb2ChrgDetectSetEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectSetEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectSetEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectSetEnB {
        Usb2ChrgDetectSetEnB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectSetEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectSetEnB) -> u8 {
        Usb2ChrgDetectSetEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectStatChrgDetected {
    #[doc = "The USB port is not connected to a charger."]
    CHARGER_NOT_PRESENT = 0x0,
    #[doc = "A charger (either a dedicated charger or a host charger) is connected to the USB port."]
    CHARGER_PRESENT = 0x01,
}
impl Usb2ChrgDetectStatChrgDetected {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectStatChrgDetected {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectStatChrgDetected {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectStatChrgDetected {
        Usb2ChrgDetectStatChrgDetected::from_bits(val)
    }
}
impl From<Usb2ChrgDetectStatChrgDetected> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectStatChrgDetected) -> u8 {
        Usb2ChrgDetectStatChrgDetected::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectStatPlugContact {
    #[doc = "The USB plug has not made contact."]
    NO_CONTACT = 0x0,
    #[doc = "The USB plug has made good contact."]
    GOOD_CONTACT = 0x01,
}
impl Usb2ChrgDetectStatPlugContact {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectStatPlugContact {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectStatPlugContact {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectStatPlugContact {
        Usb2ChrgDetectStatPlugContact::from_bits(val)
    }
}
impl From<Usb2ChrgDetectStatPlugContact> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectStatPlugContact) -> u8 {
        Usb2ChrgDetectStatPlugContact::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectTogChkChrgB {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0x0,
    #[doc = "Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 0x01,
}
impl Usb2ChrgDetectTogChkChrgB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectTogChkChrgB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectTogChkChrgB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectTogChkChrgB {
        Usb2ChrgDetectTogChkChrgB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectTogChkChrgB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectTogChkChrgB) -> u8 {
        Usb2ChrgDetectTogChkChrgB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2ChrgDetectTogEnB {
    #[doc = "Enable the charger detector."]
    ENABLE = 0x0,
    #[doc = "Disable the charger detector."]
    DISABLE = 0x01,
}
impl Usb2ChrgDetectTogEnB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2ChrgDetectTogEnB {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2ChrgDetectTogEnB {
    #[inline(always)]
    fn from(val: u8) -> Usb2ChrgDetectTogEnB {
        Usb2ChrgDetectTogEnB::from_bits(val)
    }
}
impl From<Usb2ChrgDetectTogEnB> for u8 {
    #[inline(always)]
    fn from(val: Usb2ChrgDetectTogEnB) -> u8 {
        Usb2ChrgDetectTogEnB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2VbusDetectClrVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2VbusDetectClrVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2VbusDetectClrVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2VbusDetectClrVbusvalidThresh {
        Usb2VbusDetectClrVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2VbusDetectClrVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2VbusDetectClrVbusvalidThresh) -> u8 {
        Usb2VbusDetectClrVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2VbusDetectSetVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2VbusDetectSetVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2VbusDetectSetVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2VbusDetectSetVbusvalidThresh {
        Usb2VbusDetectSetVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2VbusDetectSetVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2VbusDetectSetVbusvalidThresh) -> u8 {
        Usb2VbusDetectSetVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2VbusDetectTogVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2VbusDetectTogVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2VbusDetectTogVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2VbusDetectTogVbusvalidThresh {
        Usb2VbusDetectTogVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2VbusDetectTogVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2VbusDetectTogVbusvalidThresh) -> u8 {
        Usb2VbusDetectTogVbusvalidThresh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb2VbusDetectVbusvalidThresh {
    #[doc = "4.0V"]
    _4V0 = 0x0,
    #[doc = "4.1V"]
    _4V1 = 0x01,
    #[doc = "4.2V"]
    _4V2 = 0x02,
    #[doc = "4.3V"]
    _4V3 = 0x03,
    #[doc = "4.4V (default)"]
    _4V4 = 0x04,
    #[doc = "4.5V"]
    _4V5 = 0x05,
    #[doc = "4.6V"]
    _4V6 = 0x06,
    #[doc = "4.7V"]
    _4V7 = 0x07,
}
impl Usb2VbusDetectVbusvalidThresh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb2VbusDetectVbusvalidThresh {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb2VbusDetectVbusvalidThresh {
    #[inline(always)]
    fn from(val: u8) -> Usb2VbusDetectVbusvalidThresh {
        Usb2VbusDetectVbusvalidThresh::from_bits(val)
    }
}
impl From<Usb2VbusDetectVbusvalidThresh> for u8 {
    #[inline(always)]
    fn from(val: Usb2VbusDetectVbusvalidThresh) -> u8 {
        Usb2VbusDetectVbusvalidThresh::to_bits(val)
    }
}
