#[doc = "Enable DMA0 triggers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_ITRIG_ENA(pub u32);
impl DMA0_ITRIG_ENA {
    #[doc = "Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIG_ENA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub const fn set_ITRIG_ENA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for DMA0_ITRIG_ENA {
    #[inline(always)]
    fn default() -> DMA0_ITRIG_ENA {
        DMA0_ITRIG_ENA(0)
    }
}
impl core::fmt::Debug for DMA0_ITRIG_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_ENA")
            .field("ITRIG_ENA", &self.ITRIG_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_ITRIG_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA0_ITRIG_ENA {{ ITRIG_ENA: {=u32:?} }}",
            self.ITRIG_ENA()
        )
    }
}
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_ITRIG_ENA_CLR(pub u32);
impl DMA0_ITRIG_ENA_CLR {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_ITRIG_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for DMA0_ITRIG_ENA_CLR {
    #[inline(always)]
    fn default() -> DMA0_ITRIG_ENA_CLR {
        DMA0_ITRIG_ENA_CLR(0)
    }
}
impl core::fmt::Debug for DMA0_ITRIG_ENA_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_ENA_CLR")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_ITRIG_ENA_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_ITRIG_ENA_CLR {{ CLR: {=u32:?} }}", self.CLR())
    }
}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_ITRIG_ENA_SET(pub u32);
impl DMA0_ITRIG_ENA_SET {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn SET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn set_SET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for DMA0_ITRIG_ENA_SET {
    #[inline(always)]
    fn default() -> DMA0_ITRIG_ENA_SET {
        DMA0_ITRIG_ENA_SET(0)
    }
}
impl core::fmt::Debug for DMA0_ITRIG_ENA_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_ENA_SET")
            .field("SET", &self.SET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_ITRIG_ENA_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_ITRIG_ENA_SET {{ SET: {=u32:?} }}", self.SET())
    }
}
#[doc = "Trigger select register for DMA0 channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_ITRIG_INMUX(pub u32);
impl DMA0_ITRIG_INMUX {
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[must_use]
    #[inline(always)]
    pub const fn INP(&self) -> super::vals::DMA0_ITRIG_INMUX_INP {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::DMA0_ITRIG_INMUX_INP::from_bits(val as u8)
    }
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub const fn set_INP(&mut self, val: super::vals::DMA0_ITRIG_INMUX_INP) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for DMA0_ITRIG_INMUX {
    #[inline(always)]
    fn default() -> DMA0_ITRIG_INMUX {
        DMA0_ITRIG_INMUX(0)
    }
}
impl core::fmt::Debug for DMA0_ITRIG_INMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_ITRIG_INMUX")
            .field("INP", &self.INP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_ITRIG_INMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_ITRIG_INMUX {{ INP: {:?} }}", self.INP())
    }
}
#[doc = "DMA0 output trigger selection to become DMA0 trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_OTRIG_INMUX(pub u32);
impl DMA0_OTRIG_INMUX {
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[must_use]
    #[inline(always)]
    pub const fn INP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub const fn set_INP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for DMA0_OTRIG_INMUX {
    #[inline(always)]
    fn default() -> DMA0_OTRIG_INMUX {
        DMA0_OTRIG_INMUX(0)
    }
}
impl core::fmt::Debug for DMA0_OTRIG_INMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_OTRIG_INMUX")
            .field("INP", &self.INP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_OTRIG_INMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_OTRIG_INMUX {{ INP: {=u8:?} }}", self.INP())
    }
}
#[doc = "Enable DMA0 requests."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_REQ_ENA(pub u32);
impl DMA0_REQ_ENA {
    #[doc = "Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ_ENA(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub const fn set_REQ_ENA(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for DMA0_REQ_ENA {
    #[inline(always)]
    fn default() -> DMA0_REQ_ENA {
        DMA0_REQ_ENA(0)
    }
}
impl core::fmt::Debug for DMA0_REQ_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_REQ_ENA")
            .field("REQ_ENA", &self.REQ_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_REQ_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_REQ_ENA {{ REQ_ENA: {=u32:?} }}", self.REQ_ENA())
    }
}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_REQ_ENA_CLR(pub u32);
impl DMA0_REQ_ENA_CLR {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for DMA0_REQ_ENA_CLR {
    #[inline(always)]
    fn default() -> DMA0_REQ_ENA_CLR {
        DMA0_REQ_ENA_CLR(0)
    }
}
impl core::fmt::Debug for DMA0_REQ_ENA_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_REQ_ENA_CLR")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_REQ_ENA_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_REQ_ENA_CLR {{ CLR: {=u32:?} }}", self.CLR())
    }
}
#[doc = "Set one or several bits in DMA0_REQ_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA0_REQ_ENA_SET(pub u32);
impl DMA0_REQ_ENA_SET {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn SET(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register."]
    #[inline(always)]
    pub const fn set_SET(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for DMA0_REQ_ENA_SET {
    #[inline(always)]
    fn default() -> DMA0_REQ_ENA_SET {
        DMA0_REQ_ENA_SET(0)
    }
}
impl core::fmt::Debug for DMA0_REQ_ENA_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA0_REQ_ENA_SET")
            .field("SET", &self.SET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA0_REQ_ENA_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA0_REQ_ENA_SET {{ SET: {=u32:?} }}", self.SET())
    }
}
#[doc = "Enable DMA1 triggers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_ITRIG_ENA(pub u32);
impl DMA1_ITRIG_ENA {
    #[doc = "Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn ITRIG_ENA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub const fn set_ITRIG_ENA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for DMA1_ITRIG_ENA {
    #[inline(always)]
    fn default() -> DMA1_ITRIG_ENA {
        DMA1_ITRIG_ENA(0)
    }
}
impl core::fmt::Debug for DMA1_ITRIG_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_ENA")
            .field("ITRIG_ENA", &self.ITRIG_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_ITRIG_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DMA1_ITRIG_ENA {{ ITRIG_ENA: {=u16:?} }}",
            self.ITRIG_ENA()
        )
    }
}
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_ITRIG_ENA_CLR(pub u32);
impl DMA1_ITRIG_ENA_CLR {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_ITRIG_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for DMA1_ITRIG_ENA_CLR {
    #[inline(always)]
    fn default() -> DMA1_ITRIG_ENA_CLR {
        DMA1_ITRIG_ENA_CLR(0)
    }
}
impl core::fmt::Debug for DMA1_ITRIG_ENA_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_ENA_CLR")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_ITRIG_ENA_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_ITRIG_ENA_CLR {{ CLR: {=u16:?} }}", self.CLR())
    }
}
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_ITRIG_ENA_SET(pub u32);
impl DMA1_ITRIG_ENA_SET {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn SET(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register."]
    #[inline(always)]
    pub const fn set_SET(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for DMA1_ITRIG_ENA_SET {
    #[inline(always)]
    fn default() -> DMA1_ITRIG_ENA_SET {
        DMA1_ITRIG_ENA_SET(0)
    }
}
impl core::fmt::Debug for DMA1_ITRIG_ENA_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_ENA_SET")
            .field("SET", &self.SET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_ITRIG_ENA_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_ITRIG_ENA_SET {{ SET: {=u16:?} }}", self.SET())
    }
}
#[doc = "Trigger select register for DMA1 channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_ITRIG_INMUX(pub u32);
impl DMA1_ITRIG_INMUX {
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[must_use]
    #[inline(always)]
    pub const fn INP(&self) -> super::vals::DMA1_ITRIG_INMUX_INP {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DMA1_ITRIG_INMUX_INP::from_bits(val as u8)
    }
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub const fn set_INP(&mut self, val: super::vals::DMA1_ITRIG_INMUX_INP) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DMA1_ITRIG_INMUX {
    #[inline(always)]
    fn default() -> DMA1_ITRIG_INMUX {
        DMA1_ITRIG_INMUX(0)
    }
}
impl core::fmt::Debug for DMA1_ITRIG_INMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_ITRIG_INMUX")
            .field("INP", &self.INP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_ITRIG_INMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_ITRIG_INMUX {{ INP: {:?} }}", self.INP())
    }
}
#[doc = "DMA1 output trigger selection to become DMA1 trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_OTRIG_INMUX(pub u32);
impl DMA1_OTRIG_INMUX {
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[must_use]
    #[inline(always)]
    pub const fn INP(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub const fn set_INP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for DMA1_OTRIG_INMUX {
    #[inline(always)]
    fn default() -> DMA1_OTRIG_INMUX {
        DMA1_OTRIG_INMUX(0)
    }
}
impl core::fmt::Debug for DMA1_OTRIG_INMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_OTRIG_INMUX")
            .field("INP", &self.INP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_OTRIG_INMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_OTRIG_INMUX {{ INP: {=u8:?} }}", self.INP())
    }
}
#[doc = "Enable DMA1 requests."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_REQ_ENA(pub u32);
impl DMA1_REQ_ENA {
    #[doc = "Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn REQ_ENA(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub const fn set_REQ_ENA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DMA1_REQ_ENA {
    #[inline(always)]
    fn default() -> DMA1_REQ_ENA {
        DMA1_REQ_ENA(0)
    }
}
impl core::fmt::Debug for DMA1_REQ_ENA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_REQ_ENA")
            .field("REQ_ENA", &self.REQ_ENA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_REQ_ENA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_REQ_ENA {{ REQ_ENA: {=u16:?} }}", self.REQ_ENA())
    }
}
#[doc = "Clear one or several bits in DMA1_REQ_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_REQ_ENA_CLR(pub u32);
impl DMA1_REQ_ENA_CLR {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn CLR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register."]
    #[inline(always)]
    pub const fn set_CLR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DMA1_REQ_ENA_CLR {
    #[inline(always)]
    fn default() -> DMA1_REQ_ENA_CLR {
        DMA1_REQ_ENA_CLR(0)
    }
}
impl core::fmt::Debug for DMA1_REQ_ENA_CLR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_REQ_ENA_CLR")
            .field("CLR", &self.CLR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_REQ_ENA_CLR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_REQ_ENA_CLR {{ CLR: {=u16:?} }}", self.CLR())
    }
}
#[doc = "Set one or several bits in DMA1_REQ_ENA register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DMA1_REQ_ENA_SET(pub u32);
impl DMA1_REQ_ENA_SET {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA1_REQ_ENA register."]
    #[must_use]
    #[inline(always)]
    pub const fn SET(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA1_REQ_ENA register."]
    #[inline(always)]
    pub const fn set_SET(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for DMA1_REQ_ENA_SET {
    #[inline(always)]
    fn default() -> DMA1_REQ_ENA_SET {
        DMA1_REQ_ENA_SET(0)
    }
}
impl core::fmt::Debug for DMA1_REQ_ENA_SET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA1_REQ_ENA_SET")
            .field("SET", &self.SET())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DMA1_REQ_ENA_SET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DMA1_REQ_ENA_SET {{ SET: {=u16:?} }}", self.SET())
    }
}
#[doc = "Selection for frequency measurement reference clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQMEAS_REF(pub u32);
impl FREQMEAS_REF {
    #[doc = "Clock source number (decimal value) for frequency measure function reference clock:."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKIN(&self) -> super::vals::FREQMEAS_REF_CLKIN {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::FREQMEAS_REF_CLKIN::from_bits(val as u8)
    }
    #[doc = "Clock source number (decimal value) for frequency measure function reference clock:."]
    #[inline(always)]
    pub const fn set_CLKIN(&mut self, val: super::vals::FREQMEAS_REF_CLKIN) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for FREQMEAS_REF {
    #[inline(always)]
    fn default() -> FREQMEAS_REF {
        FREQMEAS_REF(0)
    }
}
impl core::fmt::Debug for FREQMEAS_REF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQMEAS_REF")
            .field("CLKIN", &self.CLKIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREQMEAS_REF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FREQMEAS_REF {{ CLKIN: {:?} }}", self.CLKIN())
    }
}
#[doc = "Selection for frequency measurement target clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FREQMEAS_TARGET(pub u32);
impl FREQMEAS_TARGET {
    #[doc = "Clock source number (decimal value) for frequency measure function target clock:."]
    #[must_use]
    #[inline(always)]
    pub const fn CLKIN(&self) -> super::vals::FREQMEAS_TARGET_CLKIN {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::FREQMEAS_TARGET_CLKIN::from_bits(val as u8)
    }
    #[doc = "Clock source number (decimal value) for frequency measure function target clock:."]
    #[inline(always)]
    pub const fn set_CLKIN(&mut self, val: super::vals::FREQMEAS_TARGET_CLKIN) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for FREQMEAS_TARGET {
    #[inline(always)]
    fn default() -> FREQMEAS_TARGET {
        FREQMEAS_TARGET(0)
    }
}
impl core::fmt::Debug for FREQMEAS_TARGET {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FREQMEAS_TARGET")
            .field("CLKIN", &self.CLKIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FREQMEAS_TARGET {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FREQMEAS_TARGET {{ CLKIN: {:?} }}", self.CLKIN())
    }
}
#[doc = "Pin interrupt secure select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PINTSECSEL(pub u32);
impl PINTSECSEL {
    #[doc = "Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[must_use]
    #[inline(always)]
    pub const fn INTPIN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub const fn set_INTPIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for PINTSECSEL {
    #[inline(always)]
    fn default() -> PINTSECSEL {
        PINTSECSEL(0)
    }
}
impl core::fmt::Debug for PINTSECSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINTSECSEL")
            .field("INTPIN", &self.INTPIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PINTSECSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PINTSECSEL {{ INTPIN: {=u8:?} }}", self.INTPIN())
    }
}
#[doc = "Pin interrupt select register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PINTSEL(pub u32);
impl PINTSEL {
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[must_use]
    #[inline(always)]
    pub const fn INTPIN(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub const fn set_INTPIN(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for PINTSEL {
    #[inline(always)]
    fn default() -> PINTSEL {
        PINTSEL(0)
    }
}
impl core::fmt::Debug for PINTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PINTSEL")
            .field("INTPIN", &self.INTPIN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PINTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PINTSEL {{ INTPIN: {=u8:?} }}", self.INTPIN())
    }
}
#[doc = "Input mux register for SCT0 input."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCT0_INMUX(pub u32);
impl SCT0_INMUX {
    #[doc = "Input number to SCT0 inputs 0 to 6.."]
    #[must_use]
    #[inline(always)]
    pub const fn INP_N(&self) -> super::vals::INP_N {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::INP_N::from_bits(val as u8)
    }
    #[doc = "Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub const fn set_INP_N(&mut self, val: super::vals::INP_N) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for SCT0_INMUX {
    #[inline(always)]
    fn default() -> SCT0_INMUX {
        SCT0_INMUX(0)
    }
}
impl core::fmt::Debug for SCT0_INMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCT0_INMUX")
            .field("INP_N", &self.INP_N())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SCT0_INMUX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SCT0_INMUX {{ INP_N: {:?} }}", self.INP_N())
    }
}
#[doc = "Capture select registers for TIMER0 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER0CAPTSEL(pub u32);
impl TIMER0CAPTSEL {
    #[doc = "Input number to TIMER0 capture inputs 0 to 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTSEL(&self) -> super::vals::TIMER0CAPTSEL_CAPTSEL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TIMER0CAPTSEL_CAPTSEL::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER0 capture inputs 0 to 4."]
    #[inline(always)]
    pub const fn set_CAPTSEL(&mut self, val: super::vals::TIMER0CAPTSEL_CAPTSEL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TIMER0CAPTSEL {
    #[inline(always)]
    fn default() -> TIMER0CAPTSEL {
        TIMER0CAPTSEL(0)
    }
}
impl core::fmt::Debug for TIMER0CAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0CAPTSEL")
            .field("CAPTSEL", &self.CAPTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER0CAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER0CAPTSEL {{ CAPTSEL: {:?} }}", self.CAPTSEL())
    }
}
#[doc = "Capture select registers for TIMER1 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER1CAPTSEL(pub u32);
impl TIMER1CAPTSEL {
    #[doc = "Input number to TIMER1 capture inputs 0 to 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTSEL(&self) -> super::vals::TIMER1CAPTSEL_CAPTSEL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TIMER1CAPTSEL_CAPTSEL::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER1 capture inputs 0 to 4."]
    #[inline(always)]
    pub const fn set_CAPTSEL(&mut self, val: super::vals::TIMER1CAPTSEL_CAPTSEL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TIMER1CAPTSEL {
    #[inline(always)]
    fn default() -> TIMER1CAPTSEL {
        TIMER1CAPTSEL(0)
    }
}
impl core::fmt::Debug for TIMER1CAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1CAPTSEL")
            .field("CAPTSEL", &self.CAPTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER1CAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER1CAPTSEL {{ CAPTSEL: {:?} }}", self.CAPTSEL())
    }
}
#[doc = "Capture select registers for TIMER2 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER2CAPTSEL(pub u32);
impl TIMER2CAPTSEL {
    #[doc = "Input number to TIMER2 capture inputs 0 to 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTSEL(&self) -> super::vals::TIMER2CAPTSEL_CAPTSEL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TIMER2CAPTSEL_CAPTSEL::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER2 capture inputs 0 to 4."]
    #[inline(always)]
    pub const fn set_CAPTSEL(&mut self, val: super::vals::TIMER2CAPTSEL_CAPTSEL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TIMER2CAPTSEL {
    #[inline(always)]
    fn default() -> TIMER2CAPTSEL {
        TIMER2CAPTSEL(0)
    }
}
impl core::fmt::Debug for TIMER2CAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2CAPTSEL")
            .field("CAPTSEL", &self.CAPTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER2CAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER2CAPTSEL {{ CAPTSEL: {:?} }}", self.CAPTSEL())
    }
}
#[doc = "Capture select registers for TIMER3 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER3CAPTSEL(pub u32);
impl TIMER3CAPTSEL {
    #[doc = "Input number to TIMER3 capture inputs 0 to 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTSEL(&self) -> super::vals::TIMER3CAPTSEL_CAPTSEL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TIMER3CAPTSEL_CAPTSEL::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER3 capture inputs 0 to 4."]
    #[inline(always)]
    pub const fn set_CAPTSEL(&mut self, val: super::vals::TIMER3CAPTSEL_CAPTSEL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TIMER3CAPTSEL {
    #[inline(always)]
    fn default() -> TIMER3CAPTSEL {
        TIMER3CAPTSEL(0)
    }
}
impl core::fmt::Debug for TIMER3CAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER3CAPTSEL")
            .field("CAPTSEL", &self.CAPTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER3CAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER3CAPTSEL {{ CAPTSEL: {:?} }}", self.CAPTSEL())
    }
}
#[doc = "Capture select registers for TIMER4 inputs."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TIMER4CAPTSEL(pub u32);
impl TIMER4CAPTSEL {
    #[doc = "Input number to TIMER4 capture inputs 0 to 4."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPTSEL(&self) -> super::vals::TIMER4CAPTSEL_CAPTSEL {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::TIMER4CAPTSEL_CAPTSEL::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER4 capture inputs 0 to 4."]
    #[inline(always)]
    pub const fn set_CAPTSEL(&mut self, val: super::vals::TIMER4CAPTSEL_CAPTSEL) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for TIMER4CAPTSEL {
    #[inline(always)]
    fn default() -> TIMER4CAPTSEL {
        TIMER4CAPTSEL(0)
    }
}
impl core::fmt::Debug for TIMER4CAPTSEL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER4CAPTSEL")
            .field("CAPTSEL", &self.CAPTSEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TIMER4CAPTSEL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TIMER4CAPTSEL {{ CAPTSEL: {:?} }}", self.CAPTSEL())
    }
}
