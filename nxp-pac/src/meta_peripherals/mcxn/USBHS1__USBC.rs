#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "USB."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs1Usbc {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs1Usbc {}
unsafe impl Sync for Usbhs1Usbc {}
impl Usbhs1Usbc {
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
    pub const fn id(self) -> crate::pac::common::Reg<Id, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Hardware General."]
    #[inline(always)]
    pub const fn hwgeneral(self) -> crate::pac::common::Reg<Hwgeneral, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Host Hardware Parameters."]
    #[inline(always)]
    pub const fn hwhost(self) -> crate::pac::common::Reg<Hwhost, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Device Hardware Parameters."]
    #[inline(always)]
    pub const fn hwdevice(self) -> crate::pac::common::Reg<Hwdevice, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Buffer Hardware Parameters."]
    #[inline(always)]
    pub const fn hwtxbuf(self) -> crate::pac::common::Reg<Hwtxbuf, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RX Buffer Hardware Parameters."]
    #[inline(always)]
    pub const fn hwrxbuf(self) -> crate::pac::common::Reg<Hwrxbuf, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Load."]
    #[inline(always)]
    pub const fn gptimer0ld(self) -> crate::pac::common::Reg<Gptimer0ld, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "General Purpose Timer #0 Controller."]
    #[inline(always)]
    pub const fn gptimer0ctrl(
        self,
    ) -> crate::pac::common::Reg<Gptimer0ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Load."]
    #[inline(always)]
    pub const fn gptimer1ld(self) -> crate::pac::common::Reg<Gptimer1ld, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "General Purpose Timer #1 Controller."]
    #[inline(always)]
    pub const fn gptimer1ctrl(
        self,
    ) -> crate::pac::common::Reg<Gptimer1ctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "System Bus Config."]
    #[inline(always)]
    pub const fn sbuscfg(self) -> crate::pac::common::Reg<Sbuscfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Capability Registers Length."]
    #[inline(always)]
    pub const fn caplength(self) -> crate::pac::common::Reg<Caplength, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Host Controller Interface Version."]
    #[inline(always)]
    pub const fn hciversion(self) -> crate::pac::common::Reg<Hciversion, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Host Controller Structural Parameters."]
    #[inline(always)]
    pub const fn hcsparams(self) -> crate::pac::common::Reg<Hcsparams, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Host Controller Capability Parameters."]
    #[inline(always)]
    pub const fn hccparams(self) -> crate::pac::common::Reg<Hccparams, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Device Controller Interface Version."]
    #[inline(always)]
    pub const fn dciversion(self) -> crate::pac::common::Reg<Dciversion, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Device Controller Capability Parameters."]
    #[inline(always)]
    pub const fn dccparams(self) -> crate::pac::common::Reg<Dccparams, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "USB Command."]
    #[inline(always)]
    pub const fn usbcmd(self) -> crate::pac::common::Reg<Usbcmd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "USB Status."]
    #[inline(always)]
    pub const fn usbsts(self) -> crate::pac::common::Reg<Usbsts, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn usbintr(self) -> crate::pac::common::Reg<Usbintr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "USB Frame Index."]
    #[inline(always)]
    pub const fn frindex(self) -> crate::pac::common::Reg<Frindex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn deviceaddr(self) -> crate::pac::common::Reg<Deviceaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Frame List Base Address."]
    #[inline(always)]
    pub const fn periodiclistbase(
        self,
    ) -> crate::pac::common::Reg<Periodiclistbase, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Next Asynch. Address."]
    #[inline(always)]
    pub const fn asynclistaddr(
        self,
    ) -> crate::pac::common::Reg<Asynclistaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Endpoint List Address."]
    #[inline(always)]
    pub const fn endptlistaddr(
        self,
    ) -> crate::pac::common::Reg<Endptlistaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Programmable Burst Size."]
    #[inline(always)]
    pub const fn burstsize(self) -> crate::pac::common::Reg<Burstsize, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "TX FIFO Fill Tuning."]
    #[inline(always)]
    pub const fn txfilltuning(
        self,
    ) -> crate::pac::common::Reg<Txfilltuning, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Endpoint NAK."]
    #[inline(always)]
    pub const fn endptnak(self) -> crate::pac::common::Reg<Endptnak, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn endptnaken(self) -> crate::pac::common::Reg<Endptnaken, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "Configure Flag."]
    #[inline(always)]
    pub const fn configflag(self) -> crate::pac::common::Reg<Configflag, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Port Status & Control."]
    #[inline(always)]
    pub const fn portsc1(self) -> crate::pac::common::Reg<Portsc1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "On-The-Go Status & Control."]
    #[inline(always)]
    pub const fn otgsc(self) -> crate::pac::common::Reg<Otgsc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "USB Device Mode."]
    #[inline(always)]
    pub const fn usbmode(self) -> crate::pac::common::Reg<Usbmode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "Endpoint Setup Status."]
    #[inline(always)]
    pub const fn endptsetupstat(
        self,
    ) -> crate::pac::common::Reg<Endptsetupstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "Endpoint Prime."]
    #[inline(always)]
    pub const fn endptprime(self) -> crate::pac::common::Reg<Endptprime, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Endpoint Flush."]
    #[inline(always)]
    pub const fn endptflush(self) -> crate::pac::common::Reg<Endptflush, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "Endpoint Status."]
    #[inline(always)]
    pub const fn endptstat(self) -> crate::pac::common::Reg<Endptstat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "Endpoint Complete."]
    #[inline(always)]
    pub const fn endptcomplete(
        self,
    ) -> crate::pac::common::Reg<Endptcomplete, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "Endpoint Control 0."]
    #[inline(always)]
    pub const fn endptctrl0(self) -> crate::pac::common::Reg<Endptctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "Endpoint Control."]
    #[inline(always)]
    pub const fn endptctrl(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Endptctrl, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize + n * 4usize) as _)
        }
    }
}
#[doc = "Next Asynch. Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asynclistaddr(pub u32);
impl Asynclistaddr {
    #[doc = "Link Pointer Low (LPL)."]
    #[must_use]
    #[inline(always)]
    pub const fn asybase(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Link Pointer Low (LPL)."]
    #[inline(always)]
    pub const fn set_asybase(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for Asynclistaddr {
    #[inline(always)]
    fn default() -> Asynclistaddr {
        Asynclistaddr(0)
    }
}
impl core::fmt::Debug for Asynclistaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Asynclistaddr")
            .field("asybase", &self.asybase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Asynclistaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Asynclistaddr {{ asybase: {=u32:?} }}", self.asybase())
    }
}
#[doc = "Programmable Burst Size."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Burstsize(pub u32);
impl Burstsize {
    #[doc = "Programmable RX Burst Size."]
    #[must_use]
    #[inline(always)]
    pub const fn rxpburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable RX Burst Size."]
    #[inline(always)]
    pub const fn set_rxpburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Programmable TX Burst Size."]
    #[must_use]
    #[inline(always)]
    pub const fn txpburst(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable TX Burst Size."]
    #[inline(always)]
    pub const fn set_txpburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Burstsize {
    #[inline(always)]
    fn default() -> Burstsize {
        Burstsize(0)
    }
}
impl core::fmt::Debug for Burstsize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Burstsize")
            .field("rxpburst", &self.rxpburst())
            .field("txpburst", &self.txpburst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Burstsize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Burstsize {{ rxpburst: {=u8:?}, txpburst: {=u8:?} }}",
            self.rxpburst(),
            self.txpburst()
        )
    }
}
#[doc = "Capability Registers Length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caplength(pub u8);
impl Caplength {
    #[doc = "These bits are used as an offset to add to register base to find the beginning of the Operational Register. Default value is '40h'."]
    #[must_use]
    #[inline(always)]
    pub const fn caplength(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are used as an offset to add to register base to find the beginning of the Operational Register. Default value is '40h'."]
    #[inline(always)]
    pub const fn set_caplength(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
    }
}
impl Default for Caplength {
    #[inline(always)]
    fn default() -> Caplength {
        Caplength(0)
    }
}
impl core::fmt::Debug for Caplength {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Caplength")
            .field("caplength", &self.caplength())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Caplength {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Caplength {{ caplength: {=u8:?} }}", self.caplength())
    }
}
#[doc = "Configure Flag."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configflag(pub u32);
impl Configflag {
    #[doc = "Configure Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cf(&self) -> Cf {
        let val = (self.0 >> 0usize) & 0x01;
        Cf::from_bits(val as u8)
    }
    #[doc = "Configure Flag."]
    #[inline(always)]
    pub const fn set_cf(&mut self, val: Cf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Configflag {
    #[inline(always)]
    fn default() -> Configflag {
        Configflag(0)
    }
}
impl core::fmt::Debug for Configflag {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Configflag")
            .field("cf", &self.cf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Configflag {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Configflag {{ cf: {:?} }}", self.cf())
    }
}
#[doc = "Device Controller Capability Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccparams(pub u32);
impl Dccparams {
    #[doc = "Device Endpoint Number."]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number."]
    #[inline(always)]
    pub const fn set_den(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Device Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Device Capable."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Host Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Host Capable."]
    #[inline(always)]
    pub const fn set_hc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Dccparams {
    #[inline(always)]
    fn default() -> Dccparams {
        Dccparams(0)
    }
}
impl core::fmt::Debug for Dccparams {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dccparams")
            .field("den", &self.den())
            .field("dc", &self.dc())
            .field("hc", &self.hc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dccparams {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dccparams {{ den: {=u8:?}, dc: {=bool:?}, hc: {=bool:?} }}",
            self.den(),
            self.dc(),
            self.hc()
        )
    }
}
#[doc = "Device Controller Interface Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dciversion(pub u16);
impl Dciversion {
    #[doc = "Device Controller Interface Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn dciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device Controller Interface Version Number."]
    #[inline(always)]
    pub const fn set_dciversion(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Dciversion {
    #[inline(always)]
    fn default() -> Dciversion {
        Dciversion(0)
    }
}
impl core::fmt::Debug for Dciversion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dciversion")
            .field("dciversion", &self.dciversion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dciversion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dciversion {{ dciversion: {=u16:?} }}",
            self.dciversion()
        )
    }
}
#[doc = "Device Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deviceaddr(pub u32);
impl Deviceaddr {
    #[doc = "Device Address Advance."]
    #[must_use]
    #[inline(always)]
    pub const fn usbadra(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device Address Advance."]
    #[inline(always)]
    pub const fn set_usbadra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device Address."]
    #[must_use]
    #[inline(always)]
    pub const fn usbadr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address."]
    #[inline(always)]
    pub const fn set_usbadr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Deviceaddr {
    #[inline(always)]
    fn default() -> Deviceaddr {
        Deviceaddr(0)
    }
}
impl core::fmt::Debug for Deviceaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Deviceaddr")
            .field("usbadra", &self.usbadra())
            .field("usbadr", &self.usbadr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Deviceaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Deviceaddr {{ usbadra: {=bool:?}, usbadr: {=u8:?} }}",
            self.usbadra(),
            self.usbadr()
        )
    }
}
#[doc = "Endpoint Complete."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptcomplete(pub u32);
impl Endptcomplete {
    #[doc = "Endpoint Receive Complete Event."]
    #[must_use]
    #[inline(always)]
    pub const fn erce(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Complete Event."]
    #[inline(always)]
    pub const fn set_erce(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Complete Event."]
    #[must_use]
    #[inline(always)]
    pub const fn etce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Complete Event."]
    #[inline(always)]
    pub const fn set_etce(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptcomplete {
    #[inline(always)]
    fn default() -> Endptcomplete {
        Endptcomplete(0)
    }
}
impl core::fmt::Debug for Endptcomplete {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptcomplete")
            .field("erce", &self.erce())
            .field("etce", &self.etce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptcomplete {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptcomplete {{ erce: {=u8:?}, etce: {=u8:?} }}",
            self.erce(),
            self.etce()
        )
    }
}
#[doc = "Endpoint Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl(pub u32);
impl Endptctrl {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink."]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink."]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> EndptctrlRxt {
        let val = (self.0 >> 2usize) & 0x03;
        EndptctrlRxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: EndptctrlRxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)."]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source."]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> EndptctrlTxt {
        let val = (self.0 >> 18usize) & 0x03;
        EndptctrlTxt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: EndptctrlTxt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> EndptctrlTxi {
        let val = (self.0 >> 21usize) & 0x01;
        EndptctrlTxi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit."]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: EndptctrlTxi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)."]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)."]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl {
    #[inline(always)]
    fn default() -> Endptctrl {
        Endptctrl(0)
    }
}
impl core::fmt::Debug for Endptctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl")
            .field("rxs", &self.rxs())
            .field("rxd", &self.rxd())
            .field("rxt", &self.rxt())
            .field("rxi", &self.rxi())
            .field("rxr", &self.rxr())
            .field("rxe", &self.rxe())
            .field("txs", &self.txs())
            .field("txd", &self.txd())
            .field("txt", &self.txt())
            .field("txi", &self.txi())
            .field("txr", &self.txr())
            .field("txe", &self.txe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
            self.rxs(),
            self.rxd(),
            self.rxt(),
            self.rxi(),
            self.rxr(),
            self.rxe(),
            self.txs(),
            self.txd(),
            self.txt(),
            self.txi(),
            self.txr(),
            self.txe()
        )
    }
}
#[doc = "Endpoint Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl0(pub u32);
impl Endptctrl0 {
    #[doc = "RX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> Endptctrl0Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        Endptctrl0Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type."]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: Endptctrl0Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall."]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall."]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Type."]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> Endptctrl0Txt {
        let val = (self.0 >> 18usize) & 0x03;
        Endptctrl0Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type."]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: Endptctrl0Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Endpoint Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl0 {
    #[inline(always)]
    fn default() -> Endptctrl0 {
        Endptctrl0(0)
    }
}
impl core::fmt::Debug for Endptctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl0")
            .field("rxs", &self.rxs())
            .field("rxt", &self.rxt())
            .field("rxe", &self.rxe())
            .field("txs", &self.txs())
            .field("txt", &self.txt())
            .field("txe", &self.txe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl0 {{ rxs: {=bool:?}, rxt: {:?}, rxe: {=bool:?}, txs: {=bool:?}, txt: {:?}, txe: {=bool:?} }}",
            self.rxs(),
            self.rxt(),
            self.rxe(),
            self.txs(),
            self.txt(),
            self.txe()
        )
    }
}
#[doc = "Endpoint Flush."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptflush(pub u32);
impl Endptflush {
    #[doc = "Flush Endpoint Receive Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn ferb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Receive Buffer."]
    #[inline(always)]
    pub const fn set_ferb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Flush Endpoint Transmit Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn fetb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Transmit Buffer."]
    #[inline(always)]
    pub const fn set_fetb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptflush {
    #[inline(always)]
    fn default() -> Endptflush {
        Endptflush(0)
    }
}
impl core::fmt::Debug for Endptflush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptflush")
            .field("ferb", &self.ferb())
            .field("fetb", &self.fetb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptflush {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptflush {{ ferb: {=u8:?}, fetb: {=u8:?} }}",
            self.ferb(),
            self.fetb()
        )
    }
}
#[doc = "Endpoint List Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptlistaddr(pub u32);
impl Endptlistaddr {
    #[doc = "Endpoint List Pointer (Low)."]
    #[must_use]
    #[inline(always)]
    pub const fn epbase(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Endpoint List Pointer (Low)."]
    #[inline(always)]
    pub const fn set_epbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 11usize)) | (((val as u32) & 0x001f_ffff) << 11usize);
    }
}
impl Default for Endptlistaddr {
    #[inline(always)]
    fn default() -> Endptlistaddr {
        Endptlistaddr(0)
    }
}
impl core::fmt::Debug for Endptlistaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptlistaddr")
            .field("epbase", &self.epbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptlistaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Endptlistaddr {{ epbase: {=u32:?} }}", self.epbase())
    }
}
#[doc = "Endpoint NAK."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptnak(pub u32);
impl Endptnak {
    #[doc = "RX Endpoint NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn eprn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK."]
    #[inline(always)]
    pub const fn set_eprn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK."]
    #[must_use]
    #[inline(always)]
    pub const fn eptn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK."]
    #[inline(always)]
    pub const fn set_eptn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptnak {
    #[inline(always)]
    fn default() -> Endptnak {
        Endptnak(0)
    }
}
impl core::fmt::Debug for Endptnak {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptnak")
            .field("eprn", &self.eprn())
            .field("eptn", &self.eptn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptnak {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptnak {{ eprn: {=u8:?}, eptn: {=u8:?} }}",
            self.eprn(),
            self.eptn()
        )
    }
}
#[doc = "Endpoint NAK Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptnaken(pub u32);
impl Endptnaken {
    #[doc = "RX Endpoint NAK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eprne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn set_eprne(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn eptne(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK Enable."]
    #[inline(always)]
    pub const fn set_eptne(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptnaken {
    #[inline(always)]
    fn default() -> Endptnaken {
        Endptnaken(0)
    }
}
impl core::fmt::Debug for Endptnaken {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptnaken")
            .field("eprne", &self.eprne())
            .field("eptne", &self.eptne())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptnaken {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptnaken {{ eprne: {=u8:?}, eptne: {=u8:?} }}",
            self.eprne(),
            self.eptne()
        )
    }
}
#[doc = "Endpoint Prime."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptprime(pub u32);
impl Endptprime {
    #[doc = "Prime Endpoint Receive Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn perb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Receive Buffer."]
    #[inline(always)]
    pub const fn set_perb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Prime Endpoint Transmit Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn petb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Transmit Buffer."]
    #[inline(always)]
    pub const fn set_petb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptprime {
    #[inline(always)]
    fn default() -> Endptprime {
        Endptprime(0)
    }
}
impl core::fmt::Debug for Endptprime {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptprime")
            .field("perb", &self.perb())
            .field("petb", &self.petb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptprime {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptprime {{ perb: {=u8:?}, petb: {=u8:?} }}",
            self.perb(),
            self.petb()
        )
    }
}
#[doc = "Endpoint Setup Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptsetupstat(pub u32);
impl Endptsetupstat {
    #[doc = "Setup Endpoint Status."]
    #[must_use]
    #[inline(always)]
    pub const fn endptsetupstat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Setup Endpoint Status."]
    #[inline(always)]
    pub const fn set_endptsetupstat(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Endptsetupstat {
    #[inline(always)]
    fn default() -> Endptsetupstat {
        Endptsetupstat(0)
    }
}
impl core::fmt::Debug for Endptsetupstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptsetupstat")
            .field("endptsetupstat", &self.endptsetupstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptsetupstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptsetupstat {{ endptsetupstat: {=u16:?} }}",
            self.endptsetupstat()
        )
    }
}
#[doc = "Endpoint Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptstat(pub u32);
impl Endptstat {
    #[doc = "Endpoint Receive Buffer Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn erbr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Buffer Ready."]
    #[inline(always)]
    pub const fn set_erbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Buffer Ready."]
    #[must_use]
    #[inline(always)]
    pub const fn etbr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Buffer Ready."]
    #[inline(always)]
    pub const fn set_etbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Endptstat {
    #[inline(always)]
    fn default() -> Endptstat {
        Endptstat(0)
    }
}
impl core::fmt::Debug for Endptstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptstat")
            .field("erbr", &self.erbr())
            .field("etbr", &self.etbr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Endptstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptstat {{ erbr: {=u8:?}, etbr: {=u8:?} }}",
            self.erbr(),
            self.etbr()
        )
    }
}
#[doc = "USB Frame Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frindex(pub u32);
impl Frindex {
    #[doc = "Frame Index."]
    #[must_use]
    #[inline(always)]
    pub const fn frindex(&self) -> FrindexVal {
        let val = (self.0 >> 0usize) & 0x3fff;
        FrindexVal::from_bits(val as u16)
    }
    #[doc = "Frame Index."]
    #[inline(always)]
    pub const fn set_frindex(&mut self, val: FrindexVal) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val.to_bits() as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Frindex {
    #[inline(always)]
    fn default() -> Frindex {
        Frindex(0)
    }
}
impl core::fmt::Debug for Frindex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frindex")
            .field("frindex", &self.frindex())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frindex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frindex {{ frindex: {:?} }}", self.frindex())
    }
}
#[doc = "General Purpose Timer #0 Controller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer0ctrl(pub u32);
impl Gptimer0ctrl {
    #[doc = "General Purpose Timer Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter."]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> Gptimer0ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        Gptimer0ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode."]
    #[inline(always)]
    pub const fn set_gptmode(&mut self, val: Gptimer0ctrlGptmode) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrst(&self) -> Gptimer0ctrlGptrst {
        let val = (self.0 >> 30usize) & 0x01;
        Gptimer0ctrlGptrst::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset."]
    #[inline(always)]
    pub const fn set_gptrst(&mut self, val: Gptimer0ctrlGptrst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> Gptimer0ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        Gptimer0ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run."]
    #[inline(always)]
    pub const fn set_gptrun(&mut self, val: Gptimer0ctrlGptrun) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gptimer0ctrl {
    #[inline(always)]
    fn default() -> Gptimer0ctrl {
        Gptimer0ctrl(0)
    }
}
impl core::fmt::Debug for Gptimer0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptimer0ctrl")
            .field("gptcnt", &self.gptcnt())
            .field("gptmode", &self.gptmode())
            .field("gptrst", &self.gptrst())
            .field("gptrun", &self.gptrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptimer0ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gptimer0ctrl {{ gptcnt: {=u32:?}, gptmode: {:?}, gptrst: {:?}, gptrun: {:?} }}",
            self.gptcnt(),
            self.gptmode(),
            self.gptrst(),
            self.gptrun()
        )
    }
}
#[doc = "General Purpose Timer #0 Load."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer0ld(pub u32);
impl Gptimer0ld {
    #[doc = "General Purpose Timer Load Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value."]
    #[inline(always)]
    pub const fn set_gptld(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Gptimer0ld {
    #[inline(always)]
    fn default() -> Gptimer0ld {
        Gptimer0ld(0)
    }
}
impl core::fmt::Debug for Gptimer0ld {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptimer0ld")
            .field("gptld", &self.gptld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptimer0ld {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gptimer0ld {{ gptld: {=u32:?} }}", self.gptld())
    }
}
#[doc = "General Purpose Timer #1 Controller."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer1ctrl(pub u32);
impl Gptimer1ctrl {
    #[doc = "General Purpose Timer Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter."]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> Gptimer1ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        Gptimer1ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode."]
    #[inline(always)]
    pub const fn set_gptmode(&mut self, val: Gptimer1ctrlGptmode) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrst(&self) -> Gptimer1ctrlGptrst {
        let val = (self.0 >> 30usize) & 0x01;
        Gptimer1ctrlGptrst::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset."]
    #[inline(always)]
    pub const fn set_gptrst(&mut self, val: Gptimer1ctrlGptrst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> Gptimer1ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        Gptimer1ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run."]
    #[inline(always)]
    pub const fn set_gptrun(&mut self, val: Gptimer1ctrlGptrun) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gptimer1ctrl {
    #[inline(always)]
    fn default() -> Gptimer1ctrl {
        Gptimer1ctrl(0)
    }
}
impl core::fmt::Debug for Gptimer1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptimer1ctrl")
            .field("gptcnt", &self.gptcnt())
            .field("gptmode", &self.gptmode())
            .field("gptrst", &self.gptrst())
            .field("gptrun", &self.gptrun())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptimer1ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gptimer1ctrl {{ gptcnt: {=u32:?}, gptmode: {:?}, gptrst: {:?}, gptrun: {:?} }}",
            self.gptcnt(),
            self.gptmode(),
            self.gptrst(),
            self.gptrun()
        )
    }
}
#[doc = "General Purpose Timer #1 Load."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer1ld(pub u32);
impl Gptimer1ld {
    #[doc = "General Purpose Timer Load Value."]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value."]
    #[inline(always)]
    pub const fn set_gptld(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Gptimer1ld {
    #[inline(always)]
    fn default() -> Gptimer1ld {
        Gptimer1ld(0)
    }
}
impl core::fmt::Debug for Gptimer1ld {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gptimer1ld")
            .field("gptld", &self.gptld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gptimer1ld {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gptimer1ld {{ gptld: {=u32:?} }}", self.gptld())
    }
}
#[doc = "Host Controller Capability Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccparams(pub u32);
impl Hccparams {
    #[doc = "64-bit Addressing Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "64-bit Addressing Capability."]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Programmable Frame List Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Frame List Flag."]
    #[inline(always)]
    pub const fn set_pfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous Schedule Park Capability."]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Capability."]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Isochronous Scheduling Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn ist(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Isochronous Scheduling Threshold."]
    #[inline(always)]
    pub const fn set_ist(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EHCI Extended Capabilities Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn eecp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "EHCI Extended Capabilities Pointer."]
    #[inline(always)]
    pub const fn set_eecp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Hccparams {
    #[inline(always)]
    fn default() -> Hccparams {
        Hccparams(0)
    }
}
impl core::fmt::Debug for Hccparams {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hccparams")
            .field("adc", &self.adc())
            .field("pfl", &self.pfl())
            .field("asp", &self.asp())
            .field("ist", &self.ist())
            .field("eecp", &self.eecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hccparams {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hccparams {{ adc: {=bool:?}, pfl: {=bool:?}, asp: {=bool:?}, ist: {=u8:?}, eecp: {=u8:?} }}",
            self.adc(),
            self.pfl(),
            self.asp(),
            self.ist(),
            self.eecp()
        )
    }
}
#[doc = "Host Controller Interface Version."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hciversion(pub u16);
impl Hciversion {
    #[doc = "Host Controller Interface Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn hciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Host Controller Interface Version Number."]
    #[inline(always)]
    pub const fn set_hciversion(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Hciversion {
    #[inline(always)]
    fn default() -> Hciversion {
        Hciversion(0)
    }
}
impl core::fmt::Debug for Hciversion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hciversion")
            .field("hciversion", &self.hciversion())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hciversion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hciversion {{ hciversion: {=u16:?} }}",
            self.hciversion()
        )
    }
}
#[doc = "Host Controller Structural Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcsparams(pub u32);
impl Hcsparams {
    #[doc = "Number of Downstream Ports."]
    #[must_use]
    #[inline(always)]
    pub const fn n_ports(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Downstream Ports."]
    #[inline(always)]
    pub const fn set_n_ports(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Power Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ppc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power Control."]
    #[inline(always)]
    pub const fn set_ppc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Ports per Companion Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn n_pcc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Companion Controller."]
    #[inline(always)]
    pub const fn set_n_pcc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of Companion Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn n_cc(&self) -> NCc {
        let val = (self.0 >> 12usize) & 0x0f;
        NCc::from_bits(val as u8)
    }
    #[doc = "Number of Companion Controller."]
    #[inline(always)]
    pub const fn set_n_cc(&mut self, val: NCc) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Port Indicators (P INDICATOR)."]
    #[must_use]
    #[inline(always)]
    pub const fn pi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Indicators (P INDICATOR)."]
    #[inline(always)]
    pub const fn set_pi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Number of Ports per Transaction Translator."]
    #[must_use]
    #[inline(always)]
    pub const fn n_ptt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Transaction Translator."]
    #[inline(always)]
    pub const fn set_n_ptt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of Transaction Translators."]
    #[must_use]
    #[inline(always)]
    pub const fn n_tt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Transaction Translators."]
    #[inline(always)]
    pub const fn set_n_tt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Hcsparams {
    #[inline(always)]
    fn default() -> Hcsparams {
        Hcsparams(0)
    }
}
impl core::fmt::Debug for Hcsparams {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hcsparams")
            .field("n_ports", &self.n_ports())
            .field("ppc", &self.ppc())
            .field("n_pcc", &self.n_pcc())
            .field("n_cc", &self.n_cc())
            .field("pi", &self.pi())
            .field("n_ptt", &self.n_ptt())
            .field("n_tt", &self.n_tt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hcsparams {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hcsparams {{ n_ports: {=u8:?}, ppc: {=bool:?}, n_pcc: {=u8:?}, n_cc: {:?}, pi: {=bool:?}, n_ptt: {=u8:?}, n_tt: {=u8:?} }}",
            self.n_ports(),
            self.ppc(),
            self.n_pcc(),
            self.n_cc(),
            self.pi(),
            self.n_ptt(),
            self.n_tt()
        )
    }
}
#[doc = "Device Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwdevice(pub u32);
impl Hwdevice {
    #[doc = "Device Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> Dc {
        let val = (self.0 >> 0usize) & 0x01;
        Dc::from_bits(val as u8)
    }
    #[doc = "Device Capable."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: Dc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Device Endpoint Number."]
    #[must_use]
    #[inline(always)]
    pub const fn devep(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number."]
    #[inline(always)]
    pub const fn set_devep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
}
impl Default for Hwdevice {
    #[inline(always)]
    fn default() -> Hwdevice {
        Hwdevice(0)
    }
}
impl core::fmt::Debug for Hwdevice {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwdevice")
            .field("dc", &self.dc())
            .field("devep", &self.devep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwdevice {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwdevice {{ dc: {:?}, devep: {=u8:?} }}",
            self.dc(),
            self.devep()
        )
    }
}
#[doc = "Hardware General."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwgeneral(pub u32);
impl Hwgeneral {
    #[doc = "Data width of the transceiver connected to the controller core."]
    #[must_use]
    #[inline(always)]
    pub const fn phyw(&self) -> Phyw {
        let val = (self.0 >> 4usize) & 0x03;
        Phyw::from_bits(val as u8)
    }
    #[doc = "Data width of the transceiver connected to the controller core."]
    #[inline(always)]
    pub const fn set_phyw(&mut self, val: Phyw) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Transceiver Type."]
    #[must_use]
    #[inline(always)]
    pub const fn phym(&self) -> Phym {
        let val = (self.0 >> 6usize) & 0x07;
        Phym::from_bits(val as u8)
    }
    #[doc = "Transceiver Type."]
    #[inline(always)]
    pub const fn set_phym(&mut self, val: Phym) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Serial interface mode capability."]
    #[must_use]
    #[inline(always)]
    pub const fn sm(&self) -> Sm {
        let val = (self.0 >> 9usize) & 0x03;
        Sm::from_bits(val as u8)
    }
    #[doc = "Serial interface mode capability."]
    #[inline(always)]
    pub const fn set_sm(&mut self, val: Sm) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
}
impl Default for Hwgeneral {
    #[inline(always)]
    fn default() -> Hwgeneral {
        Hwgeneral(0)
    }
}
impl core::fmt::Debug for Hwgeneral {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwgeneral")
            .field("phyw", &self.phyw())
            .field("phym", &self.phym())
            .field("sm", &self.sm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwgeneral {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwgeneral {{ phyw: {:?}, phym: {:?}, sm: {:?} }}",
            self.phyw(),
            self.phym(),
            self.sm()
        )
    }
}
#[doc = "Host Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwhost(pub u32);
impl Hwhost {
    #[doc = "Host Capable."]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> Hc {
        let val = (self.0 >> 0usize) & 0x01;
        Hc::from_bits(val as u8)
    }
    #[doc = "Host Capable."]
    #[inline(always)]
    pub const fn set_hc(&mut self, val: Hc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The Number of downstream ports supported by the host controller is (NPORT+1)."]
    #[must_use]
    #[inline(always)]
    pub const fn nport(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "The Number of downstream ports supported by the host controller is (NPORT+1)."]
    #[inline(always)]
    pub const fn set_nport(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
}
impl Default for Hwhost {
    #[inline(always)]
    fn default() -> Hwhost {
        Hwhost(0)
    }
}
impl core::fmt::Debug for Hwhost {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwhost")
            .field("hc", &self.hc())
            .field("nport", &self.nport())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwhost {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwhost {{ hc: {:?}, nport: {=u8:?} }}",
            self.hc(),
            self.nport()
        )
    }
}
#[doc = "RX Buffer Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwrxbuf(pub u32);
impl Hwrxbuf {
    #[doc = "Default burst size for memory to RX buffer transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn rxburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default burst size for memory to RX buffer transfer."]
    #[inline(always)]
    pub const fn set_rxburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Buffer total size for all receive endpoints is (2^RXADD)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxadd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Buffer total size for all receive endpoints is (2^RXADD)."]
    #[inline(always)]
    pub const fn set_rxadd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Hwrxbuf {
    #[inline(always)]
    fn default() -> Hwrxbuf {
        Hwrxbuf(0)
    }
}
impl core::fmt::Debug for Hwrxbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwrxbuf")
            .field("rxburst", &self.rxburst())
            .field("rxadd", &self.rxadd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwrxbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwrxbuf {{ rxburst: {=u8:?}, rxadd: {=u8:?} }}",
            self.rxburst(),
            self.rxadd()
        )
    }
}
#[doc = "TX Buffer Hardware Parameters."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwtxbuf(pub u32);
impl Hwtxbuf {
    #[doc = "Default burst size for memory to TX buffer transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn txburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default burst size for memory to TX buffer transfer."]
    #[inline(always)]
    pub const fn set_txburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn txchanadd(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes."]
    #[inline(always)]
    pub const fn set_txchanadd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Hwtxbuf {
    #[inline(always)]
    fn default() -> Hwtxbuf {
        Hwtxbuf(0)
    }
}
impl core::fmt::Debug for Hwtxbuf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hwtxbuf")
            .field("txburst", &self.txburst())
            .field("txchanadd", &self.txchanadd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hwtxbuf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hwtxbuf {{ txburst: {=u8:?}, txchanadd: {=u8:?} }}",
            self.txburst(),
            self.txchanadd()
        )
    }
}
#[doc = "Identification."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Configuration Number."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Configuration Number."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Complement Version of ID."]
    #[must_use]
    #[inline(always)]
    pub const fn nid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Complement Version of ID."]
    #[inline(always)]
    pub const fn set_nid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Revision Number of the Controller Core."]
    #[must_use]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Revision Number of the Controller Core."]
    #[inline(always)]
    pub const fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id")
            .field("id", &self.id())
            .field("nid", &self.nid())
            .field("revision", &self.revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Id {{ id: {=u8:?}, nid: {=u8:?}, revision: {=u8:?} }}",
            self.id(),
            self.nid(),
            self.revision()
        )
    }
}
#[doc = "On-The-Go Status & Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgsc(pub u32);
impl Otgsc {
    #[doc = "VBUS Discharge."]
    #[must_use]
    #[inline(always)]
    pub const fn vd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge."]
    #[inline(always)]
    pub const fn set_vd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Charge."]
    #[must_use]
    #[inline(always)]
    pub const fn vc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Charge."]
    #[inline(always)]
    pub const fn set_vc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OTG Termination."]
    #[must_use]
    #[inline(always)]
    pub const fn ot(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Termination."]
    #[inline(always)]
    pub const fn set_ot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Pulsing."]
    #[must_use]
    #[inline(always)]
    pub const fn dp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulsing."]
    #[inline(always)]
    pub const fn set_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID Pullup."]
    #[must_use]
    #[inline(always)]
    pub const fn idpu(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pullup."]
    #[inline(always)]
    pub const fn set_idpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB ID."]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID."]
    #[inline(always)]
    pub const fn set_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "A VBus Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn avv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid."]
    #[inline(always)]
    pub const fn set_avv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "A Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn asv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid."]
    #[inline(always)]
    pub const fn set_asv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "B Session Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn bsv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid."]
    #[inline(always)]
    pub const fn set_bsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "B Session End."]
    #[must_use]
    #[inline(always)]
    pub const fn bse(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End."]
    #[inline(always)]
    pub const fn set_bse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "1 Millisecond Timer Toggle."]
    #[must_use]
    #[inline(always)]
    pub const fn tog_1ms(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Toggle."]
    #[inline(always)]
    pub const fn set_tog_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Bus Pulsing Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Bus Pulsing Status."]
    #[inline(always)]
    pub const fn set_dps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB ID Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn idis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Status."]
    #[inline(always)]
    pub const fn set_idis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "A VBus Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn avvis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_avvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A Session Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn asvis(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_asvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "B Session Valid Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bsvis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Status."]
    #[inline(always)]
    pub const fn set_bsvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "B Session End Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn bseis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Status."]
    #[inline(always)]
    pub const fn set_bseis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn status_1ms(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Status."]
    #[inline(always)]
    pub const fn set_status_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Pulse Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn dpis(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Status."]
    #[inline(always)]
    pub const fn set_dpis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "USB ID Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn idie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Enable."]
    #[inline(always)]
    pub const fn set_idie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "A VBus Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn avvie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_avvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "A Session Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn asvie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_asvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "B Session Valid Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bsvie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bsvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "B Session End Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bseie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Enable."]
    #[inline(always)]
    pub const fn set_bseie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en_1ms(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Enable."]
    #[inline(always)]
    pub const fn set_en_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Data Pulse Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dpie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Otgsc {
    #[inline(always)]
    fn default() -> Otgsc {
        Otgsc(0)
    }
}
impl core::fmt::Debug for Otgsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Otgsc")
            .field("vd", &self.vd())
            .field("vc", &self.vc())
            .field("ot", &self.ot())
            .field("dp", &self.dp())
            .field("idpu", &self.idpu())
            .field("id", &self.id())
            .field("avv", &self.avv())
            .field("asv", &self.asv())
            .field("bsv", &self.bsv())
            .field("bse", &self.bse())
            .field("tog_1ms", &self.tog_1ms())
            .field("dps", &self.dps())
            .field("idis", &self.idis())
            .field("avvis", &self.avvis())
            .field("asvis", &self.asvis())
            .field("bsvis", &self.bsvis())
            .field("bseis", &self.bseis())
            .field("status_1ms", &self.status_1ms())
            .field("dpis", &self.dpis())
            .field("idie", &self.idie())
            .field("avvie", &self.avvie())
            .field("asvie", &self.asvie())
            .field("bsvie", &self.bsvie())
            .field("bseie", &self.bseie())
            .field("en_1ms", &self.en_1ms())
            .field("dpie", &self.dpie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Otgsc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Otgsc {{ vd: {=bool:?}, vc: {=bool:?}, ot: {=bool:?}, dp: {=bool:?}, idpu: {=bool:?}, id: {=bool:?}, avv: {=bool:?}, asv: {=bool:?}, bsv: {=bool:?}, bse: {=bool:?}, tog_1ms: {=bool:?}, dps: {=bool:?}, idis: {=bool:?}, avvis: {=bool:?}, asvis: {=bool:?}, bsvis: {=bool:?}, bseis: {=bool:?}, status_1ms: {=bool:?}, dpis: {=bool:?}, idie: {=bool:?}, avvie: {=bool:?}, asvie: {=bool:?}, bsvie: {=bool:?}, bseie: {=bool:?}, en_1ms: {=bool:?}, dpie: {=bool:?} }}",
            self.vd(),
            self.vc(),
            self.ot(),
            self.dp(),
            self.idpu(),
            self.id(),
            self.avv(),
            self.asv(),
            self.bsv(),
            self.bse(),
            self.tog_1ms(),
            self.dps(),
            self.idis(),
            self.avvis(),
            self.asvis(),
            self.bsvis(),
            self.bseis(),
            self.status_1ms(),
            self.dpis(),
            self.idie(),
            self.avvie(),
            self.asvie(),
            self.bsvie(),
            self.bseie(),
            self.en_1ms(),
            self.dpie()
        )
    }
}
#[doc = "Frame List Base Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periodiclistbase(pub u32);
impl Periodiclistbase {
    #[doc = "Base Address (Low)."]
    #[must_use]
    #[inline(always)]
    pub const fn baseadr(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address (Low)."]
    #[inline(always)]
    pub const fn set_baseadr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Periodiclistbase {
    #[inline(always)]
    fn default() -> Periodiclistbase {
        Periodiclistbase(0)
    }
}
impl core::fmt::Debug for Periodiclistbase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Periodiclistbase")
            .field("baseadr", &self.baseadr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Periodiclistbase {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Periodiclistbase {{ baseadr: {=u32:?} }}",
            self.baseadr()
        )
    }
}
#[doc = "Port Status & Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portsc1(pub u32);
impl Portsc1 {
    #[doc = "Current Connect Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Connect Status."]
    #[inline(always)]
    pub const fn set_ccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change."]
    #[must_use]
    #[inline(always)]
    pub const fn csc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect Status Change."]
    #[inline(always)]
    pub const fn set_csc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enabled/Disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled."]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable/Disable Change."]
    #[must_use]
    #[inline(always)]
    pub const fn pec(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enable/Disable Change."]
    #[inline(always)]
    pub const fn set_pec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Over-Current Active."]
    #[must_use]
    #[inline(always)]
    pub const fn oca(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Current Active."]
    #[inline(always)]
    pub const fn set_oca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Over-current Change."]
    #[must_use]
    #[inline(always)]
    pub const fn occ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current Change."]
    #[inline(always)]
    pub const fn set_occ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume."]
    #[must_use]
    #[inline(always)]
    pub const fn fpr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Port Resume."]
    #[inline(always)]
    pub const fn set_fpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend."]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset."]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "High-Speed Port."]
    #[must_use]
    #[inline(always)]
    pub const fn hsp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High-Speed Port."]
    #[inline(always)]
    pub const fn set_hsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Line Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ls(&self) -> Ls {
        let val = (self.0 >> 10usize) & 0x03;
        Ls::from_bits(val as u8)
    }
    #[doc = "Line Status."]
    #[inline(always)]
    pub const fn set_ls(&mut self, val: Ls) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power."]
    #[must_use]
    #[inline(always)]
    pub const fn pp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power."]
    #[inline(always)]
    pub const fn set_pp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Owner."]
    #[must_use]
    #[inline(always)]
    pub const fn po(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Owner."]
    #[inline(always)]
    pub const fn set_po(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Indicator Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pic(&self) -> Pic {
        let val = (self.0 >> 14usize) & 0x03;
        Pic::from_bits(val as u8)
    }
    #[doc = "Port Indicator Control."]
    #[inline(always)]
    pub const fn set_pic(&mut self, val: Pic) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ptc(&self) -> Ptc {
        let val = (self.0 >> 16usize) & 0x0f;
        Ptc::from_bits(val as u8)
    }
    #[doc = "Port Test Control."]
    #[inline(always)]
    pub const fn set_ptc(&mut self, val: Ptc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)."]
    #[must_use]
    #[inline(always)]
    pub const fn wkcn(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)."]
    #[inline(always)]
    pub const fn set_wkcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)."]
    #[must_use]
    #[inline(always)]
    pub const fn wkdc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)."]
    #[inline(always)]
    pub const fn set_wkdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake on Over-current Enable (WKOC_E)."]
    #[must_use]
    #[inline(always)]
    pub const fn wkoc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Over-current Enable (WKOC_E)."]
    #[inline(always)]
    pub const fn set_wkoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD)."]
    #[must_use]
    #[inline(always)]
    pub const fn phcd(&self) -> Phcd {
        let val = (self.0 >> 23usize) & 0x01;
        Phcd::from_bits(val as u8)
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD)."]
    #[inline(always)]
    pub const fn set_phcd(&mut self, val: Phcd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Force Full Speed Connect."]
    #[must_use]
    #[inline(always)]
    pub const fn pfsc(&self) -> Pfsc {
        let val = (self.0 >> 24usize) & 0x01;
        Pfsc::from_bits(val as u8)
    }
    #[doc = "Port Force Full Speed Connect."]
    #[inline(always)]
    pub const fn set_pfsc(&mut self, val: Pfsc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Parallel Transceiver Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pts_2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Transceiver Select."]
    #[inline(always)]
    pub const fn set_pts_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Speed."]
    #[must_use]
    #[inline(always)]
    pub const fn pspd(&self) -> Pspd {
        let val = (self.0 >> 26usize) & 0x03;
        Pspd::from_bits(val as u8)
    }
    #[doc = "Port Speed."]
    #[inline(always)]
    pub const fn set_pspd(&mut self, val: Pspd) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Parallel Transceiver Width - Read/Write."]
    #[must_use]
    #[inline(always)]
    pub const fn ptw(&self) -> Ptw {
        let val = (self.0 >> 28usize) & 0x01;
        Ptw::from_bits(val as u8)
    }
    #[doc = "Parallel Transceiver Width - Read/Write."]
    #[inline(always)]
    pub const fn set_ptw(&mut self, val: Ptw) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Serial Transceiver Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Transceiver Select."]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Parallel Transceiver Select."]
    #[must_use]
    #[inline(always)]
    pub const fn pts_1(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Parallel Transceiver Select."]
    #[inline(always)]
    pub const fn set_pts_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Portsc1 {
    #[inline(always)]
    fn default() -> Portsc1 {
        Portsc1(0)
    }
}
impl core::fmt::Debug for Portsc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Portsc1")
            .field("ccs", &self.ccs())
            .field("csc", &self.csc())
            .field("pe", &self.pe())
            .field("pec", &self.pec())
            .field("oca", &self.oca())
            .field("occ", &self.occ())
            .field("fpr", &self.fpr())
            .field("susp", &self.susp())
            .field("pr", &self.pr())
            .field("hsp", &self.hsp())
            .field("ls", &self.ls())
            .field("pp", &self.pp())
            .field("po", &self.po())
            .field("pic", &self.pic())
            .field("ptc", &self.ptc())
            .field("wkcn", &self.wkcn())
            .field("wkdc", &self.wkdc())
            .field("wkoc", &self.wkoc())
            .field("phcd", &self.phcd())
            .field("pfsc", &self.pfsc())
            .field("pts_2", &self.pts_2())
            .field("pspd", &self.pspd())
            .field("ptw", &self.ptw())
            .field("sts", &self.sts())
            .field("pts_1", &self.pts_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Portsc1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Portsc1 {{ ccs: {=bool:?}, csc: {=bool:?}, pe: {=bool:?}, pec: {=bool:?}, oca: {=bool:?}, occ: {=bool:?}, fpr: {=bool:?}, susp: {=bool:?}, pr: {=bool:?}, hsp: {=bool:?}, ls: {:?}, pp: {=bool:?}, po: {=bool:?}, pic: {:?}, ptc: {:?}, wkcn: {=bool:?}, wkdc: {=bool:?}, wkoc: {=bool:?}, phcd: {:?}, pfsc: {:?}, pts_2: {=bool:?}, pspd: {:?}, ptw: {:?}, sts: {=bool:?}, pts_1: {=u8:?} }}",
            self.ccs(),
            self.csc(),
            self.pe(),
            self.pec(),
            self.oca(),
            self.occ(),
            self.fpr(),
            self.susp(),
            self.pr(),
            self.hsp(),
            self.ls(),
            self.pp(),
            self.po(),
            self.pic(),
            self.ptc(),
            self.wkcn(),
            self.wkdc(),
            self.wkoc(),
            self.phcd(),
            self.pfsc(),
            self.pts_2(),
            self.pspd(),
            self.ptw(),
            self.sts(),
            self.pts_1()
        )
    }
}
#[doc = "System Bus Config."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbuscfg(pub u32);
impl Sbuscfg {
    #[doc = "AHB master interface Burst configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbrst(&self) -> Ahbbrst {
        let val = (self.0 >> 0usize) & 0x07;
        Ahbbrst::from_bits(val as u8)
    }
    #[doc = "AHB master interface Burst configuration."]
    #[inline(always)]
    pub const fn set_ahbbrst(&mut self, val: Ahbbrst) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sbuscfg {
    #[inline(always)]
    fn default() -> Sbuscfg {
        Sbuscfg(0)
    }
}
impl core::fmt::Debug for Sbuscfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbuscfg")
            .field("ahbbrst", &self.ahbbrst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbuscfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sbuscfg {{ ahbbrst: {:?} }}", self.ahbbrst())
    }
}
#[doc = "TX FIFO Fill Tuning."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txfilltuning(pub u32);
impl Txfilltuning {
    #[doc = "Scheduler Overhead."]
    #[must_use]
    #[inline(always)]
    pub const fn txschoh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Scheduler Overhead."]
    #[inline(always)]
    pub const fn set_txschoh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Scheduler Health Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn txschhealth(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Scheduler Health Counter."]
    #[inline(always)]
    pub const fn set_txschhealth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "FIFO Burst Threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn txfifothres(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "FIFO Burst Threshold."]
    #[inline(always)]
    pub const fn set_txfifothres(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Txfilltuning {
    #[inline(always)]
    fn default() -> Txfilltuning {
        Txfilltuning(0)
    }
}
impl core::fmt::Debug for Txfilltuning {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txfilltuning")
            .field("txschoh", &self.txschoh())
            .field("txschhealth", &self.txschhealth())
            .field("txfifothres", &self.txfifothres())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txfilltuning {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Txfilltuning {{ txschoh: {=u8:?}, txschhealth: {=u8:?}, txfifothres: {=u8:?} }}",
            self.txschoh(),
            self.txschhealth(),
            self.txfifothres()
        )
    }
}
#[doc = "USB Command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcmd(pub u32);
impl Usbcmd {
    #[doc = "Run/Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run/Stop."]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controller Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Reset."]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame List Size."]
    #[must_use]
    #[inline(always)]
    pub const fn fs_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Frame List Size."]
    #[inline(always)]
    pub const fn set_fs_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Periodic Schedule Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pse(&self) -> Pse {
        let val = (self.0 >> 4usize) & 0x01;
        Pse::from_bits(val as u8)
    }
    #[doc = "Periodic Schedule Enable."]
    #[inline(always)]
    pub const fn set_pse(&mut self, val: Pse) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Schedule Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ase(&self) -> Ase {
        let val = (self.0 >> 5usize) & 0x01;
        Ase::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Enable."]
    #[inline(always)]
    pub const fn set_ase(&mut self, val: Ase) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on Async Advance Doorbell."]
    #[must_use]
    #[inline(always)]
    pub const fn iaa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance Doorbell."]
    #[inline(always)]
    pub const fn set_iaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Count."]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Asynchronous Schedule Park Mode Count."]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aspe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Mode Enable."]
    #[inline(always)]
    pub const fn set_aspe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Setup TripWire \\[device mode only\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn sutw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Setup TripWire \\[device mode only\\]."]
    #[inline(always)]
    pub const fn set_sutw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Add dTD TripWire\\[device mode only\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn atdtw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Add dTD TripWire\\[device mode only\\]."]
    #[inline(always)]
    pub const fn set_atdtw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Frame List Size \\[host mode only\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn fs_2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Size \\[host mode only\\]."]
    #[inline(always)]
    pub const fn set_fs_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Threshold Control."]
    #[must_use]
    #[inline(always)]
    pub const fn itc(&self) -> Itc {
        let val = (self.0 >> 16usize) & 0xff;
        Itc::from_bits(val as u8)
    }
    #[doc = "Interrupt Threshold Control."]
    #[inline(always)]
    pub const fn set_itc(&mut self, val: Itc) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for Usbcmd {
    #[inline(always)]
    fn default() -> Usbcmd {
        Usbcmd(0)
    }
}
impl core::fmt::Debug for Usbcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbcmd")
            .field("rs", &self.rs())
            .field("rst", &self.rst())
            .field("fs_1", &self.fs_1())
            .field("pse", &self.pse())
            .field("ase", &self.ase())
            .field("iaa", &self.iaa())
            .field("asp", &self.asp())
            .field("aspe", &self.aspe())
            .field("sutw", &self.sutw())
            .field("atdtw", &self.atdtw())
            .field("fs_2", &self.fs_2())
            .field("itc", &self.itc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbcmd {{ rs: {=bool:?}, rst: {=bool:?}, fs_1: {=u8:?}, pse: {:?}, ase: {:?}, iaa: {=bool:?}, asp: {=u8:?}, aspe: {=bool:?}, sutw: {=bool:?}, atdtw: {=bool:?}, fs_2: {=bool:?}, itc: {:?} }}",
            self.rs(),
            self.rst(),
            self.fs_1(),
            self.pse(),
            self.ase(),
            self.iaa(),
            self.asp(),
            self.aspe(),
            self.sutw(),
            self.atdtw(),
            self.fs_2(),
            self.itc()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbintr(pub u32);
impl Usbintr {
    #[doc = "USB Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uee(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_pce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fre(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover Interrupt Enable."]
    #[inline(always)]
    pub const fn set_fre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn see(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_see(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Async Advance Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Async Advance Interrupt Enable."]
    #[inline(always)]
    pub const fn set_aae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ure(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sleep Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "NAK Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn nake(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Enable."]
    #[inline(always)]
    pub const fn set_nake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "USB Host Asynchronous Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn uaie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Asynchronous Interrupt Enable."]
    #[inline(always)]
    pub const fn set_uaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB Host Periodic Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn upie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Periodic Interrupt Enable."]
    #[inline(always)]
    pub const fn set_upie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable."]
    #[inline(always)]
    pub const fn set_tie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Usbintr {
    #[inline(always)]
    fn default() -> Usbintr {
        Usbintr(0)
    }
}
impl core::fmt::Debug for Usbintr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbintr")
            .field("ue", &self.ue())
            .field("uee", &self.uee())
            .field("pce", &self.pce())
            .field("fre", &self.fre())
            .field("see", &self.see())
            .field("aae", &self.aae())
            .field("ure", &self.ure())
            .field("sre", &self.sre())
            .field("sle", &self.sle())
            .field("nake", &self.nake())
            .field("uaie", &self.uaie())
            .field("upie", &self.upie())
            .field("tie0", &self.tie0())
            .field("tie1", &self.tie1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbintr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbintr {{ ue: {=bool:?}, uee: {=bool:?}, pce: {=bool:?}, fre: {=bool:?}, see: {=bool:?}, aae: {=bool:?}, ure: {=bool:?}, sre: {=bool:?}, sle: {=bool:?}, nake: {=bool:?}, uaie: {=bool:?}, upie: {=bool:?}, tie0: {=bool:?}, tie1: {=bool:?} }}",
            self.ue(),
            self.uee(),
            self.pce(),
            self.fre(),
            self.see(),
            self.aae(),
            self.ure(),
            self.sre(),
            self.sle(),
            self.nake(),
            self.uaie(),
            self.upie(),
            self.tie0(),
            self.tie1()
        )
    }
}
#[doc = "USB Device Mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbmode(pub u32);
impl Usbmode {
    #[doc = "Controller Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> Cm {
        let val = (self.0 >> 0usize) & 0x03;
        Cm::from_bits(val as u8)
    }
    #[doc = "Controller Mode."]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: Cm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Endian Select."]
    #[must_use]
    #[inline(always)]
    pub const fn es(&self) -> Es {
        let val = (self.0 >> 2usize) & 0x01;
        Es::from_bits(val as u8)
    }
    #[doc = "Endian Select."]
    #[inline(always)]
    pub const fn set_es(&mut self, val: Es) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Lockout Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn slom(&self) -> Slom {
        let val = (self.0 >> 3usize) & 0x01;
        Slom::from_bits(val as u8)
    }
    #[doc = "Setup Lockout Mode."]
    #[inline(always)]
    pub const fn set_slom(&mut self, val: Slom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Stream Disable Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn sdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Disable Mode."]
    #[inline(always)]
    pub const fn set_sdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usbmode {
    #[inline(always)]
    fn default() -> Usbmode {
        Usbmode(0)
    }
}
impl core::fmt::Debug for Usbmode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbmode")
            .field("cm", &self.cm())
            .field("es", &self.es())
            .field("slom", &self.slom())
            .field("sdis", &self.sdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbmode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbmode {{ cm: {:?}, es: {:?}, slom: {:?}, sdis: {=bool:?} }}",
            self.cm(),
            self.es(),
            self.slom(),
            self.sdis()
        )
    }
}
#[doc = "USB Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbsts(pub u32);
impl Usbsts {
    #[doc = "USB Interrupt (USBINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn ui(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt (USBINT)."]
    #[inline(always)]
    pub const fn set_ui(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt (USBERRINT)."]
    #[must_use]
    #[inline(always)]
    pub const fn uei(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt (USBERRINT)."]
    #[inline(always)]
    pub const fn set_uei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn pci(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect."]
    #[inline(always)]
    pub const fn set_pci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover."]
    #[must_use]
    #[inline(always)]
    pub const fn fri(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover."]
    #[inline(always)]
    pub const fn set_fri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sei(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error."]
    #[inline(always)]
    pub const fn set_sei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Async Advance."]
    #[must_use]
    #[inline(always)]
    pub const fn aai(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance."]
    #[inline(always)]
    pub const fn set_aai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Received."]
    #[must_use]
    #[inline(always)]
    pub const fn uri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Received."]
    #[inline(always)]
    pub const fn set_uri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received."]
    #[must_use]
    #[inline(always)]
    pub const fn sri(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received."]
    #[inline(always)]
    pub const fn set_sri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DCSuspend."]
    #[must_use]
    #[inline(always)]
    pub const fn sli(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DCSuspend."]
    #[inline(always)]
    pub const fn set_sli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ULPI Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ulpii(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ULPI Interrupt."]
    #[inline(always)]
    pub const fn set_ulpii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HCHaIted."]
    #[must_use]
    #[inline(always)]
    pub const fn hch(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HCHaIted."]
    #[inline(always)]
    pub const fn set_hch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reclamation."]
    #[must_use]
    #[inline(always)]
    pub const fn rcl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reclamation."]
    #[inline(always)]
    pub const fn set_rcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Periodic Schedule Status."]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic Schedule Status."]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Asynchronous Schedule Status."]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Status."]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Interrupt Bit."]
    #[must_use]
    #[inline(always)]
    pub const fn naki(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Bit."]
    #[inline(always)]
    pub const fn set_naki(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0)."]
    #[must_use]
    #[inline(always)]
    pub const fn ti0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0)."]
    #[inline(always)]
    pub const fn set_ti0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1)."]
    #[must_use]
    #[inline(always)]
    pub const fn ti1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1)."]
    #[inline(always)]
    pub const fn set_ti1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Usbsts {
    #[inline(always)]
    fn default() -> Usbsts {
        Usbsts(0)
    }
}
impl core::fmt::Debug for Usbsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usbsts")
            .field("ui", &self.ui())
            .field("uei", &self.uei())
            .field("pci", &self.pci())
            .field("fri", &self.fri())
            .field("sei", &self.sei())
            .field("aai", &self.aai())
            .field("uri", &self.uri())
            .field("sri", &self.sri())
            .field("sli", &self.sli())
            .field("ulpii", &self.ulpii())
            .field("hch", &self.hch())
            .field("rcl", &self.rcl())
            .field("ps", &self.ps())
            .field("as_", &self.as_())
            .field("naki", &self.naki())
            .field("ti0", &self.ti0())
            .field("ti1", &self.ti1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usbsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usbsts {{ ui: {=bool:?}, uei: {=bool:?}, pci: {=bool:?}, fri: {=bool:?}, sei: {=bool:?}, aai: {=bool:?}, uri: {=bool:?}, sri: {=bool:?}, sli: {=bool:?}, ulpii: {=bool:?}, hch: {=bool:?}, rcl: {=bool:?}, ps: {=bool:?}, as_: {=bool:?}, naki: {=bool:?}, ti0: {=bool:?}, ti1: {=bool:?} }}",
            self.ui(),
            self.uei(),
            self.pci(),
            self.fri(),
            self.sei(),
            self.aai(),
            self.uri(),
            self.sri(),
            self.sli(),
            self.ulpii(),
            self.hch(),
            self.rcl(),
            self.ps(),
            self.as_(),
            self.naki(),
            self.ti0(),
            self.ti1()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbrst {
    #[doc = "Incremental burst of unspecified length only."]
    IncrBurst = 0x0,
    #[doc = "INCR4 burst, then single transfer."]
    Incr4Burst = 0x01,
    #[doc = "INCR8 burst, INCR4 burst, then single transfer."]
    Incr8Burst = 0x02,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then single transfer."]
    Incr16Burst = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "INCR4 burst, then incremental burst of unspecified length."]
    Incr4Unspec = 0x05,
    #[doc = "INCR8 burst, INCR4 burst, then incremental burst of unspecified length."]
    Incr84Unspec = 0x06,
    #[doc = "INCR16 burst, INCR8 burst, INCR4 burst, then incremental burst of unspecified length."]
    Incr1684Unspec = 0x07,
}
impl Ahbbrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbrst {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbrst {
    #[inline(always)]
    fn from(val: u8) -> Ahbbrst {
        Ahbbrst::from_bits(val)
    }
}
impl From<Ahbbrst> for u8 {
    #[inline(always)]
    fn from(val: Ahbbrst) -> u8 {
        Ahbbrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ase {
    #[doc = "Do not process the Asynchronous Schedule."]
    DontProcessAsync = 0x0,
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    AccessAsync = 0x01,
}
impl Ase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ase {
    #[inline(always)]
    fn from(val: u8) -> Ase {
        Ase::from_bits(val)
    }
}
impl From<Ase> for u8 {
    #[inline(always)]
    fn from(val: Ase) -> u8 {
        Ase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cf {
    #[doc = "Port routing control logic default-routes each port to an implementation dependent classic host controller."]
    PortRoutingClassicHost = 0x0,
    #[doc = "Port routing control logic default-routes all ports to this host controller."]
    PortRoutingHost = 0x01,
}
impl Cf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cf {
    #[inline(always)]
    fn from(val: u8) -> Cf {
        Cf::from_bits(val)
    }
}
impl From<Cf> for u8 {
    #[inline(always)]
    fn from(val: Cf) -> u8 {
        Cf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cm {
    #[doc = "Idle \\[Default for combination host/device\\]."]
    Idl = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Device Controller \\[Default for device only controller\\]."]
    DeviceContr = 0x02,
    #[doc = "Host Controller \\[Default for host only controller\\]."]
    HostContr = 0x03,
}
impl Cm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cm {
    #[inline(always)]
    fn from(val: u8) -> Cm {
        Cm::from_bits(val)
    }
}
impl From<Cm> for u8 {
    #[inline(always)]
    fn from(val: Cm) -> u8 {
        Cm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dc {
    #[doc = "Not supported."]
    DeviceOpDis = 0x0,
    #[doc = "Supported."]
    DeviceOpEn = 0x01,
}
impl Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dc {
    #[inline(always)]
    fn from(val: u8) -> Dc {
        Dc::from_bits(val)
    }
}
impl From<Dc> for u8 {
    #[inline(always)]
    fn from(val: Dc) -> u8 {
        Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Rxt {
    #[doc = "Control."]
    Ctrl = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Endptctrl0Rxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Rxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Rxt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Rxt {
        Endptctrl0Rxt::from_bits(val)
    }
}
impl From<Endptctrl0Rxt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Rxt) -> u8 {
        Endptctrl0Rxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endptctrl0Txt {
    #[doc = "Control."]
    Ctrl = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Endptctrl0Txt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endptctrl0Txt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endptctrl0Txt {
    #[inline(always)]
    fn from(val: u8) -> Endptctrl0Txt {
        Endptctrl0Txt::from_bits(val)
    }
}
impl From<Endptctrl0Txt> for u8 {
    #[inline(always)]
    fn from(val: Endptctrl0Txt) -> u8 {
        Endptctrl0Txt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndptctrlRxt {
    #[doc = "Control."]
    Ctl = 0x0,
    #[doc = "Isochronous."]
    Iso = 0x01,
    #[doc = "Bulk."]
    Blk = 0x02,
    #[doc = "Interrupt."]
    Irq = 0x03,
}
impl EndptctrlRxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EndptctrlRxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EndptctrlRxt {
    #[inline(always)]
    fn from(val: u8) -> EndptctrlRxt {
        EndptctrlRxt::from_bits(val)
    }
}
impl From<EndptctrlRxt> for u8 {
    #[inline(always)]
    fn from(val: EndptctrlRxt) -> u8 {
        EndptctrlRxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndptctrlTxi {
    #[doc = "PID sequencing enabled."]
    PidEn = 0x0,
    #[doc = "PID sequencing disabled."]
    PidDis = 0x01,
}
impl EndptctrlTxi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EndptctrlTxi {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EndptctrlTxi {
    #[inline(always)]
    fn from(val: u8) -> EndptctrlTxi {
        EndptctrlTxi::from_bits(val)
    }
}
impl From<EndptctrlTxi> for u8 {
    #[inline(always)]
    fn from(val: EndptctrlTxi) -> u8 {
        EndptctrlTxi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EndptctrlTxt {
    #[doc = "Control."]
    Ctl = 0x0,
    #[doc = "Isochronous."]
    Iso = 0x01,
    #[doc = "Bulk."]
    Blk = 0x02,
    #[doc = "Interrupt."]
    Irq = 0x03,
}
impl EndptctrlTxt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EndptctrlTxt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EndptctrlTxt {
    #[inline(always)]
    fn from(val: u8) -> EndptctrlTxt {
        EndptctrlTxt::from_bits(val)
    }
}
impl From<EndptctrlTxt> for u8 {
    #[inline(always)]
    fn from(val: EndptctrlTxt) -> u8 {
        EndptctrlTxt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Es {
    #[doc = "Little Endian."]
    LittleEndian = 0x0,
    #[doc = "Big Endian."]
    BigEndian = 0x01,
}
impl Es {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Es {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Es {
    #[inline(always)]
    fn from(val: u8) -> Es {
        Es::from_bits(val)
    }
}
impl From<Es> for u8 {
    #[inline(always)]
    fn from(val: Es) -> u8 {
        Es::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct FrindexVal(u16);
impl FrindexVal {
    #[doc = "(1024) 12."]
    pub const Frindex1024: Self = Self(0x0);
    #[doc = "(512) 11."]
    pub const Frindex512: Self = Self(0x01);
    #[doc = "(256) 10."]
    pub const Frindex256: Self = Self(0x02);
    #[doc = "(128) 9."]
    pub const Frindex128: Self = Self(0x03);
    #[doc = "(64) 8."]
    pub const Frindex64: Self = Self(0x04);
    #[doc = "(32) 7."]
    pub const Frindex32: Self = Self(0x05);
    #[doc = "(16) 6."]
    pub const Frindex16: Self = Self(0x06);
    #[doc = "(8) 5."]
    pub const Frindex8: Self = Self(0x07);
}
impl FrindexVal {
    pub const fn from_bits(val: u16) -> FrindexVal {
        Self(val & 0x3fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for FrindexVal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Frindex1024"),
            0x01 => f.write_str("Frindex512"),
            0x02 => f.write_str("Frindex256"),
            0x03 => f.write_str("Frindex128"),
            0x04 => f.write_str("Frindex64"),
            0x05 => f.write_str("Frindex32"),
            0x06 => f.write_str("Frindex16"),
            0x07 => f.write_str("Frindex8"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrindexVal {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Frindex1024"),
            0x01 => defmt::write!(f, "Frindex512"),
            0x02 => defmt::write!(f, "Frindex256"),
            0x03 => defmt::write!(f, "Frindex128"),
            0x04 => defmt::write!(f, "Frindex64"),
            0x05 => defmt::write!(f, "Frindex32"),
            0x06 => defmt::write!(f, "Frindex16"),
            0x07 => defmt::write!(f, "Frindex8"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for FrindexVal {
    #[inline(always)]
    fn from(val: u16) -> FrindexVal {
        FrindexVal::from_bits(val)
    }
}
impl From<FrindexVal> for u16 {
    #[inline(always)]
    fn from(val: FrindexVal) -> u16 {
        FrindexVal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptmode {
    #[doc = "One Shot Mode."]
    OneShot = 0x0,
    #[doc = "Repeat Mode."]
    Repeat = 0x01,
}
impl Gptimer0ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptmode {
        Gptimer0ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptmode) -> u8 {
        Gptimer0ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrst {
    #[doc = "No action."]
    NoAction = 0x0,
    #[doc = "Load counter value from GPTLD bits in n_GPTIMER0LD."]
    LoadCntr = 0x01,
}
impl Gptimer0ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrst {
        Gptimer0ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrst) -> u8 {
        Gptimer0ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer0ctrlGptrun {
    #[doc = "Stop counting."]
    StopCntr = 0x0,
    #[doc = "Run."]
    Run = 0x01,
}
impl Gptimer0ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer0ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer0ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer0ctrlGptrun {
        Gptimer0ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer0ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer0ctrlGptrun) -> u8 {
        Gptimer0ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptmode {
    #[doc = "One Shot Mode."]
    OneShot = 0x0,
    #[doc = "Repeat Mode."]
    Repeat = 0x01,
}
impl Gptimer1ctrlGptmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptmode {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptmode {
        Gptimer1ctrlGptmode::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptmode> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptmode) -> u8 {
        Gptimer1ctrlGptmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrst {
    #[doc = "No action."]
    NoAction = 0x0,
    #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD."]
    LoadCntr = 0x01,
}
impl Gptimer1ctrlGptrst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrst {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrst {
        Gptimer1ctrlGptrst::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrst> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrst) -> u8 {
        Gptimer1ctrlGptrst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gptimer1ctrlGptrun {
    #[doc = "Stop counting."]
    StopCntr = 0x0,
    #[doc = "Run."]
    Run = 0x01,
}
impl Gptimer1ctrlGptrun {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gptimer1ctrlGptrun {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gptimer1ctrlGptrun {
    #[inline(always)]
    fn from(val: u8) -> Gptimer1ctrlGptrun {
        Gptimer1ctrlGptrun::from_bits(val)
    }
}
impl From<Gptimer1ctrlGptrun> for u8 {
    #[inline(always)]
    fn from(val: Gptimer1ctrlGptrun) -> u8 {
        Gptimer1ctrlGptrun::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hc {
    #[doc = "Not supported."]
    HostOpDis = 0x0,
    #[doc = "Supported."]
    HostOpEn = 0x01,
}
impl Hc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hc {
    #[inline(always)]
    fn from(val: u8) -> Hc {
        Hc::from_bits(val)
    }
}
impl From<Hc> for u8 {
    #[inline(always)]
    fn from(val: Hc) -> u8 {
        Hc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Itc(u8);
impl Itc {
    #[doc = "Immediate (no threshold)."]
    pub const Immediate: Self = Self(0x0);
    #[doc = "1 micro-frame."]
    pub const Microframe1: Self = Self(0x01);
    #[doc = "2 micro-frames."]
    pub const Microframe2: Self = Self(0x02);
    #[doc = "4 micro-frames."]
    pub const Microframe4: Self = Self(0x04);
    #[doc = "8 micro-frames."]
    pub const Microframe8: Self = Self(0x08);
    #[doc = "16 micro-frames."]
    pub const Microframe16: Self = Self(0x10);
    #[doc = "32 micro-frames."]
    pub const Microframe32: Self = Self(0x20);
    #[doc = "64 micro-frames."]
    pub const Microframe64: Self = Self(0x40);
}
impl Itc {
    pub const fn from_bits(val: u8) -> Itc {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Itc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Immediate"),
            0x01 => f.write_str("Microframe1"),
            0x02 => f.write_str("Microframe2"),
            0x04 => f.write_str("Microframe4"),
            0x08 => f.write_str("Microframe8"),
            0x10 => f.write_str("Microframe16"),
            0x20 => f.write_str("Microframe32"),
            0x40 => f.write_str("Microframe64"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itc {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Immediate"),
            0x01 => defmt::write!(f, "Microframe1"),
            0x02 => defmt::write!(f, "Microframe2"),
            0x04 => defmt::write!(f, "Microframe4"),
            0x08 => defmt::write!(f, "Microframe8"),
            0x10 => defmt::write!(f, "Microframe16"),
            0x20 => defmt::write!(f, "Microframe32"),
            0x40 => defmt::write!(f, "Microframe64"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Itc {
    #[inline(always)]
    fn from(val: u8) -> Itc {
        Itc::from_bits(val)
    }
}
impl From<Itc> for u8 {
    #[inline(always)]
    fn from(val: Itc) -> u8 {
        Itc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ls {
    #[doc = "SE0."]
    Se0 = 0x0,
    #[doc = "K-state."]
    KState = 0x01,
    #[doc = "J-state."]
    JState = 0x02,
    #[doc = "Undefined."]
    Undefined = 0x03,
}
impl Ls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ls {
    #[inline(always)]
    fn from(val: u8) -> Ls {
        Ls::from_bits(val)
    }
}
impl From<Ls> for u8 {
    #[inline(always)]
    fn from(val: Ls) -> u8 {
        Ls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NCc {
    #[doc = "There is no internal Companion Controller and port-ownership hand-off is not supported."]
    NoCompController = 0x0,
    #[doc = "There are internal companion controller(s) and port-ownership hand-offs is supported."]
    CompController = 0x01,
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
impl NCc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NCc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NCc {
    #[inline(always)]
    fn from(val: u8) -> NCc {
        NCc::from_bits(val)
    }
}
impl From<NCc> for u8 {
    #[inline(always)]
    fn from(val: NCc) -> u8 {
        NCc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pfsc {
    #[doc = "Normal operation."]
    Normal = 0x0,
    #[doc = "Forced to full speed."]
    FullSpeed = 0x01,
}
impl Pfsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfsc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfsc {
    #[inline(always)]
    fn from(val: u8) -> Pfsc {
        Pfsc::from_bits(val)
    }
}
impl From<Pfsc> for u8 {
    #[inline(always)]
    fn from(val: Pfsc) -> u8 {
        Pfsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phcd {
    #[doc = "Enable PHY clock."]
    PhyClkEn = 0x0,
    #[doc = "Disable PHY clock."]
    PhyClkDis = 0x01,
}
impl Phcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phcd {
    #[inline(always)]
    fn from(val: u8) -> Phcd {
        Phcd::from_bits(val)
    }
}
impl From<Phcd> for u8 {
    #[inline(always)]
    fn from(val: Phcd) -> u8 {
        Phcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phym {
    #[doc = "UTMI/UMTI+."]
    Utmi = 0x0,
    #[doc = "ULPI DDR."]
    UlpiDdr = 0x01,
    #[doc = "ULPI."]
    Ulpi = 0x02,
    #[doc = "Serial Only."]
    Serial = 0x03,
    #[doc = "Software programmable - reset to UTMI/UTMI+."]
    SwRstUtmi = 0x04,
    #[doc = "Software programmable - reset to ULPI DDR."]
    SwRstUlpiDdr = 0x05,
    #[doc = "Software programmable - reset to ULPI."]
    SwRstUlpi = 0x06,
    #[doc = "Software programmable - reset to Serial."]
    SwRstSerial = 0x07,
}
impl Phym {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phym {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phym {
    #[inline(always)]
    fn from(val: u8) -> Phym {
        Phym::from_bits(val)
    }
}
impl From<Phym> for u8 {
    #[inline(always)]
    fn from(val: Phym) -> u8 {
        Phym::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phyw {
    #[doc = "8 bit wide data bus (Software non-programmable)."]
    DataBus8 = 0x0,
    #[doc = "16 bit wide data bus (Software non-programmable)."]
    DataBus16 = 0x01,
    #[doc = "Reset to 8 bit wide data bus (Software programmable)."]
    SwRst8 = 0x02,
    #[doc = "Reset to 16 bit wide data bus (Software programmable)."]
    SwRst16 = 0x03,
}
impl Phyw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phyw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phyw {
    #[inline(always)]
    fn from(val: u8) -> Phyw {
        Phyw::from_bits(val)
    }
}
impl From<Phyw> for u8 {
    #[inline(always)]
    fn from(val: Phyw) -> u8 {
        Phyw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pic {
    #[doc = "Port indicators are off."]
    PortIndicatorOff = 0x0,
    #[doc = "Amber."]
    PortIndAmber = 0x01,
    #[doc = "Green."]
    PortIndGreen = 0x02,
    #[doc = "Undefined."]
    Undefined = 0x03,
}
impl Pic {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pic {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pic {
    #[inline(always)]
    fn from(val: u8) -> Pic {
        Pic::from_bits(val)
    }
}
impl From<Pic> for u8 {
    #[inline(always)]
    fn from(val: Pic) -> u8 {
        Pic::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pse {
    #[doc = "Do not process the Periodic Schedule."]
    DontProcessPt = 0x0,
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    ProcessPtPeriodiclistbase = 0x01,
}
impl Pse {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pse {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pse {
    #[inline(always)]
    fn from(val: u8) -> Pse {
        Pse::from_bits(val)
    }
}
impl From<Pse> for u8 {
    #[inline(always)]
    fn from(val: Pse) -> u8 {
        Pse::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pspd {
    #[doc = "Full Speed."]
    Fs = 0x0,
    #[doc = "Low Speed."]
    Ls = 0x01,
    #[doc = "High Speed."]
    Hs = 0x02,
    #[doc = "Undefined."]
    Undefined = 0x03,
}
impl Pspd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pspd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pspd {
    #[inline(always)]
    fn from(val: u8) -> Pspd {
        Pspd::from_bits(val)
    }
}
impl From<Pspd> for u8 {
    #[inline(always)]
    fn from(val: Pspd) -> u8 {
        Pspd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptc {
    #[doc = "TEST_MODE_DISABLE."]
    TstModeDis = 0x0,
    #[doc = "J_STATE."]
    JState = 0x01,
    #[doc = "K_STATE."]
    KState = 0x02,
    #[doc = "SE0 (host) / NAK (device)."]
    Se0 = 0x03,
    #[doc = "Packet."]
    Pckt = 0x04,
    #[doc = "FORCE_ENABLE_HS."]
    Hs = 0x05,
    #[doc = "FORCE_ENABLE_FS."]
    Fs = 0x06,
    #[doc = "FORCE_ENABLE_LS."]
    Ls = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ptc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptc {
    #[inline(always)]
    fn from(val: u8) -> Ptc {
        Ptc::from_bits(val)
    }
}
impl From<Ptc> for u8 {
    #[inline(always)]
    fn from(val: Ptc) -> u8 {
        Ptc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ptw {
    #[doc = "Select the 8-bit UTMI interface \\[60 MHz\\]."]
    Utmi8 = 0x0,
    #[doc = "Select the 16-bit UTMI interface \\[30 MHz\\]."]
    Utmi16 = 0x01,
}
impl Ptw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ptw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ptw {
    #[inline(always)]
    fn from(val: u8) -> Ptw {
        Ptw::from_bits(val)
    }
}
impl From<Ptw> for u8 {
    #[inline(always)]
    fn from(val: Ptw) -> u8 {
        Ptw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Slom {
    #[doc = "Setup Lockouts On (default);."]
    LockoutOn = 0x0,
    #[doc = "Setup Lockouts Off."]
    LockoutOff = 0x01,
}
impl Slom {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Slom {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Slom {
    #[inline(always)]
    fn from(val: u8) -> Slom {
        Slom::from_bits(val)
    }
}
impl From<Slom> for u8 {
    #[inline(always)]
    fn from(val: Slom) -> u8 {
        Slom::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm {
    #[doc = "No Serial Engine, always use parallel signalling."]
    SerialEngineNo = 0x0,
    #[doc = "Serial Engine present, always use serial signalling for FS/LS."]
    SerialEngineEn = 0x01,
    #[doc = "Software programmable - Reset to use parallel signalling for FS/LS."]
    SwRstParallel = 0x02,
    #[doc = "Software programmable - Reset to use serial signalling for FS/LS."]
    SwRstSerialEng = 0x03,
}
impl Sm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm {
    #[inline(always)]
    fn from(val: u8) -> Sm {
        Sm::from_bits(val)
    }
}
impl From<Sm> for u8 {
    #[inline(always)]
    fn from(val: Sm) -> u8 {
        Sm::to_bits(val)
    }
}
