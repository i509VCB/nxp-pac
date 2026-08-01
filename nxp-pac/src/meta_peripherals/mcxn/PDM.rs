#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "MICFIL."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdm {
    ptr: *mut u8,
}
unsafe impl Send for Pdm {}
unsafe impl Sync for Pdm {}
impl Pdm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MICFIL Control 1."]
    #[inline(always)]
    pub const fn ctrl_1(self) -> crate::pac::common::Reg<Ctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MICFIL Control 2."]
    #[inline(always)]
    pub const fn ctrl_2(self) -> crate::pac::common::Reg<Ctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MICFIL Status."]
    #[inline(always)]
    pub const fn stat(self) -> crate::pac::common::Reg<Stat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MICFIL FIFO Control."]
    #[inline(always)]
    pub const fn fifo_ctrl(self) -> crate::pac::common::Reg<FifoCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MICFIL FIFO Status."]
    #[inline(always)]
    pub const fn fifo_stat(self) -> crate::pac::common::Reg<FifoStat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "MICFIL Output Result."]
    #[inline(always)]
    pub const fn datach(self, n: usize) -> crate::pac::common::Reg<Datach, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _)
        }
    }
    #[doc = "MICFIL DC Remover Control."]
    #[inline(always)]
    pub const fn dc_ctrl(self) -> crate::pac::common::Reg<DcCtrl, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "MICFIL Output DC Remover Control."]
    #[inline(always)]
    pub const fn dc_out_ctrl(self) -> crate::pac::common::Reg<DcOutCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "MICFIL Range Control."]
    #[inline(always)]
    pub const fn range_ctrl(self) -> crate::pac::common::Reg<RangeCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "MICFIL Range Status."]
    #[inline(always)]
    pub const fn range_stat(self) -> crate::pac::common::Reg<RangeStat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Frame Synchronization Control."]
    #[inline(always)]
    pub const fn fsync_ctrl(self) -> crate::pac::common::Reg<FsyncCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Version ID."]
    #[inline(always)]
    pub const fn verid(self) -> crate::pac::common::Reg<Verid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Parameter."]
    #[inline(always)]
    pub const fn param(self) -> crate::pac::common::Reg<Param, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
}
#[doc = "MICFIL Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Channel Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn chen(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel Enable."]
    #[inline(always)]
    pub const fn set_chen(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Frame Synchronization Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn fsyncen(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Synchronization Enable."]
    #[inline(always)]
    pub const fn set_fsyncen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Decimation Filter Enable in Stop."]
    #[must_use]
    #[inline(always)]
    pub const fn decfils(&self) -> Decfils {
        let val = (self.0 >> 20usize) & 0x01;
        Decfils::from_bits(val as u8)
    }
    #[doc = "Decimation Filter Enable in Stop."]
    #[inline(always)]
    pub const fn set_decfils(&mut self, val: Decfils) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Error Interruption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn erren(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interruption Enable."]
    #[inline(always)]
    pub const fn set_erren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DMA Interrupt Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn disel(&self) -> Disel {
        let val = (self.0 >> 24usize) & 0x03;
        Disel::from_bits(val as u8)
    }
    #[doc = "DMA Interrupt Selection."]
    #[inline(always)]
    pub const fn set_disel(&mut self, val: Disel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Module Enable in Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Module Enable in Debug."]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn sres(&self) -> Sres {
        let val = (self.0 >> 27usize) & 0x01;
        Sres::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_sres(&mut self, val: Sres) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Debug Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn dbg(&self) -> Dbg {
        let val = (self.0 >> 28usize) & 0x01;
        Dbg::from_bits(val as u8)
    }
    #[doc = "Debug Mode."]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: Dbg) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "MICFIL Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pdmien(&self) -> Pdmien {
        let val = (self.0 >> 29usize) & 0x01;
        Pdmien::from_bits(val as u8)
    }
    #[doc = "MICFIL Enable."]
    #[inline(always)]
    pub const fn set_pdmien(&mut self, val: Pdmien) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Stop Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable."]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Module Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> Mdis {
        let val = (self.0 >> 31usize) & 0x01;
        Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable."]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: Mdis) {
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
            .field("chen[0]", &self.chen(0usize))
            .field("chen[1]", &self.chen(1usize))
            .field("chen[2]", &self.chen(2usize))
            .field("chen[3]", &self.chen(3usize))
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
            "Ctrl1 {{ chen[0]: {=bool:?}, chen[1]: {=bool:?}, chen[2]: {=bool:?}, chen[3]: {=bool:?}, fsyncen: {=bool:?}, decfils: {:?}, erren: {=bool:?}, disel: {:?}, dbge: {=bool:?}, sres: {:?}, dbg: {:?}, pdmien: {:?}, dozen: {=bool:?}, mdis: {:?} }}",
            self.chen(0usize),
            self.chen(1usize),
            self.chen(2usize),
            self.chen(3usize),
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
#[doc = "MICFIL Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> Clkdiv {
        let val = (self.0 >> 0usize) & 0xff;
        Clkdiv::from_bits(val as u8)
    }
    #[doc = "Clock Divider."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: Clkdiv) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock Divider Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdivdis(&self) -> Clkdivdis {
        let val = (self.0 >> 15usize) & 0x01;
        Clkdivdis::from_bits(val as u8)
    }
    #[doc = "Clock Divider Disable."]
    #[inline(always)]
    pub const fn set_clkdivdis(&mut self, val: Clkdivdis) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CIC Decimation Rate."]
    #[must_use]
    #[inline(always)]
    pub const fn cicosr(&self) -> Cicosr {
        let val = (self.0 >> 16usize) & 0x0f;
        Cicosr::from_bits(val as u8)
    }
    #[doc = "CIC Decimation Rate."]
    #[inline(always)]
    pub const fn set_cicosr(&mut self, val: Cicosr) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Quality Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn qsel(&self) -> Qsel {
        let val = (self.0 >> 25usize) & 0x07;
        Qsel::from_bits(val as u8)
    }
    #[doc = "Quality Mode."]
    #[inline(always)]
    pub const fn set_qsel(&mut self, val: Qsel) {
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
#[doc = "MICFIL Output Result."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datach(pub u32);
impl Datach {
    #[doc = "Channel n Data."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Channel n Data."]
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
#[doc = "MICFIL DC Remover Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCtrl(pub u32);
impl DcCtrl {
    #[doc = "Channel DC Remover Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig(&self, n: usize) -> DcCtrlDcconfig {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        DcCtrlDcconfig::from_bits(val as u8)
    }
    #[doc = "Channel DC Remover Configuration."]
    #[inline(always)]
    pub const fn set_dcconfig(&mut self, n: usize, val: DcCtrlDcconfig) {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
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
            .field("dcconfig[0]", &self.dcconfig(0usize))
            .field("dcconfig[1]", &self.dcconfig(1usize))
            .field("dcconfig[2]", &self.dcconfig(2usize))
            .field("dcconfig[3]", &self.dcconfig(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcCtrl {{ dcconfig[0]: {:?}, dcconfig[1]: {:?}, dcconfig[2]: {:?}, dcconfig[3]: {:?} }}",
            self.dcconfig(0usize),
            self.dcconfig(1usize),
            self.dcconfig(2usize),
            self.dcconfig(3usize)
        )
    }
}
#[doc = "MICFIL Output DC Remover Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcOutCtrl(pub u32);
impl DcOutCtrl {
    #[doc = "Channel DC Remover Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn dcconfig(&self, n: usize) -> DcOutCtrlDcconfig {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        DcOutCtrlDcconfig::from_bits(val as u8)
    }
    #[doc = "Channel DC Remover Configuration."]
    #[inline(always)]
    pub const fn set_dcconfig(&mut self, n: usize, val: DcOutCtrlDcconfig) {
        assert!(n < 4usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
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
            .field("dcconfig[0]", &self.dcconfig(0usize))
            .field("dcconfig[1]", &self.dcconfig(1usize))
            .field("dcconfig[2]", &self.dcconfig(2usize))
            .field("dcconfig[3]", &self.dcconfig(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DcOutCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DcOutCtrl {{ dcconfig[0]: {:?}, dcconfig[1]: {:?}, dcconfig[2]: {:?}, dcconfig[3]: {:?} }}",
            self.dcconfig(0usize),
            self.dcconfig(1usize),
            self.dcconfig(2usize),
            self.dcconfig(3usize)
        )
    }
}
#[doc = "MICFIL FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoCtrl(pub u32);
impl FifoCtrl {
    #[doc = "FIFO Watermark Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fifowmk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Watermark Control."]
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
#[doc = "MICFIL FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoStat(pub u32);
impl FifoStat {
    #[doc = "FIFO Overflow Exception Flag for Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovf(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow Exception Flag for Channel."]
    #[inline(always)]
    pub const fn set_fifoovf(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "FIFO Underflow Exception Flag for Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn fifound(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow Exception Flag for Channel."]
    #[inline(always)]
    pub const fn set_fifound(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 8usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("fifoovf[0]", &self.fifoovf(0usize))
            .field("fifoovf[1]", &self.fifoovf(1usize))
            .field("fifoovf[2]", &self.fifoovf(2usize))
            .field("fifoovf[3]", &self.fifoovf(3usize))
            .field("fifound[0]", &self.fifound(0usize))
            .field("fifound[1]", &self.fifound(1usize))
            .field("fifound[2]", &self.fifound(2usize))
            .field("fifound[3]", &self.fifound(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FifoStat {{ fifoovf[0]: {=bool:?}, fifoovf[1]: {=bool:?}, fifoovf[2]: {=bool:?}, fifoovf[3]: {=bool:?}, fifound[0]: {=bool:?}, fifound[1]: {=bool:?}, fifound[2]: {=bool:?}, fifound[3]: {=bool:?} }}",
            self.fifoovf(0usize),
            self.fifoovf(1usize),
            self.fifoovf(2usize),
            self.fifoovf(3usize),
            self.fifound(0usize),
            self.fifound(1usize),
            self.fifound(2usize),
            self.fifound(3usize)
        )
    }
}
#[doc = "Frame Synchronization Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FsyncCtrl(pub u32);
impl FsyncCtrl {
    #[doc = "Frame Synchronization Window Length."]
    #[must_use]
    #[inline(always)]
    pub const fn fsynclen(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Frame Synchronization Window Length."]
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
#[doc = "Parameter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Microphone Pairs."]
    #[must_use]
    #[inline(always)]
    pub const fn npair(&self) -> Npair {
        let val = (self.0 >> 0usize) & 0x0f;
        Npair::from_bits(val as u8)
    }
    #[doc = "Number of Microphone Pairs."]
    #[inline(always)]
    pub const fn set_npair(&mut self, val: Npair) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "FIFO Pointer Width."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_ptrwid(&self) -> FifoPtrwid {
        let val = (self.0 >> 4usize) & 0x0f;
        FifoPtrwid::from_bits(val as u8)
    }
    #[doc = "FIFO Pointer Width."]
    #[inline(always)]
    pub const fn set_fifo_ptrwid(&mut self, val: FifoPtrwid) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Filter Output Width."]
    #[must_use]
    #[inline(always)]
    pub const fn fil_out_width_24b(&self) -> FilOutWidth24b {
        let val = (self.0 >> 8usize) & 0x01;
        FilOutWidth24b::from_bits(val as u8)
    }
    #[doc = "Filter Output Width."]
    #[inline(always)]
    pub const fn set_fil_out_width_24b(&mut self, val: FilOutWidth24b) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Low-Power Decimation Filter."]
    #[must_use]
    #[inline(always)]
    pub const fn low_power(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Low-Power Decimation Filter."]
    #[inline(always)]
    pub const fn set_low_power(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Input DC Remover Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn dc_bypass(&self) -> DcBypass {
        let val = (self.0 >> 10usize) & 0x01;
        DcBypass::from_bits(val as u8)
    }
    #[doc = "Input DC Remover Bypass."]
    #[inline(always)]
    pub const fn set_dc_bypass(&mut self, val: DcBypass) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Output DC Remover Bypass."]
    #[must_use]
    #[inline(always)]
    pub const fn dc_out_bypass(&self) -> DcOutBypass {
        let val = (self.0 >> 11usize) & 0x01;
        DcOutBypass::from_bits(val as u8)
    }
    #[doc = "Output DC Remover Bypass."]
    #[inline(always)]
    pub const fn set_dc_out_bypass(&mut self, val: DcOutBypass) {
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
#[doc = "MICFIL Range Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeCtrl(pub u32);
impl RangeCtrl {
    #[doc = "Channel Range Adjustment."]
    #[must_use]
    #[inline(always)]
    pub const fn rangeadj(&self, n: usize) -> u8 {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x0f;
        val as u8
    }
    #[doc = "Channel Range Adjustment."]
    #[inline(always)]
    pub const fn set_rangeadj(&mut self, n: usize, val: u8) {
        assert!(n < 4usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
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
            .field("rangeadj[0]", &self.rangeadj(0usize))
            .field("rangeadj[1]", &self.rangeadj(1usize))
            .field("rangeadj[2]", &self.rangeadj(2usize))
            .field("rangeadj[3]", &self.rangeadj(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RangeCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RangeCtrl {{ rangeadj[0]: {=u8:?}, rangeadj[1]: {=u8:?}, rangeadj[2]: {=u8:?}, rangeadj[3]: {=u8:?} }}",
            self.rangeadj(0usize),
            self.rangeadj(1usize),
            self.rangeadj(2usize),
            self.rangeadj(3usize)
        )
    }
}
#[doc = "MICFIL Range Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RangeStat(pub u32);
impl RangeStat {
    #[doc = "Channel Range Overflow Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rangeovf(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel Range Overflow Error Flag."]
    #[inline(always)]
    pub const fn set_rangeovf(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Channel Range Underflow Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn rangeunf(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Channel Range Underflow Error Flag."]
    #[inline(always)]
    pub const fn set_rangeunf(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 16usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
            .field("rangeovf[0]", &self.rangeovf(0usize))
            .field("rangeovf[1]", &self.rangeovf(1usize))
            .field("rangeovf[2]", &self.rangeovf(2usize))
            .field("rangeovf[3]", &self.rangeovf(3usize))
            .field("rangeunf[0]", &self.rangeunf(0usize))
            .field("rangeunf[1]", &self.rangeunf(1usize))
            .field("rangeunf[2]", &self.rangeunf(2usize))
            .field("rangeunf[3]", &self.rangeunf(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RangeStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RangeStat {{ rangeovf[0]: {=bool:?}, rangeovf[1]: {=bool:?}, rangeovf[2]: {=bool:?}, rangeovf[3]: {=bool:?}, rangeunf[0]: {=bool:?}, rangeunf[1]: {=bool:?}, rangeunf[2]: {=bool:?}, rangeunf[3]: {=bool:?} }}",
            self.rangeovf(0usize),
            self.rangeovf(1usize),
            self.rangeovf(2usize),
            self.rangeovf(3usize),
            self.rangeunf(0usize),
            self.rangeunf(1usize),
            self.rangeunf(2usize),
            self.rangeunf(3usize)
        )
    }
}
#[doc = "MICFIL Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Channel Output Data Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn chf(&self, n: usize) -> Chf {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        Chf::from_bits(val as u8)
    }
    #[doc = "Channel Output Data Flag."]
    #[inline(always)]
    pub const fn set_chf(&mut self, n: usize, val: Chf) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "Busy Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn bsy_fil(&self) -> BsyFil {
        let val = (self.0 >> 31usize) & 0x01;
        BsyFil::from_bits(val as u8)
    }
    #[doc = "Busy Flag."]
    #[inline(always)]
    pub const fn set_bsy_fil(&mut self, val: BsyFil) {
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
            .field("chf[0]", &self.chf(0usize))
            .field("chf[1]", &self.chf(1usize))
            .field("chf[2]", &self.chf(2usize))
            .field("chf[3]", &self.chf(3usize))
            .field("bsy_fil", &self.bsy_fil())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Stat {{ chf[0]: {:?}, chf[1]: {:?}, chf[2]: {:?}, chf[3]: {:?}, bsy_fil: {:?} }}",
            self.chf(0usize),
            self.chf(1usize),
            self.chf(2usize),
            self.chf(3usize),
            self.bsy_fil()
        )
    }
}
#[doc = "Version ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number."]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Number."]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number."]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number."]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number."]
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BsyFil {
    #[doc = "MICFIL is stopped."]
    Stopped = 0x0,
    #[doc = "MICFIL is running."]
    Running = 0x01,
}
impl BsyFil {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BsyFil {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BsyFil {
    #[inline(always)]
    fn from(val: u8) -> BsyFil {
        BsyFil::from_bits(val)
    }
}
impl From<BsyFil> for u8 {
    #[inline(always)]
    fn from(val: BsyFil) -> u8 {
        BsyFil::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chf {
    #[doc = "Not surpassed."]
    WmNotreached = 0x0,
    #[doc = "Surpassed."]
    WmReached = 0x01,
}
impl Chf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chf {
    #[inline(always)]
    fn from(val: u8) -> Chf {
        Chf::from_bits(val)
    }
}
impl From<Chf> for u8 {
    #[inline(always)]
    fn from(val: Chf) -> u8 {
        Chf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cicosr {
    #[doc = "CIC oversampling rate = 0."]
    Cicosr0 = 0x0,
    #[doc = "CIC oversampling rate = 1."]
    Cicosr1 = 0x01,
    #[doc = "..."]
    Cicosr22 = 0x02,
    #[doc = "..."]
    Cicosr23 = 0x03,
    #[doc = "..."]
    Cicosr24 = 0x04,
    #[doc = "..."]
    Cicosr25 = 0x05,
    #[doc = "..."]
    Cicosr26 = 0x06,
    #[doc = "..."]
    Cicosr27 = 0x07,
    #[doc = "..."]
    Cicosr28 = 0x08,
    #[doc = "..."]
    Cicosr29 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "CIC oversampling rate = 15."]
    Cicosr15 = 0x0f,
}
impl Cicosr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cicosr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cicosr {
    #[inline(always)]
    fn from(val: u8) -> Cicosr {
        Cicosr::from_bits(val)
    }
}
impl From<Cicosr> for u8 {
    #[inline(always)]
    fn from(val: Cicosr) -> u8 {
        Cicosr::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Clkdiv(u8);
impl Clkdiv {
    #[doc = "Internal clock divider value = 0."]
    pub const Clkdiv0: Self = Self(0x0);
    #[doc = "Internal clock divider value = 1."]
    pub const Clkdiv1: Self = Self(0x01);
    #[doc = "..."]
    pub const Clkdiv22: Self = Self(0x02);
    #[doc = "..."]
    pub const Clkdiv23: Self = Self(0x03);
    #[doc = "..."]
    pub const Clkdiv24: Self = Self(0x04);
    #[doc = "..."]
    pub const Clkdiv25: Self = Self(0x05);
    #[doc = "..."]
    pub const Clkdiv26: Self = Self(0x06);
    #[doc = "..."]
    pub const Clkdiv27: Self = Self(0x07);
    #[doc = "..."]
    pub const Clkdiv28: Self = Self(0x08);
    #[doc = "..."]
    pub const Clkdiv29: Self = Self(0x09);
    #[doc = "Internal clock divider value = 255."]
    pub const Clkdiv255: Self = Self(0xff);
}
impl Clkdiv {
    pub const fn from_bits(val: u8) -> Clkdiv {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Clkdiv0"),
            0x01 => f.write_str("Clkdiv1"),
            0x02 => f.write_str("Clkdiv22"),
            0x03 => f.write_str("Clkdiv23"),
            0x04 => f.write_str("Clkdiv24"),
            0x05 => f.write_str("Clkdiv25"),
            0x06 => f.write_str("Clkdiv26"),
            0x07 => f.write_str("Clkdiv27"),
            0x08 => f.write_str("Clkdiv28"),
            0x09 => f.write_str("Clkdiv29"),
            0xff => f.write_str("Clkdiv255"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkdiv {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Clkdiv0"),
            0x01 => defmt::write!(f, "Clkdiv1"),
            0x02 => defmt::write!(f, "Clkdiv22"),
            0x03 => defmt::write!(f, "Clkdiv23"),
            0x04 => defmt::write!(f, "Clkdiv24"),
            0x05 => defmt::write!(f, "Clkdiv25"),
            0x06 => defmt::write!(f, "Clkdiv26"),
            0x07 => defmt::write!(f, "Clkdiv27"),
            0x08 => defmt::write!(f, "Clkdiv28"),
            0x09 => defmt::write!(f, "Clkdiv29"),
            0xff => defmt::write!(f, "Clkdiv255"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Clkdiv {
    #[inline(always)]
    fn from(val: u8) -> Clkdiv {
        Clkdiv::from_bits(val)
    }
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(val: Clkdiv) -> u8 {
        Clkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkdivdis {
    #[doc = "Enables."]
    Enable = 0x0,
    #[doc = "Disables."]
    Disable = 0x01,
}
impl Clkdivdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkdivdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkdivdis {
    #[inline(always)]
    fn from(val: u8) -> Clkdivdis {
        Clkdivdis::from_bits(val)
    }
}
impl From<Clkdivdis> for u8 {
    #[inline(always)]
    fn from(val: Clkdivdis) -> u8 {
        Clkdivdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbg {
    #[doc = "Normal."]
    Normal = 0x0,
    #[doc = "Debug."]
    Debug = 0x01,
}
impl Dbg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbg {
    #[inline(always)]
    fn from(val: u8) -> Dbg {
        Dbg::from_bits(val)
    }
}
impl From<Dbg> for u8 {
    #[inline(always)]
    fn from(val: Dbg) -> u8 {
        Dbg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcBypass {
    #[doc = "Active."]
    Dcactive = 0x0,
    #[doc = "Disabled."]
    Dcbypassed = 0x01,
}
impl DcBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcBypass {
    #[inline(always)]
    fn from(val: u8) -> DcBypass {
        DcBypass::from_bits(val)
    }
}
impl From<DcBypass> for u8 {
    #[inline(always)]
    fn from(val: DcBypass) -> u8 {
        DcBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcCtrlDcconfig {
    #[doc = "20 Hz (PDM_CLK = 3.072 MHz)."]
    DcRem20hz = 0x0,
    #[doc = "13.3 Hz (PDM_CLK = 3.072 MHz)."]
    DcRem13p3hz = 0x01,
    #[doc = "40 Hz (PDM_CLK = 3.072 MHz)."]
    DcRem40hz = 0x02,
    #[doc = "DC remover is bypassed."]
    DcRemBypass = 0x03,
}
impl DcCtrlDcconfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcCtrlDcconfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcCtrlDcconfig {
    #[inline(always)]
    fn from(val: u8) -> DcCtrlDcconfig {
        DcCtrlDcconfig::from_bits(val)
    }
}
impl From<DcCtrlDcconfig> for u8 {
    #[inline(always)]
    fn from(val: DcCtrlDcconfig) -> u8 {
        DcCtrlDcconfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutBypass {
    #[doc = "Active."]
    Dcactive = 0x0,
    #[doc = "Disabled."]
    Dcbypassed = 0x01,
}
impl DcOutBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutBypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutBypass {
    #[inline(always)]
    fn from(val: u8) -> DcOutBypass {
        DcOutBypass::from_bits(val)
    }
}
impl From<DcOutBypass> for u8 {
    #[inline(always)]
    fn from(val: DcOutBypass) -> u8 {
        DcOutBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DcOutCtrlDcconfig {
    #[doc = "20 Hz (FS = 48 kHz)."]
    DcRem20hz = 0x0,
    #[doc = "13.3 Hz (FS = 48 kHz)."]
    DcRem13p3hz = 0x01,
    #[doc = "40 Hz (FS = 48 kHz)."]
    DcRem40hz = 0x02,
    #[doc = "DC remover is bypassed."]
    DcRemBypassed = 0x03,
}
impl DcOutCtrlDcconfig {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DcOutCtrlDcconfig {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DcOutCtrlDcconfig {
    #[inline(always)]
    fn from(val: u8) -> DcOutCtrlDcconfig {
        DcOutCtrlDcconfig::from_bits(val)
    }
}
impl From<DcOutCtrlDcconfig> for u8 {
    #[inline(always)]
    fn from(val: DcOutCtrlDcconfig) -> u8 {
        DcOutCtrlDcconfig::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decfils {
    #[doc = "Stops decimation filter."]
    Stop = 0x0,
    #[doc = "Keeps decimation filter running."]
    Run = 0x01,
}
impl Decfils {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Decfils {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Decfils {
    #[inline(always)]
    fn from(val: u8) -> Decfils {
        Decfils::from_bits(val)
    }
}
impl From<Decfils> for u8 {
    #[inline(always)]
    fn from(val: Decfils) -> u8 {
        Decfils::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Disel {
    #[doc = "Disables DMA and interrupt requests."]
    AllDisabled = 0x0,
    #[doc = "Enables DMA requests."]
    DmareqEnabled = 0x01,
    #[doc = "Enables interrupt requests."]
    IntreqEnabled = 0x02,
    _RESERVED_3 = 0x03,
}
impl Disel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Disel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Disel {
    #[inline(always)]
    fn from(val: u8) -> Disel {
        Disel::from_bits(val)
    }
}
impl From<Disel> for u8 {
    #[inline(always)]
    fn from(val: Disel) -> u8 {
        Disel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FifoPtrwid {
    #[doc = "0 bits."]
    Ptrwid0 = 0x0,
    #[doc = "1 bit."]
    Ptrwid1 = 0x01,
    #[doc = "2 bits."]
    Ptrwid2 = 0x02,
    #[doc = "..."]
    Ptrwid33 = 0x03,
    #[doc = "..."]
    Ptrwid34 = 0x04,
    #[doc = "..."]
    Ptrwid35 = 0x05,
    #[doc = "..."]
    Ptrwid36 = 0x06,
    #[doc = "..."]
    Ptrwid37 = 0x07,
    #[doc = "..."]
    Ptrwid38 = 0x08,
    #[doc = "..."]
    Ptrwid39 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "15 bits."]
    Ptrwid15 = 0x0f,
}
impl FifoPtrwid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FifoPtrwid {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FifoPtrwid {
    #[inline(always)]
    fn from(val: u8) -> FifoPtrwid {
        FifoPtrwid::from_bits(val)
    }
}
impl From<FifoPtrwid> for u8 {
    #[inline(always)]
    fn from(val: FifoPtrwid) -> u8 {
        FifoPtrwid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FilOutWidth24b {
    #[doc = "16 bits."]
    Wid16b = 0x0,
    #[doc = "24 bits."]
    Wid24b = 0x01,
}
impl FilOutWidth24b {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FilOutWidth24b {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FilOutWidth24b {
    #[inline(always)]
    fn from(val: u8) -> FilOutWidth24b {
        FilOutWidth24b::from_bits(val)
    }
}
impl From<FilOutWidth24b> for u8 {
    #[inline(always)]
    fn from(val: FilOutWidth24b) -> u8 {
        FilOutWidth24b::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Normal mode."]
    Normal = 0x0,
    #[doc = "DLL mode."]
    LowLeakage = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Npair {
    #[doc = "None."]
    Npair0 = 0x0,
    #[doc = "1 pair."]
    Npair1 = 0x01,
    #[doc = "2 pairs."]
    Npair2 = 0x02,
    #[doc = "..."]
    Npair33 = 0x03,
    #[doc = "..."]
    Npair34 = 0x04,
    #[doc = "..."]
    Npair35 = 0x05,
    #[doc = "..."]
    Npair36 = 0x06,
    #[doc = "..."]
    Npair37 = 0x07,
    #[doc = "..."]
    Npair38 = 0x08,
    #[doc = "..."]
    Npair39 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "15 pairs."]
    Npair15 = 0x0f,
}
impl Npair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Npair {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Npair {
    #[inline(always)]
    fn from(val: u8) -> Npair {
        Npair::from_bits(val)
    }
}
impl From<Npair> for u8 {
    #[inline(always)]
    fn from(val: Npair) -> u8 {
        Npair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pdmien {
    #[doc = "Stops MICFIL operation."]
    Stopped = 0x0,
    #[doc = "Starts MICFIL operation."]
    Started = 0x01,
}
impl Pdmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pdmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pdmien {
    #[inline(always)]
    fn from(val: u8) -> Pdmien {
        Pdmien::from_bits(val)
    }
}
impl From<Pdmien> for u8 {
    #[inline(always)]
    fn from(val: Pdmien) -> u8 {
        Pdmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qsel {
    #[doc = "Medium-Quality mode."]
    MqMode = 0x0,
    #[doc = "High-Quality mode."]
    HqMode = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Very-Low-Quality 2 mode."]
    Vlq2Mode = 0x04,
    #[doc = "Very-Low-Quality 1 mode."]
    Vlq1Mode = 0x05,
    #[doc = "Very-Low-Quality 0 mode."]
    Vlq0Mode = 0x06,
    #[doc = "Low-Quality mode."]
    LqMode = 0x07,
}
impl Qsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qsel {
    #[inline(always)]
    fn from(val: u8) -> Qsel {
        Qsel::from_bits(val)
    }
}
impl From<Qsel> for u8 {
    #[inline(always)]
    fn from(val: Qsel) -> u8 {
        Qsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sres {
    #[doc = "No action."]
    NoAction = 0x0,
    #[doc = "Software reset."]
    SwReset = 0x01,
}
impl Sres {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sres {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sres {
    #[inline(always)]
    fn from(val: u8) -> Sres {
        Sres::from_bits(val)
    }
}
impl From<Sres> for u8 {
    #[inline(always)]
    fn from(val: Sres) -> u8 {
        Sres::to_bits(val)
    }
}
