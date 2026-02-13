#[doc = "ADMA Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaErrStatus(pub u32);
impl AdmaErrStatus {
    #[doc = "ADMA error state (when ADMA error is occurred)"]
    #[must_use]
    #[inline(always)]
    pub const fn admaes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADMA error state (when ADMA error is occurred)"]
    #[inline(always)]
    pub const fn set_admaes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADMA length mismatch error"]
    #[must_use]
    #[inline(always)]
    pub const fn admalme(&self) -> super::vals::Admalme {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Admalme::from_bits(val as u8)
    }
    #[doc = "ADMA length mismatch error"]
    #[inline(always)]
    pub const fn set_admalme(&mut self, val: super::vals::Admalme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ADMA descriptor error"]
    #[must_use]
    #[inline(always)]
    pub const fn admadce(&self) -> super::vals::Admadce {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Admadce::from_bits(val as u8)
    }
    #[doc = "ADMA descriptor error"]
    #[inline(always)]
    pub const fn set_admadce(&mut self, val: super::vals::Admadce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for AdmaErrStatus {
    #[inline(always)]
    fn default() -> AdmaErrStatus {
        AdmaErrStatus(0)
    }
}
impl core::fmt::Debug for AdmaErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdmaErrStatus")
            .field("admaes", &self.admaes())
            .field("admalme", &self.admalme())
            .field("admadce", &self.admadce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdmaErrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AdmaErrStatus {{ admaes: {=u8:?}, admalme: {:?}, admadce: {:?} }}",
            self.admaes(),
            self.admalme(),
            self.admadce()
        )
    }
}
#[doc = "ADMA System Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaSysAddr(pub u32);
impl AdmaSysAddr {
    #[doc = "ADMA system address"]
    #[must_use]
    #[inline(always)]
    pub const fn ads_addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "ADMA system address"]
    #[inline(always)]
    pub const fn set_ads_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for AdmaSysAddr {
    #[inline(always)]
    fn default() -> AdmaSysAddr {
        AdmaSysAddr(0)
    }
}
impl core::fmt::Debug for AdmaSysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdmaSysAddr")
            .field("ads_addr", &self.ads_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdmaSysAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdmaSysAddr {{ ads_addr: {=u32:?} }}", self.ads_addr())
    }
}
#[doc = "Auto CMD12 Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autocmd12ErrStatus(pub u32);
impl Autocmd12ErrStatus {
    #[doc = "Auto CMD12 not executed"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ne(&self) -> super::vals::Ac12ne {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ac12ne::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 not executed"]
    #[inline(always)]
    pub const fn set_ac12ne(&mut self, val: super::vals::Ac12ne) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Auto CMD12 / 23 timeout error"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12toe(&self) -> super::vals::Ac12toe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Ac12toe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 timeout error"]
    #[inline(always)]
    pub const fn set_ac12toe(&mut self, val: super::vals::Ac12toe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 / 23 end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ebe(&self) -> super::vals::Ac12ebe {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ac12ebe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 end bit error"]
    #[inline(always)]
    pub const fn set_ac12ebe(&mut self, val: super::vals::Ac12ebe) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto CMD12 / 23 CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ce(&self) -> super::vals::Ac12ce {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ac12ce::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 CRC error"]
    #[inline(always)]
    pub const fn set_ac12ce(&mut self, val: super::vals::Ac12ce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Auto CMD12 / 23 index error"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ie(&self) -> super::vals::Ac12ie {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ac12ie::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 index error"]
    #[inline(always)]
    pub const fn set_ac12ie(&mut self, val: super::vals::Ac12ie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command not issued by Auto CMD12 error"]
    #[must_use]
    #[inline(always)]
    pub const fn cnibac12e(&self) -> super::vals::Cnibac12e {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Cnibac12e::from_bits(val as u8)
    }
    #[doc = "Command not issued by Auto CMD12 error"]
    #[inline(always)]
    pub const fn set_cnibac12e(&mut self, val: super::vals::Cnibac12e) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute tuning"]
    #[must_use]
    #[inline(always)]
    pub const fn execute_tuning(&self) -> super::vals::ExecuteTuning {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ExecuteTuning::from_bits(val as u8)
    }
    #[doc = "Execute tuning"]
    #[inline(always)]
    pub const fn set_execute_tuning(&mut self, val: super::vals::ExecuteTuning) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Sample clock select"]
    #[must_use]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> super::vals::Autocmd12ErrStatusSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Autocmd12ErrStatusSmpClkSel::from_bits(val as u8)
    }
    #[doc = "Sample clock select"]
    #[inline(always)]
    pub const fn set_smp_clk_sel(&mut self, val: super::vals::Autocmd12ErrStatusSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Autocmd12ErrStatus {
    #[inline(always)]
    fn default() -> Autocmd12ErrStatus {
        Autocmd12ErrStatus(0)
    }
}
impl core::fmt::Debug for Autocmd12ErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autocmd12ErrStatus")
            .field("ac12ne", &self.ac12ne())
            .field("ac12toe", &self.ac12toe())
            .field("ac12ebe", &self.ac12ebe())
            .field("ac12ce", &self.ac12ce())
            .field("ac12ie", &self.ac12ie())
            .field("cnibac12e", &self.cnibac12e())
            .field("execute_tuning", &self.execute_tuning())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autocmd12ErrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autocmd12ErrStatus {{ ac12ne: {:?}, ac12toe: {:?}, ac12ebe: {:?}, ac12ce: {:?}, ac12ie: {:?}, cnibac12e: {:?}, execute_tuning: {:?}, smp_clk_sel: {:?} }}",
            self.ac12ne(),
            self.ac12toe(),
            self.ac12ebe(),
            self.ac12ce(),
            self.ac12ie(),
            self.cnibac12e(),
            self.execute_tuning(),
            self.smp_clk_sel()
        )
    }
}
#[doc = "Block Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkAtt(pub u32);
impl BlkAtt {
    #[doc = "Transfer block size"]
    #[must_use]
    #[inline(always)]
    pub const fn blksize(&self) -> super::vals::Blksize {
        let val = (self.0 >> 0usize) & 0x1fff;
        super::vals::Blksize::from_bits(val as u16)
    }
    #[doc = "Transfer block size"]
    #[inline(always)]
    pub const fn set_blksize(&mut self, val: super::vals::Blksize) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val.to_bits() as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Blocks count for current transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn blkcnt(&self) -> super::vals::Blkcnt {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Blkcnt::from_bits(val as u16)
    }
    #[doc = "Blocks count for current transfer"]
    #[inline(always)]
    pub const fn set_blkcnt(&mut self, val: super::vals::Blkcnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for BlkAtt {
    #[inline(always)]
    fn default() -> BlkAtt {
        BlkAtt(0)
    }
}
impl core::fmt::Debug for BlkAtt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BlkAtt")
            .field("blksize", &self.blksize())
            .field("blkcnt", &self.blkcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BlkAtt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BlkAtt {{ blksize: {:?}, blkcnt: {:?} }}",
            self.blksize(),
            self.blkcnt()
        )
    }
}
#[doc = "CLK Tuning Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTuneCtrlStatus(pub u32);
impl ClkTuneCtrlStatus {
    #[doc = "Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_post(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells on the feedback clock between CLK_OUT and CLK_POST"]
    #[inline(always)]
    pub const fn set_dly_cell_set_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_out(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells on the feedback clock between CLK_PRE and CLK_OUT"]
    #[inline(always)]
    pub const fn set_dly_cell_set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "delay cells on the feedback clock between the feedback clock and CLK_PRE"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_pre(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "delay cells on the feedback clock between the feedback clock and CLK_PRE"]
    #[inline(always)]
    pub const fn set_dly_cell_set_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "NXT error"]
    #[must_use]
    #[inline(always)]
    pub const fn nxt_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NXT error"]
    #[inline(always)]
    pub const fn set_nxt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Delay cells added on the feedback clock between CLK_OUT and CLK_POST"]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_post(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells added on the feedback clock between CLK_OUT and CLK_POST"]
    #[inline(always)]
    pub const fn set_tap_sel_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Delay cells added on the feedback clock between CLK_PRE and CLK_OUT"]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_out(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells added on the feedback clock between CLK_PRE and CLK_OUT"]
    #[inline(always)]
    pub const fn set_tap_sel_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "TAP_SEL_PRE"]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_pre(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "TAP_SEL_PRE"]
    #[inline(always)]
    pub const fn set_tap_sel_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "PRE error"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PRE error"]
    #[inline(always)]
    pub const fn set_pre_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkTuneCtrlStatus {
    #[inline(always)]
    fn default() -> ClkTuneCtrlStatus {
        ClkTuneCtrlStatus(0)
    }
}
impl core::fmt::Debug for ClkTuneCtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkTuneCtrlStatus")
            .field("dly_cell_set_post", &self.dly_cell_set_post())
            .field("dly_cell_set_out", &self.dly_cell_set_out())
            .field("dly_cell_set_pre", &self.dly_cell_set_pre())
            .field("nxt_err", &self.nxt_err())
            .field("tap_sel_post", &self.tap_sel_post())
            .field("tap_sel_out", &self.tap_sel_out())
            .field("tap_sel_pre", &self.tap_sel_pre())
            .field("pre_err", &self.pre_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkTuneCtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkTuneCtrlStatus {{ dly_cell_set_post: {=u8:?}, dly_cell_set_out: {=u8:?}, dly_cell_set_pre: {=u8:?}, nxt_err: {=bool:?}, tap_sel_post: {=u8:?}, tap_sel_out: {=u8:?}, tap_sel_pre: {=u8:?}, pre_err: {=bool:?} }}",
            self.dly_cell_set_post(),
            self.dly_cell_set_out(),
            self.dly_cell_set_pre(),
            self.nxt_err(),
            self.tap_sel_post(),
            self.tap_sel_out(),
            self.tap_sel_pre(),
            self.pre_err()
        )
    }
}
#[doc = "Command Argument"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdArg(pub u32);
impl CmdArg {
    #[doc = "Command argument"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdarg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command argument"]
    #[inline(always)]
    pub const fn set_cmdarg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdArg {
    #[inline(always)]
    fn default() -> CmdArg {
        CmdArg(0)
    }
}
impl core::fmt::Debug for CmdArg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdArg")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdArg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdArg {{ cmdarg: {=u32:?} }}", self.cmdarg())
    }
}
#[doc = "Command Response0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp0(pub u32);
impl CmdRsp0 {
    #[doc = "Command response 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrsp0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command response 0"]
    #[inline(always)]
    pub const fn set_cmdrsp0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp0 {
    #[inline(always)]
    fn default() -> CmdRsp0 {
        CmdRsp0(0)
    }
}
impl core::fmt::Debug for CmdRsp0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdRsp0")
            .field("cmdrsp0", &self.cmdrsp0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdRsp0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdRsp0 {{ cmdrsp0: {=u32:?} }}", self.cmdrsp0())
    }
}
#[doc = "Command Response1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp1(pub u32);
impl CmdRsp1 {
    #[doc = "Command response 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrsp1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command response 1"]
    #[inline(always)]
    pub const fn set_cmdrsp1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp1 {
    #[inline(always)]
    fn default() -> CmdRsp1 {
        CmdRsp1(0)
    }
}
impl core::fmt::Debug for CmdRsp1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdRsp1")
            .field("cmdrsp1", &self.cmdrsp1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdRsp1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdRsp1 {{ cmdrsp1: {=u32:?} }}", self.cmdrsp1())
    }
}
#[doc = "Command Response2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp2(pub u32);
impl CmdRsp2 {
    #[doc = "Command response 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrsp2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command response 2"]
    #[inline(always)]
    pub const fn set_cmdrsp2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp2 {
    #[inline(always)]
    fn default() -> CmdRsp2 {
        CmdRsp2(0)
    }
}
impl core::fmt::Debug for CmdRsp2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdRsp2")
            .field("cmdrsp2", &self.cmdrsp2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdRsp2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdRsp2 {{ cmdrsp2: {=u32:?} }}", self.cmdrsp2())
    }
}
#[doc = "Command Response3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp3(pub u32);
impl CmdRsp3 {
    #[doc = "Command response 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrsp3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command response 3"]
    #[inline(always)]
    pub const fn set_cmdrsp3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp3 {
    #[inline(always)]
    fn default() -> CmdRsp3 {
        CmdRsp3(0)
    }
}
impl core::fmt::Debug for CmdRsp3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdRsp3")
            .field("cmdrsp3", &self.cmdrsp3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdRsp3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdRsp3 {{ cmdrsp3: {=u32:?} }}", self.cmdrsp3())
    }
}
#[doc = "Command Transfer Type"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdXfrTyp(pub u32);
impl CmdXfrTyp {
    #[doc = "Response type select"]
    #[must_use]
    #[inline(always)]
    pub const fn rsptyp(&self) -> super::vals::Rsptyp {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Rsptyp::from_bits(val as u8)
    }
    #[doc = "Response type select"]
    #[inline(always)]
    pub const fn set_rsptyp(&mut self, val: super::vals::Rsptyp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command CRC check enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cccen(&self) -> super::vals::Cccen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cccen::from_bits(val as u8)
    }
    #[doc = "Command CRC check enable"]
    #[inline(always)]
    pub const fn set_cccen(&mut self, val: super::vals::Cccen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Command index check enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cicen(&self) -> super::vals::Cicen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Cicen::from_bits(val as u8)
    }
    #[doc = "Command index check enable"]
    #[inline(always)]
    pub const fn set_cicen(&mut self, val: super::vals::Cicen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data present select"]
    #[must_use]
    #[inline(always)]
    pub const fn dpsel(&self) -> super::vals::Dpsel {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dpsel::from_bits(val as u8)
    }
    #[doc = "Data present select"]
    #[inline(always)]
    pub const fn set_dpsel(&mut self, val: super::vals::Dpsel) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Command type"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtyp(&self) -> super::vals::Cmdtyp {
        let val = (self.0 >> 22usize) & 0x03;
        super::vals::Cmdtyp::from_bits(val as u8)
    }
    #[doc = "Command type"]
    #[inline(always)]
    pub const fn set_cmdtyp(&mut self, val: super::vals::Cmdtyp) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Command index"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdinx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index"]
    #[inline(always)]
    pub const fn set_cmdinx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for CmdXfrTyp {
    #[inline(always)]
    fn default() -> CmdXfrTyp {
        CmdXfrTyp(0)
    }
}
impl core::fmt::Debug for CmdXfrTyp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdXfrTyp")
            .field("rsptyp", &self.rsptyp())
            .field("cccen", &self.cccen())
            .field("cicen", &self.cicen())
            .field("dpsel", &self.dpsel())
            .field("cmdtyp", &self.cmdtyp())
            .field("cmdinx", &self.cmdinx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdXfrTyp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmdXfrTyp {{ rsptyp: {:?}, cccen: {:?}, cicen: {:?}, dpsel: {:?}, cmdtyp: {:?}, cmdinx: {=u8:?} }}",
            self.rsptyp(),
            self.cccen(),
            self.cicen(),
            self.dpsel(),
            self.cmdtyp(),
            self.cmdinx()
        )
    }
}
#[doc = "Data Buffer Access Port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataBuffAccPort(pub u32);
impl DataBuffAccPort {
    #[doc = "Data content"]
    #[must_use]
    #[inline(always)]
    pub const fn datcont(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data content"]
    #[inline(always)]
    pub const fn set_datcont(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DataBuffAccPort {
    #[inline(always)]
    fn default() -> DataBuffAccPort {
        DataBuffAccPort(0)
    }
}
impl core::fmt::Debug for DataBuffAccPort {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DataBuffAccPort")
            .field("datcont", &self.datcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DataBuffAccPort {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DataBuffAccPort {{ datcont: {=u32:?} }}", self.datcont())
    }
}
#[doc = "DLL (Delay Line) Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllCtrl(pub u32);
impl DllCtrl {
    #[doc = "DLL and delay chain"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL and delay chain"]
    #[inline(always)]
    pub const fn set_dll_ctrl_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL reset"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DLL reset"]
    #[inline(always)]
    pub const fn set_dll_ctrl_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_force_upd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DLL slave delay line"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_force_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DLL slave delay target0"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target0(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL slave delay target0"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_dly_target0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "DLL gate update"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_gate_update(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DLL gate update"]
    #[inline(always)]
    pub const fn set_dll_ctrl_gate_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DLL slave override"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DLL slave override"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DLL slave override val"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override_val(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "DLL slave override val"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_override_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "DLL slave delay target1"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DLL slave delay target1"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_dly_target1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Slave delay line update interval"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_update_int(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Slave delay line update interval"]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "DLL control loop update interval"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_ref_update_int(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL control loop update interval"]
    #[inline(always)]
    pub const fn set_dll_ctrl_ref_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for DllCtrl {
    #[inline(always)]
    fn default() -> DllCtrl {
        DllCtrl(0)
    }
}
impl core::fmt::Debug for DllCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DllCtrl")
            .field("dll_ctrl_enable", &self.dll_ctrl_enable())
            .field("dll_ctrl_reset", &self.dll_ctrl_reset())
            .field("dll_ctrl_slv_force_upd", &self.dll_ctrl_slv_force_upd())
            .field("dll_ctrl_slv_dly_target0", &self.dll_ctrl_slv_dly_target0())
            .field("dll_ctrl_gate_update", &self.dll_ctrl_gate_update())
            .field("dll_ctrl_slv_override", &self.dll_ctrl_slv_override())
            .field(
                "dll_ctrl_slv_override_val",
                &self.dll_ctrl_slv_override_val(),
            )
            .field("dll_ctrl_slv_dly_target1", &self.dll_ctrl_slv_dly_target1())
            .field("dll_ctrl_slv_update_int", &self.dll_ctrl_slv_update_int())
            .field("dll_ctrl_ref_update_int", &self.dll_ctrl_ref_update_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DllCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DllCtrl {{ dll_ctrl_enable: {=bool:?}, dll_ctrl_reset: {=bool:?}, dll_ctrl_slv_force_upd: {=bool:?}, dll_ctrl_slv_dly_target0: {=u8:?}, dll_ctrl_gate_update: {=bool:?}, dll_ctrl_slv_override: {=bool:?}, dll_ctrl_slv_override_val: {=u8:?}, dll_ctrl_slv_dly_target1: {=u8:?}, dll_ctrl_slv_update_int: {=u8:?}, dll_ctrl_ref_update_int: {=u8:?} }}",
            self.dll_ctrl_enable(),
            self.dll_ctrl_reset(),
            self.dll_ctrl_slv_force_upd(),
            self.dll_ctrl_slv_dly_target0(),
            self.dll_ctrl_gate_update(),
            self.dll_ctrl_slv_override(),
            self.dll_ctrl_slv_override_val(),
            self.dll_ctrl_slv_dly_target1(),
            self.dll_ctrl_slv_update_int(),
            self.dll_ctrl_ref_update_int()
        )
    }
}
#[doc = "DLL Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllStatus(pub u32);
impl DllStatus {
    #[doc = "Slave delay-line lock status"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_slv_lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay-line lock status"]
    #[inline(always)]
    pub const fn set_dll_sts_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reference DLL lock status"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_ref_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reference DLL lock status"]
    #[inline(always)]
    pub const fn set_dll_sts_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Slave delay line select status"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_slv_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave delay line select status"]
    #[inline(always)]
    pub const fn set_dll_sts_slv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
    #[doc = "Reference delay line select taps"]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_ref_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Reference delay line select taps"]
    #[inline(always)]
    pub const fn set_dll_sts_ref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
}
impl Default for DllStatus {
    #[inline(always)]
    fn default() -> DllStatus {
        DllStatus(0)
    }
}
impl core::fmt::Debug for DllStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DllStatus")
            .field("dll_sts_slv_lock", &self.dll_sts_slv_lock())
            .field("dll_sts_ref_lock", &self.dll_sts_ref_lock())
            .field("dll_sts_slv_sel", &self.dll_sts_slv_sel())
            .field("dll_sts_ref_sel", &self.dll_sts_ref_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DllStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DllStatus {{ dll_sts_slv_lock: {=bool:?}, dll_sts_ref_lock: {=bool:?}, dll_sts_slv_sel: {=u8:?}, dll_sts_ref_sel: {=u8:?} }}",
            self.dll_sts_slv_lock(),
            self.dll_sts_ref_lock(),
            self.dll_sts_slv_sel(),
            self.dll_sts_ref_sel()
        )
    }
}
#[doc = "DMA System Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsAddr(pub u32);
impl DsAddr {
    #[doc = "System address"]
    #[must_use]
    #[inline(always)]
    pub const fn ds_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "System address"]
    #[inline(always)]
    pub const fn set_ds_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DsAddr {
    #[inline(always)]
    fn default() -> DsAddr {
        DsAddr(0)
    }
}
impl core::fmt::Debug for DsAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DsAddr")
            .field("ds_addr", &self.ds_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DsAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DsAddr {{ ds_addr: {=u32:?} }}", self.ds_addr())
    }
}
#[doc = "Force Event"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceEvent(pub u32);
impl ForceEvent {
    #[doc = "Force event auto command 12 not executed"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 not executed"]
    #[inline(always)]
    pub const fn set_fevtac12ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Force event auto command 12 time out error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12toe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 time out error"]
    #[inline(always)]
    pub const fn set_fevtac12toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force event auto command 12 CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 CRC error"]
    #[inline(always)]
    pub const fn set_fevtac12ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force event Auto Command 12 end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ebe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 end bit error"]
    #[inline(always)]
    pub const fn set_fevtac12ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Force event Auto Command 12 index error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 index error"]
    #[inline(always)]
    pub const fn set_fevtac12ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force event command not executed by Auto Command 12 error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcnibac12e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command not executed by Auto Command 12 error"]
    #[inline(always)]
    pub const fn set_fevtcnibac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Force event command time out error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtctoe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command time out error"]
    #[inline(always)]
    pub const fn set_fevtctoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force event command CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcce(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command CRC error"]
    #[inline(always)]
    pub const fn set_fevtcce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force event command end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcebe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command end bit error"]
    #[inline(always)]
    pub const fn set_fevtcebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force event command index error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command index error"]
    #[inline(always)]
    pub const fn set_fevtcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Force event data time out error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdtoe(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data time out error"]
    #[inline(always)]
    pub const fn set_fevtdtoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Force event data CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdce(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data CRC error"]
    #[inline(always)]
    pub const fn set_fevtdce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Force event data end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdebe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data end bit error"]
    #[inline(always)]
    pub const fn set_fevtdebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Force event Auto Command 12 error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 error"]
    #[inline(always)]
    pub const fn set_fevtac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Force tuning error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevttne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Force tuning error"]
    #[inline(always)]
    pub const fn set_fevttne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Force event DMA error"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdmae(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Force event DMA error"]
    #[inline(always)]
    pub const fn set_fevtdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Force event card interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcint(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force event card interrupt"]
    #[inline(always)]
    pub const fn set_fevtcint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ForceEvent {
    #[inline(always)]
    fn default() -> ForceEvent {
        ForceEvent(0)
    }
}
impl core::fmt::Debug for ForceEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ForceEvent")
            .field("fevtac12ne", &self.fevtac12ne())
            .field("fevtac12toe", &self.fevtac12toe())
            .field("fevtac12ce", &self.fevtac12ce())
            .field("fevtac12ebe", &self.fevtac12ebe())
            .field("fevtac12ie", &self.fevtac12ie())
            .field("fevtcnibac12e", &self.fevtcnibac12e())
            .field("fevtctoe", &self.fevtctoe())
            .field("fevtcce", &self.fevtcce())
            .field("fevtcebe", &self.fevtcebe())
            .field("fevtcie", &self.fevtcie())
            .field("fevtdtoe", &self.fevtdtoe())
            .field("fevtdce", &self.fevtdce())
            .field("fevtdebe", &self.fevtdebe())
            .field("fevtac12e", &self.fevtac12e())
            .field("fevttne", &self.fevttne())
            .field("fevtdmae", &self.fevtdmae())
            .field("fevtcint", &self.fevtcint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ForceEvent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ForceEvent {{ fevtac12ne: {=bool:?}, fevtac12toe: {=bool:?}, fevtac12ce: {=bool:?}, fevtac12ebe: {=bool:?}, fevtac12ie: {=bool:?}, fevtcnibac12e: {=bool:?}, fevtctoe: {=bool:?}, fevtcce: {=bool:?}, fevtcebe: {=bool:?}, fevtcie: {=bool:?}, fevtdtoe: {=bool:?}, fevtdce: {=bool:?}, fevtdebe: {=bool:?}, fevtac12e: {=bool:?}, fevttne: {=bool:?}, fevtdmae: {=bool:?}, fevtcint: {=bool:?} }}",
            self.fevtac12ne(),
            self.fevtac12toe(),
            self.fevtac12ce(),
            self.fevtac12ebe(),
            self.fevtac12ie(),
            self.fevtcnibac12e(),
            self.fevtctoe(),
            self.fevtcce(),
            self.fevtcebe(),
            self.fevtcie(),
            self.fevtdtoe(),
            self.fevtdce(),
            self.fevtdebe(),
            self.fevtac12e(),
            self.fevttne(),
            self.fevtdmae(),
            self.fevtcint()
        )
    }
}
#[doc = "Host Controller Capabilities"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtrlCap(pub u32);
impl HostCtrlCap {
    #[doc = "SDR50 support"]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50_support(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDR50 support"]
    #[inline(always)]
    pub const fn set_sdr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDR104 support"]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104_support(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDR104 support"]
    #[inline(always)]
    pub const fn set_sdr104_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDR50 support"]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50_support(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDR50 support"]
    #[inline(always)]
    pub const fn set_ddr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Use Tuning for SDR50"]
    #[must_use]
    #[inline(always)]
    pub const fn use_tuning_sdr50(&self) -> super::vals::UseTuningSdr50 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::UseTuningSdr50::from_bits(val as u8)
    }
    #[doc = "Use Tuning for SDR50"]
    #[inline(always)]
    pub const fn set_use_tuning_sdr50(&mut self, val: super::vals::UseTuningSdr50) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Max block length"]
    #[must_use]
    #[inline(always)]
    pub const fn mbl(&self) -> super::vals::Mbl {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Mbl::from_bits(val as u8)
    }
    #[doc = "Max block length"]
    #[inline(always)]
    pub const fn set_mbl(&mut self, val: super::vals::Mbl) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "ADMA support"]
    #[must_use]
    #[inline(always)]
    pub const fn admas(&self) -> super::vals::Admas {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Admas::from_bits(val as u8)
    }
    #[doc = "ADMA support"]
    #[inline(always)]
    pub const fn set_admas(&mut self, val: super::vals::Admas) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High speed support"]
    #[must_use]
    #[inline(always)]
    pub const fn hss(&self) -> super::vals::Hss {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Hss::from_bits(val as u8)
    }
    #[doc = "High speed support"]
    #[inline(always)]
    pub const fn set_hss(&mut self, val: super::vals::Hss) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA support"]
    #[must_use]
    #[inline(always)]
    pub const fn dmas(&self) -> super::vals::Dmas {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmas::from_bits(val as u8)
    }
    #[doc = "DMA support"]
    #[inline(always)]
    pub const fn set_dmas(&mut self, val: super::vals::Dmas) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Suspend / resume support"]
    #[must_use]
    #[inline(always)]
    pub const fn srs(&self) -> super::vals::Srs {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Srs::from_bits(val as u8)
    }
    #[doc = "Suspend / resume support"]
    #[inline(always)]
    pub const fn set_srs(&mut self, val: super::vals::Srs) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Voltage support 3.3 V"]
    #[must_use]
    #[inline(always)]
    pub const fn vs33(&self) -> super::vals::Vs33 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Vs33::from_bits(val as u8)
    }
    #[doc = "Voltage support 3.3 V"]
    #[inline(always)]
    pub const fn set_vs33(&mut self, val: super::vals::Vs33) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Voltage support 3.0 V"]
    #[must_use]
    #[inline(always)]
    pub const fn vs30(&self) -> super::vals::Vs30 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Vs30::from_bits(val as u8)
    }
    #[doc = "Voltage support 3.0 V"]
    #[inline(always)]
    pub const fn set_vs30(&mut self, val: super::vals::Vs30) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Voltage support 1.8 V"]
    #[must_use]
    #[inline(always)]
    pub const fn vs18(&self) -> super::vals::Vs18 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Vs18::from_bits(val as u8)
    }
    #[doc = "Voltage support 1.8 V"]
    #[inline(always)]
    pub const fn set_vs18(&mut self, val: super::vals::Vs18) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for HostCtrlCap {
    #[inline(always)]
    fn default() -> HostCtrlCap {
        HostCtrlCap(0)
    }
}
impl core::fmt::Debug for HostCtrlCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HostCtrlCap")
            .field("sdr50_support", &self.sdr50_support())
            .field("sdr104_support", &self.sdr104_support())
            .field("ddr50_support", &self.ddr50_support())
            .field("use_tuning_sdr50", &self.use_tuning_sdr50())
            .field("mbl", &self.mbl())
            .field("admas", &self.admas())
            .field("hss", &self.hss())
            .field("dmas", &self.dmas())
            .field("srs", &self.srs())
            .field("vs33", &self.vs33())
            .field("vs30", &self.vs30())
            .field("vs18", &self.vs18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HostCtrlCap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HostCtrlCap {{ sdr50_support: {=bool:?}, sdr104_support: {=bool:?}, ddr50_support: {=bool:?}, use_tuning_sdr50: {:?}, mbl: {:?}, admas: {:?}, hss: {:?}, dmas: {:?}, srs: {:?}, vs33: {:?}, vs30: {:?}, vs18: {:?} }}",
            self.sdr50_support(),
            self.sdr104_support(),
            self.ddr50_support(),
            self.use_tuning_sdr50(),
            self.mbl(),
            self.admas(),
            self.hss(),
            self.dmas(),
            self.srs(),
            self.vs33(),
            self.vs30(),
            self.vs18()
        )
    }
}
#[doc = "Interrupt Signal Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSignalEn(pub u32);
impl IntSignalEn {
    #[doc = "Command complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccien(&self) -> super::vals::Ccien {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ccien::from_bits(val as u8)
    }
    #[doc = "Command complete interrupt enable"]
    #[inline(always)]
    pub const fn set_ccien(&mut self, val: super::vals::Ccien) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcien(&self) -> super::vals::Tcien {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcien::from_bits(val as u8)
    }
    #[doc = "Transfer complete interrupt enable"]
    #[inline(always)]
    pub const fn set_tcien(&mut self, val: super::vals::Tcien) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bgeien(&self) -> super::vals::Bgeien {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bgeien::from_bits(val as u8)
    }
    #[doc = "Block gap event interrupt enable"]
    #[inline(always)]
    pub const fn set_bgeien(&mut self, val: super::vals::Bgeien) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dintien(&self) -> super::vals::Dintien {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dintien::from_bits(val as u8)
    }
    #[doc = "DMA interrupt enable"]
    #[inline(always)]
    pub const fn set_dintien(&mut self, val: super::vals::Dintien) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bwrien(&self) -> super::vals::Bwrien {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwrien::from_bits(val as u8)
    }
    #[doc = "Buffer write ready interrupt enable"]
    #[inline(always)]
    pub const fn set_bwrien(&mut self, val: super::vals::Bwrien) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn brrien(&self) -> super::vals::Brrien {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brrien::from_bits(val as u8)
    }
    #[doc = "Buffer read ready interrupt enable"]
    #[inline(always)]
    pub const fn set_brrien(&mut self, val: super::vals::Brrien) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cinsien(&self) -> super::vals::Cinsien {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cinsien::from_bits(val as u8)
    }
    #[doc = "Card insertion interrupt enable"]
    #[inline(always)]
    pub const fn set_cinsien(&mut self, val: super::vals::Cinsien) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crmien(&self) -> super::vals::Crmien {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crmien::from_bits(val as u8)
    }
    #[doc = "Card removal interrupt enable"]
    #[inline(always)]
    pub const fn set_crmien(&mut self, val: super::vals::Crmien) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cintien(&self) -> super::vals::Cintien {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cintien::from_bits(val as u8)
    }
    #[doc = "Card interrupt enable"]
    #[inline(always)]
    pub const fn set_cintien(&mut self, val: super::vals::Cintien) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rteien(&self) -> super::vals::Rteien {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rteien::from_bits(val as u8)
    }
    #[doc = "Re-tuning event interrupt enable"]
    #[inline(always)]
    pub const fn set_rteien(&mut self, val: super::vals::Rteien) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning Pass interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpien(&self) -> super::vals::Tpien {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tpien::from_bits(val as u8)
    }
    #[doc = "Tuning Pass interrupt enable"]
    #[inline(always)]
    pub const fn set_tpien(&mut self, val: super::vals::Tpien) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command timeout error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctoeien(&self) -> super::vals::Ctoeien {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoeien::from_bits(val as u8)
    }
    #[doc = "Command timeout error interrupt enable"]
    #[inline(always)]
    pub const fn set_ctoeien(&mut self, val: super::vals::Ctoeien) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cceien(&self) -> super::vals::Cceien {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cceien::from_bits(val as u8)
    }
    #[doc = "Command CRC error interrupt enable"]
    #[inline(always)]
    pub const fn set_cceien(&mut self, val: super::vals::Cceien) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cebeien(&self) -> super::vals::Cebeien {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebeien::from_bits(val as u8)
    }
    #[doc = "Command end bit error interrupt enable"]
    #[inline(always)]
    pub const fn set_cebeien(&mut self, val: super::vals::Cebeien) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cieien(&self) -> super::vals::Cieien {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cieien::from_bits(val as u8)
    }
    #[doc = "Command index error interrupt enable"]
    #[inline(always)]
    pub const fn set_cieien(&mut self, val: super::vals::Cieien) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtoeien(&self) -> super::vals::Dtoeien {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoeien::from_bits(val as u8)
    }
    #[doc = "Data timeout error interrupt enable"]
    #[inline(always)]
    pub const fn set_dtoeien(&mut self, val: super::vals::Dtoeien) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dceien(&self) -> super::vals::Dceien {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dceien::from_bits(val as u8)
    }
    #[doc = "Data CRC error interrupt enable"]
    #[inline(always)]
    pub const fn set_dceien(&mut self, val: super::vals::Dceien) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn debeien(&self) -> super::vals::Debeien {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debeien::from_bits(val as u8)
    }
    #[doc = "Data end bit error interrupt enable"]
    #[inline(always)]
    pub const fn set_debeien(&mut self, val: super::vals::Debeien) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12eien(&self) -> super::vals::Ac12eien {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12eien::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error interrupt enable"]
    #[inline(always)]
    pub const fn set_ac12eien(&mut self, val: super::vals::Ac12eien) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tneien(&self) -> super::vals::Tneien {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Tneien::from_bits(val as u8)
    }
    #[doc = "Tuning error interrupt enable"]
    #[inline(always)]
    pub const fn set_tneien(&mut self, val: super::vals::Tneien) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaeien(&self) -> super::vals::Dmaeien {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmaeien::from_bits(val as u8)
    }
    #[doc = "DMA error interrupt enable"]
    #[inline(always)]
    pub const fn set_dmaeien(&mut self, val: super::vals::Dmaeien) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntSignalEn {
    #[inline(always)]
    fn default() -> IntSignalEn {
        IntSignalEn(0)
    }
}
impl core::fmt::Debug for IntSignalEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSignalEn")
            .field("ccien", &self.ccien())
            .field("tcien", &self.tcien())
            .field("bgeien", &self.bgeien())
            .field("dintien", &self.dintien())
            .field("bwrien", &self.bwrien())
            .field("brrien", &self.brrien())
            .field("cinsien", &self.cinsien())
            .field("crmien", &self.crmien())
            .field("cintien", &self.cintien())
            .field("rteien", &self.rteien())
            .field("tpien", &self.tpien())
            .field("ctoeien", &self.ctoeien())
            .field("cceien", &self.cceien())
            .field("cebeien", &self.cebeien())
            .field("cieien", &self.cieien())
            .field("dtoeien", &self.dtoeien())
            .field("dceien", &self.dceien())
            .field("debeien", &self.debeien())
            .field("ac12eien", &self.ac12eien())
            .field("tneien", &self.tneien())
            .field("dmaeien", &self.dmaeien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSignalEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSignalEn {{ ccien: {:?}, tcien: {:?}, bgeien: {:?}, dintien: {:?}, bwrien: {:?}, brrien: {:?}, cinsien: {:?}, crmien: {:?}, cintien: {:?}, rteien: {:?}, tpien: {:?}, ctoeien: {:?}, cceien: {:?}, cebeien: {:?}, cieien: {:?}, dtoeien: {:?}, dceien: {:?}, debeien: {:?}, ac12eien: {:?}, tneien: {:?}, dmaeien: {:?} }}",
            self.ccien(),
            self.tcien(),
            self.bgeien(),
            self.dintien(),
            self.bwrien(),
            self.brrien(),
            self.cinsien(),
            self.crmien(),
            self.cintien(),
            self.rteien(),
            self.tpien(),
            self.ctoeien(),
            self.cceien(),
            self.cebeien(),
            self.cieien(),
            self.dtoeien(),
            self.dceien(),
            self.debeien(),
            self.ac12eien(),
            self.tneien(),
            self.dmaeien()
        )
    }
}
#[doc = "Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Command complete"]
    #[must_use]
    #[inline(always)]
    pub const fn cc(&self) -> super::vals::Cc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cc::from_bits(val as u8)
    }
    #[doc = "Command complete"]
    #[inline(always)]
    pub const fn set_cc(&mut self, val: super::vals::Cc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete"]
    #[must_use]
    #[inline(always)]
    pub const fn tc(&self) -> super::vals::Tc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tc::from_bits(val as u8)
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: super::vals::Tc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event"]
    #[must_use]
    #[inline(always)]
    pub const fn bge(&self) -> super::vals::Bge {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bge::from_bits(val as u8)
    }
    #[doc = "Block gap event"]
    #[inline(always)]
    pub const fn set_bge(&mut self, val: super::vals::Bge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn dint(&self) -> super::vals::Dint {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dint::from_bits(val as u8)
    }
    #[doc = "DMA interrupt"]
    #[inline(always)]
    pub const fn set_dint(&mut self, val: super::vals::Dint) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready"]
    #[must_use]
    #[inline(always)]
    pub const fn bwr(&self) -> super::vals::Bwr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwr::from_bits(val as u8)
    }
    #[doc = "Buffer write ready"]
    #[inline(always)]
    pub const fn set_bwr(&mut self, val: super::vals::Bwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready"]
    #[must_use]
    #[inline(always)]
    pub const fn brr(&self) -> super::vals::Brr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brr::from_bits(val as u8)
    }
    #[doc = "Buffer read ready"]
    #[inline(always)]
    pub const fn set_brr(&mut self, val: super::vals::Brr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion"]
    #[must_use]
    #[inline(always)]
    pub const fn cins(&self) -> super::vals::Cins {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cins::from_bits(val as u8)
    }
    #[doc = "Card insertion"]
    #[inline(always)]
    pub const fn set_cins(&mut self, val: super::vals::Cins) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal"]
    #[must_use]
    #[inline(always)]
    pub const fn crm(&self) -> super::vals::Crm {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crm::from_bits(val as u8)
    }
    #[doc = "Card removal"]
    #[inline(always)]
    pub const fn set_crm(&mut self, val: super::vals::Crm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> super::vals::Cint {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cint::from_bits(val as u8)
    }
    #[doc = "Card interrupt"]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: super::vals::Cint) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn rte(&self) -> super::vals::Rte {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rte::from_bits(val as u8)
    }
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_rte(&mut self, val: super::vals::Rte) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn tp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_tp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn err_int_status(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Status"]
    #[inline(always)]
    pub const fn set_err_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Command timeout error"]
    #[must_use]
    #[inline(always)]
    pub const fn ctoe(&self) -> super::vals::Ctoe {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoe::from_bits(val as u8)
    }
    #[doc = "Command timeout error"]
    #[inline(always)]
    pub const fn set_ctoe(&mut self, val: super::vals::Ctoe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self) -> super::vals::Cce {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cce::from_bits(val as u8)
    }
    #[doc = "Command CRC error"]
    #[inline(always)]
    pub const fn set_cce(&mut self, val: super::vals::Cce) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn cebe(&self) -> super::vals::Cebe {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebe::from_bits(val as u8)
    }
    #[doc = "Command end bit error"]
    #[inline(always)]
    pub const fn set_cebe(&mut self, val: super::vals::Cebe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error"]
    #[must_use]
    #[inline(always)]
    pub const fn cie(&self) -> super::vals::Cie {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cie::from_bits(val as u8)
    }
    #[doc = "Command index error"]
    #[inline(always)]
    pub const fn set_cie(&mut self, val: super::vals::Cie) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error"]
    #[must_use]
    #[inline(always)]
    pub const fn dtoe(&self) -> super::vals::Dtoe {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoe::from_bits(val as u8)
    }
    #[doc = "Data timeout error"]
    #[inline(always)]
    pub const fn set_dtoe(&mut self, val: super::vals::Dtoe) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error"]
    #[must_use]
    #[inline(always)]
    pub const fn dce(&self) -> super::vals::Dce {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dce::from_bits(val as u8)
    }
    #[doc = "Data CRC error"]
    #[inline(always)]
    pub const fn set_dce(&mut self, val: super::vals::Dce) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error"]
    #[must_use]
    #[inline(always)]
    pub const fn debe(&self) -> super::vals::Debe {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debe::from_bits(val as u8)
    }
    #[doc = "Data end bit error"]
    #[inline(always)]
    pub const fn set_debe(&mut self, val: super::vals::Debe) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12e(&self) -> super::vals::Ac12e {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12e::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error"]
    #[inline(always)]
    pub const fn set_ac12e(&mut self, val: super::vals::Ac12e) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn tne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_tne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error"]
    #[must_use]
    #[inline(always)]
    pub const fn dmae(&self) -> super::vals::Dmae {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmae::from_bits(val as u8)
    }
    #[doc = "DMA error"]
    #[inline(always)]
    pub const fn set_dmae(&mut self, val: super::vals::Dmae) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("cc", &self.cc())
            .field("tc", &self.tc())
            .field("bge", &self.bge())
            .field("dint", &self.dint())
            .field("bwr", &self.bwr())
            .field("brr", &self.brr())
            .field("cins", &self.cins())
            .field("crm", &self.crm())
            .field("cint", &self.cint())
            .field("rte", &self.rte())
            .field("tp", &self.tp())
            .field("err_int_status", &self.err_int_status())
            .field("ctoe", &self.ctoe())
            .field("cce", &self.cce())
            .field("cebe", &self.cebe())
            .field("cie", &self.cie())
            .field("dtoe", &self.dtoe())
            .field("dce", &self.dce())
            .field("debe", &self.debe())
            .field("ac12e", &self.ac12e())
            .field("tne", &self.tne())
            .field("dmae", &self.dmae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ cc: {:?}, tc: {:?}, bge: {:?}, dint: {:?}, bwr: {:?}, brr: {:?}, cins: {:?}, crm: {:?}, cint: {:?}, rte: {:?}, tp: {=bool:?}, err_int_status: {=bool:?}, ctoe: {:?}, cce: {:?}, cebe: {:?}, cie: {:?}, dtoe: {:?}, dce: {:?}, debe: {:?}, ac12e: {:?}, tne: {=bool:?}, dmae: {:?} }}",
            self.cc(),
            self.tc(),
            self.bge(),
            self.dint(),
            self.bwr(),
            self.brr(),
            self.cins(),
            self.crm(),
            self.cint(),
            self.rte(),
            self.tp(),
            self.err_int_status(),
            self.ctoe(),
            self.cce(),
            self.cebe(),
            self.cie(),
            self.dtoe(),
            self.dce(),
            self.debe(),
            self.ac12e(),
            self.tne(),
            self.dmae()
        )
    }
}
#[doc = "Interrupt Status Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatusEn(pub u32);
impl IntStatusEn {
    #[doc = "Command complete status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccsen(&self) -> super::vals::Ccsen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ccsen::from_bits(val as u8)
    }
    #[doc = "Command complete status enable"]
    #[inline(always)]
    pub const fn set_ccsen(&mut self, val: super::vals::Ccsen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcsen(&self) -> super::vals::Tcsen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tcsen::from_bits(val as u8)
    }
    #[doc = "Transfer complete status enable"]
    #[inline(always)]
    pub const fn set_tcsen(&mut self, val: super::vals::Tcsen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bgesen(&self) -> super::vals::Bgesen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Bgesen::from_bits(val as u8)
    }
    #[doc = "Block gap event status enable"]
    #[inline(always)]
    pub const fn set_bgesen(&mut self, val: super::vals::Bgesen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dintsen(&self) -> super::vals::Dintsen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dintsen::from_bits(val as u8)
    }
    #[doc = "DMA interrupt status enable"]
    #[inline(always)]
    pub const fn set_dintsen(&mut self, val: super::vals::Dintsen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bwrsen(&self) -> super::vals::Bwrsen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Bwrsen::from_bits(val as u8)
    }
    #[doc = "Buffer write ready status enable"]
    #[inline(always)]
    pub const fn set_bwrsen(&mut self, val: super::vals::Bwrsen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn brrsen(&self) -> super::vals::Brrsen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Brrsen::from_bits(val as u8)
    }
    #[doc = "Buffer read ready status enable"]
    #[inline(always)]
    pub const fn set_brrsen(&mut self, val: super::vals::Brrsen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cinssen(&self) -> super::vals::Cinssen {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cinssen::from_bits(val as u8)
    }
    #[doc = "Card insertion status enable"]
    #[inline(always)]
    pub const fn set_cinssen(&mut self, val: super::vals::Cinssen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn crmsen(&self) -> super::vals::Crmsen {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Crmsen::from_bits(val as u8)
    }
    #[doc = "Card removal status enable"]
    #[inline(always)]
    pub const fn set_crmsen(&mut self, val: super::vals::Crmsen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cintsen(&self) -> super::vals::Cintsen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Cintsen::from_bits(val as u8)
    }
    #[doc = "Card interrupt status enable"]
    #[inline(always)]
    pub const fn set_cintsen(&mut self, val: super::vals::Cintsen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rtesen(&self) -> super::vals::Rtesen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rtesen::from_bits(val as u8)
    }
    #[doc = "Re-tuning event status enable"]
    #[inline(always)]
    pub const fn set_rtesen(&mut self, val: super::vals::Rtesen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning pass status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tpsen(&self) -> super::vals::Tpsen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Tpsen::from_bits(val as u8)
    }
    #[doc = "Tuning pass status enable"]
    #[inline(always)]
    pub const fn set_tpsen(&mut self, val: super::vals::Tpsen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command timeout error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ctoesen(&self) -> super::vals::Ctoesen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ctoesen::from_bits(val as u8)
    }
    #[doc = "Command timeout error status enable"]
    #[inline(always)]
    pub const fn set_ctoesen(&mut self, val: super::vals::Ctoesen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccesen(&self) -> super::vals::Ccesen {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Ccesen::from_bits(val as u8)
    }
    #[doc = "Command CRC error status enable"]
    #[inline(always)]
    pub const fn set_ccesen(&mut self, val: super::vals::Ccesen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cebesen(&self) -> super::vals::Cebesen {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cebesen::from_bits(val as u8)
    }
    #[doc = "Command end bit error status enable"]
    #[inline(always)]
    pub const fn set_cebesen(&mut self, val: super::vals::Cebesen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ciesen(&self) -> super::vals::Ciesen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Ciesen::from_bits(val as u8)
    }
    #[doc = "Command index error status enable"]
    #[inline(always)]
    pub const fn set_ciesen(&mut self, val: super::vals::Ciesen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtoesen(&self) -> super::vals::Dtoesen {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dtoesen::from_bits(val as u8)
    }
    #[doc = "Data timeout error status enable"]
    #[inline(always)]
    pub const fn set_dtoesen(&mut self, val: super::vals::Dtoesen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dcesen(&self) -> super::vals::Dcesen {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dcesen::from_bits(val as u8)
    }
    #[doc = "Data CRC error status enable"]
    #[inline(always)]
    pub const fn set_dcesen(&mut self, val: super::vals::Dcesen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn debesen(&self) -> super::vals::Debesen {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Debesen::from_bits(val as u8)
    }
    #[doc = "Data end bit error status enable"]
    #[inline(always)]
    pub const fn set_debesen(&mut self, val: super::vals::Debesen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12esen(&self) -> super::vals::Ac12esen {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Ac12esen::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error status enable"]
    #[inline(always)]
    pub const fn set_ac12esen(&mut self, val: super::vals::Ac12esen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tnesen(&self) -> super::vals::Tnesen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Tnesen::from_bits(val as u8)
    }
    #[doc = "Tuning error status enable"]
    #[inline(always)]
    pub const fn set_tnesen(&mut self, val: super::vals::Tnesen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error status enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaesen(&self) -> super::vals::Dmaesen {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmaesen::from_bits(val as u8)
    }
    #[doc = "DMA error status enable"]
    #[inline(always)]
    pub const fn set_dmaesen(&mut self, val: super::vals::Dmaesen) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatusEn {
    #[inline(always)]
    fn default() -> IntStatusEn {
        IntStatusEn(0)
    }
}
impl core::fmt::Debug for IntStatusEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatusEn")
            .field("ccsen", &self.ccsen())
            .field("tcsen", &self.tcsen())
            .field("bgesen", &self.bgesen())
            .field("dintsen", &self.dintsen())
            .field("bwrsen", &self.bwrsen())
            .field("brrsen", &self.brrsen())
            .field("cinssen", &self.cinssen())
            .field("crmsen", &self.crmsen())
            .field("cintsen", &self.cintsen())
            .field("rtesen", &self.rtesen())
            .field("tpsen", &self.tpsen())
            .field("ctoesen", &self.ctoesen())
            .field("ccesen", &self.ccesen())
            .field("cebesen", &self.cebesen())
            .field("ciesen", &self.ciesen())
            .field("dtoesen", &self.dtoesen())
            .field("dcesen", &self.dcesen())
            .field("debesen", &self.debesen())
            .field("ac12esen", &self.ac12esen())
            .field("tnesen", &self.tnesen())
            .field("dmaesen", &self.dmaesen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatusEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatusEn {{ ccsen: {:?}, tcsen: {:?}, bgesen: {:?}, dintsen: {:?}, bwrsen: {:?}, brrsen: {:?}, cinssen: {:?}, crmsen: {:?}, cintsen: {:?}, rtesen: {:?}, tpsen: {:?}, ctoesen: {:?}, ccesen: {:?}, cebesen: {:?}, ciesen: {:?}, dtoesen: {:?}, dcesen: {:?}, debesen: {:?}, ac12esen: {:?}, tnesen: {:?}, dmaesen: {:?} }}",
            self.ccsen(),
            self.tcsen(),
            self.bgesen(),
            self.dintsen(),
            self.bwrsen(),
            self.brrsen(),
            self.cinssen(),
            self.crmsen(),
            self.cintsen(),
            self.rtesen(),
            self.tpsen(),
            self.ctoesen(),
            self.ccesen(),
            self.cebesen(),
            self.ciesen(),
            self.dtoesen(),
            self.dcesen(),
            self.debesen(),
            self.ac12esen(),
            self.tnesen(),
            self.dmaesen()
        )
    }
}
#[doc = "Mixer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MixCtrl(pub u32);
impl MixCtrl {
    #[doc = "DMA enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Block count enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bcen(&self) -> super::vals::Bcen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Bcen::from_bits(val as u8)
    }
    #[doc = "Block count enable"]
    #[inline(always)]
    pub const fn set_bcen(&mut self, val: super::vals::Bcen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12en(&self) -> super::vals::Ac12en {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Ac12en::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 enable"]
    #[inline(always)]
    pub const fn set_ac12en(&mut self, val: super::vals::Ac12en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Dual data rate mode selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ddr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Dual data rate mode selection"]
    #[inline(always)]
    pub const fn set_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn dtdsel(&self) -> super::vals::Dtdsel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dtdsel::from_bits(val as u8)
    }
    #[doc = "Data transfer direction select"]
    #[inline(always)]
    pub const fn set_dtdsel(&mut self, val: super::vals::Dtdsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi / Single block select"]
    #[must_use]
    #[inline(always)]
    pub const fn msbsel(&self) -> super::vals::Msbsel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Msbsel::from_bits(val as u8)
    }
    #[doc = "Multi / Single block select"]
    #[inline(always)]
    pub const fn set_msbsel(&mut self, val: super::vals::Msbsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Nibble position indication"]
    #[must_use]
    #[inline(always)]
    pub const fn nibble_pos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Nibble position indication"]
    #[inline(always)]
    pub const fn set_nibble_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Auto CMD23 enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ac23en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD23 enable"]
    #[inline(always)]
    pub const fn set_ac23en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn exe_tune(&self) -> super::vals::ExeTune {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ExeTune::from_bits(val as u8)
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_exe_tune(&mut self, val: super::vals::ExeTune) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> super::vals::MixCtrlSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::MixCtrlSmpClkSel::from_bits(val as u8)
    }
    #[doc = "Clock selection"]
    #[inline(always)]
    pub const fn set_smp_clk_sel(&mut self, val: super::vals::MixCtrlSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode and and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn auto_tune_en(&self) -> super::vals::AutoTuneEn {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::AutoTuneEn::from_bits(val as u8)
    }
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode and and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_auto_tune_en(&mut self, val: super::vals::AutoTuneEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn fbclk_sel(&self) -> super::vals::FbclkSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::FbclkSel::from_bits(val as u8)
    }
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_fbclk_sel(&mut self, val: super::vals::FbclkSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for MixCtrl {
    #[inline(always)]
    fn default() -> MixCtrl {
        MixCtrl(0)
    }
}
impl core::fmt::Debug for MixCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MixCtrl")
            .field("dmaen", &self.dmaen())
            .field("bcen", &self.bcen())
            .field("ac12en", &self.ac12en())
            .field("ddr_en", &self.ddr_en())
            .field("dtdsel", &self.dtdsel())
            .field("msbsel", &self.msbsel())
            .field("nibble_pos", &self.nibble_pos())
            .field("ac23en", &self.ac23en())
            .field("exe_tune", &self.exe_tune())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .field("auto_tune_en", &self.auto_tune_en())
            .field("fbclk_sel", &self.fbclk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MixCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MixCtrl {{ dmaen: {:?}, bcen: {:?}, ac12en: {:?}, ddr_en: {=bool:?}, dtdsel: {:?}, msbsel: {:?}, nibble_pos: {=bool:?}, ac23en: {=bool:?}, exe_tune: {:?}, smp_clk_sel: {:?}, auto_tune_en: {:?}, fbclk_sel: {:?} }}",
            self.dmaen(),
            self.bcen(),
            self.ac12en(),
            self.ddr_en(),
            self.dtdsel(),
            self.msbsel(),
            self.nibble_pos(),
            self.ac23en(),
            self.exe_tune(),
            self.smp_clk_sel(),
            self.auto_tune_en(),
            self.fbclk_sel()
        )
    }
}
#[doc = "eMMC Boot"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmcBoot(pub u32);
impl MmcBoot {
    #[doc = "Boot ACK time out"]
    #[must_use]
    #[inline(always)]
    pub const fn dtocv_ack(&self) -> super::vals::DtocvAck {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DtocvAck::from_bits(val as u8)
    }
    #[doc = "Boot ACK time out"]
    #[inline(always)]
    pub const fn set_dtocv_ack(&mut self, val: super::vals::DtocvAck) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "BOOT ACK"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_ack(&self) -> super::vals::BootAck {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::BootAck::from_bits(val as u8)
    }
    #[doc = "BOOT ACK"]
    #[inline(always)]
    pub const fn set_boot_ack(&mut self, val: super::vals::BootAck) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Boot mode"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_mode(&self) -> super::vals::BootMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::BootMode::from_bits(val as u8)
    }
    #[doc = "Boot mode"]
    #[inline(always)]
    pub const fn set_boot_mode(&mut self, val: super::vals::BootMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Boot enable"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_en(&self) -> super::vals::BootEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::BootEn::from_bits(val as u8)
    }
    #[doc = "Boot enable"]
    #[inline(always)]
    pub const fn set_boot_en(&mut self, val: super::vals::BootEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Auto stop at block gap"]
    #[must_use]
    #[inline(always)]
    pub const fn auto_sabg_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto stop at block gap"]
    #[inline(always)]
    pub const fn set_auto_sabg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Time out"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_time_out(&self) -> super::vals::DisableTimeOut {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::DisableTimeOut::from_bits(val as u8)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub const fn set_disable_time_out(&mut self, val: super::vals::DisableTimeOut) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop At Block Gap value of automatic mode"]
    #[must_use]
    #[inline(always)]
    pub const fn boot_blk_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop At Block Gap value of automatic mode"]
    #[inline(always)]
    pub const fn set_boot_blk_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MmcBoot {
    #[inline(always)]
    fn default() -> MmcBoot {
        MmcBoot(0)
    }
}
impl core::fmt::Debug for MmcBoot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmcBoot")
            .field("dtocv_ack", &self.dtocv_ack())
            .field("boot_ack", &self.boot_ack())
            .field("boot_mode", &self.boot_mode())
            .field("boot_en", &self.boot_en())
            .field("auto_sabg_en", &self.auto_sabg_en())
            .field("disable_time_out", &self.disable_time_out())
            .field("boot_blk_cnt", &self.boot_blk_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmcBoot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MmcBoot {{ dtocv_ack: {:?}, boot_ack: {:?}, boot_mode: {:?}, boot_en: {:?}, auto_sabg_en: {=bool:?}, disable_time_out: {:?}, boot_blk_cnt: {=u16:?} }}",
            self.dtocv_ack(),
            self.boot_ack(),
            self.boot_mode(),
            self.boot_en(),
            self.auto_sabg_en(),
            self.disable_time_out(),
            self.boot_blk_cnt()
        )
    }
}
#[doc = "Present State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresState(pub u32);
impl PresState {
    #[doc = "Command inhibit (CMD)"]
    #[must_use]
    #[inline(always)]
    pub const fn cihb(&self) -> super::vals::Cihb {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cihb::from_bits(val as u8)
    }
    #[doc = "Command inhibit (CMD)"]
    #[inline(always)]
    pub const fn set_cihb(&mut self, val: super::vals::Cihb) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Inhibit Data (DATA)"]
    #[must_use]
    #[inline(always)]
    pub const fn cdihb(&self) -> super::vals::Cdihb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cdihb::from_bits(val as u8)
    }
    #[doc = "Command Inhibit Data (DATA)"]
    #[inline(always)]
    pub const fn set_cdihb(&mut self, val: super::vals::Cdihb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Data line active"]
    #[must_use]
    #[inline(always)]
    pub const fn dla(&self) -> super::vals::Dla {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dla::from_bits(val as u8)
    }
    #[doc = "Data line active"]
    #[inline(always)]
    pub const fn set_dla(&mut self, val: super::vals::Dla) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SD clock stable"]
    #[must_use]
    #[inline(always)]
    pub const fn sdstb(&self) -> super::vals::Sdstb {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Sdstb::from_bits(val as u8)
    }
    #[doc = "SD clock stable"]
    #[inline(always)]
    pub const fn set_sdstb(&mut self, val: super::vals::Sdstb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral clock gated off internally"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgoff(&self) -> super::vals::Ipgoff {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ipgoff::from_bits(val as u8)
    }
    #[doc = "Peripheral clock gated off internally"]
    #[inline(always)]
    pub const fn set_ipgoff(&mut self, val: super::vals::Ipgoff) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HCLK gated off internally"]
    #[must_use]
    #[inline(always)]
    pub const fn hckoff(&self) -> super::vals::Hckoff {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hckoff::from_bits(val as u8)
    }
    #[doc = "HCLK gated off internally"]
    #[inline(always)]
    pub const fn set_hckoff(&mut self, val: super::vals::Hckoff) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IPG_PERCLK gated off internally"]
    #[must_use]
    #[inline(always)]
    pub const fn peroff(&self) -> super::vals::Peroff {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Peroff::from_bits(val as u8)
    }
    #[doc = "IPG_PERCLK gated off internally"]
    #[inline(always)]
    pub const fn set_peroff(&mut self, val: super::vals::Peroff) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SD clock gated off internally"]
    #[must_use]
    #[inline(always)]
    pub const fn sdoff(&self) -> super::vals::Sdoff {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sdoff::from_bits(val as u8)
    }
    #[doc = "SD clock gated off internally"]
    #[inline(always)]
    pub const fn set_sdoff(&mut self, val: super::vals::Sdoff) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Write transfer active"]
    #[must_use]
    #[inline(always)]
    pub const fn wta(&self) -> super::vals::Wta {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wta::from_bits(val as u8)
    }
    #[doc = "Write transfer active"]
    #[inline(always)]
    pub const fn set_wta(&mut self, val: super::vals::Wta) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Read transfer active"]
    #[must_use]
    #[inline(always)]
    pub const fn rta(&self) -> super::vals::Rta {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rta::from_bits(val as u8)
    }
    #[doc = "Read transfer active"]
    #[inline(always)]
    pub const fn set_rta(&mut self, val: super::vals::Rta) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Buffer write enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bwen(&self) -> super::vals::Bwen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Bwen::from_bits(val as u8)
    }
    #[doc = "Buffer write enable"]
    #[inline(always)]
    pub const fn set_bwen(&mut self, val: super::vals::Bwen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Buffer read enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bren(&self) -> super::vals::Bren {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Bren::from_bits(val as u8)
    }
    #[doc = "Buffer read enable"]
    #[inline(always)]
    pub const fn set_bren(&mut self, val: super::vals::Bren) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode,and eMMC HS200 mode)"]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> super::vals::Rtr {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rtr::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode,and eMMC HS200 mode)"]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: super::vals::Rtr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tap select change done"]
    #[must_use]
    #[inline(always)]
    pub const fn tscd(&self) -> super::vals::Tscd {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Tscd::from_bits(val as u8)
    }
    #[doc = "Tap select change done"]
    #[inline(always)]
    pub const fn set_tscd(&mut self, val: super::vals::Tscd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Card inserted"]
    #[must_use]
    #[inline(always)]
    pub const fn cinst(&self) -> super::vals::Cinst {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cinst::from_bits(val as u8)
    }
    #[doc = "Card inserted"]
    #[inline(always)]
    pub const fn set_cinst(&mut self, val: super::vals::Cinst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "CMD line signal level"]
    #[must_use]
    #[inline(always)]
    pub const fn clsl(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CMD line signal level"]
    #[inline(always)]
    pub const fn set_clsl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DATA\\[7:0\\] line signal level"]
    #[must_use]
    #[inline(always)]
    pub const fn dlsl(&self) -> super::vals::Dlsl {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Dlsl::from_bits(val as u8)
    }
    #[doc = "DATA\\[7:0\\] line signal level"]
    #[inline(always)]
    pub const fn set_dlsl(&mut self, val: super::vals::Dlsl) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for PresState {
    #[inline(always)]
    fn default() -> PresState {
        PresState(0)
    }
}
impl core::fmt::Debug for PresState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PresState")
            .field("cihb", &self.cihb())
            .field("cdihb", &self.cdihb())
            .field("dla", &self.dla())
            .field("sdstb", &self.sdstb())
            .field("ipgoff", &self.ipgoff())
            .field("hckoff", &self.hckoff())
            .field("peroff", &self.peroff())
            .field("sdoff", &self.sdoff())
            .field("wta", &self.wta())
            .field("rta", &self.rta())
            .field("bwen", &self.bwen())
            .field("bren", &self.bren())
            .field("rtr", &self.rtr())
            .field("tscd", &self.tscd())
            .field("cinst", &self.cinst())
            .field("clsl", &self.clsl())
            .field("dlsl", &self.dlsl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PresState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PresState {{ cihb: {:?}, cdihb: {:?}, dla: {:?}, sdstb: {:?}, ipgoff: {:?}, hckoff: {:?}, peroff: {:?}, sdoff: {:?}, wta: {:?}, rta: {:?}, bwen: {:?}, bren: {:?}, rtr: {:?}, tscd: {:?}, cinst: {:?}, clsl: {=bool:?}, dlsl: {:?} }}",
            self.cihb(),
            self.cdihb(),
            self.dla(),
            self.sdstb(),
            self.ipgoff(),
            self.hckoff(),
            self.peroff(),
            self.sdoff(),
            self.wta(),
            self.rta(),
            self.bwen(),
            self.bren(),
            self.rtr(),
            self.tscd(),
            self.cinst(),
            self.clsl(),
            self.dlsl()
        )
    }
}
#[doc = "Protocol Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProtCtrl(pub u32);
impl ProtCtrl {
    #[doc = "Data transfer width"]
    #[must_use]
    #[inline(always)]
    pub const fn dtw(&self) -> super::vals::Dtw {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Dtw::from_bits(val as u8)
    }
    #[doc = "Data transfer width"]
    #[inline(always)]
    pub const fn set_dtw(&mut self, val: super::vals::Dtw) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "DATA3 as card detection pin"]
    #[must_use]
    #[inline(always)]
    pub const fn d3cd(&self) -> super::vals::D3cd {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::D3cd::from_bits(val as u8)
    }
    #[doc = "DATA3 as card detection pin"]
    #[inline(always)]
    pub const fn set_d3cd(&mut self, val: super::vals::D3cd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Endian mode"]
    #[must_use]
    #[inline(always)]
    pub const fn emode(&self) -> super::vals::Emode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Emode::from_bits(val as u8)
    }
    #[doc = "Endian mode"]
    #[inline(always)]
    pub const fn set_emode(&mut self, val: super::vals::Emode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA select"]
    #[must_use]
    #[inline(always)]
    pub const fn dmasel(&self) -> super::vals::Dmasel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Dmasel::from_bits(val as u8)
    }
    #[doc = "DMA select"]
    #[inline(always)]
    pub const fn set_dmasel(&mut self, val: super::vals::Dmasel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Stop at block gap request"]
    #[must_use]
    #[inline(always)]
    pub const fn sabgreq(&self) -> super::vals::Sabgreq {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Sabgreq::from_bits(val as u8)
    }
    #[doc = "Stop at block gap request"]
    #[inline(always)]
    pub const fn set_sabgreq(&mut self, val: super::vals::Sabgreq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Continue request"]
    #[must_use]
    #[inline(always)]
    pub const fn creq(&self) -> super::vals::Creq {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Creq::from_bits(val as u8)
    }
    #[doc = "Continue request"]
    #[inline(always)]
    pub const fn set_creq(&mut self, val: super::vals::Creq) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Read wait control"]
    #[must_use]
    #[inline(always)]
    pub const fn rwctl(&self) -> super::vals::Rwctl {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Rwctl::from_bits(val as u8)
    }
    #[doc = "Read wait control"]
    #[inline(always)]
    pub const fn set_rwctl(&mut self, val: super::vals::Rwctl) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt at block gap"]
    #[must_use]
    #[inline(always)]
    pub const fn iabg(&self) -> super::vals::Iabg {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Iabg::from_bits(val as u8)
    }
    #[doc = "Interrupt at block gap"]
    #[inline(always)]
    pub const fn set_iabg(&mut self, val: super::vals::Iabg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Read performed number 8 clock"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_done_no_8clk(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Read performed number 8 clock"]
    #[inline(always)]
    pub const fn set_rd_done_no_8clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wakeup event enable on card interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn wecint(&self) -> super::vals::Wecint {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Wecint::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on card interrupt"]
    #[inline(always)]
    pub const fn set_wecint(&mut self, val: super::vals::Wecint) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Wakeup event enable on SD card insertion"]
    #[must_use]
    #[inline(always)]
    pub const fn wecins(&self) -> super::vals::Wecins {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Wecins::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on SD card insertion"]
    #[inline(always)]
    pub const fn set_wecins(&mut self, val: super::vals::Wecins) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wakeup event enable on SD card removal"]
    #[must_use]
    #[inline(always)]
    pub const fn wecrm(&self) -> super::vals::Wecrm {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Wecrm::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on SD card removal"]
    #[inline(always)]
    pub const fn set_wecrm(&mut self, val: super::vals::Wecrm) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_en(&self) -> super::vals::BurstLenEn {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::BurstLenEn::from_bits(val as u8)
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP"]
    #[inline(always)]
    pub const fn set_burst_len_en(&mut self, val: super::vals::BurstLenEn) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Non-exact block read"]
    #[must_use]
    #[inline(always)]
    pub const fn non_exact_blk_rd(&self) -> super::vals::NonExactBlkRd {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::NonExactBlkRd::from_bits(val as u8)
    }
    #[doc = "Non-exact block read"]
    #[inline(always)]
    pub const fn set_non_exact_blk_rd(&mut self, val: super::vals::NonExactBlkRd) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for ProtCtrl {
    #[inline(always)]
    fn default() -> ProtCtrl {
        ProtCtrl(0)
    }
}
impl core::fmt::Debug for ProtCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ProtCtrl")
            .field("dtw", &self.dtw())
            .field("d3cd", &self.d3cd())
            .field("emode", &self.emode())
            .field("dmasel", &self.dmasel())
            .field("sabgreq", &self.sabgreq())
            .field("creq", &self.creq())
            .field("rwctl", &self.rwctl())
            .field("iabg", &self.iabg())
            .field("rd_done_no_8clk", &self.rd_done_no_8clk())
            .field("wecint", &self.wecint())
            .field("wecins", &self.wecins())
            .field("wecrm", &self.wecrm())
            .field("burst_len_en", &self.burst_len_en())
            .field("non_exact_blk_rd", &self.non_exact_blk_rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProtCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ProtCtrl {{ dtw: {:?}, d3cd: {:?}, emode: {:?}, dmasel: {:?}, sabgreq: {:?}, creq: {:?}, rwctl: {:?}, iabg: {:?}, rd_done_no_8clk: {=bool:?}, wecint: {:?}, wecins: {:?}, wecrm: {:?}, burst_len_en: {:?}, non_exact_blk_rd: {:?} }}",
            self.dtw(),
            self.d3cd(),
            self.emode(),
            self.dmasel(),
            self.sabgreq(),
            self.creq(),
            self.rwctl(),
            self.iabg(),
            self.rd_done_no_8clk(),
            self.wecint(),
            self.wecins(),
            self.wecrm(),
            self.burst_len_en(),
            self.non_exact_blk_rd()
        )
    }
}
#[doc = "System Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrl(pub u32);
impl SysCtrl {
    #[doc = "Divisor"]
    #[must_use]
    #[inline(always)]
    pub const fn dvs(&self) -> super::vals::Dvs {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Dvs::from_bits(val as u8)
    }
    #[doc = "Divisor"]
    #[inline(always)]
    pub const fn set_dvs(&mut self, val: super::vals::Dvs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "SDCLK frequency select"]
    #[must_use]
    #[inline(always)]
    pub const fn sdclkfs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SDCLK frequency select"]
    #[inline(always)]
    pub const fn set_sdclkfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data timeout counter value"]
    #[must_use]
    #[inline(always)]
    pub const fn dtocv(&self) -> super::vals::Dtocv {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Dtocv::from_bits(val as u8)
    }
    #[doc = "Data timeout counter value"]
    #[inline(always)]
    pub const fn set_dtocv(&mut self, val: super::vals::Dtocv) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Hardware reset"]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_rst_n(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware reset"]
    #[inline(always)]
    pub const fn set_ipp_rst_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Software reset for all"]
    #[must_use]
    #[inline(always)]
    pub const fn rsta(&self) -> super::vals::Rsta {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Rsta::from_bits(val as u8)
    }
    #[doc = "Software reset for all"]
    #[inline(always)]
    pub const fn set_rsta(&mut self, val: super::vals::Rsta) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Software reset for CMD line"]
    #[must_use]
    #[inline(always)]
    pub const fn rstc(&self) -> super::vals::Rstc {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Rstc::from_bits(val as u8)
    }
    #[doc = "Software reset for CMD line"]
    #[inline(always)]
    pub const fn set_rstc(&mut self, val: super::vals::Rstc) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Software reset for data line"]
    #[must_use]
    #[inline(always)]
    pub const fn rstd(&self) -> super::vals::Rstd {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Rstd::from_bits(val as u8)
    }
    #[doc = "Software reset for data line"]
    #[inline(always)]
    pub const fn set_rstd(&mut self, val: super::vals::Rstd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Initialization active"]
    #[must_use]
    #[inline(always)]
    pub const fn inita(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization active"]
    #[inline(always)]
    pub const fn set_inita(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Reset tuning"]
    #[must_use]
    #[inline(always)]
    pub const fn rstt(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Reset tuning"]
    #[inline(always)]
    pub const fn set_rstt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SysCtrl {
    #[inline(always)]
    fn default() -> SysCtrl {
        SysCtrl(0)
    }
}
impl core::fmt::Debug for SysCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysCtrl")
            .field("dvs", &self.dvs())
            .field("sdclkfs", &self.sdclkfs())
            .field("dtocv", &self.dtocv())
            .field("ipp_rst_n", &self.ipp_rst_n())
            .field("rsta", &self.rsta())
            .field("rstc", &self.rstc())
            .field("rstd", &self.rstd())
            .field("inita", &self.inita())
            .field("rstt", &self.rstt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SysCtrl {{ dvs: {:?}, sdclkfs: {=u8:?}, dtocv: {:?}, ipp_rst_n: {=bool:?}, rsta: {:?}, rstc: {:?}, rstd: {:?}, inita: {=bool:?}, rstt: {=bool:?} }}",
            self.dvs(),
            self.sdclkfs(),
            self.dtocv(),
            self.ipp_rst_n(),
            self.rsta(),
            self.rstc(),
            self.rstd(),
            self.inita(),
            self.rstt()
        )
    }
}
#[doc = "Tuning Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuningCtrl(pub u32);
impl TuningCtrl {
    #[doc = "Tuning start"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_start_tap(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Tuning start"]
    #[inline(always)]
    pub const fn set_tuning_start_tap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Disable command check for standard tuning"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_cmd_chk_for_std_tuning(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disable command check for standard tuning"]
    #[inline(always)]
    pub const fn set_dis_cmd_chk_for_std_tuning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tuning counter"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_counter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Tuning counter"]
    #[inline(always)]
    pub const fn set_tuning_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "TUNING_STEP"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_step(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "TUNING_STEP"]
    #[inline(always)]
    pub const fn set_tuning_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Data window"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_window(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Data window"]
    #[inline(always)]
    pub const fn set_tuning_window(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Standard tuning circuit and procedure enable"]
    #[must_use]
    #[inline(always)]
    pub const fn std_tuning_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Standard tuning circuit and procedure enable"]
    #[inline(always)]
    pub const fn set_std_tuning_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for TuningCtrl {
    #[inline(always)]
    fn default() -> TuningCtrl {
        TuningCtrl(0)
    }
}
impl core::fmt::Debug for TuningCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TuningCtrl")
            .field("tuning_start_tap", &self.tuning_start_tap())
            .field(
                "dis_cmd_chk_for_std_tuning",
                &self.dis_cmd_chk_for_std_tuning(),
            )
            .field("tuning_counter", &self.tuning_counter())
            .field("tuning_step", &self.tuning_step())
            .field("tuning_window", &self.tuning_window())
            .field("std_tuning_en", &self.std_tuning_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TuningCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TuningCtrl {{ tuning_start_tap: {=u8:?}, dis_cmd_chk_for_std_tuning: {=bool:?}, tuning_counter: {=u8:?}, tuning_step: {=u8:?}, tuning_window: {=u8:?}, std_tuning_en: {=bool:?} }}",
            self.tuning_start_tap(),
            self.dis_cmd_chk_for_std_tuning(),
            self.tuning_counter(),
            self.tuning_step(),
            self.tuning_window(),
            self.std_tuning_en()
        )
    }
}
#[doc = "Vendor Specific Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec(pub u32);
impl VendSpec {
    #[doc = "Voltage selection"]
    #[must_use]
    #[inline(always)]
    pub const fn vselect(&self) -> super::vals::Vselect {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Vselect::from_bits(val as u8)
    }
    #[doc = "Voltage selection"]
    #[inline(always)]
    pub const fn set_vselect(&mut self, val: super::vals::Vselect) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Conflict check enable"]
    #[must_use]
    #[inline(always)]
    pub const fn conflict_chk_en(&self) -> super::vals::ConflictChkEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::ConflictChkEn::from_bits(val as u8)
    }
    #[doc = "Conflict check enable"]
    #[inline(always)]
    pub const fn set_conflict_chk_en(&mut self, val: super::vals::ConflictChkEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Check busy enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ac12_wr_chkbusy_en(&self) -> super::vals::Ac12WrChkbusyEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Ac12WrChkbusyEn::from_bits(val as u8)
    }
    #[doc = "Check busy enable"]
    #[inline(always)]
    pub const fn set_ac12_wr_chkbusy_en(&mut self, val: super::vals::Ac12WrChkbusyEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Force CLK"]
    #[must_use]
    #[inline(always)]
    pub const fn frc_sdclk_on(&self) -> super::vals::FrcSdclkOn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::FrcSdclkOn::from_bits(val as u8)
    }
    #[doc = "Force CLK"]
    #[inline(always)]
    pub const fn set_frc_sdclk_on(&mut self, val: super::vals::FrcSdclkOn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CRC Check Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_chk_dis(&self) -> super::vals::CrcChkDis {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::CrcChkDis::from_bits(val as u8)
    }
    #[doc = "CRC Check Disable"]
    #[inline(always)]
    pub const fn set_crc_chk_dis(&mut self, val: super::vals::CrcChkDis) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Byte access"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_byte_en(&self) -> super::vals::CmdByteEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::CmdByteEn::from_bits(val as u8)
    }
    #[doc = "Byte access"]
    #[inline(always)]
    pub const fn set_cmd_byte_en(&mut self, val: super::vals::CmdByteEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for VendSpec {
    #[inline(always)]
    fn default() -> VendSpec {
        VendSpec(0)
    }
}
impl core::fmt::Debug for VendSpec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendSpec")
            .field("vselect", &self.vselect())
            .field("conflict_chk_en", &self.conflict_chk_en())
            .field("ac12_wr_chkbusy_en", &self.ac12_wr_chkbusy_en())
            .field("frc_sdclk_on", &self.frc_sdclk_on())
            .field("crc_chk_dis", &self.crc_chk_dis())
            .field("cmd_byte_en", &self.cmd_byte_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendSpec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendSpec {{ vselect: {:?}, conflict_chk_en: {:?}, ac12_wr_chkbusy_en: {:?}, frc_sdclk_on: {:?}, crc_chk_dis: {:?}, cmd_byte_en: {:?} }}",
            self.vselect(),
            self.conflict_chk_en(),
            self.ac12_wr_chkbusy_en(),
            self.frc_sdclk_on(),
            self.crc_chk_dis(),
            self.cmd_byte_en()
        )
    }
}
#[doc = "Vendor Specific 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec2(pub u32);
impl VendSpec2 {
    #[doc = "Card interrupt detection test"]
    #[must_use]
    #[inline(always)]
    pub const fn card_int_d3_test(&self) -> super::vals::CardIntD3Test {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::CardIntD3Test::from_bits(val as u8)
    }
    #[doc = "Card interrupt detection test"]
    #[inline(always)]
    pub const fn set_card_int_d3_test(&mut self, val: super::vals::CardIntD3Test) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Tuning 8bit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_8bit_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning 8bit enable"]
    #[inline(always)]
    pub const fn set_tuning_8bit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Tuning 1bit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_1bit_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning 1bit enable"]
    #[inline(always)]
    pub const fn set_tuning_1bit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Tuning command enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_cmd_en(&self) -> super::vals::TuningCmdEn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TuningCmdEn::from_bits(val as u8)
    }
    #[doc = "Tuning command enable"]
    #[inline(always)]
    pub const fn set_tuning_cmd_en(&mut self, val: super::vals::TuningCmdEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Argument2 register enable for ACMD23"]
    #[must_use]
    #[inline(always)]
    pub const fn acmd23_argu2_en(&self) -> super::vals::Acmd23Argu2En {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Acmd23Argu2En::from_bits(val as u8)
    }
    #[doc = "Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub const fn set_acmd23_argu2_en(&mut self, val: super::vals::Acmd23Argu2En) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
}
impl Default for VendSpec2 {
    #[inline(always)]
    fn default() -> VendSpec2 {
        VendSpec2(0)
    }
}
impl core::fmt::Debug for VendSpec2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendSpec2")
            .field("card_int_d3_test", &self.card_int_d3_test())
            .field("tuning_8bit_en", &self.tuning_8bit_en())
            .field("tuning_1bit_en", &self.tuning_1bit_en())
            .field("tuning_cmd_en", &self.tuning_cmd_en())
            .field("acmd23_argu2_en", &self.acmd23_argu2_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendSpec2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendSpec2 {{ card_int_d3_test: {:?}, tuning_8bit_en: {=bool:?}, tuning_1bit_en: {=bool:?}, tuning_cmd_en: {:?}, acmd23_argu2_en: {:?} }}",
            self.card_int_d3_test(),
            self.tuning_8bit_en(),
            self.tuning_1bit_en(),
            self.tuning_cmd_en(),
            self.acmd23_argu2_en()
        )
    }
}
#[doc = "Watermark Level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WtmkLvl(pub u32);
impl WtmkLvl {
    #[doc = "Read watermark level"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_wml(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read watermark level"]
    #[inline(always)]
    pub const fn set_rd_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read burst length due to system restriction, the actual burst length might not exceed 16"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_brst_len(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Read burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    pub const fn set_rd_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Write watermark level"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_wml(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write watermark level"]
    #[inline(always)]
    pub const fn set_wr_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Write burst length due to system restriction, the actual burst length might not exceed 16"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_brst_len(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Write burst length due to system restriction, the actual burst length might not exceed 16"]
    #[inline(always)]
    pub const fn set_wr_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for WtmkLvl {
    #[inline(always)]
    fn default() -> WtmkLvl {
        WtmkLvl(0)
    }
}
impl core::fmt::Debug for WtmkLvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WtmkLvl")
            .field("rd_wml", &self.rd_wml())
            .field("rd_brst_len", &self.rd_brst_len())
            .field("wr_wml", &self.wr_wml())
            .field("wr_brst_len", &self.wr_brst_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WtmkLvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WtmkLvl {{ rd_wml: {=u8:?}, rd_brst_len: {=u8:?}, wr_wml: {=u8:?}, wr_brst_len: {=u8:?} }}",
            self.rd_wml(),
            self.rd_brst_len(),
            self.wr_wml(),
            self.wr_brst_len()
        )
    }
}
