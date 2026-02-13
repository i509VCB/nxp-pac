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
    pub const fn txpburst(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Programmable TX Burst Size"]
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
#[doc = "Capability Registers Length"]
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
#[doc = "Configure Flag"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configflag(pub u32);
impl Configflag {
    #[doc = "Configure Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cf(&self) -> super::vals::Cf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cf::from_bits(val as u8)
    }
    #[doc = "Configure Flag"]
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
    #[doc = "Device Endpoint Number"]
    #[must_use]
    #[inline(always)]
    pub const fn den(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Device Endpoint Number"]
    #[inline(always)]
    pub const fn set_den(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Device Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Device Capable"]
    #[inline(always)]
    pub const fn set_dc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Host Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Host Capable"]
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
    #[doc = "Device Controller Interface Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn dciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Device Controller Interface Version Number"]
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
    #[doc = "Device Address"]
    #[must_use]
    #[inline(always)]
    pub const fn usbadr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Device Address"]
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
    #[doc = "Endpoint Receive Complete Event"]
    #[must_use]
    #[inline(always)]
    pub const fn erce(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Complete Event"]
    #[inline(always)]
    pub const fn set_erce(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Complete Event"]
    #[must_use]
    #[inline(always)]
    pub const fn etce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Complete Event"]
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
#[doc = "Endpoint Control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl0(pub u32);
impl Endptctrl0 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl0Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl0Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl0Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl0Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl0Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl0Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
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
#[doc = "Endpoint Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl1(pub u32);
impl Endptctrl1 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl1Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl1Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl1Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl1Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl1Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl1Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl1Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl1Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl1Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl1 {
    #[inline(always)]
    fn default() -> Endptctrl1 {
        Endptctrl1(0)
    }
}
impl core::fmt::Debug for Endptctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl1")
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
impl defmt::Format for Endptctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl1 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl2(pub u32);
impl Endptctrl2 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl2Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl2Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl2Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl2Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl2Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl2Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl2Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl2Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl2Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl2 {
    #[inline(always)]
    fn default() -> Endptctrl2 {
        Endptctrl2(0)
    }
}
impl core::fmt::Debug for Endptctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl2")
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
impl defmt::Format for Endptctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl2 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl3(pub u32);
impl Endptctrl3 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl3Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl3Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl3Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl3Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl3Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl3Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl3Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl3Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl3Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl3 {
    #[inline(always)]
    fn default() -> Endptctrl3 {
        Endptctrl3(0)
    }
}
impl core::fmt::Debug for Endptctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl3")
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
impl defmt::Format for Endptctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl3 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl4(pub u32);
impl Endptctrl4 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl4Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl4Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl4Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl4Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl4Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl4Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl4Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl4Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl4Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl4 {
    #[inline(always)]
    fn default() -> Endptctrl4 {
        Endptctrl4(0)
    }
}
impl core::fmt::Debug for Endptctrl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl4")
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
impl defmt::Format for Endptctrl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl4 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl5(pub u32);
impl Endptctrl5 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl5Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl5Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl5Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl5Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl5Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl5Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl5Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl5Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl5Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl5 {
    #[inline(always)]
    fn default() -> Endptctrl5 {
        Endptctrl5(0)
    }
}
impl core::fmt::Debug for Endptctrl5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl5")
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
impl defmt::Format for Endptctrl5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl5 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl6(pub u32);
impl Endptctrl6 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl6Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl6Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl6Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl6Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl6Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl6Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl6Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl6Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl6Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl6 {
    #[inline(always)]
    fn default() -> Endptctrl6 {
        Endptctrl6(0)
    }
}
impl core::fmt::Debug for Endptctrl6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl6")
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
impl defmt::Format for Endptctrl6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl6 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Control 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptctrl7(pub u32);
impl Endptctrl7 {
    #[doc = "RX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn rxs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_rxs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX Endpoint Data Sink"]
    #[must_use]
    #[inline(always)]
    pub const fn rxd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Data Sink"]
    #[inline(always)]
    pub const fn set_rxd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn rxt(&self) -> super::vals::Endptctrl7Rxt {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Endptctrl7Rxt::from_bits(val as u8)
    }
    #[doc = "RX Endpoint Type"]
    #[inline(always)]
    pub const fn set_rxt(&mut self, val: super::vals::Endptctrl7Rxt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn rxi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_rxi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "RX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_rxr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "RX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "RX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "TX Endpoint Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn txs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Stall"]
    #[inline(always)]
    pub const fn set_txs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TX Endpoint Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn txd(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Data Source"]
    #[inline(always)]
    pub const fn set_txd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "TX Endpoint Type"]
    #[must_use]
    #[inline(always)]
    pub const fn txt(&self) -> super::vals::Endptctrl7Txt {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::Endptctrl7Txt::from_bits(val as u8)
    }
    #[doc = "TX Endpoint Type"]
    #[inline(always)]
    pub const fn set_txt(&mut self, val: super::vals::Endptctrl7Txt) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[must_use]
    #[inline(always)]
    pub const fn txi(&self) -> super::vals::Endptctrl7Txi {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Endptctrl7Txi::from_bits(val as u8)
    }
    #[doc = "TX Data Toggle Inhibit"]
    #[inline(always)]
    pub const fn set_txi(&mut self, val: super::vals::Endptctrl7Txi) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[must_use]
    #[inline(always)]
    pub const fn txr(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "TX Data Toggle Reset (WS)"]
    #[inline(always)]
    pub const fn set_txr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "TX Endpoint Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "TX Endpoint Enable"]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Endptctrl7 {
    #[inline(always)]
    fn default() -> Endptctrl7 {
        Endptctrl7(0)
    }
}
impl core::fmt::Debug for Endptctrl7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Endptctrl7")
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
impl defmt::Format for Endptctrl7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Endptctrl7 {{ rxs: {=bool:?}, rxd: {=bool:?}, rxt: {:?}, rxi: {=bool:?}, rxr: {=bool:?}, rxe: {=bool:?}, txs: {=bool:?}, txd: {=bool:?}, txt: {:?}, txi: {:?}, txr: {=bool:?}, txe: {=bool:?} }}",
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
#[doc = "Endpoint Flush"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endptflush(pub u32);
impl Endptflush {
    #[doc = "Flush Endpoint Receive Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn ferb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Receive Buffer"]
    #[inline(always)]
    pub const fn set_ferb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Flush Endpoint Transmit Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn fetb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Flush Endpoint Transmit Buffer"]
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
    #[doc = "Endpoint List Pointer (Low)"]
    #[must_use]
    #[inline(always)]
    pub const fn epbase(&self) -> u32 {
        let val = (self.0 >> 11usize) & 0x001f_ffff;
        val as u32
    }
    #[doc = "Endpoint List Pointer (Low)"]
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
    #[doc = "RX Endpoint NAK"]
    #[must_use]
    #[inline(always)]
    pub const fn eprn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK"]
    #[inline(always)]
    pub const fn set_eprn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK"]
    #[must_use]
    #[inline(always)]
    pub const fn eptn(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK"]
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
    #[doc = "RX Endpoint NAK Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eprne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RX Endpoint NAK Enable"]
    #[inline(always)]
    pub const fn set_eprne(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "TX Endpoint NAK Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eptne(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "TX Endpoint NAK Enable"]
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
    #[doc = "Prime Endpoint Receive Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn perb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Receive Buffer"]
    #[inline(always)]
    pub const fn set_perb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Prime Endpoint Transmit Buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn petb(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Prime Endpoint Transmit Buffer"]
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
    #[doc = "Endpoint Receive Buffer Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn erbr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Receive Buffer Ready"]
    #[inline(always)]
    pub const fn set_erbr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Endpoint Transmit Buffer Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn etbr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Endpoint Transmit Buffer Ready"]
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
    #[doc = "General Purpose Timer Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter"]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> super::vals::Gptimer0ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gptimer0ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode"]
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
    #[doc = "General Purpose Timer Run"]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> super::vals::Gptimer0ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gptimer0ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run"]
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
    #[doc = "General Purpose Timer Load Value"]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value"]
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
    #[doc = "General Purpose Timer Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn gptcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Counter"]
    #[inline(always)]
    pub const fn set_gptcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "General Purpose Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn gptmode(&self) -> super::vals::Gptimer1ctrlGptmode {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gptimer1ctrlGptmode::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Mode"]
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
    #[doc = "General Purpose Timer Run"]
    #[must_use]
    #[inline(always)]
    pub const fn gptrun(&self) -> super::vals::Gptimer1ctrlGptrun {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Gptimer1ctrlGptrun::from_bits(val as u8)
    }
    #[doc = "General Purpose Timer Run"]
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
    #[doc = "General Purpose Timer Load Value"]
    #[must_use]
    #[inline(always)]
    pub const fn gptld(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "General Purpose Timer Load Value"]
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
    #[doc = "64-bit Addressing Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn adc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "64-bit Addressing Capability"]
    #[inline(always)]
    pub const fn set_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Programmable Frame List Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Programmable Frame List Flag"]
    #[inline(always)]
    pub const fn set_pfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Asynchronous Schedule Park Capability"]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Capability"]
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
    #[doc = "Host Controller Interface Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn hciversion(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Host Controller Interface Version Number"]
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
    #[doc = "Number of Downstream Ports"]
    #[must_use]
    #[inline(always)]
    pub const fn n_ports(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Downstream Ports"]
    #[inline(always)]
    pub const fn set_n_ports(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Port Power Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ppc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power Control"]
    #[inline(always)]
    pub const fn set_ppc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Ports per Companion Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn n_pcc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Companion Controller"]
    #[inline(always)]
    pub const fn set_n_pcc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of Companion Controller"]
    #[must_use]
    #[inline(always)]
    pub const fn n_cc(&self) -> super::vals::NCc {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::NCc::from_bits(val as u8)
    }
    #[doc = "Number of Companion Controller"]
    #[inline(always)]
    pub const fn set_n_cc(&mut self, val: super::vals::NCc) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Port Indicators (P INDICATOR)"]
    #[must_use]
    #[inline(always)]
    pub const fn pi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Port Indicators (P INDICATOR)"]
    #[inline(always)]
    pub const fn set_pi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Number of Ports per Transaction Translator"]
    #[must_use]
    #[inline(always)]
    pub const fn n_ptt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Ports per Transaction Translator"]
    #[inline(always)]
    pub const fn set_n_ptt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Number of Transaction Translators"]
    #[must_use]
    #[inline(always)]
    pub const fn n_tt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Transaction Translators"]
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
    #[doc = "Device Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn dc(&self) -> super::vals::Dc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dc::from_bits(val as u8)
    }
    #[doc = "Device Capable"]
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
    #[doc = "Data width of the transceiver connected to the controller core"]
    #[must_use]
    #[inline(always)]
    pub const fn phyw(&self) -> super::vals::Phyw {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Phyw::from_bits(val as u8)
    }
    #[doc = "Data width of the transceiver connected to the controller core"]
    #[inline(always)]
    pub const fn set_phyw(&mut self, val: super::vals::Phyw) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Transceiver Type"]
    #[must_use]
    #[inline(always)]
    pub const fn phym(&self) -> super::vals::Phym {
        let val = (self.0 >> 6usize) & 0x07;
        super::vals::Phym::from_bits(val as u8)
    }
    #[doc = "Transceiver Type"]
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
    #[doc = "Host Capable"]
    #[must_use]
    #[inline(always)]
    pub const fn hc(&self) -> super::vals::Hc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hc::from_bits(val as u8)
    }
    #[doc = "Host Capable"]
    #[inline(always)]
    pub const fn set_hc(&mut self, val: super::vals::Hc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "The Number of downstream ports supported by the host controller is (NPORT+1)"]
    #[must_use]
    #[inline(always)]
    pub const fn nport(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "The Number of downstream ports supported by the host controller is (NPORT+1)"]
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
#[doc = "Identification"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Configuration Number"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Configuration Number"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Complement Version of ID"]
    #[must_use]
    #[inline(always)]
    pub const fn nid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Complement Version of ID"]
    #[inline(always)]
    pub const fn set_nid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Revision Number of the Controller Core"]
    #[must_use]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Revision Number of the Controller Core"]
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
#[doc = "On-The-Go Status & Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otgsc(pub u32);
impl Otgsc {
    #[doc = "VBUS Discharge"]
    #[must_use]
    #[inline(always)]
    pub const fn vd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Discharge"]
    #[inline(always)]
    pub const fn set_vd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VBUS Charge"]
    #[must_use]
    #[inline(always)]
    pub const fn vc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS Charge"]
    #[inline(always)]
    pub const fn set_vc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "OTG Termination"]
    #[must_use]
    #[inline(always)]
    pub const fn ot(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTG Termination"]
    #[inline(always)]
    pub const fn set_ot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Pulsing"]
    #[must_use]
    #[inline(always)]
    pub const fn dp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulsing"]
    #[inline(always)]
    pub const fn set_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID Pullup"]
    #[must_use]
    #[inline(always)]
    pub const fn idpu(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "ID Pullup"]
    #[inline(always)]
    pub const fn set_idpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB ID"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "A VBus Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn avv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid"]
    #[inline(always)]
    pub const fn set_avv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "A Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn asv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid"]
    #[inline(always)]
    pub const fn set_asv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "B Session Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn bsv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid"]
    #[inline(always)]
    pub const fn set_bsv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "B Session End"]
    #[must_use]
    #[inline(always)]
    pub const fn bse(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End"]
    #[inline(always)]
    pub const fn set_bse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "1 Millisecond Timer Toggle"]
    #[must_use]
    #[inline(always)]
    pub const fn tog_1ms(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Toggle"]
    #[inline(always)]
    pub const fn set_tog_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Data Bus Pulsing Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Data Bus Pulsing Status"]
    #[inline(always)]
    pub const fn set_dps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "USB ID Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn idis(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Status"]
    #[inline(always)]
    pub const fn set_idis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "A VBus Valid Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn avvis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Status"]
    #[inline(always)]
    pub const fn set_avvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "A Session Valid Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn asvis(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Status"]
    #[inline(always)]
    pub const fn set_asvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "B Session Valid Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bsvis(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Status"]
    #[inline(always)]
    pub const fn set_bsvis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "B Session End Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn bseis(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Status"]
    #[inline(always)]
    pub const fn set_bseis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn status_1ms(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Status"]
    #[inline(always)]
    pub const fn set_status_1ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Data Pulse Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dpis(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Data Pulse Interrupt Status"]
    #[inline(always)]
    pub const fn set_dpis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "USB ID Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn idie(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "USB ID Interrupt Enable"]
    #[inline(always)]
    pub const fn set_idie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "A VBus Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn avvie(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "A VBus Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_avvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "A Session Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn asvie(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "A Session Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_asvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "B Session Valid Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bsvie(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "B Session Valid Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bsvie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "B Session End Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bseie(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "B Session End Interrupt Enable"]
    #[inline(always)]
    pub const fn set_bseie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "1 Millisecond Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en_1ms(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "1 Millisecond Timer Interrupt Enable"]
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
    #[doc = "Current Connect Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ccs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Current Connect Status"]
    #[inline(always)]
    pub const fn set_ccs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Connect Status Change"]
    #[must_use]
    #[inline(always)]
    pub const fn csc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Connect Status Change"]
    #[inline(always)]
    pub const fn set_csc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Enabled/Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enabled/Disabled"]
    #[inline(always)]
    pub const fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port Enable/Disable Change"]
    #[must_use]
    #[inline(always)]
    pub const fn pec(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port Enable/Disable Change"]
    #[inline(always)]
    pub const fn set_pec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Over-Current Active"]
    #[must_use]
    #[inline(always)]
    pub const fn oca(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Over-Current Active"]
    #[inline(always)]
    pub const fn set_oca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Over-current Change"]
    #[must_use]
    #[inline(always)]
    pub const fn occ(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Over-current Change"]
    #[inline(always)]
    pub const fn set_occ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Force Port Resume"]
    #[must_use]
    #[inline(always)]
    pub const fn fpr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Force Port Resume"]
    #[inline(always)]
    pub const fn set_fpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Suspend"]
    #[must_use]
    #[inline(always)]
    pub const fn susp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Suspend"]
    #[inline(always)]
    pub const fn set_susp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Port Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn pr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Port Reset"]
    #[inline(always)]
    pub const fn set_pr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "High-Speed Port"]
    #[must_use]
    #[inline(always)]
    pub const fn hsp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High-Speed Port"]
    #[inline(always)]
    pub const fn set_hsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Line Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ls(&self) -> super::vals::Ls {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Ls::from_bits(val as u8)
    }
    #[doc = "Line Status"]
    #[inline(always)]
    pub const fn set_ls(&mut self, val: super::vals::Ls) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Port Power"]
    #[must_use]
    #[inline(always)]
    pub const fn pp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Port Power"]
    #[inline(always)]
    pub const fn set_pp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Port Owner"]
    #[must_use]
    #[inline(always)]
    pub const fn po(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Port Owner"]
    #[inline(always)]
    pub const fn set_po(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Port Indicator Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pic(&self) -> super::vals::Pic {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pic::from_bits(val as u8)
    }
    #[doc = "Port Indicator Control"]
    #[inline(always)]
    pub const fn set_pic(&mut self, val: super::vals::Pic) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Port Test Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ptc(&self) -> super::vals::Ptc {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Ptc::from_bits(val as u8)
    }
    #[doc = "Port Test Control"]
    #[inline(always)]
    pub const fn set_ptc(&mut self, val: super::vals::Ptc) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)"]
    #[must_use]
    #[inline(always)]
    pub const fn wkcn(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Connect Enable (WKCNNT_E)"]
    #[inline(always)]
    pub const fn set_wkcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)"]
    #[must_use]
    #[inline(always)]
    pub const fn wkdc(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Disconnect Enable (WKDSCNNT_E)"]
    #[inline(always)]
    pub const fn set_wkdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Wake on Over-current Enable (WKOC_E)"]
    #[must_use]
    #[inline(always)]
    pub const fn wkoc(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Wake on Over-current Enable (WKOC_E)"]
    #[inline(always)]
    pub const fn set_wkoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD)"]
    #[must_use]
    #[inline(always)]
    pub const fn phcd(&self) -> super::vals::Phcd {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Phcd::from_bits(val as u8)
    }
    #[doc = "PHY Low Power Suspend - Clock Disable (PLPSCD)"]
    #[inline(always)]
    pub const fn set_phcd(&mut self, val: super::vals::Phcd) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Port Force Full Speed Connect"]
    #[must_use]
    #[inline(always)]
    pub const fn pfsc(&self) -> super::vals::Pfsc {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pfsc::from_bits(val as u8)
    }
    #[doc = "Port Force Full Speed Connect"]
    #[inline(always)]
    pub const fn set_pfsc(&mut self, val: super::vals::Pfsc) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Parallel Transceiver Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pts_2(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Transceiver Select"]
    #[inline(always)]
    pub const fn set_pts_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Port Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn pspd(&self) -> super::vals::Pspd {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Pspd::from_bits(val as u8)
    }
    #[doc = "Port Speed"]
    #[inline(always)]
    pub const fn set_pspd(&mut self, val: super::vals::Pspd) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Parallel Transceiver Width - Read/Write"]
    #[must_use]
    #[inline(always)]
    pub const fn ptw(&self) -> super::vals::Ptw {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Ptw::from_bits(val as u8)
    }
    #[doc = "Parallel Transceiver Width - Read/Write"]
    #[inline(always)]
    pub const fn set_ptw(&mut self, val: super::vals::Ptw) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Serial Transceiver Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sts(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Serial Transceiver Select"]
    #[inline(always)]
    pub const fn set_sts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Parallel Transceiver Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pts_1(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Parallel Transceiver Select"]
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
#[doc = "System Bus Config"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbuscfg(pub u32);
impl Sbuscfg {
    #[doc = "AHB master interface Burst configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbrst(&self) -> super::vals::Ahbbrst {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ahbbrst::from_bits(val as u8)
    }
    #[doc = "AHB master interface Burst configuration"]
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
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Scheduler Overhead"]
    #[inline(always)]
    pub const fn set_txschoh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
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
#[doc = "USB Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbcmd(pub u32);
impl Usbcmd {
    #[doc = "Run/Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn rs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Run/Stop"]
    #[inline(always)]
    pub const fn set_rs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controller Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Controller Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame List Size"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Frame List Size"]
    #[inline(always)]
    pub const fn set_fs_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Periodic Schedule Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pse(&self) -> super::vals::Pse {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pse::from_bits(val as u8)
    }
    #[doc = "Periodic Schedule Enable"]
    #[inline(always)]
    pub const fn set_pse(&mut self, val: super::vals::Pse) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Asynchronous Schedule Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ase(&self) -> super::vals::Ase {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ase::from_bits(val as u8)
    }
    #[doc = "Asynchronous Schedule Enable"]
    #[inline(always)]
    pub const fn set_ase(&mut self, val: super::vals::Ase) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt on Async Advance Doorbell"]
    #[must_use]
    #[inline(always)]
    pub const fn iaa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance Doorbell"]
    #[inline(always)]
    pub const fn set_iaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Count"]
    #[must_use]
    #[inline(always)]
    pub const fn asp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Asynchronous Schedule Park Mode Count"]
    #[inline(always)]
    pub const fn set_asp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Asynchronous Schedule Park Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aspe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Park Mode Enable"]
    #[inline(always)]
    pub const fn set_aspe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Setup TripWire \\[device mode only\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn sutw(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Setup TripWire \\[device mode only\\]"]
    #[inline(always)]
    pub const fn set_sutw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Add dTD TripWire\\[device mode only\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn atdtw(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Add dTD TripWire\\[device mode only\\]"]
    #[inline(always)]
    pub const fn set_atdtw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Frame List Size \\[host mode only\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_2(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Size \\[host mode only\\]"]
    #[inline(always)]
    pub const fn set_fs_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Interrupt Threshold Control"]
    #[must_use]
    #[inline(always)]
    pub const fn itc(&self) -> super::vals::Itc {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Itc::from_bits(val as u8)
    }
    #[doc = "Interrupt Threshold Control"]
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
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbintr(pub u32);
impl Usbintr {
    #[doc = "USB Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ue(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uee(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_uee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_pce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fre(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn see(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_see(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Async Advance Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn aae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Async Advance Interrupt Enable"]
    #[inline(always)]
    pub const fn set_aae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ure(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sre(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sre(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Sleep Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Interrupt Enable"]
    #[inline(always)]
    pub const fn set_sle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "NAK Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nake(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Enable"]
    #[inline(always)]
    pub const fn set_nake(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "USB Host Asynchronous Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uaie(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub const fn set_uaie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "USB Host Periodic Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn upie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "USB Host Periodic Interrupt Enable"]
    #[inline(always)]
    pub const fn set_upie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #0 Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer #1 Interrupt Enable"]
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
#[doc = "USB Device Mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbmode(pub u32);
impl Usbmode {
    #[doc = "Controller Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> super::vals::Cm {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Cm::from_bits(val as u8)
    }
    #[doc = "Controller Mode"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: super::vals::Cm) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Endian Select"]
    #[must_use]
    #[inline(always)]
    pub const fn es(&self) -> super::vals::Es {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Es::from_bits(val as u8)
    }
    #[doc = "Endian Select"]
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
#[doc = "USB Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbsts(pub u32);
impl Usbsts {
    #[doc = "USB Interrupt (USBINT)"]
    #[must_use]
    #[inline(always)]
    pub const fn ui(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USB Interrupt (USBINT)"]
    #[inline(always)]
    pub const fn set_ui(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB Error Interrupt (USBERRINT)"]
    #[must_use]
    #[inline(always)]
    pub const fn uei(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "USB Error Interrupt (USBERRINT)"]
    #[inline(always)]
    pub const fn set_uei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port Change Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn pci(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port Change Detect"]
    #[inline(always)]
    pub const fn set_pci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame List Rollover"]
    #[must_use]
    #[inline(always)]
    pub const fn fri(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame List Rollover"]
    #[inline(always)]
    pub const fn set_fri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "System Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sei(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "System Error"]
    #[inline(always)]
    pub const fn set_sei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt on Async Advance"]
    #[must_use]
    #[inline(always)]
    pub const fn aai(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on Async Advance"]
    #[inline(always)]
    pub const fn set_aai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "USB Reset Received"]
    #[must_use]
    #[inline(always)]
    pub const fn uri(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "USB Reset Received"]
    #[inline(always)]
    pub const fn set_uri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SOF Received"]
    #[must_use]
    #[inline(always)]
    pub const fn sri(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SOF Received"]
    #[inline(always)]
    pub const fn set_sri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DCSuspend"]
    #[must_use]
    #[inline(always)]
    pub const fn sli(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DCSuspend"]
    #[inline(always)]
    pub const fn set_sli(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ULPI Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ulpii(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "ULPI Interrupt"]
    #[inline(always)]
    pub const fn set_ulpii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "HCHaIted"]
    #[must_use]
    #[inline(always)]
    pub const fn hch(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "HCHaIted"]
    #[inline(always)]
    pub const fn set_hch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Reclamation"]
    #[must_use]
    #[inline(always)]
    pub const fn rcl(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Reclamation"]
    #[inline(always)]
    pub const fn set_rcl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Periodic Schedule Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Periodic Schedule Status"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Asynchronous Schedule Status"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronous Schedule Status"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NAK Interrupt Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn naki(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "NAK Interrupt Bit"]
    #[inline(always)]
    pub const fn set_naki(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0)"]
    #[must_use]
    #[inline(always)]
    pub const fn ti0(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 0 (GPTINT0)"]
    #[inline(always)]
    pub const fn set_ti0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1)"]
    #[must_use]
    #[inline(always)]
    pub const fn ti1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "General Purpose Timer Interrupt 1 (GPTINT1)"]
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
