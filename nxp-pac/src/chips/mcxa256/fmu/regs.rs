#[doc = "Flash Common Command Object Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob(pub u32);
impl Fccob {
    #[doc = "CCOBn"]
    #[must_use]
    #[inline(always)]
    pub const fn ccobn(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CCOBn"]
    #[inline(always)]
    pub const fn set_ccobn(&mut self, val: u32) {
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
            .field("ccobn", &self.ccobn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob {{ ccobn: {=u32:?} }}", self.ccobn())
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
    #[doc = "Mass Erase Request"]
    #[must_use]
    #[inline(always)]
    pub const fn ersreq(&self) -> super::vals::Ersreq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Ersreq::from_bits(val as u8)
    }
    #[doc = "Mass Erase Request"]
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
    pub const fn rwsc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Wait-State Control"]
    #[inline(always)]
    pub const fn set_rwsc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Low speed active mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lsactive(&self) -> super::vals::Lsactive {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lsactive::from_bits(val as u8)
    }
    #[doc = "Low speed active mode"]
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
            "Fctrl {{ rwsc: {=u8:?}, lsactive: {:?}, fdfd: {:?}, abtreq: {:?} }}",
            self.rwsc(),
            self.lsactive(),
            self.fdfd(),
            self.abtreq()
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
    #[doc = "Command protection level"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdprt(&self) -> super::vals::Cmdprt {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Cmdprt::from_bits(val as u8)
    }
    #[doc = "Command protection level"]
    #[inline(always)]
    pub const fn set_cmdprt(&mut self, val: super::vals::Cmdprt) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Command protection status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdp(&self) -> super::vals::Cmdp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Cmdp::from_bits(val as u8)
    }
    #[doc = "Command protection status flag"]
    #[inline(always)]
    pub const fn set_cmdp(&mut self, val: super::vals::Cmdp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Command domain ID"]
    #[must_use]
    #[inline(always)]
    pub const fn cmddid(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Command domain ID"]
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
    #[doc = "Program-Erase Ready Control/Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn perdy(&self) -> super::vals::Perdy {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Perdy::from_bits(val as u8)
    }
    #[doc = "Program-Erase Ready Control/Status Flag"]
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
