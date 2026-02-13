#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlphaCtrl {
    #[doc = "Indicates that the AS pixel alpha value will be used to blend the AS with PS. The ALPHA field is ignored."]
    EMBEDDED = 0x0,
    #[doc = "Indicates that the value in the ALPHA field should be used instead of the alpha values present in the input pixels."]
    OVERRIDE = 0x01,
    #[doc = "Indicates that the value in the ALPHA field should be used to scale all pixel alpha values. Each pixel alpha is multiplied by the value in the ALPHA field."]
    MULTIPLY = 0x02,
    #[doc = "Enable ROPs. The ROP field indicates an operation to be performed on the alpha surface and PS pixels."]
    ROPS = 0x03,
}
impl AlphaCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlphaCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlphaCtrl {
    #[inline(always)]
    fn from(val: u8) -> AlphaCtrl {
        AlphaCtrl::from_bits(val)
    }
}
impl From<AlphaCtrl> for u8 {
    #[inline(always)]
    fn from(val: AlphaCtrl) -> u8 {
        AlphaCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlphaInvert {
    #[doc = "Not inverted"]
    ALPHA_INVERT_0 = 0x0,
    #[doc = "Inverted"]
    INVERTED = 0x01,
}
impl AlphaInvert {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlphaInvert {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlphaInvert {
    #[inline(always)]
    fn from(val: u8) -> AlphaInvert {
        AlphaInvert::from_bits(val)
    }
}
impl From<AlphaInvert> for u8 {
    #[inline(always)]
    fn from(val: AlphaInvert) -> u8 {
        AlphaInvert::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AsCtrlFormat {
    #[doc = "32-bit pixels with alpha"]
    ARGB8888 = 0x0,
    #[doc = "2-bit pixel with alpha at low 8 bits"]
    RGBA888 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels without alpha (unpacked 24-bit format)"]
    RGB888 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16-bit pixels with alpha"]
    ARGB1555 = 0x08,
    #[doc = "16-bit pixels with alpha"]
    ARGB4444 = 0x09,
    #[doc = "16-bit pixel with alpha at low 1 bit"]
    RGBA5551 = 0x0a,
    #[doc = "16-bit pixel with alpha at low 4 bits"]
    RGBA4444 = 0x0b,
    #[doc = "16-bit pixels without alpha"]
    RGB555 = 0x0c,
    #[doc = "16-bit pixels without alpha"]
    RGB444 = 0x0d,
    #[doc = "16-bit pixels without alpha"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl AsCtrlFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AsCtrlFormat {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AsCtrlFormat {
    #[inline(always)]
    fn from(val: u8) -> AsCtrlFormat {
        AsCtrlFormat::from_bits(val)
    }
}
impl From<AsCtrlFormat> for u8 {
    #[inline(always)]
    fn from(val: AsCtrlFormat) -> u8 {
        AsCtrlFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlBlockSize {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8 = 0x0,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16 = 0x01,
}
impl CtrlBlockSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlBlockSize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlBlockSize {
    #[inline(always)]
    fn from(val: u8) -> CtrlBlockSize {
        CtrlBlockSize::from_bits(val)
    }
}
impl From<CtrlBlockSize> for u8 {
    #[inline(always)]
    fn from(val: CtrlBlockSize) -> u8 {
        CtrlBlockSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClkgate {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "All clocks to PXP is gated-off"]
    GATED = 0x01,
}
impl CtrlClkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClkgate {
    #[inline(always)]
    fn from(val: u8) -> CtrlClkgate {
        CtrlClkgate::from_bits(val)
    }
}
impl From<CtrlClkgate> for u8 {
    #[inline(always)]
    fn from(val: CtrlClkgate) -> u8 {
        CtrlClkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrBlockSize {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8 = 0x0,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16 = 0x01,
}
impl CtrlClrBlockSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrBlockSize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrBlockSize {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrBlockSize {
        CtrlClrBlockSize::from_bits(val)
    }
}
impl From<CtrlClrBlockSize> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrBlockSize) -> u8 {
        CtrlClrBlockSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrClkgate {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "All clocks to PXP is gated-off"]
    GATED = 0x01,
}
impl CtrlClrClkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrClkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrClkgate {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrClkgate {
        CtrlClrClkgate::from_bits(val)
    }
}
impl From<CtrlClrClkgate> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrClkgate) -> u8 {
        CtrlClrClkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrEnRepeat {
    #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
    COMPLETE = 0x0,
    #[doc = "PXP will repeat based on the current configuration register settings"]
    REPEAT = 0x01,
}
impl CtrlClrEnRepeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrEnRepeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrEnRepeat {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrEnRepeat {
        CtrlClrEnRepeat::from_bits(val)
    }
}
impl From<CtrlClrEnRepeat> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrEnRepeat) -> u8 {
        CtrlClrEnRepeat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrRotate {
    #[doc = "ROT_0"]
    ROT_0 = 0x0,
    #[doc = "ROT_90"]
    ROT_90 = 0x01,
    #[doc = "ROT_180"]
    ROT_180 = 0x02,
    #[doc = "ROT_270"]
    ROT_270 = 0x03,
}
impl CtrlClrRotate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrRotate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrRotate {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrRotate {
        CtrlClrRotate::from_bits(val)
    }
}
impl From<CtrlClrRotate> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrRotate) -> u8 {
        CtrlClrRotate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlClrSftrst {
    #[doc = "Normal PXP operation is enabled"]
    ENABLED = 0x0,
    #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
    DISABLED = 0x01,
}
impl CtrlClrSftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlClrSftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlClrSftrst {
    #[inline(always)]
    fn from(val: u8) -> CtrlClrSftrst {
        CtrlClrSftrst::from_bits(val)
    }
}
impl From<CtrlClrSftrst> for u8 {
    #[inline(always)]
    fn from(val: CtrlClrSftrst) -> u8 {
        CtrlClrSftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlEnRepeat {
    #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
    COMPLETE = 0x0,
    #[doc = "PXP will repeat based on the current configuration register settings"]
    REPEAT = 0x01,
}
impl CtrlEnRepeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlEnRepeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlEnRepeat {
    #[inline(always)]
    fn from(val: u8) -> CtrlEnRepeat {
        CtrlEnRepeat::from_bits(val)
    }
}
impl From<CtrlEnRepeat> for u8 {
    #[inline(always)]
    fn from(val: CtrlEnRepeat) -> u8 {
        CtrlEnRepeat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlRotate {
    #[doc = "ROT_0"]
    ROT_0 = 0x0,
    #[doc = "ROT_90"]
    ROT_90 = 0x01,
    #[doc = "ROT_180"]
    ROT_180 = 0x02,
    #[doc = "ROT_270"]
    ROT_270 = 0x03,
}
impl CtrlRotate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlRotate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlRotate {
    #[inline(always)]
    fn from(val: u8) -> CtrlRotate {
        CtrlRotate::from_bits(val)
    }
}
impl From<CtrlRotate> for u8 {
    #[inline(always)]
    fn from(val: CtrlRotate) -> u8 {
        CtrlRotate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetBlockSize {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8 = 0x0,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16 = 0x01,
}
impl CtrlSetBlockSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetBlockSize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetBlockSize {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetBlockSize {
        CtrlSetBlockSize::from_bits(val)
    }
}
impl From<CtrlSetBlockSize> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetBlockSize) -> u8 {
        CtrlSetBlockSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetClkgate {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "All clocks to PXP is gated-off"]
    GATED = 0x01,
}
impl CtrlSetClkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetClkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetClkgate {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetClkgate {
        CtrlSetClkgate::from_bits(val)
    }
}
impl From<CtrlSetClkgate> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetClkgate) -> u8 {
        CtrlSetClkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetEnRepeat {
    #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
    COMPLETE = 0x0,
    #[doc = "PXP will repeat based on the current configuration register settings"]
    REPEAT = 0x01,
}
impl CtrlSetEnRepeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetEnRepeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetEnRepeat {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetEnRepeat {
        CtrlSetEnRepeat::from_bits(val)
    }
}
impl From<CtrlSetEnRepeat> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetEnRepeat) -> u8 {
        CtrlSetEnRepeat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetRotate {
    #[doc = "ROT_0"]
    ROT_0 = 0x0,
    #[doc = "ROT_90"]
    ROT_90 = 0x01,
    #[doc = "ROT_180"]
    ROT_180 = 0x02,
    #[doc = "ROT_270"]
    ROT_270 = 0x03,
}
impl CtrlSetRotate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetRotate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetRotate {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetRotate {
        CtrlSetRotate::from_bits(val)
    }
}
impl From<CtrlSetRotate> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetRotate) -> u8 {
        CtrlSetRotate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSetSftrst {
    #[doc = "Normal PXP operation is enabled"]
    ENABLED = 0x0,
    #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
    DISABLED = 0x01,
}
impl CtrlSetSftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSetSftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSetSftrst {
    #[inline(always)]
    fn from(val: u8) -> CtrlSetSftrst {
        CtrlSetSftrst::from_bits(val)
    }
}
impl From<CtrlSetSftrst> for u8 {
    #[inline(always)]
    fn from(val: CtrlSetSftrst) -> u8 {
        CtrlSetSftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlSftrst {
    #[doc = "Normal PXP operation is enabled"]
    ENABLED = 0x0,
    #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
    DISABLED = 0x01,
}
impl CtrlSftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlSftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlSftrst {
    #[inline(always)]
    fn from(val: u8) -> CtrlSftrst {
        CtrlSftrst::from_bits(val)
    }
}
impl From<CtrlSftrst> for u8 {
    #[inline(always)]
    fn from(val: CtrlSftrst) -> u8 {
        CtrlSftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogBlockSize {
    #[doc = "Process 8x8 pixel blocks."]
    _8X8 = 0x0,
    #[doc = "Process 16x16 pixel blocks."]
    _16X16 = 0x01,
}
impl CtrlTogBlockSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogBlockSize {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogBlockSize {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogBlockSize {
        CtrlTogBlockSize::from_bits(val)
    }
}
impl From<CtrlTogBlockSize> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogBlockSize) -> u8 {
        CtrlTogBlockSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogClkgate {
    #[doc = "Normal operation"]
    NORMAL = 0x0,
    #[doc = "All clocks to PXP is gated-off"]
    GATED = 0x01,
}
impl CtrlTogClkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogClkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogClkgate {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogClkgate {
        CtrlTogClkgate::from_bits(val)
    }
}
impl From<CtrlTogClkgate> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogClkgate) -> u8 {
        CtrlTogClkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogEnRepeat {
    #[doc = "PXP will complete the process and enter the idle state ready to accept the next frame to be processed"]
    COMPLETE = 0x0,
    #[doc = "PXP will repeat based on the current configuration register settings"]
    REPEAT = 0x01,
}
impl CtrlTogEnRepeat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogEnRepeat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogEnRepeat {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogEnRepeat {
        CtrlTogEnRepeat::from_bits(val)
    }
}
impl From<CtrlTogEnRepeat> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogEnRepeat) -> u8 {
        CtrlTogEnRepeat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogRotate {
    #[doc = "ROT_0"]
    ROT_0 = 0x0,
    #[doc = "ROT_90"]
    ROT_90 = 0x01,
    #[doc = "ROT_180"]
    ROT_180 = 0x02,
    #[doc = "ROT_270"]
    ROT_270 = 0x03,
}
impl CtrlTogRotate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogRotate {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogRotate {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogRotate {
        CtrlTogRotate::from_bits(val)
    }
}
impl From<CtrlTogRotate> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogRotate) -> u8 {
        CtrlTogRotate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtrlTogSftrst {
    #[doc = "Normal PXP operation is enabled"]
    ENABLED = 0x0,
    #[doc = "Clocking with PXP is disabled and held in its reset (lowest power) state. This is the default value."]
    DISABLED = 0x01,
}
impl CtrlTogSftrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtrlTogSftrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtrlTogSftrst {
    #[inline(always)]
    fn from(val: u8) -> CtrlTogSftrst {
        CtrlTogSftrst::from_bits(val)
    }
}
impl From<CtrlTogSftrst> for u8 {
    #[inline(always)]
    fn from(val: CtrlTogSftrst) -> u8 {
        CtrlTogSftrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlAlphaOutput {
    #[doc = "Retain"]
    RETAIN = 0x0,
    #[doc = "Overwritten"]
    OVERWRITTEN = 0x01,
}
impl OutCtrlAlphaOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlAlphaOutput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlAlphaOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlAlphaOutput {
        OutCtrlAlphaOutput::from_bits(val)
    }
}
impl From<OutCtrlAlphaOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlAlphaOutput) -> u8 {
        OutCtrlAlphaOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlClrAlphaOutput {
    #[doc = "Retain"]
    RETAIN = 0x0,
    #[doc = "Overwritten"]
    OVERWRITTEN = 0x01,
}
impl OutCtrlClrAlphaOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlClrAlphaOutput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlClrAlphaOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlClrAlphaOutput {
        OutCtrlClrAlphaOutput::from_bits(val)
    }
}
impl From<OutCtrlClrAlphaOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlClrAlphaOutput) -> u8 {
        OutCtrlClrAlphaOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlClrFormat {
    #[doc = "32-bit pixels"]
    ARGB8888 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888 = 0x04,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16-bit pixels"]
    ARGB1555 = 0x08,
    #[doc = "16-bit pixels"]
    ARGB4444 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels"]
    RGB555 = 0x0c,
    #[doc = "16-bit pixels"]
    RGB444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OutCtrlClrFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlClrFormat {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlClrFormat {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlClrFormat {
        OutCtrlClrFormat::from_bits(val)
    }
}
impl From<OutCtrlClrFormat> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlClrFormat) -> u8 {
        OutCtrlClrFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlClrInterlacedOutput {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE = 0x0,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0 = 0x01,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1 = 0x02,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED = 0x03,
}
impl OutCtrlClrInterlacedOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlClrInterlacedOutput {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlClrInterlacedOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlClrInterlacedOutput {
        OutCtrlClrInterlacedOutput::from_bits(val)
    }
}
impl From<OutCtrlClrInterlacedOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlClrInterlacedOutput) -> u8 {
        OutCtrlClrInterlacedOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlFormat {
    #[doc = "32-bit pixels"]
    ARGB8888 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888 = 0x04,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16-bit pixels"]
    ARGB1555 = 0x08,
    #[doc = "16-bit pixels"]
    ARGB4444 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels"]
    RGB555 = 0x0c,
    #[doc = "16-bit pixels"]
    RGB444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OutCtrlFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlFormat {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlFormat {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlFormat {
        OutCtrlFormat::from_bits(val)
    }
}
impl From<OutCtrlFormat> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlFormat) -> u8 {
        OutCtrlFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlInterlacedOutput {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE = 0x0,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0 = 0x01,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1 = 0x02,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED = 0x03,
}
impl OutCtrlInterlacedOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlInterlacedOutput {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlInterlacedOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlInterlacedOutput {
        OutCtrlInterlacedOutput::from_bits(val)
    }
}
impl From<OutCtrlInterlacedOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlInterlacedOutput) -> u8 {
        OutCtrlInterlacedOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlSetAlphaOutput {
    #[doc = "Retain"]
    RETAIN = 0x0,
    #[doc = "Overwritten"]
    OVERWRITTEN = 0x01,
}
impl OutCtrlSetAlphaOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlSetAlphaOutput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlSetAlphaOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlSetAlphaOutput {
        OutCtrlSetAlphaOutput::from_bits(val)
    }
}
impl From<OutCtrlSetAlphaOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlSetAlphaOutput) -> u8 {
        OutCtrlSetAlphaOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlSetFormat {
    #[doc = "32-bit pixels"]
    ARGB8888 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888 = 0x04,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16-bit pixels"]
    ARGB1555 = 0x08,
    #[doc = "16-bit pixels"]
    ARGB4444 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels"]
    RGB555 = 0x0c,
    #[doc = "16-bit pixels"]
    RGB444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OutCtrlSetFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlSetFormat {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlSetFormat {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlSetFormat {
        OutCtrlSetFormat::from_bits(val)
    }
}
impl From<OutCtrlSetFormat> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlSetFormat) -> u8 {
        OutCtrlSetFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlSetInterlacedOutput {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE = 0x0,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0 = 0x01,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1 = 0x02,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED = 0x03,
}
impl OutCtrlSetInterlacedOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlSetInterlacedOutput {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlSetInterlacedOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlSetInterlacedOutput {
        OutCtrlSetInterlacedOutput::from_bits(val)
    }
}
impl From<OutCtrlSetInterlacedOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlSetInterlacedOutput) -> u8 {
        OutCtrlSetInterlacedOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlTogAlphaOutput {
    #[doc = "Retain"]
    RETAIN = 0x0,
    #[doc = "Overwritten"]
    OVERWRITTEN = 0x01,
}
impl OutCtrlTogAlphaOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlTogAlphaOutput {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlTogAlphaOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlTogAlphaOutput {
        OutCtrlTogAlphaOutput::from_bits(val)
    }
}
impl From<OutCtrlTogAlphaOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlTogAlphaOutput) -> u8 {
        OutCtrlTogAlphaOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlTogFormat {
    #[doc = "32-bit pixels"]
    ARGB8888 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit pixel in 32 bit DWORD.)"]
    RGB888 = 0x04,
    #[doc = "24-bit pixels (packed 24-bit format)"]
    RGB888P = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "16-bit pixels"]
    ARGB1555 = 0x08,
    #[doc = "16-bit pixels"]
    ARGB4444 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels"]
    RGB555 = 0x0c,
    #[doc = "16-bit pixels"]
    RGB444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl OutCtrlTogFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlTogFormat {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlTogFormat {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlTogFormat {
        OutCtrlTogFormat::from_bits(val)
    }
}
impl From<OutCtrlTogFormat> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlTogFormat) -> u8 {
        OutCtrlTogFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutCtrlTogInterlacedOutput {
    #[doc = "All data written in progressive format to the OUTBUF Pointer."]
    PROGRESSIVE = 0x0,
    #[doc = "Interlaced output: only data for field 0 is written to the OUTBUF Pointer."]
    FIELD0 = 0x01,
    #[doc = "Interlaced output: only data for field 1 is written to the OUTBUF2 Pointer."]
    FIELD1 = 0x02,
    #[doc = "Interlaced output: data for field 0 is written to OUTBUF and data for field 1 is written to OUTBUF2."]
    INTERLACED = 0x03,
}
impl OutCtrlTogInterlacedOutput {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OutCtrlTogInterlacedOutput {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OutCtrlTogInterlacedOutput {
    #[inline(always)]
    fn from(val: u8) -> OutCtrlTogInterlacedOutput {
        OutCtrlTogInterlacedOutput::from_bits(val)
    }
}
impl From<OutCtrlTogInterlacedOutput> for u8 {
    #[inline(always)]
    fn from(val: OutCtrlTogInterlacedOutput) -> u8 {
        OutCtrlTogInterlacedOutput::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlClrDecx {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECX2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECX4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECX8 = 0x03,
}
impl PsCtrlClrDecx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlClrDecx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlClrDecx {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlClrDecx {
        PsCtrlClrDecx::from_bits(val)
    }
}
impl From<PsCtrlClrDecx> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlClrDecx) -> u8 {
        PsCtrlClrDecx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlClrDecy {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECY2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECY4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECY8 = 0x03,
}
impl PsCtrlClrDecy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlClrDecy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlClrDecy {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlClrDecy {
        PsCtrlClrDecy::from_bits(val)
    }
}
impl From<PsCtrlClrDecy> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlClrDecy) -> u8 {
        PsCtrlClrDecy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlClrFormat {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
    RGB888_ARGB8888 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels with/without alpha at high 1bit"]
    RGB555_ARGB1555 = 0x0c,
    #[doc = "16-bit pixels with/without alpha at high 4 bits"]
    RGB444_ARGB4444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422 = 0x1e,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    #[doc = "2-bit pixels with alpha at the low 8 bits"]
    RGBA8888 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    #[doc = "16-bit pixels with alpha at the low 1bits"]
    RGBA5551 = 0x2c,
    #[doc = "16-bit pixels with alpha at the low 4 bits"]
    RGBA4444 = 0x2d,
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
    _RESERVED_3f = 0x3f,
}
impl PsCtrlClrFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlClrFormat {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlClrFormat {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlClrFormat {
        PsCtrlClrFormat::from_bits(val)
    }
}
impl From<PsCtrlClrFormat> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlClrFormat) -> u8 {
        PsCtrlClrFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlDecx {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECX2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECX4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECX8 = 0x03,
}
impl PsCtrlDecx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlDecx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlDecx {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlDecx {
        PsCtrlDecx::from_bits(val)
    }
}
impl From<PsCtrlDecx> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlDecx) -> u8 {
        PsCtrlDecx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlDecy {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECY2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECY4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECY8 = 0x03,
}
impl PsCtrlDecy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlDecy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlDecy {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlDecy {
        PsCtrlDecy::from_bits(val)
    }
}
impl From<PsCtrlDecy> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlDecy) -> u8 {
        PsCtrlDecy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlFormat {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
    RGB888_ARGB8888 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels with/without alpha at high 1bit"]
    RGB555_ARGB1555 = 0x0c,
    #[doc = "16-bit pixels with/without alpha at high 4 bits"]
    RGB444_ARGB4444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422 = 0x1e,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    #[doc = "2-bit pixels with alpha at the low 8 bits"]
    RGBA8888 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    #[doc = "16-bit pixels with alpha at the low 1bits"]
    RGBA5551 = 0x2c,
    #[doc = "16-bit pixels with alpha at the low 4 bits"]
    RGBA4444 = 0x2d,
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
    _RESERVED_3f = 0x3f,
}
impl PsCtrlFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlFormat {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlFormat {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlFormat {
        PsCtrlFormat::from_bits(val)
    }
}
impl From<PsCtrlFormat> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlFormat) -> u8 {
        PsCtrlFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlSetDecx {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECX2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECX4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECX8 = 0x03,
}
impl PsCtrlSetDecx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlSetDecx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlSetDecx {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlSetDecx {
        PsCtrlSetDecx::from_bits(val)
    }
}
impl From<PsCtrlSetDecx> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlSetDecx) -> u8 {
        PsCtrlSetDecx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlSetDecy {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECY2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECY4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECY8 = 0x03,
}
impl PsCtrlSetDecy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlSetDecy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlSetDecy {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlSetDecy {
        PsCtrlSetDecy::from_bits(val)
    }
}
impl From<PsCtrlSetDecy> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlSetDecy) -> u8 {
        PsCtrlSetDecy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlSetFormat {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
    RGB888_ARGB8888 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels with/without alpha at high 1bit"]
    RGB555_ARGB1555 = 0x0c,
    #[doc = "16-bit pixels with/without alpha at high 4 bits"]
    RGB444_ARGB4444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422 = 0x1e,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    #[doc = "2-bit pixels with alpha at the low 8 bits"]
    RGBA8888 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    #[doc = "16-bit pixels with alpha at the low 1bits"]
    RGBA5551 = 0x2c,
    #[doc = "16-bit pixels with alpha at the low 4 bits"]
    RGBA4444 = 0x2d,
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
    _RESERVED_3f = 0x3f,
}
impl PsCtrlSetFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlSetFormat {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlSetFormat {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlSetFormat {
        PsCtrlSetFormat::from_bits(val)
    }
}
impl From<PsCtrlSetFormat> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlSetFormat) -> u8 {
        PsCtrlSetFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlTogDecx {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECX2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECX4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECX8 = 0x03,
}
impl PsCtrlTogDecx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlTogDecx {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlTogDecx {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlTogDecx {
        PsCtrlTogDecx::from_bits(val)
    }
}
impl From<PsCtrlTogDecx> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlTogDecx) -> u8 {
        PsCtrlTogDecx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlTogDecy {
    #[doc = "Disable pre-decimation filter."]
    DISABLE = 0x0,
    #[doc = "Decimate PS by 2."]
    DECY2 = 0x01,
    #[doc = "Decimate PS by 4."]
    DECY4 = 0x02,
    #[doc = "Decimate PS by 8."]
    DECY8 = 0x03,
}
impl PsCtrlTogDecy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlTogDecy {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlTogDecy {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlTogDecy {
        PsCtrlTogDecy::from_bits(val)
    }
}
impl From<PsCtrlTogDecy> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlTogDecy) -> u8 {
        PsCtrlTogDecy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PsCtrlTogFormat {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "32-bit pixels (unpacked 24-bit format with/without alpha at high 8bits)"]
    RGB888_ARGB8888 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    #[doc = "16-bit pixels with/without alpha at high 1bit"]
    RGB555_ARGB1555 = 0x0c,
    #[doc = "16-bit pixels with/without alpha at high 4 bits"]
    RGB444_ARGB4444 = 0x0d,
    #[doc = "16-bit pixels"]
    RGB565 = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "32-bit pixels (1-plane XYUV unpacked)"]
    YUV1P444 = 0x10,
    _RESERVED_11 = 0x11,
    #[doc = "16-bit pixels (1-plane U0,Y0,V0,Y1 interleaved bytes)"]
    UYVY1P422 = 0x12,
    #[doc = "16-bit pixels (1-plane V0,Y0,U0,Y1 interleaved bytes)"]
    VYUY1P422 = 0x13,
    #[doc = "8-bit monochrome pixels (1-plane Y luma output)"]
    Y8 = 0x14,
    #[doc = "4-bit monochrome pixels (1-plane Y luma, 4 bit truncation)"]
    Y4 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    #[doc = "16-bit pixels (2-plane UV interleaved bytes)"]
    YUV2P422 = 0x18,
    #[doc = "16-bit pixels (2-plane UV)"]
    YUV2P420 = 0x19,
    #[doc = "16-bit pixels (2-plane VU interleaved bytes)"]
    YVU2P422 = 0x1a,
    #[doc = "16-bit pixels (2-plane VU)"]
    YVU2P420 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV422 = 0x1e,
    #[doc = "16-bit pixels (3-plane format)"]
    YUV420 = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    #[doc = "2-bit pixels with alpha at the low 8 bits"]
    RGBA8888 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    #[doc = "16-bit pixels with alpha at the low 1bits"]
    RGBA5551 = 0x2c,
    #[doc = "16-bit pixels with alpha at the low 4 bits"]
    RGBA4444 = 0x2d,
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
    _RESERVED_3f = 0x3f,
}
impl PsCtrlTogFormat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PsCtrlTogFormat {
        unsafe { core::mem::transmute(val & 0x3f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PsCtrlTogFormat {
    #[inline(always)]
    fn from(val: u8) -> PsCtrlTogFormat {
        PsCtrlTogFormat::from_bits(val)
    }
}
impl From<PsCtrlTogFormat> for u8 {
    #[inline(always)]
    fn from(val: PsCtrlTogFormat) -> u8 {
        PsCtrlTogFormat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rop {
    #[doc = "AS AND PS"]
    MASKAS = 0x0,
    #[doc = "nAS AND PS"]
    MASKNOTAS = 0x01,
    #[doc = "AS AND nPS"]
    MASKASNOT = 0x02,
    #[doc = "AS OR PS"]
    MERGEAS = 0x03,
    #[doc = "nAS OR PS"]
    MERGENOTAS = 0x04,
    #[doc = "AS OR nPS"]
    MERGEASNOT = 0x05,
    #[doc = "nAS"]
    NOTCOPYAS = 0x06,
    #[doc = "nPS"]
    NOT = 0x07,
    #[doc = "AS NAND PS"]
    NOTMASKAS = 0x08,
    #[doc = "AS NOR PS"]
    NOTMERGEAS = 0x09,
    #[doc = "AS XOR PS"]
    XORAS = 0x0a,
    #[doc = "AS XNOR PS"]
    NOTXORAS = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Rop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rop {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rop {
    #[inline(always)]
    fn from(val: u8) -> Rop {
        Rop::from_bits(val)
    }
}
impl From<Rop> for u8 {
    #[inline(always)]
    fn from(val: Rop) -> u8 {
        Rop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RotMemLpState {
    #[doc = "Memory is not in low power state."]
    NONE = 0x0,
    #[doc = "Light Sleep Mode. Low leakage mode, maintain memory contents."]
    LS = 0x01,
    #[doc = "Deep Sleep Mode. Low leakage mode, maintain memory contents."]
    DS = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Shut Down Mode. Shut Down periphery and core, no memory retention."]
    SD = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl RotMemLpState {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RotMemLpState {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RotMemLpState {
    #[inline(always)]
    fn from(val: u8) -> RotMemLpState {
        RotMemLpState::from_bits(val)
    }
}
impl From<RotMemLpState> for u8 {
    #[inline(always)]
    fn from(val: RotMemLpState) -> u8 {
        RotMemLpState::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0AlphaMode {
    #[doc = "Straight mode"]
    S0_ALPHA_MODE_0 = 0x0,
    #[doc = "Inverted mode"]
    S0_ALPHA_MODE_1 = 0x01,
}
impl S0AlphaMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0AlphaMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0AlphaMode {
    #[inline(always)]
    fn from(val: u8) -> S0AlphaMode {
        S0AlphaMode::from_bits(val)
    }
}
impl From<S0AlphaMode> for u8 {
    #[inline(always)]
    fn from(val: S0AlphaMode) -> u8 {
        S0AlphaMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0ColorMode {
    #[doc = "Original pixel"]
    S0_COLOR_MODE_0 = 0x0,
    #[doc = "Scaled pixel"]
    S0_COLOR_MODE_1 = 0x01,
}
impl S0ColorMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0ColorMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0ColorMode {
    #[inline(always)]
    fn from(val: u8) -> S0ColorMode {
        S0ColorMode::from_bits(val)
    }
}
impl From<S0ColorMode> for u8 {
    #[inline(always)]
    fn from(val: S0ColorMode) -> u8 {
        S0ColorMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0GlobalAlphaMode {
    #[doc = "Global alpha"]
    S0_GLOBAL_ALPHA_MODE_0 = 0x0,
    #[doc = "Local alpha"]
    S0_GLOBAL_ALPHA_MODE_1 = 0x01,
    #[doc = "Scaled alpha"]
    S0_GLOBAL_ALPHA_MODE_2 = 0x02,
    #[doc = "Scaled alpha"]
    S0_GLOBAL_ALPHA_MODE_3 = 0x03,
}
impl S0GlobalAlphaMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0GlobalAlphaMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0GlobalAlphaMode {
    #[inline(always)]
    fn from(val: u8) -> S0GlobalAlphaMode {
        S0GlobalAlphaMode::from_bits(val)
    }
}
impl From<S0GlobalAlphaMode> for u8 {
    #[inline(always)]
    fn from(val: S0GlobalAlphaMode) -> u8 {
        S0GlobalAlphaMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S0S1FactorMode {
    #[doc = "1"]
    S0_S1_FACTOR_MODE_0 = 0x0,
    #[doc = "0"]
    S0_S1_FACTOR_MODE_1 = 0x01,
    #[doc = "Straight alpha"]
    S0_S1_FACTOR_MODE_2 = 0x02,
    #[doc = "Inverse alpha"]
    S0_S1_FACTOR_MODE_3 = 0x03,
}
impl S0S1FactorMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S0S1FactorMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S0S1FactorMode {
    #[inline(always)]
    fn from(val: u8) -> S0S1FactorMode {
        S0S1FactorMode::from_bits(val)
    }
}
impl From<S0S1FactorMode> for u8 {
    #[inline(always)]
    fn from(val: S0S1FactorMode) -> u8 {
        S0S1FactorMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1AlphaMode {
    #[doc = "Straight mode"]
    S1_ALPHA_MODE_0 = 0x0,
    #[doc = "Inverted mode"]
    S1_ALPHA_MODE_1 = 0x01,
}
impl S1AlphaMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1AlphaMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1AlphaMode {
    #[inline(always)]
    fn from(val: u8) -> S1AlphaMode {
        S1AlphaMode::from_bits(val)
    }
}
impl From<S1AlphaMode> for u8 {
    #[inline(always)]
    fn from(val: S1AlphaMode) -> u8 {
        S1AlphaMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1ColorMode {
    #[doc = "Original pixel"]
    S1_COLOR_MODE_0 = 0x0,
    #[doc = "Scaled pixel"]
    S1_COLOR_MODE_1 = 0x01,
}
impl S1ColorMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1ColorMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1ColorMode {
    #[inline(always)]
    fn from(val: u8) -> S1ColorMode {
        S1ColorMode::from_bits(val)
    }
}
impl From<S1ColorMode> for u8 {
    #[inline(always)]
    fn from(val: S1ColorMode) -> u8 {
        S1ColorMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1GlobalAlphaMode {
    #[doc = "Global alpha"]
    S1_GLOBAL_ALPHA_MODE_0 = 0x0,
    #[doc = "Local alpha"]
    S1_GLOBAL_ALPHA_MODE_1 = 0x01,
    #[doc = "Scaled alpha"]
    S1_GLOBAL_ALPHA_MODE_2 = 0x02,
    #[doc = "Scaled alpha"]
    S1_GLOBAL_ALPHA_MODE_3 = 0x03,
}
impl S1GlobalAlphaMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1GlobalAlphaMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1GlobalAlphaMode {
    #[inline(always)]
    fn from(val: u8) -> S1GlobalAlphaMode {
        S1GlobalAlphaMode::from_bits(val)
    }
}
impl From<S1GlobalAlphaMode> for u8 {
    #[inline(always)]
    fn from(val: S1GlobalAlphaMode) -> u8 {
        S1GlobalAlphaMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum S1S0FactorMode {
    #[doc = "1"]
    S1_S0_FACTOR_MODE_0 = 0x0,
    #[doc = "0"]
    S1_S0_FACTOR_MODE_1 = 0x01,
    #[doc = "Straight alpha"]
    S1_S0_FACTOR_MODE_2 = 0x02,
    #[doc = "Inverse alpha"]
    S1_S0_FACTOR_MODE_3 = 0x03,
}
impl S1S0FactorMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> S1S0FactorMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for S1S0FactorMode {
    #[inline(always)]
    fn from(val: u8) -> S1S0FactorMode {
        S1S0FactorMode::from_bits(val)
    }
}
impl From<S1S0FactorMode> for u8 {
    #[inline(always)]
    fn from(val: S1S0FactorMode) -> u8 {
        S1S0FactorMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatAxiReadError {
    #[doc = "AXI read is normal"]
    NORMAL = 0x0,
    #[doc = "AXI read error has occurred"]
    ERROR = 0x01,
}
impl StatAxiReadError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatAxiReadError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatAxiReadError {
    #[inline(always)]
    fn from(val: u8) -> StatAxiReadError {
        StatAxiReadError::from_bits(val)
    }
}
impl From<StatAxiReadError> for u8 {
    #[inline(always)]
    fn from(val: StatAxiReadError) -> u8 {
        StatAxiReadError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatAxiWriteError {
    #[doc = "AXI write is normal"]
    NORMAL = 0x0,
    #[doc = "AXI write error has occurred"]
    ERROR = 0x01,
}
impl StatAxiWriteError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatAxiWriteError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatAxiWriteError {
    #[inline(always)]
    fn from(val: u8) -> StatAxiWriteError {
        StatAxiWriteError::from_bits(val)
    }
}
impl From<StatAxiWriteError> for u8 {
    #[inline(always)]
    fn from(val: StatAxiWriteError) -> u8 {
        StatAxiWriteError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatClrAxiReadError {
    #[doc = "AXI read is normal"]
    NORMAL = 0x0,
    #[doc = "AXI read error has occurred"]
    ERROR = 0x01,
}
impl StatClrAxiReadError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrAxiReadError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrAxiReadError {
    #[inline(always)]
    fn from(val: u8) -> StatClrAxiReadError {
        StatClrAxiReadError::from_bits(val)
    }
}
impl From<StatClrAxiReadError> for u8 {
    #[inline(always)]
    fn from(val: StatClrAxiReadError) -> u8 {
        StatClrAxiReadError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatClrAxiWriteError {
    #[doc = "AXI write is normal"]
    NORMAL = 0x0,
    #[doc = "AXI write error has occurred"]
    ERROR = 0x01,
}
impl StatClrAxiWriteError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrAxiWriteError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrAxiWriteError {
    #[inline(always)]
    fn from(val: u8) -> StatClrAxiWriteError {
        StatClrAxiWriteError::from_bits(val)
    }
}
impl From<StatClrAxiWriteError> for u8 {
    #[inline(always)]
    fn from(val: StatClrAxiWriteError) -> u8 {
        StatClrAxiWriteError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatClrIrq {
    #[doc = "No interrupt"]
    IRQ_0 = 0x0,
    #[doc = "Interrupt generated"]
    IRQ_1 = 0x01,
}
impl StatClrIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrIrq {
    #[inline(always)]
    fn from(val: u8) -> StatClrIrq {
        StatClrIrq::from_bits(val)
    }
}
impl From<StatClrIrq> for u8 {
    #[inline(always)]
    fn from(val: StatClrIrq) -> u8 {
        StatClrIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatClrLutDmaLoadDoneIrq {
    #[doc = "LUT DMA LOAD transfer is active"]
    ACTIVE = 0x0,
    #[doc = "LUT DMA LOAD transfer is complete"]
    COMPLETE = 0x01,
}
impl StatClrLutDmaLoadDoneIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatClrLutDmaLoadDoneIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatClrLutDmaLoadDoneIrq {
    #[inline(always)]
    fn from(val: u8) -> StatClrLutDmaLoadDoneIrq {
        StatClrLutDmaLoadDoneIrq::from_bits(val)
    }
}
impl From<StatClrLutDmaLoadDoneIrq> for u8 {
    #[inline(always)]
    fn from(val: StatClrLutDmaLoadDoneIrq) -> u8 {
        StatClrLutDmaLoadDoneIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatIrq {
    #[doc = "No interrupt"]
    IRQ_0 = 0x0,
    #[doc = "Interrupt generated"]
    IRQ_1 = 0x01,
}
impl StatIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatIrq {
    #[inline(always)]
    fn from(val: u8) -> StatIrq {
        StatIrq::from_bits(val)
    }
}
impl From<StatIrq> for u8 {
    #[inline(always)]
    fn from(val: StatIrq) -> u8 {
        StatIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatLutDmaLoadDoneIrq {
    #[doc = "LUT DMA LOAD transfer is active"]
    ACTIVE = 0x0,
    #[doc = "LUT DMA LOAD transfer is complete"]
    COMPLETE = 0x01,
}
impl StatLutDmaLoadDoneIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatLutDmaLoadDoneIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatLutDmaLoadDoneIrq {
    #[inline(always)]
    fn from(val: u8) -> StatLutDmaLoadDoneIrq {
        StatLutDmaLoadDoneIrq::from_bits(val)
    }
}
impl From<StatLutDmaLoadDoneIrq> for u8 {
    #[inline(always)]
    fn from(val: StatLutDmaLoadDoneIrq) -> u8 {
        StatLutDmaLoadDoneIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatSetAxiReadError {
    #[doc = "AXI read is normal"]
    NORMAL = 0x0,
    #[doc = "AXI read error has occurred"]
    ERROR = 0x01,
}
impl StatSetAxiReadError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetAxiReadError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetAxiReadError {
    #[inline(always)]
    fn from(val: u8) -> StatSetAxiReadError {
        StatSetAxiReadError::from_bits(val)
    }
}
impl From<StatSetAxiReadError> for u8 {
    #[inline(always)]
    fn from(val: StatSetAxiReadError) -> u8 {
        StatSetAxiReadError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatSetAxiWriteError {
    #[doc = "AXI write is normal"]
    NORMAL = 0x0,
    #[doc = "AXI write error has occurred"]
    ERROR = 0x01,
}
impl StatSetAxiWriteError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetAxiWriteError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetAxiWriteError {
    #[inline(always)]
    fn from(val: u8) -> StatSetAxiWriteError {
        StatSetAxiWriteError::from_bits(val)
    }
}
impl From<StatSetAxiWriteError> for u8 {
    #[inline(always)]
    fn from(val: StatSetAxiWriteError) -> u8 {
        StatSetAxiWriteError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatSetIrq {
    #[doc = "No interrupt"]
    IRQ_0 = 0x0,
    #[doc = "Interrupt generated"]
    IRQ_1 = 0x01,
}
impl StatSetIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetIrq {
    #[inline(always)]
    fn from(val: u8) -> StatSetIrq {
        StatSetIrq::from_bits(val)
    }
}
impl From<StatSetIrq> for u8 {
    #[inline(always)]
    fn from(val: StatSetIrq) -> u8 {
        StatSetIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatSetLutDmaLoadDoneIrq {
    #[doc = "LUT DMA LOAD transfer is active"]
    ACTIVE = 0x0,
    #[doc = "LUT DMA LOAD transfer is complete"]
    COMPLETE = 0x01,
}
impl StatSetLutDmaLoadDoneIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatSetLutDmaLoadDoneIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatSetLutDmaLoadDoneIrq {
    #[inline(always)]
    fn from(val: u8) -> StatSetLutDmaLoadDoneIrq {
        StatSetLutDmaLoadDoneIrq::from_bits(val)
    }
}
impl From<StatSetLutDmaLoadDoneIrq> for u8 {
    #[inline(always)]
    fn from(val: StatSetLutDmaLoadDoneIrq) -> u8 {
        StatSetLutDmaLoadDoneIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatTogAxiReadError {
    #[doc = "AXI read is normal"]
    NORMAL = 0x0,
    #[doc = "AXI read error has occurred"]
    ERROR = 0x01,
}
impl StatTogAxiReadError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogAxiReadError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogAxiReadError {
    #[inline(always)]
    fn from(val: u8) -> StatTogAxiReadError {
        StatTogAxiReadError::from_bits(val)
    }
}
impl From<StatTogAxiReadError> for u8 {
    #[inline(always)]
    fn from(val: StatTogAxiReadError) -> u8 {
        StatTogAxiReadError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatTogAxiWriteError {
    #[doc = "AXI write is normal"]
    NORMAL = 0x0,
    #[doc = "AXI write error has occurred"]
    ERROR = 0x01,
}
impl StatTogAxiWriteError {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogAxiWriteError {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogAxiWriteError {
    #[inline(always)]
    fn from(val: u8) -> StatTogAxiWriteError {
        StatTogAxiWriteError::from_bits(val)
    }
}
impl From<StatTogAxiWriteError> for u8 {
    #[inline(always)]
    fn from(val: StatTogAxiWriteError) -> u8 {
        StatTogAxiWriteError::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatTogIrq {
    #[doc = "No interrupt"]
    IRQ_0 = 0x0,
    #[doc = "Interrupt generated"]
    IRQ_1 = 0x01,
}
impl StatTogIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogIrq {
    #[inline(always)]
    fn from(val: u8) -> StatTogIrq {
        StatTogIrq::from_bits(val)
    }
}
impl From<StatTogIrq> for u8 {
    #[inline(always)]
    fn from(val: StatTogIrq) -> u8 {
        StatTogIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StatTogLutDmaLoadDoneIrq {
    #[doc = "LUT DMA LOAD transfer is active"]
    ACTIVE = 0x0,
    #[doc = "LUT DMA LOAD transfer is complete"]
    COMPLETE = 0x01,
}
impl StatTogLutDmaLoadDoneIrq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatTogLutDmaLoadDoneIrq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatTogLutDmaLoadDoneIrq {
    #[inline(always)]
    fn from(val: u8) -> StatTogLutDmaLoadDoneIrq {
        StatTogLutDmaLoadDoneIrq::from_bits(val)
    }
}
impl From<StatTogLutDmaLoadDoneIrq> for u8 {
    #[inline(always)]
    fn from(val: StatTogLutDmaLoadDoneIrq) -> u8 {
        StatTogLutDmaLoadDoneIrq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum YcbcrMode {
    #[doc = "YUV to RGB"]
    YCBCR_MODE_0 = 0x0,
    #[doc = "YCbCr to RGB"]
    YCBCR_MODE_1 = 0x01,
}
impl YcbcrMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> YcbcrMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for YcbcrMode {
    #[inline(always)]
    fn from(val: u8) -> YcbcrMode {
        YcbcrMode::from_bits(val)
    }
}
impl From<YcbcrMode> for u8 {
    #[inline(always)]
    fn from(val: YcbcrMode) -> u8 {
        YcbcrMode::to_bits(val)
    }
}
