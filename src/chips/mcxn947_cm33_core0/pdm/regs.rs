#[doc = "MICFIL Control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Channel 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Enable"]
    #[inline(always)]
    pub const fn set_ch0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Enable"]
    #[inline(always)]
    pub const fn set_ch1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Enable"]
    #[inline(always)]
    pub const fn set_ch2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Enable"]
    #[inline(always)]
    pub const fn set_ch3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Frame Synchronization Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fsyncen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Synchronization Enable"]
    #[inline(always)]
    pub const fn set_fsyncen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Decimation Filter Enable in Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn decfils(&self) -> super::vals::Decfils {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Decfils::from_bits(val as u8)
    }
    #[doc = "Decimation Filter Enable in Stop"]
    #[inline(always)]
    pub const fn set_decfils(&mut self, val: super::vals::Decfils) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Error Interruption Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn erren(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interruption Enable"]
    #[inline(always)]
    pub const fn set_erren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA Interrupt Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn disel(&self) -> super::vals::Disel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Disel::from_bits(val as u8)
    }
    #[doc = "DMA Interrupt Selection"]
    #[inline(always)]
    pub const fn set_disel(&mut self, val: super::vals::Disel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Module Enable in Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Module Enable in Debug"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sres(&self) -> super::vals::Sres {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Sres::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sres(&mut self, val: super::vals::Sres) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Debug Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg(&self) -> super::vals::Dbg {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dbg::from_bits(val as u8)
    }
    #[doc = "Debug Mode"]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: super::vals::Dbg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "MICFIL Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pdmien(&self) -> super::vals::Pdmien {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Pdmien::from_bits(val as u8)
    }
    #[doc = "MICFIL Enable"]
    #[inline(always)]
    pub const fn set_pdmien(&mut self, val: super::vals::Pdmien) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Module Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> super::vals::Mdis {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable"]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: super::vals::Mdis) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("ch0en", &self.ch0en())
            .field("ch1en", &self.ch1en())
            .field("ch2en", &self.ch2en())
            .field("ch3en", &self.ch3en())
            .field("fsyncen", &self.fsyncen())
            .field("decfils", &self.decfils())
            .field("erren", &self.erren())
            .field("disel", &self.disel())
            .field("dbge", &self.dbge())
            .field("sres", &self.sres())
            .field("dbg", &self.dbg())
            .field("pdmien", &self.pdmien())
            .field("dozen", &self.dozen())
            .field("mdis", &self.mdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl1 {{ ch0en: {=bool:?}, ch1en: {=bool:?}, ch2en: {=bool:?}, ch3en: {=bool:?}, fsyncen: {=bool:?}, decfils: {:?}, erren: {=bool:?}, disel: {:?}, dbge: {=bool:?}, sres: {:?}, dbg: {:?}, pdmien: {:?}, dozen: {=bool:?}, mdis: {:?} }}",
            self.ch0en(),
            self.ch1en(),
            self.ch2en(),
            self.ch3en(),
            self.fsyncen(),
            self.decfils(),
            self.erren(),
            self.disel(),
            self.dbge(),
            self.sres(),
            self.dbg(),
            self.pdmien(),
            self.dozen(),
            self.mdis()
        )
    }
}
#[doc = "MICFIL Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::Clkdiv {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Clkdiv::from_bits(val as u8)
    }
    #[doc = "Clock Divider"]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Divider Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn clkdivdis(&self) -> super::vals::Clkdivdis {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Clkdivdis::from_bits(val as u8)
    }
    #[doc = "Clock Divider Disable"]
    #[inline(always)]
    pub const fn set_clkdivdis(&mut self, val: super::vals::Clkdivdis) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CIC Decimation Rate"]
    #[must_use]
    #[inline(always)]
    pub const fn cicosr(&self) -> super::vals::Cicosr {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cicosr::from_bits(val as u8)
    }
    #[doc = "CIC Decimation Rate"]
    #[inline(always)]
    pub const fn set_cicosr(&mut self, val: super::vals::Cicosr) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Quality Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn qsel(&self) -> super::vals::Qsel {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::Qsel::from_bits(val as u8)
    }
    #[doc = "Quality Mode"]
    #[inline(always)]
    pub const fn set_qsel(&mut self, val: super::vals::Qsel) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("clkdiv", &self.clkdiv())
            .field("clkdivdis", &self.clkdivdis())
            .field("cicosr", &self.cicosr())
            .field("qsel", &self.qsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctrl2 {{ clkdiv: {:?}, clkdivdis: {:?}, cicosr: {:?}, qsel: {:?} }}",
            self.clkdiv(),
            self.clkdivdis(),
            self.cicosr(),
            self.qsel()
        )
    }
}
#[doc = "MICFIL Output Result"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datach(pub u32);
impl Datach {
    #[doc = "Channel n Data"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel n Data"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Datach {
    #[inline(always)]
    fn default() -> Datach {
        Datach(0)
    }
}
impl core::fmt::Debug for Datach {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Datach")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Datach {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Datach {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "MICFIL DC Remover Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCtrl(pub u32);
impl DcCtrl {
    #[doc = "Channel 0 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig0(&self) -> super::vals::DcCtrlDcconfig0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DcCtrlDcconfig0::from_bits(val as u8)
    }
    #[doc = "Channel 0 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig0(&mut self, val: super::vals::DcCtrlDcconfig0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig1(&self) -> super::vals::DcCtrlDcconfig1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DcCtrlDcconfig1::from_bits(val as u8)
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig1(&mut self, val: super::vals::DcCtrlDcconfig1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig2(&self) -> super::vals::DcCtrlDcconfig2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DcCtrlDcconfig2::from_bits(val as u8)
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig2(&mut self, val: super::vals::DcCtrlDcconfig2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig3(&self) -> super::vals::DcCtrlDcconfig3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DcCtrlDcconfig3::from_bits(val as u8)
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig3(&mut self, val: super::vals::DcCtrlDcconfig3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DcCtrl {
    #[inline(always)]
    fn default() -> DcCtrl {
        DcCtrl(0)
    }
}
impl core::fmt::Debug for DcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcCtrl")
            .field("dcconfig0", &self.dcconfig0())
            .field("dcconfig1", &self.dcconfig1())
            .field("dcconfig2", &self.dcconfig2())
            .field("dcconfig3", &self.dcconfig3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcCtrl {{ dcconfig0: {:?}, dcconfig1: {:?}, dcconfig2: {:?}, dcconfig3: {:?} }}",
            self.dcconfig0(),
            self.dcconfig1(),
            self.dcconfig2(),
            self.dcconfig3()
        )
    }
}
#[doc = "MICFIL Output DC Remover Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcOutCtrl(pub u32);
impl DcOutCtrl {
    #[doc = "Channel 0 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig0(&self) -> super::vals::DcOutCtrlDcconfig0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DcOutCtrlDcconfig0::from_bits(val as u8)
    }
    #[doc = "Channel 0 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig0(&mut self, val: super::vals::DcOutCtrlDcconfig0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig1(&self) -> super::vals::DcOutCtrlDcconfig1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DcOutCtrlDcconfig1::from_bits(val as u8)
    }
    #[doc = "Channel 1 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig1(&mut self, val: super::vals::DcOutCtrlDcconfig1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig2(&self) -> super::vals::DcOutCtrlDcconfig2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::DcOutCtrlDcconfig2::from_bits(val as u8)
    }
    #[doc = "Channel 2 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig2(&mut self, val: super::vals::DcOutCtrlDcconfig2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig3(&self) -> super::vals::DcOutCtrlDcconfig3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DcOutCtrlDcconfig3::from_bits(val as u8)
    }
    #[doc = "Channel 3 DC Remover Configuration"]
    #[inline(always)]
    pub const fn set_dcconfig3(&mut self, val: super::vals::DcOutCtrlDcconfig3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for DcOutCtrl {
    #[inline(always)]
    fn default() -> DcOutCtrl {
        DcOutCtrl(0)
    }
}
impl core::fmt::Debug for DcOutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DcOutCtrl")
            .field("dcconfig0", &self.dcconfig0())
            .field("dcconfig1", &self.dcconfig1())
            .field("dcconfig2", &self.dcconfig2())
            .field("dcconfig3", &self.dcconfig3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcOutCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcOutCtrl {{ dcconfig0: {:?}, dcconfig1: {:?}, dcconfig2: {:?}, dcconfig3: {:?} }}",
            self.dcconfig0(),
            self.dcconfig1(),
            self.dcconfig2(),
            self.dcconfig3()
        )
    }
}
#[doc = "MICFIL FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoCtrl(pub u32);
impl FifoCtrl {
    #[doc = "FIFO Watermark Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fifowmk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Watermark Control"]
    #[inline(always)]
    pub const fn set_fifowmk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for FifoCtrl {
    #[inline(always)]
    fn default() -> FifoCtrl {
        FifoCtrl(0)
    }
}
impl core::fmt::Debug for FifoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoCtrl")
            .field("fifowmk", &self.fifowmk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FifoCtrl {{ fifowmk: {=u8:?} }}", self.fifowmk())
    }
}
#[doc = "MICFIL FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoStat(pub u32);
impl FifoStat {
    #[doc = "FIFO Overflow Exception Flag for Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 0"]
    #[inline(always)]
    pub const fn set_fifoovf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 1"]
    #[inline(always)]
    pub const fn set_fifoovf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 2"]
    #[inline(always)]
    pub const fn set_fifoovf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Exception Flag for Channel 3"]
    #[inline(always)]
    pub const fn set_fifoovf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn fifound0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 0"]
    #[inline(always)]
    pub const fn set_fifound0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fifound1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 1"]
    #[inline(always)]
    pub const fn set_fifound1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn fifound2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 2"]
    #[inline(always)]
    pub const fn set_fifound2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn fifound3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Exception Flag for Channel 3"]
    #[inline(always)]
    pub const fn set_fifound3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for FifoStat {
    #[inline(always)]
    fn default() -> FifoStat {
        FifoStat(0)
    }
}
impl core::fmt::Debug for FifoStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoStat")
            .field("fifoovf0", &self.fifoovf0())
            .field("fifoovf1", &self.fifoovf1())
            .field("fifoovf2", &self.fifoovf2())
            .field("fifoovf3", &self.fifoovf3())
            .field("fifound0", &self.fifound0())
            .field("fifound1", &self.fifound1())
            .field("fifound2", &self.fifound2())
            .field("fifound3", &self.fifound3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FifoStat {{ fifoovf0: {=bool:?}, fifoovf1: {=bool:?}, fifoovf2: {=bool:?}, fifoovf3: {=bool:?}, fifound0: {=bool:?}, fifound1: {=bool:?}, fifound2: {=bool:?}, fifound3: {=bool:?} }}",
            self.fifoovf0(),
            self.fifoovf1(),
            self.fifoovf2(),
            self.fifoovf3(),
            self.fifound0(),
            self.fifound1(),
            self.fifound2(),
            self.fifound3()
        )
    }
}
#[doc = "Frame Synchronization Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FsyncCtrl(pub u32);
impl FsyncCtrl {
    #[doc = "Frame Synchronization Window Length"]
    #[must_use]
    #[inline(always)]
    pub const fn fsynclen(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Frame Synchronization Window Length"]
    #[inline(always)]
    pub const fn set_fsynclen(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FsyncCtrl {
    #[inline(always)]
    fn default() -> FsyncCtrl {
        FsyncCtrl(0)
    }
}
impl core::fmt::Debug for FsyncCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FsyncCtrl")
            .field("fsynclen", &self.fsynclen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FsyncCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FsyncCtrl {{ fsynclen: {=u32:?} }}", self.fsynclen())
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Microphone Pairs"]
    #[must_use]
    #[inline(always)]
    pub const fn npair(&self) -> super::vals::Npair {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Npair::from_bits(val as u8)
    }
    #[doc = "Number of Microphone Pairs"]
    #[inline(always)]
    pub const fn set_npair(&mut self, val: super::vals::Npair) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Pointer Width"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_ptrwid(&self) -> super::vals::FifoPtrwid {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::FifoPtrwid::from_bits(val as u8)
    }
    #[doc = "FIFO Pointer Width"]
    #[inline(always)]
    pub const fn set_fifo_ptrwid(&mut self, val: super::vals::FifoPtrwid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Filter Output Width"]
    #[must_use]
    #[inline(always)]
    pub const fn fil_out_width_24b(&self) -> super::vals::FilOutWidth24b {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FilOutWidth24b::from_bits(val as u8)
    }
    #[doc = "Filter Output Width"]
    #[inline(always)]
    pub const fn set_fil_out_width_24b(&mut self, val: super::vals::FilOutWidth24b) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Low-Power Decimation Filter"]
    #[must_use]
    #[inline(always)]
    pub const fn low_power(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Decimation Filter"]
    #[inline(always)]
    pub const fn set_low_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input DC Remover Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn dc_bypass(&self) -> super::vals::DcBypass {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::DcBypass::from_bits(val as u8)
    }
    #[doc = "Input DC Remover Bypass"]
    #[inline(always)]
    pub const fn set_dc_bypass(&mut self, val: super::vals::DcBypass) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Output DC Remover Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn dc_out_bypass(&self) -> super::vals::DcOutBypass {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::DcOutBypass::from_bits(val as u8)
    }
    #[doc = "Output DC Remover Bypass"]
    #[inline(always)]
    pub const fn set_dc_out_bypass(&mut self, val: super::vals::DcOutBypass) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
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
            .field("npair", &self.npair())
            .field("fifo_ptrwid", &self.fifo_ptrwid())
            .field("fil_out_width_24b", &self.fil_out_width_24b())
            .field("low_power", &self.low_power())
            .field("dc_bypass", &self.dc_bypass())
            .field("dc_out_bypass", &self.dc_out_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Param {{ npair: {:?}, fifo_ptrwid: {:?}, fil_out_width_24b: {:?}, low_power: {=bool:?}, dc_bypass: {:?}, dc_out_bypass: {:?} }}",
            self.npair(),
            self.fifo_ptrwid(),
            self.fil_out_width_24b(),
            self.low_power(),
            self.dc_bypass(),
            self.dc_out_bypass()
        )
    }
}
#[doc = "MICFIL Range Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeCtrl(pub u32);
impl RangeCtrl {
    #[doc = "Channel 0 Range Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeadj0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 0 Range Adjustment"]
    #[inline(always)]
    pub const fn set_rangeadj0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Channel 1 Range Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeadj1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 1 Range Adjustment"]
    #[inline(always)]
    pub const fn set_rangeadj1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel 2 Range Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeadj2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 2 Range Adjustment"]
    #[inline(always)]
    pub const fn set_rangeadj2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Channel 3 Range Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeadj3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 3 Range Adjustment"]
    #[inline(always)]
    pub const fn set_rangeadj3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for RangeCtrl {
    #[inline(always)]
    fn default() -> RangeCtrl {
        RangeCtrl(0)
    }
}
impl core::fmt::Debug for RangeCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RangeCtrl")
            .field("rangeadj0", &self.rangeadj0())
            .field("rangeadj1", &self.rangeadj1())
            .field("rangeadj2", &self.rangeadj2())
            .field("rangeadj3", &self.rangeadj3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RangeCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RangeCtrl {{ rangeadj0: {=u8:?}, rangeadj1: {=u8:?}, rangeadj2: {=u8:?}, rangeadj3: {=u8:?} }}",
            self.rangeadj0(),
            self.rangeadj1(),
            self.rangeadj2(),
            self.rangeadj3()
        )
    }
}
#[doc = "MICFIL Range Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeStat(pub u32);
impl RangeStat {
    #[doc = "Channel 0 Range Overflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeovf0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Range Overflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeovf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Range Overflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeovf1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Range Overflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeovf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Range Overflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeovf2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Range Overflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeovf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Range Overflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeovf3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Range Overflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeovf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel 0 Range Underflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeunf0(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Range Underflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeunf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Channel 1 Range Underflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeunf1(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Range Underflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeunf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Channel 2 Range Underflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeunf2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Range Underflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeunf2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Channel 3 Range Underflow Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rangeunf3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Range Underflow Error Flag"]
    #[inline(always)]
    pub const fn set_rangeunf3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for RangeStat {
    #[inline(always)]
    fn default() -> RangeStat {
        RangeStat(0)
    }
}
impl core::fmt::Debug for RangeStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RangeStat")
            .field("rangeovf0", &self.rangeovf0())
            .field("rangeovf1", &self.rangeovf1())
            .field("rangeovf2", &self.rangeovf2())
            .field("rangeovf3", &self.rangeovf3())
            .field("rangeunf0", &self.rangeunf0())
            .field("rangeunf1", &self.rangeunf1())
            .field("rangeunf2", &self.rangeunf2())
            .field("rangeunf3", &self.rangeunf3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RangeStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RangeStat {{ rangeovf0: {=bool:?}, rangeovf1: {=bool:?}, rangeovf2: {=bool:?}, rangeovf3: {=bool:?}, rangeunf0: {=bool:?}, rangeunf1: {=bool:?}, rangeunf2: {=bool:?}, rangeunf3: {=bool:?} }}",
            self.rangeovf0(),
            self.rangeovf1(),
            self.rangeovf2(),
            self.rangeovf3(),
            self.rangeunf0(),
            self.rangeunf1(),
            self.rangeunf2(),
            self.rangeunf3()
        )
    }
}
#[doc = "MICFIL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Channel 0 Output Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0f(&self) -> super::vals::Ch0f {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ch0f::from_bits(val as u8)
    }
    #[doc = "Channel 0 Output Data Flag"]
    #[inline(always)]
    pub const fn set_ch0f(&mut self, val: super::vals::Ch0f) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Output Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1f(&self) -> super::vals::Ch1f {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ch1f::from_bits(val as u8)
    }
    #[doc = "Channel 1 Output Data Flag"]
    #[inline(always)]
    pub const fn set_ch1f(&mut self, val: super::vals::Ch1f) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Output Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2f(&self) -> super::vals::Ch2f {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ch2f::from_bits(val as u8)
    }
    #[doc = "Channel 2 Output Data Flag"]
    #[inline(always)]
    pub const fn set_ch2f(&mut self, val: super::vals::Ch2f) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Output Data Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3f(&self) -> super::vals::Ch3f {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ch3f::from_bits(val as u8)
    }
    #[doc = "Channel 3 Output Data Flag"]
    #[inline(always)]
    pub const fn set_ch3f(&mut self, val: super::vals::Ch3f) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Busy Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy_fil(&self) -> super::vals::BsyFil {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::BsyFil::from_bits(val as u8)
    }
    #[doc = "Busy Flag"]
    #[inline(always)]
    pub const fn set_bsy_fil(&mut self, val: super::vals::BsyFil) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("ch0f", &self.ch0f())
            .field("ch1f", &self.ch1f())
            .field("ch2f", &self.ch2f())
            .field("ch3f", &self.ch3f())
            .field("bsy_fil", &self.bsy_fil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ ch0f: {:?}, ch1f: {:?}, ch2f: {:?}, ch3f: {:?}, bsy_fil: {:?} }}",
            self.ch0f(),
            self.ch1f(),
            self.ch2f(),
            self.ch3f(),
            self.bsy_fil()
        )
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
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            "Verid {{ feature: {=u16:?}, minor: {=u8:?}, major: {=u8:?} }}",
            self.feature(),
            self.minor(),
            self.major()
        )
    }
}
