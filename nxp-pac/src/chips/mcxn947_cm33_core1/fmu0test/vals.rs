#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AbortLoop {
    #[doc = "No effect"]
    ZZ335 = 0x0,
    #[doc = "Abort BIST loop commands and force the loop counter to return to 0x0"]
    ZZ336 = 0x01,
}
impl AbortLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AbortLoop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AbortLoop {
    #[inline(always)]
    fn from(val: u8) -> AbortLoop {
        AbortLoop::from_bits(val)
    }
}
impl From<AbortLoop> for u8 {
    #[inline(always)]
    fn from(val: AbortLoop) -> u8 {
        AbortLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abtreq {
    #[doc = "No request to abort a command write sequence"]
    ZZ39 = 0x0,
    #[doc = "Request to abort a command write sequence"]
    ZZ40 = 0x01,
}
impl Abtreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abtreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abtreq {
    #[inline(always)]
    fn from(val: u8) -> Abtreq {
        Abtreq::from_bits(val)
    }
}
impl From<Abtreq> for u8 {
    #[inline(always)]
    fn from(val: Abtreq) -> u8 {
        Abtreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Accerr {
    #[doc = "No access error detected"]
    ZZ21 = 0x0,
    #[doc = "Access error detected"]
    ZZ22 = 0x01,
}
impl Accerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Accerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Accerr {
    #[inline(always)]
    fn from(val: u8) -> Accerr {
        Accerr::from_bits(val)
    }
}
impl From<Accerr> for u8 {
    #[inline(always)]
    fn from(val: Accerr) -> u8 {
        Accerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrFail {
    #[doc = "The address is within the flash or IFR address space"]
    ZZ141 = 0x0,
    #[doc = "The address is outside the flash or IFR address space"]
    ZZ142 = 0x01,
}
impl AddrFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrFail {
    #[inline(always)]
    fn from(val: u8) -> AddrFail {
        AddrFail::from_bits(val)
    }
}
impl From<AddrFail> for u8 {
    #[inline(always)]
    fn from(val: AddrFail) -> u8 {
        AddrFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailBlk {
    #[doc = "The address is block-aligned"]
    ZZ143 = 0x0,
    #[doc = "The address is not block-aligned"]
    ZZ144 = 0x01,
}
impl AlignfailBlk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailBlk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailBlk {
    #[inline(always)]
    fn from(val: u8) -> AlignfailBlk {
        AlignfailBlk::from_bits(val)
    }
}
impl From<AlignfailBlk> for u8 {
    #[inline(always)]
    fn from(val: AlignfailBlk) -> u8 {
        AlignfailBlk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailPg {
    #[doc = "The address is page-aligned"]
    ZZ147 = 0x0,
    #[doc = "The address is not page-aligned"]
    ZZ148 = 0x01,
}
impl AlignfailPg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailPg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailPg {
    #[inline(always)]
    fn from(val: u8) -> AlignfailPg {
        AlignfailPg::from_bits(val)
    }
}
impl From<AlignfailPg> for u8 {
    #[inline(always)]
    fn from(val: AlignfailPg) -> u8 {
        AlignfailPg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailPhr {
    #[doc = "The address is phrase-aligned"]
    ZZ149 = 0x0,
    #[doc = "The address is not phrase-aligned"]
    ZZ150 = 0x01,
}
impl AlignfailPhr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailPhr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailPhr {
    #[inline(always)]
    fn from(val: u8) -> AlignfailPhr {
        AlignfailPhr::from_bits(val)
    }
}
impl From<AlignfailPhr> for u8 {
    #[inline(always)]
    fn from(val: AlignfailPhr) -> u8 {
        AlignfailPhr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailScr {
    #[doc = "The address is sector-aligned"]
    ZZ145 = 0x0,
    #[doc = "The address is not sector-aligned"]
    ZZ146 = 0x01,
}
impl AlignfailScr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailScr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailScr {
    #[inline(always)]
    fn from(val: u8) -> AlignfailScr {
        AlignfailScr::from_bits(val)
    }
}
impl From<AlignfailScr> for u8 {
    #[inline(always)]
    fn from(val: AlignfailScr) -> u8 {
        AlignfailScr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AllCmd {
    #[doc = "The command operates on a single flash block"]
    ZZ137 = 0x0,
    #[doc = "The command operates on all flash blocks"]
    ZZ138 = 0x01,
}
impl AllCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AllCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AllCmd {
    #[inline(always)]
    fn from(val: u8) -> AllCmd {
        AllCmd::from_bits(val)
    }
}
impl From<AllCmd> for u8 {
    #[inline(always)]
    fn from(val: AllCmd) -> u8 {
        AllCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AryTrimDone {
    #[doc = "Recall register load operation has not been completed"]
    ZZ93 = 0x0,
    #[doc = "Recall register load operation has completed"]
    ZZ94 = 0x01,
}
impl AryTrimDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AryTrimDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AryTrimDone {
    #[inline(always)]
    fn from(val: u8) -> AryTrimDone {
        AryTrimDone::from_bits(val)
    }
}
impl From<AryTrimDone> for u8 {
    #[inline(always)]
    fn from(val: AryTrimDone) -> u8 {
        AryTrimDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BdoneSel {
    #[doc = "Select internal bist_done signal from current module instantiation"]
    ZZ453 = 0x0,
    #[doc = "Select ipt_bist_fail signal from current module instantiation"]
    ZZ454 = 0x01,
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    ZZ455 = 0x02,
    #[doc = "Select AND of internal bist_done signal from current module instantiation with ipt_bist_done signal from other module instantiation"]
    ZZ456 = 0x03,
}
impl BdoneSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BdoneSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BdoneSel {
    #[inline(always)]
    fn from(val: u8) -> BdoneSel {
        BdoneSel::from_bits(val)
    }
}
impl From<BdoneSel> for u8 {
    #[inline(always)]
    fn from(val: BdoneSel) -> u8 {
        BdoneSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistBusy {
    #[doc = "BIST Command not active"]
    ZZ201 = 0x0,
    #[doc = "BIST Command is active"]
    ZZ202 = 0x01,
}
impl BistBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistBusy {
    #[inline(always)]
    fn from(val: u8) -> BistBusy {
        BistBusy::from_bits(val)
    }
}
impl From<BistBusy> for u8 {
    #[inline(always)]
    fn from(val: BistBusy) -> u8 {
        BistBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistCtl {
    #[doc = "BIST IP disabled"]
    ZZ107 = 0x0,
    #[doc = "BIST IP enabled"]
    ZZ108 = 0x01,
}
impl BistCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistCtl {
    #[inline(always)]
    fn from(val: u8) -> BistCtl {
        BistCtl::from_bits(val)
    }
}
impl From<BistCtl> for u8 {
    #[inline(always)]
    fn from(val: BistCtl) -> u8 {
        BistCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistDone {
    #[doc = "The BIST (or data dump) is running"]
    ZZ439 = 0x0,
    #[doc = "The BIST (or data dump) has completed"]
    ZZ440 = 0x01,
}
impl BistDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistDone {
    #[inline(always)]
    fn from(val: u8) -> BistDone {
        BistDone::from_bits(val)
    }
}
impl From<BistDone> for u8 {
    #[inline(always)]
    fn from(val: BistDone) -> u8 {
        BistDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistEccEn {
    #[doc = "ECC correction disabled"]
    ZZ169 = 0x0,
    #[doc = "ECC correction enabled"]
    ZZ170 = 0x01,
}
impl BistEccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistEccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistEccEn {
    #[inline(always)]
    fn from(val: u8) -> BistEccEn {
        BistEccEn::from_bits(val)
    }
}
impl From<BistEccEn> for u8 {
    #[inline(always)]
    fn from(val: BistEccEn) -> u8 {
        BistEccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistFail {
    #[doc = "The last BIST operation completed successfully (or could not fail)"]
    ZZ437 = 0x0,
    #[doc = "The last BIST operation failed"]
    ZZ438 = 0x01,
}
impl BistFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistFail {
    #[inline(always)]
    fn from(val: u8) -> BistFail {
        BistFail::from_bits(val)
    }
}
impl From<BistFail> for u8 {
    #[inline(always)]
    fn from(val: BistFail) -> u8 {
        BistFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistMuxToSmw {
    #[doc = "BIST drives fields"]
    ZZ183 = 0x0,
    #[doc = "SMW registers drive fields"]
    ZZ184 = 0x01,
}
impl BistMuxToSmw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistMuxToSmw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistMuxToSmw {
    #[inline(always)]
    fn from(val: u8) -> BistMuxToSmw {
        BistMuxToSmw::from_bits(val)
    }
}
impl From<BistMuxToSmw> for u8 {
    #[inline(always)]
    fn from(val: BistMuxToSmw) -> u8 {
        BistMuxToSmw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistOn {
    #[doc = "BIST enable not forced by user interface"]
    ZZ159 = 0x0,
    #[doc = "BIST enable control by user interface"]
    ZZ160 = 0x01,
}
impl BistOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistOn {
    #[inline(always)]
    fn from(val: u8) -> BistOn {
        BistOn::from_bits(val)
    }
}
impl From<BistOn> for u8 {
    #[inline(always)]
    fn from(val: BistOn) -> u8 {
        BistOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistPwrDis {
    #[doc = "BIST DFT logic has full control of SLM and LVE when BIST is enabled (including during commands)"]
    ZZ97 = 0x0,
    #[doc = "BIST DFT logic has no control of SLM and LVE; power mode RTL is in complete control of SLM and LVE values"]
    ZZ98 = 0x01,
}
impl BistPwrDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistPwrDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistPwrDis {
    #[inline(always)]
    fn from(val: u8) -> BistPwrDis {
        BistPwrDis::from_bits(val)
    }
}
impl From<BistPwrDis> for u8 {
    #[inline(always)]
    fn from(val: BistPwrDis) -> u8 {
        BistPwrDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BsdoSel {
    #[doc = "Select internal bist_sdo signal from current module instantiation"]
    ZZ449 = 0x0,
    #[doc = "Select ipt_bist_done signal from current module instantiation"]
    ZZ450 = 0x01,
    #[doc = "Select ipt_bist_sdo signal from other module instantiation"]
    ZZ451 = 0x02,
    #[doc = "Select ipt_bist_done signal from other module instantiation"]
    ZZ452 = 0x03,
}
impl BsdoSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BsdoSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BsdoSel {
    #[inline(always)]
    fn from(val: u8) -> BsdoSel {
        BsdoSel::from_bits(val)
    }
}
impl From<BsdoSel> for u8 {
    #[inline(always)]
    fn from(val: BsdoSel) -> u8 {
        BsdoSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "BIST is idle"]
    ZZ333 = 0x0,
    #[doc = "BIST is busy"]
    ZZ334 = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccie {
    #[doc = "Command complete interrupt disabled"]
    ZZ37 = 0x0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    ZZ38 = 0x01,
}
impl Ccie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccie {
    #[inline(always)]
    fn from(val: u8) -> Ccie {
        Ccie::from_bits(val)
    }
}
impl From<Ccie> for u8 {
    #[inline(always)]
    fn from(val: Ccie) -> u8 {
        Ccie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccif {
    #[doc = "Flash command or initialization in progress"]
    ZZ17 = 0x0,
    #[doc = "Flash command or initialization has completed"]
    ZZ18 = 0x01,
}
impl Ccif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccif {
    #[inline(always)]
    fn from(val: u8) -> Ccif {
        Ccif::from_bits(val)
    }
}
impl From<Ccif> for u8 {
    #[inline(always)]
    fn from(val: Ccif) -> u8 {
        Ccif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "IDLE"]
    ZZ193 = 0x0,
    #[doc = "ABORT"]
    ZZ194 = 0x01,
    #[doc = "SME2 to one-shot mass erase"]
    ZZ195 = 0x02,
    #[doc = "SME3 to sector erase on selected array"]
    ZZ196 = 0x03,
    #[doc = "SMP1 to program phrase or page on selected array with shot disabled on previously programmed bit"]
    ZZ197 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SMP2 to program phrase or page on selected array to repair cells of weak program after power loss"]
    ZZ199 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdabt {
    #[doc = "No command abort detected"]
    ZZ25 = 0x0,
    #[doc = "Command abort detected"]
    ZZ26 = 0x01,
}
impl Cmdabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdabt {
    #[inline(always)]
    fn from(val: u8) -> Cmdabt {
        Cmdabt::from_bits(val)
    }
}
impl From<Cmdabt> for u8 {
    #[inline(always)]
    fn from(val: Cmdabt) -> u8 {
        Cmdabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdp {
    #[doc = "Command protection level and domain ID are stale"]
    ZZ11 = 0x0,
    #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set"]
    ZZ12 = 0x01,
}
impl Cmdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdp {
    #[inline(always)]
    fn from(val: u8) -> Cmdp {
        Cmdp::from_bits(val)
    }
}
impl From<Cmdp> for u8 {
    #[inline(always)]
    fn from(val: Cmdp) -> u8 {
        Cmdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdprt {
    #[doc = "Secure, normal access"]
    ZZ13 = 0x0,
    #[doc = "Secure, privileged access"]
    ZZ14 = 0x01,
    #[doc = "Nonsecure, normal access"]
    ZZ15 = 0x02,
    #[doc = "Nonsecure, privileged access"]
    ZZ16 = 0x03,
}
impl Cmdprt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdprt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdprt {
    #[inline(always)]
    fn from(val: u8) -> Cmdprt {
        Cmdprt::from_bits(val)
    }
}
impl From<Cmdprt> for u8 {
    #[inline(always)]
    fn from(val: Cmdprt) -> u8 {
        Cmdprt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpMask {
    #[doc = "Expected data is compared to DOUT"]
    ZZ229 = 0x0,
    #[doc = "Expected data (only 0s are considered) are compared to DOUT"]
    ZZ230 = 0x01,
    #[doc = "Expected data (only 1s are considered) are compared to DOUT"]
    ZZ231 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CmpMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpMask {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpMask {
    #[inline(always)]
    fn from(val: u8) -> CmpMask {
        CmpMask::from_bits(val)
    }
}
impl From<CmpMask> for u8 {
    #[inline(always)]
    fn from(val: CmpMask) -> u8 {
        CmpMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Corehld {
    #[doc = "CPU access is allowed"]
    ZZ125 = 0x0,
    #[doc = "CPU access must be blocked"]
    ZZ126 = 0x01,
}
impl Corehld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Corehld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Corehld {
    #[inline(always)]
    fn from(val: u8) -> Corehld {
        Corehld::from_bits(val)
    }
}
impl From<Corehld> for u8 {
    #[inline(always)]
    fn from(val: Corehld) -> u8 {
        Corehld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpyParEn {
    #[doc = "Copy parity disabled"]
    ZZ185 = 0x0,
    #[doc = "Copy parity enabled"]
    ZZ186 = 0x01,
}
impl CpyParEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpyParEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpyParEn {
    #[inline(always)]
    fn from(val: u8) -> CpyParEn {
        CpyParEn::from_bits(val)
    }
}
impl From<CpyParEn> for u8 {
    #[inline(always)]
    fn from(val: CpyParEn) -> u8 {
        CpyParEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpyPhraseEn {
    #[doc = "Copy Flash read data disabled"]
    ZZ173 = 0x0,
    #[doc = "Copy Flash read data enabled"]
    ZZ174 = 0x01,
}
impl CpyPhraseEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpyPhraseEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpyPhraseEn {
    #[inline(always)]
    fn from(val: u8) -> CpyPhraseEn {
        CpyPhraseEn::from_bits(val)
    }
}
impl From<CpyPhraseEn> for u8 {
    #[inline(always)]
    fn from(val: CpyPhraseEn) -> u8 {
        CpyPhraseEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsabt {
    #[doc = "Command write sequence not aborted"]
    ZZ19 = 0x0,
    #[doc = "Command write sequence aborted"]
    ZZ20 = 0x01,
}
impl Cwsabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsabt {
    #[inline(always)]
    fn from(val: u8) -> Cwsabt {
        Cwsabt::from_bits(val)
    }
}
impl From<Cwsabt> for u8 {
    #[inline(always)]
    fn from(val: Cwsabt) -> u8 {
        Cwsabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsabten {
    #[doc = "CWS abort feature is disabled"]
    ZZ115 = 0x0,
    #[doc = "CWS abort feature is enabled"]
    ZZ116 = 0x01,
}
impl Cwsabten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsabten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsabten {
    #[inline(always)]
    fn from(val: u8) -> Cwsabten {
        Cwsabten::from_bits(val)
    }
}
impl From<Cwsabten> for u8 {
    #[inline(always)]
    fn from(val: Cwsabten) -> u8 {
        Cwsabten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpMrgen {
    #[doc = "Normal read pulse shape"]
    ZZ431 = 0x0,
    #[doc = "Margin read pulse shape"]
    ZZ432 = 0x01,
}
impl DatadumpMrgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpMrgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpMrgen {
    #[inline(always)]
    fn from(val: u8) -> DatadumpMrgen {
        DatadumpMrgen::from_bits(val)
    }
}
impl From<DatadumpMrgen> for u8 {
    #[inline(always)]
    fn from(val: DatadumpMrgen) -> u8 {
        DatadumpMrgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpMrgtype {
    #[doc = "DIN method used"]
    ZZ429 = 0x0,
    #[doc = "TM method used"]
    ZZ430 = 0x01,
}
impl DatadumpMrgtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpMrgtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpMrgtype {
    #[inline(always)]
    fn from(val: u8) -> DatadumpMrgtype {
        DatadumpMrgtype::from_bits(val)
    }
}
impl From<DatadumpMrgtype> for u8 {
    #[inline(always)]
    fn from(val: DatadumpMrgtype) -> u8 {
        DatadumpMrgtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpPatt {
    #[doc = "All ones"]
    ZZ433 = 0x0,
    #[doc = "All zeroes"]
    ZZ434 = 0x01,
    #[doc = "Checkerboard"]
    ZZ435 = 0x02,
    #[doc = "Inverse checkerboard"]
    ZZ436 = 0x03,
}
impl DatadumpPatt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpPatt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpPatt {
    #[inline(always)]
    fn from(val: u8) -> DatadumpPatt {
        DatadumpPatt::from_bits(val)
    }
}
impl From<DatadumpPatt> for u8 {
    #[inline(always)]
    fn from(val: DatadumpPatt) -> u8 {
        DatadumpPatt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dberr {
    #[doc = "No double-bit fault detected during UINT-driven read sequence"]
    ZZ163 = 0x0,
    #[doc = "Double-bit fault detected during UINT-driven read sequence"]
    ZZ164 = 0x01,
}
impl Dberr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dberr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dberr {
    #[inline(always)]
    fn from(val: u8) -> Dberr {
        Dberr::from_bits(val)
    }
}
impl From<Dberr> for u8 {
    #[inline(always)]
    fn from(val: Dberr) -> u8 {
        Dberr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DberrReg {
    #[doc = "Double-bit fault not detected"]
    ZZ177 = 0x0,
    #[doc = "Double-bit fault detected on previous UINT flash read"]
    ZZ178 = 0x01,
}
impl DberrReg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DberrReg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DberrReg {
    #[inline(always)]
    fn from(val: u8) -> DberrReg {
        DberrReg::from_bits(val)
    }
}
impl From<DberrReg> for u8 {
    #[inline(always)]
    fn from(val: DberrReg) -> u8 {
        DberrReg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgctl {
    #[doc = "Default"]
    ZZ213 = 0x0,
    #[doc = "Enable debug feature to collect failure address and data."]
    ZZ214 = 0x01,
}
impl Dbgctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgctl {
    #[inline(always)]
    fn from(val: u8) -> Dbgctl {
        Dbgctl::from_bits(val)
    }
}
impl From<Dbgctl> for u8 {
    #[inline(always)]
    fn from(val: Dbgctl) -> u8 {
        Dbgctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdie {
    #[doc = "Double bit fault detect interrupt disabled"]
    ZZ33 = 0x0,
    #[doc = "Double bit fault detect interrupt enabled; an interrupt request is generated whenever the FSTAT\\[DFDIF\\] flag is set"]
    ZZ34 = 0x01,
}
impl Dfdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdie {
    #[inline(always)]
    fn from(val: u8) -> Dfdie {
        Dfdie::from_bits(val)
    }
}
impl From<Dfdie> for u8 {
    #[inline(always)]
    fn from(val: Dfdie) -> u8 {
        Dfdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdif {
    #[doc = "Double bit fault not detected during a valid flash read access from the FMC"]
    ZZ9 = 0x0,
    #[doc = "Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access from the FMC"]
    ZZ10 = 0x01,
}
impl Dfdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdif {
    #[inline(always)]
    fn from(val: u8) -> Dfdif {
        Dfdif::from_bits(val)
    }
}
impl From<Dfdif> for u8 {
    #[inline(always)]
    fn from(val: Dfdif) -> u8 {
        Dfdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftData {
    #[doc = "CKBD pattern. For READ operations only, compare DOUT with checkerboard data pattern for each read cycle."]
    ZZ232 = 0x0,
    #[doc = "ICKBD pattern. For READ operations only, compare DOUT with inverse checkerboard data pattern for each read cycle."]
    ZZ233 = 0x01,
    #[doc = "Diagonal pattern. Used for READ operations only, compare DOUT to diagonal pattern."]
    ZZ234 = 0x02,
    #[doc = "Fixed data pattern. For READ operations, comparison to DOUT for selected groups; refer to R_ADR_CTRL\\[GRPSEL\\] for modules with multiple groups."]
    ZZ235 = 0x03,
    #[doc = "Random data pattern which will be generated based on the initial seed set in R_DATA; for READ operations, used for DOUT comparison of selected groups. For PROG operations, used to control DIN of selected groups."]
    ZZ236 = 0x04,
    #[doc = "DOUT based pattern. For READ operations only, DOUT of selected group will be latched in R_DATA. If more than one group is selected in R_ADR_CTRL\\[GRPSEL\\], the group with the lower index will be latched."]
    ZZ237 = 0x05,
    #[doc = "R_DATA based pattern. For READ operations, expected DOUT value of selected groups equals to R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]. For PROG operations, DIN of selected groups equals R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]."]
    ZZ238 = 0x06,
    #[doc = "SCAN-IO pattern. For READ operations, control expected DOUT value of selected groups to SCAN-IO data pattern. For PROG operations, control DIN of selected groups to SCAN-IO data pattern."]
    ZZ239 = 0x07,
    #[doc = "REPAIR set. For PROG operation to IFR1(7,1) and IFR1(7,2), R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 and R_REPAIR1_1 will control DIN. For READ operation on IFR1(7,1) and IFR1(7,2), DOUT will be compared against R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 andR_REPAIR1_1. When this option is selected, only one flash block can be selected."]
    ZZ240 = 0x08,
    #[doc = "REPAIR load. For READ operation only, DOUT from IFR1(7,1) and IFR1(7,2) is loaded to R_REPAIR0 and R_REPAIR1."]
    ZZ241 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftData {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftData {
    #[inline(always)]
    fn from(val: u8) -> DftData {
        DftData::from_bits(val)
    }
}
impl From<DftData> for u8 {
    #[inline(always)]
    fn from(val: DftData) -> u8 {
        DftData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftDataSrc {
    #[doc = "{R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    ZZ227 = 0x0,
    #[doc = "{R_DATA_CTRL3,R_DATA_CTRL2_EX\\[2:0\\],R_DATA_CTRL2,R_DATA_CTRL1_EX\\[2:0\\],R_DATA_CTRL1,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used"]
    ZZ228 = 0x01,
}
impl DftDataSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftDataSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftDataSrc {
    #[inline(always)]
    fn from(val: u8) -> DftDataSrc {
        DftDataSrc::from_bits(val)
    }
}
impl From<DftDataSrc> for u8 {
    #[inline(always)]
    fn from(val: DftDataSrc) -> u8 {
        DftDataSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftXadr {
    #[doc = "XADR fixed, no change at all"]
    ZZ252 = 0x0,
    #[doc = "XADR increased by 1 after row. For READ operation, XADR increases by 1 after reading the last word of row. For PROG operation, XADR increases by 1 after NVSTR falls."]
    ZZ253 = 0x01,
    #[doc = "XADR increased for diagonal. For PROG-DIAGONAL operation, XADR is increased to create diagonal pattern."]
    ZZ254 = 0x02,
    #[doc = "XADR increased by sector. During ERASE operation, XADR increased by number of rows in a sector when NVSTR falls."]
    ZZ255 = 0x03,
    #[doc = "XADR inversed. XADR is inversed after reading one word or after programming one row when NVSTR falls."]
    ZZ256 = 0x04,
    #[doc = "XADR increased by 2 after row. For READ operation, XADR is increased by 2 after reading the last word of a row. For PROG operation, XADR is increased by 2 when NVSTR falls."]
    ZZ257 = 0x05,
    #[doc = "XADR\\[0\\] inversed. XADR\\[0\\] is inversed after reading one word or after programming one row when NVSTR falls."]
    ZZ258 = 0x06,
    #[doc = "XADR increased by 1. For READ operations only, XADR increased by 1 after each read cycle."]
    ZZ259 = 0x07,
    #[doc = "XADR decreased by 1 after row. For READ operations only, XADR is decreased by 1 after YADR decreases to 0."]
    ZZ260 = 0x08,
    #[doc = "XADR decreased by 1. For READ operations only, XADR is decreased by 1 after each read cycle."]
    ZZ261 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftXadr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftXadr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftXadr {
    #[inline(always)]
    fn from(val: u8) -> DftXadr {
        DftXadr::from_bits(val)
    }
}
impl From<DftXadr> for u8 {
    #[inline(always)]
    fn from(val: DftXadr) -> u8 {
        DftXadr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftYadr {
    #[doc = "YADR fixed, no change at all"]
    ZZ242 = 0x0,
    #[doc = "YADR for ICKBD. For PROG and READ operations, YADR changed to generate inverse checkerboard pattern."]
    ZZ243 = 0x01,
    #[doc = "YADR for CKBD. For PROG and READ operations, YADR changed to generate checkerboard pattern."]
    ZZ244 = 0x02,
    #[doc = "YADR increased by 1. For READ operations, YADR increased by 1 after each read cycle. For PROG operations, YADR increased by 1 after YE falls."]
    ZZ245 = 0x03,
    #[doc = "YADR increased for diagonal. For PROG-DIAGONAL operation, YADR is increased to create diagonal pattern."]
    ZZ246 = 0x04,
    #[doc = "YADR inversed. YADR is inversed after reading one word or after programming one word when YE falls."]
    ZZ247 = 0x05,
    #[doc = "YADR\\[0\\] inversed. YADR\\[0\\] is inversed after reading one word or after programming one word when YE falls."]
    ZZ248 = 0x06,
    #[doc = "YADR increased by 1 after last row. For READ operations only, YADR is increased by 1 after XADR reaches last row."]
    ZZ249 = 0x07,
    #[doc = "YADR decreased by 1. For READ operations only, YADR is decreased by 1 after each read cycle."]
    ZZ250 = 0x08,
    #[doc = "YADR decreased by 1 after first row. For READ operations only, YADR is decreased by 1 after XADR decreases to 0."]
    ZZ251 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftYadr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftYadr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftYadr {
    #[inline(always)]
    fn from(val: u8) -> DftYadr {
        DftYadr::from_bits(val)
    }
}
impl From<DftYadr> for u8 {
    #[inline(always)]
    fn from(val: DftYadr) -> u8 {
        DftYadr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPrer {
    #[doc = "Enable pre-PV read before first program shot"]
    ZZ345 = 0x0,
    #[doc = "Disable pre-PV read before first program shot"]
    ZZ346 = 0x01,
}
impl DisPrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPrer {
    #[inline(always)]
    fn from(val: u8) -> DisPrer {
        DisPrer::from_bits(val)
    }
}
impl From<DisPrer> for u8 {
    #[inline(always)]
    fn from(val: DisPrer) -> u8 {
        DisPrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccEnableb {
    #[doc = "ECC decoder enabled (default)"]
    ZZ189 = 0x0,
    #[doc = "ECC decoder disabled"]
    ZZ190 = 0x01,
}
impl EccEnableb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccEnableb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccEnableb {
    #[inline(always)]
    fn from(val: u8) -> EccEnableb {
        EccEnableb::from_bits(val)
    }
}
impl From<EccEnableb> for u8 {
    #[inline(always)]
    fn from(val: EccEnableb) -> u8 {
        EccEnableb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccen {
    #[doc = "Default mode (no ECC encode or decode)"]
    ZZ207 = 0x0,
    #[doc = "Enable ECC encode/decode"]
    ZZ208 = 0x01,
}
impl Eccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccen {
    #[inline(always)]
    fn from(val: u8) -> Eccen {
        Eccen::from_bits(val)
    }
}
impl From<Eccen> for u8 {
    #[inline(always)]
    fn from(val: Eccen) -> u8 {
        Eccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersaack {
    #[doc = "Mass Erase operation is not active (operation has completed or has not started)"]
    ZZ111 = 0x0,
    #[doc = "Mass Erase operation is active (controller acknowledges that the soc_ersall_req input is asserted and will continue with the operation)"]
    ZZ112 = 0x01,
}
impl Ersaack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersaack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersaack {
    #[inline(always)]
    fn from(val: u8) -> Ersaack {
        Ersaack::from_bits(val)
    }
}
impl From<Ersaack> for u8 {
    #[inline(always)]
    fn from(val: Ersaack) -> u8 {
        Ersaack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien0 {
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command"]
    ZZ31 = 0x0,
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command"]
    ZZ32 = 0x01,
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
impl Ersien0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien0 {
    #[inline(always)]
    fn from(val: u8) -> Ersien0 {
        Ersien0::from_bits(val)
    }
}
impl From<Ersien0> for u8 {
    #[inline(always)]
    fn from(val: Ersien0) -> u8 {
        Ersien0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien1 {
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command"]
    ZZ29 = 0x0,
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command"]
    ZZ30 = 0x01,
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
impl Ersien1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien1 {
    #[inline(always)]
    fn from(val: u8) -> Ersien1 {
        Ersien1::from_bits(val)
    }
}
impl From<Ersien1> for u8 {
    #[inline(always)]
    fn from(val: Ersien1) -> u8 {
        Ersien1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersreq {
    #[doc = "No request or request complete"]
    ZZ35 = 0x0,
    #[doc = "Request to run the Mass Erase operation"]
    ZZ36 = 0x01,
}
impl Ersreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersreq {
    #[inline(always)]
    fn from(val: u8) -> Ersreq {
        Ersreq::from_bits(val)
    }
}
impl From<Ersreq> for u8 {
    #[inline(always)]
    fn from(val: Ersreq) -> u8 {
        Ersreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fail {
    #[doc = "Error not detected"]
    ZZ27 = 0x0,
    #[doc = "Error detected"]
    ZZ28 = 0x01,
}
impl Fail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fail {
    #[inline(always)]
    fn from(val: u8) -> Fail {
        Fail::from_bits(val)
    }
}
impl From<Fail> for u8 {
    #[inline(always)]
    fn from(val: Fail) -> u8 {
        Fail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdfd {
    #[doc = "FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the FMC"]
    ZZ41 = 0x0,
    #[doc = "FSTAT\\[DFDIF\\] sets during any valid flash read access from the FMC; an interrupt request is generated if the DFDIE bit is set"]
    ZZ42 = 0x01,
}
impl Fdfd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdfd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdfd {
    #[inline(always)]
    fn from(val: u8) -> Fdfd {
        Fdfd::from_bits(val)
    }
}
impl From<Fdfd> for u8 {
    #[inline(always)]
    fn from(val: Fdfd) -> u8 {
        Fdfd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashRd {
    #[doc = "Manual flash read not enabled.(default)"]
    ZZ155 = 0x0,
    #[doc = "Manual flash read enabled"]
    ZZ156 = 0x01,
}
impl FlashRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashRd {
    #[inline(always)]
    fn from(val: u8) -> FlashRd {
        FlashRd::from_bits(val)
    }
}
impl From<FlashRd> for u8 {
    #[inline(always)]
    fn from(val: FlashRd) -> u8 {
        FlashRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuEccCtl {
    #[doc = "ECC is enabled for FMU program operations"]
    ZZ99 = 0x0,
    #[doc = "ECC is disabled for FMU program operations"]
    ZZ100 = 0x01,
}
impl FmuEccCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuEccCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuEccCtl {
    #[inline(always)]
    fn from(val: u8) -> FmuEccCtl {
        FmuEccCtl::from_bits(val)
    }
}
impl From<FmuEccCtl> for u8 {
    #[inline(always)]
    fn from(val: FmuEccCtl) -> u8 {
        FmuEccCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuParmDone {
    #[doc = "FMU registers have not been loaded"]
    ZZ89 = 0x0,
    #[doc = "FMU registers have been loaded"]
    ZZ90 = 0x01,
}
impl FmuParmDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuParmDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuParmDone {
    #[inline(always)]
    fn from(val: u8) -> FmuParmDone {
        FmuParmDone::from_bits(val)
    }
}
impl From<FmuParmDone> for u8 {
    #[inline(always)]
    fn from(val: FmuParmDone) -> u8 {
        FmuParmDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuParmEn {
    #[doc = "C0DE_C0DEh check not attempted"]
    ZZ91 = 0x0,
    #[doc = "C0DE_C0DEh check completed"]
    ZZ92 = 0x01,
}
impl FmuParmEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuParmEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuParmEn {
    #[inline(always)]
    fn from(val: u8) -> FmuParmEn {
        FmuParmEn::from_bits(val)
    }
}
impl From<FmuParmEn> for u8 {
    #[inline(always)]
    fn from(val: FmuParmEn) -> u8 {
        FmuParmEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceSwClk {
    #[doc = "Switch clock not forced on (gated normally)"]
    ZZ157 = 0x0,
    #[doc = "Switch clock forced on"]
    ZZ158 = 0x01,
}
impl ForceSwClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceSwClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceSwClk {
    #[inline(always)]
    fn from(val: u8) -> ForceSwClk {
        ForceSwClk::from_bits(val)
    }
}
impl From<ForceSwClk> for u8 {
    #[inline(always)]
    fn from(val: ForceSwClk) -> u8 {
        ForceSwClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grpsel {
    #[doc = "Select no data"]
    ZZ270 = 0x0,
    #[doc = "Select data slice \\[34:0\\]"]
    ZZ271 = 0x01,
    #[doc = "Select data slice \\[69:35\\]"]
    ZZ272 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select data slice \\[104:70\\]"]
    ZZ273 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select data slice \\[136:105\\]"]
    ZZ274 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select data \\[136:0\\]"]
    ZZ275 = 0x0f,
}
impl Grpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grpsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grpsel {
    #[inline(always)]
    fn from(val: u8) -> Grpsel {
        Grpsel::from_bits(val)
    }
}
impl From<Grpsel> for u8 {
    #[inline(always)]
    fn from(val: Grpsel) -> u8 {
        Grpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrCmd {
    #[doc = "The command operates on a main flash address"]
    ZZ139 = 0x0,
    #[doc = "The command operates on an IFR address"]
    ZZ140 = 0x01,
}
impl IfrCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrCmd {
    #[inline(always)]
    fn from(val: u8) -> IfrCmd {
        IfrCmd::from_bits(val)
    }
}
impl From<IfrCmd> for u8 {
    #[inline(always)]
    fn from(val: IfrCmd) -> u8 {
        IfrCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IllegalCmd {
    #[doc = "Command is legal"]
    ZZ129 = 0x0,
    #[doc = "Command is illegal"]
    ZZ130 = 0x01,
}
impl IllegalCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IllegalCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IllegalCmd {
    #[inline(always)]
    fn from(val: u8) -> IllegalCmd {
        IllegalCmd::from_bits(val)
    }
}
impl From<IllegalCmd> for u8 {
    #[inline(always)]
    fn from(val: IllegalCmd) -> u8 {
        IllegalCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitDone {
    #[doc = "All initialization steps did not complete"]
    ZZ79 = 0x0,
    #[doc = "All initialization steps completed"]
    ZZ80 = 0x01,
}
impl InitDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitDone {
    #[inline(always)]
    fn from(val: u8) -> InitDone {
        InitDone::from_bits(val)
    }
}
impl From<InitDone> for u8 {
    #[inline(always)]
    fn from(val: InitDone) -> u8 {
        InitDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipsel0 {
    #[doc = "Unselect block 0"]
    ZZ223 = 0x0,
    #[doc = "not used, reserved"]
    ZZ224 = 0x01,
    #[doc = "Enable block 0 test, repair off (default)"]
    ZZ225 = 0x02,
    #[doc = "Enable block 0 test, repair on"]
    ZZ226 = 0x03,
}
impl Ipsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipsel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipsel0 {
    #[inline(always)]
    fn from(val: u8) -> Ipsel0 {
        Ipsel0::from_bits(val)
    }
}
impl From<Ipsel0> for u8 {
    #[inline(always)]
    fn from(val: Ipsel0) -> u8 {
        Ipsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipsel1 {
    #[doc = "Unselect block 1"]
    ZZ219 = 0x0,
    #[doc = "not used, reserved"]
    ZZ220 = 0x01,
    #[doc = "Enable block 1 test, repair off (default)"]
    ZZ221 = 0x02,
    #[doc = "Enable block 1 test, repair on"]
    ZZ222 = 0x03,
}
impl Ipsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipsel1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipsel1 {
    #[inline(always)]
    fn from(val: u8) -> Ipsel1 {
        Ipsel1::from_bits(val)
    }
}
impl From<Ipsel1> for u8 {
    #[inline(always)]
    fn from(val: Ipsel1) -> u8 {
        Ipsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LastRead {
    #[doc = "Latest read not last in multi-address operation"]
    ZZ167 = 0x0,
    #[doc = "Latest read last in multi-address operation"]
    ZZ168 = 0x01,
}
impl LastRead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LastRead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LastRead {
    #[inline(always)]
    fn from(val: u8) -> LastRead {
        LastRead::from_bits(val)
    }
}
impl From<LastRead> for u8 {
    #[inline(always)]
    fn from(val: LastRead) -> u8 {
        LastRead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopopt {
    #[doc = "Loop is disabled; selected BIST operation is run once"]
    ZZ284 = 0x0,
    #[doc = "Loop is enabled; XADR increments by 1 XADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    ZZ285 = 0x01,
    #[doc = "Loop is enabled; YADR increments by 1 YADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    ZZ286 = 0x02,
    #[doc = "Loop is enabled; XADR increments by 2 XADR increments by 2 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    ZZ287 = 0x03,
    #[doc = "Loop is enabled; XADR increments by sector XADR increments by 16 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    ZZ288 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Loopopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopopt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopopt {
    #[inline(always)]
    fn from(val: u8) -> Loopopt {
        Loopopt::from_bits(val)
    }
}
impl From<Loopopt> for u8 {
    #[inline(always)]
    fn from(val: Loopopt) -> u8 {
        Loopopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopunit {
    #[doc = "Clock cycles"]
    ZZ276 = 0x0,
    #[doc = "0.5 usec"]
    ZZ277 = 0x01,
    #[doc = "1 usec"]
    ZZ278 = 0x02,
    #[doc = "10 usec"]
    ZZ279 = 0x03,
    #[doc = "100 usec"]
    ZZ280 = 0x04,
    #[doc = "1 msec"]
    ZZ281 = 0x05,
    #[doc = "10 msec"]
    ZZ282 = 0x06,
    #[doc = "100 msec"]
    ZZ283 = 0x07,
}
impl Loopunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopunit {
    #[inline(always)]
    fn from(val: u8) -> Loopunit {
        Loopunit::from_bits(val)
    }
}
impl From<Loopunit> for u8 {
    #[inline(always)]
    fn from(val: Loopunit) -> u8 {
        Loopunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LsactEn {
    #[doc = "LSACTIVE feature disabled completely: FCTRL\\[LSACTIVE\\] is forced low and no longer writable, LVE cannot assert at the TSMC array interface."]
    ZZ123 = 0x0,
    #[doc = "LSACTIVE feature fully enabled and controllable by SoC and internal UINT SM."]
    ZZ124 = 0x01,
}
impl LsactEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LsactEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LsactEn {
    #[inline(always)]
    fn from(val: u8) -> LsactEn {
        LsactEn::from_bits(val)
    }
}
impl From<LsactEn> for u8 {
    #[inline(always)]
    fn from(val: LsactEn) -> u8 {
        LsactEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsactive {
    #[doc = "Full speed active mode requested"]
    ZZ43 = 0x0,
    #[doc = "Low speed active mode requested"]
    ZZ44 = 0x01,
}
impl Lsactive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsactive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsactive {
    #[inline(always)]
    fn from(val: u8) -> Lsactive {
        Lsactive::from_bits(val)
    }
}
impl From<Lsactive> for u8 {
    #[inline(always)]
    fn from(val: Lsactive) -> u8 {
        Lsactive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsactwren {
    #[doc = "Unrestricted write access allowed"]
    ZZ121 = 0x0,
    #[doc = "Write access while CMP set must match CMDDID and CMDPRT"]
    ZZ122 = 0x01,
}
impl Lsactwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsactwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsactwren {
    #[inline(always)]
    fn from(val: u8) -> Lsactwren {
        Lsactwren::from_bits(val)
    }
}
impl From<Lsactwren> for u8 {
    #[inline(always)]
    fn from(val: Lsactwren) -> u8 {
        Lsactwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mask0Opt {
    #[doc = "Mask programmed bits passing PV until extra shot"]
    ZZ347 = 0x0,
    #[doc = "Always program bits even if they pass PV"]
    ZZ348 = 0x01,
}
impl Mask0Opt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mask0Opt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mask0Opt {
    #[inline(always)]
    fn from(val: u8) -> Mask0Opt {
        Mask0Opt::from_bits(val)
    }
}
impl From<Mask0Opt> for u8 {
    #[inline(always)]
    fn from(val: Mask0Opt) -> u8 {
        Mask0Opt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterRepairEn {
    #[doc = "Repair disabled"]
    ZZ119 = 0x0,
    #[doc = "Repair enable determined by bit 0 of each REPAIR register"]
    ZZ120 = 0x01,
}
impl MasterRepairEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterRepairEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterRepairEn {
    #[inline(always)]
    fn from(val: u8) -> MasterRepairEn {
        MasterRepairEn::from_bits(val)
    }
}
impl From<MasterRepairEn> for u8 {
    #[inline(always)]
    fn from(val: MasterRepairEn) -> u8 {
        MasterRepairEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MisrEn {
    #[doc = "MISR option disabled (default)"]
    ZZ187 = 0x0,
    #[doc = "MISR option enabled"]
    ZZ188 = 0x01,
}
impl MisrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MisrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MisrEn {
    #[inline(always)]
    fn from(val: u8) -> MisrEn {
        MisrEn::from_bits(val)
    }
}
impl From<MisrEn> for u8 {
    #[inline(always)]
    fn from(val: MisrEn) -> u8 {
        MisrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MmRd {
    #[doc = "Write to register"]
    ZZ161 = 0x0,
    #[doc = "Read register"]
    ZZ162 = 0x01,
}
impl MmRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MmRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MmRd {
    #[inline(always)]
    fn from(val: u8) -> MmRd {
        MmRd::from_bits(val)
    }
}
impl From<MmRd> for u8 {
    #[inline(always)]
    fn from(val: MmRd) -> u8 {
        MmRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrgrddis {
    #[doc = "Margin Read Settings are enabled"]
    ZZ113 = 0x0,
    #[doc = "Margin Read Settings are disabled"]
    ZZ114 = 0x01,
}
impl Mrgrddis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrgrddis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrgrddis {
    #[inline(always)]
    fn from(val: u8) -> Mrgrddis {
        Mrgrddis::from_bits(val)
    }
}
impl From<Mrgrddis> for u8 {
    #[inline(always)]
    fn from(val: Mrgrddis) -> u8 {
        Mrgrddis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OptionFail {
    #[doc = "Option check passes for read command or command is not a read command"]
    ZZ131 = 0x0,
    #[doc = "Option check fails for read command"]
    ZZ132 = 0x01,
}
impl OptionFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OptionFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OptionFail {
    #[inline(always)]
    fn from(val: u8) -> OptionFail {
        OptionFail::from_bits(val)
    }
}
impl From<OptionFail> for u8 {
    #[inline(always)]
    fn from(val: OptionFail) -> u8 {
        OptionFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscH {
    #[doc = "Use APB clock"]
    ZZ95 = 0x0,
    #[doc = "Use a known fixed-frequency clock, e.g. 12 MHz"]
    ZZ96 = 0x01,
}
impl OscH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscH {
    #[inline(always)]
    fn from(val: u8) -> OscH {
        OscH::from_bits(val)
    }
}
impl From<OscH> for u8 {
    #[inline(always)]
    fn from(val: OscH) -> u8 {
        OscH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdTimerEn {
    #[doc = "BIST timer is not triggered during Power Down recovery"]
    ZZ127 = 0x0,
    #[doc = "BIST timer is triggered during Power Down recovery (default behavior)"]
    ZZ128 = 0x01,
}
impl PdTimerEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdTimerEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdTimerEn {
    #[inline(always)]
    fn from(val: u8) -> PdTimerEn {
        PdTimerEn::from_bits(val)
    }
}
impl From<PdTimerEn> for u8 {
    #[inline(always)]
    fn from(val: PdTimerEn) -> u8 {
        PdTimerEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perdy {
    #[doc = "Program or sector erase command operation is not stalled"]
    ZZ1 = 0x0,
    #[doc = "Program or sector erase command operation is stalled"]
    ZZ2 = 0x01,
}
impl Perdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perdy {
    #[inline(always)]
    fn from(val: u8) -> Perdy {
        Perdy::from_bits(val)
    }
}
impl From<Perdy> for u8 {
    #[inline(always)]
    fn from(val: Perdy) -> u8 {
        Perdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pewen {
    #[doc = "Writes are not enabled"]
    ZZ3 = 0x0,
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)"]
    ZZ4 = 0x01,
    #[doc = "Writes are enabled for one flash or IFR page (page programming)"]
    ZZ5 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pewen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pewen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pewen {
    #[inline(always)]
    fn from(val: u8) -> Pewen {
        Pewen::from_bits(val)
    }
}
impl From<Pewen> for u8 {
    #[inline(always)]
    fn from(val: Pewen) -> u8 {
        Pewen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostTers {
    #[doc = "50 usec"]
    ZZ361 = 0x0,
    #[doc = "100 usec"]
    ZZ362 = 0x01,
    #[doc = "200 usec"]
    ZZ363 = 0x02,
    #[doc = "300 usec"]
    ZZ364 = 0x03,
    #[doc = "500 usec"]
    ZZ365 = 0x04,
    #[doc = "1 msec"]
    ZZ366 = 0x05,
    #[doc = "1.5 msec"]
    ZZ367 = 0x06,
    #[doc = "2 msec"]
    ZZ368 = 0x07,
}
impl PostTers {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostTers {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostTers {
    #[inline(always)]
    fn from(val: u8) -> PostTers {
        PostTers::from_bits(val)
    }
}
impl From<PostTers> for u8 {
    #[inline(always)]
    fn from(val: PostTers) -> u8 {
        PostTers::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostTpgm {
    #[doc = "1 usec"]
    ZZ357 = 0x0,
    #[doc = "2 usec"]
    ZZ358 = 0x01,
    #[doc = "4 usec"]
    ZZ359 = 0x02,
    #[doc = "8 usec"]
    ZZ360 = 0x03,
}
impl PostTpgm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostTpgm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostTpgm {
    #[inline(always)]
    fn from(val: u8) -> PostTpgm {
        PostTpgm::from_bits(val)
    }
}
impl From<PostTpgm> for u8 {
    #[inline(always)]
    fn from(val: PostTpgm) -> u8 {
        PostTpgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgAttr {
    #[doc = "One YE pulse will program one data slice group"]
    ZZ262 = 0x0,
    #[doc = "One YE pulse will program two data slice groups"]
    ZZ263 = 0x01,
    #[doc = "One YE pulse will program three data slice groups (reserved)"]
    ZZ264 = 0x02,
    #[doc = "One YE pulse will program four data slice groups"]
    ZZ265 = 0x03,
    #[doc = "One YE pulse will program five data slice groups (reserved)"]
    ZZ266 = 0x04,
    #[doc = "One YE pulse will program six data slice groups (reserved)"]
    ZZ267 = 0x05,
    #[doc = "One YE pulse will program seven data slice groups (reserved)"]
    ZZ268 = 0x06,
    #[doc = "One YE pulse will program eight data slice groups (reserved)"]
    ZZ269 = 0x07,
}
impl ProgAttr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProgAttr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProgAttr {
    #[inline(always)]
    fn from(val: u8) -> ProgAttr {
        ProgAttr::from_bits(val)
    }
}
impl From<ProgAttr> for u8 {
    #[inline(always)]
    fn from(val: ProgAttr) -> u8 {
        ProgAttr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pviol {
    #[doc = "No protection violation detected"]
    ZZ23 = 0x0,
    #[doc = "Protection violation detected"]
    ZZ24 = 0x01,
}
impl Pviol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pviol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pviol {
    #[inline(always)]
    fn from(val: u8) -> Pviol {
        Pviol::from_bits(val)
    }
}
impl From<Pviol> for u8 {
    #[inline(always)]
    fn from(val: Pviol) -> u8 {
        Pviol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair00Rdis00 {
    #[doc = "Repair address is valid"]
    ZZ337 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ338 = 0x01,
}
impl RRepair00Rdis00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair00Rdis00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair00Rdis00 {
    #[inline(always)]
    fn from(val: u8) -> RRepair00Rdis00 {
        RRepair00Rdis00::from_bits(val)
    }
}
impl From<RRepair00Rdis00> for u8 {
    #[inline(always)]
    fn from(val: RRepair00Rdis00) -> u8 {
        RRepair00Rdis00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair01Rdis01 {
    #[doc = "Repair address is valid"]
    ZZ339 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ340 = 0x01,
}
impl RRepair01Rdis01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair01Rdis01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair01Rdis01 {
    #[inline(always)]
    fn from(val: u8) -> RRepair01Rdis01 {
        RRepair01Rdis01::from_bits(val)
    }
}
impl From<RRepair01Rdis01> for u8 {
    #[inline(always)]
    fn from(val: RRepair01Rdis01) -> u8 {
        RRepair01Rdis01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair10Rdis10 {
    #[doc = "Repair address is valid"]
    ZZ341 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ342 = 0x01,
}
impl RRepair10Rdis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair10Rdis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair10Rdis10 {
    #[inline(always)]
    fn from(val: u8) -> RRepair10Rdis10 {
        RRepair10Rdis10::from_bits(val)
    }
}
impl From<RRepair10Rdis10> for u8 {
    #[inline(always)]
    fn from(val: RRepair10Rdis10) -> u8 {
        RRepair10Rdis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair11Rdis11 {
    #[doc = "Repair address is valid"]
    ZZ343 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ344 = 0x01,
}
impl RRepair11Rdis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair11Rdis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair11Rdis11 {
    #[inline(always)]
    fn from(val: u8) -> RRepair11Rdis11 {
        RRepair11Rdis11::from_bits(val)
    }
}
impl From<RRepair11Rdis11> for u8 {
    #[inline(always)]
    fn from(val: RRepair11Rdis11) -> u8 {
        RRepair11Rdis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RangeFail {
    #[doc = "The address range is valid"]
    ZZ135 = 0x0,
    #[doc = "The address range is invalid"]
    ZZ136 = 0x01,
}
impl RangeFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RangeFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RangeFail {
    #[inline(always)]
    fn from(val: u8) -> RangeFail {
        RangeFail::from_bits(val)
    }
}
impl From<RangeFail> for u8 {
    #[inline(always)]
    fn from(val: RangeFail) -> u8 {
        RangeFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RecallDataMismatch {
    #[doc = "Data read towards end of reset matched data read for Recall"]
    ZZ71 = 0x0,
    #[doc = "Data read towards end of reset did not match data read for recall"]
    ZZ72 = 0x01,
}
impl RecallDataMismatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RecallDataMismatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RecallDataMismatch {
    #[inline(always)]
    fn from(val: u8) -> RecallDataMismatch {
        RecallDataMismatch::from_bits(val)
    }
}
impl From<RecallDataMismatch> for u8 {
    #[inline(always)]
    fn from(val: RecallDataMismatch) -> u8 {
        RecallDataMismatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair00Rdis00 {
    #[doc = "Repair address is valid"]
    ZZ405 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ406 = 0x01,
}
impl Repair00Rdis00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair00Rdis00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair00Rdis00 {
    #[inline(always)]
    fn from(val: u8) -> Repair00Rdis00 {
        Repair00Rdis00::from_bits(val)
    }
}
impl From<Repair00Rdis00> for u8 {
    #[inline(always)]
    fn from(val: Repair00Rdis00) -> u8 {
        Repair00Rdis00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair01Rdis01 {
    #[doc = "Repair address is valid"]
    ZZ407 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ408 = 0x01,
}
impl Repair01Rdis01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair01Rdis01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair01Rdis01 {
    #[inline(always)]
    fn from(val: u8) -> Repair01Rdis01 {
        Repair01Rdis01::from_bits(val)
    }
}
impl From<Repair01Rdis01> for u8 {
    #[inline(always)]
    fn from(val: Repair01Rdis01) -> u8 {
        Repair01Rdis01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair10Rdis10 {
    #[doc = "Repair address is valid"]
    ZZ409 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ410 = 0x01,
}
impl Repair10Rdis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair10Rdis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair10Rdis10 {
    #[inline(always)]
    fn from(val: u8) -> Repair10Rdis10 {
        Repair10Rdis10::from_bits(val)
    }
}
impl From<Repair10Rdis10> for u8 {
    #[inline(always)]
    fn from(val: Repair10Rdis10) -> u8 {
        Repair10Rdis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair11Rdis11 {
    #[doc = "Repair address is valid"]
    ZZ411 = 0x0,
    #[doc = "Repair address is not valid"]
    ZZ412 = 0x01,
}
impl Repair11Rdis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair11Rdis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair11Rdis11 {
    #[inline(always)]
    fn from(val: u8) -> Repair11Rdis11 {
        Repair11Rdis11::from_bits(val)
    }
}
impl From<Repair11Rdis11> for u8 {
    #[inline(always)]
    fn from(val: Repair11Rdis11) -> u8 {
        Repair11Rdis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfcmden {
    #[doc = "Flash commands blocked (CCIF not writable)"]
    ZZ117 = 0x0,
    #[doc = "Flash commands allowed"]
    ZZ118 = 0x01,
}
impl Rfcmden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfcmden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfcmden {
    #[inline(always)]
    fn from(val: u8) -> Rfcmden {
        Rfcmden::from_bits(val)
    }
}
impl From<Rfcmden> for u8 {
    #[inline(always)]
    fn from(val: Rfcmden) -> u8 {
        Rfcmden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RprDone {
    #[doc = "Repair registers have not been loaded"]
    ZZ81 = 0x0,
    #[doc = "Repair registers have been loaded"]
    ZZ82 = 0x01,
}
impl RprDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RprDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RprDone {
    #[inline(always)]
    fn from(val: u8) -> RprDone {
        RprDone::from_bits(val)
    }
}
impl From<RprDone> for u8 {
    #[inline(always)]
    fn from(val: RprDone) -> u8 {
        RprDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstDfErr {
    #[doc = "No double-bit faults detected during initialization"]
    ZZ75 = 0x0,
    #[doc = "Double-bit ECC fault was detected during initialization"]
    ZZ76 = 0x01,
}
impl RstDfErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstDfErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstDfErr {
    #[inline(always)]
    fn from(val: u8) -> RstDfErr {
        RstDfErr::from_bits(val)
    }
}
impl From<RstDfErr> for u8 {
    #[inline(always)]
    fn from(val: RstDfErr) -> u8 {
        RstDfErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstPatchLd {
    #[doc = "No patch required to be loaded during reset"]
    ZZ73 = 0x0,
    #[doc = "Patch loaded during reset"]
    ZZ74 = 0x01,
}
impl RstPatchLd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstPatchLd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstPatchLd {
    #[inline(always)]
    fn from(val: u8) -> RstPatchLd {
        RstPatchLd::from_bits(val)
    }
}
impl From<RstPatchLd> for u8 {
    #[inline(always)]
    fn from(val: RstPatchLd) -> u8 {
        RstPatchLd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstSfErr {
    #[doc = "No single-bit faults detected during initialization"]
    ZZ77 = 0x0,
    #[doc = "At least one single ECC fault was detected during initialization"]
    ZZ78 = 0x01,
}
impl RstSfErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstSfErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstSfErr {
    #[inline(always)]
    fn from(val: u8) -> RstSfErr {
        RstSfErr::from_bits(val)
    }
}
impl From<RstSfErr> for u8 {
    #[inline(always)]
    fn from(val: RstSfErr) -> u8 {
        RstSfErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwsc {
    #[doc = "no additional wait-states are added (single cycle access)"]
    ZZ45 = 0x0,
    #[doc = "1 additional wait-state is added"]
    ZZ46 = 0x01,
    #[doc = "2 additional wait-states are added"]
    ZZ47 = 0x02,
    #[doc = "3 additional wait-states are added"]
    ZZ48 = 0x03,
    #[doc = "4 additional wait-states are added"]
    ZZ49 = 0x04,
    #[doc = "5 additional wait-states are added"]
    ZZ50 = 0x05,
    #[doc = "6 additional wait-states are added"]
    ZZ51 = 0x06,
    #[doc = "7 additional wait-states are added"]
    ZZ52 = 0x07,
    #[doc = "8 additional wait-states are added"]
    ZZ53 = 0x08,
    #[doc = "9 additional wait-states are added"]
    ZZ54 = 0x09,
    #[doc = "10 additional wait-states are added"]
    ZZ55 = 0x0a,
    #[doc = "11 additional wait-states are added"]
    ZZ56 = 0x0b,
    #[doc = "12 additional wait-states are added"]
    ZZ57 = 0x0c,
    #[doc = "13 additional wait-states are added"]
    ZZ58 = 0x0d,
    #[doc = "14 additional wait-states are added"]
    ZZ59 = 0x0e,
    #[doc = "15 additional wait-states are added"]
    ZZ60 = 0x0f,
}
impl Rwsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwsc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwsc {
    #[inline(always)]
    fn from(val: u8) -> Rwsc {
        Rwsc::from_bits(val)
    }
}
impl From<Rwsc> for u8 {
    #[inline(always)]
    fn from(val: Rwsc) -> u8 {
        Rwsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SalvDis {
    #[doc = "Salvage enabled (ECC used during erase verify)"]
    ZZ103 = 0x0,
    #[doc = "Salvage disabled (ECC not used during erase verify)"]
    ZZ104 = 0x01,
}
impl SalvDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SalvDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SalvDis {
    #[inline(always)]
    fn from(val: u8) -> SalvDis {
        SalvDis::from_bits(val)
    }
}
impl From<SalvDis> for u8 {
    #[inline(always)]
    fn from(val: SalvDis) -> u8 {
        SalvDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SalvUsed {
    #[doc = "Salvage not used during the last operation"]
    ZZ7 = 0x0,
    #[doc = "Salvage used during the last erase operation"]
    ZZ8 = 0x01,
}
impl SalvUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SalvUsed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SalvUsed {
    #[inline(always)]
    fn from(val: u8) -> SalvUsed {
        SalvUsed::from_bits(val)
    }
}
impl From<SalvUsed> for u8 {
    #[inline(always)]
    fn from(val: SalvUsed) -> u8 {
        SalvUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SberrReg {
    #[doc = "Single-bit fault not detected"]
    ZZ175 = 0x0,
    #[doc = "Single-bit fault detected on previous UINT flash read"]
    ZZ176 = 0x01,
}
impl SberrReg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SberrReg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SberrReg {
    #[inline(always)]
    fn from(val: u8) -> SberrReg {
        SberrReg::from_bits(val)
    }
}
impl From<SberrReg> for u8 {
    #[inline(always)]
    fn from(val: SberrReg) -> u8 {
        SberrReg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanObs {
    #[doc = "Normal functional behavior"]
    ZZ109 = 0x0,
    #[doc = "Enables observation of signals that may otherwise be ATPG untestable"]
    ZZ110 = 0x01,
}
impl ScanObs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanObs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanObs {
    #[inline(always)]
    fn from(val: u8) -> ScanObs {
        ScanObs::from_bits(val)
    }
}
impl From<ScanObs> for u8 {
    #[inline(always)]
    fn from(val: ScanObs) -> u8 {
        ScanObs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrAlignChk {
    #[doc = "No sector alignment check"]
    ZZ133 = 0x0,
    #[doc = "Sector alignment check"]
    ZZ134 = 0x01,
}
impl ScrAlignChk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrAlignChk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrAlignChk {
    #[inline(always)]
    fn from(val: u8) -> ScrAlignChk {
        ScrAlignChk::from_bits(val)
    }
}
impl From<ScrAlignChk> for u8 {
    #[inline(always)]
    fn from(val: ScrAlignChk) -> u8 {
        ScrAlignChk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SetFail {
    #[doc = "FAIL flag should not be set on command exit (no failure detected)"]
    ZZ165 = 0x0,
    #[doc = "FAIL flag should be set on command exit"]
    ZZ166 = 0x01,
}
impl SetFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SetFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SetFail {
    #[inline(always)]
    fn from(val: u8) -> SetFail {
        SetFail::from_bits(val)
    }
}
impl From<SetFail> for u8 {
    #[inline(always)]
    fn from(val: SetFail) -> u8 {
        SetFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleRd {
    #[doc = "Normal UINT operation"]
    ZZ151 = 0x0,
    #[doc = "UINT configured for single cycle reads"]
    ZZ152 = 0x01,
}
impl SingleRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleRd {
    #[inline(always)]
    fn from(val: u8) -> SingleRd {
        SingleRd::from_bits(val)
    }
}
impl From<SingleRd> for u8 {
    #[inline(always)]
    fn from(val: SingleRd) -> u8 {
        SingleRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwArray {
    #[doc = "Main array"]
    ZZ425 = 0x0,
    #[doc = "IFR space only or main (and REDEN space) with IFR space for mass erase"]
    ZZ426 = 0x01,
    #[doc = "IFR1 space"]
    ZZ427 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "REDEN space"]
    ZZ428 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SmwArray {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwArray {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwArray {
    #[inline(always)]
    fn from(val: u8) -> SmwArray {
        SmwArray::from_bits(val)
    }
}
impl From<SmwArray> for u8 {
    #[inline(always)]
    fn from(val: SmwArray) -> u8 {
        SmwArray::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwArray1Smw0Sel {
    #[doc = "Select block 0"]
    ZZ171 = 0x0,
    #[doc = "Select block 1"]
    ZZ172 = 0x01,
}
impl SmwArray1Smw0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwArray1Smw0Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwArray1Smw0Sel {
    #[inline(always)]
    fn from(val: u8) -> SmwArray1Smw0Sel {
        SmwArray1Smw0Sel::from_bits(val)
    }
}
impl From<SmwArray1Smw0Sel> for u8 {
    #[inline(always)]
    fn from(val: SmwArray1Smw0Sel) -> u8 {
        SmwArray1Smw0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwBusy {
    #[doc = "SMW command not active"]
    ZZ203 = 0x0,
    #[doc = "SMW command is active"]
    ZZ204 = 0x01,
}
impl SmwBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwBusy {
    #[inline(always)]
    fn from(val: u8) -> SmwBusy {
        SmwBusy::from_bits(val)
    }
}
impl From<SmwBusy> for u8 {
    #[inline(always)]
    fn from(val: SmwBusy) -> u8 {
        SmwBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwErr {
    #[doc = "Error not detected"]
    ZZ205 = 0x0,
    #[doc = "Error detected"]
    ZZ206 = 0x01,
}
impl SmwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwErr {
    #[inline(always)]
    fn from(val: u8) -> SmwErr {
        SmwErr::from_bits(val)
    }
}
impl From<SmwErr> for u8 {
    #[inline(always)]
    fn from(val: SmwErr) -> u8 {
        SmwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwrCtl {
    #[doc = "SMWR IP disabled"]
    ZZ105 = 0x0,
    #[doc = "SMWR IP enabled"]
    ZZ106 = 0x01,
}
impl SmwrCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwrCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwrCtl {
    #[inline(always)]
    fn from(val: u8) -> SmwrCtl {
        SmwrCtl::from_bits(val)
    }
}
impl From<SmwrCtl> for u8 {
    #[inline(always)]
    fn from(val: SmwrCtl) -> u8 {
        SmwrCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smwtst {
    #[doc = "Default"]
    ZZ209 = 0x0,
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 0"]
    ZZ210 = 0x01,
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 1"]
    ZZ211 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Smwtst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smwtst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smwtst {
    #[inline(always)]
    fn from(val: u8) -> Smwtst {
        Smwtst::from_bits(val)
    }
}
impl From<Smwtst> for u8 {
    #[inline(always)]
    fn from(val: Smwtst) -> u8 {
        Smwtst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocEccCtl {
    #[doc = "ECC is enabled for SOC read access"]
    ZZ101 = 0x0,
    #[doc = "ECC is disabled for SOC read access"]
    ZZ102 = 0x01,
}
impl SocEccCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocEccCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocEccCtl {
    #[inline(always)]
    fn from(val: u8) -> SocEccCtl {
        SocEccCtl::from_bits(val)
    }
}
impl From<SocEccCtl> for u8 {
    #[inline(always)]
    fn from(val: SocEccCtl) -> u8 {
        SocEccCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimDone {
    #[doc = "SoC Trim registers have not been updated"]
    ZZ83 = 0x0,
    #[doc = "All SoC Trim registers have been updated"]
    ZZ84 = 0x01,
}
impl SocTrimDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimDone {
    #[inline(always)]
    fn from(val: u8) -> SocTrimDone {
        SocTrimDone::from_bits(val)
    }
}
impl From<SocTrimDone> for u8 {
    #[inline(always)]
    fn from(val: SocTrimDone) -> u8 {
        SocTrimDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimEcc {
    #[doc = "C0DE_C0DEh check failed"]
    ZZ85 = 0x0,
    #[doc = "C0DE_C0DEh check passed"]
    ZZ86 = 0x01,
}
impl SocTrimEcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimEcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimEcc {
    #[inline(always)]
    fn from(val: u8) -> SocTrimEcc {
        SocTrimEcc::from_bits(val)
    }
}
impl From<SocTrimEcc> for u8 {
    #[inline(always)]
    fn from(val: SocTrimEcc) -> u8 {
        SocTrimEcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimEn {
    #[doc = "C0DE_C0DEh check not attempted"]
    ZZ87 = 0x0,
    #[doc = "C0DE_C0DEh check completed"]
    ZZ88 = 0x01,
}
impl SocTrimEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimEn {
    #[inline(always)]
    fn from(val: u8) -> SocTrimEn {
        SocTrimEn::from_bits(val)
    }
}
impl From<SocTrimEn> for u8 {
    #[inline(always)]
    fn from(val: SocTrimEn) -> u8 {
        SocTrimEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status0 {
    #[doc = "BIST test passed on flash block 0"]
    ZZ331 = 0x0,
    #[doc = "BIST test failed on flash block 0"]
    ZZ332 = 0x01,
}
impl Status0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status0 {
    #[inline(always)]
    fn from(val: u8) -> Status0 {
        Status0::from_bits(val)
    }
}
impl From<Status0> for u8 {
    #[inline(always)]
    fn from(val: Status0) -> u8 {
        Status0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status1 {
    #[doc = "BIST test passed on flash block 1"]
    ZZ329 = 0x0,
    #[doc = "BIST test failed on flash block 1"]
    ZZ330 = 0x01,
}
impl Status1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status1 {
    #[inline(always)]
    fn from(val: u8) -> Status1 {
        Status1::from_bits(val)
    }
}
impl From<Status1> for u8 {
    #[inline(always)]
    fn from(val: Status1) -> u8 {
        Status1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TersCtrl0 {
    #[doc = "50 usec"]
    ZZ397 = 0x0,
    #[doc = "100 usec"]
    ZZ398 = 0x01,
    #[doc = "200 usec"]
    ZZ399 = 0x02,
    #[doc = "300 usec"]
    ZZ400 = 0x03,
    #[doc = "500 usec"]
    ZZ401 = 0x04,
    #[doc = "1 msec"]
    ZZ402 = 0x05,
    #[doc = "1.5 msec"]
    ZZ403 = 0x06,
    #[doc = "2 msec"]
    ZZ404 = 0x07,
}
impl TersCtrl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TersCtrl0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TersCtrl0 {
    #[inline(always)]
    fn from(val: u8) -> TersCtrl0 {
        TersCtrl0::from_bits(val)
    }
}
impl From<TersCtrl0> for u8 {
    #[inline(always)]
    fn from(val: TersCtrl0) -> u8 {
        TersCtrl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tlvsunit {
    #[doc = "Clock cycles"]
    ZZ289 = 0x0,
    #[doc = "0.5 usec"]
    ZZ290 = 0x01,
    #[doc = "1 usec"]
    ZZ291 = 0x02,
    #[doc = "10 usec"]
    ZZ292 = 0x03,
    #[doc = "100 usec"]
    ZZ293 = 0x04,
    #[doc = "1 msec"]
    ZZ294 = 0x05,
    #[doc = "10 msec"]
    ZZ295 = 0x06,
    #[doc = "100 msec"]
    ZZ296 = 0x07,
}
impl Tlvsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tlvsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tlvsunit {
    #[inline(always)]
    fn from(val: u8) -> Tlvsunit {
        Tlvsunit::from_bits(val)
    }
}
impl From<Tlvsunit> for u8 {
    #[inline(always)]
    fn from(val: Tlvsunit) -> u8 {
        Tlvsunit::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TmToAtx(u8);
impl TmToAtx {
    #[doc = "TM\\[0\\] to ATX0"]
    pub const ZZ441: Self = Self(0x01);
    #[doc = "TM\\[1\\] to ATX0"]
    pub const ZZ442: Self = Self(0x02);
    #[doc = "TM\\[2\\] to ATX0"]
    pub const ZZ443: Self = Self(0x04);
    #[doc = "TM\\[3\\] to ATX0"]
    pub const ZZ444: Self = Self(0x08);
    #[doc = "TM\\[0\\] to ATX1"]
    pub const ZZ445: Self = Self(0x10);
    #[doc = "TM\\[1\\] to ATX1"]
    pub const ZZ446: Self = Self(0x20);
    #[doc = "TM\\[2\\] to ATX1"]
    pub const ZZ447: Self = Self(0x40);
    #[doc = "TM\\[3\\] to ATX1"]
    pub const ZZ448: Self = Self(0x80);
}
impl TmToAtx {
    pub const fn from_bits(val: u8) -> TmToAtx {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for TmToAtx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("ZZ441"),
            0x02 => f.write_str("ZZ442"),
            0x04 => f.write_str("ZZ443"),
            0x08 => f.write_str("ZZ444"),
            0x10 => f.write_str("ZZ445"),
            0x20 => f.write_str("ZZ446"),
            0x40 => f.write_str("ZZ447"),
            0x80 => f.write_str("ZZ448"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmToAtx {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "ZZ441"),
            0x02 => defmt::write!(f, "ZZ442"),
            0x04 => defmt::write!(f, "ZZ443"),
            0x08 => defmt::write!(f, "ZZ444"),
            0x10 => defmt::write!(f, "ZZ445"),
            0x20 => defmt::write!(f, "ZZ446"),
            0x40 => defmt::write!(f, "ZZ447"),
            0x80 => defmt::write!(f, "ZZ448"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for TmToAtx {
    #[inline(always)]
    fn from(val: u8) -> TmToAtx {
        TmToAtx::from_bits(val)
    }
}
impl From<TmToAtx> for u8 {
    #[inline(always)]
    fn from(val: TmToAtx) -> u8 {
        TmToAtx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tme {
    #[doc = "Test mode entry not requested"]
    ZZ65 = 0x0,
    #[doc = "Test mode entry requested"]
    ZZ66 = 0x01,
}
impl Tme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tme {
    #[inline(always)]
    fn from(val: u8) -> Tme {
        Tme::from_bits(val)
    }
}
impl From<Tme> for u8 {
    #[inline(always)]
    fn from(val: Tme) -> u8 {
        Tme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmectl {
    #[doc = "FTEST register always reads 0 and writes to FTEST are ignored"]
    ZZ69 = 0x0,
    #[doc = "FTEST register is readable and can be written to enable writability of TME"]
    ZZ70 = 0x01,
}
impl Tmectl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmectl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmectl {
    #[inline(always)]
    fn from(val: u8) -> Tmectl {
        Tmectl::from_bits(val)
    }
}
impl From<Tmectl> for u8 {
    #[inline(always)]
    fn from(val: Tmectl) -> u8 {
        Tmectl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmelock {
    #[doc = "FTEST register not locked from accepting writes"]
    ZZ61 = 0x0,
    #[doc = "FTEST register locked from accepting writes"]
    ZZ62 = 0x01,
}
impl Tmelock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmelock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmelock {
    #[inline(always)]
    fn from(val: u8) -> Tmelock {
        Tmelock::from_bits(val)
    }
}
impl From<Tmelock> for u8 {
    #[inline(always)]
    fn from(val: Tmelock) -> u8 {
        Tmelock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmewr {
    #[doc = "TME bit is not writable"]
    ZZ67 = 0x0,
    #[doc = "TME bit is writable"]
    ZZ68 = 0x01,
}
impl Tmewr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmewr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmewr {
    #[inline(always)]
    fn from(val: u8) -> Tmewr {
        Tmewr::from_bits(val)
    }
}
impl From<Tmewr> for u8 {
    #[inline(always)]
    fn from(val: Tmewr) -> u8 {
        Tmewr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmode {
    #[doc = "Test mode not active"]
    ZZ63 = 0x0,
    #[doc = "Test mode active"]
    ZZ64 = 0x01,
}
impl Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tmode {
        Tmode::from_bits(val)
    }
}
impl From<Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tmode) -> u8 {
        Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnvhCtrl {
    #[doc = "2 usec"]
    ZZ377 = 0x0,
    #[doc = "2.5 usec"]
    ZZ378 = 0x01,
    #[doc = "3 usec"]
    ZZ379 = 0x02,
    #[doc = "3.5 usec"]
    ZZ380 = 0x03,
    #[doc = "4 usec"]
    ZZ381 = 0x04,
    #[doc = "4.5 usec"]
    ZZ382 = 0x05,
    #[doc = "5 usec"]
    ZZ383 = 0x06,
    #[doc = "5.5 usec"]
    ZZ384 = 0x07,
}
impl TnvhCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnvhCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnvhCtrl {
    #[inline(always)]
    fn from(val: u8) -> TnvhCtrl {
        TnvhCtrl::from_bits(val)
    }
}
impl From<TnvhCtrl> for u8 {
    #[inline(always)]
    fn from(val: TnvhCtrl) -> u8 {
        TnvhCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnvhunit {
    #[doc = "Clock cycles"]
    ZZ313 = 0x0,
    #[doc = "0.5 usec"]
    ZZ314 = 0x01,
    #[doc = "1 usec"]
    ZZ315 = 0x02,
    #[doc = "10 usec"]
    ZZ316 = 0x03,
    #[doc = "100 usec"]
    ZZ317 = 0x04,
    #[doc = "1 msec"]
    ZZ318 = 0x05,
    #[doc = "10 msec"]
    ZZ319 = 0x06,
    #[doc = "100 msec"]
    ZZ320 = 0x07,
}
impl Tnvhunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnvhunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnvhunit {
    #[inline(always)]
    fn from(val: u8) -> Tnvhunit {
        Tnvhunit::from_bits(val)
    }
}
impl From<Tnvhunit> for u8 {
    #[inline(always)]
    fn from(val: Tnvhunit) -> u8 {
        Tnvhunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnvsCtrl {
    #[doc = "5 usec"]
    ZZ385 = 0x0,
    #[doc = "8 usec"]
    ZZ386 = 0x01,
    #[doc = "11 usec"]
    ZZ387 = 0x02,
    #[doc = "14 usec"]
    ZZ388 = 0x03,
    #[doc = "17 usec"]
    ZZ389 = 0x04,
    #[doc = "20 usec"]
    ZZ390 = 0x05,
    #[doc = "23 usec"]
    ZZ391 = 0x06,
    #[doc = "26 usec"]
    ZZ392 = 0x07,
}
impl TnvsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnvsCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnvsCtrl {
    #[inline(always)]
    fn from(val: u8) -> TnvsCtrl {
        TnvsCtrl::from_bits(val)
    }
}
impl From<TnvsCtrl> for u8 {
    #[inline(always)]
    fn from(val: TnvsCtrl) -> u8 {
        TnvsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnvsunit {
    #[doc = "Clock cycles"]
    ZZ321 = 0x0,
    #[doc = "0.5 usec"]
    ZZ322 = 0x01,
    #[doc = "1 usec"]
    ZZ323 = 0x02,
    #[doc = "10 usec"]
    ZZ324 = 0x03,
    #[doc = "100 usec"]
    ZZ325 = 0x04,
    #[doc = "1 msec"]
    ZZ326 = 0x05,
    #[doc = "10 msec"]
    ZZ327 = 0x06,
    #[doc = "100 msec"]
    ZZ328 = 0x07,
}
impl Tnvsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnvsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnvsunit {
    #[inline(always)]
    fn from(val: u8) -> Tnvsunit {
        Tnvsunit::from_bits(val)
    }
}
impl From<Tnvsunit> for u8 {
    #[inline(always)]
    fn from(val: Tnvsunit) -> u8 {
        Tnvsunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgmCtrl {
    #[doc = "1 usec"]
    ZZ393 = 0x0,
    #[doc = "2 usec"]
    ZZ394 = 0x01,
    #[doc = "4 usec"]
    ZZ395 = 0x02,
    #[doc = "8 usec"]
    ZZ396 = 0x03,
}
impl TpgmCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgmCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgmCtrl {
    #[inline(always)]
    fn from(val: u8) -> TpgmCtrl {
        TpgmCtrl::from_bits(val)
    }
}
impl From<TpgmCtrl> for u8 {
    #[inline(always)]
    fn from(val: TpgmCtrl) -> u8 {
        TpgmCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgmOpt {
    #[doc = "Fixed Tpgm for all shots, except post shot"]
    ZZ349 = 0x0,
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 4 usec"]
    ZZ350 = 0x01,
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 8 usec"]
    ZZ351 = 0x02,
    #[doc = "Unused"]
    ZZ352 = 0x03,
}
impl TpgmOpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgmOpt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgmOpt {
    #[inline(always)]
    fn from(val: u8) -> TpgmOpt {
        TpgmOpt::from_bits(val)
    }
}
impl From<TpgmOpt> for u8 {
    #[inline(always)]
    fn from(val: TpgmOpt) -> u8 {
        TpgmOpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgsCtrl {
    #[doc = "1 usec"]
    ZZ369 = 0x0,
    #[doc = "2 usec"]
    ZZ370 = 0x01,
    #[doc = "3 usec"]
    ZZ371 = 0x02,
    #[doc = "4 usec"]
    ZZ372 = 0x03,
    #[doc = "5 usec"]
    ZZ373 = 0x04,
    #[doc = "6 usec"]
    ZZ374 = 0x05,
    #[doc = "7 usec"]
    ZZ375 = 0x06,
    #[doc = "8 usec"]
    ZZ376 = 0x07,
}
impl TpgsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgsCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgsCtrl {
    #[inline(always)]
    fn from(val: u8) -> TpgsCtrl {
        TpgsCtrl::from_bits(val)
    }
}
impl From<TpgsCtrl> for u8 {
    #[inline(always)]
    fn from(val: TpgsCtrl) -> u8 {
        TpgsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpgsunit {
    #[doc = "Clock cycles"]
    ZZ305 = 0x0,
    #[doc = "0.5 usec"]
    ZZ306 = 0x01,
    #[doc = "1 usec"]
    ZZ307 = 0x02,
    #[doc = "10 usec"]
    ZZ308 = 0x03,
    #[doc = "100 usec"]
    ZZ309 = 0x04,
    #[doc = "1 msec"]
    ZZ310 = 0x05,
    #[doc = "10 msec"]
    ZZ311 = 0x06,
    #[doc = "100 msec"]
    ZZ312 = 0x07,
}
impl Tpgsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpgsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpgsunit {
    #[inline(always)]
    fn from(val: u8) -> Tpgsunit {
        Tpgsunit::from_bits(val)
    }
}
impl From<Tpgsunit> for u8 {
    #[inline(always)]
    fn from(val: Tpgsunit) -> u8 {
        Tpgsunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trcvunit {
    #[doc = "Clock cycles"]
    ZZ297 = 0x0,
    #[doc = "0.5 usec"]
    ZZ298 = 0x01,
    #[doc = "1 usec"]
    ZZ299 = 0x02,
    #[doc = "10 usec"]
    ZZ300 = 0x03,
    #[doc = "100 usec"]
    ZZ301 = 0x04,
    #[doc = "1 msec"]
    ZZ302 = 0x05,
    #[doc = "10 msec"]
    ZZ303 = 0x06,
    #[doc = "100 msec"]
    ZZ304 = 0x07,
}
impl Trcvunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trcvunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trcvunit {
    #[inline(always)]
    fn from(val: u8) -> Trcvunit {
        Trcvunit::from_bits(val)
    }
}
impl From<Trcvunit> for u8 {
    #[inline(always)]
    fn from(val: Trcvunit) -> u8 {
        Trcvunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstctl {
    #[doc = "Default, disable both BIST self-test and MISR"]
    ZZ215 = 0x0,
    #[doc = "Enable BIST self-test mode DOUT from macro will be forced to '0', and disable MISR."]
    ZZ216 = 0x01,
    #[doc = "Enable MISR"]
    ZZ217 = 0x02,
    #[doc = "Enable both BIST self-test mode and MISR"]
    ZZ218 = 0x03,
}
impl Tstctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstctl {
    #[inline(always)]
    fn from(val: u8) -> Tstctl {
        Tstctl::from_bits(val)
    }
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(val: Tstctl) -> u8 {
        Tstctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserEv {
    #[doc = "EV input to the flash array is driven LOW"]
    ZZ419 = 0x0,
    #[doc = "EV input to the flash array is driven HIGH"]
    ZZ420 = 0x01,
}
impl UserEv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserEv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserEv {
    #[inline(always)]
    fn from(val: u8) -> UserEv {
        UserEv::from_bits(val)
    }
}
impl From<UserEv> for u8 {
    #[inline(always)]
    fn from(val: UserEv) -> u8 {
        UserEv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserHem {
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven LOW"]
    ZZ413 = 0x0,
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven HIGH"]
    ZZ414 = 0x01,
}
impl UserHem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserHem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserHem {
    #[inline(always)]
    fn from(val: u8) -> UserHem {
        UserHem::from_bits(val)
    }
}
impl From<UserHem> for u8 {
    #[inline(always)]
    fn from(val: UserHem) -> u8 {
        UserHem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserIfren {
    #[doc = "IFREN input to the flash array is driven LOW"]
    ZZ417 = 0x0,
    #[doc = "IFREN input to the flash array is driven HIGH"]
    ZZ418 = 0x01,
}
impl UserIfren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserIfren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserIfren {
    #[inline(always)]
    fn from(val: u8) -> UserIfren {
        UserIfren::from_bits(val)
    }
}
impl From<UserIfren> for u8 {
    #[inline(always)]
    fn from(val: UserIfren) -> u8 {
        UserIfren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserIfren1 {
    #[doc = "IFREN1 input to the flash array is driven LOW"]
    ZZ423 = 0x0,
    #[doc = "IFREN1 input to the flash array is driven HIGH"]
    ZZ424 = 0x01,
}
impl UserIfren1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserIfren1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserIfren1 {
    #[inline(always)]
    fn from(val: u8) -> UserIfren1 {
        UserIfren1::from_bits(val)
    }
}
impl From<UserIfren1> for u8 {
    #[inline(always)]
    fn from(val: UserIfren1) -> u8 {
        UserIfren1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserPv {
    #[doc = "PV input to the flash array is driven LOW"]
    ZZ421 = 0x0,
    #[doc = "PV input to the flash array is driven HIGH"]
    ZZ422 = 0x01,
}
impl UserPv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserPv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserPv {
    #[inline(always)]
    fn from(val: u8) -> UserPv {
        UserPv::from_bits(val)
    }
}
impl From<UserPv> for u8 {
    #[inline(always)]
    fn from(val: UserPv) -> u8 {
        UserPv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserReden {
    #[doc = "REDEN input to the flash array is driven LOW"]
    ZZ415 = 0x0,
    #[doc = "REDEN input to the flash array is driven HIGH"]
    ZZ416 = 0x01,
}
impl UserReden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserReden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserReden {
    #[inline(always)]
    fn from(val: u8) -> UserReden {
        UserReden::from_bits(val)
    }
}
impl From<UserReden> for u8 {
    #[inline(always)]
    fn from(val: UserReden) -> u8 {
        UserReden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfyOpt {
    #[doc = "Skip verify for post shot only, verify for all other shots"]
    ZZ353 = 0x0,
    #[doc = "Skip verify for the 1st and post shots"]
    ZZ354 = 0x01,
    #[doc = "Skip the 1st, 2nd, and post shots"]
    ZZ355 = 0x02,
    #[doc = "Skip verify for all shots"]
    ZZ356 = 0x03,
}
impl VfyOpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfyOpt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfyOpt {
    #[inline(always)]
    fn from(val: u8) -> VfyOpt {
        VfyOpt::from_bits(val)
    }
}
impl From<VfyOpt> for u8 {
    #[inline(always)]
    fn from(val: VfyOpt) -> u8 {
        VfyOpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WaitEn {
    #[doc = "Wait feature disabled"]
    ZZ191 = 0x0,
    #[doc = "Wait feature enabled"]
    ZZ192 = 0x01,
}
impl WaitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WaitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WaitEn {
    #[inline(always)]
    fn from(val: u8) -> WaitEn {
        WaitEn::from_bits(val)
    }
}
impl From<WaitEn> for u8 {
    #[inline(always)]
    fn from(val: WaitEn) -> u8 {
        WaitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WideLoad {
    #[doc = "Wide load mode disabled (default)"]
    ZZ153 = 0x0,
    #[doc = "Wide load mode enabled"]
    ZZ154 = 0x01,
}
impl WideLoad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WideLoad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WideLoad {
    #[inline(always)]
    fn from(val: u8) -> WideLoad {
        WideLoad::from_bits(val)
    }
}
impl From<WideLoad> for u8 {
    #[inline(always)]
    fn from(val: WideLoad) -> u8 {
        WideLoad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrPathEccEn {
    #[doc = "ECC encoding disabled"]
    ZZ179 = 0x0,
    #[doc = "ECC encoding enabled"]
    ZZ180 = 0x01,
}
impl WrPathEccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrPathEccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrPathEccEn {
    #[inline(always)]
    fn from(val: u8) -> WrPathEccEn {
        WrPathEccEn::from_bits(val)
    }
}
impl From<WrPathEccEn> for u8 {
    #[inline(always)]
    fn from(val: WrPathEccEn) -> u8 {
        WrPathEccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrPathEn {
    #[doc = "Writes to BIST setting registers driven by MM_WDATA"]
    ZZ181 = 0x0,
    #[doc = "Writes to BIST setting registers driven by SMW_DIN"]
    ZZ182 = 0x01,
}
impl WrPathEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrPathEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrPathEn {
    #[inline(always)]
    fn from(val: u8) -> WrPathEn {
        WrPathEn::from_bits(val)
    }
}
impl From<WrPathEn> for u8 {
    #[inline(always)]
    fn from(val: WrPathEn) -> u8 {
        WrPathEn::to_bits(val)
    }
}
