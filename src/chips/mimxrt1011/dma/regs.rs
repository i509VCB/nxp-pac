#[doc = "Clear DONE Status Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdne(pub u8);
impl Cdne {
    #[doc = "Clear DONE field"]
    #[must_use]
    #[inline(always)]
    pub const fn cdne(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear DONE field"]
    #[inline(always)]
    pub const fn set_cdne(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Clears All DONE fields"]
    #[must_use]
    #[inline(always)]
    pub const fn cadn(&self) -> super::vals::Cadn {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cadn::from_bits(val as u8)
    }
    #[doc = "Clears All DONE fields"]
    #[inline(always)]
    pub const fn set_cadn(&mut self, val: super::vals::Cadn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CdneNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CdneNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CdneNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cdne {
    #[inline(always)]
    fn default() -> Cdne {
        Cdne(0)
    }
}
impl core::fmt::Debug for Cdne {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdne")
            .field("cdne", &self.cdne())
            .field("cadn", &self.cadn())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdne {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cdne {{ cdne: {=u8:?}, cadn: {:?}, nop: {:?} }}",
            self.cdne(),
            self.cadn(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceei(pub u8);
impl Ceei {
    #[doc = "Clear Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn ceei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_ceei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn caee(&self) -> super::vals::Caee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caee::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_caee(&mut self, val: super::vals::Caee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ceei {
    #[inline(always)]
    fn default() -> Ceei {
        Ceei(0)
    }
}
impl core::fmt::Debug for Ceei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ceei")
            .field("ceei", &self.ceei())
            .field("caee", &self.caee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ceei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ceei {{ ceei: {=u8:?}, caee: {:?}, nop: {:?} }}",
            self.ceei(),
            self.caee(),
            self.nop()
        )
    }
}
#[doc = "Clear Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerq(pub u8);
impl Cerq {
    #[doc = "Clear Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cerq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Enable Request"]
    #[inline(always)]
    pub const fn set_cerq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Clear All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn caer(&self) -> super::vals::Caer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caer::from_bits(val as u8)
    }
    #[doc = "Clear All Enable Requests"]
    #[inline(always)]
    pub const fn set_caer(&mut self, val: super::vals::Caer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerq {
    #[inline(always)]
    fn default() -> Cerq {
        Cerq(0)
    }
}
impl core::fmt::Debug for Cerq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerq")
            .field("cerq", &self.cerq())
            .field("caer", &self.caer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerq {{ cerq: {=u8:?}, caer: {:?}, nop: {:?} }}",
            self.cerq(),
            self.caer(),
            self.nop()
        )
    }
}
#[doc = "Clear Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerr(pub u8);
impl Cerr {
    #[doc = "Clear Error Indicator"]
    #[must_use]
    #[inline(always)]
    pub const fn cerr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Error Indicator"]
    #[inline(always)]
    pub const fn set_cerr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Clear All Error Indicators"]
    #[must_use]
    #[inline(always)]
    pub const fn caei(&self) -> super::vals::Caei {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Caei::from_bits(val as u8)
    }
    #[doc = "Clear All Error Indicators"]
    #[inline(always)]
    pub const fn set_caei(&mut self, val: super::vals::Caei) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CerrNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CerrNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CerrNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cerr {
    #[inline(always)]
    fn default() -> Cerr {
        Cerr(0)
    }
}
impl core::fmt::Debug for Cerr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cerr")
            .field("cerr", &self.cerr())
            .field("caei", &self.caei())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cerr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cerr {{ cerr: {=u8:?}, caei: {:?}, nop: {:?} }}",
            self.cerr(),
            self.caei(),
            self.nop()
        )
    }
}
#[doc = "Clear Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cint(pub u8);
impl Cint {
    #[doc = "Clear Interrupt Request"]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clear Interrupt Request"]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Clear All Interrupt Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn cair(&self) -> super::vals::Cair {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Cair::from_bits(val as u8)
    }
    #[doc = "Clear All Interrupt Requests"]
    #[inline(always)]
    pub const fn set_cair(&mut self, val: super::vals::Cair) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::CintNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CintNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::CintNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Cint {
    #[inline(always)]
    fn default() -> Cint {
        Cint(0)
    }
}
impl core::fmt::Debug for Cint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cint")
            .field("cint", &self.cint())
            .field("cair", &self.cair())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cint {{ cint: {=u8:?}, cair: {:?}, nop: {:?} }}",
            self.cint(),
            self.cair(),
            self.nop()
        )
    }
}
#[doc = "Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
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
    #[doc = "Halt On Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hoe(&self) -> super::vals::Hoe {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Hoe::from_bits(val as u8)
    }
    #[doc = "Halt On Error"]
    #[inline(always)]
    pub const fn set_hoe(&mut self, val: super::vals::Hoe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt eDMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> super::vals::Halt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Halt::from_bits(val as u8)
    }
    #[doc = "Halt eDMA Operations"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: super::vals::Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Continuous Link Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn clm(&self) -> super::vals::Clm {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Clm::from_bits(val as u8)
    }
    #[doc = "Continuous Link Mode"]
    #[inline(always)]
    pub const fn set_clm(&mut self, val: super::vals::Clm) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn emlm(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Minor Loop Mapping"]
    #[inline(always)]
    pub const fn set_emlm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::CrEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::CrEcx::from_bits(val as u8)
    }
    #[doc = "Error Cancel Transfer"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::CrEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Cancel Transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> super::vals::Cx {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer"]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: super::vals::Cx) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "eDMA Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> super::vals::Active {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Active::from_bits(val as u8)
    }
    #[doc = "eDMA Active Status"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: super::vals::Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hoe", &self.hoe())
            .field("halt", &self.halt())
            .field("clm", &self.clm())
            .field("emlm", &self.emlm())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cr {{ edbg: {=bool:?}, erca: {=bool:?}, hoe: {:?}, halt: {:?}, clm: {:?}, emlm: {=bool:?}, ecx: {:?}, cx: {:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.hoe(),
            self.halt(),
            self.clm(),
            self.emlm(),
            self.ecx(),
            self.cx(),
            self.active()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri0(pub u8);
impl Dchpri0 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri0Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri0Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri0Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri0 {
    #[inline(always)]
    fn default() -> Dchpri0 {
        Dchpri0(0)
    }
}
impl core::fmt::Debug for Dchpri0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri0")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri0 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri1(pub u8);
impl Dchpri1 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri1Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri1Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri1Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri1 {
    #[inline(always)]
    fn default() -> Dchpri1 {
        Dchpri1(0)
    }
}
impl core::fmt::Debug for Dchpri1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri1")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri1 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri10(pub u8);
impl Dchpri10 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri10Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri10Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri10Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri10 {
    #[inline(always)]
    fn default() -> Dchpri10 {
        Dchpri10(0)
    }
}
impl core::fmt::Debug for Dchpri10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri10")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri10 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri11(pub u8);
impl Dchpri11 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri11Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri11Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri11Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri11 {
    #[inline(always)]
    fn default() -> Dchpri11 {
        Dchpri11(0)
    }
}
impl core::fmt::Debug for Dchpri11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri11")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri11 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri12(pub u8);
impl Dchpri12 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri12Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri12Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri12Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri12 {
    #[inline(always)]
    fn default() -> Dchpri12 {
        Dchpri12(0)
    }
}
impl core::fmt::Debug for Dchpri12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri12")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri12 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri13(pub u8);
impl Dchpri13 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri13Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri13Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri13Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri13 {
    #[inline(always)]
    fn default() -> Dchpri13 {
        Dchpri13(0)
    }
}
impl core::fmt::Debug for Dchpri13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri13")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri13 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri14(pub u8);
impl Dchpri14 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri14Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri14Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri14Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri14 {
    #[inline(always)]
    fn default() -> Dchpri14 {
        Dchpri14(0)
    }
}
impl core::fmt::Debug for Dchpri14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri14")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri14 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri15(pub u8);
impl Dchpri15 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri15Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri15Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri15Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri15 {
    #[inline(always)]
    fn default() -> Dchpri15 {
        Dchpri15(0)
    }
}
impl core::fmt::Debug for Dchpri15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri15")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri15 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri2(pub u8);
impl Dchpri2 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri2Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri2Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri2Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri2 {
    #[inline(always)]
    fn default() -> Dchpri2 {
        Dchpri2(0)
    }
}
impl core::fmt::Debug for Dchpri2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri2")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri2 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri3(pub u8);
impl Dchpri3 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri3Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri3Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri3Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri3 {
    #[inline(always)]
    fn default() -> Dchpri3 {
        Dchpri3(0)
    }
}
impl core::fmt::Debug for Dchpri3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri3")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri3 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri4(pub u8);
impl Dchpri4 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri4Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri4Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri4Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri4 {
    #[inline(always)]
    fn default() -> Dchpri4 {
        Dchpri4(0)
    }
}
impl core::fmt::Debug for Dchpri4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri4")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri4 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri5(pub u8);
impl Dchpri5 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri5Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri5Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri5Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri5 {
    #[inline(always)]
    fn default() -> Dchpri5 {
        Dchpri5(0)
    }
}
impl core::fmt::Debug for Dchpri5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri5")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri5 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri6(pub u8);
impl Dchpri6 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri6Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri6Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri6Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri6 {
    #[inline(always)]
    fn default() -> Dchpri6 {
        Dchpri6(0)
    }
}
impl core::fmt::Debug for Dchpri6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri6")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri6 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri7(pub u8);
impl Dchpri7 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri7Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri7Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri7Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri7 {
    #[inline(always)]
    fn default() -> Dchpri7 {
        Dchpri7(0)
    }
}
impl core::fmt::Debug for Dchpri7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri7")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri7 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri8(pub u8);
impl Dchpri8 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri8Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri8Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri8Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri8 {
    #[inline(always)]
    fn default() -> Dchpri8 {
        Dchpri8(0)
    }
}
impl core::fmt::Debug for Dchpri8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri8")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri8 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel Priority"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dchpri9(pub u8);
impl Dchpri9 {
    #[doc = "Channel n Arbitration Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn chpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel n Arbitration Priority"]
    #[inline(always)]
    pub const fn set_chpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> super::vals::Dchpri9Dpa {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dchpri9Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability. This field resets to 0."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: super::vals::Dchpri9Dpa) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Channel Preemption. This field resets to 0."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for Dchpri9 {
    #[inline(always)]
    fn default() -> Dchpri9 {
        Dchpri9(0)
    }
}
impl core::fmt::Debug for Dchpri9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dchpri9")
            .field("chpri", &self.chpri())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dchpri9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dchpri9 {{ chpri: {=u8:?}, dpa: {:?}, ecp: {=bool:?} }}",
            self.chpri(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Enable Asynchronous Request in Stop"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ears(pub u32);
impl Ears {
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 0."]
    #[inline(always)]
    pub const fn set_edreq_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 1."]
    #[inline(always)]
    pub const fn set_edreq_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 2."]
    #[inline(always)]
    pub const fn set_edreq_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 3."]
    #[inline(always)]
    pub const fn set_edreq_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 4."]
    #[inline(always)]
    pub const fn set_edreq_4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 5."]
    #[inline(always)]
    pub const fn set_edreq_5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 6."]
    #[inline(always)]
    pub const fn set_edreq_6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 7."]
    #[inline(always)]
    pub const fn set_edreq_7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 8."]
    #[inline(always)]
    pub const fn set_edreq_8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 9."]
    #[inline(always)]
    pub const fn set_edreq_9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 10."]
    #[inline(always)]
    pub const fn set_edreq_10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 11."]
    #[inline(always)]
    pub const fn set_edreq_11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 12."]
    #[inline(always)]
    pub const fn set_edreq_12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 13."]
    #[inline(always)]
    pub const fn set_edreq_13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 14."]
    #[inline(always)]
    pub const fn set_edreq_14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[must_use]
    #[inline(always)]
    pub const fn edreq_15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable asynchronous DMA request in stop mode for channel 15."]
    #[inline(always)]
    pub const fn set_edreq_15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ears {
    #[inline(always)]
    fn default() -> Ears {
        Ears(0)
    }
}
impl core::fmt::Debug for Ears {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ears")
            .field("edreq_0", &self.edreq_0())
            .field("edreq_1", &self.edreq_1())
            .field("edreq_2", &self.edreq_2())
            .field("edreq_3", &self.edreq_3())
            .field("edreq_4", &self.edreq_4())
            .field("edreq_5", &self.edreq_5())
            .field("edreq_6", &self.edreq_6())
            .field("edreq_7", &self.edreq_7())
            .field("edreq_8", &self.edreq_8())
            .field("edreq_9", &self.edreq_9())
            .field("edreq_10", &self.edreq_10())
            .field("edreq_11", &self.edreq_11())
            .field("edreq_12", &self.edreq_12())
            .field("edreq_13", &self.edreq_13())
            .field("edreq_14", &self.edreq_14())
            .field("edreq_15", &self.edreq_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ears {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ears {{ edreq_0: {=bool:?}, edreq_1: {=bool:?}, edreq_2: {=bool:?}, edreq_3: {=bool:?}, edreq_4: {=bool:?}, edreq_5: {=bool:?}, edreq_6: {=bool:?}, edreq_7: {=bool:?}, edreq_8: {=bool:?}, edreq_9: {=bool:?}, edreq_10: {=bool:?}, edreq_11: {=bool:?}, edreq_12: {=bool:?}, edreq_13: {=bool:?}, edreq_14: {=bool:?}, edreq_15: {=bool:?} }}",
            self.edreq_0(),
            self.edreq_1(),
            self.edreq_2(),
            self.edreq_3(),
            self.edreq_4(),
            self.edreq_5(),
            self.edreq_6(),
            self.edreq_7(),
            self.edreq_8(),
            self.edreq_9(),
            self.edreq_10(),
            self.edreq_11(),
            self.edreq_12(),
            self.edreq_13(),
            self.edreq_14(),
            self.edreq_15()
        )
    }
}
#[doc = "Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eei(pub u32);
impl Eei {
    #[doc = "Enable Error Interrupt 0"]
    #[must_use]
    #[inline(always)]
    pub const fn eei0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 0"]
    #[inline(always)]
    pub const fn set_eei0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Error Interrupt 1"]
    #[must_use]
    #[inline(always)]
    pub const fn eei1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 1"]
    #[inline(always)]
    pub const fn set_eei1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Error Interrupt 2"]
    #[must_use]
    #[inline(always)]
    pub const fn eei2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 2"]
    #[inline(always)]
    pub const fn set_eei2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Error Interrupt 3"]
    #[must_use]
    #[inline(always)]
    pub const fn eei3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 3"]
    #[inline(always)]
    pub const fn set_eei3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable Error Interrupt 4"]
    #[must_use]
    #[inline(always)]
    pub const fn eei4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 4"]
    #[inline(always)]
    pub const fn set_eei4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable Error Interrupt 5"]
    #[must_use]
    #[inline(always)]
    pub const fn eei5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 5"]
    #[inline(always)]
    pub const fn set_eei5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Error Interrupt 6"]
    #[must_use]
    #[inline(always)]
    pub const fn eei6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 6"]
    #[inline(always)]
    pub const fn set_eei6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable Error Interrupt 7"]
    #[must_use]
    #[inline(always)]
    pub const fn eei7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 7"]
    #[inline(always)]
    pub const fn set_eei7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable Error Interrupt 8"]
    #[must_use]
    #[inline(always)]
    pub const fn eei8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 8"]
    #[inline(always)]
    pub const fn set_eei8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable Error Interrupt 9"]
    #[must_use]
    #[inline(always)]
    pub const fn eei9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 9"]
    #[inline(always)]
    pub const fn set_eei9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Error Interrupt 10"]
    #[must_use]
    #[inline(always)]
    pub const fn eei10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 10"]
    #[inline(always)]
    pub const fn set_eei10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable Error Interrupt 11"]
    #[must_use]
    #[inline(always)]
    pub const fn eei11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 11"]
    #[inline(always)]
    pub const fn set_eei11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable Error Interrupt 12"]
    #[must_use]
    #[inline(always)]
    pub const fn eei12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 12"]
    #[inline(always)]
    pub const fn set_eei12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable Error Interrupt 13"]
    #[must_use]
    #[inline(always)]
    pub const fn eei13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 13"]
    #[inline(always)]
    pub const fn set_eei13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable Error Interrupt 14"]
    #[must_use]
    #[inline(always)]
    pub const fn eei14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 14"]
    #[inline(always)]
    pub const fn set_eei14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable Error Interrupt 15"]
    #[must_use]
    #[inline(always)]
    pub const fn eei15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt 15"]
    #[inline(always)]
    pub const fn set_eei15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Eei {
    #[inline(always)]
    fn default() -> Eei {
        Eei(0)
    }
}
impl core::fmt::Debug for Eei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eei")
            .field("eei0", &self.eei0())
            .field("eei1", &self.eei1())
            .field("eei2", &self.eei2())
            .field("eei3", &self.eei3())
            .field("eei4", &self.eei4())
            .field("eei5", &self.eei5())
            .field("eei6", &self.eei6())
            .field("eei7", &self.eei7())
            .field("eei8", &self.eei8())
            .field("eei9", &self.eei9())
            .field("eei10", &self.eei10())
            .field("eei11", &self.eei11())
            .field("eei12", &self.eei12())
            .field("eei13", &self.eei13())
            .field("eei14", &self.eei14())
            .field("eei15", &self.eei15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Eei {{ eei0: {=bool:?}, eei1: {=bool:?}, eei2: {=bool:?}, eei3: {=bool:?}, eei4: {=bool:?}, eei5: {=bool:?}, eei6: {=bool:?}, eei7: {=bool:?}, eei8: {=bool:?}, eei9: {=bool:?}, eei10: {=bool:?}, eei11: {=bool:?}, eei12: {=bool:?}, eei13: {=bool:?}, eei14: {=bool:?}, eei15: {=bool:?} }}",
            self.eei0(),
            self.eei1(),
            self.eei2(),
            self.eei3(),
            self.eei4(),
            self.eei5(),
            self.eei6(),
            self.eei7(),
            self.eei8(),
            self.eei9(),
            self.eei10(),
            self.eei11(),
            self.eei12(),
            self.eei13(),
            self.eei14(),
            self.eei15()
        )
    }
}
#[doc = "Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Erq(pub u32);
impl Erq {
    #[doc = "Enable DMA Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn erq0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 0"]
    #[inline(always)]
    pub const fn set_erq0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable DMA Request 1"]
    #[must_use]
    #[inline(always)]
    pub const fn erq1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 1"]
    #[inline(always)]
    pub const fn set_erq1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable DMA Request 2"]
    #[must_use]
    #[inline(always)]
    pub const fn erq2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 2"]
    #[inline(always)]
    pub const fn set_erq2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable DMA Request 3"]
    #[must_use]
    #[inline(always)]
    pub const fn erq3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 3"]
    #[inline(always)]
    pub const fn set_erq3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable DMA Request 4"]
    #[must_use]
    #[inline(always)]
    pub const fn erq4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 4"]
    #[inline(always)]
    pub const fn set_erq4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable DMA Request 5"]
    #[must_use]
    #[inline(always)]
    pub const fn erq5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 5"]
    #[inline(always)]
    pub const fn set_erq5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable DMA Request 6"]
    #[must_use]
    #[inline(always)]
    pub const fn erq6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 6"]
    #[inline(always)]
    pub const fn set_erq6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable DMA Request 7"]
    #[must_use]
    #[inline(always)]
    pub const fn erq7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 7"]
    #[inline(always)]
    pub const fn set_erq7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable DMA Request 8"]
    #[must_use]
    #[inline(always)]
    pub const fn erq8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 8"]
    #[inline(always)]
    pub const fn set_erq8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable DMA Request 9"]
    #[must_use]
    #[inline(always)]
    pub const fn erq9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 9"]
    #[inline(always)]
    pub const fn set_erq9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable DMA Request 10"]
    #[must_use]
    #[inline(always)]
    pub const fn erq10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 10"]
    #[inline(always)]
    pub const fn set_erq10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable DMA Request 11"]
    #[must_use]
    #[inline(always)]
    pub const fn erq11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 11"]
    #[inline(always)]
    pub const fn set_erq11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable DMA Request 12"]
    #[must_use]
    #[inline(always)]
    pub const fn erq12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 12"]
    #[inline(always)]
    pub const fn set_erq12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enable DMA Request 13"]
    #[must_use]
    #[inline(always)]
    pub const fn erq13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 13"]
    #[inline(always)]
    pub const fn set_erq13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable DMA Request 14"]
    #[must_use]
    #[inline(always)]
    pub const fn erq14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 14"]
    #[inline(always)]
    pub const fn set_erq14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable DMA Request 15"]
    #[must_use]
    #[inline(always)]
    pub const fn erq15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request 15"]
    #[inline(always)]
    pub const fn set_erq15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Erq {
    #[inline(always)]
    fn default() -> Erq {
        Erq(0)
    }
}
impl core::fmt::Debug for Erq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Erq")
            .field("erq0", &self.erq0())
            .field("erq1", &self.erq1())
            .field("erq2", &self.erq2())
            .field("erq3", &self.erq3())
            .field("erq4", &self.erq4())
            .field("erq5", &self.erq5())
            .field("erq6", &self.erq6())
            .field("erq7", &self.erq7())
            .field("erq8", &self.erq8())
            .field("erq9", &self.erq9())
            .field("erq10", &self.erq10())
            .field("erq11", &self.erq11())
            .field("erq12", &self.erq12())
            .field("erq13", &self.erq13())
            .field("erq14", &self.erq14())
            .field("erq15", &self.erq15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Erq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Erq {{ erq0: {=bool:?}, erq1: {=bool:?}, erq2: {=bool:?}, erq3: {=bool:?}, erq4: {=bool:?}, erq5: {=bool:?}, erq6: {=bool:?}, erq7: {=bool:?}, erq8: {=bool:?}, erq9: {=bool:?}, erq10: {=bool:?}, erq11: {=bool:?}, erq12: {=bool:?}, erq13: {=bool:?}, erq14: {=bool:?}, erq15: {=bool:?} }}",
            self.erq0(),
            self.erq1(),
            self.erq2(),
            self.erq3(),
            self.erq4(),
            self.erq5(),
            self.erq6(),
            self.erq7(),
            self.erq8(),
            self.erq9(),
            self.erq10(),
            self.erq11(),
            self.erq12(),
            self.erq13(),
            self.erq14(),
            self.erq15()
        )
    }
}
#[doc = "Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Err(pub u32);
impl Err {
    #[doc = "Error In Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn err0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 0"]
    #[inline(always)]
    pub const fn set_err0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Error In Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn err1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 1"]
    #[inline(always)]
    pub const fn set_err1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error In Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn err2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 2"]
    #[inline(always)]
    pub const fn set_err2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Error In Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn err3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 3"]
    #[inline(always)]
    pub const fn set_err3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Error In Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn err4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 4"]
    #[inline(always)]
    pub const fn set_err4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Error In Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn err5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 5"]
    #[inline(always)]
    pub const fn set_err5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Error In Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn err6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 6"]
    #[inline(always)]
    pub const fn set_err6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Error In Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn err7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 7"]
    #[inline(always)]
    pub const fn set_err7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error In Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn err8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 8"]
    #[inline(always)]
    pub const fn set_err8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Error In Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn err9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 9"]
    #[inline(always)]
    pub const fn set_err9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Error In Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn err10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 10"]
    #[inline(always)]
    pub const fn set_err10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error In Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn err11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 11"]
    #[inline(always)]
    pub const fn set_err11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Error In Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn err12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 12"]
    #[inline(always)]
    pub const fn set_err12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Error In Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn err13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 13"]
    #[inline(always)]
    pub const fn set_err13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error In Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn err14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 14"]
    #[inline(always)]
    pub const fn set_err14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error In Channel 15"]
    #[must_use]
    #[inline(always)]
    pub const fn err15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel 15"]
    #[inline(always)]
    pub const fn set_err15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Err {
    #[inline(always)]
    fn default() -> Err {
        Err(0)
    }
}
impl core::fmt::Debug for Err {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Err")
            .field("err0", &self.err0())
            .field("err1", &self.err1())
            .field("err2", &self.err2())
            .field("err3", &self.err3())
            .field("err4", &self.err4())
            .field("err5", &self.err5())
            .field("err6", &self.err6())
            .field("err7", &self.err7())
            .field("err8", &self.err8())
            .field("err9", &self.err9())
            .field("err10", &self.err10())
            .field("err11", &self.err11())
            .field("err12", &self.err12())
            .field("err13", &self.err13())
            .field("err14", &self.err14())
            .field("err15", &self.err15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Err {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Err {{ err0: {=bool:?}, err1: {=bool:?}, err2: {=bool:?}, err3: {=bool:?}, err4: {=bool:?}, err5: {=bool:?}, err6: {=bool:?}, err7: {=bool:?}, err8: {=bool:?}, err9: {=bool:?}, err10: {=bool:?}, err11: {=bool:?}, err12: {=bool:?}, err13: {=bool:?}, err14: {=bool:?}, err15: {=bool:?} }}",
            self.err0(),
            self.err1(),
            self.err2(),
            self.err3(),
            self.err4(),
            self.err5(),
            self.err6(),
            self.err7(),
            self.err8(),
            self.err9(),
            self.err10(),
            self.err11(),
            self.err12(),
            self.err13(),
            self.err14(),
            self.err15()
        )
    }
}
#[doc = "Error Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Es(pub u32);
impl Es {
    #[doc = "Destination Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error"]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error"]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error"]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error"]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Channel Priority Error"]
    #[must_use]
    #[inline(always)]
    pub const fn cpe(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Priority Error"]
    #[inline(always)]
    pub const fn set_cpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transfer Canceled"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> super::vals::EsEcx {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: super::vals::EsEcx) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of all ERR status fields"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Es {
    #[inline(always)]
    fn default() -> Es {
        Es(0)
    }
}
impl core::fmt::Debug for Es {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Es")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("errchn", &self.errchn())
            .field("cpe", &self.cpe())
            .field("ecx", &self.ecx())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Es {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Es {{ dbe: {=bool:?}, sbe: {=bool:?}, sge: {=bool:?}, nce: {=bool:?}, doe: {=bool:?}, dae: {=bool:?}, soe: {=bool:?}, sae: {=bool:?}, errchn: {=u8:?}, cpe: {=bool:?}, ecx: {:?}, vld: {=bool:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.errchn(),
            self.cpe(),
            self.ecx(),
            self.vld()
        )
    }
}
#[doc = "Hardware Request Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hrs(pub u32);
impl Hrs {
    #[doc = "Hardware Request Status Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 0"]
    #[inline(always)]
    pub const fn set_hrs0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Hardware Request Status Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 1"]
    #[inline(always)]
    pub const fn set_hrs1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Hardware Request Status Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 2"]
    #[inline(always)]
    pub const fn set_hrs2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Hardware Request Status Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 3"]
    #[inline(always)]
    pub const fn set_hrs3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Hardware Request Status Channel 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 4"]
    #[inline(always)]
    pub const fn set_hrs4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Hardware Request Status Channel 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 5"]
    #[inline(always)]
    pub const fn set_hrs5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Hardware Request Status Channel 6"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 6"]
    #[inline(always)]
    pub const fn set_hrs6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Hardware Request Status Channel 7"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 7"]
    #[inline(always)]
    pub const fn set_hrs7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Hardware Request Status Channel 8"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 8"]
    #[inline(always)]
    pub const fn set_hrs8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Hardware Request Status Channel 9"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 9"]
    #[inline(always)]
    pub const fn set_hrs9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Hardware Request Status Channel 10"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 10"]
    #[inline(always)]
    pub const fn set_hrs10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Hardware Request Status Channel 11"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 11"]
    #[inline(always)]
    pub const fn set_hrs11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Hardware Request Status Channel 12"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 12"]
    #[inline(always)]
    pub const fn set_hrs12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Hardware Request Status Channel 13"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 13"]
    #[inline(always)]
    pub const fn set_hrs13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Hardware Request Status Channel 14"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 14"]
    #[inline(always)]
    pub const fn set_hrs14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Hardware Request Status Channel 15"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Request Status Channel 15"]
    #[inline(always)]
    pub const fn set_hrs15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Hrs {
    #[inline(always)]
    fn default() -> Hrs {
        Hrs(0)
    }
}
impl core::fmt::Debug for Hrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hrs")
            .field("hrs0", &self.hrs0())
            .field("hrs1", &self.hrs1())
            .field("hrs2", &self.hrs2())
            .field("hrs3", &self.hrs3())
            .field("hrs4", &self.hrs4())
            .field("hrs5", &self.hrs5())
            .field("hrs6", &self.hrs6())
            .field("hrs7", &self.hrs7())
            .field("hrs8", &self.hrs8())
            .field("hrs9", &self.hrs9())
            .field("hrs10", &self.hrs10())
            .field("hrs11", &self.hrs11())
            .field("hrs12", &self.hrs12())
            .field("hrs13", &self.hrs13())
            .field("hrs14", &self.hrs14())
            .field("hrs15", &self.hrs15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Hrs {{ hrs0: {=bool:?}, hrs1: {=bool:?}, hrs2: {=bool:?}, hrs3: {=bool:?}, hrs4: {=bool:?}, hrs5: {=bool:?}, hrs6: {=bool:?}, hrs7: {=bool:?}, hrs8: {=bool:?}, hrs9: {=bool:?}, hrs10: {=bool:?}, hrs11: {=bool:?}, hrs12: {=bool:?}, hrs13: {=bool:?}, hrs14: {=bool:?}, hrs15: {=bool:?} }}",
            self.hrs0(),
            self.hrs1(),
            self.hrs2(),
            self.hrs3(),
            self.hrs4(),
            self.hrs5(),
            self.hrs6(),
            self.hrs7(),
            self.hrs8(),
            self.hrs9(),
            self.hrs10(),
            self.hrs11(),
            self.hrs12(),
            self.hrs13(),
            self.hrs14(),
            self.hrs15()
        )
    }
}
#[doc = "Interrupt Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Interrupt Request 0"]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 0"]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Request 1"]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 1"]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt Request 2"]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 2"]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt Request 3"]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 3"]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt Request 4"]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 4"]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt Request 5"]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 5"]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt Request 6"]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> super::vals::Int6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Int6::from_bits(val as u8)
    }
    #[doc = "Interrupt Request 6"]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: super::vals::Int6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt Request 7"]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 7"]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt Request 8"]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 8"]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt Request 9"]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 9"]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt Request 10"]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 10"]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt Request 11"]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 11"]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt Request 12"]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 12"]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt Request 13"]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 13"]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt Request 14"]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 14"]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt Request 15"]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request 15"]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Int {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15()
        )
    }
}
#[doc = "Set Enable Error Interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seei(pub u8);
impl Seei {
    #[doc = "Set Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn seei(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Set Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_seei(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn saee(&self) -> super::vals::Saee {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saee::from_bits(val as u8)
    }
    #[doc = "Set All Enable Error Interrupts"]
    #[inline(always)]
    pub const fn set_saee(&mut self, val: super::vals::Saee) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SeeiNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SeeiNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SeeiNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Seei {
    #[inline(always)]
    fn default() -> Seei {
        Seei(0)
    }
}
impl core::fmt::Debug for Seei {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Seei")
            .field("seei", &self.seei())
            .field("saee", &self.saee())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Seei {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Seei {{ seei: {=u8:?}, saee: {:?}, nop: {:?} }}",
            self.seei(),
            self.saee(),
            self.nop()
        )
    }
}
#[doc = "Set Enable Request"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serq(pub u8);
impl Serq {
    #[doc = "Set Enable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn serq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Set Enable Request"]
    #[inline(always)]
    pub const fn set_serq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Set All Enable Requests"]
    #[must_use]
    #[inline(always)]
    pub const fn saer(&self) -> super::vals::Saer {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Saer::from_bits(val as u8)
    }
    #[doc = "Set All Enable Requests"]
    #[inline(always)]
    pub const fn set_saer(&mut self, val: super::vals::Saer) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SerqNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SerqNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SerqNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Serq {
    #[inline(always)]
    fn default() -> Serq {
        Serq(0)
    }
}
impl core::fmt::Debug for Serq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Serq")
            .field("serq", &self.serq())
            .field("saer", &self.saer())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Serq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Serq {{ serq: {=u8:?}, saer: {:?}, nop: {:?} }}",
            self.serq(),
            self.saer(),
            self.nop()
        )
    }
}
#[doc = "Set START Bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssrt(pub u8);
impl Ssrt {
    #[doc = "Set START field"]
    #[must_use]
    #[inline(always)]
    pub const fn ssrt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Set START field"]
    #[inline(always)]
    pub const fn set_ssrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[must_use]
    #[inline(always)]
    pub const fn sast(&self) -> super::vals::Sast {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sast::from_bits(val as u8)
    }
    #[doc = "Set All START fields (activates all channels)"]
    #[inline(always)]
    pub const fn set_sast(&mut self, val: super::vals::Sast) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u8) & 0x01) << 6usize);
    }
    #[doc = "No Op Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn nop(&self) -> super::vals::SsrtNop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::SsrtNop::from_bits(val as u8)
    }
    #[doc = "No Op Enable"]
    #[inline(always)]
    pub const fn set_nop(&mut self, val: super::vals::SsrtNop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u8) & 0x01) << 7usize);
    }
}
impl Default for Ssrt {
    #[inline(always)]
    fn default() -> Ssrt {
        Ssrt(0)
    }
}
impl core::fmt::Debug for Ssrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssrt")
            .field("ssrt", &self.ssrt())
            .field("sast", &self.sast())
            .field("nop", &self.nop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ssrt {{ ssrt: {=u8:?}, sast: {:?}, nop: {:?} }}",
            self.ssrt(),
            self.sast(),
            self.nop()
        )
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Attr(pub u16);
impl Tcd0Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd0AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd0AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd0AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd0Attr {
    #[inline(always)]
    fn default() -> Tcd0Attr {
        Tcd0Attr(0)
    }
}
impl core::fmt::Debug for Tcd0Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0BiterElinkno(pub u16);
impl Tcd0BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd0BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd0BiterElinkno {
        Tcd0BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd0BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0BiterElinkyes(pub u16);
impl Tcd0BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd0BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd0BiterElinkyes {
        Tcd0BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd0BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0CiterElinkno(pub u16);
impl Tcd0CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd0CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd0CiterElinkno {
        Tcd0CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd0CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0CiterElinkyes(pub u16);
impl Tcd0CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd0CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd0CiterElinkyes {
        Tcd0CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd0CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Csr(pub u16);
impl Tcd0Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd0CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd0CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd0CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd0CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd0CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd0CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd0Csr {
    #[inline(always)]
    fn default() -> Tcd0Csr {
        Tcd0Csr(0)
    }
}
impl core::fmt::Debug for Tcd0Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Daddr(pub u32);
impl Tcd0Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd0Daddr {
    #[inline(always)]
    fn default() -> Tcd0Daddr {
        Tcd0Daddr(0)
    }
}
impl core::fmt::Debug for Tcd0Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Dlastsga(pub u32);
impl Tcd0Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd0Dlastsga {
    #[inline(always)]
    fn default() -> Tcd0Dlastsga {
        Tcd0Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd0Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Doff(pub u16);
impl Tcd0Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd0Doff {
    #[inline(always)]
    fn default() -> Tcd0Doff {
        Tcd0Doff(0)
    }
}
impl core::fmt::Debug for Tcd0Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0NbytesMlno(pub u32);
impl Tcd0NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd0NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd0NbytesMlno {
        Tcd0NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd0NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0NbytesMloffno(pub u32);
impl Tcd0NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd0NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd0NbytesMloffno {
        Tcd0NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd0NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0NbytesMloffyes(pub u32);
impl Tcd0NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd0NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd0NbytesMloffyes {
        Tcd0NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd0NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd0NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Saddr(pub u32);
impl Tcd0Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd0Saddr {
    #[inline(always)]
    fn default() -> Tcd0Saddr {
        Tcd0Saddr(0)
    }
}
impl core::fmt::Debug for Tcd0Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Slast(pub u32);
impl Tcd0Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd0Slast {
    #[inline(always)]
    fn default() -> Tcd0Slast {
        Tcd0Slast(0)
    }
}
impl core::fmt::Debug for Tcd0Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd0Soff(pub u16);
impl Tcd0Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd0Soff {
    #[inline(always)]
    fn default() -> Tcd0Soff {
        Tcd0Soff(0)
    }
}
impl core::fmt::Debug for Tcd0Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd0Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd0Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd0Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Attr(pub u16);
impl Tcd10Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd10AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd10AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd10AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd10Attr {
    #[inline(always)]
    fn default() -> Tcd10Attr {
        Tcd10Attr(0)
    }
}
impl core::fmt::Debug for Tcd10Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10BiterElinkno(pub u16);
impl Tcd10BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd10BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd10BiterElinkno {
        Tcd10BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd10BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10BiterElinkyes(pub u16);
impl Tcd10BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd10BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd10BiterElinkyes {
        Tcd10BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd10BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10CiterElinkno(pub u16);
impl Tcd10CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd10CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd10CiterElinkno {
        Tcd10CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd10CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10CiterElinkyes(pub u16);
impl Tcd10CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd10CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd10CiterElinkyes {
        Tcd10CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd10CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Csr(pub u16);
impl Tcd10Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd10CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd10CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd10CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd10CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd10CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd10CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd10Csr {
    #[inline(always)]
    fn default() -> Tcd10Csr {
        Tcd10Csr(0)
    }
}
impl core::fmt::Debug for Tcd10Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Daddr(pub u32);
impl Tcd10Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd10Daddr {
    #[inline(always)]
    fn default() -> Tcd10Daddr {
        Tcd10Daddr(0)
    }
}
impl core::fmt::Debug for Tcd10Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Dlastsga(pub u32);
impl Tcd10Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd10Dlastsga {
    #[inline(always)]
    fn default() -> Tcd10Dlastsga {
        Tcd10Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd10Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Doff(pub u16);
impl Tcd10Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd10Doff {
    #[inline(always)]
    fn default() -> Tcd10Doff {
        Tcd10Doff(0)
    }
}
impl core::fmt::Debug for Tcd10Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10NbytesMlno(pub u32);
impl Tcd10NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd10NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd10NbytesMlno {
        Tcd10NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd10NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10NbytesMloffno(pub u32);
impl Tcd10NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd10NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd10NbytesMloffno {
        Tcd10NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd10NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10NbytesMloffyes(pub u32);
impl Tcd10NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd10NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd10NbytesMloffyes {
        Tcd10NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd10NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd10NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Saddr(pub u32);
impl Tcd10Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd10Saddr {
    #[inline(always)]
    fn default() -> Tcd10Saddr {
        Tcd10Saddr(0)
    }
}
impl core::fmt::Debug for Tcd10Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Slast(pub u32);
impl Tcd10Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd10Slast {
    #[inline(always)]
    fn default() -> Tcd10Slast {
        Tcd10Slast(0)
    }
}
impl core::fmt::Debug for Tcd10Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd10Soff(pub u16);
impl Tcd10Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd10Soff {
    #[inline(always)]
    fn default() -> Tcd10Soff {
        Tcd10Soff(0)
    }
}
impl core::fmt::Debug for Tcd10Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd10Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd10Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd10Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Attr(pub u16);
impl Tcd11Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd11AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd11AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd11AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd11Attr {
    #[inline(always)]
    fn default() -> Tcd11Attr {
        Tcd11Attr(0)
    }
}
impl core::fmt::Debug for Tcd11Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11BiterElinkno(pub u16);
impl Tcd11BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd11BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd11BiterElinkno {
        Tcd11BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd11BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11BiterElinkyes(pub u16);
impl Tcd11BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd11BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd11BiterElinkyes {
        Tcd11BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd11BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11CiterElinkno(pub u16);
impl Tcd11CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd11CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd11CiterElinkno {
        Tcd11CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd11CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11CiterElinkyes(pub u16);
impl Tcd11CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd11CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd11CiterElinkyes {
        Tcd11CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd11CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Csr(pub u16);
impl Tcd11Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd11CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd11CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd11CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd11CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd11CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd11CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd11Csr {
    #[inline(always)]
    fn default() -> Tcd11Csr {
        Tcd11Csr(0)
    }
}
impl core::fmt::Debug for Tcd11Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Daddr(pub u32);
impl Tcd11Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd11Daddr {
    #[inline(always)]
    fn default() -> Tcd11Daddr {
        Tcd11Daddr(0)
    }
}
impl core::fmt::Debug for Tcd11Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Dlastsga(pub u32);
impl Tcd11Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd11Dlastsga {
    #[inline(always)]
    fn default() -> Tcd11Dlastsga {
        Tcd11Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd11Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Doff(pub u16);
impl Tcd11Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd11Doff {
    #[inline(always)]
    fn default() -> Tcd11Doff {
        Tcd11Doff(0)
    }
}
impl core::fmt::Debug for Tcd11Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11NbytesMlno(pub u32);
impl Tcd11NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd11NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd11NbytesMlno {
        Tcd11NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd11NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11NbytesMloffno(pub u32);
impl Tcd11NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd11NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd11NbytesMloffno {
        Tcd11NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd11NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11NbytesMloffyes(pub u32);
impl Tcd11NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd11NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd11NbytesMloffyes {
        Tcd11NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd11NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd11NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Saddr(pub u32);
impl Tcd11Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd11Saddr {
    #[inline(always)]
    fn default() -> Tcd11Saddr {
        Tcd11Saddr(0)
    }
}
impl core::fmt::Debug for Tcd11Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Slast(pub u32);
impl Tcd11Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd11Slast {
    #[inline(always)]
    fn default() -> Tcd11Slast {
        Tcd11Slast(0)
    }
}
impl core::fmt::Debug for Tcd11Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd11Soff(pub u16);
impl Tcd11Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd11Soff {
    #[inline(always)]
    fn default() -> Tcd11Soff {
        Tcd11Soff(0)
    }
}
impl core::fmt::Debug for Tcd11Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd11Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd11Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd11Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Attr(pub u16);
impl Tcd12Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd12AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd12AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd12AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd12Attr {
    #[inline(always)]
    fn default() -> Tcd12Attr {
        Tcd12Attr(0)
    }
}
impl core::fmt::Debug for Tcd12Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12BiterElinkno(pub u16);
impl Tcd12BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd12BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd12BiterElinkno {
        Tcd12BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd12BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12BiterElinkyes(pub u16);
impl Tcd12BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd12BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd12BiterElinkyes {
        Tcd12BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd12BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12CiterElinkno(pub u16);
impl Tcd12CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd12CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd12CiterElinkno {
        Tcd12CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd12CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12CiterElinkyes(pub u16);
impl Tcd12CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd12CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd12CiterElinkyes {
        Tcd12CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd12CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Csr(pub u16);
impl Tcd12Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd12CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd12CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd12CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd12CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd12CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd12CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd12Csr {
    #[inline(always)]
    fn default() -> Tcd12Csr {
        Tcd12Csr(0)
    }
}
impl core::fmt::Debug for Tcd12Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Daddr(pub u32);
impl Tcd12Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd12Daddr {
    #[inline(always)]
    fn default() -> Tcd12Daddr {
        Tcd12Daddr(0)
    }
}
impl core::fmt::Debug for Tcd12Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Dlastsga(pub u32);
impl Tcd12Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd12Dlastsga {
    #[inline(always)]
    fn default() -> Tcd12Dlastsga {
        Tcd12Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd12Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Doff(pub u16);
impl Tcd12Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd12Doff {
    #[inline(always)]
    fn default() -> Tcd12Doff {
        Tcd12Doff(0)
    }
}
impl core::fmt::Debug for Tcd12Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12NbytesMlno(pub u32);
impl Tcd12NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd12NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd12NbytesMlno {
        Tcd12NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd12NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12NbytesMloffno(pub u32);
impl Tcd12NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd12NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd12NbytesMloffno {
        Tcd12NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd12NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12NbytesMloffyes(pub u32);
impl Tcd12NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd12NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd12NbytesMloffyes {
        Tcd12NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd12NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd12NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Saddr(pub u32);
impl Tcd12Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd12Saddr {
    #[inline(always)]
    fn default() -> Tcd12Saddr {
        Tcd12Saddr(0)
    }
}
impl core::fmt::Debug for Tcd12Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Slast(pub u32);
impl Tcd12Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd12Slast {
    #[inline(always)]
    fn default() -> Tcd12Slast {
        Tcd12Slast(0)
    }
}
impl core::fmt::Debug for Tcd12Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd12Soff(pub u16);
impl Tcd12Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd12Soff {
    #[inline(always)]
    fn default() -> Tcd12Soff {
        Tcd12Soff(0)
    }
}
impl core::fmt::Debug for Tcd12Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd12Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd12Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd12Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Attr(pub u16);
impl Tcd13Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd13AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd13AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd13AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd13Attr {
    #[inline(always)]
    fn default() -> Tcd13Attr {
        Tcd13Attr(0)
    }
}
impl core::fmt::Debug for Tcd13Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13BiterElinkno(pub u16);
impl Tcd13BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd13BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd13BiterElinkno {
        Tcd13BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd13BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13BiterElinkyes(pub u16);
impl Tcd13BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd13BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd13BiterElinkyes {
        Tcd13BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd13BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13CiterElinkno(pub u16);
impl Tcd13CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd13CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd13CiterElinkno {
        Tcd13CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd13CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13CiterElinkyes(pub u16);
impl Tcd13CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd13CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd13CiterElinkyes {
        Tcd13CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd13CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Csr(pub u16);
impl Tcd13Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd13CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd13CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd13CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd13CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd13CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd13CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd13Csr {
    #[inline(always)]
    fn default() -> Tcd13Csr {
        Tcd13Csr(0)
    }
}
impl core::fmt::Debug for Tcd13Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Daddr(pub u32);
impl Tcd13Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd13Daddr {
    #[inline(always)]
    fn default() -> Tcd13Daddr {
        Tcd13Daddr(0)
    }
}
impl core::fmt::Debug for Tcd13Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Dlastsga(pub u32);
impl Tcd13Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd13Dlastsga {
    #[inline(always)]
    fn default() -> Tcd13Dlastsga {
        Tcd13Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd13Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Doff(pub u16);
impl Tcd13Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd13Doff {
    #[inline(always)]
    fn default() -> Tcd13Doff {
        Tcd13Doff(0)
    }
}
impl core::fmt::Debug for Tcd13Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13NbytesMlno(pub u32);
impl Tcd13NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd13NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd13NbytesMlno {
        Tcd13NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd13NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13NbytesMloffno(pub u32);
impl Tcd13NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd13NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd13NbytesMloffno {
        Tcd13NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd13NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13NbytesMloffyes(pub u32);
impl Tcd13NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd13NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd13NbytesMloffyes {
        Tcd13NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd13NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd13NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Saddr(pub u32);
impl Tcd13Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd13Saddr {
    #[inline(always)]
    fn default() -> Tcd13Saddr {
        Tcd13Saddr(0)
    }
}
impl core::fmt::Debug for Tcd13Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Slast(pub u32);
impl Tcd13Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd13Slast {
    #[inline(always)]
    fn default() -> Tcd13Slast {
        Tcd13Slast(0)
    }
}
impl core::fmt::Debug for Tcd13Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd13Soff(pub u16);
impl Tcd13Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd13Soff {
    #[inline(always)]
    fn default() -> Tcd13Soff {
        Tcd13Soff(0)
    }
}
impl core::fmt::Debug for Tcd13Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd13Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd13Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd13Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Attr(pub u16);
impl Tcd14Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd14AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd14AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd14AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd14Attr {
    #[inline(always)]
    fn default() -> Tcd14Attr {
        Tcd14Attr(0)
    }
}
impl core::fmt::Debug for Tcd14Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14BiterElinkno(pub u16);
impl Tcd14BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd14BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd14BiterElinkno {
        Tcd14BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd14BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14BiterElinkyes(pub u16);
impl Tcd14BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd14BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd14BiterElinkyes {
        Tcd14BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd14BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14CiterElinkno(pub u16);
impl Tcd14CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd14CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd14CiterElinkno {
        Tcd14CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd14CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14CiterElinkyes(pub u16);
impl Tcd14CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd14CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd14CiterElinkyes {
        Tcd14CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd14CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Csr(pub u16);
impl Tcd14Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd14CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd14CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd14CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd14CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd14CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd14CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd14Csr {
    #[inline(always)]
    fn default() -> Tcd14Csr {
        Tcd14Csr(0)
    }
}
impl core::fmt::Debug for Tcd14Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Daddr(pub u32);
impl Tcd14Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd14Daddr {
    #[inline(always)]
    fn default() -> Tcd14Daddr {
        Tcd14Daddr(0)
    }
}
impl core::fmt::Debug for Tcd14Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Dlastsga(pub u32);
impl Tcd14Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd14Dlastsga {
    #[inline(always)]
    fn default() -> Tcd14Dlastsga {
        Tcd14Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd14Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Doff(pub u16);
impl Tcd14Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd14Doff {
    #[inline(always)]
    fn default() -> Tcd14Doff {
        Tcd14Doff(0)
    }
}
impl core::fmt::Debug for Tcd14Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14NbytesMlno(pub u32);
impl Tcd14NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd14NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd14NbytesMlno {
        Tcd14NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd14NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14NbytesMloffno(pub u32);
impl Tcd14NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd14NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd14NbytesMloffno {
        Tcd14NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd14NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14NbytesMloffyes(pub u32);
impl Tcd14NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd14NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd14NbytesMloffyes {
        Tcd14NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd14NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd14NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Saddr(pub u32);
impl Tcd14Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd14Saddr {
    #[inline(always)]
    fn default() -> Tcd14Saddr {
        Tcd14Saddr(0)
    }
}
impl core::fmt::Debug for Tcd14Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Slast(pub u32);
impl Tcd14Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd14Slast {
    #[inline(always)]
    fn default() -> Tcd14Slast {
        Tcd14Slast(0)
    }
}
impl core::fmt::Debug for Tcd14Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd14Soff(pub u16);
impl Tcd14Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd14Soff {
    #[inline(always)]
    fn default() -> Tcd14Soff {
        Tcd14Soff(0)
    }
}
impl core::fmt::Debug for Tcd14Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd14Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd14Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd14Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Attr(pub u16);
impl Tcd15Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd15AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd15AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd15AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd15Attr {
    #[inline(always)]
    fn default() -> Tcd15Attr {
        Tcd15Attr(0)
    }
}
impl core::fmt::Debug for Tcd15Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15BiterElinkno(pub u16);
impl Tcd15BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd15BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd15BiterElinkno {
        Tcd15BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd15BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15BiterElinkyes(pub u16);
impl Tcd15BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd15BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd15BiterElinkyes {
        Tcd15BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd15BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15CiterElinkno(pub u16);
impl Tcd15CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd15CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd15CiterElinkno {
        Tcd15CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd15CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15CiterElinkyes(pub u16);
impl Tcd15CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd15CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd15CiterElinkyes {
        Tcd15CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd15CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Csr(pub u16);
impl Tcd15Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd15CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd15CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd15CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd15CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd15CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd15CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd15Csr {
    #[inline(always)]
    fn default() -> Tcd15Csr {
        Tcd15Csr(0)
    }
}
impl core::fmt::Debug for Tcd15Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Daddr(pub u32);
impl Tcd15Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd15Daddr {
    #[inline(always)]
    fn default() -> Tcd15Daddr {
        Tcd15Daddr(0)
    }
}
impl core::fmt::Debug for Tcd15Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Dlastsga(pub u32);
impl Tcd15Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd15Dlastsga {
    #[inline(always)]
    fn default() -> Tcd15Dlastsga {
        Tcd15Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd15Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Doff(pub u16);
impl Tcd15Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd15Doff {
    #[inline(always)]
    fn default() -> Tcd15Doff {
        Tcd15Doff(0)
    }
}
impl core::fmt::Debug for Tcd15Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15NbytesMlno(pub u32);
impl Tcd15NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd15NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd15NbytesMlno {
        Tcd15NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd15NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15NbytesMloffno(pub u32);
impl Tcd15NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd15NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd15NbytesMloffno {
        Tcd15NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd15NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15NbytesMloffyes(pub u32);
impl Tcd15NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd15NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd15NbytesMloffyes {
        Tcd15NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd15NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd15NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Saddr(pub u32);
impl Tcd15Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd15Saddr {
    #[inline(always)]
    fn default() -> Tcd15Saddr {
        Tcd15Saddr(0)
    }
}
impl core::fmt::Debug for Tcd15Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Slast(pub u32);
impl Tcd15Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd15Slast {
    #[inline(always)]
    fn default() -> Tcd15Slast {
        Tcd15Slast(0)
    }
}
impl core::fmt::Debug for Tcd15Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd15Soff(pub u16);
impl Tcd15Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd15Soff {
    #[inline(always)]
    fn default() -> Tcd15Soff {
        Tcd15Soff(0)
    }
}
impl core::fmt::Debug for Tcd15Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd15Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd15Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd15Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Attr(pub u16);
impl Tcd1Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd1AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd1AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd1AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd1Attr {
    #[inline(always)]
    fn default() -> Tcd1Attr {
        Tcd1Attr(0)
    }
}
impl core::fmt::Debug for Tcd1Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1BiterElinkno(pub u16);
impl Tcd1BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd1BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd1BiterElinkno {
        Tcd1BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd1BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1BiterElinkyes(pub u16);
impl Tcd1BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd1BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd1BiterElinkyes {
        Tcd1BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd1BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1CiterElinkno(pub u16);
impl Tcd1CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd1CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd1CiterElinkno {
        Tcd1CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd1CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1CiterElinkyes(pub u16);
impl Tcd1CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd1CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd1CiterElinkyes {
        Tcd1CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd1CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Csr(pub u16);
impl Tcd1Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd1CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd1CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd1CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd1CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd1CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd1CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd1Csr {
    #[inline(always)]
    fn default() -> Tcd1Csr {
        Tcd1Csr(0)
    }
}
impl core::fmt::Debug for Tcd1Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Daddr(pub u32);
impl Tcd1Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd1Daddr {
    #[inline(always)]
    fn default() -> Tcd1Daddr {
        Tcd1Daddr(0)
    }
}
impl core::fmt::Debug for Tcd1Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Dlastsga(pub u32);
impl Tcd1Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd1Dlastsga {
    #[inline(always)]
    fn default() -> Tcd1Dlastsga {
        Tcd1Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd1Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Doff(pub u16);
impl Tcd1Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd1Doff {
    #[inline(always)]
    fn default() -> Tcd1Doff {
        Tcd1Doff(0)
    }
}
impl core::fmt::Debug for Tcd1Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1NbytesMlno(pub u32);
impl Tcd1NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd1NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd1NbytesMlno {
        Tcd1NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd1NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1NbytesMloffno(pub u32);
impl Tcd1NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd1NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd1NbytesMloffno {
        Tcd1NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd1NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1NbytesMloffyes(pub u32);
impl Tcd1NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd1NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd1NbytesMloffyes {
        Tcd1NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd1NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd1NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Saddr(pub u32);
impl Tcd1Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd1Saddr {
    #[inline(always)]
    fn default() -> Tcd1Saddr {
        Tcd1Saddr(0)
    }
}
impl core::fmt::Debug for Tcd1Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Slast(pub u32);
impl Tcd1Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd1Slast {
    #[inline(always)]
    fn default() -> Tcd1Slast {
        Tcd1Slast(0)
    }
}
impl core::fmt::Debug for Tcd1Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd1Soff(pub u16);
impl Tcd1Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd1Soff {
    #[inline(always)]
    fn default() -> Tcd1Soff {
        Tcd1Soff(0)
    }
}
impl core::fmt::Debug for Tcd1Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd1Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd1Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd1Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Attr(pub u16);
impl Tcd2Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd2AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd2AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd2AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd2Attr {
    #[inline(always)]
    fn default() -> Tcd2Attr {
        Tcd2Attr(0)
    }
}
impl core::fmt::Debug for Tcd2Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2BiterElinkno(pub u16);
impl Tcd2BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd2BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd2BiterElinkno {
        Tcd2BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd2BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2BiterElinkyes(pub u16);
impl Tcd2BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd2BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd2BiterElinkyes {
        Tcd2BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd2BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2CiterElinkno(pub u16);
impl Tcd2CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd2CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd2CiterElinkno {
        Tcd2CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd2CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2CiterElinkyes(pub u16);
impl Tcd2CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd2CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd2CiterElinkyes {
        Tcd2CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd2CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Csr(pub u16);
impl Tcd2Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd2CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd2CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd2CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd2CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd2CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd2CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd2Csr {
    #[inline(always)]
    fn default() -> Tcd2Csr {
        Tcd2Csr(0)
    }
}
impl core::fmt::Debug for Tcd2Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Daddr(pub u32);
impl Tcd2Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd2Daddr {
    #[inline(always)]
    fn default() -> Tcd2Daddr {
        Tcd2Daddr(0)
    }
}
impl core::fmt::Debug for Tcd2Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Dlastsga(pub u32);
impl Tcd2Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd2Dlastsga {
    #[inline(always)]
    fn default() -> Tcd2Dlastsga {
        Tcd2Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd2Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Doff(pub u16);
impl Tcd2Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd2Doff {
    #[inline(always)]
    fn default() -> Tcd2Doff {
        Tcd2Doff(0)
    }
}
impl core::fmt::Debug for Tcd2Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2NbytesMlno(pub u32);
impl Tcd2NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd2NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd2NbytesMlno {
        Tcd2NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd2NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2NbytesMloffno(pub u32);
impl Tcd2NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd2NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd2NbytesMloffno {
        Tcd2NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd2NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2NbytesMloffyes(pub u32);
impl Tcd2NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd2NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd2NbytesMloffyes {
        Tcd2NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd2NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd2NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Saddr(pub u32);
impl Tcd2Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd2Saddr {
    #[inline(always)]
    fn default() -> Tcd2Saddr {
        Tcd2Saddr(0)
    }
}
impl core::fmt::Debug for Tcd2Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Slast(pub u32);
impl Tcd2Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd2Slast {
    #[inline(always)]
    fn default() -> Tcd2Slast {
        Tcd2Slast(0)
    }
}
impl core::fmt::Debug for Tcd2Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd2Soff(pub u16);
impl Tcd2Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd2Soff {
    #[inline(always)]
    fn default() -> Tcd2Soff {
        Tcd2Soff(0)
    }
}
impl core::fmt::Debug for Tcd2Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd2Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd2Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd2Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Attr(pub u16);
impl Tcd3Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd3AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd3AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd3AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd3Attr {
    #[inline(always)]
    fn default() -> Tcd3Attr {
        Tcd3Attr(0)
    }
}
impl core::fmt::Debug for Tcd3Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3BiterElinkno(pub u16);
impl Tcd3BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd3BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd3BiterElinkno {
        Tcd3BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd3BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3BiterElinkyes(pub u16);
impl Tcd3BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd3BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd3BiterElinkyes {
        Tcd3BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd3BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3CiterElinkno(pub u16);
impl Tcd3CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd3CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd3CiterElinkno {
        Tcd3CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd3CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3CiterElinkyes(pub u16);
impl Tcd3CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd3CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd3CiterElinkyes {
        Tcd3CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd3CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Csr(pub u16);
impl Tcd3Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd3CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd3CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd3CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd3CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd3CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd3CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd3Csr {
    #[inline(always)]
    fn default() -> Tcd3Csr {
        Tcd3Csr(0)
    }
}
impl core::fmt::Debug for Tcd3Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Daddr(pub u32);
impl Tcd3Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd3Daddr {
    #[inline(always)]
    fn default() -> Tcd3Daddr {
        Tcd3Daddr(0)
    }
}
impl core::fmt::Debug for Tcd3Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Dlastsga(pub u32);
impl Tcd3Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd3Dlastsga {
    #[inline(always)]
    fn default() -> Tcd3Dlastsga {
        Tcd3Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd3Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Doff(pub u16);
impl Tcd3Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd3Doff {
    #[inline(always)]
    fn default() -> Tcd3Doff {
        Tcd3Doff(0)
    }
}
impl core::fmt::Debug for Tcd3Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3NbytesMlno(pub u32);
impl Tcd3NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd3NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd3NbytesMlno {
        Tcd3NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd3NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3NbytesMloffno(pub u32);
impl Tcd3NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd3NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd3NbytesMloffno {
        Tcd3NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd3NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3NbytesMloffyes(pub u32);
impl Tcd3NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd3NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd3NbytesMloffyes {
        Tcd3NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd3NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd3NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Saddr(pub u32);
impl Tcd3Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd3Saddr {
    #[inline(always)]
    fn default() -> Tcd3Saddr {
        Tcd3Saddr(0)
    }
}
impl core::fmt::Debug for Tcd3Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Slast(pub u32);
impl Tcd3Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd3Slast {
    #[inline(always)]
    fn default() -> Tcd3Slast {
        Tcd3Slast(0)
    }
}
impl core::fmt::Debug for Tcd3Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd3Soff(pub u16);
impl Tcd3Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd3Soff {
    #[inline(always)]
    fn default() -> Tcd3Soff {
        Tcd3Soff(0)
    }
}
impl core::fmt::Debug for Tcd3Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd3Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd3Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd3Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Attr(pub u16);
impl Tcd4Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd4AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd4AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd4AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd4Attr {
    #[inline(always)]
    fn default() -> Tcd4Attr {
        Tcd4Attr(0)
    }
}
impl core::fmt::Debug for Tcd4Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4BiterElinkno(pub u16);
impl Tcd4BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd4BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd4BiterElinkno {
        Tcd4BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd4BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4BiterElinkyes(pub u16);
impl Tcd4BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd4BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd4BiterElinkyes {
        Tcd4BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd4BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4CiterElinkno(pub u16);
impl Tcd4CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd4CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd4CiterElinkno {
        Tcd4CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd4CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4CiterElinkyes(pub u16);
impl Tcd4CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd4CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd4CiterElinkyes {
        Tcd4CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd4CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Csr(pub u16);
impl Tcd4Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd4CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd4CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd4CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd4CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd4CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd4CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd4Csr {
    #[inline(always)]
    fn default() -> Tcd4Csr {
        Tcd4Csr(0)
    }
}
impl core::fmt::Debug for Tcd4Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Daddr(pub u32);
impl Tcd4Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd4Daddr {
    #[inline(always)]
    fn default() -> Tcd4Daddr {
        Tcd4Daddr(0)
    }
}
impl core::fmt::Debug for Tcd4Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Dlastsga(pub u32);
impl Tcd4Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd4Dlastsga {
    #[inline(always)]
    fn default() -> Tcd4Dlastsga {
        Tcd4Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd4Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Doff(pub u16);
impl Tcd4Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd4Doff {
    #[inline(always)]
    fn default() -> Tcd4Doff {
        Tcd4Doff(0)
    }
}
impl core::fmt::Debug for Tcd4Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4NbytesMlno(pub u32);
impl Tcd4NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd4NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd4NbytesMlno {
        Tcd4NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd4NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4NbytesMloffno(pub u32);
impl Tcd4NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd4NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd4NbytesMloffno {
        Tcd4NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd4NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4NbytesMloffyes(pub u32);
impl Tcd4NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd4NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd4NbytesMloffyes {
        Tcd4NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd4NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd4NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Saddr(pub u32);
impl Tcd4Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd4Saddr {
    #[inline(always)]
    fn default() -> Tcd4Saddr {
        Tcd4Saddr(0)
    }
}
impl core::fmt::Debug for Tcd4Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Slast(pub u32);
impl Tcd4Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd4Slast {
    #[inline(always)]
    fn default() -> Tcd4Slast {
        Tcd4Slast(0)
    }
}
impl core::fmt::Debug for Tcd4Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4Soff(pub u16);
impl Tcd4Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd4Soff {
    #[inline(always)]
    fn default() -> Tcd4Soff {
        Tcd4Soff(0)
    }
}
impl core::fmt::Debug for Tcd4Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd4Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd4Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd4Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Attr(pub u16);
impl Tcd5Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd5AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd5AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd5AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd5Attr {
    #[inline(always)]
    fn default() -> Tcd5Attr {
        Tcd5Attr(0)
    }
}
impl core::fmt::Debug for Tcd5Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5BiterElinkno(pub u16);
impl Tcd5BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd5BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd5BiterElinkno {
        Tcd5BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd5BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5BiterElinkyes(pub u16);
impl Tcd5BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd5BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd5BiterElinkyes {
        Tcd5BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd5BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5CiterElinkno(pub u16);
impl Tcd5CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd5CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd5CiterElinkno {
        Tcd5CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd5CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5CiterElinkyes(pub u16);
impl Tcd5CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd5CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd5CiterElinkyes {
        Tcd5CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd5CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Csr(pub u16);
impl Tcd5Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd5CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd5CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd5CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd5CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd5CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd5CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd5Csr {
    #[inline(always)]
    fn default() -> Tcd5Csr {
        Tcd5Csr(0)
    }
}
impl core::fmt::Debug for Tcd5Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Daddr(pub u32);
impl Tcd5Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd5Daddr {
    #[inline(always)]
    fn default() -> Tcd5Daddr {
        Tcd5Daddr(0)
    }
}
impl core::fmt::Debug for Tcd5Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Dlastsga(pub u32);
impl Tcd5Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd5Dlastsga {
    #[inline(always)]
    fn default() -> Tcd5Dlastsga {
        Tcd5Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd5Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Doff(pub u16);
impl Tcd5Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd5Doff {
    #[inline(always)]
    fn default() -> Tcd5Doff {
        Tcd5Doff(0)
    }
}
impl core::fmt::Debug for Tcd5Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5NbytesMlno(pub u32);
impl Tcd5NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd5NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd5NbytesMlno {
        Tcd5NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd5NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5NbytesMloffno(pub u32);
impl Tcd5NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd5NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd5NbytesMloffno {
        Tcd5NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd5NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5NbytesMloffyes(pub u32);
impl Tcd5NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd5NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd5NbytesMloffyes {
        Tcd5NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd5NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd5NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Saddr(pub u32);
impl Tcd5Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd5Saddr {
    #[inline(always)]
    fn default() -> Tcd5Saddr {
        Tcd5Saddr(0)
    }
}
impl core::fmt::Debug for Tcd5Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Slast(pub u32);
impl Tcd5Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd5Slast {
    #[inline(always)]
    fn default() -> Tcd5Slast {
        Tcd5Slast(0)
    }
}
impl core::fmt::Debug for Tcd5Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd5Soff(pub u16);
impl Tcd5Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd5Soff {
    #[inline(always)]
    fn default() -> Tcd5Soff {
        Tcd5Soff(0)
    }
}
impl core::fmt::Debug for Tcd5Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd5Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd5Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd5Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Attr(pub u16);
impl Tcd6Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd6AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd6AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd6AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd6Attr {
    #[inline(always)]
    fn default() -> Tcd6Attr {
        Tcd6Attr(0)
    }
}
impl core::fmt::Debug for Tcd6Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6BiterElinkno(pub u16);
impl Tcd6BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd6BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd6BiterElinkno {
        Tcd6BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd6BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6BiterElinkyes(pub u16);
impl Tcd6BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd6BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd6BiterElinkyes {
        Tcd6BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd6BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6CiterElinkno(pub u16);
impl Tcd6CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd6CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd6CiterElinkno {
        Tcd6CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd6CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6CiterElinkyes(pub u16);
impl Tcd6CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd6CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd6CiterElinkyes {
        Tcd6CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd6CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Csr(pub u16);
impl Tcd6Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd6CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd6CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd6CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd6CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd6CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd6CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd6Csr {
    #[inline(always)]
    fn default() -> Tcd6Csr {
        Tcd6Csr(0)
    }
}
impl core::fmt::Debug for Tcd6Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Daddr(pub u32);
impl Tcd6Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd6Daddr {
    #[inline(always)]
    fn default() -> Tcd6Daddr {
        Tcd6Daddr(0)
    }
}
impl core::fmt::Debug for Tcd6Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Dlastsga(pub u32);
impl Tcd6Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd6Dlastsga {
    #[inline(always)]
    fn default() -> Tcd6Dlastsga {
        Tcd6Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd6Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Doff(pub u16);
impl Tcd6Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd6Doff {
    #[inline(always)]
    fn default() -> Tcd6Doff {
        Tcd6Doff(0)
    }
}
impl core::fmt::Debug for Tcd6Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6NbytesMlno(pub u32);
impl Tcd6NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd6NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd6NbytesMlno {
        Tcd6NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd6NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6NbytesMloffno(pub u32);
impl Tcd6NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd6NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd6NbytesMloffno {
        Tcd6NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd6NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6NbytesMloffyes(pub u32);
impl Tcd6NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd6NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd6NbytesMloffyes {
        Tcd6NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd6NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd6NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Saddr(pub u32);
impl Tcd6Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd6Saddr {
    #[inline(always)]
    fn default() -> Tcd6Saddr {
        Tcd6Saddr(0)
    }
}
impl core::fmt::Debug for Tcd6Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Slast(pub u32);
impl Tcd6Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd6Slast {
    #[inline(always)]
    fn default() -> Tcd6Slast {
        Tcd6Slast(0)
    }
}
impl core::fmt::Debug for Tcd6Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd6Soff(pub u16);
impl Tcd6Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd6Soff {
    #[inline(always)]
    fn default() -> Tcd6Soff {
        Tcd6Soff(0)
    }
}
impl core::fmt::Debug for Tcd6Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd6Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd6Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd6Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Attr(pub u16);
impl Tcd7Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd7AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd7AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd7AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd7Attr {
    #[inline(always)]
    fn default() -> Tcd7Attr {
        Tcd7Attr(0)
    }
}
impl core::fmt::Debug for Tcd7Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7BiterElinkno(pub u16);
impl Tcd7BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd7BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd7BiterElinkno {
        Tcd7BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd7BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7BiterElinkyes(pub u16);
impl Tcd7BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd7BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd7BiterElinkyes {
        Tcd7BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd7BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7CiterElinkno(pub u16);
impl Tcd7CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd7CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd7CiterElinkno {
        Tcd7CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd7CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7CiterElinkyes(pub u16);
impl Tcd7CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd7CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd7CiterElinkyes {
        Tcd7CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd7CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Csr(pub u16);
impl Tcd7Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd7CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd7CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd7CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd7CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd7CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd7CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd7Csr {
    #[inline(always)]
    fn default() -> Tcd7Csr {
        Tcd7Csr(0)
    }
}
impl core::fmt::Debug for Tcd7Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Daddr(pub u32);
impl Tcd7Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd7Daddr {
    #[inline(always)]
    fn default() -> Tcd7Daddr {
        Tcd7Daddr(0)
    }
}
impl core::fmt::Debug for Tcd7Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Dlastsga(pub u32);
impl Tcd7Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd7Dlastsga {
    #[inline(always)]
    fn default() -> Tcd7Dlastsga {
        Tcd7Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd7Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Doff(pub u16);
impl Tcd7Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd7Doff {
    #[inline(always)]
    fn default() -> Tcd7Doff {
        Tcd7Doff(0)
    }
}
impl core::fmt::Debug for Tcd7Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7NbytesMlno(pub u32);
impl Tcd7NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd7NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd7NbytesMlno {
        Tcd7NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd7NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7NbytesMloffno(pub u32);
impl Tcd7NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd7NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd7NbytesMloffno {
        Tcd7NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd7NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7NbytesMloffyes(pub u32);
impl Tcd7NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd7NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd7NbytesMloffyes {
        Tcd7NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd7NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd7NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Saddr(pub u32);
impl Tcd7Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd7Saddr {
    #[inline(always)]
    fn default() -> Tcd7Saddr {
        Tcd7Saddr(0)
    }
}
impl core::fmt::Debug for Tcd7Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Slast(pub u32);
impl Tcd7Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd7Slast {
    #[inline(always)]
    fn default() -> Tcd7Slast {
        Tcd7Slast(0)
    }
}
impl core::fmt::Debug for Tcd7Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd7Soff(pub u16);
impl Tcd7Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd7Soff {
    #[inline(always)]
    fn default() -> Tcd7Soff {
        Tcd7Soff(0)
    }
}
impl core::fmt::Debug for Tcd7Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd7Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd7Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd7Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Attr(pub u16);
impl Tcd8Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd8AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd8AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd8AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd8Attr {
    #[inline(always)]
    fn default() -> Tcd8Attr {
        Tcd8Attr(0)
    }
}
impl core::fmt::Debug for Tcd8Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8BiterElinkno(pub u16);
impl Tcd8BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd8BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd8BiterElinkno {
        Tcd8BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd8BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8BiterElinkyes(pub u16);
impl Tcd8BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd8BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd8BiterElinkyes {
        Tcd8BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd8BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8CiterElinkno(pub u16);
impl Tcd8CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd8CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd8CiterElinkno {
        Tcd8CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd8CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8CiterElinkyes(pub u16);
impl Tcd8CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd8CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd8CiterElinkyes {
        Tcd8CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd8CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Csr(pub u16);
impl Tcd8Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd8CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd8CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd8CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd8CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd8CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd8CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd8Csr {
    #[inline(always)]
    fn default() -> Tcd8Csr {
        Tcd8Csr(0)
    }
}
impl core::fmt::Debug for Tcd8Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Daddr(pub u32);
impl Tcd8Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd8Daddr {
    #[inline(always)]
    fn default() -> Tcd8Daddr {
        Tcd8Daddr(0)
    }
}
impl core::fmt::Debug for Tcd8Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Dlastsga(pub u32);
impl Tcd8Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd8Dlastsga {
    #[inline(always)]
    fn default() -> Tcd8Dlastsga {
        Tcd8Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd8Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Doff(pub u16);
impl Tcd8Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd8Doff {
    #[inline(always)]
    fn default() -> Tcd8Doff {
        Tcd8Doff(0)
    }
}
impl core::fmt::Debug for Tcd8Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8NbytesMlno(pub u32);
impl Tcd8NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd8NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd8NbytesMlno {
        Tcd8NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd8NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8NbytesMloffno(pub u32);
impl Tcd8NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd8NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd8NbytesMloffno {
        Tcd8NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd8NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8NbytesMloffyes(pub u32);
impl Tcd8NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd8NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd8NbytesMloffyes {
        Tcd8NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd8NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd8NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Saddr(pub u32);
impl Tcd8Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd8Saddr {
    #[inline(always)]
    fn default() -> Tcd8Saddr {
        Tcd8Saddr(0)
    }
}
impl core::fmt::Debug for Tcd8Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Slast(pub u32);
impl Tcd8Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd8Slast {
    #[inline(always)]
    fn default() -> Tcd8Slast {
        Tcd8Slast(0)
    }
}
impl core::fmt::Debug for Tcd8Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8Soff(pub u16);
impl Tcd8Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd8Soff {
    #[inline(always)]
    fn default() -> Tcd8Soff {
        Tcd8Soff(0)
    }
}
impl core::fmt::Debug for Tcd8Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd8Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd8Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd8Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[doc = "TCD Transfer Attributes"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Attr(pub u16);
impl Tcd9Attr {
    #[doc = "Destination data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Destination data transfer size"]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo"]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source data transfer size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Tcd9AttrSsize {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Tcd9AttrSsize::from_bits(val as u8)
    }
    #[doc = "Source data transfer size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Tcd9AttrSsize) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for Tcd9Attr {
    #[inline(always)]
    fn default() -> Tcd9Attr {
        Tcd9Attr(0)
    }
}
impl core::fmt::Debug for Tcd9Attr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Attr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Attr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9Attr {{ dsize: {=u8:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9BiterElinkno(pub u16);
impl Tcd9BiterElinkno {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd9BiterElinkno {
    #[inline(always)]
    fn default() -> Tcd9BiterElinkno {
        Tcd9BiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd9BiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9BiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9BiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9BiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9BiterElinkyes(pub u16);
impl Tcd9BiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd9BiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd9BiterElinkyes {
        Tcd9BiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd9BiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9BiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9BiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9BiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9CiterElinkno(pub u16);
impl Tcd9CiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd9CiterElinkno {
    #[inline(always)]
    fn default() -> Tcd9CiterElinkno {
        Tcd9CiterElinkno(0)
    }
}
impl core::fmt::Debug for Tcd9CiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9CiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9CiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9CiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9CiterElinkyes(pub u16);
impl Tcd9CiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for Tcd9CiterElinkyes {
    #[inline(always)]
    fn default() -> Tcd9CiterElinkyes {
        Tcd9CiterElinkyes(0)
    }
}
impl core::fmt::Debug for Tcd9CiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9CiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9CiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9CiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Csr(pub u16);
impl Tcd9Csr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> super::vals::Tcd9CsrEsg {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Tcd9CsrEsg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: super::vals::Tcd9CsrEsg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Tcd9CsrBwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tcd9CsrBwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Tcd9CsrBwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for Tcd9Csr {
    #[inline(always)]
    fn default() -> Tcd9Csr {
        Tcd9Csr(0)
    }
}
impl core::fmt::Debug for Tcd9Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Csr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("active", &self.active())
            .field("done", &self.done())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Csr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9Csr {{ start: {=bool:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {=bool:?}, esg: {:?}, majorelink: {=bool:?}, active: {=bool:?}, done: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.active(),
            self.done(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Daddr(pub u32);
impl Tcd9Daddr {
    #[doc = "Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address"]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd9Daddr {
    #[inline(always)]
    fn default() -> Tcd9Daddr {
        Tcd9Daddr(0)
    }
}
impl core::fmt::Debug for Tcd9Daddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Daddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Daddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Daddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Dlastsga(pub u32);
impl Tcd9Dlastsga {
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[must_use]
    #[inline(always)]
    pub const fn dlastsga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub const fn set_dlastsga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd9Dlastsga {
    #[inline(always)]
    fn default() -> Tcd9Dlastsga {
        Tcd9Dlastsga(0)
    }
}
impl core::fmt::Debug for Tcd9Dlastsga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Dlastsga")
            .field("dlastsga", &self.dlastsga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Dlastsga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Dlastsga {{ dlastsga: {=u32:?} }}", self.dlastsga())
    }
}
#[doc = "TCD Signed Destination Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Doff(pub u16);
impl Tcd9Doff {
    #[doc = "Destination Address Signed Offset"]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset"]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd9Doff {
    #[inline(always)]
    fn default() -> Tcd9Doff {
        Tcd9Doff(0)
    }
}
impl core::fmt::Debug for Tcd9Doff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Doff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Doff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Doff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9NbytesMlno(pub u32);
impl Tcd9NbytesMlno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd9NbytesMlno {
    #[inline(always)]
    fn default() -> Tcd9NbytesMlno {
        Tcd9NbytesMlno(0)
    }
}
impl core::fmt::Debug for Tcd9NbytesMlno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9NbytesMlno")
            .field("nbytes", &self.nbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9NbytesMlno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9NbytesMlno {{ nbytes: {=u32:?} }}", self.nbytes())
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9NbytesMloffno(pub u32);
impl Tcd9NbytesMloffno {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd9NbytesMloffno {
    #[inline(always)]
    fn default() -> Tcd9NbytesMloffno {
        Tcd9NbytesMloffno(0)
    }
}
impl core::fmt::Debug for Tcd9NbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9NbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9NbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9NbytesMloffno {{ nbytes: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9NbytesMloffyes(pub u32);
impl Tcd9NbytesMloffyes {
    #[doc = "Minor Byte Transfer Count"]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Minor Byte Transfer Count"]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "If SMLOE = 1 or DMLOE = 1, this field represents a sign-extended offset applied to the source or destination address to form the next-state value after the minor loop completes."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Source Minor Loop Offset Enable"]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcd9NbytesMloffyes {
    #[inline(always)]
    fn default() -> Tcd9NbytesMloffyes {
        Tcd9NbytesMloffyes(0)
    }
}
impl core::fmt::Debug for Tcd9NbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9NbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9NbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tcd9NbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {=bool:?}, smloe: {=bool:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Saddr(pub u32);
impl Tcd9Saddr {
    #[doc = "Source Address"]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address"]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd9Saddr {
    #[inline(always)]
    fn default() -> Tcd9Saddr {
        Tcd9Saddr(0)
    }
}
impl core::fmt::Debug for Tcd9Saddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Saddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Saddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Saddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Slast(pub u32);
impl Tcd9Slast {
    #[doc = "Last Source Address Adjustment"]
    #[must_use]
    #[inline(always)]
    pub const fn slast(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn set_slast(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tcd9Slast {
    #[inline(always)]
    fn default() -> Tcd9Slast {
        Tcd9Slast(0)
    }
}
impl core::fmt::Debug for Tcd9Slast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Slast")
            .field("slast", &self.slast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Slast {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Slast {{ slast: {=u32:?} }}", self.slast())
    }
}
#[doc = "TCD Signed Source Address Offset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd9Soff(pub u16);
impl Tcd9Soff {
    #[doc = "Source address signed offset"]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source address signed offset"]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for Tcd9Soff {
    #[inline(always)]
    fn default() -> Tcd9Soff {
        Tcd9Soff(0)
    }
}
impl core::fmt::Debug for Tcd9Soff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcd9Soff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcd9Soff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tcd9Soff {{ soff: {=u16:?} }}", self.soff())
    }
}
