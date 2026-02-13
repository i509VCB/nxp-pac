#[doc = "Enable DMA0 triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ItrigEna(pub u32);
impl Dma0ItrigEna {
    #[doc = "Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn itrig_ena(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Controls the 22 trigger inputs of DMA0. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub const fn set_itrig_ena(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Dma0ItrigEna {
    #[inline(always)]
    fn default() -> Dma0ItrigEna {
        Dma0ItrigEna(0)
    }
}
impl core::fmt::Debug for Dma0ItrigEna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ItrigEna")
            .field("itrig_ena", &self.itrig_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ItrigEna {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma0ItrigEna {{ itrig_ena: {=u32:?} }}",
            self.itrig_ena()
        )
    }
}
#[doc = "Clear one or several bits in DMA0_ITRIG_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ItrigEnaClr(pub u32);
impl Dma0ItrigEnaClr {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Dma0ItrigEnaClr {
    #[inline(always)]
    fn default() -> Dma0ItrigEnaClr {
        Dma0ItrigEnaClr(0)
    }
}
impl core::fmt::Debug for Dma0ItrigEnaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ItrigEnaClr")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ItrigEnaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ItrigEnaClr {{ clr: {=u32:?} }}", self.clr())
    }
}
#[doc = "Set one or several bits in DMA0_ITRIG_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ItrigEnaSet(pub u32);
impl Dma0ItrigEnaSet {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA0_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Dma0ItrigEnaSet {
    #[inline(always)]
    fn default() -> Dma0ItrigEnaSet {
        Dma0ItrigEnaSet(0)
    }
}
impl core::fmt::Debug for Dma0ItrigEnaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ItrigEnaSet")
            .field("set", &self.set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ItrigEnaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ItrigEnaSet {{ set: {=u32:?} }}", self.set())
    }
}
#[doc = "Trigger select register for DMA0 channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ItrigInmux(pub u32);
impl Dma0ItrigInmux {
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Dma0ItrigInmuxInp {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Dma0ItrigInmuxInp::from_bits(val as u8)
    }
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Dma0ItrigInmuxInp) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Dma0ItrigInmux {
    #[inline(always)]
    fn default() -> Dma0ItrigInmux {
        Dma0ItrigInmux(0)
    }
}
impl core::fmt::Debug for Dma0ItrigInmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ItrigInmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ItrigInmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ItrigInmux {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DMA0 output trigger selection to become DMA0 trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0OtrigInmux(pub u32);
impl Dma0OtrigInmux {
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 22)."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Dma0OtrigInmux {
    #[inline(always)]
    fn default() -> Dma0OtrigInmux {
        Dma0OtrigInmux(0)
    }
}
impl core::fmt::Debug for Dma0OtrigInmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0OtrigInmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0OtrigInmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0OtrigInmux {{ inp: {=u8:?} }}", self.inp())
    }
}
#[doc = "Enable DMA0 requests"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEna(pub u32);
impl Dma0ReqEna {
    #[doc = "Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn req_ena(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Controls the 23 request inputs of DMA0. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub const fn set_req_ena(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Dma0ReqEna {
    #[inline(always)]
    fn default() -> Dma0ReqEna {
        Dma0ReqEna(0)
    }
}
impl core::fmt::Debug for Dma0ReqEna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEna")
            .field("req_ena", &self.req_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEna {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ReqEna {{ req_ena: {=u32:?} }}", self.req_ena())
    }
}
#[doc = "Clear one or several bits in DMA0_REQ_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnaClr(pub u32);
impl Dma0ReqEnaClr {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Dma0ReqEnaClr {
    #[inline(always)]
    fn default() -> Dma0ReqEnaClr {
        Dma0ReqEnaClr(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnaClr")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ReqEnaClr {{ clr: {=u32:?} }}", self.clr())
    }
}
#[doc = "Set one or several bits in DMA0_REQ_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0ReqEnaSet(pub u32);
impl Dma0ReqEnaSet {
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA0_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA0_REQ_ENA register"]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
}
impl Default for Dma0ReqEnaSet {
    #[inline(always)]
    fn default() -> Dma0ReqEnaSet {
        Dma0ReqEnaSet(0)
    }
}
impl core::fmt::Debug for Dma0ReqEnaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma0ReqEnaSet")
            .field("set", &self.set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma0ReqEnaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma0ReqEnaSet {{ set: {=u32:?} }}", self.set())
    }
}
#[doc = "Enable DMA1 triggers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ItrigEna(pub u32);
impl Dma1ItrigEna {
    #[doc = "Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn itrig_ena(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Controls the 15 trigger inputs of DMA1. If bit i is '1' the DMA trigger input #i is enabled."]
    #[inline(always)]
    pub const fn set_itrig_ena(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Dma1ItrigEna {
    #[inline(always)]
    fn default() -> Dma1ItrigEna {
        Dma1ItrigEna(0)
    }
}
impl core::fmt::Debug for Dma1ItrigEna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ItrigEna")
            .field("itrig_ena", &self.itrig_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ItrigEna {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dma1ItrigEna {{ itrig_ena: {=u16:?} }}",
            self.itrig_ena()
        )
    }
}
#[doc = "Clear one or several bits in DMA1_ITRIG_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ItrigEnaClr(pub u32);
impl Dma1ItrigEnaClr {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Dma1ItrigEnaClr {
    #[inline(always)]
    fn default() -> Dma1ItrigEnaClr {
        Dma1ItrigEnaClr(0)
    }
}
impl core::fmt::Debug for Dma1ItrigEnaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ItrigEnaClr")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ItrigEnaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ItrigEnaClr {{ clr: {=u16:?} }}", self.clr())
    }
}
#[doc = "Set one or several bits in DMA1_ITRIG_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ItrigEnaSet(pub u32);
impl Dma1ItrigEnaSet {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_ITRIG_ENA register is set to 1; if bit #i = 0 , no change in DMA1_ITRIG_ENA register"]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Dma1ItrigEnaSet {
    #[inline(always)]
    fn default() -> Dma1ItrigEnaSet {
        Dma1ItrigEnaSet(0)
    }
}
impl core::fmt::Debug for Dma1ItrigEnaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ItrigEnaSet")
            .field("set", &self.set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ItrigEnaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ItrigEnaSet {{ set: {=u16:?} }}", self.set())
    }
}
#[doc = "Trigger select register for DMA1 channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ItrigInmux(pub u32);
impl Dma1ItrigInmux {
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> super::vals::Dma1ItrigInmuxInp {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Dma1ItrigInmuxInp::from_bits(val as u8)
    }
    #[doc = "Trigger input number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: super::vals::Dma1ItrigInmuxInp) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dma1ItrigInmux {
    #[inline(always)]
    fn default() -> Dma1ItrigInmux {
        Dma1ItrigInmux(0)
    }
}
impl core::fmt::Debug for Dma1ItrigInmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ItrigInmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ItrigInmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ItrigInmux {{ inp: {:?} }}", self.inp())
    }
}
#[doc = "DMA1 output trigger selection to become DMA1 trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1OtrigInmux(pub u32);
impl Dma1OtrigInmux {
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[must_use]
    #[inline(always)]
    pub const fn inp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA trigger output number (decimal value) for DMA channel n (n = 0 to 9)."]
    #[inline(always)]
    pub const fn set_inp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dma1OtrigInmux {
    #[inline(always)]
    fn default() -> Dma1OtrigInmux {
        Dma1OtrigInmux(0)
    }
}
impl core::fmt::Debug for Dma1OtrigInmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1OtrigInmux")
            .field("inp", &self.inp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1OtrigInmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1OtrigInmux {{ inp: {=u8:?} }}", self.inp())
    }
}
#[doc = "Enable DMA1 requests"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEna(pub u32);
impl Dma1ReqEna {
    #[doc = "Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn req_ena(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Controls the 10 request inputs of DMA1. If bit i is '1' the DMA request input #i is enabled."]
    #[inline(always)]
    pub const fn set_req_ena(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Dma1ReqEna {
    #[inline(always)]
    fn default() -> Dma1ReqEna {
        Dma1ReqEna(0)
    }
}
impl core::fmt::Debug for Dma1ReqEna {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEna")
            .field("req_ena", &self.req_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEna {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ReqEna {{ req_ena: {=u16:?} }}", self.req_ena())
    }
}
#[doc = "Clear one or several bits in DMA1_REQ_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnaClr(pub u32);
impl Dma1ReqEnaClr {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is reset to 0; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Dma1ReqEnaClr {
    #[inline(always)]
    fn default() -> Dma1ReqEnaClr {
        Dma1ReqEnaClr(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnaClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnaClr")
            .field("clr", &self.clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnaClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ReqEnaClr {{ clr: {=u16:?} }}", self.clr())
    }
}
#[doc = "Set one or several bits in DMA1_REQ_ENA register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma1ReqEnaSet(pub u32);
impl Dma1ReqEnaSet {
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[must_use]
    #[inline(always)]
    pub const fn set(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Write : If bit #i = 1, bit #i in DMA1_REQ_ENA register is set to 1; if bit #i = 0 , no change in DMA1_REQ_ENA register"]
    #[inline(always)]
    pub const fn set_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Dma1ReqEnaSet {
    #[inline(always)]
    fn default() -> Dma1ReqEnaSet {
        Dma1ReqEnaSet(0)
    }
}
impl core::fmt::Debug for Dma1ReqEnaSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma1ReqEnaSet")
            .field("set", &self.set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dma1ReqEnaSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dma1ReqEnaSet {{ set: {=u16:?} }}", self.set())
    }
}
#[doc = "Selection for frequency measurement reference clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasRef(pub u32);
impl FreqmeasRef {
    #[doc = "Clock source number (decimal value) for frequency measure function reference clock:"]
    #[must_use]
    #[inline(always)]
    pub const fn clkin(&self) -> super::vals::FreqmeasRefClkin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::FreqmeasRefClkin::from_bits(val as u8)
    }
    #[doc = "Clock source number (decimal value) for frequency measure function reference clock:"]
    #[inline(always)]
    pub const fn set_clkin(&mut self, val: super::vals::FreqmeasRefClkin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for FreqmeasRef {
    #[inline(always)]
    fn default() -> FreqmeasRef {
        FreqmeasRef(0)
    }
}
impl core::fmt::Debug for FreqmeasRef {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasRef")
            .field("clkin", &self.clkin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasRef {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasRef {{ clkin: {:?} }}", self.clkin())
    }
}
#[doc = "Selection for frequency measurement target clock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FreqmeasTarget(pub u32);
impl FreqmeasTarget {
    #[doc = "Clock source number (decimal value) for frequency measure function target clock:"]
    #[must_use]
    #[inline(always)]
    pub const fn clkin(&self) -> super::vals::FreqmeasTargetClkin {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::FreqmeasTargetClkin::from_bits(val as u8)
    }
    #[doc = "Clock source number (decimal value) for frequency measure function target clock:"]
    #[inline(always)]
    pub const fn set_clkin(&mut self, val: super::vals::FreqmeasTargetClkin) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for FreqmeasTarget {
    #[inline(always)]
    fn default() -> FreqmeasTarget {
        FreqmeasTarget(0)
    }
}
impl core::fmt::Debug for FreqmeasTarget {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FreqmeasTarget")
            .field("clkin", &self.clkin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FreqmeasTarget {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FreqmeasTarget {{ clkin: {:?} }}", self.clkin())
    }
}
#[doc = "Pin interrupt secure select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pintsecsel(pub u32);
impl Pintsecsel {
    #[doc = "Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[must_use]
    #[inline(always)]
    pub const fn intpin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Pin number select for pin interrupt secure or pattern match engine input. For PIO0_x: INTPIN = x. PIO0_0 to PIO0_31 correspond to numbers 0 to 31."]
    #[inline(always)]
    pub const fn set_intpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Pintsecsel {
    #[inline(always)]
    fn default() -> Pintsecsel {
        Pintsecsel(0)
    }
}
impl core::fmt::Debug for Pintsecsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pintsecsel")
            .field("intpin", &self.intpin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pintsecsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pintsecsel {{ intpin: {=u8:?} }}", self.intpin())
    }
}
#[doc = "Pin interrupt select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pintsel(pub u32);
impl Pintsel {
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[must_use]
    #[inline(always)]
    pub const fn intpin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Pin number select for pin interrupt or pattern match engine input. For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub const fn set_intpin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Pintsel {
    #[inline(always)]
    fn default() -> Pintsel {
        Pintsel(0)
    }
}
impl core::fmt::Debug for Pintsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pintsel")
            .field("intpin", &self.intpin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pintsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pintsel {{ intpin: {=u8:?} }}", self.intpin())
    }
}
#[doc = "Input mux register for SCT0 input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0Inmux(pub u32);
impl Sct0Inmux {
    #[doc = "Input number to SCT0 inputs 0 to 6.."]
    #[must_use]
    #[inline(always)]
    pub const fn inp_n(&self) -> super::vals::InpN {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::InpN::from_bits(val as u8)
    }
    #[doc = "Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub const fn set_inp_n(&mut self, val: super::vals::InpN) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Sct0Inmux {
    #[inline(always)]
    fn default() -> Sct0Inmux {
        Sct0Inmux(0)
    }
}
impl core::fmt::Debug for Sct0Inmux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sct0Inmux")
            .field("inp_n", &self.inp_n())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sct0Inmux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sct0Inmux {{ inp_n: {:?} }}", self.inp_n())
    }
}
#[doc = "Capture select registers for TIMER0 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0captsel(pub u32);
impl Timer0captsel {
    #[doc = "Input number to TIMER0 capture inputs 0 to 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captsel(&self) -> super::vals::Timer0captselCaptsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Timer0captselCaptsel::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER0 capture inputs 0 to 4"]
    #[inline(always)]
    pub const fn set_captsel(&mut self, val: super::vals::Timer0captselCaptsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Timer0captsel {
    #[inline(always)]
    fn default() -> Timer0captsel {
        Timer0captsel(0)
    }
}
impl core::fmt::Debug for Timer0captsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0captsel")
            .field("captsel", &self.captsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0captsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer0captsel {{ captsel: {:?} }}", self.captsel())
    }
}
#[doc = "Capture select registers for TIMER1 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1captsel(pub u32);
impl Timer1captsel {
    #[doc = "Input number to TIMER1 capture inputs 0 to 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captsel(&self) -> super::vals::Timer1captselCaptsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Timer1captselCaptsel::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER1 capture inputs 0 to 4"]
    #[inline(always)]
    pub const fn set_captsel(&mut self, val: super::vals::Timer1captselCaptsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Timer1captsel {
    #[inline(always)]
    fn default() -> Timer1captsel {
        Timer1captsel(0)
    }
}
impl core::fmt::Debug for Timer1captsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1captsel")
            .field("captsel", &self.captsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1captsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer1captsel {{ captsel: {:?} }}", self.captsel())
    }
}
#[doc = "Capture select registers for TIMER2 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer2captsel(pub u32);
impl Timer2captsel {
    #[doc = "Input number to TIMER2 capture inputs 0 to 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captsel(&self) -> super::vals::Timer2captselCaptsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Timer2captselCaptsel::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER2 capture inputs 0 to 4"]
    #[inline(always)]
    pub const fn set_captsel(&mut self, val: super::vals::Timer2captselCaptsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Timer2captsel {
    #[inline(always)]
    fn default() -> Timer2captsel {
        Timer2captsel(0)
    }
}
impl core::fmt::Debug for Timer2captsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer2captsel")
            .field("captsel", &self.captsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer2captsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer2captsel {{ captsel: {:?} }}", self.captsel())
    }
}
#[doc = "Capture select registers for TIMER3 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer3captsel(pub u32);
impl Timer3captsel {
    #[doc = "Input number to TIMER3 capture inputs 0 to 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captsel(&self) -> super::vals::Timer3captselCaptsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Timer3captselCaptsel::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER3 capture inputs 0 to 4"]
    #[inline(always)]
    pub const fn set_captsel(&mut self, val: super::vals::Timer3captselCaptsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Timer3captsel {
    #[inline(always)]
    fn default() -> Timer3captsel {
        Timer3captsel(0)
    }
}
impl core::fmt::Debug for Timer3captsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer3captsel")
            .field("captsel", &self.captsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer3captsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer3captsel {{ captsel: {:?} }}", self.captsel())
    }
}
#[doc = "Capture select registers for TIMER4 inputs"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer4captsel(pub u32);
impl Timer4captsel {
    #[doc = "Input number to TIMER4 capture inputs 0 to 4"]
    #[must_use]
    #[inline(always)]
    pub const fn captsel(&self) -> super::vals::Timer4captselCaptsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Timer4captselCaptsel::from_bits(val as u8)
    }
    #[doc = "Input number to TIMER4 capture inputs 0 to 4"]
    #[inline(always)]
    pub const fn set_captsel(&mut self, val: super::vals::Timer4captselCaptsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Timer4captsel {
    #[inline(always)]
    fn default() -> Timer4captsel {
        Timer4captsel(0)
    }
}
impl core::fmt::Debug for Timer4captsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer4captsel")
            .field("captsel", &self.captsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer4captsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Timer4captsel {{ captsel: {:?} }}", self.captsel())
    }
}
