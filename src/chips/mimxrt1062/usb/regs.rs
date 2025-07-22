#[doc = "Next Asynch. Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asynclistaddr(pub u32);
impl Asynclistaddr {
    #[doc = "Link Pointer Low (LPL)"]
    #[must_use]
    #[inline(always)]
    pub const fn asybase(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Link Pointer Low (LPL)"]
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
#[doc = "Programmable Burst Size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Burstsize(pub u32);
impl Burstsize {
    #[doc = "Programmable RX Burst Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rxpburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable RX Burst Size"]
    #[inline(always)]
    pub const fn set_rxpburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Programmable TX Burst Size"]
    #[must_use]
    #[inline(always)]
    pub const fn txpburst(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "Programmable TX Burst Size"]
    #[inline(always)]
    pub const fn set_txpburst(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
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
            "Burstsize {{ rxpburst: {=u8:?}, txpburst: {=u16:?} }}",
            self.rxpburst(),
            self.txpburst()
        )
    }
}
#[doc = "Capability Registers Length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caplength(pub u8);
impl Caplength {
    #[doc = "These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
    #[must_use]
    #[inline(always)]
    pub const fn caplength(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits are used as an offset to add to register base to find the beginning of the Operational Register"]
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
#[doc = "Configure Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configflag(pub u32);
impl Configflag {
    #[doc = "Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn cf(&self) -> super::vals::Cf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cf::from_bits(val as u8)
    }
    #[doc = "Configure Flag Host software sets this bit as the last action in its process of configuring the Host Controller"]
    #[inline(always)]
    pub const fn set_cf(&mut self, val: super::vals::Cf) {
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
#[doc = "Device Controller Capability Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dccparams(pub u32);
impl Dccparams {
    #[doc = "Device Endpoint Number This field indicates the number of endpoints built into the device controller"]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number This field indicates the number of endpoints built into the device controller"]
    #[inline(always)]
    pub const fn set_den(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device."]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Device Capable When this bit is 1, this controller is capable of operating as a USB 2.0 device."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Host Capable When this bit is 1, this controller is capable of operating as an EHCI compatible USB 2"]
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
#[doc = "Device Controller Interface Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dciversion(pub u16);
impl Dciversion {
    #[doc = "Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
    #[must_use]
    #[inline(always)]
    pub const fn dciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device Controller Interface Version Number Default value is '01h', which means rev0.1."]
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
#[doc = "Device Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deviceaddr(pub u32);
impl Deviceaddr {
    #[doc = "Device Address Advance"]
    #[must_use]
    #[inline(always)]
    pub const fn usbadra(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device Address Advance"]
    #[inline(always)]
    pub const fn set_usbadra(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device Address. These bits correspond to the USB device address"]
    #[must_use]
    #[inline(always)]
    pub const fn usbadr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address. These bits correspond to the USB device address"]
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
#[doc = "Endpoint Complete"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptcomplete(pub u32);
impl Endptcomplete {
    #[doc = "Endpoint Receive Complete Event - RW/C"]
    #[must_use]
    #[inline(always)]
    pub const fn erce(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Complete Event - RW/C"]
    #[inline(always)]
    pub const fn set_erce(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Complete Event - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn etce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Complete Event - R/WC"]
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
#[doc = "Endpoint Control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EndptctrlX(pub u32);
impl EndptctrlX {
    #[doc = "RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall - Read/Write 0 End Point OK"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "RX Endpoint Type - Read/Write 00 Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\] 1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall - Read/Write 0 End Point OK \\[Default\\] 1 End Point Stalled Software can write a one to this bit to force the endpoint to return a STALL handshake to the Host"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "TX Endpoint Type - Read/Write 00 - Control Endpoint0 is fixed as a Control End Point."]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable 1 Enabled Endpoint0 is always enabled."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for EndptctrlX {
    #[inline(always)]
    fn default() -> EndptctrlX {
        EndptctrlX(0)
    }
}
impl core::fmt::Debug for EndptctrlX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EndptctrlX")
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
impl defmt::Format for EndptctrlX {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EndptctrlX {{ rxs: {=bool:?}, rxt: {=u8:?}, rxe: {=bool:?}, txs: {=bool:?}, txt: {=u8:?}, txe: {=bool:?} }}",
            self.rxs(),
            self.rxt(),
            self.rxe(),
            self.txs(),
            self.txt(),
            self.txe()
        )
    }
}
#[doc = "Endpoint Flush"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptflush(pub u32);
impl Endptflush {
    #[doc = "Flush Endpoint Receive Buffer - R/WS"]
    #[must_use]
    #[inline(always)]
    pub const fn ferb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub const fn set_ferb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Flush Endpoint Transmit Buffer - R/WS"]
    #[must_use]
    #[inline(always)]
    pub const fn fetb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Transmit Buffer - R/WS"]
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
#[doc = "Endpoint List Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptlistaddr(pub u32);
impl Endptlistaddr {
    #[doc = "Endpoint List Pointer(Low)"]
    #[must_use]
    #[inline(always)]
    pub const fn epbase(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Endpoint List Pointer(Low)"]
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
#[doc = "Endpoint NAK"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptnak(pub u32);
impl Endptnak {
    #[doc = "RX Endpoint NAK - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn eprn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK - R/WC"]
    #[inline(always)]
    pub const fn set_eprn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn eptn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK - R/WC"]
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
#[doc = "Endpoint NAK Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptnaken(pub u32);
impl Endptnaken {
    #[doc = "RX Endpoint NAK Enable - R/W"]
    #[must_use]
    #[inline(always)]
    pub const fn eprne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK Enable - R/W"]
    #[inline(always)]
    pub const fn set_eprne(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK Enable - R/W"]
    #[must_use]
    #[inline(always)]
    pub const fn eptne(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK Enable - R/W"]
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
#[doc = "Endpoint Prime"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptprime(pub u32);
impl Endptprime {
    #[doc = "Prime Endpoint Receive Buffer - R/WS"]
    #[must_use]
    #[inline(always)]
    pub const fn perb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Receive Buffer - R/WS"]
    #[inline(always)]
    pub const fn set_perb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Prime Endpoint Transmit Buffer - R/WS"]
    #[must_use]
    #[inline(always)]
    pub const fn petb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Transmit Buffer - R/WS"]
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
#[doc = "Endpoint Setup Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptsetupstat(pub u32);
impl Endptsetupstat {
    #[doc = "Setup Endpoint Status"]
    #[must_use]
    #[inline(always)]
    pub const fn endptsetupstat(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Setup Endpoint Status"]
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
#[doc = "Endpoint Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptstat(pub u32);
impl Endptstat {
    #[doc = "Endpoint Receive Buffer Ready -- Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn erbr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Buffer Ready -- Read Only"]
    #[inline(always)]
    pub const fn set_erbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Buffer Ready -- Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn etbr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Buffer Ready -- Read Only"]
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
#[doc = "USB Frame Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frindex(pub u32);
impl Frindex {
    #[doc = "Frame Index"]
    #[must_use]
    #[inline(always)]
    pub const fn frindex(&self) -> super::vals::Frindex {
        let val = (self.0 >> 0usize) & 0x3fff;
        super::vals::Frindex::from_bits(val as u16)
    }
    #[doc = "Frame Index"]
    #[inline(always)]
    pub const fn set_frindex(&mut self, val: super::vals::Frindex) {
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
#[doc = "General Purpose Timer #0 Controller"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer0ctrl(pub u32);
impl Gptimer0ctrl {
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> super::vals::Gptimer0ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gptimer0ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software; In repeat mode, the timer will count down to zero, generate an interrupt and automatically reload the counter value from GPTLD bits to start again"]
    #[inline(always)]
    pub const fn set_gptmode(&mut self, val: super::vals::Gptimer0ctrlGptmode) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gptrst(&self) -> super::vals::Gptimer0ctrlGptrst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gptimer0ctrlGptrst::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset"]
    #[inline(always)]
    pub const fn set_gptrst(&mut self, val: super::vals::Gptimer0ctrlGptrst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> super::vals::Gptimer0ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gptimer0ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    pub const fn set_gptrun(&mut self, val: super::vals::Gptimer0ctrlGptrun) {
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
#[doc = "General Purpose Timer #0 Load"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer0ld(pub u32);
impl Gptimer0ld {
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
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
#[doc = "General Purpose Timer #1 Controller"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer1ctrl(pub u32);
impl Gptimer1ctrl {
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> super::vals::Gptimer1ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gptimer1ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
    #[inline(always)]
    pub const fn set_gptmode(&mut self, val: super::vals::Gptimer1ctrlGptmode) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn gptrst(&self) -> super::vals::Gptimer1ctrlGptrst {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Gptimer1ctrlGptrst::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Reset"]
    #[inline(always)]
    pub const fn set_gptrst(&mut self, val: super::vals::Gptimer1ctrlGptrst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> super::vals::Gptimer1ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gptimer1ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    pub const fn set_gptrun(&mut self, val: super::vals::Gptimer1ctrlGptrun) {
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
#[doc = "General Purpose Timer #1 Load"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gptimer1ld(pub u32);
impl Gptimer1ld {
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value These bit fields are loaded to GPTCNT bits when GPTRST bit is set '1b'"]
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
#[doc = "Host Controller Capability Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hccparams(pub u32);
impl Hccparams {
    #[doc = "64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "64-bit Addressing Capability This bit is set '0b' in all controller core, no 64-bit addressing capability is supported"]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
    #[must_use]
    #[inline(always)]
    pub const fn pfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Frame List Flag If this bit is set to zero, then the system software must use a frame list length of 1024 elements with this host controller"]
    #[inline(always)]
    pub const fn set_pfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Capability If this bit is set to a one, then the host controller supports the park feature for high-speed queue heads in the Asynchronous Schedule"]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Isochronous Scheduling Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn ist(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub const fn set_ist(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "EHCI Extended Capabilities Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn eecp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "EHCI Extended Capabilities Pointer"]
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
#[doc = "Host Controller Interface Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hciversion(pub u16);
impl Hciversion {
    #[doc = "Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
    #[must_use]
    #[inline(always)]
    pub const fn hciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Host Controller Interface Version Number Default value is '10h', which means EHCI rev1.0."]
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
#[doc = "Host Controller Structural Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hcsparams(pub u32);
impl Hcsparams {
    #[doc = "Number of downstream ports"]
    #[must_use]
    #[inline(always)]
    pub const fn n_ports(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of downstream ports"]
    #[inline(always)]
    pub const fn set_n_ports(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Power Control This field indicates whether the host controller implementation includes port power control"]
    #[must_use]
    #[inline(always)]
    pub const fn ppc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power Control This field indicates whether the host controller implementation includes port power control"]
    #[inline(always)]
    pub const fn set_ppc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn n_pcc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
    #[inline(always)]
    pub const fn set_n_pcc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of Companion Controller (N_CC)"]
    #[must_use]
    #[inline(always)]
    pub const fn n_cc(&self) -> super::vals::NCc {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::NCc::from_bits(val as u8)
    }
    #[doc = "Number of Companion Controller (N_CC)"]
    #[inline(always)]
    pub const fn set_n_cc(&mut self, val: super::vals::NCc) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
    #[must_use]
    #[inline(always)]
    pub const fn pi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
    #[inline(always)]
    pub const fn set_pi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Number of Ports per Transaction Translator (N_PTT)"]
    #[must_use]
    #[inline(always)]
    pub const fn n_ptt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Transaction Translator (N_PTT)"]
    #[inline(always)]
    pub const fn set_n_ptt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of Transaction Translators (N_TT)"]
    #[must_use]
    #[inline(always)]
    pub const fn n_tt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Transaction Translators (N_TT)"]
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
#[doc = "Device Hardware Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwdevice(pub u32);
impl Hwdevice {
    #[doc = "Device Capable. Indicating whether device operation mode is supported or not."]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> super::vals::Dc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dc::from_bits(val as u8)
    }
    #[doc = "Device Capable. Indicating whether device operation mode is supported or not."]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: super::vals::Dc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Device Endpoint Number"]
    #[must_use]
    #[inline(always)]
    pub const fn devep(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number"]
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
#[doc = "Hardware General"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwgeneral(pub u32);
impl Hwgeneral {
    #[doc = "Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    #[must_use]
    #[inline(always)]
    pub const fn phyw(&self) -> super::vals::Phyw {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Phyw::from_bits(val as u8)
    }
    #[doc = "Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    #[inline(always)]
    pub const fn set_phyw(&mut self, val: super::vals::Phyw) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Transciever type"]
    #[must_use]
    #[inline(always)]
    pub const fn phym(&self) -> super::vals::Phym {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Phym::from_bits(val as u8)
    }
    #[doc = "Transciever type"]
    #[inline(always)]
    pub const fn set_phym(&mut self, val: super::vals::Phym) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
    }
    #[doc = "Serial interface mode capability"]
    #[must_use]
    #[inline(always)]
    pub const fn sm(&self) -> super::vals::Sm {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Sm::from_bits(val as u8)
    }
    #[doc = "Serial interface mode capability"]
    #[inline(always)]
    pub const fn set_sm(&mut self, val: super::vals::Sm) {
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
#[doc = "Host Hardware Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwhost(pub u32);
impl Hwhost {
    #[doc = "Host Capable. Indicating whether host operation mode is supported or not."]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> super::vals::Hc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hc::from_bits(val as u8)
    }
    #[doc = "Host Capable. Indicating whether host operation mode is supported or not."]
    #[inline(always)]
    pub const fn set_hc(&mut self, val: super::vals::Hc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
    #[must_use]
    #[inline(always)]
    pub const fn nport(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "The Nmber of downstream ports supported by the host controller is (NPORT+1)"]
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
#[doc = "RX Buffer Hardware Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwrxbuf(pub u32);
impl Hwrxbuf {
    #[doc = "Default burst size for memory to RX buffer transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn rxburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default burst size for memory to RX buffer transfer"]
    #[inline(always)]
    pub const fn set_rxburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Buffer total size for all receive endpoints is (2^RXADD)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxadd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Buffer total size for all receive endpoints is (2^RXADD)"]
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
#[doc = "TX Buffer Hardware Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwtxbuf(pub u32);
impl Hwtxbuf {
    #[doc = "Default burst size for memory to TX buffer transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn txburst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Default burst size for memory to TX buffer transfer"]
    #[inline(always)]
    pub const fn set_txburst(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn txchanadd(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX FIFO Buffer size is: (2^TXCHANADD) * 4 Bytes"]
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
#[doc = "Identification register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Configuration number"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Configuration number"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Complement version of ID"]
    #[must_use]
    #[inline(always)]
    pub const fn nid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Complement version of ID"]
    #[inline(always)]
    pub const fn set_nid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Revision number of the controller core."]
    #[must_use]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Revision number of the controller core."]
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
#[doc = "On-The-Go Status & control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgsc(pub u32);
impl Otgsc {
    #[doc = "VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[must_use]
    #[inline(always)]
    pub const fn vd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub const fn set_vd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Charge - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn vc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Charge - Read/Write"]
    #[inline(always)]
    pub const fn set_vc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OTG Termination - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn ot(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Termination - Read/Write"]
    #[inline(always)]
    pub const fn set_ot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Pulsing - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn dp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulsing - Read/Write"]
    #[inline(always)]
    pub const fn set_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn idpu(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[inline(always)]
    pub const fn set_idpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB ID - Read Only. 0 = A device, 1 = B device"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID - Read Only. 0 = A device, 1 = B device"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn avv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[inline(always)]
    pub const fn set_avv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn asv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[inline(always)]
    pub const fn set_asv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn bsv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
    #[inline(always)]
    pub const fn set_bsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "B Session End - Read Only. Indicates VBus is below the B session end threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn bse(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End - Read Only. Indicates VBus is below the B session end threshold."]
    #[inline(always)]
    pub const fn set_bse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
    #[must_use]
    #[inline(always)]
    pub const fn tog_1ms(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
    #[inline(always)]
    pub const fn set_tog_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Bus Pulsing Status - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn dps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Bus Pulsing Status - Read Only"]
    #[inline(always)]
    pub const fn set_dps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB ID Interrupt Status - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn idis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Status - Read/Write"]
    #[inline(always)]
    pub const fn set_idis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn avvis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_avvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A Session Valid Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn asvis(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_asvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "B Session Valid Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn bsvis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_bsvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "B Session End Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn bseis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_bseis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn status_1ms(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_status_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Pulse Interrupt Status - Read/Write to Clear"]
    #[must_use]
    #[inline(always)]
    pub const fn dpis(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub const fn set_dpis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn idie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub const fn set_idie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn avvie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub const fn set_avvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "A Session Valid Interrupt Enable - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn asvie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub const fn set_asvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "B Session Valid Interrupt Enable - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn bsvie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub const fn set_bsvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn bseie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[inline(always)]
    pub const fn set_bseie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "1 millisecond timer Interrupt Enable - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn en_1ms(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "1 millisecond timer Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub const fn set_en_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Data Pulse Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dpie(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Enable"]
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
#[doc = "Frame List Base Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Periodiclistbase(pub u32);
impl Periodiclistbase {
    #[doc = "Base Address (Low)"]
    #[must_use]
    #[inline(always)]
    pub const fn baseadr(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Base Address (Low)"]
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
#[doc = "Port Status & Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Portsc1(pub u32);
impl Portsc1 {
    #[doc = "Current Connect Status-Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Connect Status-Read Only"]
    #[inline(always)]
    pub const fn set_ccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change-R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn csc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect Status Change-R/WC"]
    #[inline(always)]
    pub const fn set_csc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enabled/Disabled-Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled-Read/Write"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable/Disable Change-R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn pec(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enable/Disable Change-R/WC"]
    #[inline(always)]
    pub const fn set_pec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Over-current Active-Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn oca(&self) -> super::vals::Oca {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Oca::from_bits(val as u8)
    }
    #[doc = "Over-current Active-Read Only"]
    #[inline(always)]
    pub const fn set_oca(&mut self, val: super::vals::Oca) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Over-current Change-R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn occ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current Change-R/WC"]
    #[inline(always)]
    pub const fn set_occ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume -Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn fpr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Port Resume -Read/Write"]
    #[inline(always)]
    pub const fn set_fpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend - Read/Write or Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend - Read/Write or Read Only"]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset - Read/Write or Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset - Read/Write or Read Only"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "High-Speed Port - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn hsp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High-Speed Port - Read Only"]
    #[inline(always)]
    pub const fn set_hsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Line Status-Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ls(&self) -> super::vals::Ls {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Ls::from_bits(val as u8)
    }
    #[doc = "Line Status-Read Only"]
    #[inline(always)]
    pub const fn set_ls(&mut self, val: super::vals::Ls) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power (PP)-Read/Write or Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn pp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power (PP)-Read/Write or Read Only"]
    #[inline(always)]
    pub const fn set_pp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Owner-Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn po(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Owner-Read/Write"]
    #[inline(always)]
    pub const fn set_po(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Indicator Control - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn pic(&self) -> super::vals::Pic {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pic::from_bits(val as u8)
    }
    #[doc = "Port Indicator Control - Read/Write"]
    #[inline(always)]
    pub const fn set_pic(&mut self, val: super::vals::Pic) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc(&self) -> super::vals::Ptc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ptc::from_bits(val as u8)
    }
    #[doc = "Port Test Control - Read/Write"]
    #[inline(always)]
    pub const fn set_ptc(&mut self, val: super::vals::Ptc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn wkcn(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E) - Read/Write"]
    #[inline(always)]
    pub const fn set_wkcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn wkdc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E) - Read/Write"]
    #[inline(always)]
    pub const fn set_wkdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn wkoc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Over-current Enable (WKOC_E) - Read/Write"]
    #[inline(always)]
    pub const fn set_wkoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn phcd(&self) -> super::vals::Phcd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Phcd::from_bits(val as u8)
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD) - Read/Write"]
    #[inline(always)]
    pub const fn set_phcd(&mut self, val: super::vals::Phcd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Force Full Speed Connect - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn pfsc(&self) -> super::vals::Pfsc {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pfsc::from_bits(val as u8)
    }
    #[doc = "Port Force Full Speed Connect - Read/Write"]
    #[inline(always)]
    pub const fn set_pfsc(&mut self, val: super::vals::Pfsc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "See description at bits 31-30"]
    #[must_use]
    #[inline(always)]
    pub const fn pts_2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "See description at bits 31-30"]
    #[inline(always)]
    pub const fn set_pts_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[must_use]
    #[inline(always)]
    pub const fn pspd(&self) -> super::vals::Pspd {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Pspd::from_bits(val as u8)
    }
    #[doc = "Port Speed - Read Only. This register field indicates the speed at which the port is operating."]
    #[inline(always)]
    pub const fn set_pspd(&mut self, val: super::vals::Pspd) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[must_use]
    #[inline(always)]
    pub const fn ptw(&self) -> super::vals::Ptw {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ptw::from_bits(val as u8)
    }
    #[doc = "Parallel Transceiver Width This bit has no effect if serial interface engine is used"]
    #[inline(always)]
    pub const fn set_ptw(&mut self, val: super::vals::Ptw) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Transceiver Select 1 Serial Interface Engine is selected 0 Parallel Interface signals is selected Serial Interface Engine can be used in combination with UTMI+/ULPI physical interface to provide FS/LS signaling instead of the parallel interface signals"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "All USB port interface modes are listed in this field description, but not all are supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pts_1(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "All USB port interface modes are listed in this field description, but not all are supported"]
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
            "Portsc1 {{ ccs: {=bool:?}, csc: {=bool:?}, pe: {=bool:?}, pec: {=bool:?}, oca: {:?}, occ: {=bool:?}, fpr: {=bool:?}, susp: {=bool:?}, pr: {=bool:?}, hsp: {=bool:?}, ls: {:?}, pp: {=bool:?}, po: {=bool:?}, pic: {:?}, ptc: {:?}, wkcn: {=bool:?}, wkdc: {=bool:?}, wkoc: {=bool:?}, phcd: {:?}, pfsc: {:?}, pts_2: {=bool:?}, pspd: {:?}, ptw: {:?}, sts: {=bool:?}, pts_1: {=u8:?} }}",
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
#[doc = "System Bus Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbuscfg(pub u32);
impl Sbuscfg {
    #[doc = "AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbrst(&self) -> super::vals::Ahbbrst {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ahbbrst::from_bits(val as u8)
    }
    #[doc = "AHB master interface Burst configuration These bits control AHB master transfer type sequence (or priority)"]
    #[inline(always)]
    pub const fn set_ahbbrst(&mut self, val: super::vals::Ahbbrst) {
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
#[doc = "TX FIFO Fill Tuning"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txfilltuning(pub u32);
impl Txfilltuning {
    #[doc = "Scheduler Overhead"]
    #[must_use]
    #[inline(always)]
    pub const fn txschoh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Scheduler Overhead"]
    #[inline(always)]
    pub const fn set_txschoh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Scheduler Health Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn txschhealth(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Scheduler Health Counter"]
    #[inline(always)]
    pub const fn set_txschhealth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "FIFO Burst Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn txfifothres(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "FIFO Burst Threshold"]
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
#[doc = "USB Command Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcmd(pub u32);
impl Usbcmd {
    #[doc = "Run/Stop (RS) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run/Stop (RS) - Read/Write"]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controller Reset (RESET) - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Reset (RESET) - Read/Write"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "See description at bit 15"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "See description at bit 15"]
    #[inline(always)]
    pub const fn set_fs_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Periodic Schedule Enable- Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn pse(&self) -> super::vals::Pse {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pse::from_bits(val as u8)
    }
    #[doc = "Periodic Schedule Enable- Read/Write"]
    #[inline(always)]
    pub const fn set_pse(&mut self, val: super::vals::Pse) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Schedule Enable - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn ase(&self) -> super::vals::Ase {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ase::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Enable - Read/Write"]
    #[inline(always)]
    pub const fn set_ase(&mut self, val: super::vals::Ase) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on Async Advance Doorbell - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn iaa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance Doorbell - Read/Write"]
    #[inline(always)]
    pub const fn set_iaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Count - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Asynchronous Schedule Park Mode Count - Read/Write"]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn aspe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Mode Enable - Read/Write"]
    #[inline(always)]
    pub const fn set_aspe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Setup TripWire - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn sutw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Setup TripWire - Read/Write"]
    #[inline(always)]
    pub const fn set_sutw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Add dTD TripWire - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn atdtw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Add dTD TripWire - Read/Write"]
    #[inline(always)]
    pub const fn set_atdtw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Frame List Size - (Read/Write or Read Only)"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_2(&self) -> super::vals::Fs2 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Fs2::from_bits(val as u8)
    }
    #[doc = "Frame List Size - (Read/Write or Read Only)"]
    #[inline(always)]
    pub const fn set_fs_2(&mut self, val: super::vals::Fs2) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Threshold Control -Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn itc(&self) -> super::vals::Itc {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Itc::from_bits(val as u8)
    }
    #[doc = "Interrupt Threshold Control -Read/Write"]
    #[inline(always)]
    pub const fn set_itc(&mut self, val: super::vals::Itc) {
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
            "Usbcmd {{ rs: {=bool:?}, rst: {=bool:?}, fs_1: {=u8:?}, pse: {:?}, ase: {:?}, iaa: {=bool:?}, asp: {=u8:?}, aspe: {=bool:?}, sutw: {=bool:?}, atdtw: {=bool:?}, fs_2: {:?}, itc: {:?} }}",
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
#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbintr(pub u32);
impl Usbintr {
    #[doc = "USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt Enable When this bit is one and the UI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn uee(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt Enable When this bit is one and the UEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_uee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect Interrupt Enable When this bit is one and the PCI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_pce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn fre(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover Interrupt Enable When this bit is one and the FRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_fre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn see(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error Interrupt Enable When this bit is one and the SEI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_see(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn aae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Async Advance Interrupt Enable When this bit is one and the AAI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_aae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ure(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Interrupt Enable When this bit is one and the URI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_ure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received Interrupt Enable When this bit is one and the SRI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Interrupt Enable When this bit is one and the SLI bit in n_n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_sle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ulpie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ULPI Interrupt Enable When this bit is one and the UPLII bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_ulpie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn nake(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Enable When this bit is one and the NAKI bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_nake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn uaie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Asynchronous Interrupt Enable When this bit is one, and the UAI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub const fn set_uaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn upie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Periodic Interrupt Enable When this bit is one, and the UPI bit in the n_USBSTS register is one, host controller will issue an interrupt at the next interrupt threshold"]
    #[inline(always)]
    pub const fn set_upie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable When this bit is one and the TI0 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable When this bit is one and the TI1 bit in n_USBSTS register is a one the controller will issue an interrupt"]
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
            .field("ulpie", &self.ulpie())
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
            "Usbintr {{ ue: {=bool:?}, uee: {=bool:?}, pce: {=bool:?}, fre: {=bool:?}, see: {=bool:?}, aae: {=bool:?}, ure: {=bool:?}, sre: {=bool:?}, sle: {=bool:?}, ulpie: {=bool:?}, nake: {=bool:?}, uaie: {=bool:?}, upie: {=bool:?}, tie0: {=bool:?}, tie1: {=bool:?} }}",
            self.ue(),
            self.uee(),
            self.pce(),
            self.fre(),
            self.see(),
            self.aae(),
            self.ure(),
            self.sre(),
            self.sle(),
            self.ulpie(),
            self.nake(),
            self.uaie(),
            self.upie(),
            self.tie0(),
            self.tie1()
        )
    }
}
#[doc = "USB Device Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbmode(pub u32);
impl Usbmode {
    #[doc = "Controller Mode - R/WO"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Cm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cm::from_bits(val as u8)
    }
    #[doc = "Controller Mode - R/WO"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Cm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Endian Select - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn es(&self) -> super::vals::Es {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Es::from_bits(val as u8)
    }
    #[doc = "Endian Select - Read/Write"]
    #[inline(always)]
    pub const fn set_es(&mut self, val: super::vals::Es) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Setup Lockout Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slom(&self) -> super::vals::Slom {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Slom::from_bits(val as u8)
    }
    #[doc = "Setup Lockout Mode"]
    #[inline(always)]
    pub const fn set_slom(&mut self, val: super::vals::Slom) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Stream Disable Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sdis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Disable Mode"]
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
#[doc = "USB Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbsts(pub u32);
impl Usbsts {
    #[doc = "USB Interrupt (USBINT) - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn ui(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt (USBINT) - R/WC"]
    #[inline(always)]
    pub const fn set_ui(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt (USBERRINT) - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn uei(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt (USBERRINT) - R/WC"]
    #[inline(always)]
    pub const fn set_uei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn pci(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect - R/WC"]
    #[inline(always)]
    pub const fn set_pci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn fri(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover - R/WC"]
    #[inline(always)]
    pub const fn set_fri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error- R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn sei(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error- R/WC"]
    #[inline(always)]
    pub const fn set_sei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Async Advance - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn aai(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance - R/WC"]
    #[inline(always)]
    pub const fn set_aai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Received - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn uri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Received - R/WC"]
    #[inline(always)]
    pub const fn set_uri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn sri(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received - R/WC"]
    #[inline(always)]
    pub const fn set_sri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DCSuspend - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn sli(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DCSuspend - R/WC"]
    #[inline(always)]
    pub const fn set_sli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ULPI Interrupt - R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn ulpii(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ULPI Interrupt - R/WC"]
    #[inline(always)]
    pub const fn set_ulpii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HCHaIted - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn hch(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HCHaIted - Read Only"]
    #[inline(always)]
    pub const fn set_hch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reclamation - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn rcl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reclamation - Read Only"]
    #[inline(always)]
    pub const fn set_rcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Periodic Schedule Status - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic Schedule Status - Read Only"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Asynchronous Schedule Status - Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Status - Read Only"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Interrupt Bit--RO"]
    #[must_use]
    #[inline(always)]
    pub const fn naki(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Bit--RO"]
    #[inline(always)]
    pub const fn set_naki(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn ti0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[inline(always)]
    pub const fn set_ti0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    #[must_use]
    #[inline(always)]
    pub const fn ti1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
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
