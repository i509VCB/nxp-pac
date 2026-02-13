#[doc = "PGC CPU Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuCtrl(pub u32);
impl CpuCtrl {
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::CpuCtrlPcr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CpuCtrlPcr::from_bits(val as u8)
    }
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::CpuCtrlPcr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CpuCtrl {
    #[inline(always)]
    fn default() -> CpuCtrl {
        CpuCtrl(0)
    }
}
impl core::fmt::Debug for CpuCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CpuCtrl").field("pcr", &self.pcr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CpuCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CpuCtrl {{ pcr: {:?} }}", self.pcr())
    }
}
#[doc = "PGC CPU Pull Down Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuPdnscr(pub u32);
impl CpuPdnscr {
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[must_use]
    #[inline(always)]
    pub const fn iso2sw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn set_iso2sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for CpuPdnscr {
    #[inline(always)]
    fn default() -> CpuPdnscr {
        CpuPdnscr(0)
    }
}
impl core::fmt::Debug for CpuPdnscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CpuPdnscr")
            .field("iso", &self.iso())
            .field("iso2sw", &self.iso2sw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CpuPdnscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CpuPdnscr {{ iso: {=u8:?}, iso2sw: {=u8:?} }}",
            self.iso(),
            self.iso2sw()
        )
    }
}
#[doc = "PGC CPU Power Up Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuPupscr(pub u32);
impl CpuPupscr {
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn sw2iso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn set_sw2iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for CpuPupscr {
    #[inline(always)]
    fn default() -> CpuPupscr {
        CpuPupscr(0)
    }
}
impl core::fmt::Debug for CpuPupscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CpuPupscr")
            .field("sw", &self.sw())
            .field("sw2iso", &self.sw2iso())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CpuPupscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CpuPupscr {{ sw: {=u8:?}, sw2iso: {=u8:?} }}",
            self.sw(),
            self.sw2iso()
        )
    }
}
#[doc = "PGC CPU Power Gating Controller Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuSr(pub u32);
impl CpuSr {
    #[doc = "Power status"]
    #[must_use]
    #[inline(always)]
    pub const fn psr(&self) -> super::vals::CpuSrPsr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::CpuSrPsr::from_bits(val as u8)
    }
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn set_psr(&mut self, val: super::vals::CpuSrPsr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for CpuSr {
    #[inline(always)]
    fn default() -> CpuSr {
        CpuSr(0)
    }
}
impl core::fmt::Debug for CpuSr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CpuSr").field("psr", &self.psr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CpuSr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CpuSr {{ psr: {:?} }}", self.psr())
    }
}
#[doc = "PGC Mega Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MegaCtrl(pub u32);
impl MegaCtrl {
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[must_use]
    #[inline(always)]
    pub const fn pcr(&self) -> super::vals::MegaCtrlPcr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MegaCtrlPcr::from_bits(val as u8)
    }
    #[doc = "Power Control PCR must not change from power-down request (pdn_req) assertion until the target subsystem is completely powered up"]
    #[inline(always)]
    pub const fn set_pcr(&mut self, val: super::vals::MegaCtrlPcr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for MegaCtrl {
    #[inline(always)]
    fn default() -> MegaCtrl {
        MegaCtrl(0)
    }
}
impl core::fmt::Debug for MegaCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MegaCtrl")
            .field("pcr", &self.pcr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MegaCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MegaCtrl {{ pcr: {:?} }}", self.pcr())
    }
}
#[doc = "PGC Mega Pull Down Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MegaPdnscr(pub u32);
impl MegaPdnscr {
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn iso(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-down request (pdn_req assertion), the PGC waits a number of IPG clocks equal to the value of ISO before asserting isolation"]
    #[inline(always)]
    pub const fn set_iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[must_use]
    #[inline(always)]
    pub const fn iso2sw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting isolation, the PGC waits a number of IPG clocks equal to the value of ISO2SW before negating power toggle on/off signal (switch_b) to switch off power"]
    #[inline(always)]
    pub const fn set_iso2sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for MegaPdnscr {
    #[inline(always)]
    fn default() -> MegaPdnscr {
        MegaPdnscr(0)
    }
}
impl core::fmt::Debug for MegaPdnscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MegaPdnscr")
            .field("iso", &self.iso())
            .field("iso2sw", &self.iso2sw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MegaPdnscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MegaPdnscr {{ iso: {=u8:?}, iso2sw: {=u8:?} }}",
            self.iso(),
            self.iso2sw()
        )
    }
}
#[doc = "PGC Mega Power Up Sequence Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MegaPupscr(pub u32);
impl MegaPupscr {
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[must_use]
    #[inline(always)]
    pub const fn sw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "After a power-up request (pup_req assertion), the PGC waits a number of IPG clocks equal to the value of SW before asserting power toggle on/off signal (switch_b) to switch on power"]
    #[inline(always)]
    pub const fn set_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[must_use]
    #[inline(always)]
    pub const fn sw2iso(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "After asserting power toggle on/off signal (switch_b) to switch on power, the PGC waits a number of IPG clocks equal to the value of SW2ISO before negating isolation"]
    #[inline(always)]
    pub const fn set_sw2iso(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
}
impl Default for MegaPupscr {
    #[inline(always)]
    fn default() -> MegaPupscr {
        MegaPupscr(0)
    }
}
impl core::fmt::Debug for MegaPupscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MegaPupscr")
            .field("sw", &self.sw())
            .field("sw2iso", &self.sw2iso())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MegaPupscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MegaPupscr {{ sw: {=u8:?}, sw2iso: {=u8:?} }}",
            self.sw(),
            self.sw2iso()
        )
    }
}
#[doc = "PGC Mega Power Gating Controller Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MegaSr(pub u32);
impl MegaSr {
    #[doc = "Power status"]
    #[must_use]
    #[inline(always)]
    pub const fn psr(&self) -> super::vals::MegaSrPsr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MegaSrPsr::from_bits(val as u8)
    }
    #[doc = "Power status"]
    #[inline(always)]
    pub const fn set_psr(&mut self, val: super::vals::MegaSrPsr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for MegaSr {
    #[inline(always)]
    fn default() -> MegaSr {
        MegaSr(0)
    }
}
impl core::fmt::Debug for MegaSr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MegaSr").field("psr", &self.psr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MegaSr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MegaSr {{ psr: {:?} }}", self.psr())
    }
}
