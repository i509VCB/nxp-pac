#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Datalines"]
    #[must_use]
    #[inline(always)]
    pub const fn dataline(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Datalines"]
    #[inline(always)]
    pub const fn set_dataline(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Size"]
    #[inline(always)]
    pub const fn set_fifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Frame Size"]
    #[must_use]
    #[inline(always)]
    pub const fn frame(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_frame(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(0)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("dataline", &self.dataline())
            .field("fifo", &self.fifo())
            .field("frame", &self.frame())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ dataline: {=u8:?}, fifo: {=u8:?}, frame: {=u8:?} }}",
            self.dataline(),
            self.fifo(),
            self.frame()
        )
    }
}
#[doc = "Receive Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr1(pub u32);
impl Rcr1 {
    #[doc = "Receive FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn rfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn set_rfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Rcr1 {
    #[inline(always)]
    fn default() -> Rcr1 {
        Rcr1(0)
    }
}
impl core::fmt::Debug for Rcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr1").field("rfw", &self.rfw()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rcr1 {{ rfw: {=u8:?} }}", self.rfw())
    }
}
#[doc = "Receive Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr2(pub u32);
impl Rcr2 {
    #[doc = "Bit Clock Divide"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn bcd(&self) -> super::vals::Rcr2Bcd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Rcr2Bcd::from_bits(val as u8)
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: super::vals::Rcr2Bcd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn bcp(&self) -> super::vals::Rcr2Bcp {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Rcr2Bcp::from_bits(val as u8)
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: super::vals::Rcr2Bcp) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Rcr2Msel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Rcr2Msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Rcr2Msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input"]
    #[must_use]
    #[inline(always)]
    pub const fn bci(&self) -> super::vals::Rcr2Bci {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Rcr2Bci::from_bits(val as u8)
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: super::vals::Rcr2Bci) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap"]
    #[must_use]
    #[inline(always)]
    pub const fn bcs(&self) -> super::vals::Rcr2Bcs {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Rcr2Bcs::from_bits(val as u8)
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: super::vals::Rcr2Bcs) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> super::vals::Rcr2Sync {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Rcr2Sync::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: super::vals::Rcr2Sync) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Rcr2 {
    #[inline(always)]
    fn default() -> Rcr2 {
        Rcr2(0)
    }
}
impl core::fmt::Debug for Rcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr2")
            .field("div", &self.div())
            .field("bcd", &self.bcd())
            .field("bcp", &self.bcp())
            .field("msel", &self.msel())
            .field("bci", &self.bci())
            .field("bcs", &self.bcs())
            .field("sync", &self.sync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr2 {{ div: {=u8:?}, bcd: {:?}, bcp: {:?}, msel: {:?}, bci: {:?}, bcs: {:?}, sync: {:?} }}",
            self.div(),
            self.bcd(),
            self.bcp(),
            self.msel(),
            self.bci(),
            self.bcs(),
            self.sync()
        )
    }
}
#[doc = "Receive Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr3(pub u32);
impl Rcr3 {
    #[doc = "Word Flag Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn wdfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Receive Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive Channel Enable"]
    #[inline(always)]
    pub const fn set_rce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Channel FIFO Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Rcr3 {
    #[inline(always)]
    fn default() -> Rcr3 {
        Rcr3(0)
    }
}
impl core::fmt::Debug for Rcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr3")
            .field("wdfl", &self.wdfl())
            .field("rce", &self.rce())
            .field("cfr", &self.cfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr3 {{ wdfl: {=u8:?}, rce: {=u8:?}, cfr: {=u8:?} }}",
            self.wdfl(),
            self.rce(),
            self.cfr()
        )
    }
}
#[doc = "Receive Configuration 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr4(pub u32);
impl Rcr4 {
    #[doc = "Frame Sync Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn fsd(&self) -> super::vals::Rcr4Fsd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Rcr4Fsd::from_bits(val as u8)
    }
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: super::vals::Rcr4Fsd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> super::vals::Rcr4Fsp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rcr4Fsp::from_bits(val as u8)
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: super::vals::Rcr4Fsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "On Demand Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ondem(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early"]
    #[must_use]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First"]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Sync Width"]
    #[must_use]
    #[inline(always)]
    pub const fn sywd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame Size"]
    #[must_use]
    #[inline(always)]
    pub const fn frsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fpack(&self) -> super::vals::Rcr4Fpack {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Rcr4Fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: super::vals::Rcr4Fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fcomb(&self) -> super::vals::Rcr4Fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Rcr4Fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: super::vals::Rcr4Fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error"]
    #[must_use]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Rcr4 {
    #[inline(always)]
    fn default() -> Rcr4 {
        Rcr4(0)
    }
}
impl core::fmt::Debug for Rcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr4")
            .field("fsd", &self.fsd())
            .field("fsp", &self.fsp())
            .field("ondem", &self.ondem())
            .field("fse", &self.fse())
            .field("mf", &self.mf())
            .field("sywd", &self.sywd())
            .field("frsz", &self.frsz())
            .field("fpack", &self.fpack())
            .field("fcomb", &self.fcomb())
            .field("fcont", &self.fcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr4 {{ fsd: {:?}, fsp: {:?}, ondem: {=bool:?}, fse: {=bool:?}, mf: {=bool:?}, sywd: {=u8:?}, frsz: {=u8:?}, fpack: {:?}, fcomb: {:?}, fcont: {=bool:?} }}",
            self.fsd(),
            self.fsp(),
            self.ondem(),
            self.fse(),
            self.mf(),
            self.sywd(),
            self.frsz(),
            self.fpack(),
            self.fcomb(),
            self.fcont()
        )
    }
}
#[doc = "Receive Configuration 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr5(pub u32);
impl Rcr5 {
    #[doc = "First Bit Shifted"]
    #[must_use]
    #[inline(always)]
    pub const fn fbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width"]
    #[must_use]
    #[inline(always)]
    pub const fn w0w(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width"]
    #[must_use]
    #[inline(always)]
    pub const fn wnw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Rcr5 {
    #[inline(always)]
    fn default() -> Rcr5 {
        Rcr5(0)
    }
}
impl core::fmt::Debug for Rcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr5")
            .field("fbt", &self.fbt())
            .field("w0w", &self.w0w())
            .field("wnw", &self.wnw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcr5 {{ fbt: {=u8:?}, w0w: {=u8:?}, wnw: {=u8:?} }}",
            self.fbt(),
            self.w0w(),
            self.wnw()
        )
    }
}
#[doc = "Receive Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcsr(pub u32);
impl Rcsr {
    #[doc = "FIFO Request DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn frf(&self) -> super::vals::RcsrFrf {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::RcsrFrf::from_bits(val as u8)
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: super::vals::RcsrFrf) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> super::vals::RcsrSr {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RcsrSr::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: super::vals::RcsrSr) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn fr(&self) -> super::vals::RcsrFr {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::RcsrFr::from_bits(val as u8)
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: super::vals::RcsrFr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Receiver Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn re(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Receiver Enable"]
    #[inline(always)]
    pub const fn set_re(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rcsr {
    #[inline(always)]
    fn default() -> Rcsr {
        Rcsr(0)
    }
}
impl core::fmt::Debug for Rcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcsr")
            .field("frde", &self.frde())
            .field("fwde", &self.fwde())
            .field("frie", &self.frie())
            .field("fwie", &self.fwie())
            .field("feie", &self.feie())
            .field("seie", &self.seie())
            .field("wsie", &self.wsie())
            .field("frf", &self.frf())
            .field("fwf", &self.fwf())
            .field("fef", &self.fef())
            .field("sef", &self.sef())
            .field("wsf", &self.wsf())
            .field("sr", &self.sr())
            .field("fr", &self.fr())
            .field("bce", &self.bce())
            .field("dbge", &self.dbge())
            .field("stope", &self.stope())
            .field("re", &self.re())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rcsr {{ frde: {=bool:?}, fwde: {=bool:?}, frie: {=bool:?}, fwie: {=bool:?}, feie: {=bool:?}, seie: {=bool:?}, wsie: {=bool:?}, frf: {:?}, fwf: {=bool:?}, fef: {=bool:?}, sef: {=bool:?}, wsf: {=bool:?}, sr: {:?}, fr: {:?}, bce: {=bool:?}, dbge: {=bool:?}, stope: {=bool:?}, re: {=bool:?} }}",
            self.frde(),
            self.fwde(),
            self.frie(),
            self.fwie(),
            self.feie(),
            self.seie(),
            self.wsie(),
            self.frf(),
            self.fwf(),
            self.fef(),
            self.sef(),
            self.wsf(),
            self.sr(),
            self.fr(),
            self.bce(),
            self.dbge(),
            self.stope(),
            self.re()
        )
    }
}
#[doc = "Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Receive Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn rdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data Register"]
    #[inline(always)]
    pub const fn set_rdr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        Rdr(0)
    }
}
impl core::fmt::Debug for Rdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdr").field("rdr", &self.rdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ rdr: {=u32:?} }}", self.rdr())
    }
}
#[doc = "Receive FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfr(pub u32);
impl Rfr {
    #[doc = "Read FIFO Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Receive Channel Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn rcp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Channel Pointer"]
    #[inline(always)]
    pub const fn set_rcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write FIFO Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Rfr {
    #[inline(always)]
    fn default() -> Rfr {
        Rfr(0)
    }
}
impl core::fmt::Debug for Rfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfr")
            .field("rfp", &self.rfp())
            .field("rcp", &self.rcp())
            .field("wfp", &self.wfp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rfr {{ rfp: {=u8:?}, rcp: {=bool:?}, wfp: {=u8:?} }}",
            self.rfp(),
            self.rcp(),
            self.wfp()
        )
    }
}
#[doc = "Receive Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmr(pub u32);
impl Rmr {
    #[doc = "Receive Word Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rwm(&self) -> super::vals::Rwm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Rwm::from_bits(val as u32)
    }
    #[doc = "Receive Word Mask"]
    #[inline(always)]
    pub const fn set_rwm(&mut self, val: super::vals::Rwm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rmr {
    #[inline(always)]
    fn default() -> Rmr {
        Rmr(0)
    }
}
impl core::fmt::Debug for Rmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rmr").field("rwm", &self.rwm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rmr {{ rwm: {:?} }}", self.rwm())
    }
}
#[doc = "Transmit Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr1(pub u32);
impl Tcr1 {
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn tfw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_tfw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Tcr1 {
    #[inline(always)]
    fn default() -> Tcr1 {
        Tcr1(0)
    }
}
impl core::fmt::Debug for Tcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr1").field("tfw", &self.tfw()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcr1 {{ tfw: {=u8:?} }}", self.tfw())
    }
}
#[doc = "Transmit Configuration 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr2(pub u32);
impl Tcr2 {
    #[doc = "Bit Clock Divide"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Bit Clock Divide"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Bit Clock Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn bcd(&self) -> super::vals::Tcr2Bcd {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Tcr2Bcd::from_bits(val as u8)
    }
    #[doc = "Bit Clock Direction"]
    #[inline(always)]
    pub const fn set_bcd(&mut self, val: super::vals::Tcr2Bcd) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn bcp(&self) -> super::vals::Tcr2Bcp {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Tcr2Bcp::from_bits(val as u8)
    }
    #[doc = "Bit Clock Polarity"]
    #[inline(always)]
    pub const fn set_bcp(&mut self, val: super::vals::Tcr2Bcp) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "MCLK Select"]
    #[must_use]
    #[inline(always)]
    pub const fn msel(&self) -> super::vals::Tcr2Msel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Tcr2Msel::from_bits(val as u8)
    }
    #[doc = "MCLK Select"]
    #[inline(always)]
    pub const fn set_msel(&mut self, val: super::vals::Tcr2Msel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Bit Clock Input"]
    #[must_use]
    #[inline(always)]
    pub const fn bci(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Input"]
    #[inline(always)]
    pub const fn set_bci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Bit Clock Swap"]
    #[must_use]
    #[inline(always)]
    pub const fn bcs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Swap"]
    #[inline(always)]
    pub const fn set_bcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Synchronous Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sync(&self) -> super::vals::Tcr2Sync {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Tcr2Sync::from_bits(val as u8)
    }
    #[doc = "Synchronous Mode"]
    #[inline(always)]
    pub const fn set_sync(&mut self, val: super::vals::Tcr2Sync) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Tcr2 {
    #[inline(always)]
    fn default() -> Tcr2 {
        Tcr2(0)
    }
}
impl core::fmt::Debug for Tcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr2")
            .field("div", &self.div())
            .field("bcd", &self.bcd())
            .field("bcp", &self.bcp())
            .field("msel", &self.msel())
            .field("bci", &self.bci())
            .field("bcs", &self.bcs())
            .field("sync", &self.sync())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr2 {{ div: {=u8:?}, bcd: {:?}, bcp: {:?}, msel: {:?}, bci: {=bool:?}, bcs: {=bool:?}, sync: {:?} }}",
            self.div(),
            self.bcd(),
            self.bcp(),
            self.msel(),
            self.bci(),
            self.bcs(),
            self.sync()
        )
    }
}
#[doc = "Transmit Configuration 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr3(pub u32);
impl Tcr3 {
    #[doc = "Word Flag Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn wdfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Word Flag Configuration"]
    #[inline(always)]
    pub const fn set_wdfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Transmit Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tce(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit Channel Enable"]
    #[inline(always)]
    pub const fn set_tce(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Channel FIFO Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cfr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel FIFO Reset"]
    #[inline(always)]
    pub const fn set_cfr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tcr3 {
    #[inline(always)]
    fn default() -> Tcr3 {
        Tcr3(0)
    }
}
impl core::fmt::Debug for Tcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr3")
            .field("wdfl", &self.wdfl())
            .field("tce", &self.tce())
            .field("cfr", &self.cfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr3 {{ wdfl: {=u8:?}, tce: {=u8:?}, cfr: {=u8:?} }}",
            self.wdfl(),
            self.tce(),
            self.cfr()
        )
    }
}
#[doc = "Transmit Configuration 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr4(pub u32);
impl Tcr4 {
    #[doc = "Frame Sync Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn fsd(&self) -> super::vals::Tcr4Fsd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tcr4Fsd::from_bits(val as u8)
    }
    #[doc = "Frame Sync Direction"]
    #[inline(always)]
    pub const fn set_fsd(&mut self, val: super::vals::Tcr4Fsd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame Sync Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> super::vals::Tcr4Fsp {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcr4Fsp::from_bits(val as u8)
    }
    #[doc = "Frame Sync Polarity"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: super::vals::Tcr4Fsp) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "On Demand Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ondem(&self) -> super::vals::Tcr4Ondem {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tcr4Ondem::from_bits(val as u8)
    }
    #[doc = "On Demand Mode"]
    #[inline(always)]
    pub const fn set_ondem(&mut self, val: super::vals::Tcr4Ondem) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame Sync Early"]
    #[must_use]
    #[inline(always)]
    pub const fn fse(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Sync Early"]
    #[inline(always)]
    pub const fn set_fse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "MSB First"]
    #[must_use]
    #[inline(always)]
    pub const fn mf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MSB First"]
    #[inline(always)]
    pub const fn set_mf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn chmod(&self) -> super::vals::Chmod {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Chmod::from_bits(val as u8)
    }
    #[doc = "Channel Mode"]
    #[inline(always)]
    pub const fn set_chmod(&mut self, val: super::vals::Chmod) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Sync Width"]
    #[must_use]
    #[inline(always)]
    pub const fn sywd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Sync Width"]
    #[inline(always)]
    pub const fn set_sywd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Frame size"]
    #[must_use]
    #[inline(always)]
    pub const fn frsz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Frame size"]
    #[inline(always)]
    pub const fn set_frsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "FIFO Packing Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fpack(&self) -> super::vals::Tcr4Fpack {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Tcr4Fpack::from_bits(val as u8)
    }
    #[doc = "FIFO Packing Mode"]
    #[inline(always)]
    pub const fn set_fpack(&mut self, val: super::vals::Tcr4Fpack) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "FIFO Combine Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn fcomb(&self) -> super::vals::Tcr4Fcomb {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Tcr4Fcomb::from_bits(val as u8)
    }
    #[doc = "FIFO Combine Mode"]
    #[inline(always)]
    pub const fn set_fcomb(&mut self, val: super::vals::Tcr4Fcomb) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO Continue on Error"]
    #[must_use]
    #[inline(always)]
    pub const fn fcont(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Continue on Error"]
    #[inline(always)]
    pub const fn set_fcont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Tcr4 {
    #[inline(always)]
    fn default() -> Tcr4 {
        Tcr4(0)
    }
}
impl core::fmt::Debug for Tcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr4")
            .field("fsd", &self.fsd())
            .field("fsp", &self.fsp())
            .field("ondem", &self.ondem())
            .field("fse", &self.fse())
            .field("mf", &self.mf())
            .field("chmod", &self.chmod())
            .field("sywd", &self.sywd())
            .field("frsz", &self.frsz())
            .field("fpack", &self.fpack())
            .field("fcomb", &self.fcomb())
            .field("fcont", &self.fcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr4 {{ fsd: {:?}, fsp: {:?}, ondem: {:?}, fse: {=bool:?}, mf: {=bool:?}, chmod: {:?}, sywd: {=u8:?}, frsz: {=u8:?}, fpack: {:?}, fcomb: {:?}, fcont: {=bool:?} }}",
            self.fsd(),
            self.fsp(),
            self.ondem(),
            self.fse(),
            self.mf(),
            self.chmod(),
            self.sywd(),
            self.frsz(),
            self.fpack(),
            self.fcomb(),
            self.fcont()
        )
    }
}
#[doc = "Transmit Configuration 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr5(pub u32);
impl Tcr5 {
    #[doc = "First Bit Shifted"]
    #[must_use]
    #[inline(always)]
    pub const fn fbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "First Bit Shifted"]
    #[inline(always)]
    pub const fn set_fbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Word 0 Width"]
    #[must_use]
    #[inline(always)]
    pub const fn w0w(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Word 0 Width"]
    #[inline(always)]
    pub const fn set_w0w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Word N Width"]
    #[must_use]
    #[inline(always)]
    pub const fn wnw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Word N Width"]
    #[inline(always)]
    pub const fn set_wnw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for Tcr5 {
    #[inline(always)]
    fn default() -> Tcr5 {
        Tcr5(0)
    }
}
impl core::fmt::Debug for Tcr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr5")
            .field("fbt", &self.fbt())
            .field("w0w", &self.w0w())
            .field("wnw", &self.wnw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr5 {{ fbt: {=u8:?}, w0w: {=u8:?}, wnw: {=u8:?} }}",
            self.fbt(),
            self.w0w(),
            self.wnw()
        )
    }
}
#[doc = "Transmit Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcsr(pub u32);
impl Tcsr {
    #[doc = "FIFO Request DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request DMA Enable"]
    #[inline(always)]
    pub const fn set_frde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning DMA Enable"]
    #[inline(always)]
    pub const fn set_fwde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn frie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub const fn set_frie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fwie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fwie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn feie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_feie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Word Start Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wsie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wsie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "FIFO Request Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn frf(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Request Flag"]
    #[inline(always)]
    pub const fn set_frf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Warning Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fwf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Warning Flag"]
    #[inline(always)]
    pub const fn set_fwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fef(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Error Flag"]
    #[inline(always)]
    pub const fn set_fef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Sync Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Sync Error Flag"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Word Start Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wsf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Word Start Flag"]
    #[inline(always)]
    pub const fn set_wsf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "FIFO Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn fr(&self) -> super::vals::TcsrFr {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::TcsrFr::from_bits(val as u8)
    }
    #[doc = "FIFO Reset"]
    #[inline(always)]
    pub const fn set_fr(&mut self, val: super::vals::TcsrFr) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Bit Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bce(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Clock Enable"]
    #[inline(always)]
    pub const fn set_bce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stope(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stope(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Transmitter Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn te(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter Enable"]
    #[inline(always)]
    pub const fn set_te(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcsr {
    #[inline(always)]
    fn default() -> Tcsr {
        Tcsr(0)
    }
}
impl core::fmt::Debug for Tcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcsr")
            .field("frde", &self.frde())
            .field("fwde", &self.fwde())
            .field("frie", &self.frie())
            .field("fwie", &self.fwie())
            .field("feie", &self.feie())
            .field("seie", &self.seie())
            .field("wsie", &self.wsie())
            .field("frf", &self.frf())
            .field("fwf", &self.fwf())
            .field("fef", &self.fef())
            .field("sef", &self.sef())
            .field("wsf", &self.wsf())
            .field("sr", &self.sr())
            .field("fr", &self.fr())
            .field("bce", &self.bce())
            .field("dbge", &self.dbge())
            .field("stope", &self.stope())
            .field("te", &self.te())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcsr {{ frde: {=bool:?}, fwde: {=bool:?}, frie: {=bool:?}, fwie: {=bool:?}, feie: {=bool:?}, seie: {=bool:?}, wsie: {=bool:?}, frf: {=bool:?}, fwf: {=bool:?}, fef: {=bool:?}, sef: {=bool:?}, wsf: {=bool:?}, sr: {=bool:?}, fr: {:?}, bce: {=bool:?}, dbge: {=bool:?}, stope: {=bool:?}, te: {=bool:?} }}",
            self.frde(),
            self.fwde(),
            self.frie(),
            self.fwie(),
            self.feie(),
            self.seie(),
            self.wsie(),
            self.frf(),
            self.fwf(),
            self.fef(),
            self.sef(),
            self.wsf(),
            self.sr(),
            self.fr(),
            self.bce(),
            self.dbge(),
            self.stope(),
            self.te()
        )
    }
}
#[doc = "Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit Data Register"]
    #[must_use]
    #[inline(always)]
    pub const fn tdr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data Register"]
    #[inline(always)]
    pub const fn set_tdr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdr {
    #[inline(always)]
    fn default() -> Tdr {
        Tdr(0)
    }
}
impl core::fmt::Debug for Tdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdr").field("tdr", &self.tdr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ tdr: {=u32:?} }}", self.tdr())
    }
}
#[doc = "Transmit FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfr(pub u32);
impl Tfr {
    #[doc = "Read FIFO Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Read FIFO Pointer"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Write FIFO Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn wfp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Write FIFO Pointer"]
    #[inline(always)]
    pub const fn set_wfp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Write Channel Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn wcp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write Channel Pointer"]
    #[inline(always)]
    pub const fn set_wcp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tfr {
    #[inline(always)]
    fn default() -> Tfr {
        Tfr(0)
    }
}
impl core::fmt::Debug for Tfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfr")
            .field("rfp", &self.rfp())
            .field("wfp", &self.wfp())
            .field("wcp", &self.wcp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tfr {{ rfp: {=u8:?}, wfp: {=u8:?}, wcp: {=bool:?} }}",
            self.rfp(),
            self.wfp(),
            self.wcp()
        )
    }
}
#[doc = "Transmit Mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmr(pub u32);
impl Tmr {
    #[doc = "Transmit Word Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn twm(&self) -> super::vals::Twm {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Twm::from_bits(val as u32)
    }
    #[doc = "Transmit Word Mask"]
    #[inline(always)]
    pub const fn set_twm(&mut self, val: super::vals::Twm) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tmr {
    #[inline(always)]
    fn default() -> Tmr {
        Tmr(0)
    }
}
impl core::fmt::Debug for Tmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tmr").field("twm", &self.twm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tmr {{ twm: {:?} }}", self.twm())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(0)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Verid {{ feature: {:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
