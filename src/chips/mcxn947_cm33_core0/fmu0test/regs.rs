#[doc = "ATX Pin Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AtxPinCtrl(pub u32);
impl AtxPinCtrl {
    #[doc = "TM to ATX"]
    #[must_use]
    #[inline(always)]
    pub const fn tm_to_atx(&self) -> super::vals::TmToAtx {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::TmToAtx::from_bits(val as u8)
    }
    #[doc = "TM to ATX"]
    #[inline(always)]
    pub const fn set_tm_to_atx(&mut self, val: super::vals::TmToAtx) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for AtxPinCtrl {
    #[inline(always)]
    fn default() -> AtxPinCtrl {
        AtxPinCtrl(0)
    }
}
impl core::fmt::Debug for AtxPinCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AtxPinCtrl")
            .field("tm_to_atx", &self.tm_to_atx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AtxPinCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtxPinCtrl {{ tm_to_atx: {:?} }}", self.tm_to_atx())
    }
}
#[doc = "BIST Datadump Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistDumpCtrl(pub u32);
impl BistDumpCtrl {
    #[doc = "BIST Done"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_done(&self) -> super::vals::BistDone {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::BistDone::from_bits(val as u8)
    }
    #[doc = "BIST Done"]
    #[inline(always)]
    pub const fn set_bist_done(&mut self, val: super::vals::BistDone) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "BIST Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_fail(&self) -> super::vals::BistFail {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::BistFail::from_bits(val as u8)
    }
    #[doc = "BIST Fail"]
    #[inline(always)]
    pub const fn set_bist_fail(&mut self, val: super::vals::BistFail) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Data Dump Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn datadump(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Data Dump Enable"]
    #[inline(always)]
    pub const fn set_datadump(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Data Dump Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_trig(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Data Dump Trigger"]
    #[inline(always)]
    pub const fn set_datadump_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Dump Pattern Select"]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_patt(&self) -> super::vals::DatadumpPatt {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::DatadumpPatt::from_bits(val as u8)
    }
    #[doc = "Data Dump Pattern Select"]
    #[inline(always)]
    pub const fn set_datadump_patt(&mut self, val: super::vals::DatadumpPatt) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Data Dump Margin Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_mrgen(&self) -> super::vals::DatadumpMrgen {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::DatadumpMrgen::from_bits(val as u8)
    }
    #[doc = "Data Dump Margin Enable"]
    #[inline(always)]
    pub const fn set_datadump_mrgen(&mut self, val: super::vals::DatadumpMrgen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Data Dump Margin Type"]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_mrgtype(&self) -> super::vals::DatadumpMrgtype {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::DatadumpMrgtype::from_bits(val as u8)
    }
    #[doc = "Data Dump Margin Type"]
    #[inline(always)]
    pub const fn set_datadump_mrgtype(&mut self, val: super::vals::DatadumpMrgtype) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for BistDumpCtrl {
    #[inline(always)]
    fn default() -> BistDumpCtrl {
        BistDumpCtrl(0)
    }
}
impl core::fmt::Debug for BistDumpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BistDumpCtrl")
            .field("bist_done", &self.bist_done())
            .field("bist_fail", &self.bist_fail())
            .field("datadump", &self.datadump())
            .field("datadump_trig", &self.datadump_trig())
            .field("datadump_patt", &self.datadump_patt())
            .field("datadump_mrgen", &self.datadump_mrgen())
            .field("datadump_mrgtype", &self.datadump_mrgtype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BistDumpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BistDumpCtrl {{ bist_done: {:?}, bist_fail: {:?}, datadump: {=bool:?}, datadump_trig: {=bool:?}, datadump_patt: {:?}, datadump_mrgen: {:?}, datadump_mrgtype: {:?} }}",
            self.bist_done(),
            self.bist_fail(),
            self.datadump(),
            self.datadump_trig(),
            self.datadump_patt(),
            self.datadump_mrgen(),
            self.datadump_mrgtype()
        )
    }
}
#[doc = "FMU Block Select Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsel(pub u32);
impl Bsel {
    #[doc = "Slave Block Select"]
    #[must_use]
    #[inline(always)]
    pub const fn sbsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Slave Block Select"]
    #[inline(always)]
    pub const fn set_sbsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Master Block Select"]
    #[must_use]
    #[inline(always)]
    pub const fn mbsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Master Block Select"]
    #[inline(always)]
    pub const fn set_mbsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Bsel {
    #[inline(always)]
    fn default() -> Bsel {
        Bsel(0)
    }
}
impl core::fmt::Debug for Bsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bsel")
            .field("sbsel", &self.sbsel())
            .field("mbsel", &self.mbsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bsel {{ sbsel: {=u8:?}, mbsel: {=u8:?} }}",
            self.sbsel(),
            self.mbsel()
        )
    }
}
#[doc = "FMU Block Select Generation Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BselGen(pub u32);
impl BselGen {
    #[doc = "Generated SBSEL"]
    #[must_use]
    #[inline(always)]
    pub const fn sbsel_gen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Generated SBSEL"]
    #[inline(always)]
    pub const fn set_sbsel_gen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Generated MBSEL"]
    #[must_use]
    #[inline(always)]
    pub const fn mbsel_gen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Generated MBSEL"]
    #[inline(always)]
    pub const fn set_mbsel_gen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for BselGen {
    #[inline(always)]
    fn default() -> BselGen {
        BselGen(0)
    }
}
impl core::fmt::Debug for BselGen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BselGen")
            .field("sbsel_gen", &self.sbsel_gen())
            .field("mbsel_gen", &self.mbsel_gen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BselGen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BselGen {{ sbsel_gen: {=u8:?}, mbsel_gen: {=u8:?} }}",
            self.sbsel_gen(),
            self.mbsel_gen()
        )
    }
}
#[doc = "FMU Command Check Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdCheck(pub u32);
impl CmdCheck {
    #[doc = "Phrase Alignment Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_phr(&self) -> super::vals::AlignfailPhr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AlignfailPhr::from_bits(val as u8)
    }
    #[doc = "Phrase Alignment Fail"]
    #[inline(always)]
    pub const fn set_alignfail_phr(&mut self, val: super::vals::AlignfailPhr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Page Alignment Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_pg(&self) -> super::vals::AlignfailPg {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AlignfailPg::from_bits(val as u8)
    }
    #[doc = "Page Alignment Fail"]
    #[inline(always)]
    pub const fn set_alignfail_pg(&mut self, val: super::vals::AlignfailPg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sector Alignment Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_scr(&self) -> super::vals::AlignfailScr {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AlignfailScr::from_bits(val as u8)
    }
    #[doc = "Sector Alignment Fail"]
    #[inline(always)]
    pub const fn set_alignfail_scr(&mut self, val: super::vals::AlignfailScr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Block Alignment Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_blk(&self) -> super::vals::AlignfailBlk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::AlignfailBlk::from_bits(val as u8)
    }
    #[doc = "Block Alignment Fail"]
    #[inline(always)]
    pub const fn set_alignfail_blk(&mut self, val: super::vals::AlignfailBlk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Address Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_fail(&self) -> super::vals::AddrFail {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AddrFail::from_bits(val as u8)
    }
    #[doc = "Address Fail"]
    #[inline(always)]
    pub const fn set_addr_fail(&mut self, val: super::vals::AddrFail) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IFR Command"]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_cmd(&self) -> super::vals::IfrCmd {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IfrCmd::from_bits(val as u8)
    }
    #[doc = "IFR Command"]
    #[inline(always)]
    pub const fn set_ifr_cmd(&mut self, val: super::vals::IfrCmd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "All Blocks Command"]
    #[must_use]
    #[inline(always)]
    pub const fn all_cmd(&self) -> super::vals::AllCmd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::AllCmd::from_bits(val as u8)
    }
    #[doc = "All Blocks Command"]
    #[inline(always)]
    pub const fn set_all_cmd(&mut self, val: super::vals::AllCmd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Address Range Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn range_fail(&self) -> super::vals::RangeFail {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::RangeFail::from_bits(val as u8)
    }
    #[doc = "Address Range Fail"]
    #[inline(always)]
    pub const fn set_range_fail(&mut self, val: super::vals::RangeFail) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sector Alignment Check"]
    #[must_use]
    #[inline(always)]
    pub const fn scr_align_chk(&self) -> super::vals::ScrAlignChk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::ScrAlignChk::from_bits(val as u8)
    }
    #[doc = "Sector Alignment Check"]
    #[inline(always)]
    pub const fn set_scr_align_chk(&mut self, val: super::vals::ScrAlignChk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Option Check Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn option_fail(&self) -> super::vals::OptionFail {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::OptionFail::from_bits(val as u8)
    }
    #[doc = "Option Check Fail"]
    #[inline(always)]
    pub const fn set_option_fail(&mut self, val: super::vals::OptionFail) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Illegal Command"]
    #[must_use]
    #[inline(always)]
    pub const fn illegal_cmd(&self) -> super::vals::IllegalCmd {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::IllegalCmd::from_bits(val as u8)
    }
    #[doc = "Illegal Command"]
    #[inline(always)]
    pub const fn set_illegal_cmd(&mut self, val: super::vals::IllegalCmd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for CmdCheck {
    #[inline(always)]
    fn default() -> CmdCheck {
        CmdCheck(0)
    }
}
impl core::fmt::Debug for CmdCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdCheck")
            .field("alignfail_phr", &self.alignfail_phr())
            .field("alignfail_pg", &self.alignfail_pg())
            .field("alignfail_scr", &self.alignfail_scr())
            .field("alignfail_blk", &self.alignfail_blk())
            .field("addr_fail", &self.addr_fail())
            .field("ifr_cmd", &self.ifr_cmd())
            .field("all_cmd", &self.all_cmd())
            .field("range_fail", &self.range_fail())
            .field("scr_align_chk", &self.scr_align_chk())
            .field("option_fail", &self.option_fail())
            .field("illegal_cmd", &self.illegal_cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdCheck {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmdCheck {{ alignfail_phr: {:?}, alignfail_pg: {:?}, alignfail_scr: {:?}, alignfail_blk: {:?}, addr_fail: {:?}, ifr_cmd: {:?}, all_cmd: {:?}, range_fail: {:?}, scr_align_chk: {:?}, option_fail: {:?}, illegal_cmd: {:?} }}",
            self.alignfail_phr(),
            self.alignfail_pg(),
            self.alignfail_scr(),
            self.alignfail_blk(),
            self.addr_fail(),
            self.ifr_cmd(),
            self.all_cmd(),
            self.range_fail(),
            self.scr_align_chk(),
            self.option_fail(),
            self.illegal_cmd()
        )
    }
}
#[doc = "Erase Pulse Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErsPulseCnt(pub u32);
impl ErsPulseCnt {
    #[doc = "Block 0 Erase Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ers_cnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block 0 Erase Pulse Count"]
    #[inline(always)]
    pub const fn set_ers_cnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Block 1 Erase Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn ers_cnt1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Block 1 Erase Pulse Count"]
    #[inline(always)]
    pub const fn set_ers_cnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ErsPulseCnt {
    #[inline(always)]
    fn default() -> ErsPulseCnt {
        ErsPulseCnt(0)
    }
}
impl core::fmt::Debug for ErsPulseCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErsPulseCnt")
            .field("ers_cnt0", &self.ers_cnt0())
            .field("ers_cnt1", &self.ers_cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErsPulseCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ErsPulseCnt {{ ers_cnt0: {=u16:?}, ers_cnt1: {=u16:?} }}",
            self.ers_cnt0(),
            self.ers_cnt1()
        )
    }
}
#[doc = "Fail Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Failcnt(pub u32);
impl Failcnt {
    #[doc = "Fail Count"]
    #[must_use]
    #[inline(always)]
    pub const fn failcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fail Count"]
    #[inline(always)]
    pub const fn set_failcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Failcnt {
    #[inline(always)]
    fn default() -> Failcnt {
        Failcnt(0)
    }
}
impl core::fmt::Debug for Failcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Failcnt")
            .field("failcnt", &self.failcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Failcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Failcnt {{ failcnt: {=u32:?} }}", self.failcnt())
    }
}
#[doc = "Flash Command Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob0(pub u32);
impl Fccob0 {
    #[doc = "Command code"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command code"]
    #[inline(always)]
    pub const fn set_cmdcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fccob0 {
    #[inline(always)]
    fn default() -> Fccob0 {
        Fccob0(0)
    }
}
impl core::fmt::Debug for Fccob0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob0")
            .field("cmdcode", &self.cmdcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob0 {{ cmdcode: {=u8:?} }}", self.cmdcode())
    }
}
#[doc = "Flash Command Control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob1(pub u32);
impl Fccob1 {
    #[doc = "Command options"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdopt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command options"]
    #[inline(always)]
    pub const fn set_cmdopt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fccob1 {
    #[inline(always)]
    fn default() -> Fccob1 {
        Fccob1(0)
    }
}
impl core::fmt::Debug for Fccob1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob1")
            .field("cmdopt", &self.cmdopt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob1 {{ cmdopt: {=u8:?} }}", self.cmdopt())
    }
}
#[doc = "Flash Command Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob2(pub u32);
impl Fccob2 {
    #[doc = "Command starting address"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command starting address"]
    #[inline(always)]
    pub const fn set_cmdaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob2 {
    #[inline(always)]
    fn default() -> Fccob2 {
        Fccob2(0)
    }
}
impl core::fmt::Debug for Fccob2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob2")
            .field("cmdaddr", &self.cmdaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob2 {{ cmdaddr: {=u32:?} }}", self.cmdaddr())
    }
}
#[doc = "Flash Command Control 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob3(pub u32);
impl Fccob3 {
    #[doc = "Command ending address"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdaddre(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command ending address"]
    #[inline(always)]
    pub const fn set_cmdaddre(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob3 {
    #[inline(always)]
    fn default() -> Fccob3 {
        Fccob3(0)
    }
}
impl core::fmt::Debug for Fccob3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob3")
            .field("cmdaddre", &self.cmdaddre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob3 {{ cmdaddre: {=u32:?} }}", self.cmdaddre())
    }
}
#[doc = "Flash Command Control 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob4(pub u32);
impl Fccob4 {
    #[doc = "Command data word 0"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 0"]
    #[inline(always)]
    pub const fn set_cmddata0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob4 {
    #[inline(always)]
    fn default() -> Fccob4 {
        Fccob4(0)
    }
}
impl core::fmt::Debug for Fccob4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob4")
            .field("cmddata0", &self.cmddata0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob4 {{ cmddata0: {=u32:?} }}", self.cmddata0())
    }
}
#[doc = "Flash Command Control 5 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob5(pub u32);
impl Fccob5 {
    #[doc = "Command data word 1"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 1"]
    #[inline(always)]
    pub const fn set_cmddata1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob5 {
    #[inline(always)]
    fn default() -> Fccob5 {
        Fccob5(0)
    }
}
impl core::fmt::Debug for Fccob5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob5")
            .field("cmddata1", &self.cmddata1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob5 {{ cmddata1: {=u32:?} }}", self.cmddata1())
    }
}
#[doc = "Flash Command Control 6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob6(pub u32);
impl Fccob6 {
    #[doc = "Command data word 2"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 2"]
    #[inline(always)]
    pub const fn set_cmddata2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob6 {
    #[inline(always)]
    fn default() -> Fccob6 {
        Fccob6(0)
    }
}
impl core::fmt::Debug for Fccob6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob6")
            .field("cmddata2", &self.cmddata2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob6 {{ cmddata2: {=u32:?} }}", self.cmddata2())
    }
}
#[doc = "Flash Command Control 7 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob7(pub u32);
impl Fccob7 {
    #[doc = "Command data word 3"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 3"]
    #[inline(always)]
    pub const fn set_cmddata3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob7 {
    #[inline(always)]
    fn default() -> Fccob7 {
        Fccob7(0)
    }
}
impl core::fmt::Debug for Fccob7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob7")
            .field("cmddata3", &self.cmddata3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob7 {{ cmddata3: {=u32:?} }}", self.cmddata3())
    }
}
#[doc = "Flash Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcnfg(pub u32);
impl Fcnfg {
    #[doc = "Command Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ccie(&self) -> super::vals::Ccie {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ccie::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ccie(&mut self, val: super::vals::Ccie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mass Erase (Erase All) Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ersreq(&self) -> super::vals::Ersreq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ersreq::from_bits(val as u8)
    }
    #[doc = "Mass Erase (Erase All) Request"]
    #[inline(always)]
    pub const fn set_ersreq(&mut self, val: super::vals::Ersreq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dfdie(&self) -> super::vals::Dfdie {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dfdie::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dfdie(&mut self, val: super::vals::Dfdie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ersien0(&self) -> super::vals::Ersien0 {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::Ersien0::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 0"]
    #[inline(always)]
    pub const fn set_ersien0(&mut self, val: super::vals::Ersien0) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)"]
    #[must_use]
    #[inline(always)]
    pub const fn ersien1(&self) -> super::vals::Ersien1 {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ersien1::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)"]
    #[inline(always)]
    pub const fn set_ersien1(&mut self, val: super::vals::Ersien1) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Fcnfg {
    #[inline(always)]
    fn default() -> Fcnfg {
        Fcnfg(0)
    }
}
impl core::fmt::Debug for Fcnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcnfg")
            .field("ccie", &self.ccie())
            .field("ersreq", &self.ersreq())
            .field("dfdie", &self.dfdie())
            .field("ersien0", &self.ersien0())
            .field("ersien1", &self.ersien1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcnfg {{ ccie: {:?}, ersreq: {:?}, dfdie: {:?}, ersien0: {:?}, ersien1: {:?} }}",
            self.ccie(),
            self.ersreq(),
            self.dfdie(),
            self.ersien0(),
            self.ersien1()
        )
    }
}
#[doc = "Flash Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Read Wait-State Control"]
    #[must_use]
    #[inline(always)]
    pub const fn rwsc(&self) -> super::vals::Rwsc {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Rwsc::from_bits(val as u8)
    }
    #[doc = "Read Wait-State Control"]
    #[inline(always)]
    pub const fn set_rwsc(&mut self, val: super::vals::Rwsc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Low Speed Active Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lsactive(&self) -> super::vals::Lsactive {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lsactive::from_bits(val as u8)
    }
    #[doc = "Low Speed Active Mode"]
    #[inline(always)]
    pub const fn set_lsactive(&mut self, val: super::vals::Lsactive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Force Double Bit Fault Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn fdfd(&self) -> super::vals::Fdfd {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fdfd::from_bits(val as u8)
    }
    #[doc = "Force Double Bit Fault Detect"]
    #[inline(always)]
    pub const fn set_fdfd(&mut self, val: super::vals::Fdfd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Abort Request"]
    #[must_use]
    #[inline(always)]
    pub const fn abtreq(&self) -> super::vals::Abtreq {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Abtreq::from_bits(val as u8)
    }
    #[doc = "Abort Request"]
    #[inline(always)]
    pub const fn set_abtreq(&mut self, val: super::vals::Abtreq) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        Fctrl(0)
    }
}
impl core::fmt::Debug for Fctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl")
            .field("rwsc", &self.rwsc())
            .field("lsactive", &self.lsactive())
            .field("fdfd", &self.fdfd())
            .field("abtreq", &self.abtreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl {{ rwsc: {:?}, lsactive: {:?}, fdfd: {:?}, abtreq: {:?} }}",
            self.rwsc(),
            self.lsactive(),
            self.fdfd(),
            self.abtreq()
        )
    }
}
#[doc = "Flash Read Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashRdAdd(pub u32);
impl FlashRdAdd {
    #[doc = "Flash Read Address"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rd_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Flash Read Address"]
    #[inline(always)]
    pub const fn set_flash_rd_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlashRdAdd {
    #[inline(always)]
    fn default() -> FlashRdAdd {
        FlashRdAdd(0)
    }
}
impl core::fmt::Debug for FlashRdAdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashRdAdd")
            .field("flash_rd_add", &self.flash_rd_add())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashRdAdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashRdAdd {{ flash_rd_add: {=u32:?} }}",
            self.flash_rd_add()
        )
    }
}
#[doc = "Flash Read Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashRdCtrl(pub u32);
impl FlashRdCtrl {
    #[doc = "Flash Read Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rd(&self) -> super::vals::FlashRd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::FlashRd::from_bits(val as u8)
    }
    #[doc = "Flash Read Enable"]
    #[inline(always)]
    pub const fn set_flash_rd(&mut self, val: super::vals::FlashRd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wide Load Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wide_load(&self) -> super::vals::WideLoad {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::WideLoad::from_bits(val as u8)
    }
    #[doc = "Wide Load Enable"]
    #[inline(always)]
    pub const fn set_wide_load(&mut self, val: super::vals::WideLoad) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Flash Read"]
    #[must_use]
    #[inline(always)]
    pub const fn single_rd(&self) -> super::vals::SingleRd {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SingleRd::from_bits(val as u8)
    }
    #[doc = "Single Flash Read"]
    #[inline(always)]
    pub const fn set_single_rd(&mut self, val: super::vals::SingleRd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for FlashRdCtrl {
    #[inline(always)]
    fn default() -> FlashRdCtrl {
        FlashRdCtrl(0)
    }
}
impl core::fmt::Debug for FlashRdCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashRdCtrl")
            .field("flash_rd", &self.flash_rd())
            .field("wide_load", &self.wide_load())
            .field("single_rd", &self.single_rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashRdCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashRdCtrl {{ flash_rd: {:?}, wide_load: {:?}, single_rd: {:?} }}",
            self.flash_rd(),
            self.wide_load(),
            self.single_rd()
        )
    }
}
#[doc = "Flash Stop Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashStopAdd(pub u32);
impl FlashStopAdd {
    #[doc = "Flash Stop Address"]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stop_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Flash Stop Address"]
    #[inline(always)]
    pub const fn set_flash_stop_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlashStopAdd {
    #[inline(always)]
    fn default() -> FlashStopAdd {
        FlashStopAdd(0)
    }
}
impl core::fmt::Debug for FlashStopAdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashStopAdd")
            .field("flash_stop_add", &self.flash_stop_add())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashStopAdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashStopAdd {{ flash_stop_add: {=u32:?} }}",
            self.flash_stop_add()
        )
    }
}
#[doc = "Flash Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstat(pub u32);
impl Fstat {
    #[doc = "Command Fail Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> super::vals::Fail {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fail::from_bits(val as u8)
    }
    #[doc = "Command Fail Flag"]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: super::vals::Fail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Abort Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdabt(&self) -> super::vals::Cmdabt {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cmdabt::from_bits(val as u8)
    }
    #[doc = "Command Abort Flag"]
    #[inline(always)]
    pub const fn set_cmdabt(&mut self, val: super::vals::Cmdabt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Command Protection Violation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pviol(&self) -> super::vals::Pviol {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pviol::from_bits(val as u8)
    }
    #[doc = "Command Protection Violation Flag"]
    #[inline(always)]
    pub const fn set_pviol(&mut self, val: super::vals::Pviol) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command Access Error Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn accerr(&self) -> super::vals::Accerr {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Accerr::from_bits(val as u8)
    }
    #[doc = "Command Access Error Flag"]
    #[inline(always)]
    pub const fn set_accerr(&mut self, val: super::vals::Accerr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Command Write Sequence Abort Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cwsabt(&self) -> super::vals::Cwsabt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cwsabt::from_bits(val as u8)
    }
    #[doc = "Command Write Sequence Abort Flag"]
    #[inline(always)]
    pub const fn set_cwsabt(&mut self, val: super::vals::Cwsabt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Command Complete Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ccif(&self) -> super::vals::Ccif {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Ccif::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Flag"]
    #[inline(always)]
    pub const fn set_ccif(&mut self, val: super::vals::Ccif) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Protection Level"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdprt(&self) -> super::vals::Cmdprt {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Cmdprt::from_bits(val as u8)
    }
    #[doc = "Command Protection Level"]
    #[inline(always)]
    pub const fn set_cmdprt(&mut self, val: super::vals::Cmdprt) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Command Protection Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdp(&self) -> super::vals::Cmdp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmdp::from_bits(val as u8)
    }
    #[doc = "Command Protection Status Flag"]
    #[inline(always)]
    pub const fn set_cmdp(&mut self, val: super::vals::Cmdp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Command Domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddid(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Command Domain ID"]
    #[inline(always)]
    pub const fn set_cmddid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn dfdif(&self) -> super::vals::Dfdif {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dfdif::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag"]
    #[inline(always)]
    pub const fn set_dfdif(&mut self, val: super::vals::Dfdif) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Salvage Used for Erase operation"]
    #[must_use]
    #[inline(always)]
    pub const fn salv_used(&self) -> super::vals::SalvUsed {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::SalvUsed::from_bits(val as u8)
    }
    #[doc = "Salvage Used for Erase operation"]
    #[inline(always)]
    pub const fn set_salv_used(&mut self, val: super::vals::SalvUsed) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Program-Erase Write Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pewen(&self) -> super::vals::Pewen {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Pewen::from_bits(val as u8)
    }
    #[doc = "Program-Erase Write Enable Control"]
    #[inline(always)]
    pub const fn set_pewen(&mut self, val: super::vals::Pewen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Program/Erase Ready Control/Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn perdy(&self) -> super::vals::Perdy {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Perdy::from_bits(val as u8)
    }
    #[doc = "Program/Erase Ready Control/Status Flag"]
    #[inline(always)]
    pub const fn set_perdy(&mut self, val: super::vals::Perdy) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Fstat {
    #[inline(always)]
    fn default() -> Fstat {
        Fstat(0)
    }
}
impl core::fmt::Debug for Fstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fstat")
            .field("fail", &self.fail())
            .field("cmdabt", &self.cmdabt())
            .field("pviol", &self.pviol())
            .field("accerr", &self.accerr())
            .field("cwsabt", &self.cwsabt())
            .field("ccif", &self.ccif())
            .field("cmdprt", &self.cmdprt())
            .field("cmdp", &self.cmdp())
            .field("cmddid", &self.cmddid())
            .field("dfdif", &self.dfdif())
            .field("salv_used", &self.salv_used())
            .field("pewen", &self.pewen())
            .field("perdy", &self.perdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fstat {{ fail: {:?}, cmdabt: {:?}, pviol: {:?}, accerr: {:?}, cwsabt: {:?}, ccif: {:?}, cmdprt: {:?}, cmdp: {:?}, cmddid: {=u8:?}, dfdif: {:?}, salv_used: {:?}, pewen: {:?}, perdy: {:?} }}",
            self.fail(),
            self.cmdabt(),
            self.pviol(),
            self.accerr(),
            self.cwsabt(),
            self.ccif(),
            self.cmdprt(),
            self.cmdp(),
            self.cmddid(),
            self.dfdif(),
            self.salv_used(),
            self.pewen(),
            self.perdy()
        )
    }
}
#[doc = "Flash Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftest(pub u32);
impl Ftest {
    #[doc = "Test Mode Entry Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tmectl(&self) -> super::vals::Tmectl {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tmectl::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Control"]
    #[inline(always)]
    pub const fn set_tmectl(&mut self, val: super::vals::Tmectl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Mode Entry Writable"]
    #[must_use]
    #[inline(always)]
    pub const fn tmewr(&self) -> super::vals::Tmewr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tmewr::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Writable"]
    #[inline(always)]
    pub const fn set_tmewr(&mut self, val: super::vals::Tmewr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Mode Entry"]
    #[must_use]
    #[inline(always)]
    pub const fn tme(&self) -> super::vals::Tme {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tme::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry"]
    #[inline(always)]
    pub const fn set_tme(&mut self, val: super::vals::Tme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Mode Status"]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> super::vals::Tmode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tmode::from_bits(val as u8)
    }
    #[doc = "Test Mode Status"]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: super::vals::Tmode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Mode Entry Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn tmelock(&self) -> super::vals::Tmelock {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tmelock::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Lock"]
    #[inline(always)]
    pub const fn set_tmelock(&mut self, val: super::vals::Tmelock) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Ftest {
    #[inline(always)]
    fn default() -> Ftest {
        Ftest(0)
    }
}
impl core::fmt::Debug for Ftest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftest")
            .field("tmectl", &self.tmectl())
            .field("tmewr", &self.tmewr())
            .field("tme", &self.tme())
            .field("tmode", &self.tmode())
            .field("tmelock", &self.tmelock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ftest {{ tmectl: {:?}, tmewr: {:?}, tme: {:?}, tmode: {:?}, tmelock: {:?} }}",
            self.tmectl(),
            self.tmewr(),
            self.tme(),
            self.tmode(),
            self.tmelock()
        )
    }
}
#[doc = "Maximum Pulse Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaxPulseCnt(pub u32);
impl MaxPulseCnt {
    #[doc = "Last SMW Operation's Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn last_pcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Last SMW Operation's Pulse Count"]
    #[inline(always)]
    pub const fn set_last_pcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Maximum Erase Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn max_ers_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Maximum Erase Pulse Count"]
    #[inline(always)]
    pub const fn set_max_ers_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Maximum Program Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn max_pgm_cnt(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "Maximum Program Pulse Count"]
    #[inline(always)]
    pub const fn set_max_pgm_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for MaxPulseCnt {
    #[inline(always)]
    fn default() -> MaxPulseCnt {
        MaxPulseCnt(0)
    }
}
impl core::fmt::Debug for MaxPulseCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaxPulseCnt")
            .field("last_pcnt", &self.last_pcnt())
            .field("max_ers_cnt", &self.max_ers_cnt())
            .field("max_pgm_cnt", &self.max_pgm_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaxPulseCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MaxPulseCnt {{ last_pcnt: {=u16:?}, max_ers_cnt: {=u16:?}, max_pgm_cnt: {=u8:?} }}",
            self.last_pcnt(),
            self.max_ers_cnt(),
            self.max_pgm_cnt()
        )
    }
}
#[doc = "FMU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctl(pub u32);
impl Mctl {
    #[doc = "Core Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn corehld(&self) -> super::vals::Corehld {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Corehld::from_bits(val as u8)
    }
    #[doc = "Core Hold"]
    #[inline(always)]
    pub const fn set_corehld(&mut self, val: super::vals::Corehld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSACTIVE Feature Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lsact_en(&self) -> super::vals::LsactEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::LsactEn::from_bits(val as u8)
    }
    #[doc = "LSACTIVE Feature Enable"]
    #[inline(always)]
    pub const fn set_lsact_en(&mut self, val: super::vals::LsactEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSACTIVE Write Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lsactwren(&self) -> super::vals::Lsactwren {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lsactwren::from_bits(val as u8)
    }
    #[doc = "LSACTIVE Write Enable"]
    #[inline(always)]
    pub const fn set_lsactwren(&mut self, val: super::vals::Lsactwren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Master Repair Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn master_repair_en(&self) -> super::vals::MasterRepairEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::MasterRepairEn::from_bits(val as u8)
    }
    #[doc = "Master Repair Enable"]
    #[inline(always)]
    pub const fn set_master_repair_en(&mut self, val: super::vals::MasterRepairEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "RF Active Command Enable Control"]
    #[must_use]
    #[inline(always)]
    pub const fn rfcmden(&self) -> super::vals::Rfcmden {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Rfcmden::from_bits(val as u8)
    }
    #[doc = "RF Active Command Enable Control"]
    #[inline(always)]
    pub const fn set_rfcmden(&mut self, val: super::vals::Rfcmden) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Command Write Sequence Abort Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cwsabten(&self) -> super::vals::Cwsabten {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cwsabten::from_bits(val as u8)
    }
    #[doc = "Command Write Sequence Abort Enable"]
    #[inline(always)]
    pub const fn set_cwsabten(&mut self, val: super::vals::Cwsabten) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Margin Read Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrddis(&self) -> super::vals::Mrgrddis {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Mrgrddis::from_bits(val as u8)
    }
    #[doc = "Margin Read Disable"]
    #[inline(always)]
    pub const fn set_mrgrddis(&mut self, val: super::vals::Mrgrddis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Margin Read Setting for Program"]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrd0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Margin Read Setting for Program"]
    #[inline(always)]
    pub const fn set_mrgrd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Margin Read Setting for Erase"]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Margin Read Setting for Erase"]
    #[inline(always)]
    pub const fn set_mrgrd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mass Erase (Erase All) Acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn ersaack(&self) -> super::vals::Ersaack {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Ersaack::from_bits(val as u8)
    }
    #[doc = "Mass Erase (Erase All) Acknowledge"]
    #[inline(always)]
    pub const fn set_ersaack(&mut self, val: super::vals::Ersaack) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Scan Observability Control"]
    #[must_use]
    #[inline(always)]
    pub const fn scan_obs(&self) -> super::vals::ScanObs {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::ScanObs::from_bits(val as u8)
    }
    #[doc = "Scan Observability Control"]
    #[inline(always)]
    pub const fn set_scan_obs(&mut self, val: super::vals::ScanObs) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "BIST IP Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_ctl(&self) -> super::vals::BistCtl {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::BistCtl::from_bits(val as u8)
    }
    #[doc = "BIST IP Control"]
    #[inline(always)]
    pub const fn set_bist_ctl(&mut self, val: super::vals::BistCtl) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SMWR IP Control"]
    #[must_use]
    #[inline(always)]
    pub const fn smwr_ctl(&self) -> super::vals::SmwrCtl {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::SmwrCtl::from_bits(val as u8)
    }
    #[doc = "SMWR IP Control"]
    #[inline(always)]
    pub const fn set_smwr_ctl(&mut self, val: super::vals::SmwrCtl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Salvage Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn salv_dis(&self) -> super::vals::SalvDis {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::SalvDis::from_bits(val as u8)
    }
    #[doc = "Salvage Disable"]
    #[inline(always)]
    pub const fn set_salv_dis(&mut self, val: super::vals::SalvDis) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SOC ECC Control"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_ecc_ctl(&self) -> super::vals::SocEccCtl {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SocEccCtl::from_bits(val as u8)
    }
    #[doc = "SOC ECC Control"]
    #[inline(always)]
    pub const fn set_soc_ecc_ctl(&mut self, val: super::vals::SocEccCtl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FMU ECC Control"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_ecc_ctl(&self) -> super::vals::FmuEccCtl {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::FmuEccCtl::from_bits(val as u8)
    }
    #[doc = "FMU ECC Control"]
    #[inline(always)]
    pub const fn set_fmu_ecc_ctl(&mut self, val: super::vals::FmuEccCtl) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BIST Power Mode Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_pwr_dis(&self) -> super::vals::BistPwrDis {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::BistPwrDis::from_bits(val as u8)
    }
    #[doc = "BIST Power Mode Disable"]
    #[inline(always)]
    pub const fn set_bist_pwr_dis(&mut self, val: super::vals::BistPwrDis) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Oscillator control"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_h(&self) -> super::vals::OscH {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::OscH::from_bits(val as u8)
    }
    #[doc = "Oscillator control"]
    #[inline(always)]
    pub const fn set_osc_h(&mut self, val: super::vals::OscH) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mctl {
    #[inline(always)]
    fn default() -> Mctl {
        Mctl(0)
    }
}
impl core::fmt::Debug for Mctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctl")
            .field("corehld", &self.corehld())
            .field("lsact_en", &self.lsact_en())
            .field("lsactwren", &self.lsactwren())
            .field("master_repair_en", &self.master_repair_en())
            .field("rfcmden", &self.rfcmden())
            .field("cwsabten", &self.cwsabten())
            .field("mrgrddis", &self.mrgrddis())
            .field("mrgrd0", &self.mrgrd0())
            .field("mrgrd1", &self.mrgrd1())
            .field("ersaack", &self.ersaack())
            .field("scan_obs", &self.scan_obs())
            .field("bist_ctl", &self.bist_ctl())
            .field("smwr_ctl", &self.smwr_ctl())
            .field("salv_dis", &self.salv_dis())
            .field("soc_ecc_ctl", &self.soc_ecc_ctl())
            .field("fmu_ecc_ctl", &self.fmu_ecc_ctl())
            .field("bist_pwr_dis", &self.bist_pwr_dis())
            .field("osc_h", &self.osc_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctl {{ corehld: {:?}, lsact_en: {:?}, lsactwren: {:?}, master_repair_en: {:?}, rfcmden: {:?}, cwsabten: {:?}, mrgrddis: {:?}, mrgrd0: {=u8:?}, mrgrd1: {=u8:?}, ersaack: {:?}, scan_obs: {:?}, bist_ctl: {:?}, smwr_ctl: {:?}, salv_dis: {:?}, soc_ecc_ctl: {:?}, fmu_ecc_ctl: {:?}, bist_pwr_dis: {:?}, osc_h: {:?} }}",
            self.corehld(),
            self.lsact_en(),
            self.lsactwren(),
            self.master_repair_en(),
            self.rfcmden(),
            self.cwsabten(),
            self.mrgrddis(),
            self.mrgrd0(),
            self.mrgrd1(),
            self.ersaack(),
            self.scan_obs(),
            self.bist_ctl(),
            self.smwr_ctl(),
            self.salv_dis(),
            self.soc_ecc_ctl(),
            self.fmu_ecc_ctl(),
            self.bist_pwr_dis(),
            self.osc_h()
        )
    }
}
#[doc = "Memory Map Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmAddr(pub u32);
impl MmAddr {
    #[doc = "Memory Map Address"]
    #[must_use]
    #[inline(always)]
    pub const fn mm_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory Map Address"]
    #[inline(always)]
    pub const fn set_mm_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MmAddr {
    #[inline(always)]
    fn default() -> MmAddr {
        MmAddr(0)
    }
}
impl core::fmt::Debug for MmAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmAddr")
            .field("mm_addr", &self.mm_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MmAddr {{ mm_addr: {=u32:?} }}", self.mm_addr())
    }
}
#[doc = "Memory Map Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmCtl(pub u32);
impl MmCtl {
    #[doc = "Register Access Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mm_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Access Enable"]
    #[inline(always)]
    pub const fn set_mm_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Register R/W Control"]
    #[must_use]
    #[inline(always)]
    pub const fn mm_rd(&self) -> super::vals::MmRd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MmRd::from_bits(val as u8)
    }
    #[doc = "Register R/W Control"]
    #[inline(always)]
    pub const fn set_mm_rd(&mut self, val: super::vals::MmRd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST on"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_on(&self) -> super::vals::BistOn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::BistOn::from_bits(val as u8)
    }
    #[doc = "BIST on"]
    #[inline(always)]
    pub const fn set_bist_on(&mut self, val: super::vals::BistOn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Force Switch Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sw_clk(&self) -> super::vals::ForceSwClk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ForceSwClk::from_bits(val as u8)
    }
    #[doc = "Force Switch Clock"]
    #[inline(always)]
    pub const fn set_force_sw_clk(&mut self, val: super::vals::ForceSwClk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for MmCtl {
    #[inline(always)]
    fn default() -> MmCtl {
        MmCtl(0)
    }
}
impl core::fmt::Debug for MmCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmCtl")
            .field("mm_sel", &self.mm_sel())
            .field("mm_rd", &self.mm_rd())
            .field("bist_on", &self.bist_on())
            .field("force_sw_clk", &self.force_sw_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MmCtl {{ mm_sel: {=bool:?}, mm_rd: {:?}, bist_on: {:?}, force_sw_clk: {:?} }}",
            self.mm_sel(),
            self.mm_rd(),
            self.bist_on(),
            self.force_sw_clk()
        )
    }
}
#[doc = "Memory Map Write Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmWdata(pub u32);
impl MmWdata {
    #[doc = "Memory Map Write Data"]
    #[must_use]
    #[inline(always)]
    pub const fn mm_wdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory Map Write Data"]
    #[inline(always)]
    pub const fn set_mm_wdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MmWdata {
    #[inline(always)]
    fn default() -> MmWdata {
        MmWdata(0)
    }
}
impl core::fmt::Debug for MmWdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmWdata")
            .field("mm_wdata", &self.mm_wdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmWdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MmWdata {{ mm_wdata: {=u32:?} }}", self.mm_wdata())
    }
}
#[doc = "FMU Memory Size Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msize(pub u32);
impl Msize {
    #[doc = "Size of Flash Block 0"]
    #[must_use]
    #[inline(always)]
    pub const fn maxaddr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Size of Flash Block 0"]
    #[inline(always)]
    pub const fn set_maxaddr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Size of Flash Block 1"]
    #[must_use]
    #[inline(always)]
    pub const fn maxaddr1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Size of Flash Block 1"]
    #[inline(always)]
    pub const fn set_maxaddr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Msize {
    #[inline(always)]
    fn default() -> Msize {
        Msize(0)
    }
}
impl core::fmt::Debug for Msize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msize")
            .field("maxaddr0", &self.maxaddr0())
            .field("maxaddr1", &self.maxaddr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msize {{ maxaddr0: {=u8:?}, maxaddr1: {=u8:?} }}",
            self.maxaddr0(),
            self.maxaddr1()
        )
    }
}
#[doc = "Parity Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parity(pub u32);
impl Parity {
    #[doc = "Read data \\[136:128\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn parity(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read data \\[136:128\\]"]
    #[inline(always)]
    pub const fn set_parity(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Parity {
    #[inline(always)]
    fn default() -> Parity {
        Parity(0)
    }
}
impl core::fmt::Debug for Parity {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parity")
            .field("parity", &self.parity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Parity {{ parity: {=u16:?} }}", self.parity())
    }
}
#[doc = "Block 0 Program Pulse Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmPulseCnt0(pub u32);
impl PgmPulseCnt0 {
    #[doc = "Program Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pgm_cnt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Program Pulse Count"]
    #[inline(always)]
    pub const fn set_pgm_cnt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmPulseCnt0 {
    #[inline(always)]
    fn default() -> PgmPulseCnt0 {
        PgmPulseCnt0(0)
    }
}
impl core::fmt::Debug for PgmPulseCnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmPulseCnt0")
            .field("pgm_cnt0", &self.pgm_cnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmPulseCnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmPulseCnt0 {{ pgm_cnt0: {=u32:?} }}", self.pgm_cnt0())
    }
}
#[doc = "Block 1 Program Pulse Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmPulseCnt1(pub u32);
impl PgmPulseCnt1 {
    #[doc = "Program Pulse Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pgm_cnt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Program Pulse Count"]
    #[inline(always)]
    pub const fn set_pgm_cnt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmPulseCnt1 {
    #[inline(always)]
    fn default() -> PgmPulseCnt1 {
        PgmPulseCnt1(0)
    }
}
impl core::fmt::Debug for PgmPulseCnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmPulseCnt1")
            .field("pgm_cnt1", &self.pgm_cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmPulseCnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmPulseCnt1 {{ pgm_cnt1: {=u32:?} }}", self.pgm_cnt1())
    }
}
#[doc = "Port Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortCtrl(pub u32);
impl PortCtrl {
    #[doc = "BIST Done Select"]
    #[must_use]
    #[inline(always)]
    pub const fn bdone_sel(&self) -> super::vals::BdoneSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::BdoneSel::from_bits(val as u8)
    }
    #[doc = "BIST Done Select"]
    #[inline(always)]
    pub const fn set_bdone_sel(&mut self, val: super::vals::BdoneSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "BIST Serial Data Output Select"]
    #[must_use]
    #[inline(always)]
    pub const fn bsdo_sel(&self) -> super::vals::BsdoSel {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::BsdoSel::from_bits(val as u8)
    }
    #[doc = "BIST Serial Data Output Select"]
    #[inline(always)]
    pub const fn set_bsdo_sel(&mut self, val: super::vals::BsdoSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for PortCtrl {
    #[inline(always)]
    fn default() -> PortCtrl {
        PortCtrl(0)
    }
}
impl core::fmt::Debug for PortCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PortCtrl")
            .field("bdone_sel", &self.bdone_sel())
            .field("bsdo_sel", &self.bsdo_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PortCtrl {{ bdone_sel: {:?}, bsdo_sel: {:?} }}",
            self.bdone_sel(),
            self.bsdo_sel()
        )
    }
}
#[doc = "Power Mode Options Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrOpt(pub u32);
impl PwrOpt {
    #[doc = "Power Down Clock Divider Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_cdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Power Down Clock Divider Setting"]
    #[inline(always)]
    pub const fn set_pd_cdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sleep Recovery Timer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn slm_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sleep Recovery Timer Count"]
    #[inline(always)]
    pub const fn set_slm_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Power Down BIST Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_timer_en(&self) -> super::vals::PdTimerEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::PdTimerEn::from_bits(val as u8)
    }
    #[doc = "Power Down BIST Timer Enable"]
    #[inline(always)]
    pub const fn set_pd_timer_en(&mut self, val: super::vals::PdTimerEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrOpt {
    #[inline(always)]
    fn default() -> PwrOpt {
        PwrOpt(0)
    }
}
impl core::fmt::Debug for PwrOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwrOpt")
            .field("pd_cdiv", &self.pd_cdiv())
            .field("slm_count", &self.slm_count())
            .field("pd_timer_en", &self.pd_timer_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwrOpt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwrOpt {{ pd_cdiv: {=u8:?}, slm_count: {=u16:?}, pd_timer_en: {:?} }}",
            self.pd_cdiv(),
            self.slm_count(),
            self.pd_timer_en()
        )
    }
}
#[doc = "BIST Address MISR 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMisr0(pub u32);
impl RAMisr0 {
    #[doc = "Address Signature"]
    #[must_use]
    #[inline(always)]
    pub const fn adrsig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address Signature"]
    #[inline(always)]
    pub const fn set_adrsig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RAMisr0 {
    #[inline(always)]
    fn default() -> RAMisr0 {
        RAMisr0(0)
    }
}
impl core::fmt::Debug for RAMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMisr0")
            .field("adrsig0", &self.adrsig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAMisr0 {{ adrsig0: {=u32:?} }}", self.adrsig0())
    }
}
#[doc = "BIST Address MISR 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMisr1(pub u32);
impl RAMisr1 {
    #[doc = "MISR Address Signature High"]
    #[must_use]
    #[inline(always)]
    pub const fn adrsig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Address Signature High"]
    #[inline(always)]
    pub const fn set_adrsig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RAMisr1 {
    #[inline(always)]
    fn default() -> RAMisr1 {
        RAMisr1(0)
    }
}
impl core::fmt::Debug for RAMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMisr1")
            .field("adrsig1", &self.adrsig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAMisr1 {{ adrsig1: {=u8:?} }}", self.adrsig1())
    }
}
#[doc = "BIST Abort Loop Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAbortLoop(pub u32);
impl RAbortLoop {
    #[doc = "Abort Loop"]
    #[must_use]
    #[inline(always)]
    pub const fn abort_loop(&self) -> super::vals::AbortLoop {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AbortLoop::from_bits(val as u8)
    }
    #[doc = "Abort Loop"]
    #[inline(always)]
    pub const fn set_abort_loop(&mut self, val: super::vals::AbortLoop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RAbortLoop {
    #[inline(always)]
    fn default() -> RAbortLoop {
        RAbortLoop(0)
    }
}
impl core::fmt::Debug for RAbortLoop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAbortLoop")
            .field("abort_loop", &self.abort_loop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAbortLoop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAbortLoop {{ abort_loop: {:?} }}", self.abort_loop())
    }
}
#[doc = "BIST Address Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAdrCtrl(pub u32);
impl RAdrCtrl {
    #[doc = "Data Group Select"]
    #[must_use]
    #[inline(always)]
    pub const fn grpsel(&self) -> super::vals::Grpsel {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Grpsel::from_bits(val as u8)
    }
    #[doc = "Data Group Select"]
    #[inline(always)]
    pub const fn set_grpsel(&mut self, val: super::vals::Grpsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "BIST XADR"]
    #[must_use]
    #[inline(always)]
    pub const fn xadr(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "BIST XADR"]
    #[inline(always)]
    pub const fn set_xadr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "BIST YADR"]
    #[must_use]
    #[inline(always)]
    pub const fn yadr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "BIST YADR"]
    #[inline(always)]
    pub const fn set_yadr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Program Attribute"]
    #[must_use]
    #[inline(always)]
    pub const fn prog_attr(&self) -> super::vals::ProgAttr {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::ProgAttr::from_bits(val as u8)
    }
    #[doc = "Program Attribute"]
    #[inline(always)]
    pub const fn set_prog_attr(&mut self, val: super::vals::ProgAttr) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for RAdrCtrl {
    #[inline(always)]
    fn default() -> RAdrCtrl {
        RAdrCtrl(0)
    }
}
impl core::fmt::Debug for RAdrCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAdrCtrl")
            .field("grpsel", &self.grpsel())
            .field("xadr", &self.xadr())
            .field("yadr", &self.yadr())
            .field("prog_attr", &self.prog_attr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAdrCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAdrCtrl {{ grpsel: {:?}, xadr: {=u16:?}, yadr: {=u8:?}, prog_attr: {:?} }}",
            self.grpsel(),
            self.xadr(),
            self.yadr(),
            self.prog_attr()
        )
    }
}
#[doc = "BIST Address Query Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAdrQuery(pub u32);
impl RAdrQuery {
    #[doc = "Failing YADR"]
    #[must_use]
    #[inline(always)]
    pub const fn yadrfail(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Failing YADR"]
    #[inline(always)]
    pub const fn set_yadrfail(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Failing XADR"]
    #[must_use]
    #[inline(always)]
    pub const fn xadrfail(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x0fff;
        val as u16
    }
    #[doc = "Failing XADR"]
    #[inline(always)]
    pub const fn set_xadrfail(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
    }
}
impl Default for RAdrQuery {
    #[inline(always)]
    fn default() -> RAdrQuery {
        RAdrQuery(0)
    }
}
impl core::fmt::Debug for RAdrQuery {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAdrQuery")
            .field("yadrfail", &self.yadrfail())
            .field("xadrfail", &self.xadrfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAdrQuery {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAdrQuery {{ yadrfail: {=u8:?}, xadrfail: {=u16:?} }}",
            self.yadrfail(),
            self.xadrfail()
        )
    }
}
#[doc = "BIST Control MISR 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCMisr0(pub u32);
impl RCMisr0 {
    #[doc = "Control Signature"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrlsig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Control Signature"]
    #[inline(always)]
    pub const fn set_ctrlsig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCMisr0 {
    #[inline(always)]
    fn default() -> RCMisr0 {
        RCMisr0(0)
    }
}
impl core::fmt::Debug for RCMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCMisr0")
            .field("ctrlsig0", &self.ctrlsig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCMisr0 {{ ctrlsig0: {=u32:?} }}", self.ctrlsig0())
    }
}
#[doc = "BIST Control MISR 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCMisr1(pub u32);
impl RCMisr1 {
    #[doc = "MISR Control Signature High"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrlsig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Control Signature High"]
    #[inline(always)]
    pub const fn set_ctrlsig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RCMisr1 {
    #[inline(always)]
    fn default() -> RCMisr1 {
        RCMisr1(0)
    }
}
impl core::fmt::Debug for RCMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCMisr1")
            .field("ctrlsig1", &self.ctrlsig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCMisr1 {{ ctrlsig1: {=u8:?} }}", self.ctrlsig1())
    }
}
#[doc = "BIST Loop Count Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCntLoopCtrl(pub u32);
impl RCntLoopCtrl {
    #[doc = "Loop Count Control"]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Loop Count Control"]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Loop Option"]
    #[must_use]
    #[inline(always)]
    pub const fn loopopt(&self) -> super::vals::Loopopt {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Loopopt::from_bits(val as u8)
    }
    #[doc = "Loop Option"]
    #[inline(always)]
    pub const fn set_loopopt(&mut self, val: super::vals::Loopopt) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn loopunit(&self) -> super::vals::Loopunit {
        let val = (self.0 >> 15usize) & 0x07;
        super::vals::Loopunit::from_bits(val as u8)
    }
    #[doc = "Loop Time Unit"]
    #[inline(always)]
    pub const fn set_loopunit(&mut self, val: super::vals::Loopunit) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Loop Time Delay Scalar"]
    #[must_use]
    #[inline(always)]
    pub const fn loopdly(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x7f;
        val as u8
    }
    #[doc = "Loop Time Delay Scalar"]
    #[inline(always)]
    pub const fn set_loopdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
    }
}
impl Default for RCntLoopCtrl {
    #[inline(always)]
    fn default() -> RCntLoopCtrl {
        RCntLoopCtrl(0)
    }
}
impl core::fmt::Debug for RCntLoopCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCntLoopCtrl")
            .field("loopcnt", &self.loopcnt())
            .field("loopopt", &self.loopopt())
            .field("loopunit", &self.loopunit())
            .field("loopdly", &self.loopdly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCntLoopCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCntLoopCtrl {{ loopcnt: {=u16:?}, loopopt: {:?}, loopunit: {:?}, loopdly: {=u8:?} }}",
            self.loopcnt(),
            self.loopopt(),
            self.loopunit(),
            self.loopdly()
        )
    }
}
#[doc = "BIST DIN MISR 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDMisr0(pub u32);
impl RDMisr0 {
    #[doc = "Data Signature"]
    #[must_use]
    #[inline(always)]
    pub const fn datasig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Signature"]
    #[inline(always)]
    pub const fn set_datasig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDMisr0 {
    #[inline(always)]
    fn default() -> RDMisr0 {
        RDMisr0(0)
    }
}
impl core::fmt::Debug for RDMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDMisr0")
            .field("datasig0", &self.datasig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDMisr0 {{ datasig0: {=u32:?} }}", self.datasig0())
    }
}
#[doc = "BIST DIN MISR 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDMisr1(pub u32);
impl RDMisr1 {
    #[doc = "MISR Data Signature High"]
    #[must_use]
    #[inline(always)]
    pub const fn datasig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Data Signature High"]
    #[inline(always)]
    pub const fn set_datasig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RDMisr1 {
    #[inline(always)]
    fn default() -> RDMisr1 {
        RDMisr1(0)
    }
}
impl core::fmt::Debug for RDMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDMisr1")
            .field("datasig1", &self.datasig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDMisr1 {{ datasig1: {=u8:?} }}", self.datasig1())
    }
}
#[doc = "BIST Data Control 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl0(pub u32);
impl RDataCtrl0 {
    #[doc = "BIST Data 0 Low"]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 0 Low"]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl0 {
    #[inline(always)]
    fn default() -> RDataCtrl0 {
        RDataCtrl0(0)
    }
}
impl core::fmt::Debug for RDataCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl0")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl0 {{ data0: {=u32:?} }}", self.data0())
    }
}
#[doc = "BIST Data Control 0 Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl0Ex(pub u32);
impl RDataCtrl0Ex {
    #[doc = "BIST Data 0 High"]
    #[must_use]
    #[inline(always)]
    pub const fn data0x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 0 High"]
    #[inline(always)]
    pub const fn set_data0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl0Ex {
    #[inline(always)]
    fn default() -> RDataCtrl0Ex {
        RDataCtrl0Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl0Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl0Ex")
            .field("data0x", &self.data0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl0Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl0Ex {{ data0x: {=u8:?} }}", self.data0x())
    }
}
#[doc = "BIST Data Control 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl1(pub u32);
impl RDataCtrl1 {
    #[doc = "BIST Data 1 Low"]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 1 Low"]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl1 {
    #[inline(always)]
    fn default() -> RDataCtrl1 {
        RDataCtrl1(0)
    }
}
impl core::fmt::Debug for RDataCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl1")
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl1 {{ data1: {=u32:?} }}", self.data1())
    }
}
#[doc = "BIST Data Control 1 Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl1Ex(pub u32);
impl RDataCtrl1Ex {
    #[doc = "BIST Data 1 High"]
    #[must_use]
    #[inline(always)]
    pub const fn data1x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 1 High"]
    #[inline(always)]
    pub const fn set_data1x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl1Ex {
    #[inline(always)]
    fn default() -> RDataCtrl1Ex {
        RDataCtrl1Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl1Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl1Ex")
            .field("data1x", &self.data1x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl1Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl1Ex {{ data1x: {=u8:?} }}", self.data1x())
    }
}
#[doc = "BIST Data Control 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl2(pub u32);
impl RDataCtrl2 {
    #[doc = "BIST Data 2 Low"]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 2 Low"]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl2 {
    #[inline(always)]
    fn default() -> RDataCtrl2 {
        RDataCtrl2(0)
    }
}
impl core::fmt::Debug for RDataCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl2")
            .field("data2", &self.data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl2 {{ data2: {=u32:?} }}", self.data2())
    }
}
#[doc = "BIST Data Control 2 Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl2Ex(pub u32);
impl RDataCtrl2Ex {
    #[doc = "BIST Data 2 High"]
    #[must_use]
    #[inline(always)]
    pub const fn data2x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 2 High"]
    #[inline(always)]
    pub const fn set_data2x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl2Ex {
    #[inline(always)]
    fn default() -> RDataCtrl2Ex {
        RDataCtrl2Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl2Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl2Ex")
            .field("data2x", &self.data2x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl2Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl2Ex {{ data2x: {=u8:?} }}", self.data2x())
    }
}
#[doc = "BIST Data Control 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl3(pub u32);
impl RDataCtrl3 {
    #[doc = "BIST Data 3 Low"]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 3 Low"]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl3 {
    #[inline(always)]
    fn default() -> RDataCtrl3 {
        RDataCtrl3(0)
    }
}
impl core::fmt::Debug for RDataCtrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl3")
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl3 {{ data3: {=u32:?} }}", self.data3())
    }
}
#[doc = "BIST Data Control 3 Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl3Ex(pub u32);
impl RDataCtrl3Ex {
    #[doc = "BIST Data 3 High"]
    #[must_use]
    #[inline(always)]
    pub const fn data3x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 3 High"]
    #[inline(always)]
    pub const fn set_data3x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl3Ex {
    #[inline(always)]
    fn default() -> RDataCtrl3Ex {
        RDataCtrl3Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl3Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl3Ex")
            .field("data3x", &self.data3x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl3Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl3Ex {{ data3x: {=u8:?} }}", self.data3x())
    }
}
#[doc = "BIST DFT Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDftCtrl(pub u32);
impl RDftCtrl {
    #[doc = "DFT XADR Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn dft_xadr(&self) -> super::vals::DftXadr {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::DftXadr::from_bits(val as u8)
    }
    #[doc = "DFT XADR Pattern"]
    #[inline(always)]
    pub const fn set_dft_xadr(&mut self, val: super::vals::DftXadr) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DFT YADR Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn dft_yadr(&self) -> super::vals::DftYadr {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::DftYadr::from_bits(val as u8)
    }
    #[doc = "DFT YADR Pattern"]
    #[inline(always)]
    pub const fn set_dft_yadr(&mut self, val: super::vals::DftYadr) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "DFT Data Pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn dft_data(&self) -> super::vals::DftData {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::DftData::from_bits(val as u8)
    }
    #[doc = "DFT Data Pattern"]
    #[inline(always)]
    pub const fn set_dft_data(&mut self, val: super::vals::DftData) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Data Compare Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_mask(&self) -> super::vals::CmpMask {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::CmpMask::from_bits(val as u8)
    }
    #[doc = "Data Compare Mask"]
    #[inline(always)]
    pub const fn set_cmp_mask(&mut self, val: super::vals::CmpMask) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DFT Data Source"]
    #[must_use]
    #[inline(always)]
    pub const fn dft_data_src(&self) -> super::vals::DftDataSrc {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::DftDataSrc::from_bits(val as u8)
    }
    #[doc = "DFT Data Source"]
    #[inline(always)]
    pub const fn set_dft_data_src(&mut self, val: super::vals::DftDataSrc) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for RDftCtrl {
    #[inline(always)]
    fn default() -> RDftCtrl {
        RDftCtrl(0)
    }
}
impl core::fmt::Debug for RDftCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDftCtrl")
            .field("dft_xadr", &self.dft_xadr())
            .field("dft_yadr", &self.dft_yadr())
            .field("dft_data", &self.dft_data())
            .field("cmp_mask", &self.cmp_mask())
            .field("dft_data_src", &self.dft_data_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDftCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RDftCtrl {{ dft_xadr: {:?}, dft_yadr: {:?}, dft_data: {:?}, cmp_mask: {:?}, dft_data_src: {:?} }}",
            self.dft_xadr(),
            self.dft_yadr(),
            self.dft_data(),
            self.cmp_mask(),
            self.dft_data_src()
        )
    }
}
#[doc = "BIST DOUT Query 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDoutQuery0(pub u32);
impl RDoutQuery0 {
    #[doc = "Failing DOUT Low"]
    #[must_use]
    #[inline(always)]
    pub const fn doutfail(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Failing DOUT Low"]
    #[inline(always)]
    pub const fn set_doutfail(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDoutQuery0 {
    #[inline(always)]
    fn default() -> RDoutQuery0 {
        RDoutQuery0(0)
    }
}
impl core::fmt::Debug for RDoutQuery0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDoutQuery0")
            .field("doutfail", &self.doutfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDoutQuery0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDoutQuery0 {{ doutfail: {=u32:?} }}", self.doutfail())
    }
}
#[doc = "BIST DOUT Query 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDoutQuery1(pub u32);
impl RDoutQuery1 {
    #[doc = "Failing DOUT High"]
    #[must_use]
    #[inline(always)]
    pub const fn dout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Failing DOUT High"]
    #[inline(always)]
    pub const fn set_dout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDoutQuery1 {
    #[inline(always)]
    fn default() -> RDoutQuery1 {
        RDoutQuery1(0)
    }
}
impl core::fmt::Debug for RDoutQuery1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDoutQuery1")
            .field("dout", &self.dout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDoutQuery1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDoutQuery1 {{ dout: {=u8:?} }}", self.dout())
    }
}
#[doc = "BIST Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIpConfig(pub u32);
impl RIpConfig {
    #[doc = "Block 0 Select Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ipsel0(&self) -> super::vals::Ipsel0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Ipsel0::from_bits(val as u8)
    }
    #[doc = "Block 0 Select Control"]
    #[inline(always)]
    pub const fn set_ipsel0(&mut self, val: super::vals::Ipsel0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Block 1 Select Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ipsel1(&self) -> super::vals::Ipsel1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Ipsel1::from_bits(val as u8)
    }
    #[doc = "Block 1 Select Control"]
    #[inline(always)]
    pub const fn set_ipsel1(&mut self, val: super::vals::Ipsel1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Clock Divide Scalar for Long Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_cdivl(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub const fn set_bist_cdivl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Number of clock cycles to generate short pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn cdivs(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of clock cycles to generate short pulse"]
    #[inline(always)]
    pub const fn set_cdivs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer adjust for verify"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_tvfy(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer adjust for verify"]
    #[inline(always)]
    pub const fn set_bist_tvfy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "BIST self-test control"]
    #[must_use]
    #[inline(always)]
    pub const fn tstctl(&self) -> super::vals::Tstctl {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Tstctl::from_bits(val as u8)
    }
    #[doc = "BIST self-test control"]
    #[inline(always)]
    pub const fn set_tstctl(&mut self, val: super::vals::Tstctl) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Debug feature control"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgctl(&self) -> super::vals::Dbgctl {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dbgctl::from_bits(val as u8)
    }
    #[doc = "Debug feature control"]
    #[inline(always)]
    pub const fn set_dbgctl(&mut self, val: super::vals::Dbgctl) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "BIST Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_clk_sel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Clock Select"]
    #[inline(always)]
    pub const fn set_bist_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "SMWR DOUT Function Control"]
    #[must_use]
    #[inline(always)]
    pub const fn smwtst(&self) -> super::vals::Smwtst {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Smwtst::from_bits(val as u8)
    }
    #[doc = "SMWR DOUT Function Control"]
    #[inline(always)]
    pub const fn set_smwtst(&mut self, val: super::vals::Smwtst) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "BIST ECC Control"]
    #[must_use]
    #[inline(always)]
    pub const fn eccen(&self) -> super::vals::Eccen {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Eccen::from_bits(val as u8)
    }
    #[doc = "BIST ECC Control"]
    #[inline(always)]
    pub const fn set_eccen(&mut self, val: super::vals::Eccen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RIpConfig {
    #[inline(always)]
    fn default() -> RIpConfig {
        RIpConfig(0)
    }
}
impl core::fmt::Debug for RIpConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIpConfig")
            .field("ipsel0", &self.ipsel0())
            .field("ipsel1", &self.ipsel1())
            .field("bist_cdivl", &self.bist_cdivl())
            .field("cdivs", &self.cdivs())
            .field("bist_tvfy", &self.bist_tvfy())
            .field("tstctl", &self.tstctl())
            .field("dbgctl", &self.dbgctl())
            .field("bist_clk_sel", &self.bist_clk_sel())
            .field("smwtst", &self.smwtst())
            .field("eccen", &self.eccen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIpConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIpConfig {{ ipsel0: {:?}, ipsel1: {:?}, bist_cdivl: {=u8:?}, cdivs: {=u8:?}, bist_tvfy: {=u8:?}, tstctl: {:?}, dbgctl: {:?}, bist_clk_sel: {=bool:?}, smwtst: {:?}, eccen: {:?} }}",
            self.ipsel0(),
            self.ipsel1(),
            self.bist_cdivl(),
            self.cdivs(),
            self.bist_tvfy(),
            self.tstctl(),
            self.dbgctl(),
            self.bist_clk_sel(),
            self.smwtst(),
            self.eccen()
        )
    }
}
#[doc = "BIST Pin Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RPinCtrl(pub u32);
impl RPinCtrl {
    #[doc = "Mass Erase"]
    #[must_use]
    #[inline(always)]
    pub const fn mas1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mass Erase"]
    #[inline(always)]
    pub const fn set_mas1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ifren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IFR Enable"]
    #[inline(always)]
    pub const fn set_ifren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IFR1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ifren1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IFR1 Enable"]
    #[inline(always)]
    pub const fn set_ifren1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Redundancy Block Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn reden(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Redundancy Block Enable"]
    #[inline(always)]
    pub const fn set_reden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Low Voltage Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lve(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Low Voltage Enable"]
    #[inline(always)]
    pub const fn set_lve(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Program Verify Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Program Verify Enable"]
    #[inline(always)]
    pub const fn set_pv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Erase Verify Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ev(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Verify Enable"]
    #[inline(always)]
    pub const fn set_ev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Program Current"]
    #[must_use]
    #[inline(always)]
    pub const fn wipgm(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current"]
    #[inline(always)]
    pub const fn set_wipgm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "High Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn whv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "High Voltage Level"]
    #[inline(always)]
    pub const fn set_whv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "Medium Voltage Level"]
    #[must_use]
    #[inline(always)]
    pub const fn wmv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level"]
    #[inline(always)]
    pub const fn set_wmv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "X Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn xe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "X Address Enable"]
    #[inline(always)]
    pub const fn set_xe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Y Address Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ye(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Y Address Enable"]
    #[inline(always)]
    pub const fn set_ye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Sense Amp Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn se(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Sense Amp Enable"]
    #[inline(always)]
    pub const fn set_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Erase Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn erase(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Mode"]
    #[inline(always)]
    pub const fn set_erase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Program Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn prog(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Program Mode"]
    #[inline(always)]
    pub const fn set_prog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "NVM Store"]
    #[must_use]
    #[inline(always)]
    pub const fn nvstr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "NVM Store"]
    #[inline(always)]
    pub const fn set_nvstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Sleep Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn slm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Mode Enable"]
    #[inline(always)]
    pub const fn set_slm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Recall Trim Code"]
    #[must_use]
    #[inline(always)]
    pub const fn recall(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Recall Trim Code"]
    #[inline(always)]
    pub const fn set_recall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "HEM Control"]
    #[must_use]
    #[inline(always)]
    pub const fn hem(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HEM Control"]
    #[inline(always)]
    pub const fn set_hem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RPinCtrl {
    #[inline(always)]
    fn default() -> RPinCtrl {
        RPinCtrl(0)
    }
}
impl core::fmt::Debug for RPinCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPinCtrl")
            .field("mas1", &self.mas1())
            .field("ifren", &self.ifren())
            .field("ifren1", &self.ifren1())
            .field("reden", &self.reden())
            .field("lve", &self.lve())
            .field("pv", &self.pv())
            .field("ev", &self.ev())
            .field("wipgm", &self.wipgm())
            .field("whv", &self.whv())
            .field("wmv", &self.wmv())
            .field("xe", &self.xe())
            .field("ye", &self.ye())
            .field("se", &self.se())
            .field("erase", &self.erase())
            .field("prog", &self.prog())
            .field("nvstr", &self.nvstr())
            .field("slm", &self.slm())
            .field("recall", &self.recall())
            .field("hem", &self.hem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RPinCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RPinCtrl {{ mas1: {=bool:?}, ifren: {=bool:?}, ifren1: {=bool:?}, reden: {=bool:?}, lve: {=bool:?}, pv: {=bool:?}, ev: {=bool:?}, wipgm: {=u8:?}, whv: {=u8:?}, wmv: {=u8:?}, xe: {=bool:?}, ye: {=bool:?}, se: {=bool:?}, erase: {=bool:?}, prog: {=bool:?}, nvstr: {=bool:?}, slm: {=bool:?}, recall: {=bool:?}, hem: {=bool:?} }}",
            self.mas1(),
            self.ifren(),
            self.ifren1(),
            self.reden(),
            self.lve(),
            self.pv(),
            self.ev(),
            self.wipgm(),
            self.whv(),
            self.wmv(),
            self.xe(),
            self.ye(),
            self.se(),
            self.erase(),
            self.prog(),
            self.nvstr(),
            self.slm(),
            self.recall(),
            self.hem()
        )
    }
}
#[doc = "BIST Repair 0 for Block 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair00(pub u32);
impl RRepair00 {
    #[doc = "Control Repair 0 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_0(&self) -> super::vals::RRepair00Rdis00 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RRepair00Rdis00::from_bits(val as u8)
    }
    #[doc = "Control Repair 0 in Block 0."]
    #[inline(always)]
    pub const fn set_rdis0_0(&mut self, val: super::vals::RRepair00Rdis00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 0 in Block 0"]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 0 in Block 0"]
    #[inline(always)]
    pub const fn set_radr0_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair00 {
    #[inline(always)]
    fn default() -> RRepair00 {
        RRepair00(0)
    }
}
impl core::fmt::Debug for RRepair00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair00")
            .field("rdis0_0", &self.rdis0_0())
            .field("radr0_0", &self.radr0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair00 {{ rdis0_0: {:?}, radr0_0: {=u8:?} }}",
            self.rdis0_0(),
            self.radr0_0()
        )
    }
}
#[doc = "BIST Repair 1 Block 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair01(pub u32);
impl RRepair01 {
    #[doc = "Control Repair 1 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_1(&self) -> super::vals::RRepair01Rdis01 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RRepair01Rdis01::from_bits(val as u8)
    }
    #[doc = "Control Repair 1 in Block 0."]
    #[inline(always)]
    pub const fn set_rdis0_1(&mut self, val: super::vals::RRepair01Rdis01) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 1 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 1 in Block 0."]
    #[inline(always)]
    pub const fn set_radr0_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair01 {
    #[inline(always)]
    fn default() -> RRepair01 {
        RRepair01(0)
    }
}
impl core::fmt::Debug for RRepair01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair01")
            .field("rdis0_1", &self.rdis0_1())
            .field("radr0_1", &self.radr0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair01 {{ rdis0_1: {:?}, radr0_1: {=u8:?} }}",
            self.rdis0_1(),
            self.radr0_1()
        )
    }
}
#[doc = "BIST Repair 0 Block 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair10(pub u32);
impl RRepair10 {
    #[doc = "Control Repair 0 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_0(&self) -> super::vals::RRepair10Rdis10 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RRepair10Rdis10::from_bits(val as u8)
    }
    #[doc = "Control Repair 0 in Block 1."]
    #[inline(always)]
    pub const fn set_rdis1_0(&mut self, val: super::vals::RRepair10Rdis10) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 0 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 0 in Block 1."]
    #[inline(always)]
    pub const fn set_radr1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair10 {
    #[inline(always)]
    fn default() -> RRepair10 {
        RRepair10(0)
    }
}
impl core::fmt::Debug for RRepair10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair10")
            .field("rdis1_0", &self.rdis1_0())
            .field("radr1_0", &self.radr1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair10 {{ rdis1_0: {:?}, radr1_0: {=u8:?} }}",
            self.rdis1_0(),
            self.radr1_0()
        )
    }
}
#[doc = "BIST Repair 1 Block 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair11(pub u32);
impl RRepair11 {
    #[doc = "Control Repair 1 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_1(&self) -> super::vals::RRepair11Rdis11 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RRepair11Rdis11::from_bits(val as u8)
    }
    #[doc = "Control Repair 1 in Block 1."]
    #[inline(always)]
    pub const fn set_rdis1_1(&mut self, val: super::vals::RRepair11Rdis11) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 1 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 1 in Block 1."]
    #[inline(always)]
    pub const fn set_radr1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair11 {
    #[inline(always)]
    fn default() -> RRepair11 {
        RRepair11(0)
    }
}
impl core::fmt::Debug for RRepair11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair11")
            .field("rdis1_1", &self.rdis1_1())
            .field("radr1_1", &self.radr1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair11 {{ rdis1_1: {:?}, radr1_1: {=u8:?} }}",
            self.rdis1_1(),
            self.radr1_1()
        )
    }
}
#[doc = "BIST SME WHV Setting 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmeWhv0(pub u32);
impl RSmeWhv0 {
    #[doc = "SME WHV Parameter Set 0"]
    #[must_use]
    #[inline(always)]
    pub const fn smewhv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SME WHV Parameter Set 0"]
    #[inline(always)]
    pub const fn set_smewhv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmeWhv0 {
    #[inline(always)]
    fn default() -> RSmeWhv0 {
        RSmeWhv0(0)
    }
}
impl core::fmt::Debug for RSmeWhv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmeWhv0")
            .field("smewhv0", &self.smewhv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmeWhv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmeWhv0 {{ smewhv0: {=u32:?} }}", self.smewhv0())
    }
}
#[doc = "BIST SME WHV Setting 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmeWhv1(pub u32);
impl RSmeWhv1 {
    #[doc = "SME WHV Parameter Set 1"]
    #[must_use]
    #[inline(always)]
    pub const fn smewhv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SME WHV Parameter Set 1"]
    #[inline(always)]
    pub const fn set_smewhv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmeWhv1 {
    #[inline(always)]
    fn default() -> RSmeWhv1 {
        RSmeWhv1(0)
    }
}
impl core::fmt::Debug for RSmeWhv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmeWhv1")
            .field("smewhv1", &self.smewhv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmeWhv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmeWhv1 {{ smewhv1: {=u32:?} }}", self.smewhv1())
    }
}
#[doc = "BIST SMP WHV Setting 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmpWhv0(pub u32);
impl RSmpWhv0 {
    #[doc = "SMP WHV Parameter Set 0"]
    #[must_use]
    #[inline(always)]
    pub const fn smpwhv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMP WHV Parameter Set 0"]
    #[inline(always)]
    pub const fn set_smpwhv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmpWhv0 {
    #[inline(always)]
    fn default() -> RSmpWhv0 {
        RSmpWhv0(0)
    }
}
impl core::fmt::Debug for RSmpWhv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmpWhv0")
            .field("smpwhv0", &self.smpwhv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmpWhv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmpWhv0 {{ smpwhv0: {=u32:?} }}", self.smpwhv0())
    }
}
#[doc = "BIST SMP WHV Setting 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmpWhv1(pub u32);
impl RSmpWhv1 {
    #[doc = "SMP WHV Parameter Set 1"]
    #[must_use]
    #[inline(always)]
    pub const fn smpwhv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMP WHV Parameter Set 1"]
    #[inline(always)]
    pub const fn set_smpwhv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmpWhv1 {
    #[inline(always)]
    fn default() -> RSmpWhv1 {
        RSmpWhv1(0)
    }
}
impl core::fmt::Debug for RSmpWhv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmpWhv1")
            .field("smpwhv1", &self.smpwhv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmpWhv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmpWhv1 {{ smpwhv1: {=u32:?} }}", self.smpwhv1())
    }
}
#[doc = "BIST SMW Query Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwQuery(pub u32);
impl RSmwQuery {
    #[doc = "SMW Total Loop Count"]
    #[must_use]
    #[inline(always)]
    pub const fn smwloop(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SMW Total Loop Count"]
    #[inline(always)]
    pub const fn set_smwloop(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SMW Last Voltage Setting"]
    #[must_use]
    #[inline(always)]
    pub const fn smwlast(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x01ff;
        val as u16
    }
    #[doc = "SMW Last Voltage Setting"]
    #[inline(always)]
    pub const fn set_smwlast(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 10usize)) | (((val as u32) & 0x01ff) << 10usize);
    }
}
impl Default for RSmwQuery {
    #[inline(always)]
    fn default() -> RSmwQuery {
        RSmwQuery(0)
    }
}
impl core::fmt::Debug for RSmwQuery {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwQuery")
            .field("smwloop", &self.smwloop())
            .field("smwlast", &self.smwlast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwQuery {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RSmwQuery {{ smwloop: {=u16:?}, smwlast: {=u16:?} }}",
            self.smwloop(),
            self.smwlast()
        )
    }
}
#[doc = "BIST SMW Setting 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting0(pub u32);
impl RSmwSetting0 {
    #[doc = "SMW Parameter Set 0"]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 0"]
    #[inline(always)]
    pub const fn set_smwparm0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting0 {
    #[inline(always)]
    fn default() -> RSmwSetting0 {
        RSmwSetting0(0)
    }
}
impl core::fmt::Debug for RSmwSetting0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting0")
            .field("smwparm0", &self.smwparm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting0 {{ smwparm0: {=u32:?} }}", self.smwparm0())
    }
}
#[doc = "BIST SMW Setting 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting1(pub u32);
impl RSmwSetting1 {
    #[doc = "SMW Parameter Set 1"]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 1"]
    #[inline(always)]
    pub const fn set_smwparm1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting1 {
    #[inline(always)]
    fn default() -> RSmwSetting1 {
        RSmwSetting1(0)
    }
}
impl core::fmt::Debug for RSmwSetting1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting1")
            .field("smwparm1", &self.smwparm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting1 {{ smwparm1: {=u32:?} }}", self.smwparm1())
    }
}
#[doc = "BIST SMW Setting 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting2(pub u32);
impl RSmwSetting2 {
    #[doc = "SMW Parameter Set 2"]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 2"]
    #[inline(always)]
    pub const fn set_smwparm2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting2 {
    #[inline(always)]
    fn default() -> RSmwSetting2 {
        RSmwSetting2(0)
    }
}
impl core::fmt::Debug for RSmwSetting2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting2")
            .field("smwparm2", &self.smwparm2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting2 {{ smwparm2: {=u32:?} }}", self.smwparm2())
    }
}
#[doc = "BIST SMW Setting 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting3(pub u32);
impl RSmwSetting3 {
    #[doc = "SMW Parameter Set 3"]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 3"]
    #[inline(always)]
    pub const fn set_smwparm3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for RSmwSetting3 {
    #[inline(always)]
    fn default() -> RSmwSetting3 {
        RSmwSetting3(0)
    }
}
impl core::fmt::Debug for RSmwSetting3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting3")
            .field("smwparm3", &self.smwparm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting3 {{ smwparm3: {=u32:?} }}", self.smwparm3())
    }
}
#[doc = "BIST Test Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTestCtrl(pub u32);
impl RTestCtrl {
    #[doc = "BIST Busy Status"]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> super::vals::Busy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Busy::from_bits(val as u8)
    }
    #[doc = "BIST Busy Status"]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: super::vals::Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BIST Debug Status"]
    #[must_use]
    #[inline(always)]
    pub const fn debug(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Debug Status"]
    #[inline(always)]
    pub const fn set_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST Status 0"]
    #[must_use]
    #[inline(always)]
    pub const fn status0(&self) -> super::vals::Status0 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Status0::from_bits(val as u8)
    }
    #[doc = "BIST Status 0"]
    #[inline(always)]
    pub const fn set_status0(&mut self, val: super::vals::Status0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "BIST status 1"]
    #[must_use]
    #[inline(always)]
    pub const fn status1(&self) -> super::vals::Status1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Status1::from_bits(val as u8)
    }
    #[doc = "BIST status 1"]
    #[inline(always)]
    pub const fn set_status1(&mut self, val: super::vals::Status1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "BIST Continue Debug Run"]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Continue Debug Run"]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Run New BIST Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn startrun(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Run New BIST Operation"]
    #[inline(always)]
    pub const fn set_startrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "BIST Command Index (code)"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindex(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "BIST Command Index (code)"]
    #[inline(always)]
    pub const fn set_cmdindex(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
    }
    #[doc = "BIST Disable IP1"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_ip1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Disable IP1"]
    #[inline(always)]
    pub const fn set_disable_ip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RTestCtrl {
    #[inline(always)]
    fn default() -> RTestCtrl {
        RTestCtrl(0)
    }
}
impl core::fmt::Debug for RTestCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTestCtrl")
            .field("busy", &self.busy())
            .field("debug", &self.debug())
            .field("status0", &self.status0())
            .field("status1", &self.status1())
            .field("debugrun", &self.debugrun())
            .field("startrun", &self.startrun())
            .field("cmdindex", &self.cmdindex())
            .field("disable_ip1", &self.disable_ip1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTestCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTestCtrl {{ busy: {:?}, debug: {=bool:?}, status0: {:?}, status1: {:?}, debugrun: {=bool:?}, startrun: {=bool:?}, cmdindex: {=u16:?}, disable_ip1: {=bool:?} }}",
            self.busy(),
            self.debug(),
            self.status0(),
            self.status1(),
            self.debugrun(),
            self.startrun(),
            self.cmdindex(),
            self.disable_ip1()
        )
    }
}
#[doc = "BIST Test Code Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTestcode(pub u32);
impl RTestcode {
    #[doc = "Used to store test code information before running TMR-RST/TMRSET BIST command"]
    #[must_use]
    #[inline(always)]
    pub const fn testcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Used to store test code information before running TMR-RST/TMRSET BIST command"]
    #[inline(always)]
    pub const fn set_testcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RTestcode {
    #[inline(always)]
    fn default() -> RTestcode {
        RTestcode(0)
    }
}
impl core::fmt::Debug for RTestcode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTestcode")
            .field("testcode", &self.testcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTestcode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RTestcode {{ testcode: {=u8:?} }}", self.testcode())
    }
}
#[doc = "BIST Timer Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTimerCtrl(pub u32);
impl RTimerCtrl {
    #[doc = "Tnvs Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvsunit(&self) -> super::vals::Tnvsunit {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Tnvsunit::from_bits(val as u8)
    }
    #[doc = "Tnvs Time Unit"]
    #[inline(always)]
    pub const fn set_tnvsunit(&mut self, val: super::vals::Tnvsunit) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Tnvs Time Delay Scalar"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvsdly(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Tnvs Time Delay Scalar"]
    #[inline(always)]
    pub const fn set_tnvsdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Tnvh Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvhunit(&self) -> super::vals::Tnvhunit {
        let val = (self.0 >> 7usize) & 0x07;
        super::vals::Tnvhunit::from_bits(val as u8)
    }
    #[doc = "Tnvh Time Unit"]
    #[inline(always)]
    pub const fn set_tnvhunit(&mut self, val: super::vals::Tnvhunit) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "Tnvh Time Delay Scalar"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvhdly(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Tnvh Time Delay Scalar"]
    #[inline(always)]
    pub const fn set_tnvhdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Tpgs Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgsunit(&self) -> super::vals::Tpgsunit {
        let val = (self.0 >> 14usize) & 0x07;
        super::vals::Tpgsunit::from_bits(val as u8)
    }
    #[doc = "Tpgs Time Unit"]
    #[inline(always)]
    pub const fn set_tpgsunit(&mut self, val: super::vals::Tpgsunit) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Tpgs Time Delay Scalar"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgsdly(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[doc = "Tpgs Time Delay Scalar"]
    #[inline(always)]
    pub const fn set_tpgsdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
    #[doc = "Trcv Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn trcvunit(&self) -> super::vals::Trcvunit {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Trcvunit::from_bits(val as u8)
    }
    #[doc = "Trcv Time Unit"]
    #[inline(always)]
    pub const fn set_trcvunit(&mut self, val: super::vals::Trcvunit) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Trcv Time Delay Scalar"]
    #[must_use]
    #[inline(always)]
    pub const fn trcvdly(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Trcv Time Delay Scalar"]
    #[inline(always)]
    pub const fn set_trcvdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Tlvs Time Unit"]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsunit(&self) -> super::vals::Tlvsunit {
        let val = (self.0 >> 28usize) & 0x07;
        super::vals::Tlvsunit::from_bits(val as u8)
    }
    #[doc = "Tlvs Time Unit"]
    #[inline(always)]
    pub const fn set_tlvsunit(&mut self, val: super::vals::Tlvsunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Tlvs Time Delay Scalar Low"]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsdly_l(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tlvs Time Delay Scalar Low"]
    #[inline(always)]
    pub const fn set_tlvsdly_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RTimerCtrl {
    #[inline(always)]
    fn default() -> RTimerCtrl {
        RTimerCtrl(0)
    }
}
impl core::fmt::Debug for RTimerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTimerCtrl")
            .field("tnvsunit", &self.tnvsunit())
            .field("tnvsdly", &self.tnvsdly())
            .field("tnvhunit", &self.tnvhunit())
            .field("tnvhdly", &self.tnvhdly())
            .field("tpgsunit", &self.tpgsunit())
            .field("tpgsdly", &self.tpgsdly())
            .field("trcvunit", &self.trcvunit())
            .field("trcvdly", &self.trcvdly())
            .field("tlvsunit", &self.tlvsunit())
            .field("tlvsdly_l", &self.tlvsdly_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTimerCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTimerCtrl {{ tnvsunit: {:?}, tnvsdly: {=u8:?}, tnvhunit: {:?}, tnvhdly: {=u8:?}, tpgsunit: {:?}, tpgsdly: {=u8:?}, trcvunit: {:?}, trcvdly: {=u8:?}, tlvsunit: {:?}, tlvsdly_l: {=bool:?} }}",
            self.tnvsunit(),
            self.tnvsdly(),
            self.tnvhunit(),
            self.tnvhdly(),
            self.tpgsunit(),
            self.tpgsdly(),
            self.trcvunit(),
            self.trcvdly(),
            self.tlvsunit(),
            self.tlvsdly_l()
        )
    }
}
#[doc = "BIST Timer Control Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTimerCtrlEx(pub u32);
impl RTimerCtrlEx {
    #[doc = "Tlvs Time Delay Scalar High"]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsdly_h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tlvs Time Delay Scalar High"]
    #[inline(always)]
    pub const fn set_tlvsdly_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RTimerCtrlEx {
    #[inline(always)]
    fn default() -> RTimerCtrlEx {
        RTimerCtrlEx(0)
    }
}
impl core::fmt::Debug for RTimerCtrlEx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTimerCtrlEx")
            .field("tlvsdly_h", &self.tlvsdly_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTimerCtrlEx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RTimerCtrlEx {{ tlvsdly_h: {=u8:?} }}", self.tlvsdly_h())
    }
}
#[doc = "Read Data 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData0(pub u32);
impl RdData0 {
    #[doc = "Read Data 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 0"]
    #[inline(always)]
    pub const fn set_rd_data0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData0 {
    #[inline(always)]
    fn default() -> RdData0 {
        RdData0(0)
    }
}
impl core::fmt::Debug for RdData0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData0")
            .field("rd_data0", &self.rd_data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData0 {{ rd_data0: {=u32:?} }}", self.rd_data0())
    }
}
#[doc = "Read Data 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData1(pub u32);
impl RdData1 {
    #[doc = "Read Data 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 1"]
    #[inline(always)]
    pub const fn set_rd_data1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData1 {
    #[inline(always)]
    fn default() -> RdData1 {
        RdData1(0)
    }
}
impl core::fmt::Debug for RdData1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData1")
            .field("rd_data1", &self.rd_data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData1 {{ rd_data1: {=u32:?} }}", self.rd_data1())
    }
}
#[doc = "Read Data 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData2(pub u32);
impl RdData2 {
    #[doc = "Read Data 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 2"]
    #[inline(always)]
    pub const fn set_rd_data2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData2 {
    #[inline(always)]
    fn default() -> RdData2 {
        RdData2(0)
    }
}
impl core::fmt::Debug for RdData2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData2")
            .field("rd_data2", &self.rd_data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData2 {{ rd_data2: {=u32:?} }}", self.rd_data2())
    }
}
#[doc = "Read Data 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData3(pub u32);
impl RdData3 {
    #[doc = "Read Data 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 3"]
    #[inline(always)]
    pub const fn set_rd_data3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData3 {
    #[inline(always)]
    fn default() -> RdData3 {
        RdData3(0)
    }
}
impl core::fmt::Debug for RdData3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData3")
            .field("rd_data3", &self.rd_data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData3 {{ rd_data3: {=u32:?} }}", self.rd_data3())
    }
}
#[doc = "Read Path Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdPathCtrlStatus(pub u32);
impl RdPathCtrlStatus {
    #[doc = "Read Capture Clock Periods"]
    #[must_use]
    #[inline(always)]
    pub const fn rd_capt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Capture Clock Periods"]
    #[inline(always)]
    pub const fn set_rd_capt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SE Clock Periods"]
    #[must_use]
    #[inline(always)]
    pub const fn se_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SE Clock Periods"]
    #[inline(always)]
    pub const fn set_se_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ECC Decoder Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_enableb(&self) -> super::vals::EccEnableb {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EccEnableb::from_bits(val as u8)
    }
    #[doc = "ECC Decoder Control"]
    #[inline(always)]
    pub const fn set_ecc_enableb(&mut self, val: super::vals::EccEnableb) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "MISR Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn misr_en(&self) -> super::vals::MisrEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::MisrEn::from_bits(val as u8)
    }
    #[doc = "MISR Enable"]
    #[inline(always)]
    pub const fn set_misr_en(&mut self, val: super::vals::MisrEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Copy Parity Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpy_par_en(&self) -> super::vals::CpyParEn {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::CpyParEn::from_bits(val as u8)
    }
    #[doc = "Copy Parity Enable"]
    #[inline(always)]
    pub const fn set_cpy_par_en(&mut self, val: super::vals::CpyParEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "BIST Mux to SMW"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_mux_to_smw(&self) -> super::vals::BistMuxToSmw {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::BistMuxToSmw::from_bits(val as u8)
    }
    #[doc = "BIST Mux to SMW"]
    #[inline(always)]
    pub const fn set_bist_mux_to_smw(&mut self, val: super::vals::BistMuxToSmw) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Multi-Cycle Address Setup Time"]
    #[must_use]
    #[inline(always)]
    pub const fn ad_set(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Multi-Cycle Address Setup Time"]
    #[inline(always)]
    pub const fn set_ad_set(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Write Path Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_path_en(&self) -> super::vals::WrPathEn {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::WrPathEn::from_bits(val as u8)
    }
    #[doc = "Write Path Enable"]
    #[inline(always)]
    pub const fn set_wr_path_en(&mut self, val: super::vals::WrPathEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Write Path ECC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wr_path_ecc_en(&self) -> super::vals::WrPathEccEn {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::WrPathEccEn::from_bits(val as u8)
    }
    #[doc = "Write Path ECC Enable"]
    #[inline(always)]
    pub const fn set_wr_path_ecc_en(&mut self, val: super::vals::WrPathEccEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Double-Bit Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dberr_reg(&self) -> super::vals::DberrReg {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::DberrReg::from_bits(val as u8)
    }
    #[doc = "Double-Bit Error"]
    #[inline(always)]
    pub const fn set_dberr_reg(&mut self, val: super::vals::DberrReg) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Single-Bit Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sberr_reg(&self) -> super::vals::SberrReg {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::SberrReg::from_bits(val as u8)
    }
    #[doc = "Single-Bit Error"]
    #[inline(always)]
    pub const fn set_sberr_reg(&mut self, val: super::vals::SberrReg) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Copy Phrase Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cpy_phrase_en(&self) -> super::vals::CpyPhraseEn {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::CpyPhraseEn::from_bits(val as u8)
    }
    #[doc = "Copy Phrase Enable"]
    #[inline(always)]
    pub const fn set_cpy_phrase_en(&mut self, val: super::vals::CpyPhraseEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SMW_ARRAY1_SMW0_SEL"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_array1_smw0_sel(&self) -> super::vals::SmwArray1Smw0Sel {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::SmwArray1Smw0Sel::from_bits(val as u8)
    }
    #[doc = "SMW_ARRAY1_SMW0_SEL"]
    #[inline(always)]
    pub const fn set_smw_array1_smw0_sel(&mut self, val: super::vals::SmwArray1Smw0Sel) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "BIST ECC Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_ecc_en(&self) -> super::vals::BistEccEn {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::BistEccEn::from_bits(val as u8)
    }
    #[doc = "BIST ECC Enable"]
    #[inline(always)]
    pub const fn set_bist_ecc_en(&mut self, val: super::vals::BistEccEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Last Read"]
    #[must_use]
    #[inline(always)]
    pub const fn last_read(&self) -> super::vals::LastRead {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LastRead::from_bits(val as u8)
    }
    #[doc = "Last Read"]
    #[inline(always)]
    pub const fn set_last_read(&mut self, val: super::vals::LastRead) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RdPathCtrlStatus {
    #[inline(always)]
    fn default() -> RdPathCtrlStatus {
        RdPathCtrlStatus(0)
    }
}
impl core::fmt::Debug for RdPathCtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdPathCtrlStatus")
            .field("rd_capt", &self.rd_capt())
            .field("se_size", &self.se_size())
            .field("ecc_enableb", &self.ecc_enableb())
            .field("misr_en", &self.misr_en())
            .field("cpy_par_en", &self.cpy_par_en())
            .field("bist_mux_to_smw", &self.bist_mux_to_smw())
            .field("ad_set", &self.ad_set())
            .field("wr_path_en", &self.wr_path_en())
            .field("wr_path_ecc_en", &self.wr_path_ecc_en())
            .field("dberr_reg", &self.dberr_reg())
            .field("sberr_reg", &self.sberr_reg())
            .field("cpy_phrase_en", &self.cpy_phrase_en())
            .field("smw_array1_smw0_sel", &self.smw_array1_smw0_sel())
            .field("bist_ecc_en", &self.bist_ecc_en())
            .field("last_read", &self.last_read())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdPathCtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RdPathCtrlStatus {{ rd_capt: {=u8:?}, se_size: {=u8:?}, ecc_enableb: {:?}, misr_en: {:?}, cpy_par_en: {:?}, bist_mux_to_smw: {:?}, ad_set: {=u8:?}, wr_path_en: {:?}, wr_path_ecc_en: {:?}, dberr_reg: {:?}, sberr_reg: {:?}, cpy_phrase_en: {:?}, smw_array1_smw0_sel: {:?}, bist_ecc_en: {:?}, last_read: {:?} }}",
            self.rd_capt(),
            self.se_size(),
            self.ecc_enableb(),
            self.misr_en(),
            self.cpy_par_en(),
            self.bist_mux_to_smw(),
            self.ad_set(),
            self.wr_path_en(),
            self.wr_path_ecc_en(),
            self.dberr_reg(),
            self.sberr_reg(),
            self.cpy_phrase_en(),
            self.smw_array1_smw0_sel(),
            self.bist_ecc_en(),
            self.last_read()
        )
    }
}
#[doc = "FMU Repair 0 Block 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair00(pub u32);
impl Repair00 {
    #[doc = "RDIS0_0"]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_0(&self) -> super::vals::Repair00Rdis00 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Repair00Rdis00::from_bits(val as u8)
    }
    #[doc = "RDIS0_0"]
    #[inline(always)]
    pub const fn set_rdis0_0(&mut self, val: super::vals::Repair00Rdis00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR0_0"]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR0_0"]
    #[inline(always)]
    pub const fn set_radr0_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair00 {
    #[inline(always)]
    fn default() -> Repair00 {
        Repair00(0)
    }
}
impl core::fmt::Debug for Repair00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair00")
            .field("rdis0_0", &self.rdis0_0())
            .field("radr0_0", &self.radr0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair00 {{ rdis0_0: {:?}, radr0_0: {=u8:?} }}",
            self.rdis0_0(),
            self.radr0_0()
        )
    }
}
#[doc = "FMU Repair 1 Block 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair01(pub u32);
impl Repair01 {
    #[doc = "RDIS0_1"]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_1(&self) -> super::vals::Repair01Rdis01 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Repair01Rdis01::from_bits(val as u8)
    }
    #[doc = "RDIS0_1"]
    #[inline(always)]
    pub const fn set_rdis0_1(&mut self, val: super::vals::Repair01Rdis01) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR0_1"]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR0_1"]
    #[inline(always)]
    pub const fn set_radr0_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair01 {
    #[inline(always)]
    fn default() -> Repair01 {
        Repair01(0)
    }
}
impl core::fmt::Debug for Repair01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair01")
            .field("rdis0_1", &self.rdis0_1())
            .field("radr0_1", &self.radr0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair01 {{ rdis0_1: {:?}, radr0_1: {=u8:?} }}",
            self.rdis0_1(),
            self.radr0_1()
        )
    }
}
#[doc = "FMU Repair 0 Block 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair10(pub u32);
impl Repair10 {
    #[doc = "RDIS1_0"]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_0(&self) -> super::vals::Repair10Rdis10 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Repair10Rdis10::from_bits(val as u8)
    }
    #[doc = "RDIS1_0"]
    #[inline(always)]
    pub const fn set_rdis1_0(&mut self, val: super::vals::Repair10Rdis10) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR1_0"]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR1_0"]
    #[inline(always)]
    pub const fn set_radr1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair10 {
    #[inline(always)]
    fn default() -> Repair10 {
        Repair10(0)
    }
}
impl core::fmt::Debug for Repair10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair10")
            .field("rdis1_0", &self.rdis1_0())
            .field("radr1_0", &self.radr1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair10 {{ rdis1_0: {:?}, radr1_0: {=u8:?} }}",
            self.rdis1_0(),
            self.radr1_0()
        )
    }
}
#[doc = "FMU Repair 1 Block 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair11(pub u32);
impl Repair11 {
    #[doc = "RDIS1_1"]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_1(&self) -> super::vals::Repair11Rdis11 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Repair11Rdis11::from_bits(val as u8)
    }
    #[doc = "RDIS1_1"]
    #[inline(always)]
    pub const fn set_rdis1_1(&mut self, val: super::vals::Repair11Rdis11) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR1_1"]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR1_1"]
    #[inline(always)]
    pub const fn set_radr1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair11 {
    #[inline(always)]
    fn default() -> Repair11 {
        Repair11(0)
    }
}
impl core::fmt::Debug for Repair11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair11")
            .field("rdis1_1", &self.rdis1_1())
            .field("radr1_1", &self.radr1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair11 {{ rdis1_1: {:?}, radr1_1: {=u8:?} }}",
            self.rdis1_1(),
            self.radr1_1()
        )
    }
}
#[doc = "FMU Initialization Tracking Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetStatus(pub u32);
impl ResetStatus {
    #[doc = "Array Trim Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn ary_trim_done(&self) -> super::vals::AryTrimDone {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AryTrimDone::from_bits(val as u8)
    }
    #[doc = "Array Trim Complete"]
    #[inline(always)]
    pub const fn set_ary_trim_done(&mut self, val: super::vals::AryTrimDone) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_parm_en(&self) -> super::vals::FmuParmEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::FmuParmEn::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the FMU parameters"]
    #[inline(always)]
    pub const fn set_fmu_parm_en(&mut self, val: super::vals::FmuParmEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FMU Register Load Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_parm_done(&self) -> super::vals::FmuParmDone {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FmuParmDone::from_bits(val as u8)
    }
    #[doc = "FMU Register Load Complete"]
    #[inline(always)]
    pub const fn set_fmu_parm_done(&mut self, val: super::vals::FmuParmDone) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_en(&self) -> super::vals::SocTrimEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::SocTrimEn::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the SoC trim settings"]
    #[inline(always)]
    pub const fn set_soc_trim_en(&mut self, val: super::vals::SocTrimEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_ecc(&self) -> super::vals::SocTrimEcc {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::SocTrimEcc::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings"]
    #[inline(always)]
    pub const fn set_soc_trim_ecc(&mut self, val: super::vals::SocTrimEcc) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SoC Trim Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_done(&self) -> super::vals::SocTrimDone {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::SocTrimDone::from_bits(val as u8)
    }
    #[doc = "SoC Trim Complete"]
    #[inline(always)]
    pub const fn set_soc_trim_done(&mut self, val: super::vals::SocTrimDone) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Repair Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn rpr_done(&self) -> super::vals::RprDone {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::RprDone::from_bits(val as u8)
    }
    #[doc = "Array Repair Complete"]
    #[inline(always)]
    pub const fn set_rpr_done(&mut self, val: super::vals::RprDone) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Initialization Done"]
    #[must_use]
    #[inline(always)]
    pub const fn init_done(&self) -> super::vals::InitDone {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::InitDone::from_bits(val as u8)
    }
    #[doc = "Initialization Done"]
    #[inline(always)]
    pub const fn set_init_done(&mut self, val: super::vals::InitDone) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "ECC Single Fault during Reset Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_sf_err(&self) -> super::vals::RstSfErr {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RstSfErr::from_bits(val as u8)
    }
    #[doc = "ECC Single Fault during Reset Recovery"]
    #[inline(always)]
    pub const fn set_rst_sf_err(&mut self, val: super::vals::RstSfErr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "ECC Double Fault during Reset Recovery"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_df_err(&self) -> super::vals::RstDfErr {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RstDfErr::from_bits(val as u8)
    }
    #[doc = "ECC Double Fault during Reset Recovery"]
    #[inline(always)]
    pub const fn set_rst_df_err(&mut self, val: super::vals::RstDfErr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "ECC Double Fault during load of SoC Trim phrases"]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_df_err(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0xff;
        val as u8
    }
    #[doc = "ECC Double Fault during load of SoC Trim phrases"]
    #[inline(always)]
    pub const fn set_soc_trim_df_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
    }
    #[doc = "Reset Patch Required"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_patch_ld(&self) -> super::vals::RstPatchLd {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::RstPatchLd::from_bits(val as u8)
    }
    #[doc = "Reset Patch Required"]
    #[inline(always)]
    pub const fn set_rst_patch_ld(&mut self, val: super::vals::RstPatchLd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Recall Data Mismatch"]
    #[must_use]
    #[inline(always)]
    pub const fn recall_data_mismatch(&self) -> super::vals::RecallDataMismatch {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::RecallDataMismatch::from_bits(val as u8)
    }
    #[doc = "Recall Data Mismatch"]
    #[inline(always)]
    pub const fn set_recall_data_mismatch(&mut self, val: super::vals::RecallDataMismatch) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for ResetStatus {
    #[inline(always)]
    fn default() -> ResetStatus {
        ResetStatus(0)
    }
}
impl core::fmt::Debug for ResetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ResetStatus")
            .field("ary_trim_done", &self.ary_trim_done())
            .field("fmu_parm_en", &self.fmu_parm_en())
            .field("fmu_parm_done", &self.fmu_parm_done())
            .field("soc_trim_en", &self.soc_trim_en())
            .field("soc_trim_ecc", &self.soc_trim_ecc())
            .field("soc_trim_done", &self.soc_trim_done())
            .field("rpr_done", &self.rpr_done())
            .field("init_done", &self.init_done())
            .field("rst_sf_err", &self.rst_sf_err())
            .field("rst_df_err", &self.rst_df_err())
            .field("soc_trim_df_err", &self.soc_trim_df_err())
            .field("rst_patch_ld", &self.rst_patch_ld())
            .field("recall_data_mismatch", &self.recall_data_mismatch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ResetStatus {{ ary_trim_done: {:?}, fmu_parm_en: {:?}, fmu_parm_done: {:?}, soc_trim_en: {:?}, soc_trim_ecc: {:?}, soc_trim_done: {:?}, rpr_done: {:?}, init_done: {:?}, rst_sf_err: {:?}, rst_df_err: {:?}, soc_trim_df_err: {=u8:?}, rst_patch_ld: {:?}, recall_data_mismatch: {:?} }}",
            self.ary_trim_done(),
            self.fmu_parm_en(),
            self.fmu_parm_done(),
            self.soc_trim_en(),
            self.soc_trim_ecc(),
            self.soc_trim_done(),
            self.rpr_done(),
            self.init_done(),
            self.rst_sf_err(),
            self.rst_df_err(),
            self.soc_trim_df_err(),
            self.rst_patch_ld(),
            self.recall_data_mismatch()
        )
    }
}
#[doc = "SMW Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwAddr(pub u32);
impl SmwAddr {
    #[doc = "SMW Address"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW Address"]
    #[inline(always)]
    pub const fn set_smw_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwAddr {
    #[inline(always)]
    fn default() -> SmwAddr {
        SmwAddr(0)
    }
}
impl core::fmt::Debug for SmwAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwAddr")
            .field("smw_addr", &self.smw_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwAddr {{ smw_addr: {=u32:?} }}", self.smw_addr())
    }
}
#[doc = "SMW Command and Wait Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwCmdWait(pub u32);
impl SmwCmdWait {
    #[doc = "SMW Command"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> super::vals::Cmd {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Cmd::from_bits(val as u8)
    }
    #[doc = "SMW Command"]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SMW Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_en(&self) -> super::vals::WaitEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::WaitEn::from_bits(val as u8)
    }
    #[doc = "SMW Wait Enable"]
    #[inline(always)]
    pub const fn set_wait_en(&mut self, val: super::vals::WaitEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SMW Wait Auto Set"]
    #[must_use]
    #[inline(always)]
    pub const fn wait_auto_set(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SMW Wait Auto Set"]
    #[inline(always)]
    pub const fn set_wait_auto_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SmwCmdWait {
    #[inline(always)]
    fn default() -> SmwCmdWait {
        SmwCmdWait(0)
    }
}
impl core::fmt::Debug for SmwCmdWait {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwCmdWait")
            .field("cmd", &self.cmd())
            .field("wait_en", &self.wait_en())
            .field("wait_auto_set", &self.wait_auto_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwCmdWait {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwCmdWait {{ cmd: {:?}, wait_en: {:?}, wait_auto_set: {=bool:?} }}",
            self.cmd(),
            self.wait_en(),
            self.wait_auto_set()
        )
    }
}
#[doc = "SMW DIN 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin0(pub u32);
impl SmwDin0 {
    #[doc = "SMW DIN 0"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 0"]
    #[inline(always)]
    pub const fn set_smw_din0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin0 {
    #[inline(always)]
    fn default() -> SmwDin0 {
        SmwDin0(0)
    }
}
impl core::fmt::Debug for SmwDin0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin0")
            .field("smw_din0", &self.smw_din0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin0 {{ smw_din0: {=u32:?} }}", self.smw_din0())
    }
}
#[doc = "SMW DIN 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin1(pub u32);
impl SmwDin1 {
    #[doc = "SMW DIN 1"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 1"]
    #[inline(always)]
    pub const fn set_smw_din1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin1 {
    #[inline(always)]
    fn default() -> SmwDin1 {
        SmwDin1(0)
    }
}
impl core::fmt::Debug for SmwDin1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin1")
            .field("smw_din1", &self.smw_din1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin1 {{ smw_din1: {=u32:?} }}", self.smw_din1())
    }
}
#[doc = "SMW DIN 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin2(pub u32);
impl SmwDin2 {
    #[doc = "SMW DIN 2"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 2"]
    #[inline(always)]
    pub const fn set_smw_din2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin2 {
    #[inline(always)]
    fn default() -> SmwDin2 {
        SmwDin2(0)
    }
}
impl core::fmt::Debug for SmwDin2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin2")
            .field("smw_din2", &self.smw_din2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin2 {{ smw_din2: {=u32:?} }}", self.smw_din2())
    }
}
#[doc = "SMW DIN 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin3(pub u32);
impl SmwDin3 {
    #[doc = "SMW DIN 3"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 3"]
    #[inline(always)]
    pub const fn set_smw_din3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin3 {
    #[inline(always)]
    fn default() -> SmwDin3 {
        SmwDin3(0)
    }
}
impl core::fmt::Debug for SmwDin3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin3")
            .field("smw_din3", &self.smw_din3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin3 {{ smw_din3: {=u32:?} }}", self.smw_din3())
    }
}
#[doc = "SMW HB Signals Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwHbSignals(pub u32);
impl SmwHbSignals {
    #[doc = "SMW Region Select"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_array(&self) -> super::vals::SmwArray {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SmwArray::from_bits(val as u8)
    }
    #[doc = "SMW Region Select"]
    #[inline(always)]
    pub const fn set_smw_array(&mut self, val: super::vals::SmwArray) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "IFR1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn user_ifren1(&self) -> super::vals::UserIfren1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::UserIfren1::from_bits(val as u8)
    }
    #[doc = "IFR1 Enable"]
    #[inline(always)]
    pub const fn set_user_ifren1(&mut self, val: super::vals::UserIfren1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Program Verify"]
    #[must_use]
    #[inline(always)]
    pub const fn user_pv(&self) -> super::vals::UserPv {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::UserPv::from_bits(val as u8)
    }
    #[doc = "Program Verify"]
    #[inline(always)]
    pub const fn set_user_pv(&mut self, val: super::vals::UserPv) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Erase Verify"]
    #[must_use]
    #[inline(always)]
    pub const fn user_ev(&self) -> super::vals::UserEv {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::UserEv::from_bits(val as u8)
    }
    #[doc = "Erase Verify"]
    #[inline(always)]
    pub const fn set_user_ev(&mut self, val: super::vals::UserEv) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IFR Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn user_ifren(&self) -> super::vals::UserIfren {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::UserIfren::from_bits(val as u8)
    }
    #[doc = "IFR Enable"]
    #[inline(always)]
    pub const fn set_user_ifren(&mut self, val: super::vals::UserIfren) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Repair Read Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn user_reden(&self) -> super::vals::UserReden {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::UserReden::from_bits(val as u8)
    }
    #[doc = "Repair Read Enable"]
    #[inline(always)]
    pub const fn set_user_reden(&mut self, val: super::vals::UserReden) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "High Endurance Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn user_hem(&self) -> super::vals::UserHem {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::UserHem::from_bits(val as u8)
    }
    #[doc = "High Endurance Enable"]
    #[inline(always)]
    pub const fn set_user_hem(&mut self, val: super::vals::UserHem) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for SmwHbSignals {
    #[inline(always)]
    fn default() -> SmwHbSignals {
        SmwHbSignals(0)
    }
}
impl core::fmt::Debug for SmwHbSignals {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwHbSignals")
            .field("smw_array", &self.smw_array())
            .field("user_ifren1", &self.user_ifren1())
            .field("user_pv", &self.user_pv())
            .field("user_ev", &self.user_ev())
            .field("user_ifren", &self.user_ifren())
            .field("user_reden", &self.user_reden())
            .field("user_hem", &self.user_hem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwHbSignals {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwHbSignals {{ smw_array: {:?}, user_ifren1: {:?}, user_pv: {:?}, user_ev: {:?}, user_ifren: {:?}, user_reden: {:?}, user_hem: {:?} }}",
            self.smw_array(),
            self.user_ifren1(),
            self.user_pv(),
            self.user_ev(),
            self.user_ifren(),
            self.user_reden(),
            self.user_hem()
        )
    }
}
#[doc = "SMW Setting Option 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption0(pub u32);
impl SmwSettingOption0 {
    #[doc = "Medium Voltage Level Select Initial"]
    #[must_use]
    #[inline(always)]
    pub const fn mv_init(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level Select Initial"]
    #[inline(always)]
    pub const fn set_mv_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "Medium Voltage Level Select Final"]
    #[must_use]
    #[inline(always)]
    pub const fn mv_end(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level Select Final"]
    #[inline(always)]
    pub const fn set_mv_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Medium Voltage Control Misc"]
    #[must_use]
    #[inline(always)]
    pub const fn mv_misc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Medium Voltage Control Misc"]
    #[inline(always)]
    pub const fn set_mv_misc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Program Current Control Initial"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_init(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current Control Initial"]
    #[inline(always)]
    pub const fn set_ipgm_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Program Current Control Final"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_end(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current Control Final"]
    #[inline(always)]
    pub const fn set_ipgm_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Program Current Control Misc"]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_misc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Program Current Control Misc"]
    #[inline(always)]
    pub const fn set_ipgm_misc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SmwSettingOption0 {
    #[inline(always)]
    fn default() -> SmwSettingOption0 {
        SmwSettingOption0(0)
    }
}
impl core::fmt::Debug for SmwSettingOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption0")
            .field("mv_init", &self.mv_init())
            .field("mv_end", &self.mv_end())
            .field("mv_misc", &self.mv_misc())
            .field("ipgm_init", &self.ipgm_init())
            .field("ipgm_end", &self.ipgm_end())
            .field("ipgm_misc", &self.ipgm_misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption0 {{ mv_init: {=u8:?}, mv_end: {=u8:?}, mv_misc: {=u8:?}, ipgm_init: {=u8:?}, ipgm_end: {=u8:?}, ipgm_misc: {=u8:?} }}",
            self.mv_init(),
            self.mv_end(),
            self.mv_misc(),
            self.ipgm_init(),
            self.ipgm_end(),
            self.ipgm_misc()
        )
    }
}
#[doc = "SMW Setting Option 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption1(pub u32);
impl SmwSettingOption1 {
    #[doc = "Ters Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ters_ctrl0(&self) -> super::vals::TersCtrl0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::TersCtrl0::from_bits(val as u8)
    }
    #[doc = "Ters Control"]
    #[inline(always)]
    pub const fn set_ters_ctrl0(&mut self, val: super::vals::TersCtrl0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Tpgm Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgm_ctrl(&self) -> super::vals::TpgmCtrl {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::TpgmCtrl::from_bits(val as u8)
    }
    #[doc = "Tpgm Control"]
    #[inline(always)]
    pub const fn set_tpgm_ctrl(&mut self, val: super::vals::TpgmCtrl) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Tnvs Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvs_ctrl(&self) -> super::vals::TnvsCtrl {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::TnvsCtrl::from_bits(val as u8)
    }
    #[doc = "Tnvs Control"]
    #[inline(always)]
    pub const fn set_tnvs_ctrl(&mut self, val: super::vals::TnvsCtrl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Tnvh Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tnvh_ctrl(&self) -> super::vals::TnvhCtrl {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::TnvhCtrl::from_bits(val as u8)
    }
    #[doc = "Tnvh Control"]
    #[inline(always)]
    pub const fn set_tnvh_ctrl(&mut self, val: super::vals::TnvhCtrl) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Tpgs Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgs_ctrl(&self) -> super::vals::TpgsCtrl {
        let val = (self.0 >> 11usize) & 0x07;
        super::vals::TpgsCtrl::from_bits(val as u8)
    }
    #[doc = "Tpgs Control"]
    #[inline(always)]
    pub const fn set_tpgs_ctrl(&mut self, val: super::vals::TpgsCtrl) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Number of Erase Shots"]
    #[must_use]
    #[inline(always)]
    pub const fn max_erase(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of Erase Shots"]
    #[inline(always)]
    pub const fn set_max_erase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 14usize)) | (((val as u32) & 0x01ff) << 14usize);
    }
    #[doc = "Number of Program Shots"]
    #[must_use]
    #[inline(always)]
    pub const fn max_prog(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Program Shots"]
    #[inline(always)]
    pub const fn set_max_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
}
impl Default for SmwSettingOption1 {
    #[inline(always)]
    fn default() -> SmwSettingOption1 {
        SmwSettingOption1(0)
    }
}
impl core::fmt::Debug for SmwSettingOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption1")
            .field("ters_ctrl0", &self.ters_ctrl0())
            .field("tpgm_ctrl", &self.tpgm_ctrl())
            .field("tnvs_ctrl", &self.tnvs_ctrl())
            .field("tnvh_ctrl", &self.tnvh_ctrl())
            .field("tpgs_ctrl", &self.tpgs_ctrl())
            .field("max_erase", &self.max_erase())
            .field("max_prog", &self.max_prog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption1 {{ ters_ctrl0: {:?}, tpgm_ctrl: {:?}, tnvs_ctrl: {:?}, tnvh_ctrl: {:?}, tpgs_ctrl: {:?}, max_erase: {=u16:?}, max_prog: {=u8:?} }}",
            self.ters_ctrl0(),
            self.tpgm_ctrl(),
            self.tnvs_ctrl(),
            self.tnvh_ctrl(),
            self.tpgs_ctrl(),
            self.max_erase(),
            self.max_prog()
        )
    }
}
#[doc = "SMW Setting Option 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption2(pub u32);
impl SmwSettingOption2 {
    #[doc = "Thvs control"]
    #[must_use]
    #[inline(always)]
    pub const fn thvs_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Thvs control"]
    #[inline(always)]
    pub const fn set_thvs_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Trcv Control"]
    #[must_use]
    #[inline(always)]
    pub const fn trcv_ctrl(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Trcv Control"]
    #[inline(always)]
    pub const fn set_trcv_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Number of Post Shots for SME"]
    #[must_use]
    #[inline(always)]
    pub const fn xtra_ers(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Post Shots for SME"]
    #[inline(always)]
    pub const fn set_xtra_ers(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Number of Post Shots for SMP"]
    #[must_use]
    #[inline(always)]
    pub const fn xtra_pgm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Post Shots for SMP"]
    #[inline(always)]
    pub const fn set_xtra_pgm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "WHV Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn whv_cntr(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0xff;
        val as u8
    }
    #[doc = "WHV Counter"]
    #[inline(always)]
    pub const fn set_whv_cntr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
    }
    #[doc = "Post Ters Time"]
    #[must_use]
    #[inline(always)]
    pub const fn post_ters(&self) -> super::vals::PostTers {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::PostTers::from_bits(val as u8)
    }
    #[doc = "Post Ters Time"]
    #[inline(always)]
    pub const fn set_post_ters(&mut self, val: super::vals::PostTers) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Post Tpgm Time"]
    #[must_use]
    #[inline(always)]
    pub const fn post_tpgm(&self) -> super::vals::PostTpgm {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::PostTpgm::from_bits(val as u8)
    }
    #[doc = "Post Tpgm Time"]
    #[inline(always)]
    pub const fn set_post_tpgm(&mut self, val: super::vals::PostTpgm) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Verify Option"]
    #[must_use]
    #[inline(always)]
    pub const fn vfy_opt(&self) -> super::vals::VfyOpt {
        let val = (self.0 >> 23usize) & 0x03;
        super::vals::VfyOpt::from_bits(val as u8)
    }
    #[doc = "Verify Option"]
    #[inline(always)]
    pub const fn set_vfy_opt(&mut self, val: super::vals::VfyOpt) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val.to_bits() as u32) & 0x03) << 23usize);
    }
    #[doc = "Tpgm Option"]
    #[must_use]
    #[inline(always)]
    pub const fn tpgm_opt(&self) -> super::vals::TpgmOpt {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::TpgmOpt::from_bits(val as u8)
    }
    #[doc = "Tpgm Option"]
    #[inline(always)]
    pub const fn set_tpgm_opt(&mut self, val: super::vals::TpgmOpt) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "MASK0_OPT"]
    #[must_use]
    #[inline(always)]
    pub const fn mask0_opt(&self) -> super::vals::Mask0Opt {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mask0Opt::from_bits(val as u8)
    }
    #[doc = "MASK0_OPT"]
    #[inline(always)]
    pub const fn set_mask0_opt(&mut self, val: super::vals::Mask0Opt) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Disable pre-PV Read before First Program Shot"]
    #[must_use]
    #[inline(always)]
    pub const fn dis_prer(&self) -> super::vals::DisPrer {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::DisPrer::from_bits(val as u8)
    }
    #[doc = "Disable pre-PV Read before First Program Shot"]
    #[inline(always)]
    pub const fn set_dis_prer(&mut self, val: super::vals::DisPrer) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for SmwSettingOption2 {
    #[inline(always)]
    fn default() -> SmwSettingOption2 {
        SmwSettingOption2(0)
    }
}
impl core::fmt::Debug for SmwSettingOption2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption2")
            .field("thvs_ctrl", &self.thvs_ctrl())
            .field("trcv_ctrl", &self.trcv_ctrl())
            .field("xtra_ers", &self.xtra_ers())
            .field("xtra_pgm", &self.xtra_pgm())
            .field("whv_cntr", &self.whv_cntr())
            .field("post_ters", &self.post_ters())
            .field("post_tpgm", &self.post_tpgm())
            .field("vfy_opt", &self.vfy_opt())
            .field("tpgm_opt", &self.tpgm_opt())
            .field("mask0_opt", &self.mask0_opt())
            .field("dis_prer", &self.dis_prer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption2 {{ thvs_ctrl: {=u8:?}, trcv_ctrl: {=u8:?}, xtra_ers: {=u8:?}, xtra_pgm: {=u8:?}, whv_cntr: {=u8:?}, post_ters: {:?}, post_tpgm: {:?}, vfy_opt: {:?}, tpgm_opt: {:?}, mask0_opt: {:?}, dis_prer: {:?} }}",
            self.thvs_ctrl(),
            self.trcv_ctrl(),
            self.xtra_ers(),
            self.xtra_pgm(),
            self.whv_cntr(),
            self.post_ters(),
            self.post_tpgm(),
            self.vfy_opt(),
            self.tpgm_opt(),
            self.mask0_opt(),
            self.dis_prer()
        )
    }
}
#[doc = "SMW Setting Option 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption3(pub u32);
impl SmwSettingOption3 {
    #[doc = "WHV_COUNTER for HEM-erase Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn hem_whv_cntr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "WHV_COUNTER for HEM-erase Cycle"]
    #[inline(always)]
    pub const fn set_hem_whv_cntr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "HEM Max Erase Shot Count"]
    #[must_use]
    #[inline(always)]
    pub const fn hem_max_ers(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "HEM Max Erase Shot Count"]
    #[inline(always)]
    pub const fn set_hem_max_ers(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
    }
}
impl Default for SmwSettingOption3 {
    #[inline(always)]
    fn default() -> SmwSettingOption3 {
        SmwSettingOption3(0)
    }
}
impl core::fmt::Debug for SmwSettingOption3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption3")
            .field("hem_whv_cntr", &self.hem_whv_cntr())
            .field("hem_max_ers", &self.hem_max_ers())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption3 {{ hem_whv_cntr: {=u8:?}, hem_max_ers: {=u16:?} }}",
            self.hem_whv_cntr(),
            self.hem_max_ers()
        )
    }
}
#[doc = "SMW SME WHV Option 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmeWhvOption0(pub u32);
impl SmwSmeWhvOption0 {
    #[doc = "Smart Erase WHV Option Low"]
    #[must_use]
    #[inline(always)]
    pub const fn sme_whv_opt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Erase WHV Option Low"]
    #[inline(always)]
    pub const fn set_sme_whv_opt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmeWhvOption0 {
    #[inline(always)]
    fn default() -> SmwSmeWhvOption0 {
        SmwSmeWhvOption0(0)
    }
}
impl core::fmt::Debug for SmwSmeWhvOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmeWhvOption0")
            .field("sme_whv_opt0", &self.sme_whv_opt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmeWhvOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmeWhvOption0 {{ sme_whv_opt0: {=u32:?} }}",
            self.sme_whv_opt0()
        )
    }
}
#[doc = "SMW SME WHV Option 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmeWhvOption1(pub u32);
impl SmwSmeWhvOption1 {
    #[doc = "Smart Erase WHV Option High"]
    #[must_use]
    #[inline(always)]
    pub const fn sme_whv_opt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Erase WHV Option High"]
    #[inline(always)]
    pub const fn set_sme_whv_opt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmeWhvOption1 {
    #[inline(always)]
    fn default() -> SmwSmeWhvOption1 {
        SmwSmeWhvOption1(0)
    }
}
impl core::fmt::Debug for SmwSmeWhvOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmeWhvOption1")
            .field("sme_whv_opt1", &self.sme_whv_opt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmeWhvOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmeWhvOption1 {{ sme_whv_opt1: {=u32:?} }}",
            self.sme_whv_opt1()
        )
    }
}
#[doc = "SMW SMP WHV Option 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmpWhvOption0(pub u32);
impl SmwSmpWhvOption0 {
    #[doc = "Smart Program WHV Option Low"]
    #[must_use]
    #[inline(always)]
    pub const fn smp_whv_opt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Program WHV Option Low"]
    #[inline(always)]
    pub const fn set_smp_whv_opt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmpWhvOption0 {
    #[inline(always)]
    fn default() -> SmwSmpWhvOption0 {
        SmwSmpWhvOption0(0)
    }
}
impl core::fmt::Debug for SmwSmpWhvOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmpWhvOption0")
            .field("smp_whv_opt0", &self.smp_whv_opt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmpWhvOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmpWhvOption0 {{ smp_whv_opt0: {=u32:?} }}",
            self.smp_whv_opt0()
        )
    }
}
#[doc = "SMW SMP WHV Option 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmpWhvOption1(pub u32);
impl SmwSmpWhvOption1 {
    #[doc = "Smart Program WHV Option High"]
    #[must_use]
    #[inline(always)]
    pub const fn smp_whv_opt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Program WHV Option High"]
    #[inline(always)]
    pub const fn set_smp_whv_opt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmpWhvOption1 {
    #[inline(always)]
    fn default() -> SmwSmpWhvOption1 {
        SmwSmpWhvOption1(0)
    }
}
impl core::fmt::Debug for SmwSmpWhvOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmpWhvOption1")
            .field("smp_whv_opt1", &self.smp_whv_opt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmpWhvOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmpWhvOption1 {{ smp_whv_opt1: {=u32:?} }}",
            self.smp_whv_opt1()
        )
    }
}
#[doc = "SMW Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwStatus(pub u32);
impl SmwStatus {
    #[doc = "SMW Error"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_err(&self) -> super::vals::SmwErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SmwErr::from_bits(val as u8)
    }
    #[doc = "SMW Error"]
    #[inline(always)]
    pub const fn set_smw_err(&mut self, val: super::vals::SmwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SMW Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_busy(&self) -> super::vals::SmwBusy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SmwBusy::from_bits(val as u8)
    }
    #[doc = "SMW Busy"]
    #[inline(always)]
    pub const fn set_smw_busy(&mut self, val: super::vals::SmwBusy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST Busy"]
    #[must_use]
    #[inline(always)]
    pub const fn bist_busy(&self) -> super::vals::BistBusy {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::BistBusy::from_bits(val as u8)
    }
    #[doc = "BIST Busy"]
    #[inline(always)]
    pub const fn set_bist_busy(&mut self, val: super::vals::BistBusy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for SmwStatus {
    #[inline(always)]
    fn default() -> SmwStatus {
        SmwStatus(0)
    }
}
impl core::fmt::Debug for SmwStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwStatus")
            .field("smw_err", &self.smw_err())
            .field("smw_busy", &self.smw_busy())
            .field("bist_busy", &self.bist_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwStatus {{ smw_err: {:?}, smw_busy: {:?}, bist_busy: {:?} }}",
            self.smw_err(),
            self.smw_busy(),
            self.bist_busy()
        )
    }
}
#[doc = "SMW Timer Option Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwTimerOption(pub u32);
impl SmwTimerOption {
    #[doc = "Clock Divide Scalar for Long Pulse"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_cdivl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divide Scalar for Long Pulse"]
    #[inline(always)]
    pub const fn set_smw_cdivl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Adjust for Verify"]
    #[must_use]
    #[inline(always)]
    pub const fn smw_tvfy(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Adjust for Verify"]
    #[inline(always)]
    pub const fn set_smw_tvfy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for SmwTimerOption {
    #[inline(always)]
    fn default() -> SmwTimerOption {
        SmwTimerOption(0)
    }
}
impl core::fmt::Debug for SmwTimerOption {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwTimerOption")
            .field("smw_cdivl", &self.smw_cdivl())
            .field("smw_tvfy", &self.smw_tvfy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwTimerOption {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwTimerOption {{ smw_cdivl: {=u8:?}, smw_tvfy: {=u8:?} }}",
            self.smw_cdivl(),
            self.smw_tvfy()
        )
    }
}
#[doc = "SoC Trim Phrase 0 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim00(pub u32);
impl Soctrim00 {
    #[doc = "TRIM0_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_0"]
    #[inline(always)]
    pub const fn set_trim0_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim00 {
    #[inline(always)]
    fn default() -> Soctrim00 {
        Soctrim00(0)
    }
}
impl core::fmt::Debug for Soctrim00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim00")
            .field("trim0_0", &self.trim0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim00 {{ trim0_0: {=u32:?} }}", self.trim0_0())
    }
}
#[doc = "SoC Trim Phrase 0 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim01(pub u32);
impl Soctrim01 {
    #[doc = "TRIM0_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_1"]
    #[inline(always)]
    pub const fn set_trim0_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim01 {
    #[inline(always)]
    fn default() -> Soctrim01 {
        Soctrim01(0)
    }
}
impl core::fmt::Debug for Soctrim01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim01")
            .field("trim0_1", &self.trim0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim01 {{ trim0_1: {=u32:?} }}", self.trim0_1())
    }
}
#[doc = "SoC Trim Phrase 0 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim02(pub u32);
impl Soctrim02 {
    #[doc = "TRIM0_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_2"]
    #[inline(always)]
    pub const fn set_trim0_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim02 {
    #[inline(always)]
    fn default() -> Soctrim02 {
        Soctrim02(0)
    }
}
impl core::fmt::Debug for Soctrim02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim02")
            .field("trim0_2", &self.trim0_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim02 {{ trim0_2: {=u32:?} }}", self.trim0_2())
    }
}
#[doc = "SoC Trim Phrase 0 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim03(pub u32);
impl Soctrim03 {
    #[doc = "TRIM0_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_3"]
    #[inline(always)]
    pub const fn set_trim0_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim03 {
    #[inline(always)]
    fn default() -> Soctrim03 {
        Soctrim03(0)
    }
}
impl core::fmt::Debug for Soctrim03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim03")
            .field("trim0_3", &self.trim0_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim03 {{ trim0_3: {=u32:?} }}", self.trim0_3())
    }
}
#[doc = "SoC Trim Phrase 1 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim10(pub u32);
impl Soctrim10 {
    #[doc = "TRIM1_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_0"]
    #[inline(always)]
    pub const fn set_trim1_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim10 {
    #[inline(always)]
    fn default() -> Soctrim10 {
        Soctrim10(0)
    }
}
impl core::fmt::Debug for Soctrim10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim10")
            .field("trim1_0", &self.trim1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim10 {{ trim1_0: {=u32:?} }}", self.trim1_0())
    }
}
#[doc = "SoC Trim Phrase 1 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim11(pub u32);
impl Soctrim11 {
    #[doc = "TRIM1_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_1"]
    #[inline(always)]
    pub const fn set_trim1_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim11 {
    #[inline(always)]
    fn default() -> Soctrim11 {
        Soctrim11(0)
    }
}
impl core::fmt::Debug for Soctrim11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim11")
            .field("trim1_1", &self.trim1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim11 {{ trim1_1: {=u32:?} }}", self.trim1_1())
    }
}
#[doc = "SoC Trim Phrase 1 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim12(pub u32);
impl Soctrim12 {
    #[doc = "TRIM1_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_2"]
    #[inline(always)]
    pub const fn set_trim1_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim12 {
    #[inline(always)]
    fn default() -> Soctrim12 {
        Soctrim12(0)
    }
}
impl core::fmt::Debug for Soctrim12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim12")
            .field("trim1_2", &self.trim1_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim12 {{ trim1_2: {=u32:?} }}", self.trim1_2())
    }
}
#[doc = "SoC Trim Phrase 1 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim13(pub u32);
impl Soctrim13 {
    #[doc = "TRIM1_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_3"]
    #[inline(always)]
    pub const fn set_trim1_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim13 {
    #[inline(always)]
    fn default() -> Soctrim13 {
        Soctrim13(0)
    }
}
impl core::fmt::Debug for Soctrim13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim13")
            .field("trim1_3", &self.trim1_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim13 {{ trim1_3: {=u32:?} }}", self.trim1_3())
    }
}
#[doc = "SoC Trim Phrase 2 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim20(pub u32);
impl Soctrim20 {
    #[doc = "TRIM2_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_0"]
    #[inline(always)]
    pub const fn set_trim2_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim20 {
    #[inline(always)]
    fn default() -> Soctrim20 {
        Soctrim20(0)
    }
}
impl core::fmt::Debug for Soctrim20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim20")
            .field("trim2_0", &self.trim2_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim20 {{ trim2_0: {=u32:?} }}", self.trim2_0())
    }
}
#[doc = "SoC Trim Phrase 2 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim21(pub u32);
impl Soctrim21 {
    #[doc = "TRIM2_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_1"]
    #[inline(always)]
    pub const fn set_trim2_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim21 {
    #[inline(always)]
    fn default() -> Soctrim21 {
        Soctrim21(0)
    }
}
impl core::fmt::Debug for Soctrim21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim21")
            .field("trim2_1", &self.trim2_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim21 {{ trim2_1: {=u32:?} }}", self.trim2_1())
    }
}
#[doc = "SoC Trim Phrase 2 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim22(pub u32);
impl Soctrim22 {
    #[doc = "TRIM2_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_2"]
    #[inline(always)]
    pub const fn set_trim2_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim22 {
    #[inline(always)]
    fn default() -> Soctrim22 {
        Soctrim22(0)
    }
}
impl core::fmt::Debug for Soctrim22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim22")
            .field("trim2_2", &self.trim2_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim22 {{ trim2_2: {=u32:?} }}", self.trim2_2())
    }
}
#[doc = "SoC Trim Phrase 2 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim23(pub u32);
impl Soctrim23 {
    #[doc = "TRIM2_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_3"]
    #[inline(always)]
    pub const fn set_trim2_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim23 {
    #[inline(always)]
    fn default() -> Soctrim23 {
        Soctrim23(0)
    }
}
impl core::fmt::Debug for Soctrim23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim23")
            .field("trim2_3", &self.trim2_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim23 {{ trim2_3: {=u32:?} }}", self.trim2_3())
    }
}
#[doc = "SoC Trim Phrase 3 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim30(pub u32);
impl Soctrim30 {
    #[doc = "TRIM3_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_0"]
    #[inline(always)]
    pub const fn set_trim3_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim30 {
    #[inline(always)]
    fn default() -> Soctrim30 {
        Soctrim30(0)
    }
}
impl core::fmt::Debug for Soctrim30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim30")
            .field("trim3_0", &self.trim3_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim30 {{ trim3_0: {=u32:?} }}", self.trim3_0())
    }
}
#[doc = "SoC Trim Phrase 3 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim31(pub u32);
impl Soctrim31 {
    #[doc = "TRIM3_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_1"]
    #[inline(always)]
    pub const fn set_trim3_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim31 {
    #[inline(always)]
    fn default() -> Soctrim31 {
        Soctrim31(0)
    }
}
impl core::fmt::Debug for Soctrim31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim31")
            .field("trim3_1", &self.trim3_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim31 {{ trim3_1: {=u32:?} }}", self.trim3_1())
    }
}
#[doc = "SoC Trim Phrase 3 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim32(pub u32);
impl Soctrim32 {
    #[doc = "TRIM3_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_2"]
    #[inline(always)]
    pub const fn set_trim3_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim32 {
    #[inline(always)]
    fn default() -> Soctrim32 {
        Soctrim32(0)
    }
}
impl core::fmt::Debug for Soctrim32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim32")
            .field("trim3_2", &self.trim3_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim32 {{ trim3_2: {=u32:?} }}", self.trim3_2())
    }
}
#[doc = "SoC Trim Phrase 3 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim33(pub u32);
impl Soctrim33 {
    #[doc = "TRIM3_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_3"]
    #[inline(always)]
    pub const fn set_trim3_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim33 {
    #[inline(always)]
    fn default() -> Soctrim33 {
        Soctrim33(0)
    }
}
impl core::fmt::Debug for Soctrim33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim33")
            .field("trim3_3", &self.trim3_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim33 {{ trim3_3: {=u32:?} }}", self.trim3_3())
    }
}
#[doc = "SoC Trim Phrase 4 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim40(pub u32);
impl Soctrim40 {
    #[doc = "TRIM4_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_0"]
    #[inline(always)]
    pub const fn set_trim4_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim40 {
    #[inline(always)]
    fn default() -> Soctrim40 {
        Soctrim40(0)
    }
}
impl core::fmt::Debug for Soctrim40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim40")
            .field("trim4_0", &self.trim4_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim40 {{ trim4_0: {=u32:?} }}", self.trim4_0())
    }
}
#[doc = "SoC Trim Phrase 4 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim41(pub u32);
impl Soctrim41 {
    #[doc = "TRIM4_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_1"]
    #[inline(always)]
    pub const fn set_trim4_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim41 {
    #[inline(always)]
    fn default() -> Soctrim41 {
        Soctrim41(0)
    }
}
impl core::fmt::Debug for Soctrim41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim41")
            .field("trim4_1", &self.trim4_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim41 {{ trim4_1: {=u32:?} }}", self.trim4_1())
    }
}
#[doc = "SoC Trim Phrase 4 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim42(pub u32);
impl Soctrim42 {
    #[doc = "TRIM4_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_2"]
    #[inline(always)]
    pub const fn set_trim4_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim42 {
    #[inline(always)]
    fn default() -> Soctrim42 {
        Soctrim42(0)
    }
}
impl core::fmt::Debug for Soctrim42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim42")
            .field("trim4_2", &self.trim4_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim42 {{ trim4_2: {=u32:?} }}", self.trim4_2())
    }
}
#[doc = "SoC Trim Phrase 4 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim43(pub u32);
impl Soctrim43 {
    #[doc = "TRIM4_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_3"]
    #[inline(always)]
    pub const fn set_trim4_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim43 {
    #[inline(always)]
    fn default() -> Soctrim43 {
        Soctrim43(0)
    }
}
impl core::fmt::Debug for Soctrim43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim43")
            .field("trim4_3", &self.trim4_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim43 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim43 {{ trim4_3: {=u32:?} }}", self.trim4_3())
    }
}
#[doc = "SoC Trim Phrase 5 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim50(pub u32);
impl Soctrim50 {
    #[doc = "TRIM5_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_0"]
    #[inline(always)]
    pub const fn set_trim5_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim50 {
    #[inline(always)]
    fn default() -> Soctrim50 {
        Soctrim50(0)
    }
}
impl core::fmt::Debug for Soctrim50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim50")
            .field("trim5_0", &self.trim5_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim50 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim50 {{ trim5_0: {=u32:?} }}", self.trim5_0())
    }
}
#[doc = "SoC Trim Phrase 5 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim51(pub u32);
impl Soctrim51 {
    #[doc = "TRIM5_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_1"]
    #[inline(always)]
    pub const fn set_trim5_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim51 {
    #[inline(always)]
    fn default() -> Soctrim51 {
        Soctrim51(0)
    }
}
impl core::fmt::Debug for Soctrim51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim51")
            .field("trim5_1", &self.trim5_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim51 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim51 {{ trim5_1: {=u32:?} }}", self.trim5_1())
    }
}
#[doc = "SoC Trim Phrase 5 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim52(pub u32);
impl Soctrim52 {
    #[doc = "TRIM5_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_2"]
    #[inline(always)]
    pub const fn set_trim5_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim52 {
    #[inline(always)]
    fn default() -> Soctrim52 {
        Soctrim52(0)
    }
}
impl core::fmt::Debug for Soctrim52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim52")
            .field("trim5_2", &self.trim5_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim52 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim52 {{ trim5_2: {=u32:?} }}", self.trim5_2())
    }
}
#[doc = "SoC Trim Phrase 5 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim53(pub u32);
impl Soctrim53 {
    #[doc = "TRIM5_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_3"]
    #[inline(always)]
    pub const fn set_trim5_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim53 {
    #[inline(always)]
    fn default() -> Soctrim53 {
        Soctrim53(0)
    }
}
impl core::fmt::Debug for Soctrim53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim53")
            .field("trim5_3", &self.trim5_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim53 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim53 {{ trim5_3: {=u32:?} }}", self.trim5_3())
    }
}
#[doc = "SoC Trim Phrase 6 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim60(pub u32);
impl Soctrim60 {
    #[doc = "TRIM6_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_0"]
    #[inline(always)]
    pub const fn set_trim6_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim60 {
    #[inline(always)]
    fn default() -> Soctrim60 {
        Soctrim60(0)
    }
}
impl core::fmt::Debug for Soctrim60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim60")
            .field("trim6_0", &self.trim6_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim60 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim60 {{ trim6_0: {=u32:?} }}", self.trim6_0())
    }
}
#[doc = "SoC Trim Phrase 6 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim61(pub u32);
impl Soctrim61 {
    #[doc = "TRIM6_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_1"]
    #[inline(always)]
    pub const fn set_trim6_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim61 {
    #[inline(always)]
    fn default() -> Soctrim61 {
        Soctrim61(0)
    }
}
impl core::fmt::Debug for Soctrim61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim61")
            .field("trim6_1", &self.trim6_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim61 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim61 {{ trim6_1: {=u32:?} }}", self.trim6_1())
    }
}
#[doc = "SoC Trim Phrase 6 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim62(pub u32);
impl Soctrim62 {
    #[doc = "TRIM6_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_2"]
    #[inline(always)]
    pub const fn set_trim6_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim62 {
    #[inline(always)]
    fn default() -> Soctrim62 {
        Soctrim62(0)
    }
}
impl core::fmt::Debug for Soctrim62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim62")
            .field("trim6_2", &self.trim6_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim62 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim62 {{ trim6_2: {=u32:?} }}", self.trim6_2())
    }
}
#[doc = "SoC Trim Phrase 6 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim63(pub u32);
impl Soctrim63 {
    #[doc = "TRIM6_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_3"]
    #[inline(always)]
    pub const fn set_trim6_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim63 {
    #[inline(always)]
    fn default() -> Soctrim63 {
        Soctrim63(0)
    }
}
impl core::fmt::Debug for Soctrim63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim63")
            .field("trim6_3", &self.trim6_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim63 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim63 {{ trim6_3: {=u32:?} }}", self.trim6_3())
    }
}
#[doc = "SoC Trim Phrase 7 Word 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim70(pub u32);
impl Soctrim70 {
    #[doc = "TRIM7_0"]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_0"]
    #[inline(always)]
    pub const fn set_trim7_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim70 {
    #[inline(always)]
    fn default() -> Soctrim70 {
        Soctrim70(0)
    }
}
impl core::fmt::Debug for Soctrim70 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim70")
            .field("trim7_0", &self.trim7_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim70 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim70 {{ trim7_0: {=u32:?} }}", self.trim7_0())
    }
}
#[doc = "SoC Trim Phrase 7 Word 1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim71(pub u32);
impl Soctrim71 {
    #[doc = "TRIM7_1"]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_1"]
    #[inline(always)]
    pub const fn set_trim7_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim71 {
    #[inline(always)]
    fn default() -> Soctrim71 {
        Soctrim71(0)
    }
}
impl core::fmt::Debug for Soctrim71 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim71")
            .field("trim7_1", &self.trim7_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim71 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim71 {{ trim7_1: {=u32:?} }}", self.trim7_1())
    }
}
#[doc = "SoC Trim Phrase 7 Word 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim72(pub u32);
impl Soctrim72 {
    #[doc = "TRIM7_2"]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_2"]
    #[inline(always)]
    pub const fn set_trim7_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim72 {
    #[inline(always)]
    fn default() -> Soctrim72 {
        Soctrim72(0)
    }
}
impl core::fmt::Debug for Soctrim72 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim72")
            .field("trim7_2", &self.trim7_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim72 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim72 {{ trim7_2: {=u32:?} }}", self.trim7_2())
    }
}
#[doc = "SoC Trim Phrase 7 Word 3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim73(pub u32);
impl Soctrim73 {
    #[doc = "TRIM7_3"]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_3"]
    #[inline(always)]
    pub const fn set_trim7_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim73 {
    #[inline(always)]
    fn default() -> Soctrim73 {
        Soctrim73(0)
    }
}
impl core::fmt::Debug for Soctrim73 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim73")
            .field("trim7_3", &self.trim7_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim73 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim73 {{ trim7_3: {=u32:?} }}", self.trim7_3())
    }
}
#[doc = "User Interface Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UintCtl(pub u32);
impl UintCtl {
    #[doc = "Set Fail On Exit"]
    #[must_use]
    #[inline(always)]
    pub const fn set_fail(&self) -> super::vals::SetFail {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SetFail::from_bits(val as u8)
    }
    #[doc = "Set Fail On Exit"]
    #[inline(always)]
    pub const fn set_set_fail(&mut self, val: super::vals::SetFail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Double-Bit ECC Fault Detect"]
    #[must_use]
    #[inline(always)]
    pub const fn dberr(&self) -> super::vals::Dberr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dberr::from_bits(val as u8)
    }
    #[doc = "Double-Bit ECC Fault Detect"]
    #[inline(always)]
    pub const fn set_dberr(&mut self, val: super::vals::Dberr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for UintCtl {
    #[inline(always)]
    fn default() -> UintCtl {
        UintCtl(0)
    }
}
impl core::fmt::Debug for UintCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UintCtl")
            .field("set_fail", &self.set_fail())
            .field("dberr", &self.dberr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UintCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UintCtl {{ set_fail: {:?}, dberr: {:?} }}",
            self.set_fail(),
            self.dberr()
        )
    }
}
