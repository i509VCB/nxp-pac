#[doc = "Channel Arbitration Group"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChGrpri(pub u32);
impl ChGrpri {
    #[doc = "Arbitration Group For Channel n"]
    #[must_use]
    #[inline(always)]
    pub const fn grpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Arbitration Group For Channel n"]
    #[inline(always)]
    pub const fn set_grpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for ChGrpri {
    #[inline(always)]
    fn default() -> ChGrpri {
        ChGrpri(0)
    }
}
impl core::fmt::Debug for ChGrpri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChGrpri")
            .field("grpri", &self.grpri())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChGrpri {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChGrpri {{ grpri: {=u8:?} }}", self.grpri())
    }
}
#[doc = "Management Page Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpCsr(pub u32);
impl MpCsr {
    #[doc = "Enable Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug"]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Halt After Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hae(&self) -> super::vals::Hae {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hae::from_bits(val as u8)
    }
    #[doc = "Halt After Error"]
    #[inline(always)]
    pub const fn set_hae(&mut self, val: super::vals::Hae) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt DMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Halt DMA Operations"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Channel Linking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn gclc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Channel Linking Control"]
    #[inline(always)]
    pub const fn set_gclc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Master ID Replication Control"]
    #[must_use]
    #[inline(always)]
    pub const fn gmrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Master ID Replication Control"]
    #[inline(always)]
    pub const fn set_gmrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Cancel Transfer With Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::MpCsrEcx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MpCsrEcx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer With Error"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::MpCsrEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> super::vals::Cx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer"]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: super::vals::Cx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Active Channel ID"]
    #[must_use]
    #[inline(always)]
    pub const fn active_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Active Channel ID"]
    #[inline(always)]
    pub const fn set_active_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DMA Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "DMA Active Status"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpCsr {
    #[inline(always)]
    fn default() -> MpCsr {
        MpCsr(0)
    }
}
impl core::fmt::Debug for MpCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpCsr")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hae", &self.hae())
            .field("halt", &self.halt())
            .field("gclc", &self.gclc())
            .field("gmrc", &self.gmrc())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active_id", &self.active_id())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpCsr {{ edbg: {=bool:?}, erca: {=bool:?}, hae: {:?}, halt: {:?}, gclc: {=bool:?}, gmrc: {=bool:?}, ecx: {:?}, cx: {:?}, active_id: {=u8:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.hae(),
            self.halt(),
            self.gclc(),
            self.gmrc(),
            self.ecx(),
            self.cx(),
            self.active_id(),
            self.active()
        )
    }
}
#[doc = "Management Page Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpEs(pub u32);
impl MpEs {
    #[doc = "Destination Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> super::vals::Dbe {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dbe::from_bits(val as u8)
    }
    #[doc = "Destination Bus Error"]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: super::vals::Dbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> super::vals::Sbe {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Sbe::from_bits(val as u8)
    }
    #[doc = "Source Bus Error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: super::vals::Sbe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> super::vals::Sge {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Sge::from_bits(val as u8)
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: super::vals::Sge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> super::vals::Nce {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Nce::from_bits(val as u8)
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: super::vals::Nce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> super::vals::Doe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Doe::from_bits(val as u8)
    }
    #[doc = "Destination Offset Error"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: super::vals::Doe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> super::vals::Dae {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dae::from_bits(val as u8)
    }
    #[doc = "Destination Address Error"]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: super::vals::Dae) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> super::vals::Soe {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Soe::from_bits(val as u8)
    }
    #[doc = "Source Offset Error"]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: super::vals::Soe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> super::vals::Sae {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sae::from_bits(val as u8)
    }
    #[doc = "Source Address Error"]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: super::vals::Sae) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer Canceled"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::MpEsEcx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MpEsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::MpEsEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> super::vals::Vld {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Vld::from_bits(val as u8)
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: super::vals::Vld) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpEs {
    #[inline(always)]
    fn default() -> MpEs {
        MpEs(0)
    }
}
impl core::fmt::Debug for MpEs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpEs")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("ecx", &self.ecx())
            .field("errchn", &self.errchn())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpEs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpEs {{ dbe: {:?}, sbe: {:?}, sge: {:?}, nce: {:?}, doe: {:?}, dae: {:?}, soe: {:?}, sae: {:?}, ecx: {:?}, errchn: {=u8:?}, vld: {:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.ecx(),
            self.errchn(),
            self.vld()
        )
    }
}
#[doc = "Management Page Hardware Request Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpHrs(pub u32);
impl MpHrs {
    #[doc = "Hardware Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Hardware Request Status"]
    #[inline(always)]
    pub const fn set_hrs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MpHrs {
    #[inline(always)]
    fn default() -> MpHrs {
        MpHrs(0)
    }
}
impl core::fmt::Debug for MpHrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpHrs").field("hrs", &self.hrs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MpHrs {{ hrs: {=u32:?} }}", self.hrs())
    }
}
#[doc = "Management Page Interrupt Request Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpInt(pub u32);
impl MpInt {
    #[doc = "Interrupt Request Status"]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Interrupt Request Status"]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MpInt {
    #[inline(always)]
    fn default() -> MpInt {
        MpInt(0)
    }
}
impl core::fmt::Debug for MpInt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpInt").field("int", &self.int()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpInt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MpInt {{ int: {=u16:?} }}", self.int())
    }
}
