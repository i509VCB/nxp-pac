#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USBC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs {}
unsafe impl Sync for Usbhs {}
impl Usbhs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Identification."]
    #[inline(always)]
    pub const fn ID(self) -> crate::pac::common::Reg<ID, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Hardware General."]
    #[inline(always)]
    pub const fn HWGENERAL(self) -> crate::pac::common::Reg<HWGENERAL, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Host Hardware Parameters."]
    #[inline(always)]
    pub const fn HWHOST(self) -> crate::pac::common::Reg<HWHOST, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Device Hardware Parameters."]
    #[inline(always)]
    pub const fn HWDEVICE(self) -> crate::pac::common::Reg<HWDEVICE, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Buffer Hardware Parameters."]
    #[inline(always)]
    pub const fn HWTXBUF(self) -> crate::pac::common::Reg<HWTXBUF, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RX Buffer Hardware Parameters."]
    #[inline(always)]
    pub const fn HWRXBUF(self) -> crate::pac::common::Reg<HWRXBUF, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Purpose Timer 0 Load."]
    #[inline(always)]
    pub const fn GPTIMER0LD(self) -> crate::pac::common::Reg<GPTIMER0LD, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "General Purpose Timer 0 Controller."]
    #[inline(always)]
    pub const fn GPTIMER0CTRL(
        self,
    ) -> crate::pac::common::Reg<GPTIMER0CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "General Purpose Timer 1 Load."]
    #[inline(always)]
    pub const fn GPTIMER1LD(self) -> crate::pac::common::Reg<GPTIMER1LD, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "General Purpose Timer 1 Controller."]
    #[inline(always)]
    pub const fn GPTIMER1CTRL(
        self,
    ) -> crate::pac::common::Reg<GPTIMER1CTRL, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "System Bus Configuration."]
    #[inline(always)]
    pub const fn SBUSCFG(self) -> crate::pac::common::Reg<SBUSCFG, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Capability Registers Length."]
    #[inline(always)]
    pub const fn CAPLENGTH(self) -> crate::pac::common::Reg<CAPLENGTH, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Host Controller Interface Version."]
    #[inline(always)]
    pub const fn HCIVERSION(self) -> crate::pac::common::Reg<HCIVERSION, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters."]
    #[inline(always)]
    pub const fn HCSPARAMS(self) -> crate::pac::common::Reg<HCSPARAMS, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Host Controller Capability Parameters."]
    #[inline(always)]
    pub const fn HCCPARAMS(self) -> crate::pac::common::Reg<HCCPARAMS, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Device Controller Interface Version."]
    #[inline(always)]
    pub const fn DCIVERSION(self) -> crate::pac::common::Reg<DCIVERSION, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Device Controller Capability Parameters."]
    #[inline(always)]
    pub const fn DCCPARAMS(self) -> crate::pac::common::Reg<DCCPARAMS, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB Command."]
    #[inline(always)]
    pub const fn USBCMD(self) -> crate::pac::common::Reg<USBCMD, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "USB Status."]
    #[inline(always)]
    pub const fn USBSTS(self) -> crate::pac::common::Reg<USBSTS, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn USBINTR(self) -> crate::pac::common::Reg<USBINTR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "USB Frame Index."]
    #[inline(always)]
    pub const fn FRINDEX(self) -> crate::pac::common::Reg<FRINDEX, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn DEVICEADDR(self) -> crate::pac::common::Reg<DEVICEADDR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Frame List Base Address."]
    #[inline(always)]
    pub const fn PERIODICLISTBASE(
        self,
    ) -> crate::pac::common::Reg<PERIODICLISTBASE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Next Asynchronous Address."]
    #[inline(always)]
    pub const fn ASYNCLISTADDR(
        self,
    ) -> crate::pac::common::Reg<ASYNCLISTADDR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Endpoint List Address."]
    #[inline(always)]
    pub const fn ENDPTLISTADDR(
        self,
    ) -> crate::pac::common::Reg<ENDPTLISTADDR, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Programmable Burst Size."]
    #[inline(always)]
    pub const fn BURSTSIZE(self) -> crate::pac::common::Reg<BURSTSIZE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "TX FIFO Fill Tuning."]
    #[inline(always)]
    pub const fn TXFILLTUNING(
        self,
    ) -> crate::pac::common::Reg<TXFILLTUNING, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Endpoint NAK."]
    #[inline(always)]
    pub const fn ENDPTNAK(self) -> crate::pac::common::Reg<ENDPTNAK, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn ENDPTNAKEN(self) -> crate::pac::common::Reg<ENDPTNAKEN, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Configure Flag."]
    #[inline(always)]
    pub const fn CONFIGFLAG(self) -> crate::pac::common::Reg<CONFIGFLAG, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Port Status and Control."]
    #[inline(always)]
    pub const fn PORTSC1(self) -> crate::pac::common::Reg<PORTSC1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "On-The-Go Status and Control."]
    #[inline(always)]
    pub const fn OTGSC(self) -> crate::pac::common::Reg<OTGSC, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "USB Device Mode."]
    #[inline(always)]
    pub const fn USBMODE(self) -> crate::pac::common::Reg<USBMODE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Endpoint Setup Status."]
    #[inline(always)]
    pub const fn ENDPTSETUPSTAT(
        self,
    ) -> crate::pac::common::Reg<ENDPTSETUPSTAT, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Endpoint Prime."]
    #[inline(always)]
    pub const fn ENDPTPRIME(self) -> crate::pac::common::Reg<ENDPTPRIME, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Endpoint Flush."]
    #[inline(always)]
    pub const fn ENDPTFLUSH(self) -> crate::pac::common::Reg<ENDPTFLUSH, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Endpoint Status."]
    #[inline(always)]
    pub const fn ENDPTSTAT(self) -> crate::pac::common::Reg<ENDPTSTAT, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Endpoint Complete."]
    #[inline(always)]
    pub const fn ENDPTCOMPLETE(
        self,
    ) -> crate::pac::common::Reg<ENDPTCOMPLETE, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Endpoint Control 0."]
    #[inline(always)]
    pub const fn ENDPTCTRL0(self) -> crate::pac::common::Reg<ENDPTCTRL0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Endpoint Control 1."]
    #[inline(always)]
    pub const fn ENDPTCTRL1(self) -> crate::pac::common::Reg<ENDPTCTRL1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "Endpoint Control 2."]
    #[inline(always)]
    pub const fn ENDPTCTRL2(self) -> crate::pac::common::Reg<ENDPTCTRL2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Endpoint Control 3."]
    #[inline(always)]
    pub const fn ENDPTCTRL3(self) -> crate::pac::common::Reg<ENDPTCTRL3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Endpoint Control 4."]
    #[inline(always)]
    pub const fn ENDPTCTRL4(self) -> crate::pac::common::Reg<ENDPTCTRL4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Endpoint Control 5."]
    #[inline(always)]
    pub const fn ENDPTCTRL5(self) -> crate::pac::common::Reg<ENDPTCTRL5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "Endpoint Control 6."]
    #[inline(always)]
    pub const fn ENDPTCTRL6(self) -> crate::pac::common::Reg<ENDPTCTRL6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "Endpoint Control 7."]
    #[inline(always)]
    pub const fn ENDPTCTRL7(self) -> crate::pac::common::Reg<ENDPTCTRL7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
}
#[doc = "Next Asynchronous Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ASYNCLISTADDR(pub u32);
impl ASYNCLISTADDR {
    #[doc = "Link Pointer Low (LPL)."]
    #[must_use]
    #[inline(always)]
    pub const fn ASYBASE(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Link Pointer Low (LPL)."]
    #[inline(always)]
    pub const fn set_ASYBASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for ASYNCLISTADDR {
    #[inline(always)]
    fn default() -> ASYNCLISTADDR {
        ASYNCLISTADDR(0)
    }
}
impl core::fmt::Debug for ASYNCLISTADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASYNCLISTADDR")
            .field("ASYBASE", &self.ASYBASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ASYNCLISTADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ASYNCLISTADDR {{ ASYBASE: {=u32:?} }}", self.ASYBASE())
    }
}
#[doc = "Programmable Burst Size."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BURSTSIZE(pub u32);
impl BURSTSIZE {
    #[doc = "Programmable RX Burst Size."]
    #[must_use]
    #[inline(always)]
    pub const fn RXPBURST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable RX Burst Size."]
    #[inline(always)]
    pub const fn set_RXPBURST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Programmable TX Burst Size."]
    #[must_use]
    #[inline(always)]
    pub const fn TXPBURST(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable TX Burst Size."]
    #[inline(always)]
    pub const fn set_TXPBURST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for BURSTSIZE {
    #[inline(always)]
    fn default() -> BURSTSIZE {
        BURSTSIZE(0)
    }
}
impl core::fmt::Debug for BURSTSIZE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BURSTSIZE")
            .field("RXPBURST", &self.RXPBURST())
            .field("TXPBURST", &self.TXPBURST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BURSTSIZE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BURSTSIZE {{ RXPBURST: {=u8:?}, TXPBURST: {=u8:?} }}",
            self.RXPBURST(),
            self.TXPBURST()
        )
    }
}
#[doc = "Capability Registers Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CAPLENGTH(pub u8);
impl CAPLENGTH {
    #[doc = "Capability Length."]
    #[must_use]
    #[inline(always)]
    pub const fn CAPLENGTH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Capability Length."]
    #[inline(always)]
    pub const fn set_CAPLENGTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for CAPLENGTH {
    #[inline(always)]
    fn default() -> CAPLENGTH {
        CAPLENGTH(0)
    }
}
impl core::fmt::Debug for CAPLENGTH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAPLENGTH")
            .field("CAPLENGTH", &self.CAPLENGTH())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CAPLENGTH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CAPLENGTH {{ CAPLENGTH: {=u8:?} }}", self.CAPLENGTH())
    }
}
#[doc = "Configure Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CONFIGFLAG(pub u32);
impl CONFIGFLAG {
    #[doc = "Configure Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn CF(&self) -> CF {
        let val = (self.0 >> 0usize) & 0x01;
        CF::from_bits(val as u8)
    }
    #[doc = "Configure Flag."]
    #[inline(always)]
    pub const fn set_CF(&mut self, val: CF) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CONFIGFLAG {
    #[inline(always)]
    fn default() -> CONFIGFLAG {
        CONFIGFLAG(0)
    }
}
impl core::fmt::Debug for CONFIGFLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIGFLAG")
            .field("CF", &self.CF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CONFIGFLAG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CONFIGFLAG {{ CF: {:?} }}", self.CF())
    }
}
#[doc = "Device Controller Capability Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCCPARAMS(pub u32);
impl DCCPARAMS {
    #[doc = "Device Endpoint Number."]
    #[must_use]
    #[inline(always)]
    pub const fn DEN(&self) -> DEN {
        let val = (self.0 >> 0usize) & 0x1f;
        DEN::from_bits(val as u8)
    }
    #[doc = "Device Endpoint Number."]
    #[inline(always)]
    pub const fn set_DEN(&mut self, val: DEN) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Device Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn DC(&self) -> DCCPARAMS_DC {
        let val = (self.0 >> 7usize) & 0x01;
        DCCPARAMS_DC::from_bits(val as u8)
    }
    #[doc = "Device Capable."]
    #[inline(always)]
    pub const fn set_DC(&mut self, val: DCCPARAMS_DC) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Host Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn HC(&self) -> DCCPARAMS_HC {
        let val = (self.0 >> 8usize) & 0x01;
        DCCPARAMS_HC::from_bits(val as u8)
    }
    #[doc = "Host Capable."]
    #[inline(always)]
    pub const fn set_HC(&mut self, val: DCCPARAMS_HC) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for DCCPARAMS {
    #[inline(always)]
    fn default() -> DCCPARAMS {
        DCCPARAMS(0)
    }
}
impl core::fmt::Debug for DCCPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCCPARAMS")
            .field("DEN", &self.DEN())
            .field("DC", &self.DC())
            .field("HC", &self.HC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCCPARAMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCCPARAMS {{ DEN: {:?}, DC: {:?}, HC: {:?} }}",
            self.DEN(),
            self.DC(),
            self.HC()
        )
    }
}
#[doc = "Device Controller Interface Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DCIVERSION(pub u16);
impl DCIVERSION {
    #[doc = "Device Controller Interface Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn DCIVERSION(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device Controller Interface Version Number."]
    #[inline(always)]
    pub const fn set_DCIVERSION(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for DCIVERSION {
    #[inline(always)]
    fn default() -> DCIVERSION {
        DCIVERSION(0)
    }
}
impl core::fmt::Debug for DCIVERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCIVERSION")
            .field("DCIVERSION", &self.DCIVERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DCIVERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DCIVERSION {{ DCIVERSION: {=u16:?} }}",
            self.DCIVERSION()
        )
    }
}
#[doc = "Device Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEVICEADDR(pub u32);
impl DEVICEADDR {
    #[doc = "Device Address Advance."]
    #[must_use]
    #[inline(always)]
    pub const fn USBADRA(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device Address Advance."]
    #[inline(always)]
    pub const fn set_USBADRA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn USBADR(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn set_USBADR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for DEVICEADDR {
    #[inline(always)]
    fn default() -> DEVICEADDR {
        DEVICEADDR(0)
    }
}
impl core::fmt::Debug for DEVICEADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICEADDR")
            .field("USBADRA", &self.USBADRA())
            .field("USBADR", &self.USBADR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEVICEADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEVICEADDR {{ USBADRA: {=bool:?}, USBADR: {=u8:?} }}",
            self.USBADRA(),
            self.USBADR()
        )
    }
}
#[doc = "Endpoint Complete."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCOMPLETE(pub u32);
impl ENDPTCOMPLETE {
    #[doc = "Endpoint Receive Complete Event."]
    #[must_use]
    #[inline(always)]
    pub const fn ERCE(&self) -> ERCE {
        let val = (self.0 >> 0usize) & 0xff;
        ERCE::from_bits(val as u8)
    }
    #[doc = "Endpoint Receive Complete Event."]
    #[inline(always)]
    pub const fn set_ERCE(&mut self, val: ERCE) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Complete Event."]
    #[must_use]
    #[inline(always)]
    pub const fn ETCE(&self) -> ETCE {
        let val = (self.0 >> 16usize) & 0xff;
        ETCE::from_bits(val as u8)
    }
    #[doc = "Endpoint Transmit Complete Event."]
    #[inline(always)]
    pub const fn set_ETCE(&mut self, val: ETCE) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTCOMPLETE {
    #[inline(always)]
    fn default() -> ENDPTCOMPLETE {
        ENDPTCOMPLETE(0)
    }
}
impl core::fmt::Debug for ENDPTCOMPLETE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCOMPLETE")
            .field("ERCE", &self.ERCE())
            .field("ETCE", &self.ETCE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCOMPLETE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCOMPLETE {{ ERCE: {:?}, ETCE: {:?} }}",
            self.ERCE(),
            self.ETCE()
        )
    }
}
#[doc = "Endpoint Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL0(pub u32);
impl ENDPTCTRL0 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL0_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL0_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL0_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL0_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL0_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL0_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL0_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL0_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL0_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL0_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL0_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL0_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL0 {
    #[inline(always)]
    fn default() -> ENDPTCTRL0 {
        ENDPTCTRL0(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL0")
            .field("RXS", &self.RXS())
            .field("RXT", &self.RXT())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXT", &self.TXT())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL0 {{ RXS: {:?}, RXT: {=u8:?}, RXE: {:?}, TXS: {:?}, TXT: {=u8:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXT(),
            self.RXE(),
            self.TXS(),
            self.TXT(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL1(pub u32);
impl ENDPTCTRL1 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL1_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL1_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL1_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL1_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL1_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL1_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL1_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL1_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL1_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL1_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL1_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL1_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL1_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL1_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL1_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL1_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL1_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL1_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL1_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL1_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL1_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL1_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL1_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL1_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL1_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL1_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL1_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL1_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL1_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL1_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL1 {
    #[inline(always)]
    fn default() -> ENDPTCTRL1 {
        ENDPTCTRL1(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL1")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL1 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL2(pub u32);
impl ENDPTCTRL2 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL2_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL2_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL2_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL2_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL2_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL2_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL2_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL2_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL2_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL2_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL2_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL2_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL2_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL2_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL2_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL2_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL2_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL2_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL2_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL2_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL2_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL2_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL2_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL2_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL2_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL2_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL2_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL2_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL2_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL2_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL2 {
    #[inline(always)]
    fn default() -> ENDPTCTRL2 {
        ENDPTCTRL2(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL2")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL2 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL3(pub u32);
impl ENDPTCTRL3 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL3_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL3_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL3_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL3_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL3_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL3_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL3_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL3_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL3_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL3_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL3_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL3_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL3_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL3_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL3_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL3_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL3_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL3_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL3_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL3_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL3_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL3_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL3_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL3_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL3_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL3_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL3_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL3_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL3_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL3_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL3 {
    #[inline(always)]
    fn default() -> ENDPTCTRL3 {
        ENDPTCTRL3(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL3")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL3 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL4(pub u32);
impl ENDPTCTRL4 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL4_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL4_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL4_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL4_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL4_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL4_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL4_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL4_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL4_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL4_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL4_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL4_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL4_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL4_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL4_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL4_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL4_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL4_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL4_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL4_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL4_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL4_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL4_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL4_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL4_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL4_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL4_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL4_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL4_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL4_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL4 {
    #[inline(always)]
    fn default() -> ENDPTCTRL4 {
        ENDPTCTRL4(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL4")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL4 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 5."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL5(pub u32);
impl ENDPTCTRL5 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL5_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL5_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL5_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL5_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL5_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL5_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL5_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL5_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL5_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL5_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL5_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL5_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL5_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL5_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL5_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL5_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL5_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL5_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL5_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL5_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL5_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL5_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL5_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL5_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL5_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL5_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL5_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL5_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL5_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL5_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL5 {
    #[inline(always)]
    fn default() -> ENDPTCTRL5 {
        ENDPTCTRL5(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL5")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL5 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 6."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL6(pub u32);
impl ENDPTCTRL6 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL6_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL6_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL6_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL6_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL6_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL6_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL6_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL6_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL6_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL6_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL6_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL6_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL6_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL6_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL6_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL6_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL6_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL6_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL6_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL6_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL6_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL6_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL6_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL6_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL6_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL6_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL6_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL6_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL6_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL6_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL6 {
    #[inline(always)]
    fn default() -> ENDPTCTRL6 {
        ENDPTCTRL6(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL6")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL6 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Control 7."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTCTRL7(pub u32);
impl ENDPTCTRL7 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn RXS(&self) -> ENDPTCTRL7_RXS {
        let val = (self.0 >> 0usize) & 0x01;
        ENDPTCTRL7_RXS::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_RXS(&mut self, val: ENDPTCTRL7_RXS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn RXD(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_RXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn RXT(&self) -> ENDPTCTRL7_RXT {
        let val = (self.0 >> 2usize) & 0x03;
        ENDPTCTRL7_RXT::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_RXT(&mut self, val: ENDPTCTRL7_RXT) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn RXI(&self) -> ENDPTCTRL7_RXI {
        let val = (self.0 >> 5usize) & 0x01;
        ENDPTCTRL7_RXI::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_RXI(&mut self, val: ENDPTCTRL7_RXI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RXR(&self) -> ENDPTCTRL7_RXR {
        let val = (self.0 >> 6usize) & 0x01;
        ENDPTCTRL7_RXR::from_bits(val as u8)
    }
    #[doc = "RX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_RXR(&mut self, val: ENDPTCTRL7_RXR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RXE(&self) -> ENDPTCTRL7_RXE {
        let val = (self.0 >> 7usize) & 0x01;
        ENDPTCTRL7_RXE::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_RXE(&mut self, val: ENDPTCTRL7_RXE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn TXS(&self) -> ENDPTCTRL7_TXS {
        let val = (self.0 >> 16usize) & 0x01;
        ENDPTCTRL7_TXS::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_TXS(&mut self, val: ENDPTCTRL7_TXS) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn TXD(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_TXD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn TXT(&self) -> ENDPTCTRL7_TXT {
        let val = (self.0 >> 18usize) & 0x03;
        ENDPTCTRL7_TXT::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_TXT(&mut self, val: ENDPTCTRL7_TXT) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn TXI(&self) -> ENDPTCTRL7_TXI {
        let val = (self.0 >> 21usize) & 0x01;
        ENDPTCTRL7_TXI::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_TXI(&mut self, val: ENDPTCTRL7_TXI) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn TXR(&self) -> ENDPTCTRL7_TXR {
        let val = (self.0 >> 22usize) & 0x01;
        ENDPTCTRL7_TXR::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Reset."]
    #[inline(always)]
    pub const fn set_TXR(&mut self, val: ENDPTCTRL7_TXR) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TXE(&self) -> ENDPTCTRL7_TXE {
        let val = (self.0 >> 23usize) & 0x01;
        ENDPTCTRL7_TXE::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_TXE(&mut self, val: ENDPTCTRL7_TXE) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for ENDPTCTRL7 {
    #[inline(always)]
    fn default() -> ENDPTCTRL7 {
        ENDPTCTRL7(0)
    }
}
impl core::fmt::Debug for ENDPTCTRL7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTCTRL7")
            .field("RXS", &self.RXS())
            .field("RXD", &self.RXD())
            .field("RXT", &self.RXT())
            .field("RXI", &self.RXI())
            .field("RXR", &self.RXR())
            .field("RXE", &self.RXE())
            .field("TXS", &self.TXS())
            .field("TXD", &self.TXD())
            .field("TXT", &self.TXT())
            .field("TXI", &self.TXI())
            .field("TXR", &self.TXR())
            .field("TXE", &self.TXE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTCTRL7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTCTRL7 {{ RXS: {:?}, RXD: {=bool:?}, RXT: {:?}, RXI: {:?}, RXR: {:?}, RXE: {:?}, TXS: {:?}, TXD: {=bool:?}, TXT: {:?}, TXI: {:?}, TXR: {:?}, TXE: {:?} }}",
            self.RXS(),
            self.RXD(),
            self.RXT(),
            self.RXI(),
            self.RXR(),
            self.RXE(),
            self.TXS(),
            self.TXD(),
            self.TXT(),
            self.TXI(),
            self.TXR(),
            self.TXE()
        )
    }
}
#[doc = "Endpoint Flush."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTFLUSH(pub u32);
impl ENDPTFLUSH {
    #[doc = "Flush Endpoint Receive Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn FERB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Receive Buffer."]
    #[inline(always)]
    pub const fn set_FERB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Flush Endpoint Transmit Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn FETB(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Transmit Buffer."]
    #[inline(always)]
    pub const fn set_FETB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTFLUSH {
    #[inline(always)]
    fn default() -> ENDPTFLUSH {
        ENDPTFLUSH(0)
    }
}
impl core::fmt::Debug for ENDPTFLUSH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTFLUSH")
            .field("FERB", &self.FERB())
            .field("FETB", &self.FETB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTFLUSH {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTFLUSH {{ FERB: {=u8:?}, FETB: {=u8:?} }}",
            self.FERB(),
            self.FETB()
        )
    }
}
#[doc = "Endpoint List Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTLISTADDR(pub u32);
impl ENDPTLISTADDR {
    #[doc = "Endpoint List Pointer (Low)."]
    #[must_use]
    #[inline(always)]
    pub const fn EPBASE(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Endpoint List Pointer (Low)."]
    #[inline(always)]
    pub const fn set_EPBASE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for ENDPTLISTADDR {
    #[inline(always)]
    fn default() -> ENDPTLISTADDR {
        ENDPTLISTADDR(0)
    }
}
impl core::fmt::Debug for ENDPTLISTADDR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTLISTADDR")
            .field("EPBASE", &self.EPBASE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTLISTADDR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ENDPTLISTADDR {{ EPBASE: {=u32:?} }}", self.EPBASE())
    }
}
#[doc = "Endpoint NAK."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTNAK(pub u32);
impl ENDPTNAK {
    #[doc = "RX Endpoint NAK Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn EPRN(&self) -> EPRN {
        let val = (self.0 >> 0usize) & 0xff;
        EPRN::from_bits(val as u8)
    }
    #[doc = "RX Endpoint NAK Flag."]
    #[inline(always)]
    pub const fn set_EPRN(&mut self, val: EPRN) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn EPTN(&self) -> EPTN {
        let val = (self.0 >> 16usize) & 0xff;
        EPTN::from_bits(val as u8)
    }
    #[doc = "TX Endpoint NAK Flag."]
    #[inline(always)]
    pub const fn set_EPTN(&mut self, val: EPTN) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTNAK {
    #[inline(always)]
    fn default() -> ENDPTNAK {
        ENDPTNAK(0)
    }
}
impl core::fmt::Debug for ENDPTNAK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTNAK")
            .field("EPRN", &self.EPRN())
            .field("EPTN", &self.EPTN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTNAK {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTNAK {{ EPRN: {:?}, EPTN: {:?} }}",
            self.EPRN(),
            self.EPTN()
        )
    }
}
#[doc = "Endpoint NAK Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTNAKEN(pub u32);
impl ENDPTNAKEN {
    #[doc = "RX Endpoint NAK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EPRNE(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn set_EPRNE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EPTNE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn set_EPTNE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTNAKEN {
    #[inline(always)]
    fn default() -> ENDPTNAKEN {
        ENDPTNAKEN(0)
    }
}
impl core::fmt::Debug for ENDPTNAKEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTNAKEN")
            .field("EPRNE", &self.EPRNE())
            .field("EPTNE", &self.EPTNE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTNAKEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTNAKEN {{ EPRNE: {=u8:?}, EPTNE: {=u8:?} }}",
            self.EPRNE(),
            self.EPTNE()
        )
    }
}
#[doc = "Endpoint Prime."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTPRIME(pub u32);
impl ENDPTPRIME {
    #[doc = "Prime Endpoint Receive Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn PERB(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Receive Buffer."]
    #[inline(always)]
    pub const fn set_PERB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Prime Endpoint Transmit Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn PETB(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Transmit Buffer."]
    #[inline(always)]
    pub const fn set_PETB(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTPRIME {
    #[inline(always)]
    fn default() -> ENDPTPRIME {
        ENDPTPRIME(0)
    }
}
impl core::fmt::Debug for ENDPTPRIME {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTPRIME")
            .field("PERB", &self.PERB())
            .field("PETB", &self.PETB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTPRIME {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTPRIME {{ PERB: {=u8:?}, PETB: {=u8:?} }}",
            self.PERB(),
            self.PETB()
        )
    }
}
#[doc = "Endpoint Setup Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTSETUPSTAT(pub u32);
impl ENDPTSETUPSTAT {
    #[doc = "Endpoint Setup Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ENDPTSETUPSTAT(&self) -> ENDPTSETUPSTAT_FLAG {
        let val = (self.0 >> 0usize) & 0xffff;
        ENDPTSETUPSTAT_FLAG::from_bits(val as u16)
    }
    #[doc = "Endpoint Setup Status Flag."]
    #[inline(always)]
    pub const fn set_ENDPTSETUPSTAT(&mut self, val: ENDPTSETUPSTAT_FLAG) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for ENDPTSETUPSTAT {
    #[inline(always)]
    fn default() -> ENDPTSETUPSTAT {
        ENDPTSETUPSTAT(0)
    }
}
impl core::fmt::Debug for ENDPTSETUPSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTSETUPSTAT")
            .field("ENDPTSETUPSTAT", &self.ENDPTSETUPSTAT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTSETUPSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTSETUPSTAT {{ ENDPTSETUPSTAT: {:?} }}",
            self.ENDPTSETUPSTAT()
        )
    }
}
#[doc = "Endpoint Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ENDPTSTAT(pub u32);
impl ENDPTSTAT {
    #[doc = "Endpoint Receive Buffer Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ERBR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Buffer Ready."]
    #[inline(always)]
    pub const fn set_ERBR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Buffer Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn ETBR(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Buffer Ready."]
    #[inline(always)]
    pub const fn set_ETBR(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ENDPTSTAT {
    #[inline(always)]
    fn default() -> ENDPTSTAT {
        ENDPTSTAT(0)
    }
}
impl core::fmt::Debug for ENDPTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDPTSTAT")
            .field("ERBR", &self.ERBR())
            .field("ETBR", &self.ETBR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ENDPTSTAT {{ ERBR: {=u8:?}, ETBR: {=u8:?} }}",
            self.ERBR(),
            self.ETBR()
        )
    }
}
#[doc = "USB Frame Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FRINDEX(pub u32);
impl FRINDEX {
    #[doc = "Frame Index."]
    #[must_use]
    #[inline(always)]
    pub const fn FRINDEX(&self) -> FRINDEX_VALUE {
        let val = (self.0 >> 0usize) & 0x3fff;
        FRINDEX_VALUE::from_bits(val as u16)
    }
    #[doc = "Frame Index."]
    #[inline(always)]
    pub const fn set_FRINDEX(&mut self, val: FRINDEX_VALUE) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val.to_bits() as u32) & 0x3fff) << 0usize);
    }
}
impl Default for FRINDEX {
    #[inline(always)]
    fn default() -> FRINDEX {
        FRINDEX(0)
    }
}
impl core::fmt::Debug for FRINDEX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRINDEX")
            .field("FRINDEX", &self.FRINDEX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRINDEX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FRINDEX {{ FRINDEX: {:?} }}", self.FRINDEX())
    }
}
#[doc = "General Purpose Timer 0 Controller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTIMER0CTRL(pub u32);
impl GPTIMER0CTRL {
    #[doc = "General Purpose Timer Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTCNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter."]
    #[inline(always)]
    pub const fn set_GPTCNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTMODE(&self) -> GPTIMER0CTRL_GPTMODE {
        let val = (self.0 >> 24usize) & 0x01;
        GPTIMER0CTRL_GPTMODE::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode."]
    #[inline(always)]
    pub const fn set_GPTMODE(&mut self, val: GPTIMER0CTRL_GPTMODE) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTRST(&self) -> GPTIMER0CTRL_GPTRST {
        let val = (self.0 >> 30usize) & 0x01;
        GPTIMER0CTRL_GPTRST::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset."]
    #[inline(always)]
    pub const fn set_GPTRST(&mut self, val: GPTIMER0CTRL_GPTRST) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTRUN(&self) -> GPTIMER0CTRL_GPTRUN {
        let val = (self.0 >> 31usize) & 0x01;
        GPTIMER0CTRL_GPTRUN::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run."]
    #[inline(always)]
    pub const fn set_GPTRUN(&mut self, val: GPTIMER0CTRL_GPTRUN) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for GPTIMER0CTRL {
    #[inline(always)]
    fn default() -> GPTIMER0CTRL {
        GPTIMER0CTRL(0)
    }
}
impl core::fmt::Debug for GPTIMER0CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMER0CTRL")
            .field("GPTCNT", &self.GPTCNT())
            .field("GPTMODE", &self.GPTMODE())
            .field("GPTRST", &self.GPTRST())
            .field("GPTRUN", &self.GPTRUN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTIMER0CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTIMER0CTRL {{ GPTCNT: {=u32:?}, GPTMODE: {:?}, GPTRST: {:?}, GPTRUN: {:?} }}",
            self.GPTCNT(),
            self.GPTMODE(),
            self.GPTRST(),
            self.GPTRUN()
        )
    }
}
#[doc = "General Purpose Timer 0 Load."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTIMER0LD(pub u32);
impl GPTIMER0LD {
    #[doc = "General Purpose Timer Load Value."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTLD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value."]
    #[inline(always)]
    pub const fn set_GPTLD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPTIMER0LD {
    #[inline(always)]
    fn default() -> GPTIMER0LD {
        GPTIMER0LD(0)
    }
}
impl core::fmt::Debug for GPTIMER0LD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMER0LD")
            .field("GPTLD", &self.GPTLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTIMER0LD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPTIMER0LD {{ GPTLD: {=u32:?} }}", self.GPTLD())
    }
}
#[doc = "General Purpose Timer 1 Controller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTIMER1CTRL(pub u32);
impl GPTIMER1CTRL {
    #[doc = "General Purpose Timer Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTCNT(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter."]
    #[inline(always)]
    pub const fn set_GPTCNT(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTMODE(&self) -> GPTIMER1CTRL_GPTMODE {
        let val = (self.0 >> 24usize) & 0x01;
        GPTIMER1CTRL_GPTMODE::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode."]
    #[inline(always)]
    pub const fn set_GPTMODE(&mut self, val: GPTIMER1CTRL_GPTMODE) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTRST(&self) -> GPTIMER1CTRL_GPTRST {
        let val = (self.0 >> 30usize) & 0x01;
        GPTIMER1CTRL_GPTRST::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset."]
    #[inline(always)]
    pub const fn set_GPTRST(&mut self, val: GPTIMER1CTRL_GPTRST) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTRUN(&self) -> GPTIMER1CTRL_GPTRUN {
        let val = (self.0 >> 31usize) & 0x01;
        GPTIMER1CTRL_GPTRUN::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run."]
    #[inline(always)]
    pub const fn set_GPTRUN(&mut self, val: GPTIMER1CTRL_GPTRUN) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for GPTIMER1CTRL {
    #[inline(always)]
    fn default() -> GPTIMER1CTRL {
        GPTIMER1CTRL(0)
    }
}
impl core::fmt::Debug for GPTIMER1CTRL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMER1CTRL")
            .field("GPTCNT", &self.GPTCNT())
            .field("GPTMODE", &self.GPTMODE())
            .field("GPTRST", &self.GPTRST())
            .field("GPTRUN", &self.GPTRUN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTIMER1CTRL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GPTIMER1CTRL {{ GPTCNT: {=u32:?}, GPTMODE: {:?}, GPTRST: {:?}, GPTRUN: {:?} }}",
            self.GPTCNT(),
            self.GPTMODE(),
            self.GPTRST(),
            self.GPTRUN()
        )
    }
}
#[doc = "General Purpose Timer 1 Load."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GPTIMER1LD(pub u32);
impl GPTIMER1LD {
    #[doc = "General Purpose Timer Load Value."]
    #[must_use]
    #[inline(always)]
    pub const fn GPTLD(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value."]
    #[inline(always)]
    pub const fn set_GPTLD(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for GPTIMER1LD {
    #[inline(always)]
    fn default() -> GPTIMER1LD {
        GPTIMER1LD(0)
    }
}
impl core::fmt::Debug for GPTIMER1LD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPTIMER1LD")
            .field("GPTLD", &self.GPTLD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GPTIMER1LD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GPTIMER1LD {{ GPTLD: {=u32:?} }}", self.GPTLD())
    }
}
#[doc = "Host Controller Capability Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCCPARAMS(pub u32);
impl HCCPARAMS {
    #[doc = "Addressing Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn ADC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Addressing Capability."]
    #[inline(always)]
    pub const fn set_ADC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Programmable Frame List Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn PFL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Frame List Flag."]
    #[inline(always)]
    pub const fn set_PFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous Schedule Park Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn ASP(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Capability."]
    #[inline(always)]
    pub const fn set_ASP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Isochronous Scheduling Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn IST(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Isochronous Scheduling Threshold."]
    #[inline(always)]
    pub const fn set_IST(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EHCI Extended Capabilities Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn EECP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "EHCI Extended Capabilities Pointer."]
    #[inline(always)]
    pub const fn set_EECP(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for HCCPARAMS {
    #[inline(always)]
    fn default() -> HCCPARAMS {
        HCCPARAMS(0)
    }
}
impl core::fmt::Debug for HCCPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCPARAMS")
            .field("ADC", &self.ADC())
            .field("PFL", &self.PFL())
            .field("ASP", &self.ASP())
            .field("IST", &self.IST())
            .field("EECP", &self.EECP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCCPARAMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCCPARAMS {{ ADC: {=bool:?}, PFL: {=bool:?}, ASP: {=bool:?}, IST: {=u8:?}, EECP: {=u8:?} }}",
            self.ADC(),
            self.PFL(),
            self.ASP(),
            self.IST(),
            self.EECP()
        )
    }
}
#[doc = "Host Controller Interface Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCIVERSION(pub u16);
impl HCIVERSION {
    #[doc = "Host Controller Interface Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn HCIVERSION(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Host Controller Interface Version Number."]
    #[inline(always)]
    pub const fn set_HCIVERSION(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for HCIVERSION {
    #[inline(always)]
    fn default() -> HCIVERSION {
        HCIVERSION(0)
    }
}
impl core::fmt::Debug for HCIVERSION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCIVERSION")
            .field("HCIVERSION", &self.HCIVERSION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCIVERSION {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCIVERSION {{ HCIVERSION: {=u16:?} }}",
            self.HCIVERSION()
        )
    }
}
#[doc = "Host Controller Structural Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HCSPARAMS(pub u32);
impl HCSPARAMS {
    #[doc = "Number of Ports."]
    #[must_use]
    #[inline(always)]
    pub const fn N_PORTS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports."]
    #[inline(always)]
    pub const fn set_N_PORTS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PPC(&self) -> PPC {
        let val = (self.0 >> 4usize) & 0x01;
        PPC::from_bits(val as u8)
    }
    #[doc = "Port Power Control."]
    #[inline(always)]
    pub const fn set_PPC(&mut self, val: PPC) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Ports per Companion Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn N_PCC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Companion Controller."]
    #[inline(always)]
    pub const fn set_N_PCC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of Companion Controller (N_CC)."]
    #[must_use]
    #[inline(always)]
    pub const fn N_CC(&self) -> N_CC {
        let val = (self.0 >> 12usize) & 0x0f;
        N_CC::from_bits(val as u8)
    }
    #[doc = "Number of Companion Controller (N_CC)."]
    #[inline(always)]
    pub const fn set_N_CC(&mut self, val: N_CC) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Port Indicators (P_INDICATOR)."]
    #[must_use]
    #[inline(always)]
    pub const fn PI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Indicators (P_INDICATOR)."]
    #[inline(always)]
    pub const fn set_PI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Number of Ports per Transaction Translator (N_PTT)."]
    #[must_use]
    #[inline(always)]
    pub const fn N_PTT(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Transaction Translator (N_PTT)."]
    #[inline(always)]
    pub const fn set_N_PTT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of Transaction Translators (N_TT)."]
    #[must_use]
    #[inline(always)]
    pub const fn N_TT(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Transaction Translators (N_TT)."]
    #[inline(always)]
    pub const fn set_N_TT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for HCSPARAMS {
    #[inline(always)]
    fn default() -> HCSPARAMS {
        HCSPARAMS(0)
    }
}
impl core::fmt::Debug for HCSPARAMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCSPARAMS")
            .field("N_PORTS", &self.N_PORTS())
            .field("PPC", &self.PPC())
            .field("N_PCC", &self.N_PCC())
            .field("N_CC", &self.N_CC())
            .field("PI", &self.PI())
            .field("N_PTT", &self.N_PTT())
            .field("N_TT", &self.N_TT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HCSPARAMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HCSPARAMS {{ N_PORTS: {=u8:?}, PPC: {:?}, N_PCC: {=u8:?}, N_CC: {:?}, PI: {=bool:?}, N_PTT: {=u8:?}, N_TT: {=u8:?} }}",
            self.N_PORTS(),
            self.PPC(),
            self.N_PCC(),
            self.N_CC(),
            self.PI(),
            self.N_PTT(),
            self.N_TT()
        )
    }
}
#[doc = "Device Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWDEVICE(pub u32);
impl HWDEVICE {
    #[doc = "Device Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn DC(&self) -> HWDEVICE_DC {
        let val = (self.0 >> 0usize) & 0x01;
        HWDEVICE_DC::from_bits(val as u8)
    }
    #[doc = "Device Capable."]
    #[inline(always)]
    pub const fn set_DC(&mut self, val: HWDEVICE_DC) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Device Endpoint Number."]
    #[must_use]
    #[inline(always)]
    pub const fn DEVEP(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number."]
    #[inline(always)]
    pub const fn set_DEVEP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
}
impl Default for HWDEVICE {
    #[inline(always)]
    fn default() -> HWDEVICE {
        HWDEVICE(0)
    }
}
impl core::fmt::Debug for HWDEVICE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWDEVICE")
            .field("DC", &self.DC())
            .field("DEVEP", &self.DEVEP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWDEVICE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWDEVICE {{ DC: {:?}, DEVEP: {=u8:?} }}",
            self.DC(),
            self.DEVEP()
        )
    }
}
#[doc = "Hardware General."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWGENERAL(pub u32);
impl HWGENERAL {
    #[doc = "PHY Width."]
    #[must_use]
    #[inline(always)]
    pub const fn PHYW(&self) -> PHYW {
        let val = (self.0 >> 4usize) & 0x03;
        PHYW::from_bits(val as u8)
    }
    #[doc = "PHY Width."]
    #[inline(always)]
    pub const fn set_PHYW(&mut self, val: PHYW) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Transceiver Type."]
    #[must_use]
    #[inline(always)]
    pub const fn PHYM(&self) -> PHYM {
        let val = (self.0 >> 6usize) & 0x0f;
        PHYM::from_bits(val as u8)
    }
    #[doc = "Transceiver Type."]
    #[inline(always)]
    pub const fn set_PHYM(&mut self, val: PHYM) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val.to_bits() as u32) & 0x0f) << 6usize);
    }
    #[doc = "Serial Interface Mode Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn SM(&self) -> SM {
        let val = (self.0 >> 10usize) & 0x03;
        SM::from_bits(val as u8)
    }
    #[doc = "Serial Interface Mode Capability."]
    #[inline(always)]
    pub const fn set_SM(&mut self, val: SM) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Link Power Management Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM(&self) -> LPM {
        let val = (self.0 >> 12usize) & 0x01;
        LPM::from_bits(val as u8)
    }
    #[doc = "Link Power Management Capability."]
    #[inline(always)]
    pub const fn set_LPM(&mut self, val: LPM) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for HWGENERAL {
    #[inline(always)]
    fn default() -> HWGENERAL {
        HWGENERAL(0)
    }
}
impl core::fmt::Debug for HWGENERAL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWGENERAL")
            .field("PHYW", &self.PHYW())
            .field("PHYM", &self.PHYM())
            .field("SM", &self.SM())
            .field("LPM", &self.LPM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWGENERAL {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWGENERAL {{ PHYW: {:?}, PHYM: {:?}, SM: {:?}, LPM: {:?} }}",
            self.PHYW(),
            self.PHYM(),
            self.SM(),
            self.LPM()
        )
    }
}
#[doc = "Host Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWHOST(pub u32);
impl HWHOST {
    #[doc = "Host Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn HC(&self) -> HWHOST_HC {
        let val = (self.0 >> 0usize) & 0x01;
        HWHOST_HC::from_bits(val as u8)
    }
    #[doc = "Host Capable."]
    #[inline(always)]
    pub const fn set_HC(&mut self, val: HWHOST_HC) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Number of Ports."]
    #[must_use]
    #[inline(always)]
    pub const fn NPORT(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "Number of Ports."]
    #[inline(always)]
    pub const fn set_NPORT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
}
impl Default for HWHOST {
    #[inline(always)]
    fn default() -> HWHOST {
        HWHOST(0)
    }
}
impl core::fmt::Debug for HWHOST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWHOST")
            .field("HC", &self.HC())
            .field("NPORT", &self.NPORT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWHOST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWHOST {{ HC: {:?}, NPORT: {=u8:?} }}",
            self.HC(),
            self.NPORT()
        )
    }
}
#[doc = "RX Buffer Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWRXBUF(pub u32);
impl HWRXBUF {
    #[doc = "RX Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn RXBURST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Burst."]
    #[inline(always)]
    pub const fn set_RXBURST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RX Add."]
    #[must_use]
    #[inline(always)]
    pub const fn RXADD(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "RX Add."]
    #[inline(always)]
    pub const fn set_RXADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for HWRXBUF {
    #[inline(always)]
    fn default() -> HWRXBUF {
        HWRXBUF(0)
    }
}
impl core::fmt::Debug for HWRXBUF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWRXBUF")
            .field("RXBURST", &self.RXBURST())
            .field("RXADD", &self.RXADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWRXBUF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWRXBUF {{ RXBURST: {=u8:?}, RXADD: {=u8:?} }}",
            self.RXBURST(),
            self.RXADD()
        )
    }
}
#[doc = "TX Buffer Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HWTXBUF(pub u32);
impl HWTXBUF {
    #[doc = "TX Burst."]
    #[must_use]
    #[inline(always)]
    pub const fn TXBURST(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TX Burst."]
    #[inline(always)]
    pub const fn set_TXBURST(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Channel Add."]
    #[must_use]
    #[inline(always)]
    pub const fn TXCHANADD(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Channel Add."]
    #[inline(always)]
    pub const fn set_TXCHANADD(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for HWTXBUF {
    #[inline(always)]
    fn default() -> HWTXBUF {
        HWTXBUF(0)
    }
}
impl core::fmt::Debug for HWTXBUF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HWTXBUF")
            .field("TXBURST", &self.TXBURST())
            .field("TXCHANADD", &self.TXCHANADD())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HWTXBUF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HWTXBUF {{ TXBURST: {=u8:?}, TXCHANADD: {=u8:?} }}",
            self.TXBURST(),
            self.TXCHANADD()
        )
    }
}
#[doc = "Identification."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ID(pub u32);
impl ID {
    #[doc = "Configuration Number."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Configuration Number."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Complement Version."]
    #[must_use]
    #[inline(always)]
    pub const fn NID(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Complement Version."]
    #[inline(always)]
    pub const fn set_NID(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Revision Number."]
    #[must_use]
    #[inline(always)]
    pub const fn REVISION(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Revision Number."]
    #[inline(always)]
    pub const fn set_REVISION(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ID {
    #[inline(always)]
    fn default() -> ID {
        ID(0)
    }
}
impl core::fmt::Debug for ID {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ID")
            .field("ID", &self.ID())
            .field("NID", &self.NID())
            .field("REVISION", &self.REVISION())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ID {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ID {{ ID: {=u8:?}, NID: {=u8:?}, REVISION: {=u8:?} }}",
            self.ID(),
            self.NID(),
            self.REVISION()
        )
    }
}
#[doc = "On-The-Go Status and Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OTGSC(pub u32);
impl OTGSC {
    #[doc = "VBUS Discharge."]
    #[must_use]
    #[inline(always)]
    pub const fn VD(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge."]
    #[inline(always)]
    pub const fn set_VD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Charge."]
    #[must_use]
    #[inline(always)]
    pub const fn VC(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Charge."]
    #[inline(always)]
    pub const fn set_VC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Hardware Assist Auto Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn HAAR(&self) -> HAAR {
        let val = (self.0 >> 2usize) & 0x01;
        HAAR::from_bits(val as u8)
    }
    #[doc = "Hardware Assist Auto Reset."]
    #[inline(always)]
    pub const fn set_HAAR(&mut self, val: HAAR) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "OTG Termination."]
    #[must_use]
    #[inline(always)]
    pub const fn OT(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Termination."]
    #[inline(always)]
    pub const fn set_OT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Pulsing."]
    #[must_use]
    #[inline(always)]
    pub const fn DP(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulsing."]
    #[inline(always)]
    pub const fn set_DP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID Pullup."]
    #[must_use]
    #[inline(always)]
    pub const fn IDPU(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pullup."]
    #[inline(always)]
    pub const fn set_IDPU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hardware Assist Data Pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn HADP(&self) -> HADP {
        let val = (self.0 >> 6usize) & 0x01;
        HADP::from_bits(val as u8)
    }
    #[doc = "Hardware Assist Data Pulse."]
    #[inline(always)]
    pub const fn set_HADP(&mut self, val: HADP) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Assist B-Disconnect to A-connect."]
    #[must_use]
    #[inline(always)]
    pub const fn HABA(&self) -> HABA {
        let val = (self.0 >> 7usize) & 0x01;
        HABA::from_bits(val as u8)
    }
    #[doc = "Hardware Assist B-Disconnect to A-connect."]
    #[inline(always)]
    pub const fn set_HABA(&mut self, val: HABA) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "USB ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ID(&self) -> OTG_ID {
        let val = (self.0 >> 8usize) & 0x01;
        OTG_ID::from_bits(val as u8)
    }
    #[doc = "USB ID."]
    #[inline(always)]
    pub const fn set_ID(&mut self, val: OTG_ID) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "A VBUS Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn AVV(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A VBUS Valid."]
    #[inline(always)]
    pub const fn set_AVV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "A Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn ASV(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid."]
    #[inline(always)]
    pub const fn set_ASV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "B Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn BSV(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid."]
    #[inline(always)]
    pub const fn set_BSV(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "B Session End."]
    #[must_use]
    #[inline(always)]
    pub const fn BSE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End."]
    #[inline(always)]
    pub const fn set_BSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "1 Millisecond Timer Toggle."]
    #[must_use]
    #[inline(always)]
    pub const fn TOG_1MS(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Toggle."]
    #[inline(always)]
    pub const fn set_TOG_1MS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Bus Pulsing Status."]
    #[must_use]
    #[inline(always)]
    pub const fn DPS(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Bus Pulsing Status."]
    #[inline(always)]
    pub const fn set_DPS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB ID Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn IDIS(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Status."]
    #[inline(always)]
    pub const fn set_IDIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "A VBUS Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AVVIS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A VBUS Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_AVVIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A Session Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ASVIS(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_ASVIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "B Session Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BSVIS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_BSVIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "B Session End Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BSEIS(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Status."]
    #[inline(always)]
    pub const fn set_BSEIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn STATUS_1MS(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Status."]
    #[inline(always)]
    pub const fn set_STATUS_1MS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Pulse Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn DPIS(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Status."]
    #[inline(always)]
    pub const fn set_DPIS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "USB ID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn IDIE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_IDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "A VBUS Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AVVIE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "A VBUS Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_AVVIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "A Session Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ASVIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ASVIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "B Session Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BSVIE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BSVIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "B Session End Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BSEIE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Enable."]
    #[inline(always)]
    pub const fn set_BSEIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EN_1MS(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Enable."]
    #[inline(always)]
    pub const fn set_EN_1MS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Data Pulse Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DPIE(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Enable."]
    #[inline(always)]
    pub const fn set_DPIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for OTGSC {
    #[inline(always)]
    fn default() -> OTGSC {
        OTGSC(0)
    }
}
impl core::fmt::Debug for OTGSC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTGSC")
            .field("VD", &self.VD())
            .field("VC", &self.VC())
            .field("HAAR", &self.HAAR())
            .field("OT", &self.OT())
            .field("DP", &self.DP())
            .field("IDPU", &self.IDPU())
            .field("HADP", &self.HADP())
            .field("HABA", &self.HABA())
            .field("ID", &self.ID())
            .field("AVV", &self.AVV())
            .field("ASV", &self.ASV())
            .field("BSV", &self.BSV())
            .field("BSE", &self.BSE())
            .field("TOG_1MS", &self.TOG_1MS())
            .field("DPS", &self.DPS())
            .field("IDIS", &self.IDIS())
            .field("AVVIS", &self.AVVIS())
            .field("ASVIS", &self.ASVIS())
            .field("BSVIS", &self.BSVIS())
            .field("BSEIS", &self.BSEIS())
            .field("STATUS_1MS", &self.STATUS_1MS())
            .field("DPIS", &self.DPIS())
            .field("IDIE", &self.IDIE())
            .field("AVVIE", &self.AVVIE())
            .field("ASVIE", &self.ASVIE())
            .field("BSVIE", &self.BSVIE())
            .field("BSEIE", &self.BSEIE())
            .field("EN_1MS", &self.EN_1MS())
            .field("DPIE", &self.DPIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OTGSC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OTGSC {{ VD: {=bool:?}, VC: {=bool:?}, HAAR: {:?}, OT: {=bool:?}, DP: {=bool:?}, IDPU: {=bool:?}, HADP: {:?}, HABA: {:?}, ID: {:?}, AVV: {=bool:?}, ASV: {=bool:?}, BSV: {=bool:?}, BSE: {=bool:?}, TOG_1MS: {=bool:?}, DPS: {=bool:?}, IDIS: {=bool:?}, AVVIS: {=bool:?}, ASVIS: {=bool:?}, BSVIS: {=bool:?}, BSEIS: {=bool:?}, STATUS_1MS: {=bool:?}, DPIS: {=bool:?}, IDIE: {=bool:?}, AVVIE: {=bool:?}, ASVIE: {=bool:?}, BSVIE: {=bool:?}, BSEIE: {=bool:?}, EN_1MS: {=bool:?}, DPIE: {=bool:?} }}",
            self.VD(),
            self.VC(),
            self.HAAR(),
            self.OT(),
            self.DP(),
            self.IDPU(),
            self.HADP(),
            self.HABA(),
            self.ID(),
            self.AVV(),
            self.ASV(),
            self.BSV(),
            self.BSE(),
            self.TOG_1MS(),
            self.DPS(),
            self.IDIS(),
            self.AVVIS(),
            self.ASVIS(),
            self.BSVIS(),
            self.BSEIS(),
            self.STATUS_1MS(),
            self.DPIS(),
            self.IDIE(),
            self.AVVIE(),
            self.ASVIE(),
            self.BSVIE(),
            self.BSEIE(),
            self.EN_1MS(),
            self.DPIE()
        )
    }
}
#[doc = "Frame List Base Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PERIODICLISTBASE(pub u32);
impl PERIODICLISTBASE {
    #[doc = "Base Address (Low)."]
    #[must_use]
    #[inline(always)]
    pub const fn BASEADR(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address (Low)."]
    #[inline(always)]
    pub const fn set_BASEADR(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for PERIODICLISTBASE {
    #[inline(always)]
    fn default() -> PERIODICLISTBASE {
        PERIODICLISTBASE(0)
    }
}
impl core::fmt::Debug for PERIODICLISTBASE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIODICLISTBASE")
            .field("BASEADR", &self.BASEADR())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PERIODICLISTBASE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PERIODICLISTBASE {{ BASEADR: {=u32:?} }}",
            self.BASEADR()
        )
    }
}
#[doc = "Port Status and Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PORTSC1(pub u32);
impl PORTSC1 {
    #[doc = "Current Connect Status."]
    #[must_use]
    #[inline(always)]
    pub const fn CCS(&self) -> CCS {
        let val = (self.0 >> 0usize) & 0x01;
        CCS::from_bits(val as u8)
    }
    #[doc = "Current Connect Status."]
    #[inline(always)]
    pub const fn set_CCS(&mut self, val: CCS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn CSC(&self) -> CSC {
        let val = (self.0 >> 1usize) & 0x01;
        CSC::from_bits(val as u8)
    }
    #[doc = "Connect Status Change Flag."]
    #[inline(always)]
    pub const fn set_CSC(&mut self, val: CSC) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enable and Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn PE(&self) -> PE {
        let val = (self.0 >> 2usize) & 0x01;
        PE::from_bits(val as u8)
    }
    #[doc = "Port Enable and Disable."]
    #[inline(always)]
    pub const fn set_PE(&mut self, val: PE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable and Disable Change Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn PEC(&self) -> PEC {
        let val = (self.0 >> 3usize) & 0x01;
        PEC::from_bits(val as u8)
    }
    #[doc = "Port Enable and Disable Change Flag."]
    #[inline(always)]
    pub const fn set_PEC(&mut self, val: PEC) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Overcurrent Active."]
    #[must_use]
    #[inline(always)]
    pub const fn OCA(&self) -> OCA {
        let val = (self.0 >> 4usize) & 0x01;
        OCA::from_bits(val as u8)
    }
    #[doc = "Overcurrent Active."]
    #[inline(always)]
    pub const fn set_OCA(&mut self, val: OCA) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Overcurrent Change Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn OCC(&self) -> OCC {
        let val = (self.0 >> 5usize) & 0x01;
        OCC::from_bits(val as u8)
    }
    #[doc = "Overcurrent Change Flag."]
    #[inline(always)]
    pub const fn set_OCC(&mut self, val: OCC) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn FPR(&self) -> FPR {
        let val = (self.0 >> 6usize) & 0x01;
        FPR::from_bits(val as u8)
    }
    #[doc = "Force Port Resume."]
    #[inline(always)]
    pub const fn set_FPR(&mut self, val: FPR) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn SUSP(&self) -> SUSP {
        let val = (self.0 >> 7usize) & 0x01;
        SUSP::from_bits(val as u8)
    }
    #[doc = "Suspend."]
    #[inline(always)]
    pub const fn set_SUSP(&mut self, val: SUSP) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn PR(&self) -> PR {
        let val = (self.0 >> 8usize) & 0x01;
        PR::from_bits(val as u8)
    }
    #[doc = "Port Reset."]
    #[inline(always)]
    pub const fn set_PR(&mut self, val: PR) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "High-Speed Port."]
    #[must_use]
    #[inline(always)]
    pub const fn HSP(&self) -> HSP {
        let val = (self.0 >> 9usize) & 0x01;
        HSP::from_bits(val as u8)
    }
    #[doc = "High-Speed Port."]
    #[inline(always)]
    pub const fn set_HSP(&mut self, val: HSP) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Line Status."]
    #[must_use]
    #[inline(always)]
    pub const fn LS(&self) -> LS {
        let val = (self.0 >> 10usize) & 0x03;
        LS::from_bits(val as u8)
    }
    #[doc = "Line Status."]
    #[inline(always)]
    pub const fn set_LS(&mut self, val: LS) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power (PP)."]
    #[must_use]
    #[inline(always)]
    pub const fn PP(&self) -> PP {
        let val = (self.0 >> 12usize) & 0x01;
        PP::from_bits(val as u8)
    }
    #[doc = "Port Power (PP)."]
    #[inline(always)]
    pub const fn set_PP(&mut self, val: PP) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Owner."]
    #[must_use]
    #[inline(always)]
    pub const fn PO(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Owner."]
    #[inline(always)]
    pub const fn set_PO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Indicator Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PIC(&self) -> PIC {
        let val = (self.0 >> 14usize) & 0x03;
        PIC::from_bits(val as u8)
    }
    #[doc = "Port Indicator Control."]
    #[inline(always)]
    pub const fn set_PIC(&mut self, val: PIC) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control."]
    #[must_use]
    #[inline(always)]
    pub const fn PTC(&self) -> PTC {
        let val = (self.0 >> 16usize) & 0x0f;
        PTC::from_bits(val as u8)
    }
    #[doc = "Port Test Control."]
    #[inline(always)]
    pub const fn set_PTC(&mut self, val: PTC) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)."]
    #[must_use]
    #[inline(always)]
    pub const fn WKCN(&self) -> WKCN {
        let val = (self.0 >> 20usize) & 0x01;
        WKCN::from_bits(val as u8)
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)."]
    #[inline(always)]
    pub const fn set_WKCN(&mut self, val: WKCN) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)."]
    #[must_use]
    #[inline(always)]
    pub const fn WKDC(&self) -> WKDC {
        let val = (self.0 >> 21usize) & 0x01;
        WKDC::from_bits(val as u8)
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)."]
    #[inline(always)]
    pub const fn set_WKDC(&mut self, val: WKDC) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake on Overcurrent Enable (WKOC)."]
    #[must_use]
    #[inline(always)]
    pub const fn WKOC(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Overcurrent Enable (WKOC)."]
    #[inline(always)]
    pub const fn set_WKOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PHY Low-Power Suspend - Clock Disable (PLPSCD)."]
    #[must_use]
    #[inline(always)]
    pub const fn PHCD(&self) -> PHCD {
        let val = (self.0 >> 23usize) & 0x01;
        PHCD::from_bits(val as u8)
    }
    #[doc = "PHY Low-Power Suspend - Clock Disable (PLPSCD)."]
    #[inline(always)]
    pub const fn set_PHCD(&mut self, val: PHCD) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Force Full Speed Connect."]
    #[must_use]
    #[inline(always)]
    pub const fn PFSC(&self) -> PFSC {
        let val = (self.0 >> 24usize) & 0x01;
        PFSC::from_bits(val as u8)
    }
    #[doc = "Port Force Full Speed Connect."]
    #[inline(always)]
    pub const fn set_PFSC(&mut self, val: PFSC) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Parallel Transceiver Select 2."]
    #[must_use]
    #[inline(always)]
    pub const fn PTS_2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Transceiver Select 2."]
    #[inline(always)]
    pub const fn set_PTS_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn PSPD(&self) -> PSPD {
        let val = (self.0 >> 26usize) & 0x03;
        PSPD::from_bits(val as u8)
    }
    #[doc = "Port Speed."]
    #[inline(always)]
    pub const fn set_PSPD(&mut self, val: PSPD) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Parallel Transceiver Width."]
    #[must_use]
    #[inline(always)]
    pub const fn PTW(&self) -> PTW {
        let val = (self.0 >> 28usize) & 0x01;
        PTW::from_bits(val as u8)
    }
    #[doc = "Parallel Transceiver Width."]
    #[inline(always)]
    pub const fn set_PTW(&mut self, val: PTW) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Serial Transceiver Select."]
    #[must_use]
    #[inline(always)]
    pub const fn STS(&self) -> STS {
        let val = (self.0 >> 29usize) & 0x01;
        STS::from_bits(val as u8)
    }
    #[doc = "Serial Transceiver Select."]
    #[inline(always)]
    pub const fn set_STS(&mut self, val: STS) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Parallel Transceiver Select 1."]
    #[must_use]
    #[inline(always)]
    pub const fn PTS_1(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Parallel Transceiver Select 1."]
    #[inline(always)]
    pub const fn set_PTS_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for PORTSC1 {
    #[inline(always)]
    fn default() -> PORTSC1 {
        PORTSC1(0)
    }
}
impl core::fmt::Debug for PORTSC1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORTSC1")
            .field("CCS", &self.CCS())
            .field("CSC", &self.CSC())
            .field("PE", &self.PE())
            .field("PEC", &self.PEC())
            .field("OCA", &self.OCA())
            .field("OCC", &self.OCC())
            .field("FPR", &self.FPR())
            .field("SUSP", &self.SUSP())
            .field("PR", &self.PR())
            .field("HSP", &self.HSP())
            .field("LS", &self.LS())
            .field("PP", &self.PP())
            .field("PO", &self.PO())
            .field("PIC", &self.PIC())
            .field("PTC", &self.PTC())
            .field("WKCN", &self.WKCN())
            .field("WKDC", &self.WKDC())
            .field("WKOC", &self.WKOC())
            .field("PHCD", &self.PHCD())
            .field("PFSC", &self.PFSC())
            .field("PTS_2", &self.PTS_2())
            .field("PSPD", &self.PSPD())
            .field("PTW", &self.PTW())
            .field("STS", &self.STS())
            .field("PTS_1", &self.PTS_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PORTSC1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PORTSC1 {{ CCS: {:?}, CSC: {:?}, PE: {:?}, PEC: {:?}, OCA: {:?}, OCC: {:?}, FPR: {:?}, SUSP: {:?}, PR: {:?}, HSP: {:?}, LS: {:?}, PP: {:?}, PO: {=bool:?}, PIC: {:?}, PTC: {:?}, WKCN: {:?}, WKDC: {:?}, WKOC: {=bool:?}, PHCD: {:?}, PFSC: {:?}, PTS_2: {=bool:?}, PSPD: {:?}, PTW: {:?}, STS: {:?}, PTS_1: {=u8:?} }}",
            self.CCS(),
            self.CSC(),
            self.PE(),
            self.PEC(),
            self.OCA(),
            self.OCC(),
            self.FPR(),
            self.SUSP(),
            self.PR(),
            self.HSP(),
            self.LS(),
            self.PP(),
            self.PO(),
            self.PIC(),
            self.PTC(),
            self.WKCN(),
            self.WKDC(),
            self.WKOC(),
            self.PHCD(),
            self.PFSC(),
            self.PTS_2(),
            self.PSPD(),
            self.PTW(),
            self.STS(),
            self.PTS_1()
        )
    }
}
#[doc = "System Bus Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SBUSCFG(pub u32);
impl SBUSCFG {
    #[doc = "AHB Manager Interface Burst Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn AHBBRST(&self) -> AHBBRST {
        let val = (self.0 >> 0usize) & 0x07;
        AHBBRST::from_bits(val as u8)
    }
    #[doc = "AHB Manager Interface Burst Configuration."]
    #[inline(always)]
    pub const fn set_AHBBRST(&mut self, val: AHBBRST) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for SBUSCFG {
    #[inline(always)]
    fn default() -> SBUSCFG {
        SBUSCFG(0)
    }
}
impl core::fmt::Debug for SBUSCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SBUSCFG")
            .field("AHBBRST", &self.AHBBRST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SBUSCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SBUSCFG {{ AHBBRST: {:?} }}", self.AHBBRST())
    }
}
#[doc = "TX FIFO Fill Tuning."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXFILLTUNING(pub u32);
impl TXFILLTUNING {
    #[doc = "Scheduler Overhead."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSCHOH(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Scheduler Overhead."]
    #[inline(always)]
    pub const fn set_TXSCHOH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Scheduler Health Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn TXSCHHEALTH(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Scheduler Health Counter."]
    #[inline(always)]
    pub const fn set_TXSCHHEALTH(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "FIFO Burst Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn TXFIFOTHRES(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "FIFO Burst Threshold."]
    #[inline(always)]
    pub const fn set_TXFIFOTHRES(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for TXFILLTUNING {
    #[inline(always)]
    fn default() -> TXFILLTUNING {
        TXFILLTUNING(0)
    }
}
impl core::fmt::Debug for TXFILLTUNING {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFILLTUNING")
            .field("TXSCHOH", &self.TXSCHOH())
            .field("TXSCHHEALTH", &self.TXSCHHEALTH())
            .field("TXFIFOTHRES", &self.TXFIFOTHRES())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXFILLTUNING {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TXFILLTUNING {{ TXSCHOH: {=u8:?}, TXSCHHEALTH: {=u8:?}, TXFIFOTHRES: {=u8:?} }}",
            self.TXSCHOH(),
            self.TXSCHHEALTH(),
            self.TXFIFOTHRES()
        )
    }
}
#[doc = "USB Command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBCMD(pub u32);
impl USBCMD {
    #[doc = "Run/Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn RS(&self) -> RS {
        let val = (self.0 >> 0usize) & 0x01;
        RS::from_bits(val as u8)
    }
    #[doc = "Run/Stop."]
    #[inline(always)]
    pub const fn set_RS(&mut self, val: RS) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Controller Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn RST(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Reset."]
    #[inline(always)]
    pub const fn set_RST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame List Size 1."]
    #[must_use]
    #[inline(always)]
    pub const fn FS_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Frame List Size 1."]
    #[inline(always)]
    pub const fn set_FS_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Periodic Schedule Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PSE(&self) -> PSE {
        let val = (self.0 >> 4usize) & 0x01;
        PSE::from_bits(val as u8)
    }
    #[doc = "Periodic Schedule Enable."]
    #[inline(always)]
    pub const fn set_PSE(&mut self, val: PSE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Schedule Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ASE(&self) -> ASE {
        let val = (self.0 >> 5usize) & 0x01;
        ASE::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Enable."]
    #[inline(always)]
    pub const fn set_ASE(&mut self, val: ASE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on Async Advance Doorbell."]
    #[must_use]
    #[inline(always)]
    pub const fn IAA(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance Doorbell."]
    #[inline(always)]
    pub const fn set_IAA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Count."]
    #[must_use]
    #[inline(always)]
    pub const fn ASP(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Asynchronous Schedule Park Mode Count."]
    #[inline(always)]
    pub const fn set_ASP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ASPE(&self) -> ASPE {
        let val = (self.0 >> 11usize) & 0x01;
        ASPE::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Park Mode Enable."]
    #[inline(always)]
    pub const fn set_ASPE(&mut self, val: ASPE) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Setup Trip Wire (Device mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn SUTW(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Setup Trip Wire (Device mode only)."]
    #[inline(always)]
    pub const fn set_SUTW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Add dTD Trip Wire (Device mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn ATDTW(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Add dTD Trip Wire (Device mode only)."]
    #[inline(always)]
    pub const fn set_ATDTW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Frame List Size 2 (Host mode only)."]
    #[must_use]
    #[inline(always)]
    pub const fn FS_2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Size 2 (Host mode only)."]
    #[inline(always)]
    pub const fn set_FS_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ITC(&self) -> ITC {
        let val = (self.0 >> 16usize) & 0xff;
        ITC::from_bits(val as u8)
    }
    #[doc = "Interrupt Threshold Control."]
    #[inline(always)]
    pub const fn set_ITC(&mut self, val: ITC) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for USBCMD {
    #[inline(always)]
    fn default() -> USBCMD {
        USBCMD(0)
    }
}
impl core::fmt::Debug for USBCMD {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBCMD")
            .field("RS", &self.RS())
            .field("RST", &self.RST())
            .field("FS_1", &self.FS_1())
            .field("PSE", &self.PSE())
            .field("ASE", &self.ASE())
            .field("IAA", &self.IAA())
            .field("ASP", &self.ASP())
            .field("ASPE", &self.ASPE())
            .field("SUTW", &self.SUTW())
            .field("ATDTW", &self.ATDTW())
            .field("FS_2", &self.FS_2())
            .field("ITC", &self.ITC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBCMD {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBCMD {{ RS: {:?}, RST: {=bool:?}, FS_1: {=u8:?}, PSE: {:?}, ASE: {:?}, IAA: {=bool:?}, ASP: {=u8:?}, ASPE: {:?}, SUTW: {=bool:?}, ATDTW: {=bool:?}, FS_2: {=bool:?}, ITC: {:?} }}",
            self.RS(),
            self.RST(),
            self.FS_1(),
            self.PSE(),
            self.ASE(),
            self.IAA(),
            self.ASP(),
            self.ASPE(),
            self.SUTW(),
            self.ATDTW(),
            self.FS_2(),
            self.ITC()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBINTR(pub u32);
impl USBINTR {
    #[doc = "USB Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn UE(&self) -> UE {
        let val = (self.0 >> 0usize) & 0x01;
        UE::from_bits(val as u8)
    }
    #[doc = "USB Interrupt Enable."]
    #[inline(always)]
    pub const fn set_UE(&mut self, val: UE) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn UEE(&self) -> UEE {
        let val = (self.0 >> 1usize) & 0x01;
        UEE::from_bits(val as u8)
    }
    #[doc = "USB Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_UEE(&mut self, val: UEE) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PCE(&self) -> PCE {
        let val = (self.0 >> 2usize) & 0x01;
        PCE::from_bits(val as u8)
    }
    #[doc = "Port Change Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_PCE(&mut self, val: PCE) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FRE(&self) -> FRE {
        let val = (self.0 >> 3usize) & 0x01;
        FRE::from_bits(val as u8)
    }
    #[doc = "Frame List Rollover Interrupt Enable."]
    #[inline(always)]
    pub const fn set_FRE(&mut self, val: FRE) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SEE(&self) -> SEE {
        let val = (self.0 >> 4usize) & 0x01;
        SEE::from_bits(val as u8)
    }
    #[doc = "System Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_SEE(&mut self, val: SEE) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Advance Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn AAE(&self) -> AAE {
        let val = (self.0 >> 5usize) & 0x01;
        AAE::from_bits(val as u8)
    }
    #[doc = "Asynchronous Advance Interrupt Enable."]
    #[inline(always)]
    pub const fn set_AAE(&mut self, val: AAE) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn URE(&self) -> URE {
        let val = (self.0 >> 6usize) & 0x01;
        URE::from_bits(val as u8)
    }
    #[doc = "USB Reset Interrupt Enable."]
    #[inline(always)]
    pub const fn set_URE(&mut self, val: URE) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SRE(&self) -> SRE {
        let val = (self.0 >> 7usize) & 0x01;
        SRE::from_bits(val as u8)
    }
    #[doc = "SOF Received Interrupt Enable."]
    #[inline(always)]
    pub const fn set_SRE(&mut self, val: SRE) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sleep Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn SLE(&self) -> SLE {
        let val = (self.0 >> 8usize) & 0x01;
        SLE::from_bits(val as u8)
    }
    #[doc = "Sleep Interrupt Enable."]
    #[inline(always)]
    pub const fn set_SLE(&mut self, val: SLE) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "NAK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn NAKE(&self) -> NAKE {
        let val = (self.0 >> 16usize) & 0x01;
        NAKE::from_bits(val as u8)
    }
    #[doc = "NAK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_NAKE(&mut self, val: NAKE) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB Host Asynchronous Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn UAIE(&self) -> UAIE {
        let val = (self.0 >> 18usize) & 0x01;
        UAIE::from_bits(val as u8)
    }
    #[doc = "USB Host Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub const fn set_UAIE(&mut self, val: UAIE) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "USB Host Periodic Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn UPIE(&self) -> UPIE {
        let val = (self.0 >> 19usize) & 0x01;
        UPIE::from_bits(val as u8)
    }
    #[doc = "USB Host Periodic Interrupt Enable."]
    #[inline(always)]
    pub const fn set_UPIE(&mut self, val: UPIE) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "General Purpose Timer 0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TIE0(&self) -> TIE0 {
        let val = (self.0 >> 24usize) & 0x01;
        TIE0::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer 0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TIE0(&mut self, val: TIE0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer 1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TIE1(&self) -> TIE1 {
        let val = (self.0 >> 25usize) & 0x01;
        TIE1::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer 1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_TIE1(&mut self, val: TIE1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "L1 Exit Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_L1_EXITIE(&self) -> LPM_L1_EXITIE {
        let val = (self.0 >> 28usize) & 0x01;
        LPM_L1_EXITIE::from_bits(val as u8)
    }
    #[doc = "L1 Exit Interrupt Enable."]
    #[inline(always)]
    pub const fn set_LPM_L1_EXITIE(&mut self, val: LPM_L1_EXITIE) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "L1 Entry Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_L1_ENTRYIE(&self) -> LPM_L1_ENTRYIE {
        let val = (self.0 >> 29usize) & 0x01;
        LPM_L1_ENTRYIE::from_bits(val as u8)
    }
    #[doc = "L1 Entry Interrupt Enable."]
    #[inline(always)]
    pub const fn set_LPM_L1_ENTRYIE(&mut self, val: LPM_L1_ENTRYIE) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Device Received Extension Token Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_RCVDIE(&self) -> LPM_DEV_RCVDIE {
        let val = (self.0 >> 30usize) & 0x01;
        LPM_DEV_RCVDIE::from_bits(val as u8)
    }
    #[doc = "Device Received Extension Token Interrupt Enable."]
    #[inline(always)]
    pub const fn set_LPM_DEV_RCVDIE(&mut self, val: LPM_DEV_RCVDIE) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Host Completed LPM Transaction Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_COMPIE(&self) -> LPM_HST_COMPIE {
        let val = (self.0 >> 31usize) & 0x01;
        LPM_HST_COMPIE::from_bits(val as u8)
    }
    #[doc = "Host Completed LPM Transaction Interrupt Enable."]
    #[inline(always)]
    pub const fn set_LPM_HST_COMPIE(&mut self, val: LPM_HST_COMPIE) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USBINTR {
    #[inline(always)]
    fn default() -> USBINTR {
        USBINTR(0)
    }
}
impl core::fmt::Debug for USBINTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBINTR")
            .field("UE", &self.UE())
            .field("UEE", &self.UEE())
            .field("PCE", &self.PCE())
            .field("FRE", &self.FRE())
            .field("SEE", &self.SEE())
            .field("AAE", &self.AAE())
            .field("URE", &self.URE())
            .field("SRE", &self.SRE())
            .field("SLE", &self.SLE())
            .field("NAKE", &self.NAKE())
            .field("UAIE", &self.UAIE())
            .field("UPIE", &self.UPIE())
            .field("TIE0", &self.TIE0())
            .field("TIE1", &self.TIE1())
            .field("LPM_L1_EXITIE", &self.LPM_L1_EXITIE())
            .field("LPM_L1_ENTRYIE", &self.LPM_L1_ENTRYIE())
            .field("LPM_DEV_RCVDIE", &self.LPM_DEV_RCVDIE())
            .field("LPM_HST_COMPIE", &self.LPM_HST_COMPIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBINTR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBINTR {{ UE: {:?}, UEE: {:?}, PCE: {:?}, FRE: {:?}, SEE: {:?}, AAE: {:?}, URE: {:?}, SRE: {:?}, SLE: {:?}, NAKE: {:?}, UAIE: {:?}, UPIE: {:?}, TIE0: {:?}, TIE1: {:?}, LPM_L1_EXITIE: {:?}, LPM_L1_ENTRYIE: {:?}, LPM_DEV_RCVDIE: {:?}, LPM_HST_COMPIE: {:?} }}",
            self.UE(),
            self.UEE(),
            self.PCE(),
            self.FRE(),
            self.SEE(),
            self.AAE(),
            self.URE(),
            self.SRE(),
            self.SLE(),
            self.NAKE(),
            self.UAIE(),
            self.UPIE(),
            self.TIE0(),
            self.TIE1(),
            self.LPM_L1_EXITIE(),
            self.LPM_L1_ENTRYIE(),
            self.LPM_DEV_RCVDIE(),
            self.LPM_HST_COMPIE()
        )
    }
}
#[doc = "USB Device Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBMODE(pub u32);
impl USBMODE {
    #[doc = "Controller Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn CM(&self) -> CM {
        let val = (self.0 >> 0usize) & 0x03;
        CM::from_bits(val as u8)
    }
    #[doc = "Controller Mode."]
    #[inline(always)]
    pub const fn set_CM(&mut self, val: CM) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Endian Select."]
    #[must_use]
    #[inline(always)]
    pub const fn ES(&self) -> ES {
        let val = (self.0 >> 2usize) & 0x01;
        ES::from_bits(val as u8)
    }
    #[doc = "Endian Select."]
    #[inline(always)]
    pub const fn set_ES(&mut self, val: ES) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Lockout Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SLOM(&self) -> SLOM {
        let val = (self.0 >> 3usize) & 0x01;
        SLOM::from_bits(val as u8)
    }
    #[doc = "Setup Lockout Mode."]
    #[inline(always)]
    pub const fn set_SLOM(&mut self, val: SLOM) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Stream Disable Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn SDIS(&self) -> SDIS {
        let val = (self.0 >> 4usize) & 0x01;
        SDIS::from_bits(val as u8)
    }
    #[doc = "Stream Disable Mode."]
    #[inline(always)]
    pub const fn set_SDIS(&mut self, val: SDIS) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for USBMODE {
    #[inline(always)]
    fn default() -> USBMODE {
        USBMODE(0)
    }
}
impl core::fmt::Debug for USBMODE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBMODE")
            .field("CM", &self.CM())
            .field("ES", &self.ES())
            .field("SLOM", &self.SLOM())
            .field("SDIS", &self.SDIS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBMODE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBMODE {{ CM: {:?}, ES: {:?}, SLOM: {:?}, SDIS: {:?} }}",
            self.CM(),
            self.ES(),
            self.SLOM(),
            self.SDIS()
        )
    }
}
#[doc = "USB Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USBSTS(pub u32);
impl USBSTS {
    #[doc = "USB Interrupt (USBINT) Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UI(&self) -> UI {
        let val = (self.0 >> 0usize) & 0x01;
        UI::from_bits(val as u8)
    }
    #[doc = "USB Interrupt (USBINT) Flag."]
    #[inline(always)]
    pub const fn set_UI(&mut self, val: UI) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt (USBERRINT) Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UEI(&self) -> UEI {
        let val = (self.0 >> 1usize) & 0x01;
        UEI::from_bits(val as u8)
    }
    #[doc = "USB Error Interrupt (USBERRINT) Flag."]
    #[inline(always)]
    pub const fn set_UEI(&mut self, val: UEI) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn PCI(&self) -> PCI {
        let val = (self.0 >> 2usize) & 0x01;
        PCI::from_bits(val as u8)
    }
    #[doc = "Port Change Detect Flag."]
    #[inline(always)]
    pub const fn set_PCI(&mut self, val: PCI) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn FRI(&self) -> FRI {
        let val = (self.0 >> 3usize) & 0x01;
        FRI::from_bits(val as u8)
    }
    #[doc = "Frame List Rollover Flag."]
    #[inline(always)]
    pub const fn set_FRI(&mut self, val: FRI) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SEI(&self) -> SEI {
        let val = (self.0 >> 4usize) & 0x01;
        SEI::from_bits(val as u8)
    }
    #[doc = "System Error Flag."]
    #[inline(always)]
    pub const fn set_SEI(&mut self, val: SEI) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Asynchronous Advance Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn AAI(&self) -> AAI {
        let val = (self.0 >> 5usize) & 0x01;
        AAI::from_bits(val as u8)
    }
    #[doc = "Interrupt on Asynchronous Advance Flag."]
    #[inline(always)]
    pub const fn set_AAI(&mut self, val: AAI) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Received."]
    #[must_use]
    #[inline(always)]
    pub const fn URI(&self) -> URI {
        let val = (self.0 >> 6usize) & 0x01;
        URI::from_bits(val as u8)
    }
    #[doc = "USB Reset Received."]
    #[inline(always)]
    pub const fn set_URI(&mut self, val: URI) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SRI(&self) -> SRI {
        let val = (self.0 >> 7usize) & 0x01;
        SRI::from_bits(val as u8)
    }
    #[doc = "SOF Received Flag."]
    #[inline(always)]
    pub const fn set_SRI(&mut self, val: SRI) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Device Controller Suspend Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn SLI(&self) -> SLI {
        let val = (self.0 >> 8usize) & 0x01;
        SLI::from_bits(val as u8)
    }
    #[doc = "Device Controller Suspend Flag."]
    #[inline(always)]
    pub const fn set_SLI(&mut self, val: SLI) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "ULPI Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ULPII(&self) -> ULPII {
        let val = (self.0 >> 10usize) & 0x01;
        ULPII::from_bits(val as u8)
    }
    #[doc = "ULPI Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ULPII(&mut self, val: ULPII) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "HC Halted."]
    #[must_use]
    #[inline(always)]
    pub const fn HCH(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HC Halted."]
    #[inline(always)]
    pub const fn set_HCH(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reclamation."]
    #[must_use]
    #[inline(always)]
    pub const fn RCL(&self) -> RCL {
        let val = (self.0 >> 13usize) & 0x01;
        RCL::from_bits(val as u8)
    }
    #[doc = "Reclamation."]
    #[inline(always)]
    pub const fn set_RCL(&mut self, val: RCL) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Periodic Schedule Status."]
    #[must_use]
    #[inline(always)]
    pub const fn PS(&self) -> PS {
        let val = (self.0 >> 14usize) & 0x01;
        PS::from_bits(val as u8)
    }
    #[doc = "Periodic Schedule Status."]
    #[inline(always)]
    pub const fn set_PS(&mut self, val: PS) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Asynchronous Schedule Status."]
    #[must_use]
    #[inline(always)]
    pub const fn AS(&self) -> AS {
        let val = (self.0 >> 15usize) & 0x01;
        AS::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Status."]
    #[inline(always)]
    pub const fn set_AS(&mut self, val: AS) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn NAKI(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt."]
    #[inline(always)]
    pub const fn set_NAKI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "USB Host Asynchronous Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UAI(&self) -> UAI {
        let val = (self.0 >> 18usize) & 0x01;
        UAI::from_bits(val as u8)
    }
    #[doc = "USB Host Asynchronous Interrupt Flag."]
    #[inline(always)]
    pub const fn set_UAI(&mut self, val: UAI) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "USB Host Periodic Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn UPI(&self) -> UPI {
        let val = (self.0 >> 19usize) & 0x01;
        UPI::from_bits(val as u8)
    }
    #[doc = "USB Host Periodic Interrupt Flag."]
    #[inline(always)]
    pub const fn set_UPI(&mut self, val: UPI) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0) Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn TI0(&self) -> TI0 {
        let val = (self.0 >> 24usize) & 0x01;
        TI0::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0) Flag."]
    #[inline(always)]
    pub const fn set_TI0(&mut self, val: TI0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1) Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn TI1(&self) -> TI1 {
        let val = (self.0 >> 25usize) & 0x01;
        TI1::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1) Flag."]
    #[inline(always)]
    pub const fn set_TI1(&mut self, val: TI1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "L1 Exit Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_L1_EXITI(&self) -> LPM_L1_EXITI {
        let val = (self.0 >> 28usize) & 0x01;
        LPM_L1_EXITI::from_bits(val as u8)
    }
    #[doc = "L1 Exit Interrupt Flag."]
    #[inline(always)]
    pub const fn set_LPM_L1_EXITI(&mut self, val: LPM_L1_EXITI) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "L1 Entry Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_L1_ENTRYI(&self) -> LPM_L1_ENTRYI {
        let val = (self.0 >> 29usize) & 0x01;
        LPM_L1_ENTRYI::from_bits(val as u8)
    }
    #[doc = "L1 Entry Interrupt Flag."]
    #[inline(always)]
    pub const fn set_LPM_L1_ENTRYI(&mut self, val: LPM_L1_ENTRYI) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Device Received Extension Token Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_DEV_RCVDI(&self) -> LPM_DEV_RCVDI {
        let val = (self.0 >> 30usize) & 0x01;
        LPM_DEV_RCVDI::from_bits(val as u8)
    }
    #[doc = "Device Received Extension Token Interrupt Flag."]
    #[inline(always)]
    pub const fn set_LPM_DEV_RCVDI(&mut self, val: LPM_DEV_RCVDI) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Host Completes the LPM Transaction Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_HST_COMPI(&self) -> LPM_HST_COMPI {
        let val = (self.0 >> 31usize) & 0x01;
        LPM_HST_COMPI::from_bits(val as u8)
    }
    #[doc = "Host Completes the LPM Transaction Interrupt Flag."]
    #[inline(always)]
    pub const fn set_LPM_HST_COMPI(&mut self, val: LPM_HST_COMPI) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USBSTS {
    #[inline(always)]
    fn default() -> USBSTS {
        USBSTS(0)
    }
}
impl core::fmt::Debug for USBSTS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USBSTS")
            .field("UI", &self.UI())
            .field("UEI", &self.UEI())
            .field("PCI", &self.PCI())
            .field("FRI", &self.FRI())
            .field("SEI", &self.SEI())
            .field("AAI", &self.AAI())
            .field("URI", &self.URI())
            .field("SRI", &self.SRI())
            .field("SLI", &self.SLI())
            .field("ULPII", &self.ULPII())
            .field("HCH", &self.HCH())
            .field("RCL", &self.RCL())
            .field("PS", &self.PS())
            .field("AS", &self.AS())
            .field("NAKI", &self.NAKI())
            .field("UAI", &self.UAI())
            .field("UPI", &self.UPI())
            .field("TI0", &self.TI0())
            .field("TI1", &self.TI1())
            .field("LPM_L1_EXITI", &self.LPM_L1_EXITI())
            .field("LPM_L1_ENTRYI", &self.LPM_L1_ENTRYI())
            .field("LPM_DEV_RCVDI", &self.LPM_DEV_RCVDI())
            .field("LPM_HST_COMPI", &self.LPM_HST_COMPI())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USBSTS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USBSTS {{ UI: {:?}, UEI: {:?}, PCI: {:?}, FRI: {:?}, SEI: {:?}, AAI: {:?}, URI: {:?}, SRI: {:?}, SLI: {:?}, ULPII: {:?}, HCH: {=bool:?}, RCL: {:?}, PS: {:?}, AS: {:?}, NAKI: {=bool:?}, UAI: {:?}, UPI: {:?}, TI0: {:?}, TI1: {:?}, LPM_L1_EXITI: {:?}, LPM_L1_ENTRYI: {:?}, LPM_DEV_RCVDI: {:?}, LPM_HST_COMPI: {:?} }}",
            self.UI(),
            self.UEI(),
            self.PCI(),
            self.FRI(),
            self.SEI(),
            self.AAI(),
            self.URI(),
            self.SRI(),
            self.SLI(),
            self.ULPII(),
            self.HCH(),
            self.RCL(),
            self.PS(),
            self.AS(),
            self.NAKI(),
            self.UAI(),
            self.UPI(),
            self.TI0(),
            self.TI1(),
            self.LPM_L1_EXITI(),
            self.LPM_L1_ENTRYI(),
            self.LPM_DEV_RCVDI(),
            self.LPM_HST_COMPI()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AAE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl AAE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AAE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AAE {
    #[inline(always)]
    fn from(val: u8) -> AAE {
        AAE::from_bits(val)
    }
}
impl From<AAE> for u8 {
    #[inline(always)]
    fn from(val: AAE) -> u8 {
        AAE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AAI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl AAI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AAI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AAI {
    #[inline(always)]
    fn from(val: u8) -> AAI {
        AAI::from_bits(val)
    }
}
impl From<AAI> for u8 {
    #[inline(always)]
    fn from(val: AAI) -> u8 {
        AAI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AHBBRST {
    #[doc = "Incremental burst of unspecified length only."]
    INCR_BURST = 0x0,
    #[doc = "INCR4 burst, then single transfer."]
    INCR4_BURST = 0x01,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer."]
    INCR8_BURST = 0x02,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer."]
    INCR16_BURST = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "INCR4 burst, then incremental burst of unspecified length."]
    INCR4_UNSPEC = 0x05,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length."]
    INCR8_4_UNSPEC = 0x06,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length."]
    INCR16_8_4_UNSPEC = 0x07,
}
impl AHBBRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AHBBRST {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AHBBRST {
    #[inline(always)]
    fn from(val: u8) -> AHBBRST {
        AHBBRST::from_bits(val)
    }
}
impl From<AHBBRST> for u8 {
    #[inline(always)]
    fn from(val: AHBBRST) -> u8 {
        AHBBRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AS {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Enabled."]
    ENABLE = 0x01,
}
impl AS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AS {
    #[inline(always)]
    fn from(val: u8) -> AS {
        AS::from_bits(val)
    }
}
impl From<AS> for u8 {
    #[inline(always)]
    fn from(val: AS) -> u8 {
        AS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ASE {
    #[doc = "Do not process the asynchronous schedule."]
    DONT_PROCESS_ASYNC = 0x0,
    #[doc = "Access the asynchronous schedule."]
    ACCESS_ASYNC = 0x01,
}
impl ASE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ASE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ASE {
    #[inline(always)]
    fn from(val: u8) -> ASE {
        ASE::from_bits(val)
    }
}
impl From<ASE> for u8 {
    #[inline(always)]
    fn from(val: ASE) -> u8 {
        ASE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ASPE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ASPE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ASPE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ASPE {
    #[inline(always)]
    fn from(val: u8) -> ASPE {
        ASPE::from_bits(val)
    }
}
impl From<ASPE> for u8 {
    #[inline(always)]
    fn from(val: ASPE) -> u8 {
        ASPE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CCS {
    #[doc = "No device present or attached."]
    DEVICE_UNAVAILABLE = 0x0,
    #[doc = "Device present and attached."]
    DEVICE_AVAILABLE = 0x01,
}
impl CCS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CCS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CCS {
    #[inline(always)]
    fn from(val: u8) -> CCS {
        CCS::from_bits(val)
    }
}
impl From<CCS> for u8 {
    #[inline(always)]
    fn from(val: CCS) -> u8 {
        CCS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CF {
    #[doc = "Port routing to classic host controller."]
    PORT_ROUTING_CLASSIC_HOST = 0x0,
    #[doc = "Port routing to this host controller."]
    PORT_ROUTING_HOST = 0x01,
}
impl CF {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CF {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CF {
    #[inline(always)]
    fn from(val: u8) -> CF {
        CF::from_bits(val)
    }
}
impl From<CF> for u8 {
    #[inline(always)]
    fn from(val: CF) -> u8 {
        CF::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CM {
    #[doc = "Idle (default for host and device combination)."]
    IDL = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Device controller (default for device-only controller)."]
    DEVICE_CONTR = 0x02,
    #[doc = "Host controller (default for host-only controller)."]
    HOST_CONTR = 0x03,
}
impl CM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CM {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CM {
    #[inline(always)]
    fn from(val: u8) -> CM {
        CM::from_bits(val)
    }
}
impl From<CM> for u8 {
    #[inline(always)]
    fn from(val: CM) -> u8 {
        CM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CSC {
    #[doc = "No change occurred."]
    NO_CHANGE = 0x0,
    #[doc = "Change occurred."]
    CHANGE = 0x01,
}
impl CSC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CSC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CSC {
    #[inline(always)]
    fn from(val: u8) -> CSC {
        CSC::from_bits(val)
    }
}
impl From<CSC> for u8 {
    #[inline(always)]
    fn from(val: CSC) -> u8 {
        CSC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCCPARAMS_DC {
    #[doc = "Not device capable."]
    NOT_DC = 0x0,
    #[doc = "Device capable."]
    DC = 0x01,
}
impl DCCPARAMS_DC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCCPARAMS_DC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCCPARAMS_DC {
    #[inline(always)]
    fn from(val: u8) -> DCCPARAMS_DC {
        DCCPARAMS_DC::from_bits(val)
    }
}
impl From<DCCPARAMS_DC> for u8 {
    #[inline(always)]
    fn from(val: DCCPARAMS_DC) -> u8 {
        DCCPARAMS_DC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DCCPARAMS_HC {
    #[doc = "Not host capable."]
    NOT_HC = 0x0,
    #[doc = "Host capable."]
    HC = 0x01,
}
impl DCCPARAMS_HC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DCCPARAMS_HC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DCCPARAMS_HC {
    #[inline(always)]
    fn from(val: u8) -> DCCPARAMS_HC {
        DCCPARAMS_HC::from_bits(val)
    }
}
impl From<DCCPARAMS_HC> for u8 {
    #[inline(always)]
    fn from(val: DCCPARAMS_HC) -> u8 {
        DCCPARAMS_HC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DEN {
    #[doc = "Not device capable."]
    NOT_DEN = 0x0,
    #[doc = "Device capable."]
    DEN = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
}
impl DEN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DEN {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DEN {
    #[inline(always)]
    fn from(val: u8) -> DEN {
        DEN::from_bits(val)
    }
}
impl From<DEN> for u8 {
    #[inline(always)]
    fn from(val: DEN) -> u8 {
        DEN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL0_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL0_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL0_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL0_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL0_RXE {
        ENDPTCTRL0_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL0_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL0_RXE) -> u8 {
        ENDPTCTRL0_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL0_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL0_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL0_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL0_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL0_RXS {
        ENDPTCTRL0_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL0_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL0_RXS) -> u8 {
        ENDPTCTRL0_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL0_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL0_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL0_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL0_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL0_TXE {
        ENDPTCTRL0_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL0_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL0_TXE) -> u8 {
        ENDPTCTRL0_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL0_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL0_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL0_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL0_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL0_TXS {
        ENDPTCTRL0_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL0_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL0_TXS) -> u8 {
        ENDPTCTRL0_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL1_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_RXE {
        ENDPTCTRL1_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL1_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_RXE) -> u8 {
        ENDPTCTRL1_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL1_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_RXI {
        ENDPTCTRL1_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL1_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_RXI) -> u8 {
        ENDPTCTRL1_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL1_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_RXR {
        ENDPTCTRL1_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL1_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_RXR) -> u8 {
        ENDPTCTRL1_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL1_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_RXS {
        ENDPTCTRL1_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL1_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_RXS) -> u8 {
        ENDPTCTRL1_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL1_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_RXT {
        ENDPTCTRL1_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL1_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_RXT) -> u8 {
        ENDPTCTRL1_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL1_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_TXE {
        ENDPTCTRL1_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL1_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_TXE) -> u8 {
        ENDPTCTRL1_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL1_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_TXI {
        ENDPTCTRL1_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL1_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_TXI) -> u8 {
        ENDPTCTRL1_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL1_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_TXR {
        ENDPTCTRL1_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL1_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_TXR) -> u8 {
        ENDPTCTRL1_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL1_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_TXS {
        ENDPTCTRL1_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL1_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_TXS) -> u8 {
        ENDPTCTRL1_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL1_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL1_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL1_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL1_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL1_TXT {
        ENDPTCTRL1_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL1_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL1_TXT) -> u8 {
        ENDPTCTRL1_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL2_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_RXE {
        ENDPTCTRL2_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL2_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_RXE) -> u8 {
        ENDPTCTRL2_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL2_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_RXI {
        ENDPTCTRL2_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL2_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_RXI) -> u8 {
        ENDPTCTRL2_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL2_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_RXR {
        ENDPTCTRL2_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL2_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_RXR) -> u8 {
        ENDPTCTRL2_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL2_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_RXS {
        ENDPTCTRL2_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL2_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_RXS) -> u8 {
        ENDPTCTRL2_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL2_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_RXT {
        ENDPTCTRL2_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL2_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_RXT) -> u8 {
        ENDPTCTRL2_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL2_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_TXE {
        ENDPTCTRL2_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL2_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_TXE) -> u8 {
        ENDPTCTRL2_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL2_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_TXI {
        ENDPTCTRL2_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL2_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_TXI) -> u8 {
        ENDPTCTRL2_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL2_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_TXR {
        ENDPTCTRL2_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL2_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_TXR) -> u8 {
        ENDPTCTRL2_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL2_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_TXS {
        ENDPTCTRL2_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL2_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_TXS) -> u8 {
        ENDPTCTRL2_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL2_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL2_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL2_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL2_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL2_TXT {
        ENDPTCTRL2_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL2_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL2_TXT) -> u8 {
        ENDPTCTRL2_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL3_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_RXE {
        ENDPTCTRL3_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL3_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_RXE) -> u8 {
        ENDPTCTRL3_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL3_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_RXI {
        ENDPTCTRL3_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL3_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_RXI) -> u8 {
        ENDPTCTRL3_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL3_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_RXR {
        ENDPTCTRL3_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL3_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_RXR) -> u8 {
        ENDPTCTRL3_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL3_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_RXS {
        ENDPTCTRL3_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL3_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_RXS) -> u8 {
        ENDPTCTRL3_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL3_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_RXT {
        ENDPTCTRL3_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL3_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_RXT) -> u8 {
        ENDPTCTRL3_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL3_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_TXE {
        ENDPTCTRL3_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL3_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_TXE) -> u8 {
        ENDPTCTRL3_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL3_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_TXI {
        ENDPTCTRL3_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL3_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_TXI) -> u8 {
        ENDPTCTRL3_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL3_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_TXR {
        ENDPTCTRL3_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL3_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_TXR) -> u8 {
        ENDPTCTRL3_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL3_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_TXS {
        ENDPTCTRL3_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL3_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_TXS) -> u8 {
        ENDPTCTRL3_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL3_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL3_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL3_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL3_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL3_TXT {
        ENDPTCTRL3_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL3_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL3_TXT) -> u8 {
        ENDPTCTRL3_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL4_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_RXE {
        ENDPTCTRL4_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL4_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_RXE) -> u8 {
        ENDPTCTRL4_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_RXI {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL4_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_RXI {
        ENDPTCTRL4_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL4_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_RXI) -> u8 {
        ENDPTCTRL4_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL4_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_RXR {
        ENDPTCTRL4_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL4_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_RXR) -> u8 {
        ENDPTCTRL4_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL4_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_RXS {
        ENDPTCTRL4_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL4_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_RXS) -> u8 {
        ENDPTCTRL4_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL4_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_RXT {
        ENDPTCTRL4_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL4_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_RXT) -> u8 {
        ENDPTCTRL4_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL4_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_TXE {
        ENDPTCTRL4_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL4_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_TXE) -> u8 {
        ENDPTCTRL4_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL4_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_TXI {
        ENDPTCTRL4_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL4_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_TXI) -> u8 {
        ENDPTCTRL4_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL4_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_TXR {
        ENDPTCTRL4_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL4_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_TXR) -> u8 {
        ENDPTCTRL4_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL4_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_TXS {
        ENDPTCTRL4_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL4_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_TXS) -> u8 {
        ENDPTCTRL4_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL4_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL4_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL4_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL4_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL4_TXT {
        ENDPTCTRL4_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL4_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL4_TXT) -> u8 {
        ENDPTCTRL4_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL5_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_RXE {
        ENDPTCTRL5_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL5_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_RXE) -> u8 {
        ENDPTCTRL5_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL5_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_RXI {
        ENDPTCTRL5_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL5_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_RXI) -> u8 {
        ENDPTCTRL5_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL5_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_RXR {
        ENDPTCTRL5_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL5_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_RXR) -> u8 {
        ENDPTCTRL5_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL5_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_RXS {
        ENDPTCTRL5_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL5_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_RXS) -> u8 {
        ENDPTCTRL5_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL5_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_RXT {
        ENDPTCTRL5_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL5_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_RXT) -> u8 {
        ENDPTCTRL5_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL5_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_TXE {
        ENDPTCTRL5_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL5_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_TXE) -> u8 {
        ENDPTCTRL5_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL5_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_TXI {
        ENDPTCTRL5_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL5_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_TXI) -> u8 {
        ENDPTCTRL5_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL5_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_TXR {
        ENDPTCTRL5_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL5_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_TXR) -> u8 {
        ENDPTCTRL5_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL5_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_TXS {
        ENDPTCTRL5_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL5_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_TXS) -> u8 {
        ENDPTCTRL5_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL5_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL5_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL5_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL5_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL5_TXT {
        ENDPTCTRL5_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL5_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL5_TXT) -> u8 {
        ENDPTCTRL5_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL6_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_RXE {
        ENDPTCTRL6_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL6_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_RXE) -> u8 {
        ENDPTCTRL6_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL6_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_RXI {
        ENDPTCTRL6_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL6_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_RXI) -> u8 {
        ENDPTCTRL6_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL6_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_RXR {
        ENDPTCTRL6_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL6_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_RXR) -> u8 {
        ENDPTCTRL6_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL6_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_RXS {
        ENDPTCTRL6_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL6_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_RXS) -> u8 {
        ENDPTCTRL6_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL6_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_RXT {
        ENDPTCTRL6_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL6_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_RXT) -> u8 {
        ENDPTCTRL6_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL6_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_TXE {
        ENDPTCTRL6_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL6_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_TXE) -> u8 {
        ENDPTCTRL6_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL6_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_TXI {
        ENDPTCTRL6_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL6_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_TXI) -> u8 {
        ENDPTCTRL6_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL6_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_TXR {
        ENDPTCTRL6_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL6_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_TXR) -> u8 {
        ENDPTCTRL6_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL6_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_TXS {
        ENDPTCTRL6_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL6_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_TXS) -> u8 {
        ENDPTCTRL6_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL6_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL6_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL6_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL6_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL6_TXT {
        ENDPTCTRL6_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL6_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL6_TXT) -> u8 {
        ENDPTCTRL6_TXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_RXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL7_RXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_RXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_RXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_RXE {
        ENDPTCTRL7_RXE::from_bits(val)
    }
}
impl From<ENDPTCTRL7_RXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_RXE) -> u8 {
        ENDPTCTRL7_RXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_RXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL7_RXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_RXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_RXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_RXI {
        ENDPTCTRL7_RXI::from_bits(val)
    }
}
impl From<ENDPTCTRL7_RXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_RXI) -> u8 {
        ENDPTCTRL7_RXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_RXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL7_RXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_RXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_RXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_RXR {
        ENDPTCTRL7_RXR::from_bits(val)
    }
}
impl From<ENDPTCTRL7_RXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_RXR) -> u8 {
        ENDPTCTRL7_RXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_RXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL7_RXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_RXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_RXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_RXS {
        ENDPTCTRL7_RXS::from_bits(val)
    }
}
impl From<ENDPTCTRL7_RXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_RXS) -> u8 {
        ENDPTCTRL7_RXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_RXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL7_RXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_RXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_RXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_RXT {
        ENDPTCTRL7_RXT::from_bits(val)
    }
}
impl From<ENDPTCTRL7_RXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_RXT) -> u8 {
        ENDPTCTRL7_RXT::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_TXE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl ENDPTCTRL7_TXE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_TXE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_TXE {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_TXE {
        ENDPTCTRL7_TXE::from_bits(val)
    }
}
impl From<ENDPTCTRL7_TXE> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_TXE) -> u8 {
        ENDPTCTRL7_TXE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_TXI {
    #[doc = "Allow."]
    ENABLE = 0x0,
    #[doc = "Inhibit."]
    INHIBIT = 0x01,
}
impl ENDPTCTRL7_TXI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_TXI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_TXI {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_TXI {
        ENDPTCTRL7_TXI::from_bits(val)
    }
}
impl From<ENDPTCTRL7_TXI> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_TXI) -> u8 {
        ENDPTCTRL7_TXI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_TXR {
    #[doc = "Does not reset."]
    NO_RESET = 0x0,
    #[doc = "Resets."]
    RESET = 0x01,
}
impl ENDPTCTRL7_TXR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_TXR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_TXR {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_TXR {
        ENDPTCTRL7_TXR::from_bits(val)
    }
}
impl From<ENDPTCTRL7_TXR> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_TXR) -> u8 {
        ENDPTCTRL7_TXR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_TXS {
    #[doc = "Endpoint OK."]
    DISABLE = 0x0,
    #[doc = "Endpoint stalled."]
    ENABLE = 0x01,
}
impl ENDPTCTRL7_TXS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_TXS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_TXS {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_TXS {
        ENDPTCTRL7_TXS::from_bits(val)
    }
}
impl From<ENDPTCTRL7_TXS> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_TXS) -> u8 {
        ENDPTCTRL7_TXS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ENDPTCTRL7_TXT {
    #[doc = "Control."]
    CTL = 0x0,
    #[doc = "Isochronous."]
    ISO = 0x01,
    #[doc = "Bulk."]
    BLK = 0x02,
    #[doc = "Interrupt."]
    IRQ = 0x03,
}
impl ENDPTCTRL7_TXT {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ENDPTCTRL7_TXT {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ENDPTCTRL7_TXT {
    #[inline(always)]
    fn from(val: u8) -> ENDPTCTRL7_TXT {
        ENDPTCTRL7_TXT::from_bits(val)
    }
}
impl From<ENDPTCTRL7_TXT> for u8 {
    #[inline(always)]
    fn from(val: ENDPTCTRL7_TXT) -> u8 {
        ENDPTCTRL7_TXT::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ENDPTSETUPSTAT_FLAG(u16);
impl ENDPTSETUPSTAT_FLAG {
    #[doc = "Not received."]
    pub const NOTREC: Self = Self(0x0);
    #[doc = "Received."]
    pub const RECVD: Self = Self(0x01);
}
impl ENDPTSETUPSTAT_FLAG {
    pub const fn from_bits(val: u16) -> ENDPTSETUPSTAT_FLAG {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for ENDPTSETUPSTAT_FLAG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NOTREC"),
            0x01 => f.write_str("RECVD"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ENDPTSETUPSTAT_FLAG {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NOTREC"),
            0x01 => defmt::write!(f, "RECVD"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for ENDPTSETUPSTAT_FLAG {
    #[inline(always)]
    fn from(val: u16) -> ENDPTSETUPSTAT_FLAG {
        ENDPTSETUPSTAT_FLAG::from_bits(val)
    }
}
impl From<ENDPTSETUPSTAT_FLAG> for u16 {
    #[inline(always)]
    fn from(val: ENDPTSETUPSTAT_FLAG) -> u16 {
        ENDPTSETUPSTAT_FLAG::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EPRN(u8);
impl EPRN {
    #[doc = "No NACK."]
    pub const NONACK: Self = Self(0x0);
    #[doc = "NACK."]
    pub const NACK: Self = Self(0x01);
}
impl EPRN {
    pub const fn from_bits(val: u8) -> EPRN {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EPRN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONACK"),
            0x01 => f.write_str("NACK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPRN {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONACK"),
            0x01 => defmt::write!(f, "NACK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EPRN {
    #[inline(always)]
    fn from(val: u8) -> EPRN {
        EPRN::from_bits(val)
    }
}
impl From<EPRN> for u8 {
    #[inline(always)]
    fn from(val: EPRN) -> u8 {
        EPRN::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EPTN(u8);
impl EPTN {
    #[doc = "No NACK."]
    pub const NONACK: Self = Self(0x0);
    #[doc = "NACK."]
    pub const NACK: Self = Self(0x01);
}
impl EPTN {
    pub const fn from_bits(val: u8) -> EPTN {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for EPTN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("NONACK"),
            0x01 => f.write_str("NACK"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPTN {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "NONACK"),
            0x01 => defmt::write!(f, "NACK"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for EPTN {
    #[inline(always)]
    fn from(val: u8) -> EPTN {
        EPTN::from_bits(val)
    }
}
impl From<EPTN> for u8 {
    #[inline(always)]
    fn from(val: EPTN) -> u8 {
        EPTN::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ERCE(u8);
impl ERCE {
    #[doc = "Transmit incomplete."]
    pub const TRANSNOTCOMP: Self = Self(0x0);
    #[doc = "Transmit complete."]
    pub const TRANSCOMP: Self = Self(0x01);
}
impl ERCE {
    pub const fn from_bits(val: u8) -> ERCE {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ERCE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TRANSNOTCOMP"),
            0x01 => f.write_str("TRANSCOMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ERCE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRANSNOTCOMP"),
            0x01 => defmt::write!(f, "TRANSCOMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ERCE {
    #[inline(always)]
    fn from(val: u8) -> ERCE {
        ERCE::from_bits(val)
    }
}
impl From<ERCE> for u8 {
    #[inline(always)]
    fn from(val: ERCE) -> u8 {
        ERCE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ES {
    #[doc = "Little endian (default)."]
    LITTLE_ENDIAN = 0x0,
    #[doc = "Big endian."]
    BIG_ENDIAN = 0x01,
}
impl ES {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ES {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ES {
    #[inline(always)]
    fn from(val: u8) -> ES {
        ES::from_bits(val)
    }
}
impl From<ES> for u8 {
    #[inline(always)]
    fn from(val: ES) -> u8 {
        ES::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ETCE(u8);
impl ETCE {
    #[doc = "Transmit incomplete."]
    pub const TRANSNOTCOMP: Self = Self(0x0);
    #[doc = "Transmit complete."]
    pub const TRANSCOMP: Self = Self(0x01);
}
impl ETCE {
    pub const fn from_bits(val: u8) -> ETCE {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ETCE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("TRANSNOTCOMP"),
            0x01 => f.write_str("TRANSCOMP"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ETCE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "TRANSNOTCOMP"),
            0x01 => defmt::write!(f, "TRANSCOMP"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ETCE {
    #[inline(always)]
    fn from(val: u8) -> ETCE {
        ETCE::from_bits(val)
    }
}
impl From<ETCE> for u8 {
    #[inline(always)]
    fn from(val: ETCE) -> u8 {
        ETCE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FPR {
    #[doc = "No resume (K-state) detected or driven on port."]
    DISABLE = 0x0,
    #[doc = "Resume detected or driven on port."]
    ENABLE = 0x01,
}
impl FPR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FPR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FPR {
    #[inline(always)]
    fn from(val: u8) -> FPR {
        FPR::from_bits(val)
    }
}
impl From<FPR> for u8 {
    #[inline(always)]
    fn from(val: FPR) -> u8 {
        FPR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl FRE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRE {
    #[inline(always)]
    fn from(val: u8) -> FRE {
        FRE::from_bits(val)
    }
}
impl From<FRE> for u8 {
    #[inline(always)]
    fn from(val: FRE) -> u8 {
        FRE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FRI {
    #[doc = "Frame list index did not roll over."]
    ROLL_NO = 0x0,
    #[doc = "Frame list index rolled over."]
    ROLL_YES = 0x01,
}
impl FRI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FRI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FRI {
    #[inline(always)]
    fn from(val: u8) -> FRI {
        FRI::from_bits(val)
    }
}
impl From<FRI> for u8 {
    #[inline(always)]
    fn from(val: FRI) -> u8 {
        FRI::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FRINDEX_VALUE(u16);
impl FRINDEX_VALUE {
    #[doc = "(1024) 12."]
    pub const FRINDEX_1024: Self = Self(0x0);
    #[doc = "(512) 11."]
    pub const FRINDEX_512: Self = Self(0x01);
    #[doc = "(256) 10."]
    pub const FRINDEX_256: Self = Self(0x02);
    #[doc = "(128) 9."]
    pub const FRINDEX_128: Self = Self(0x03);
    #[doc = "(64) 8."]
    pub const FRINDEX_64: Self = Self(0x04);
    #[doc = "(32) 7."]
    pub const FRINDEX_32: Self = Self(0x05);
    #[doc = "(16) 6."]
    pub const FRINDEX_16: Self = Self(0x06);
    #[doc = "(8) 5."]
    pub const FRINDEX_8: Self = Self(0x07);
}
impl FRINDEX_VALUE {
    pub const fn from_bits(val: u16) -> FRINDEX_VALUE {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for FRINDEX_VALUE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FRINDEX_1024"),
            0x01 => f.write_str("FRINDEX_512"),
            0x02 => f.write_str("FRINDEX_256"),
            0x03 => f.write_str("FRINDEX_128"),
            0x04 => f.write_str("FRINDEX_64"),
            0x05 => f.write_str("FRINDEX_32"),
            0x06 => f.write_str("FRINDEX_16"),
            0x07 => f.write_str("FRINDEX_8"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FRINDEX_VALUE {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FRINDEX_1024"),
            0x01 => defmt::write!(f, "FRINDEX_512"),
            0x02 => defmt::write!(f, "FRINDEX_256"),
            0x03 => defmt::write!(f, "FRINDEX_128"),
            0x04 => defmt::write!(f, "FRINDEX_64"),
            0x05 => defmt::write!(f, "FRINDEX_32"),
            0x06 => defmt::write!(f, "FRINDEX_16"),
            0x07 => defmt::write!(f, "FRINDEX_8"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for FRINDEX_VALUE {
    #[inline(always)]
    fn from(val: u16) -> FRINDEX_VALUE {
        FRINDEX_VALUE::from_bits(val)
    }
}
impl From<FRINDEX_VALUE> for u16 {
    #[inline(always)]
    fn from(val: FRINDEX_VALUE) -> u16 {
        FRINDEX_VALUE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER0CTRL_GPTMODE {
    #[doc = "One Shot mode."]
    ONE_SHOT = 0x0,
    #[doc = "Repeat mode."]
    REPEAT = 0x01,
}
impl GPTIMER0CTRL_GPTMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER0CTRL_GPTMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER0CTRL_GPTMODE {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER0CTRL_GPTMODE {
        GPTIMER0CTRL_GPTMODE::from_bits(val)
    }
}
impl From<GPTIMER0CTRL_GPTMODE> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER0CTRL_GPTMODE) -> u8 {
        GPTIMER0CTRL_GPTMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER0CTRL_GPTRST {
    #[doc = "No action."]
    NO_ACTION = 0x0,
    #[doc = "Load counter value."]
    LOAD_CNTR = 0x01,
}
impl GPTIMER0CTRL_GPTRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER0CTRL_GPTRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER0CTRL_GPTRST {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER0CTRL_GPTRST {
        GPTIMER0CTRL_GPTRST::from_bits(val)
    }
}
impl From<GPTIMER0CTRL_GPTRST> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER0CTRL_GPTRST) -> u8 {
        GPTIMER0CTRL_GPTRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER0CTRL_GPTRUN {
    #[doc = "Stopped counting."]
    STOP_CNTR = 0x0,
    #[doc = "Running."]
    RUN = 0x01,
}
impl GPTIMER0CTRL_GPTRUN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER0CTRL_GPTRUN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER0CTRL_GPTRUN {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER0CTRL_GPTRUN {
        GPTIMER0CTRL_GPTRUN::from_bits(val)
    }
}
impl From<GPTIMER0CTRL_GPTRUN> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER0CTRL_GPTRUN) -> u8 {
        GPTIMER0CTRL_GPTRUN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER1CTRL_GPTMODE {
    #[doc = "One Shot mode."]
    ONE_SHOT = 0x0,
    #[doc = "Repeat mode."]
    REPEAT = 0x01,
}
impl GPTIMER1CTRL_GPTMODE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER1CTRL_GPTMODE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER1CTRL_GPTMODE {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER1CTRL_GPTMODE {
        GPTIMER1CTRL_GPTMODE::from_bits(val)
    }
}
impl From<GPTIMER1CTRL_GPTMODE> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER1CTRL_GPTMODE) -> u8 {
        GPTIMER1CTRL_GPTMODE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER1CTRL_GPTRST {
    #[doc = "No action."]
    NO_ACTION = 0x0,
    #[doc = "Load counter value."]
    LOAD_CNTR = 0x01,
}
impl GPTIMER1CTRL_GPTRST {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER1CTRL_GPTRST {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER1CTRL_GPTRST {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER1CTRL_GPTRST {
        GPTIMER1CTRL_GPTRST::from_bits(val)
    }
}
impl From<GPTIMER1CTRL_GPTRST> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER1CTRL_GPTRST) -> u8 {
        GPTIMER1CTRL_GPTRST::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GPTIMER1CTRL_GPTRUN {
    #[doc = "Stopped counting."]
    STOP_CNTR = 0x0,
    #[doc = "Running."]
    RUN = 0x01,
}
impl GPTIMER1CTRL_GPTRUN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GPTIMER1CTRL_GPTRUN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GPTIMER1CTRL_GPTRUN {
    #[inline(always)]
    fn from(val: u8) -> GPTIMER1CTRL_GPTRUN {
        GPTIMER1CTRL_GPTRUN::from_bits(val)
    }
}
impl From<GPTIMER1CTRL_GPTRUN> for u8 {
    #[inline(always)]
    fn from(val: GPTIMER1CTRL_GPTRUN) -> u8 {
        GPTIMER1CTRL_GPTRUN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HAAR {
    #[doc = "Disable."]
    HAAR_0 = 0x0,
    #[doc = "Enable."]
    HAAR_1 = 0x01,
}
impl HAAR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HAAR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HAAR {
    #[inline(always)]
    fn from(val: u8) -> HAAR {
        HAAR::from_bits(val)
    }
}
impl From<HAAR> for u8 {
    #[inline(always)]
    fn from(val: HAAR) -> u8 {
        HAAR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HABA {
    #[doc = "Disable."]
    HABA_0 = 0x0,
    #[doc = "Enable."]
    HABA_1 = 0x01,
}
impl HABA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HABA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HABA {
    #[inline(always)]
    fn from(val: u8) -> HABA {
        HABA::from_bits(val)
    }
}
impl From<HABA> for u8 {
    #[inline(always)]
    fn from(val: HABA) -> u8 {
        HABA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HADP {
    #[doc = "Disable."]
    HADP_0 = 0x0,
    #[doc = "Enable."]
    HADP_1 = 0x01,
}
impl HADP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HADP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HADP {
    #[inline(always)]
    fn from(val: u8) -> HADP {
        HADP::from_bits(val)
    }
}
impl From<HADP> for u8 {
    #[inline(always)]
    fn from(val: HADP) -> u8 {
        HADP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HSP {
    #[doc = "Not in HS mode."]
    NOTHS = 0x0,
    #[doc = "In HS mode."]
    HS = 0x01,
}
impl HSP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HSP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HSP {
    #[inline(always)]
    fn from(val: u8) -> HSP {
        HSP::from_bits(val)
    }
}
impl From<HSP> for u8 {
    #[inline(always)]
    fn from(val: HSP) -> u8 {
        HSP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HWDEVICE_DC {
    #[doc = "Not supported."]
    DEVICE_OP_DIS = 0x0,
    #[doc = "Supported."]
    DEVICE_OP_EN = 0x01,
}
impl HWDEVICE_DC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HWDEVICE_DC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HWDEVICE_DC {
    #[inline(always)]
    fn from(val: u8) -> HWDEVICE_DC {
        HWDEVICE_DC::from_bits(val)
    }
}
impl From<HWDEVICE_DC> for u8 {
    #[inline(always)]
    fn from(val: HWDEVICE_DC) -> u8 {
        HWDEVICE_DC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HWHOST_HC {
    #[doc = "Not supported."]
    HOST_OP_DIS = 0x0,
    #[doc = "Supported."]
    HOST_OP_EN = 0x01,
}
impl HWHOST_HC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HWHOST_HC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HWHOST_HC {
    #[inline(always)]
    fn from(val: u8) -> HWHOST_HC {
        HWHOST_HC::from_bits(val)
    }
}
impl From<HWHOST_HC> for u8 {
    #[inline(always)]
    fn from(val: HWHOST_HC) -> u8 {
        HWHOST_HC::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ITC(u8);
impl ITC {
    #[doc = "Immediate (no threshold)."]
    pub const IMMEDIATE: Self = Self(0x0);
    #[doc = "1 microframe."]
    pub const MICROFRAME_1: Self = Self(0x01);
    #[doc = "2 microframes."]
    pub const MICROFRAME_2: Self = Self(0x02);
    #[doc = "4 microframes."]
    pub const MICROFRAME_4: Self = Self(0x04);
    #[doc = "8 microframes."]
    pub const MICROFRAME_8: Self = Self(0x08);
    #[doc = "16 microframes."]
    pub const MICROFRAME_16: Self = Self(0x10);
    #[doc = "32 microframes."]
    pub const MICROFRAME_32: Self = Self(0x20);
    #[doc = "64 microframes."]
    pub const MICROFRAME_64: Self = Self(0x40);
}
impl ITC {
    pub const fn from_bits(val: u8) -> ITC {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for ITC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IMMEDIATE"),
            0x01 => f.write_str("MICROFRAME_1"),
            0x02 => f.write_str("MICROFRAME_2"),
            0x04 => f.write_str("MICROFRAME_4"),
            0x08 => f.write_str("MICROFRAME_8"),
            0x10 => f.write_str("MICROFRAME_16"),
            0x20 => f.write_str("MICROFRAME_32"),
            0x40 => f.write_str("MICROFRAME_64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ITC {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IMMEDIATE"),
            0x01 => defmt::write!(f, "MICROFRAME_1"),
            0x02 => defmt::write!(f, "MICROFRAME_2"),
            0x04 => defmt::write!(f, "MICROFRAME_4"),
            0x08 => defmt::write!(f, "MICROFRAME_8"),
            0x10 => defmt::write!(f, "MICROFRAME_16"),
            0x20 => defmt::write!(f, "MICROFRAME_32"),
            0x40 => defmt::write!(f, "MICROFRAME_64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for ITC {
    #[inline(always)]
    fn from(val: u8) -> ITC {
        ITC::from_bits(val)
    }
}
impl From<ITC> for u8 {
    #[inline(always)]
    fn from(val: ITC) -> u8 {
        ITC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM {
    #[doc = "Not supported."]
    LPM_NO = 0x0,
    #[doc = "Supported."]
    LPM_EN = 0x01,
}
impl LPM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM {
    #[inline(always)]
    fn from(val: u8) -> LPM {
        LPM::from_bits(val)
    }
}
impl From<LPM> for u8 {
    #[inline(always)]
    fn from(val: LPM) -> u8 {
        LPM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_RCVDI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl LPM_DEV_RCVDI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_RCVDI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_RCVDI {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_RCVDI {
        LPM_DEV_RCVDI::from_bits(val)
    }
}
impl From<LPM_DEV_RCVDI> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_RCVDI) -> u8 {
        LPM_DEV_RCVDI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_DEV_RCVDIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl LPM_DEV_RCVDIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_DEV_RCVDIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_DEV_RCVDIE {
    #[inline(always)]
    fn from(val: u8) -> LPM_DEV_RCVDIE {
        LPM_DEV_RCVDIE::from_bits(val)
    }
}
impl From<LPM_DEV_RCVDIE> for u8 {
    #[inline(always)]
    fn from(val: LPM_DEV_RCVDIE) -> u8 {
        LPM_DEV_RCVDIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_HST_COMPI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl LPM_HST_COMPI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_HST_COMPI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_HST_COMPI {
    #[inline(always)]
    fn from(val: u8) -> LPM_HST_COMPI {
        LPM_HST_COMPI::from_bits(val)
    }
}
impl From<LPM_HST_COMPI> for u8 {
    #[inline(always)]
    fn from(val: LPM_HST_COMPI) -> u8 {
        LPM_HST_COMPI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_HST_COMPIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl LPM_HST_COMPIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_HST_COMPIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_HST_COMPIE {
    #[inline(always)]
    fn from(val: u8) -> LPM_HST_COMPIE {
        LPM_HST_COMPIE::from_bits(val)
    }
}
impl From<LPM_HST_COMPIE> for u8 {
    #[inline(always)]
    fn from(val: LPM_HST_COMPIE) -> u8 {
        LPM_HST_COMPIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_L1_ENTRYI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl LPM_L1_ENTRYI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_L1_ENTRYI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_L1_ENTRYI {
    #[inline(always)]
    fn from(val: u8) -> LPM_L1_ENTRYI {
        LPM_L1_ENTRYI::from_bits(val)
    }
}
impl From<LPM_L1_ENTRYI> for u8 {
    #[inline(always)]
    fn from(val: LPM_L1_ENTRYI) -> u8 {
        LPM_L1_ENTRYI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_L1_ENTRYIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl LPM_L1_ENTRYIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_L1_ENTRYIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_L1_ENTRYIE {
    #[inline(always)]
    fn from(val: u8) -> LPM_L1_ENTRYIE {
        LPM_L1_ENTRYIE::from_bits(val)
    }
}
impl From<LPM_L1_ENTRYIE> for u8 {
    #[inline(always)]
    fn from(val: LPM_L1_ENTRYIE) -> u8 {
        LPM_L1_ENTRYIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_L1_EXITI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl LPM_L1_EXITI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_L1_EXITI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_L1_EXITI {
    #[inline(always)]
    fn from(val: u8) -> LPM_L1_EXITI {
        LPM_L1_EXITI::from_bits(val)
    }
}
impl From<LPM_L1_EXITI> for u8 {
    #[inline(always)]
    fn from(val: LPM_L1_EXITI) -> u8 {
        LPM_L1_EXITI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LPM_L1_EXITIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl LPM_L1_EXITIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LPM_L1_EXITIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LPM_L1_EXITIE {
    #[inline(always)]
    fn from(val: u8) -> LPM_L1_EXITIE {
        LPM_L1_EXITIE::from_bits(val)
    }
}
impl From<LPM_L1_EXITIE> for u8 {
    #[inline(always)]
    fn from(val: LPM_L1_EXITIE) -> u8 {
        LPM_L1_EXITIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LS {
    #[doc = "SE0."]
    SE0 = 0x0,
    #[doc = "K-state."]
    K_STATE = 0x01,
    #[doc = "J-state."]
    J_STATE = 0x02,
    #[doc = "Undefined."]
    UNDEFINED = 0x03,
}
impl LS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LS {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LS {
    #[inline(always)]
    fn from(val: u8) -> LS {
        LS::from_bits(val)
    }
}
impl From<LS> for u8 {
    #[inline(always)]
    fn from(val: LS) -> u8 {
        LS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NAKE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl NAKE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NAKE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NAKE {
    #[inline(always)]
    fn from(val: u8) -> NAKE {
        NAKE::from_bits(val)
    }
}
impl From<NAKE> for u8 {
    #[inline(always)]
    fn from(val: NAKE) -> u8 {
        NAKE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum N_CC {
    #[doc = "No internal companion controller exists."]
    NO_COMP_CONTROLLER = 0x0,
    #[doc = "Internal companion controllers exist."]
    COMP_CONTROLLER = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl N_CC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> N_CC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for N_CC {
    #[inline(always)]
    fn from(val: u8) -> N_CC {
        N_CC::from_bits(val)
    }
}
impl From<N_CC> for u8 {
    #[inline(always)]
    fn from(val: N_CC) -> u8 {
        N_CC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OCA {
    #[doc = "No overcurrent condition exists."]
    NO_OVERCURRENT = 0x0,
    #[doc = "Overcurrent condition exists."]
    OVERCURRENT = 0x01,
}
impl OCA {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OCA {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OCA {
    #[inline(always)]
    fn from(val: u8) -> OCA {
        OCA::from_bits(val)
    }
}
impl From<OCA> for u8 {
    #[inline(always)]
    fn from(val: OCA) -> u8 {
        OCA::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OCC {
    #[doc = "No change occurred."]
    NO_CHANGE = 0x0,
    #[doc = "Change occurred."]
    CHANGE = 0x01,
}
impl OCC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OCC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OCC {
    #[inline(always)]
    fn from(val: u8) -> OCC {
        OCC::from_bits(val)
    }
}
impl From<OCC> for u8 {
    #[inline(always)]
    fn from(val: OCC) -> u8 {
        OCC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OTG_ID {
    #[doc = "A device."]
    DEV_A = 0x0,
    #[doc = "B device."]
    DEV_B = 0x01,
}
impl OTG_ID {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OTG_ID {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OTG_ID {
    #[inline(always)]
    fn from(val: u8) -> OTG_ID {
        OTG_ID::from_bits(val)
    }
}
impl From<OTG_ID> for u8 {
    #[inline(always)]
    fn from(val: OTG_ID) -> u8 {
        OTG_ID::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PCE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl PCE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PCE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PCE {
    #[inline(always)]
    fn from(val: u8) -> PCE {
        PCE::from_bits(val)
    }
}
impl From<PCE> for u8 {
    #[inline(always)]
    fn from(val: PCE) -> u8 {
        PCE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PCI {
    #[doc = "Port change not detected."]
    DETECT_NO = 0x0,
    #[doc = "Port change detected."]
    DETECT_YES = 0x01,
}
impl PCI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PCI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PCI {
    #[inline(always)]
    fn from(val: u8) -> PCI {
        PCI::from_bits(val)
    }
}
impl From<PCI> for u8 {
    #[inline(always)]
    fn from(val: PCI) -> u8 {
        PCI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl PE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PE {
    #[inline(always)]
    fn from(val: u8) -> PE {
        PE::from_bits(val)
    }
}
impl From<PE> for u8 {
    #[inline(always)]
    fn from(val: PE) -> u8 {
        PE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PEC {
    #[doc = "No change occurred."]
    DISABLE = 0x0,
    #[doc = "Change occurred."]
    ENABLE = 0x01,
}
impl PEC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PEC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PEC {
    #[inline(always)]
    fn from(val: u8) -> PEC {
        PEC::from_bits(val)
    }
}
impl From<PEC> for u8 {
    #[inline(always)]
    fn from(val: PEC) -> u8 {
        PEC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PFSC {
    #[doc = "Normal operation."]
    NORMAL = 0x0,
    #[doc = "Forced to full speed."]
    FULL_SPEED = 0x01,
}
impl PFSC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PFSC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PFSC {
    #[inline(always)]
    fn from(val: u8) -> PFSC {
        PFSC::from_bits(val)
    }
}
impl From<PFSC> for u8 {
    #[inline(always)]
    fn from(val: PFSC) -> u8 {
        PFSC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PHCD {
    #[doc = "Enable."]
    PHY_CLK_EN = 0x0,
    #[doc = "Disable."]
    PHY_CLK_DIS = 0x01,
}
impl PHCD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PHCD {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PHCD {
    #[inline(always)]
    fn from(val: u8) -> PHCD {
        PHCD::from_bits(val)
    }
}
impl From<PHCD> for u8 {
    #[inline(always)]
    fn from(val: PHCD) -> u8 {
        PHCD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PHYM {
    #[doc = "UTMI/UMTI+."]
    UTMI = 0x0,
    #[doc = "ULPI DDR."]
    ULPI_DDR = 0x01,
    #[doc = "ULPI."]
    ULPI = 0x02,
    #[doc = "Serial only."]
    SERIAL = 0x03,
    #[doc = "Software programmable: reset to UTMI/UTMI+."]
    SW_RST_UTMI = 0x04,
    #[doc = "Software programmable: reset to ULPI DDR."]
    SW_RST_ULPI_DDR = 0x05,
    #[doc = "Software programmable: reset to ULPI."]
    SW_RST_ULPI = 0x06,
    #[doc = "Software programmable: reset to Serial."]
    SW_RST_SERIAL = 0x07,
    #[doc = "IC-USB."]
    ICUSB = 0x08,
    #[doc = "Software programmable: reset to IC-USB."]
    SW_RST_ICUSB = 0x09,
    #[doc = "HSIC."]
    HSIC = 0x0a,
    #[doc = "Software programmable: reset to HSIC."]
    SW_RST_HSIC = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PHYM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PHYM {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PHYM {
    #[inline(always)]
    fn from(val: u8) -> PHYM {
        PHYM::from_bits(val)
    }
}
impl From<PHYM> for u8 {
    #[inline(always)]
    fn from(val: PHYM) -> u8 {
        PHYM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PHYW {
    #[doc = "8-bit wide data bus (software nonprogrammable)."]
    DATA_BUS_8 = 0x0,
    #[doc = "16-bit wide data bus (software nonprogrammable)."]
    DATA_BUS_16 = 0x01,
    #[doc = "Reset to 8-bit wide data bus (software programmable)."]
    SW_RST_8 = 0x02,
    #[doc = "Reset to 16-bit wide data bus (software programmable)."]
    SW_RST_16 = 0x03,
}
impl PHYW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PHYW {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PHYW {
    #[inline(always)]
    fn from(val: u8) -> PHYW {
        PHYW::from_bits(val)
    }
}
impl From<PHYW> for u8 {
    #[inline(always)]
    fn from(val: PHYW) -> u8 {
        PHYW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PIC {
    #[doc = "Port indicators are off."]
    PORT_INDICATOR_OFF = 0x0,
    #[doc = "Amber."]
    PORT_IND_AMBER = 0x01,
    #[doc = "Green."]
    PORT_IND_GREEN = 0x02,
    #[doc = "Undefined."]
    UNDEFINED = 0x03,
}
impl PIC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PIC {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PIC {
    #[inline(always)]
    fn from(val: u8) -> PIC {
        PIC::from_bits(val)
    }
}
impl From<PIC> for u8 {
    #[inline(always)]
    fn from(val: PIC) -> u8 {
        PIC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PP {
    #[doc = "Off."]
    OFF = 0x0,
    #[doc = "On."]
    ON = 0x01,
}
impl PP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PP {
    #[inline(always)]
    fn from(val: u8) -> PP {
        PP::from_bits(val)
    }
}
impl From<PP> for u8 {
    #[inline(always)]
    fn from(val: PP) -> u8 {
        PP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PPC {
    #[doc = "No port power switches."]
    NO_SWITCHES = 0x0,
    #[doc = "Port power switches exist."]
    PORT_SWITCHES = 0x01,
}
impl PPC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PPC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PPC {
    #[inline(always)]
    fn from(val: u8) -> PPC {
        PPC::from_bits(val)
    }
}
impl From<PPC> for u8 {
    #[inline(always)]
    fn from(val: PPC) -> u8 {
        PPC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PR {
    #[doc = "Port not in reset."]
    DISABLE = 0x0,
    #[doc = "Port in reset."]
    ENABLE = 0x01,
}
impl PR {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PR {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PR {
    #[inline(always)]
    fn from(val: u8) -> PR {
        PR::from_bits(val)
    }
}
impl From<PR> for u8 {
    #[inline(always)]
    fn from(val: PR) -> u8 {
        PR::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PS {
    #[doc = "Disabled."]
    DISABLE = 0x0,
    #[doc = "Enabled."]
    ENABLE = 0x01,
}
impl PS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PS {
    #[inline(always)]
    fn from(val: u8) -> PS {
        PS::from_bits(val)
    }
}
impl From<PS> for u8 {
    #[inline(always)]
    fn from(val: PS) -> u8 {
        PS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PSE {
    #[doc = "Do not process the periodic schedule."]
    DONT_PROCESS_PT = 0x0,
    #[doc = "Process the periodic schedule."]
    PROCESS_PT_PERIODICLISTBASE = 0x01,
}
impl PSE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PSE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PSE {
    #[inline(always)]
    fn from(val: u8) -> PSE {
        PSE::from_bits(val)
    }
}
impl From<PSE> for u8 {
    #[inline(always)]
    fn from(val: PSE) -> u8 {
        PSE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PSPD {
    #[doc = "FS."]
    FS = 0x0,
    #[doc = "LS."]
    LS = 0x01,
    #[doc = "HS."]
    HS = 0x02,
    #[doc = "Undefined."]
    UNDEFINED = 0x03,
}
impl PSPD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PSPD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PSPD {
    #[inline(always)]
    fn from(val: u8) -> PSPD {
        PSPD::from_bits(val)
    }
}
impl From<PSPD> for u8 {
    #[inline(always)]
    fn from(val: PSPD) -> u8 {
        PSPD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PTC {
    #[doc = "TEST_MODE_DISABLE."]
    TST_MODE_DIS = 0x0,
    #[doc = "J_STATE."]
    J_STATE = 0x01,
    #[doc = "K_STATE."]
    K_STATE = 0x02,
    #[doc = "SE0 (host) or NAK (device)."]
    SE0 = 0x03,
    #[doc = "Packet."]
    PCKT = 0x04,
    #[doc = "FORCE_ENABLE_HS."]
    HS = 0x05,
    #[doc = "FORCE_ENABLE_FS."]
    FS = 0x06,
    #[doc = "FORCE_ENABLE_LS."]
    LS = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl PTC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PTC {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PTC {
    #[inline(always)]
    fn from(val: u8) -> PTC {
        PTC::from_bits(val)
    }
}
impl From<PTC> for u8 {
    #[inline(always)]
    fn from(val: PTC) -> u8 {
        PTC::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PTW {
    #[doc = "8-bit UTMI interface (60 MHz)."]
    UTMI_8 = 0x0,
    #[doc = "16-bit UTMI interface (30 MHz)."]
    UTMI_16 = 0x01,
}
impl PTW {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PTW {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PTW {
    #[inline(always)]
    fn from(val: u8) -> PTW {
        PTW::from_bits(val)
    }
}
impl From<PTW> for u8 {
    #[inline(always)]
    fn from(val: PTW) -> u8 {
        PTW::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RCL {
    #[doc = "Does not detect."]
    DISABLE = 0x0,
    #[doc = "Detects."]
    ENABLE = 0x01,
}
impl RCL {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RCL {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RCL {
    #[inline(always)]
    fn from(val: u8) -> RCL {
        RCL::from_bits(val)
    }
}
impl From<RCL> for u8 {
    #[inline(always)]
    fn from(val: RCL) -> u8 {
        RCL::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RS {
    #[doc = "Stopped executing."]
    STOP = 0x0,
    #[doc = "Running."]
    RUN = 0x01,
}
impl RS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RS {
    #[inline(always)]
    fn from(val: u8) -> RS {
        RS::from_bits(val)
    }
}
impl From<RS> for u8 {
    #[inline(always)]
    fn from(val: RS) -> u8 {
        RS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SDIS {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl SDIS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SDIS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SDIS {
    #[inline(always)]
    fn from(val: u8) -> SDIS {
        SDIS::from_bits(val)
    }
}
impl From<SDIS> for u8 {
    #[inline(always)]
    fn from(val: SDIS) -> u8 {
        SDIS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl SEE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEE {
    #[inline(always)]
    fn from(val: u8) -> SEE {
        SEE::from_bits(val)
    }
}
impl From<SEE> for u8 {
    #[inline(always)]
    fn from(val: SEE) -> u8 {
        SEE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SEI {
    #[doc = "Error response did not occur."]
    INT_NO = 0x0,
    #[doc = "Error response occurred."]
    INT_YES = 0x01,
}
impl SEI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SEI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SEI {
    #[inline(always)]
    fn from(val: u8) -> SEI {
        SEI::from_bits(val)
    }
}
impl From<SEI> for u8 {
    #[inline(always)]
    fn from(val: SEI) -> u8 {
        SEI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl SLE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLE {
    #[inline(always)]
    fn from(val: u8) -> SLE {
        SLE::from_bits(val)
    }
}
impl From<SLE> for u8 {
    #[inline(always)]
    fn from(val: SLE) -> u8 {
        SLE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLI {
    #[doc = "Did not enter Suspended state."]
    SUS_NO = 0x0,
    #[doc = "Entered Suspended state."]
    SUS_YES = 0x01,
}
impl SLI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLI {
    #[inline(always)]
    fn from(val: u8) -> SLI {
        SLI::from_bits(val)
    }
}
impl From<SLI> for u8 {
    #[inline(always)]
    fn from(val: SLI) -> u8 {
        SLI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SLOM {
    #[doc = "On (default)."]
    LOCKOUT_ON = 0x0,
    #[doc = "Off."]
    LOCKOUT_OFF = 0x01,
}
impl SLOM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SLOM {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SLOM {
    #[inline(always)]
    fn from(val: u8) -> SLOM {
        SLOM::from_bits(val)
    }
}
impl From<SLOM> for u8 {
    #[inline(always)]
    fn from(val: SLOM) -> u8 {
        SLOM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SM {
    #[doc = "No serial engine; always use parallel signaling."]
    SERIAL_ENGINE_NO = 0x0,
    #[doc = "Serial engine present; always use serial signaling for FS and LS."]
    SERIAL_ENGINE_EN = 0x01,
    #[doc = "Software programmable; reset to use parallel signaling for FS and LS."]
    SW_RST_PARALLEL = 0x02,
    #[doc = "Software programmable; reset to use serial signaling for FS and LS."]
    SW_RST_SERIAL_ENG = 0x03,
}
impl SM {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SM {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SM {
    #[inline(always)]
    fn from(val: u8) -> SM {
        SM::from_bits(val)
    }
}
impl From<SM> for u8 {
    #[inline(always)]
    fn from(val: SM) -> u8 {
        SM::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl SRE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRE {
    #[inline(always)]
    fn from(val: u8) -> SRE {
        SRE::from_bits(val)
    }
}
impl From<SRE> for u8 {
    #[inline(always)]
    fn from(val: SRE) -> u8 {
        SRE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SRI {
    #[doc = "SOF not received."]
    SOF_NO = 0x0,
    #[doc = "SOF received."]
    SOF_YES = 0x01,
}
impl SRI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SRI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SRI {
    #[inline(always)]
    fn from(val: u8) -> SRI {
        SRI::from_bits(val)
    }
}
impl From<SRI> for u8 {
    #[inline(always)]
    fn from(val: SRI) -> u8 {
        SRI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum STS {
    #[doc = "Parallel interface signals."]
    DISABLE = 0x0,
    #[doc = "Serial interface engine."]
    ENABLE = 0x01,
}
impl STS {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> STS {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for STS {
    #[inline(always)]
    fn from(val: u8) -> STS {
        STS::from_bits(val)
    }
}
impl From<STS> for u8 {
    #[inline(always)]
    fn from(val: STS) -> u8 {
        STS::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SUSP {
    #[doc = "Port not in Suspended state."]
    DISABLE = 0x0,
    #[doc = "Port in Suspended state."]
    ENABLE = 0x01,
}
impl SUSP {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SUSP {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SUSP {
    #[inline(always)]
    fn from(val: u8) -> SUSP {
        SUSP::from_bits(val)
    }
}
impl From<SUSP> for u8 {
    #[inline(always)]
    fn from(val: SUSP) -> u8 {
        SUSP::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TI0 {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl TI0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TI0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TI0 {
    #[inline(always)]
    fn from(val: u8) -> TI0 {
        TI0::from_bits(val)
    }
}
impl From<TI0> for u8 {
    #[inline(always)]
    fn from(val: TI0) -> u8 {
        TI0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TI1 {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl TI1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TI1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TI1 {
    #[inline(always)]
    fn from(val: u8) -> TI1 {
        TI1::from_bits(val)
    }
}
impl From<TI1> for u8 {
    #[inline(always)]
    fn from(val: TI1) -> u8 {
        TI1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIE0 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl TIE0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIE0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIE0 {
    #[inline(always)]
    fn from(val: u8) -> TIE0 {
        TIE0::from_bits(val)
    }
}
impl From<TIE0> for u8 {
    #[inline(always)]
    fn from(val: TIE0) -> u8 {
        TIE0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TIE1 {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl TIE1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TIE1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TIE1 {
    #[inline(always)]
    fn from(val: u8) -> TIE1 {
        TIE1::from_bits(val)
    }
}
impl From<TIE1> for u8 {
    #[inline(always)]
    fn from(val: TIE1) -> u8 {
        TIE1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UAI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl UAI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UAI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UAI {
    #[inline(always)]
    fn from(val: u8) -> UAI {
        UAI::from_bits(val)
    }
}
impl From<UAI> for u8 {
    #[inline(always)]
    fn from(val: UAI) -> u8 {
        UAI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UAIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl UAIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UAIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UAIE {
    #[inline(always)]
    fn from(val: u8) -> UAIE {
        UAIE::from_bits(val)
    }
}
impl From<UAIE> for u8 {
    #[inline(always)]
    fn from(val: UAIE) -> u8 {
        UAIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl UE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UE {
    #[inline(always)]
    fn from(val: u8) -> UE {
        UE::from_bits(val)
    }
}
impl From<UE> for u8 {
    #[inline(always)]
    fn from(val: UE) -> u8 {
        UE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UEE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl UEE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UEE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UEE {
    #[inline(always)]
    fn from(val: u8) -> UEE {
        UEE::from_bits(val)
    }
}
impl From<UEE> for u8 {
    #[inline(always)]
    fn from(val: UEE) -> u8 {
        UEE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UEI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl UEI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UEI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UEI {
    #[inline(always)]
    fn from(val: u8) -> UEI {
        UEI::from_bits(val)
    }
}
impl From<UEI> for u8 {
    #[inline(always)]
    fn from(val: UEI) -> u8 {
        UEI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl UI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UI {
    #[inline(always)]
    fn from(val: u8) -> UI {
        UI::from_bits(val)
    }
}
impl From<UI> for u8 {
    #[inline(always)]
    fn from(val: UI) -> u8 {
        UI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ULPII {
    #[doc = "Event completion did not occur."]
    EVENT_NO = 0x0,
    #[doc = "Event completion occurred."]
    EVENT_YES = 0x01,
}
impl ULPII {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ULPII {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ULPII {
    #[inline(always)]
    fn from(val: u8) -> ULPII {
        ULPII::from_bits(val)
    }
}
impl From<ULPII> for u8 {
    #[inline(always)]
    fn from(val: ULPII) -> u8 {
        ULPII::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UPI {
    #[doc = "Interrupt did not occur."]
    INT_NO = 0x0,
    #[doc = "Interrupt occurred."]
    INT_YES = 0x01,
}
impl UPI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UPI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UPI {
    #[inline(always)]
    fn from(val: u8) -> UPI {
        UPI::from_bits(val)
    }
}
impl From<UPI> for u8 {
    #[inline(always)]
    fn from(val: UPI) -> u8 {
        UPI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UPIE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl UPIE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UPIE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UPIE {
    #[inline(always)]
    fn from(val: u8) -> UPIE {
        UPIE::from_bits(val)
    }
}
impl From<UPIE> for u8 {
    #[inline(always)]
    fn from(val: UPIE) -> u8 {
        UPIE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum URE {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl URE {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> URE {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for URE {
    #[inline(always)]
    fn from(val: u8) -> URE {
        URE::from_bits(val)
    }
}
impl From<URE> for u8 {
    #[inline(always)]
    fn from(val: URE) -> u8 {
        URE::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum URI {
    #[doc = "USB reset not received."]
    USB_NO = 0x0,
    #[doc = "USB reset received."]
    USB_YES = 0x01,
}
impl URI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> URI {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for URI {
    #[inline(always)]
    fn from(val: u8) -> URI {
        URI::from_bits(val)
    }
}
impl From<URI> for u8 {
    #[inline(always)]
    fn from(val: URI) -> u8 {
        URI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKCN {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl WKCN {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKCN {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKCN {
    #[inline(always)]
    fn from(val: u8) -> WKCN {
        WKCN::from_bits(val)
    }
}
impl From<WKCN> for u8 {
    #[inline(always)]
    fn from(val: WKCN) -> u8 {
        WKCN::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WKDC {
    #[doc = "Disable."]
    DISABLE = 0x0,
    #[doc = "Enable."]
    ENABLE = 0x01,
}
impl WKDC {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WKDC {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WKDC {
    #[inline(always)]
    fn from(val: u8) -> WKDC {
        WKDC::from_bits(val)
    }
}
impl From<WKDC> for u8 {
    #[inline(always)]
    fn from(val: WKDC) -> u8 {
        WKDC::to_bits(val)
    }
}
