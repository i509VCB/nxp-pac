#[doc = "Clock Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "SCK Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn sckdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Divider"]
    #[inline(always)]
    pub const fn set_sckdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Delay Between Transfers"]
    #[must_use]
    #[inline(always)]
    pub const fn dbt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Delay Between Transfers"]
    #[inline(always)]
    pub const fn set_dbt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS-to-SCK Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pcssck(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS-to-SCK Delay"]
    #[inline(always)]
    pub const fn set_pcssck(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SCK-to-PCS Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn sckpcs(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SCK-to-PCS Delay"]
    #[inline(always)]
    pub const fn set_sckpcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("sckdiv", &self.sckdiv())
            .field("dbt", &self.dbt())
            .field("pcssck", &self.pcssck())
            .field("sckpcs", &self.sckpcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr {{ sckdiv: {=u8:?}, dbt: {=u8:?}, pcssck: {=u8:?}, sckpcs: {=u8:?} }}",
            self.sckdiv(),
            self.dbt(),
            self.pcssck(),
            self.sckpcs()
        )
    }
}
#[doc = "Clock Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr1(pub u32);
impl Ccr1 {
    #[doc = "SCK Setup"]
    #[must_use]
    #[inline(always)]
    pub const fn sckset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Setup"]
    #[inline(always)]
    pub const fn set_sckset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SCK Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn sckhld(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Hold"]
    #[inline(always)]
    pub const fn set_sckhld(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS to PCS Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn pcspcs(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS to PCS Delay"]
    #[inline(always)]
    pub const fn set_pcspcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "SCK Inter-Frame Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn scksck(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SCK Inter-Frame Delay"]
    #[inline(always)]
    pub const fn set_scksck(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ccr1 {
    #[inline(always)]
    fn default() -> Ccr1 {
        Ccr1(0)
    }
}
impl core::fmt::Debug for Ccr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr1")
            .field("sckset", &self.sckset())
            .field("sckhld", &self.sckhld())
            .field("pcspcs", &self.pcspcs())
            .field("scksck", &self.scksck())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ccr1 {{ sckset: {=u8:?}, sckhld: {=u8:?}, pcspcs: {=u8:?}, scksck: {=u8:?} }}",
            self.sckset(),
            self.sckhld(),
            self.pcspcs(),
            self.scksck()
        )
    }
}
#[doc = "Configuration 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr0(pub u32);
impl Cfgr0 {
    #[doc = "Host Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hren(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Enable"]
    #[inline(always)]
    pub const fn set_hren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host Request Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn hrpol(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host Request Polarity"]
    #[inline(always)]
    pub const fn set_hrpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host Request Select"]
    #[must_use]
    #[inline(always)]
    pub const fn hrsel(&self) -> super::vals::Hrsel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Hrsel::from_bits(val as u8)
    }
    #[doc = "Host Request Select"]
    #[inline(always)]
    pub const fn set_hrsel(&mut self, val: super::vals::Hrsel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Host Request Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn hrdir(&self) -> super::vals::Hrdir {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Hrdir::from_bits(val as u8)
    }
    #[doc = "Host Request Direction"]
    #[inline(always)]
    pub const fn set_hrdir(&mut self, val: super::vals::Hrdir) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Circular FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cirfifo(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Circular FIFO Enable"]
    #[inline(always)]
    pub const fn set_cirfifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive Data Match Only"]
    #[must_use]
    #[inline(always)]
    pub const fn rdmo(&self) -> super::vals::Rdmo {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rdmo::from_bits(val as u8)
    }
    #[doc = "Receive Data Match Only"]
    #[inline(always)]
    pub const fn set_rdmo(&mut self, val: super::vals::Rdmo) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Cfgr0 {
    #[inline(always)]
    fn default() -> Cfgr0 {
        Cfgr0(0)
    }
}
impl core::fmt::Debug for Cfgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr0")
            .field("hren", &self.hren())
            .field("hrpol", &self.hrpol())
            .field("hrsel", &self.hrsel())
            .field("hrdir", &self.hrdir())
            .field("cirfifo", &self.cirfifo())
            .field("rdmo", &self.rdmo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr0 {{ hren: {=bool:?}, hrpol: {=bool:?}, hrsel: {:?}, hrdir: {:?}, cirfifo: {=bool:?}, rdmo: {:?} }}",
            self.hren(),
            self.hrpol(),
            self.hrsel(),
            self.hrdir(),
            self.cirfifo(),
            self.rdmo()
        )
    }
}
#[doc = "Configuration 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc = "Controller Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Controller Mode"]
    #[inline(always)]
    pub const fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Sample Point"]
    #[must_use]
    #[inline(always)]
    pub const fn sample(&self) -> super::vals::Sample {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sample::from_bits(val as u8)
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub const fn set_sample(&mut self, val: super::vals::Sample) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Automatic PCS"]
    #[must_use]
    #[inline(always)]
    pub const fn autopcs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Automatic PCS"]
    #[inline(always)]
    pub const fn set_autopcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "No Stall"]
    #[must_use]
    #[inline(always)]
    pub const fn nostall(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No Stall"]
    #[inline(always)]
    pub const fn set_nostall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Partial Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn partial(&self) -> super::vals::Partial {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Partial::from_bits(val as u8)
    }
    #[doc = "Partial Enable"]
    #[inline(always)]
    pub const fn set_partial(&mut self, val: super::vals::Partial) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Peripheral Chip Select Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pcspol(&self) -> super::vals::Pcspol {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Pcspol::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select Polarity"]
    #[inline(always)]
    pub const fn set_pcspol(&mut self, val: super::vals::Pcspol) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Match Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn matcfg(&self) -> super::vals::Matcfg {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Matcfg::from_bits(val as u8)
    }
    #[doc = "Match Configuration"]
    #[inline(always)]
    pub const fn set_matcfg(&mut self, val: super::vals::Matcfg) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::Pincfg {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Pincfg::from_bits(val as u8)
    }
    #[doc = "Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::Pincfg) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Output Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn outcfg(&self) -> super::vals::Outcfg {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Outcfg::from_bits(val as u8)
    }
    #[doc = "Output Configuration"]
    #[inline(always)]
    pub const fn set_outcfg(&mut self, val: super::vals::Outcfg) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Peripheral Chip Select Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pcscfg(&self) -> super::vals::Pcscfg {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Pcscfg::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select Configuration"]
    #[inline(always)]
    pub const fn set_pcscfg(&mut self, val: super::vals::Pcscfg) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Cfgr1 {
    #[inline(always)]
    fn default() -> Cfgr1 {
        Cfgr1(0)
    }
}
impl core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgr1")
            .field("master", &self.master())
            .field("sample", &self.sample())
            .field("autopcs", &self.autopcs())
            .field("nostall", &self.nostall())
            .field("partial", &self.partial())
            .field("pcspol", &self.pcspol())
            .field("matcfg", &self.matcfg())
            .field("pincfg", &self.pincfg())
            .field("outcfg", &self.outcfg())
            .field("pcscfg", &self.pcscfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfgr1 {{ master: {:?}, sample: {:?}, autopcs: {=bool:?}, nostall: {=bool:?}, partial: {:?}, pcspol: {:?}, matcfg: {:?}, pincfg: {:?}, outcfg: {:?}, pcscfg: {:?} }}",
            self.master(),
            self.sample(),
            self.autopcs(),
            self.nostall(),
            self.partial(),
            self.pcspol(),
            self.matcfg(),
            self.pincfg(),
            self.outcfg(),
            self.pcscfg()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "Module Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Enable"]
    #[inline(always)]
    pub const fn set_men(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Reset Transmit FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rtf(&self) -> super::vals::Rtf {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rtf::from_bits(val as u8)
    }
    #[doc = "Reset Transmit FIFO"]
    #[inline(always)]
    pub const fn set_rtf(&mut self, val: super::vals::Rtf) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Receive FIFO"]
    #[must_use]
    #[inline(always)]
    pub const fn rrf(&self) -> super::vals::Rrf {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rrf::from_bits(val as u8)
    }
    #[doc = "Reset Receive FIFO"]
    #[inline(always)]
    pub const fn set_rrf(&mut self, val: super::vals::Rrf) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("men", &self.men())
            .field("rst", &self.rst())
            .field("dbgen", &self.dbgen())
            .field("rtf", &self.rtf())
            .field("rrf", &self.rrf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ men: {=bool:?}, rst: {=bool:?}, dbgen: {=bool:?}, rtf: {:?}, rrf: {:?} }}",
            self.men(),
            self.rst(),
            self.dbgen(),
            self.rtf(),
            self.rrf()
        )
    }
}
#[doc = "DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Der(pub u32);
impl Der {
    #[doc = "Transmit Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdde(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data DMA Enable"]
    #[inline(always)]
    pub const fn set_tdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdde(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data DMA Enable"]
    #[inline(always)]
    pub const fn set_rdde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame Complete DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fcde(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete DMA Enable"]
    #[inline(always)]
    pub const fn set_fcde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Der {
    #[inline(always)]
    fn default() -> Der {
        Der(0)
    }
}
impl core::fmt::Debug for Der {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Der")
            .field("tdde", &self.tdde())
            .field("rdde", &self.rdde())
            .field("fcde", &self.fcde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Der {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Der {{ tdde: {=bool:?}, rdde: {=bool:?}, fcde: {=bool:?} }}",
            self.tdde(),
            self.rdde(),
            self.fcde()
        )
    }
}
#[doc = "Data Match 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmr0(pub u32);
impl Dmr0 {
    #[doc = "Match 0 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 0 Value"]
    #[inline(always)]
    pub const fn set_match0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmr0 {
    #[inline(always)]
    fn default() -> Dmr0 {
        Dmr0(0)
    }
}
impl core::fmt::Debug for Dmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmr0")
            .field("match0", &self.match0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmr0 {{ match0: {=u32:?} }}", self.match0())
    }
}
#[doc = "Data Match 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmr1(pub u32);
impl Dmr1 {
    #[doc = "Match 1 Value"]
    #[must_use]
    #[inline(always)]
    pub const fn match1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Match 1 Value"]
    #[inline(always)]
    pub const fn set_match1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dmr1 {
    #[inline(always)]
    fn default() -> Dmr1 {
        Dmr1(0)
    }
}
impl core::fmt::Debug for Dmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmr1")
            .field("match1", &self.match1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmr1 {{ match1: {=u32:?} }}", self.match1())
    }
}
#[doc = "FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcr(pub u32);
impl Fcr {
    #[doc = "Transmit FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn txwater(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit FIFO Watermark"]
    #[inline(always)]
    pub const fn set_txwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Receive FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn rxwater(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Receive FIFO Watermark"]
    #[inline(always)]
    pub const fn set_rxwater(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for Fcr {
    #[inline(always)]
    fn default() -> Fcr {
        Fcr(0)
    }
}
impl core::fmt::Debug for Fcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcr")
            .field("txwater", &self.txwater())
            .field("rxwater", &self.rxwater())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcr {{ txwater: {=u8:?}, rxwater: {=u8:?} }}",
            self.txwater(),
            self.rxwater()
        )
    }
}
#[doc = "FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr(pub u32);
impl Fsr {
    #[doc = "Transmit FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit FIFO Count"]
    #[inline(always)]
    pub const fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive FIFO Count"]
    #[must_use]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Receive FIFO Count"]
    #[inline(always)]
    pub const fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Fsr {
    #[inline(always)]
    fn default() -> Fsr {
        Fsr(0)
    }
}
impl core::fmt::Debug for Fsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fsr")
            .field("txcount", &self.txcount())
            .field("rxcount", &self.rxcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fsr {{ txcount: {=u8:?}, rxcount: {=u8:?} }}",
            self.txcount(),
            self.rxcount()
        )
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Transmit Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdie(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rdie(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rdie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Word Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wcie(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Word Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fcie(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcie(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reie(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_reie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Data Match Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmie(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("tdie", &self.tdie())
            .field("rdie", &self.rdie())
            .field("wcie", &self.wcie())
            .field("fcie", &self.fcie())
            .field("tcie", &self.tcie())
            .field("teie", &self.teie())
            .field("reie", &self.reie())
            .field("dmie", &self.dmie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ier {{ tdie: {=bool:?}, rdie: {=bool:?}, wcie: {=bool:?}, fcie: {=bool:?}, tcie: {=bool:?}, teie: {=bool:?}, reie: {=bool:?}, dmie: {=bool:?} }}",
            self.tdie(),
            self.rdie(),
            self.wcie(),
            self.fcie(),
            self.tcie(),
            self.teie(),
            self.reie(),
            self.dmie()
        )
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Transmit FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn txfifo(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Size"]
    #[inline(always)]
    pub const fn set_txfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive FIFO Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rxfifo(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Size"]
    #[inline(always)]
    pub const fn set_rxfifo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "PCS Number"]
    #[must_use]
    #[inline(always)]
    pub const fn pcsnum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "PCS Number"]
    #[inline(always)]
    pub const fn set_pcsnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
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
            .field("txfifo", &self.txfifo())
            .field("rxfifo", &self.rxfifo())
            .field("pcsnum", &self.pcsnum())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ txfifo: {=u8:?}, rxfifo: {=u8:?}, pcsnum: {=u8:?} }}",
            self.txfifo(),
            self.rxfifo(),
            self.pcsnum()
        )
    }
}
#[doc = "Receive Data Burst"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdbr(pub u32);
impl Rdbr {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdbr {
    #[inline(always)]
    fn default() -> Rdbr {
        Rdbr(0)
    }
}
impl core::fmt::Debug for Rdbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr(pub u32);
impl Rdr {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
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
        f.debug_struct("Rdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Data Read Only"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdror(pub u32);
impl Rdror {
    #[doc = "Receive Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rdror {
    #[inline(always)]
    fn default() -> Rdror {
        Rdror(0)
    }
}
impl core::fmt::Debug for Rdror {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdror").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdror {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rdror {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Receive Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc = "Start of Frame"]
    #[must_use]
    #[inline(always)]
    pub const fn sof(&self) -> super::vals::Sof {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sof::from_bits(val as u8)
    }
    #[doc = "Start of Frame"]
    #[inline(always)]
    pub const fn set_sof(&mut self, val: super::vals::Sof) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO Empty"]
    #[inline(always)]
    pub const fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        Rsr(0)
    }
}
impl core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsr")
            .field("sof", &self.sof())
            .field("rxempty", &self.rxempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rsr {{ sof: {:?}, rxempty: {=bool:?} }}",
            self.sof(),
            self.rxempty()
        )
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Transmit Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tdf(&self) -> super::vals::Tdf {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tdf::from_bits(val as u8)
    }
    #[doc = "Transmit Data Flag"]
    #[inline(always)]
    pub const fn set_tdf(&mut self, val: super::vals::Tdf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rdf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Data Flag"]
    #[inline(always)]
    pub const fn set_rdf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Word Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wcf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Word Complete Flag"]
    #[inline(always)]
    pub const fn set_wcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fcf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Complete Flag"]
    #[inline(always)]
    pub const fn set_fcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transfer Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Complete Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tef(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Error Flag"]
    #[inline(always)]
    pub const fn set_tef(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive Error Flag"]
    #[inline(always)]
    pub const fn set_ref_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Data Match Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dmf(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Data Match Flag"]
    #[inline(always)]
    pub const fn set_dmf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Module Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn mbf(&self) -> super::vals::Mbf {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mbf::from_bits(val as u8)
    }
    #[doc = "Module Busy Flag"]
    #[inline(always)]
    pub const fn set_mbf(&mut self, val: super::vals::Mbf) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("tdf", &self.tdf())
            .field("rdf", &self.rdf())
            .field("wcf", &self.wcf())
            .field("fcf", &self.fcf())
            .field("tcf", &self.tcf())
            .field("tef", &self.tef())
            .field("ref_", &self.ref_())
            .field("dmf", &self.dmf())
            .field("mbf", &self.mbf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sr {{ tdf: {:?}, rdf: {=bool:?}, wcf: {=bool:?}, fcf: {=bool:?}, tcf: {=bool:?}, tef: {=bool:?}, ref_: {=bool:?}, dmf: {=bool:?}, mbf: {:?} }}",
            self.tdf(),
            self.rdf(),
            self.wcf(),
            self.fcf(),
            self.tcf(),
            self.tef(),
            self.ref_(),
            self.dmf(),
            self.mbf()
        )
    }
}
#[doc = "Transmit Command Burst"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcbr(pub u32);
impl Tcbr {
    #[doc = "Command Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcbr {
    #[inline(always)]
    fn default() -> Tcbr {
        Tcbr(0)
    }
}
impl core::fmt::Debug for Tcbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Transmit Command"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Frame Size"]
    #[must_use]
    #[inline(always)]
    pub const fn framesz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Frame Size"]
    #[inline(always)]
    pub const fn set_framesz(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Transfer Width"]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> super::vals::Width {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Width::from_bits(val as u8)
    }
    #[doc = "Transfer Width"]
    #[inline(always)]
    pub const fn set_width(&mut self, val: super::vals::Width) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Transmit Data Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn txmsk(&self) -> super::vals::Txmsk {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Txmsk::from_bits(val as u8)
    }
    #[doc = "Transmit Data Mask"]
    #[inline(always)]
    pub const fn set_txmsk(&mut self, val: super::vals::Txmsk) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Receive Data Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rxmsk(&self) -> super::vals::Rxmsk {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Rxmsk::from_bits(val as u8)
    }
    #[doc = "Receive Data Mask"]
    #[inline(always)]
    pub const fn set_rxmsk(&mut self, val: super::vals::Rxmsk) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Continuing Command"]
    #[must_use]
    #[inline(always)]
    pub const fn contc(&self) -> super::vals::Contc {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Contc::from_bits(val as u8)
    }
    #[doc = "Continuing Command"]
    #[inline(always)]
    pub const fn set_contc(&mut self, val: super::vals::Contc) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Continuous Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Continuous Transfer"]
    #[inline(always)]
    pub const fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Byte Swap"]
    #[must_use]
    #[inline(always)]
    pub const fn bysw(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Byte Swap"]
    #[inline(always)]
    pub const fn set_bysw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LSB First"]
    #[must_use]
    #[inline(always)]
    pub const fn lsbf(&self) -> super::vals::Lsbf {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Lsbf::from_bits(val as u8)
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub const fn set_lsbf(&mut self, val: super::vals::Lsbf) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Peripheral Chip Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Pcs {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Pcs::from_bits(val as u8)
    }
    #[doc = "Peripheral Chip Select"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Pcs) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Prescaler Value"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler Value"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Clock Phase"]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> super::vals::Cpha {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cpha::from_bits(val as u8)
    }
    #[doc = "Clock Phase"]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Clock Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> super::vals::Cpol {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cpol::from_bits(val as u8)
    }
    #[doc = "Clock Polarity"]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr")
            .field("framesz", &self.framesz())
            .field("width", &self.width())
            .field("txmsk", &self.txmsk())
            .field("rxmsk", &self.rxmsk())
            .field("contc", &self.contc())
            .field("cont", &self.cont())
            .field("bysw", &self.bysw())
            .field("lsbf", &self.lsbf())
            .field("pcs", &self.pcs())
            .field("prescale", &self.prescale())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcr {{ framesz: {=u16:?}, width: {:?}, txmsk: {:?}, rxmsk: {:?}, contc: {:?}, cont: {=bool:?}, bysw: {=bool:?}, lsbf: {:?}, pcs: {:?}, prescale: {:?}, cpha: {:?}, cpol: {:?} }}",
            self.framesz(),
            self.width(),
            self.txmsk(),
            self.rxmsk(),
            self.contc(),
            self.cont(),
            self.bysw(),
            self.lsbf(),
            self.pcs(),
            self.prescale(),
            self.cpha(),
            self.cpol()
        )
    }
}
#[doc = "Transmit Data Burst"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdbr(pub u32);
impl Tdbr {
    #[doc = "Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tdbr {
    #[inline(always)]
    fn default() -> Tdbr {
        Tdbr(0)
    }
}
impl core::fmt::Debug for Tdbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tdbr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdbr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Transmit Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tdr(pub u32);
impl Tdr {
    #[doc = "Transmit Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
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
        f.debug_struct("Tdr").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tdr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Module Identification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Module Identification Number"]
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
