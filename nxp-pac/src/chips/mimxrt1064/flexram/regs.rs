#[doc = "Interrupt Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSigEn(pub u32);
impl IntSigEn {
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn itcm_err_sig_en(&self) -> super::vals::ItcmErrSigEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ItcmErrSigEn::from_bits(val as u8)
    }
    #[doc = "ITCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_itcm_err_sig_en(&mut self, val: super::vals::ItcmErrSigEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm_err_sig_en(&self) -> super::vals::DtcmErrSigEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DtcmErrSigEn::from_bits(val as u8)
    }
    #[doc = "DTCM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_dtcm_err_sig_en(&mut self, val: super::vals::DtcmErrSigEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_err_sig_en(&self) -> super::vals::OcramErrSigEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OcramErrSigEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ocram_err_sig_en(&mut self, val: super::vals::OcramErrSigEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IntSigEn {
    #[inline(always)]
    fn default() -> IntSigEn {
        IntSigEn(0)
    }
}
impl core::fmt::Debug for IntSigEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSigEn")
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("reserved2", &self.reserved2())
            .field("itcm_err_sig_en", &self.itcm_err_sig_en())
            .field("dtcm_err_sig_en", &self.dtcm_err_sig_en())
            .field("ocram_err_sig_en", &self.ocram_err_sig_en())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSigEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSigEn {{ reserved0: {=bool:?}, reserved1: {=bool:?}, reserved2: {=bool:?}, itcm_err_sig_en: {:?}, dtcm_err_sig_en: {:?}, ocram_err_sig_en: {:?}, reserved: {=u32:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.reserved2(),
            self.itcm_err_sig_en(),
            self.dtcm_err_sig_en(),
            self.ocram_err_sig_en(),
            self.reserved()
        )
    }
}
#[doc = "Interrupt Status Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatEn(pub u32);
impl IntStatEn {
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn itcm_err_stat_en(&self) -> super::vals::ItcmErrStatEn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ItcmErrStatEn::from_bits(val as u8)
    }
    #[doc = "ITCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_itcm_err_stat_en(&mut self, val: super::vals::ItcmErrStatEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm_err_stat_en(&self) -> super::vals::DtcmErrStatEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DtcmErrStatEn::from_bits(val as u8)
    }
    #[doc = "DTCM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_dtcm_err_stat_en(&mut self, val: super::vals::DtcmErrStatEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Status Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_err_stat_en(&self) -> super::vals::OcramErrStatEn {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OcramErrStatEn::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status Enable"]
    #[inline(always)]
    pub const fn set_ocram_err_stat_en(&mut self, val: super::vals::OcramErrStatEn) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
    }
}
impl Default for IntStatEn {
    #[inline(always)]
    fn default() -> IntStatEn {
        IntStatEn(0)
    }
}
impl core::fmt::Debug for IntStatEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatEn")
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("reserved2", &self.reserved2())
            .field("itcm_err_stat_en", &self.itcm_err_stat_en())
            .field("dtcm_err_stat_en", &self.dtcm_err_stat_en())
            .field("ocram_err_stat_en", &self.ocram_err_stat_en())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatEn {{ reserved0: {=bool:?}, reserved1: {=bool:?}, reserved2: {=bool:?}, itcm_err_stat_en: {:?}, dtcm_err_stat_en: {:?}, ocram_err_stat_en: {:?}, reserved: {=u32:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.reserved2(),
            self.itcm_err_stat_en(),
            self.dtcm_err_stat_en(),
            self.ocram_err_stat_en(),
            self.reserved()
        )
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "ITCM Access Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn itcm_err_status(&self) -> super::vals::ItcmErrStatus {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::ItcmErrStatus::from_bits(val as u8)
    }
    #[doc = "ITCM Access Error Status"]
    #[inline(always)]
    pub const fn set_itcm_err_status(&mut self, val: super::vals::ItcmErrStatus) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DTCM Access Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dtcm_err_status(&self) -> super::vals::DtcmErrStatus {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DtcmErrStatus::from_bits(val as u8)
    }
    #[doc = "DTCM Access Error Status"]
    #[inline(always)]
    pub const fn set_dtcm_err_status(&mut self, val: super::vals::DtcmErrStatus) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM Access Error Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_err_status(&self) -> super::vals::OcramErrStatus {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::OcramErrStatus::from_bits(val as u8)
    }
    #[doc = "OCRAM Access Error Status"]
    #[inline(always)]
    pub const fn set_ocram_err_status(&mut self, val: super::vals::OcramErrStatus) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 6usize) & 0x03ff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
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
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .field("reserved2", &self.reserved2())
            .field("itcm_err_status", &self.itcm_err_status())
            .field("dtcm_err_status", &self.dtcm_err_status())
            .field("ocram_err_status", &self.ocram_err_status())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ reserved0: {=bool:?}, reserved1: {=bool:?}, reserved2: {=bool:?}, itcm_err_status: {:?}, dtcm_err_status: {:?}, ocram_err_status: {:?}, reserved: {=u32:?} }}",
            self.reserved0(),
            self.reserved1(),
            self.reserved2(),
            self.itcm_err_status(),
            self.dtcm_err_status(),
            self.ocram_err_status(),
            self.reserved()
        )
    }
}
#[doc = "TCM CRTL Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcmCtrl(pub u32);
impl TcmCtrl {
    #[doc = "TCM Write Wait Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcm_wwait_en(&self) -> super::vals::TcmWwaitEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TcmWwaitEn::from_bits(val as u8)
    }
    #[doc = "TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub const fn set_tcm_wwait_en(&mut self, val: super::vals::TcmWwaitEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "TCM Read Wait Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tcm_rwait_en(&self) -> super::vals::TcmRwaitEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::TcmRwaitEn::from_bits(val as u8)
    }
    #[doc = "TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub const fn set_tcm_rwait_en(&mut self, val: super::vals::TcmRwaitEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Force RAM Clock Always On"]
    #[must_use]
    #[inline(always)]
    pub const fn force_clk_on(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force RAM Clock Always On"]
    #[inline(always)]
    pub const fn set_force_clk_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub const fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for TcmCtrl {
    #[inline(always)]
    fn default() -> TcmCtrl {
        TcmCtrl(0)
    }
}
impl core::fmt::Debug for TcmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcmCtrl")
            .field("tcm_wwait_en", &self.tcm_wwait_en())
            .field("tcm_rwait_en", &self.tcm_rwait_en())
            .field("force_clk_on", &self.force_clk_on())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcmCtrl {{ tcm_wwait_en: {:?}, tcm_rwait_en: {:?}, force_clk_on: {=bool:?}, reserved: {=u32:?} }}",
            self.tcm_wwait_en(),
            self.tcm_rwait_en(),
            self.force_clk_on(),
            self.reserved()
        )
    }
}
