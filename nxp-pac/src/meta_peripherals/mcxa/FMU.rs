#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Flash."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmu {
    ptr: *mut u8,
}
unsafe impl Send for Fmu {}
unsafe impl Sync for Fmu {}
impl Fmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Status Register."]
    #[inline(always)]
    pub const fn fstat(self) -> crate::pac::common::Reg<Fstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash Configuration Register."]
    #[inline(always)]
    pub const fn fcnfg(self) -> crate::pac::common::Reg<Fcnfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash Control Register."]
    #[inline(always)]
    pub const fn fctrl(self) -> crate::pac::common::Reg<Fctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Test Register."]
    #[inline(always)]
    pub const fn ftest(self) -> crate::pac::common::Reg<Ftest, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Flash Common Command Object Registers."]
    #[inline(always)]
    pub const fn fccob(self, n: usize) -> crate::pac::common::Reg<Fccob, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
}
#[doc = "Flash Common Command Object Registers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob(pub u32);
impl Fccob {
    #[doc = "CCOBn."]
    #[must_use]
    #[inline(always)]
    pub const fn cco_bn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CCOBn."]
    #[inline(always)]
    pub const fn set_cco_bn(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob {
    #[inline(always)]
    fn default() -> Fccob {
        Fccob(0)
    }
}
impl core::fmt::Debug for Fccob {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob")
            .field("cco_bn", &self.cco_bn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob {{ cco_bn: {=u32:?} }}", self.cco_bn())
    }
}
#[doc = "Flash Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcnfg(pub u32);
impl Fcnfg {
    #[doc = "Command Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccie(&self) -> Ccie {
        let val = (self.0 >> 7usize) & 0x01;
        Ccie::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ccie(&mut self, val: Ccie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mass Erase Request."]
    #[must_use]
    #[inline(always)]
    pub const fn ersreq(&self) -> Ersreq {
        let val = (self.0 >> 8usize) & 0x01;
        Ersreq::from_bits(val as u8)
    }
    #[doc = "Mass Erase Request."]
    #[inline(always)]
    pub const fn set_ersreq(&mut self, val: Ersreq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dfdie(&self) -> Dfdie {
        let val = (self.0 >> 16usize) & 0x01;
        Dfdie::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dfdie(&mut self, val: Dfdie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ersien0(&self) -> Ersien0 {
        let val = (self.0 >> 24usize) & 0x0f;
        Ersien0::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 0."]
    #[inline(always)]
    pub const fn set_ersien0(&mut self, val: Ersien0) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)."]
    #[must_use]
    #[inline(always)]
    pub const fn ersien1(&self) -> Ersien1 {
        let val = (self.0 >> 28usize) & 0x0f;
        Ersien1::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)."]
    #[inline(always)]
    pub const fn set_ersien1(&mut self, val: Ersien1) {
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
#[doc = "Flash Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Read Wait-State Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rwsc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Wait-State Control."]
    #[inline(always)]
    pub const fn set_rwsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Low speed active mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lsactive(&self) -> Lsactive {
        let val = (self.0 >> 8usize) & 0x01;
        Lsactive::from_bits(val as u8)
    }
    #[doc = "Low speed active mode."]
    #[inline(always)]
    pub const fn set_lsactive(&mut self, val: Lsactive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Force Double Bit Fault Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn fdfd(&self) -> Fdfd {
        let val = (self.0 >> 16usize) & 0x01;
        Fdfd::from_bits(val as u8)
    }
    #[doc = "Force Double Bit Fault Detect."]
    #[inline(always)]
    pub const fn set_fdfd(&mut self, val: Fdfd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Abort Request."]
    #[must_use]
    #[inline(always)]
    pub const fn abtreq(&self) -> Abtreq {
        let val = (self.0 >> 24usize) & 0x01;
        Abtreq::from_bits(val as u8)
    }
    #[doc = "Abort Request."]
    #[inline(always)]
    pub const fn set_abtreq(&mut self, val: Abtreq) {
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
            "Fctrl {{ rwsc: {=u8:?}, lsactive: {:?}, fdfd: {:?}, abtreq: {:?} }}",
            self.rwsc(),
            self.lsactive(),
            self.fdfd(),
            self.abtreq()
        )
    }
}
#[doc = "Flash Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstat(pub u32);
impl Fstat {
    #[doc = "Command Fail Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> Fail {
        let val = (self.0 >> 0usize) & 0x01;
        Fail::from_bits(val as u8)
    }
    #[doc = "Command Fail Flag."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: Fail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Abort Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdabt(&self) -> Cmdabt {
        let val = (self.0 >> 2usize) & 0x01;
        Cmdabt::from_bits(val as u8)
    }
    #[doc = "Command Abort Flag."]
    #[inline(always)]
    pub const fn set_cmdabt(&mut self, val: Cmdabt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Command Protection Violation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pviol(&self) -> Pviol {
        let val = (self.0 >> 4usize) & 0x01;
        Pviol::from_bits(val as u8)
    }
    #[doc = "Command Protection Violation Flag."]
    #[inline(always)]
    pub const fn set_pviol(&mut self, val: Pviol) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command Access Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn accerr(&self) -> Accerr {
        let val = (self.0 >> 5usize) & 0x01;
        Accerr::from_bits(val as u8)
    }
    #[doc = "Command Access Error Flag."]
    #[inline(always)]
    pub const fn set_accerr(&mut self, val: Accerr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Command Write Sequence Abort Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cwsabt(&self) -> Cwsabt {
        let val = (self.0 >> 6usize) & 0x01;
        Cwsabt::from_bits(val as u8)
    }
    #[doc = "Command Write Sequence Abort Flag."]
    #[inline(always)]
    pub const fn set_cwsabt(&mut self, val: Cwsabt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Command Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccif(&self) -> Ccif {
        let val = (self.0 >> 7usize) & 0x01;
        Ccif::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ccif(&mut self, val: Ccif) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Command protection level."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdprt(&self) -> Cmdprt {
        let val = (self.0 >> 8usize) & 0x03;
        Cmdprt::from_bits(val as u8)
    }
    #[doc = "Command protection level."]
    #[inline(always)]
    pub const fn set_cmdprt(&mut self, val: Cmdprt) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Command protection status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdp(&self) -> Cmdp {
        let val = (self.0 >> 11usize) & 0x01;
        Cmdp::from_bits(val as u8)
    }
    #[doc = "Command protection status flag."]
    #[inline(always)]
    pub const fn set_cmdp(&mut self, val: Cmdp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Command domain ID."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddid(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Command domain ID."]
    #[inline(always)]
    pub const fn set_cmddid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dfdif(&self) -> Dfdif {
        let val = (self.0 >> 16usize) & 0x01;
        Dfdif::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dfdif(&mut self, val: Dfdif) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Salvage Used for Erase operation."]
    #[must_use]
    #[inline(always)]
    pub const fn salv_used(&self) -> SalvUsed {
        let val = (self.0 >> 17usize) & 0x01;
        SalvUsed::from_bits(val as u8)
    }
    #[doc = "Salvage Used for Erase operation."]
    #[inline(always)]
    pub const fn set_salv_used(&mut self, val: SalvUsed) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Program-Erase Write Enable Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pewen(&self) -> Pewen {
        let val = (self.0 >> 24usize) & 0x03;
        Pewen::from_bits(val as u8)
    }
    #[doc = "Program-Erase Write Enable Control."]
    #[inline(always)]
    pub const fn set_pewen(&mut self, val: Pewen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Program-Erase Ready Control/Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perdy(&self) -> Perdy {
        let val = (self.0 >> 31usize) & 0x01;
        Perdy::from_bits(val as u8)
    }
    #[doc = "Program-Erase Ready Control/Status Flag."]
    #[inline(always)]
    pub const fn set_perdy(&mut self, val: Perdy) {
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
#[doc = "Flash Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftest(pub u32);
impl Ftest {
    #[doc = "Test Mode Entry Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmectl(&self) -> Tmectl {
        let val = (self.0 >> 0usize) & 0x01;
        Tmectl::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Control."]
    #[inline(always)]
    pub const fn set_tmectl(&mut self, val: Tmectl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Mode Entry Writable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmewr(&self) -> Tmewr {
        let val = (self.0 >> 1usize) & 0x01;
        Tmewr::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Writable."]
    #[inline(always)]
    pub const fn set_tmewr(&mut self, val: Tmewr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Mode Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn tme(&self) -> Tme {
        let val = (self.0 >> 2usize) & 0x01;
        Tme::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry."]
    #[inline(always)]
    pub const fn set_tme(&mut self, val: Tme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Mode Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> Tmode {
        let val = (self.0 >> 3usize) & 0x01;
        Tmode::from_bits(val as u8)
    }
    #[doc = "Test Mode Status."]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: Tmode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Mode Entry Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tmelock(&self) -> Tmelock {
        let val = (self.0 >> 4usize) & 0x01;
        Tmelock::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Lock."]
    #[inline(always)]
    pub const fn set_tmelock(&mut self, val: Tmelock) {
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
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abtreq {
    #[doc = "No request to abort a command write sequence."]
    Abtreq0 = 0x0,
    #[doc = "Request to abort a command write sequence."]
    Abtreq1 = 0x01,
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
    #[doc = "No access error detected."]
    Accerr0 = 0x0,
    #[doc = "Access error detected."]
    Accerr1 = 0x01,
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
pub enum Ccie {
    #[doc = "Command complete interrupt disabled."]
    Ccie0 = 0x0,
    #[doc = "Command complete interrupt enabled."]
    Ccie1 = 0x01,
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
    #[doc = "Flash command, initialization, or power mode recovery in progress."]
    Ccif0 = 0x0,
    #[doc = "Flash command, initialization, or power mode recovery has completed."]
    Ccif1 = 0x01,
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
pub enum Cmdabt {
    #[doc = "No command abort detected."]
    Cmdabt0 = 0x0,
    #[doc = "Command abort detected."]
    Cmdabt1 = 0x01,
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
    #[doc = "Command protection level and domain ID are stale."]
    Cmdp0 = 0x0,
    #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set."]
    Cmdp1 = 0x01,
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
    #[doc = "Secure, normal access."]
    Cmdprt00 = 0x0,
    #[doc = "Secure, privileged access."]
    Cmdprt01 = 0x01,
    #[doc = "Nonsecure, normal access."]
    Cmdprt10 = 0x02,
    #[doc = "Nonsecure, privileged access."]
    Cmdprt11 = 0x03,
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
pub enum Cwsabt {
    #[doc = "Command write sequence not aborted."]
    Cwsabt0 = 0x0,
    #[doc = "Command write sequence aborted."]
    Cwsabt1 = 0x01,
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
pub enum Dfdie {
    #[doc = "Double bit fault detect interrupt disabled."]
    Dfdie0 = 0x0,
    #[doc = "Double bit fault detect interrupt enabled."]
    Dfdie1 = 0x01,
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
    #[doc = "Double bit fault not detected during a valid flash read access."]
    Dfdif0 = 0x0,
    #[doc = "Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access."]
    Dfdif1 = 0x01,
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
pub enum Ersien0 {
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command."]
    Ersien00 = 0x0,
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command."]
    Ersien01 = 0x01,
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
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command."]
    Ersien10 = 0x0,
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command."]
    Ersien11 = 0x01,
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
    #[doc = "No request or request complete."]
    Ersreq0 = 0x0,
    #[doc = "Request to run the Mass Erase operation."]
    Ersreq1 = 0x01,
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
    #[doc = "Error not detected."]
    Fail0 = 0x0,
    #[doc = "Error detected."]
    Fail1 = 0x01,
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
    #[doc = "FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the platform flash controller."]
    Fdfd0 = 0x0,
    #[doc = "FSTAT\\[DFDIF\\] sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    Fdfd1 = 0x01,
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
pub enum Lsactive {
    #[doc = "Full speed active mode requested."]
    Lsactive0 = 0x0,
    #[doc = "Low speed active mode requested."]
    Lsactive1 = 0x01,
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
pub enum Perdy {
    #[doc = "Program or sector erase command operation not stalled."]
    Perdy0 = 0x0,
    #[doc = "Program or sector erase command operation ready to execute."]
    Perdy1 = 0x01,
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
    #[doc = "Writes are not enabled."]
    Pewen00 = 0x0,
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)."]
    Pewen01 = 0x01,
    #[doc = "Writes are enabled for one flash or IFR page (page programming)."]
    Pewen10 = 0x02,
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
pub enum Pviol {
    #[doc = "No protection violation detected."]
    Pviol0 = 0x0,
    #[doc = "Protection violation detected."]
    Pviol1 = 0x01,
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
pub enum SalvUsed {
    #[doc = "Salvage not used during last operation."]
    SalvUsed0 = 0x0,
    #[doc = "Salvage used during the last erase operation."]
    SalvUsed1 = 0x01,
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
pub enum Tme {
    #[doc = "Test mode entry not requested."]
    Tme0 = 0x0,
    #[doc = "Test mode entry requested."]
    Tme1 = 0x01,
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
    #[doc = "FTEST register always reads 0 and writes to FTEST are ignored."]
    Tmectl0 = 0x0,
    #[doc = "FTEST register is readable and can be written to enable writability of TME."]
    Tmectl1 = 0x01,
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
    #[doc = "FTEST register not locked from accepting writes."]
    Tmelock0 = 0x0,
    #[doc = "FTEST register locked from accepting writes."]
    Tmelock1 = 0x01,
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
    #[doc = "TME bit is not writable."]
    Tmewr0 = 0x0,
    #[doc = "TME bit is writable."]
    Tmewr1 = 0x01,
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
    #[doc = "Test mode not active."]
    Tmode0 = 0x0,
    #[doc = "Test mode active."]
    Tmode1 = 0x01,
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
