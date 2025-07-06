#[doc = "CoolFlux BSP32 gating override"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfGatingOverride(pub u32);
impl CfGatingOverride {
    #[doc = "CoolFlux BSP32 gating override"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 gating override"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for CfGatingOverride {
    #[inline(always)]
    fn default() -> CfGatingOverride {
        CfGatingOverride(0)
    }
}
impl core::fmt::Debug for CfGatingOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CfGatingOverride")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CfGatingOverride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CfGatingOverride {{ val: {=bool:?} }}", self.val())
    }
}
#[doc = "External interrupt register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InterruptsExternal(pub u32);
impl InterruptsExternal {
    #[doc = "External interrupt register"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External interrupt register"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for InterruptsExternal {
    #[inline(always)]
    fn default() -> InterruptsExternal {
        InterruptsExternal(0)
    }
}
impl core::fmt::Debug for InterruptsExternal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InterruptsExternal")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InterruptsExternal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "InterruptsExternal {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "Interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InterruptsStatus(pub u32);
impl InterruptsStatus {
    #[doc = "Interrupt status register"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for InterruptsStatus {
    #[inline(always)]
    fn default() -> InterruptsStatus {
        InterruptsStatus(0)
    }
}
impl core::fmt::Debug for InterruptsStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InterruptsStatus")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InterruptsStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "InterruptsStatus {{ val: {=bool:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT register 0 content"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivt0(pub u32);
impl Ivt0 {
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ivt0 {
    #[inline(always)]
    fn default() -> Ivt0 {
        Ivt0(0)
    }
}
impl core::fmt::Debug for Ivt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ivt0").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ivt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ivt0 {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT register 1 content"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivt1(pub u32);
impl Ivt1 {
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ivt1 {
    #[inline(always)]
    fn default() -> Ivt1 {
        Ivt1(0)
    }
}
impl core::fmt::Debug for Ivt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ivt1").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ivt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ivt1 {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT register 2 content"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivt2(pub u32);
impl Ivt2 {
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ivt2 {
    #[inline(always)]
    fn default() -> Ivt2 {
        Ivt2(0)
    }
}
impl core::fmt::Debug for Ivt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ivt2").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ivt2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ivt2 {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT register 3 content"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivt3(pub u32);
impl Ivt3 {
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ivt3 {
    #[inline(always)]
    fn default() -> Ivt3 {
        Ivt3(0)
    }
}
impl core::fmt::Debug for Ivt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ivt3").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ivt3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ivt3 {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT disable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvtDisable(pub u32);
impl IvtDisable {
    #[doc = "CoolFlux BSP32 IVT disable register"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 IVT disable register"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for IvtDisable {
    #[inline(always)]
    fn default() -> IvtDisable {
        IvtDisable(0)
    }
}
impl core::fmt::Debug for IvtDisable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvtDisable")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvtDisable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvtDisable {{ val: {=bool:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvtOffset(pub u32);
impl IvtOffset {
    #[doc = "CoolFlux BSP32 IVT offset register"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT offset register"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for IvtOffset {
    #[inline(always)]
    fn default() -> IvtOffset {
        IvtOffset(0)
    }
}
impl core::fmt::Debug for IvtOffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IvtOffset")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IvtOffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "IvtOffset {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "Offset address register for mailbox peripheral"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetMailbox(pub u32);
impl OffsetMailbox {
    #[doc = "Offset address register for mailbox peripheral"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Offset address register for mailbox peripheral"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OffsetMailbox {
    #[inline(always)]
    fn default() -> OffsetMailbox {
        OffsetMailbox(0)
    }
}
impl core::fmt::Debug for OffsetMailbox {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OffsetMailbox")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OffsetMailbox {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OffsetMailbox {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "Offset address register for program memory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetPmem(pub u32);
impl OffsetPmem {
    #[doc = "Offset address register for program memory"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Offset address register for program memory"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for OffsetPmem {
    #[inline(always)]
    fn default() -> OffsetPmem {
        OffsetPmem(0)
    }
}
impl core::fmt::Debug for OffsetPmem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OffsetPmem")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OffsetPmem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OffsetPmem {{ val: {=u8:?} }}", self.val())
    }
}
#[doc = "Offset address register for X-data memory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetXmem(pub u32);
impl OffsetXmem {
    #[doc = "Offset address register for X-data memory"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Offset address register for X-data memory"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for OffsetXmem {
    #[inline(always)]
    fn default() -> OffsetXmem {
        OffsetXmem(0)
    }
}
impl core::fmt::Debug for OffsetXmem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OffsetXmem")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OffsetXmem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OffsetXmem {{ val: {=u8:?} }}", self.val())
    }
}
#[doc = "Offset address register for Y-data memory"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetYmem(pub u32);
impl OffsetYmem {
    #[doc = "Offset address register for Y-data memory"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Offset address register for Y-data memory"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for OffsetYmem {
    #[inline(always)]
    fn default() -> OffsetYmem {
        OffsetYmem(0)
    }
}
impl core::fmt::Debug for OffsetYmem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OffsetYmem")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OffsetYmem {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OffsetYmem {{ val: {=u8:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 sleep mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SleepMode(pub u32);
impl SleepMode {
    #[doc = "CoolFlux BSP32 sleep mode register"]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 sleep mode register"]
    #[inline(always)]
    pub const fn set_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for SleepMode {
    #[inline(always)]
    fn default() -> SleepMode {
        SleepMode(0)
    }
}
impl core::fmt::Debug for SleepMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SleepMode")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SleepMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SleepMode {{ val: {=bool:?} }}", self.val())
    }
}
