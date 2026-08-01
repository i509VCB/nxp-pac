#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "CoolFlux BSP32."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsp32 {
    ptr: *mut u8,
}
unsafe impl Send for Bsp32 {}
unsafe impl Sync for Bsp32 {}
impl Bsp32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Offset address register for program memory."]
    #[inline(always)]
    pub const fn offset_pmem(self) -> crate::pac::common::Reg<OffsetPmem, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Offset address register for X-data memory."]
    #[inline(always)]
    pub const fn offset_xmem(self) -> crate::pac::common::Reg<OffsetXmem, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Offset address register for Y-data memory."]
    #[inline(always)]
    pub const fn offset_ymem(self) -> crate::pac::common::Reg<OffsetYmem, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Offset address register for mailbox peripheral."]
    #[inline(always)]
    pub const fn offset_mailbox(
        self,
    ) -> crate::pac::common::Reg<OffsetMailbox, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "External interrupt register."]
    #[inline(always)]
    pub const fn interrupts_external(
        self,
    ) -> crate::pac::common::Reg<InterruptsExternal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt status register."]
    #[inline(always)]
    pub const fn interrupts_status(
        self,
    ) -> crate::pac::common::Reg<InterruptsStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CoolFlux BSP32 gating override."]
    #[inline(always)]
    pub const fn cf_gating_override(
        self,
    ) -> crate::pac::common::Reg<CfGatingOverride, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT offset register."]
    #[inline(always)]
    pub const fn ivt_offset(self) -> crate::pac::common::Reg<IvtOffset, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CoolFlux BSP32 sleep mode register."]
    #[inline(always)]
    pub const fn sleep_mode(self) -> crate::pac::common::Reg<SleepMode, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT register content."]
    #[inline(always)]
    pub const fn ivt(self, n: usize) -> crate::pac::common::Reg<Ivt, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _)
        }
    }
    #[doc = "CoolFlux BSP32 IVT disable register."]
    #[inline(always)]
    pub const fn ivt_disable(self) -> crate::pac::common::Reg<IvtDisable, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
#[doc = "CoolFlux BSP32 gating override."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfGatingOverride(pub u32);
impl CfGatingOverride {
    #[doc = "CoolFlux BSP32 gating override."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 gating override."]
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
#[doc = "External interrupt register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InterruptsExternal(pub u32);
impl InterruptsExternal {
    #[doc = "External interrupt register."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "External interrupt register."]
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
#[doc = "Interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InterruptsStatus(pub u32);
impl InterruptsStatus {
    #[doc = "Interrupt status register."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register."]
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
#[doc = "CoolFlux BSP32 IVT register content."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ivt(pub u32);
impl Ivt {
    #[doc = "CoolFlux BSP32 IVT register content."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT register content."]
    #[inline(always)]
    pub const fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ivt {
    #[inline(always)]
    fn default() -> Ivt {
        Ivt(0)
    }
}
impl core::fmt::Debug for Ivt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ivt").field("val", &self.val()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ivt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ivt {{ val: {=u32:?} }}", self.val())
    }
}
#[doc = "CoolFlux BSP32 IVT disable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvtDisable(pub u32);
impl IvtDisable {
    #[doc = "CoolFlux BSP32 IVT disable register."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 IVT disable register."]
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
#[doc = "CoolFlux BSP32 IVT offset register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IvtOffset(pub u32);
impl IvtOffset {
    #[doc = "CoolFlux BSP32 IVT offset register."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "CoolFlux BSP32 IVT offset register."]
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
#[doc = "Offset address register for mailbox peripheral."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetMailbox(pub u32);
impl OffsetMailbox {
    #[doc = "Offset address register for mailbox peripheral."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Offset address register for mailbox peripheral."]
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
#[doc = "Offset address register for program memory."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetPmem(pub u32);
impl OffsetPmem {
    #[doc = "Offset address register for program memory."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Offset address register for program memory."]
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
#[doc = "Offset address register for X-data memory."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetXmem(pub u32);
impl OffsetXmem {
    #[doc = "Offset address register for X-data memory."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Offset address register for X-data memory."]
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
#[doc = "Offset address register for Y-data memory."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OffsetYmem(pub u32);
impl OffsetYmem {
    #[doc = "Offset address register for Y-data memory."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Offset address register for Y-data memory."]
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
#[doc = "CoolFlux BSP32 sleep mode register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SleepMode(pub u32);
impl SleepMode {
    #[doc = "CoolFlux BSP32 sleep mode register."]
    #[must_use]
    #[inline(always)]
    pub const fn val(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "CoolFlux BSP32 sleep mode register."]
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
