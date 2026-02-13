#[doc = "Entropy Read Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ent(pub u32);
impl Ent {
    #[doc = "Entropy Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ent(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Entropy Value"]
    #[inline(always)]
    pub const fn set_ent(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ent {
    #[inline(always)]
    fn default() -> Ent {
        Ent(0)
    }
}
impl core::fmt::Debug for Ent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ent").field("ent", &self.ent()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ent {{ ent: {=u32:?} }}", self.ent())
    }
}
#[doc = "Frequency Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqcnt(pub u32);
impl Frqcnt {
    #[doc = "Frequency Count"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count"]
    #[inline(always)]
    pub const fn set_frq_ct(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqcnt {
    #[inline(always)]
    fn default() -> Frqcnt {
        Frqcnt(0)
    }
}
impl core::fmt::Debug for Frqcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqcnt")
            .field("frq_ct", &self.frq_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqcnt {{ frq_ct: {=u32:?} }}", self.frq_ct())
    }
}
#[doc = "Frequency Count Maximum Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmax(pub u32);
impl Frqmax {
    #[doc = "Frequency Counter Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_max(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Counter Maximum Limit"]
    #[inline(always)]
    pub const fn set_frq_max(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmax {
    #[inline(always)]
    fn default() -> Frqmax {
        Frqmax(0)
    }
}
impl core::fmt::Debug for Frqmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmax")
            .field("frq_max", &self.frq_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmax {{ frq_max: {=u32:?} }}", self.frq_max())
    }
}
#[doc = "Frequency Count Minimum Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frqmin(pub u32);
impl Frqmin {
    #[doc = "Frequency Count Minimum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_min(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Frequency Count Minimum Limit"]
    #[inline(always)]
    pub const fn set_frq_min(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for Frqmin {
    #[inline(always)]
    fn default() -> Frqmin {
        Frqmin(0)
    }
}
impl core::fmt::Debug for Frqmin {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frqmin")
            .field("frq_min", &self.frq_min())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frqmin {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Frqmin {{ frq_min: {=u32:?} }}", self.frq_min())
    }
}
#[doc = "Interrupt Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntCtrl(pub u32);
impl IntCtrl {
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> super::vals::IntCtrlHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntCtrlHwErr::from_bits(val as u8)
    }
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: super::vals::IntCtrlHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Same behavior as bit 0 above."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> super::vals::IntCtrlEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntCtrlEntVal::from_bits(val as u8)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: super::vals::IntCtrlEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Same behavior as bit 0 above."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> super::vals::IntCtrlFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IntCtrlFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: super::vals::IntCtrlFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for IntCtrl {
    #[inline(always)]
    fn default() -> IntCtrl {
        IntCtrl(0)
    }
}
impl core::fmt::Debug for IntCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntCtrl")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntCtrl {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail()
        )
    }
}
#[doc = "Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> super::vals::IntMaskHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntMaskHwErr::from_bits(val as u8)
    }
    #[doc = "Bit position that can be cleared if corresponding bit of INT_STATUS has been asserted."]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: super::vals::IntMaskHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Same behavior as bit 0 above."]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> super::vals::IntMaskEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntMaskEntVal::from_bits(val as u8)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: super::vals::IntMaskEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Same behavior as bit 0 above."]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> super::vals::IntMaskFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IntMaskFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Same behavior as bit 0 above."]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: super::vals::IntMaskFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMask {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail()
        )
    }
}
#[doc = "Interrupt Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Read: Error status"]
    #[must_use]
    #[inline(always)]
    pub const fn hw_err(&self) -> super::vals::IntStatusHwErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IntStatusHwErr::from_bits(val as u8)
    }
    #[doc = "Read: Error status"]
    #[inline(always)]
    pub const fn set_hw_err(&mut self, val: super::vals::IntStatusHwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Read only: Entropy Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> super::vals::IntStatusEntVal {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IntStatusEntVal::from_bits(val as u8)
    }
    #[doc = "Read only: Entropy Valid"]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: super::vals::IntStatusEntVal) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Read only: Frequency Count Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn frq_ct_fail(&self) -> super::vals::IntStatusFrqCtFail {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IntStatusFrqCtFail::from_bits(val as u8)
    }
    #[doc = "Read only: Frequency Count Fail"]
    #[inline(always)]
    pub const fn set_frq_ct_fail(&mut self, val: super::vals::IntStatusFrqCtFail) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
            .field("hw_err", &self.hw_err())
            .field("ent_val", &self.ent_val())
            .field("frq_ct_fail", &self.frq_ct_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ hw_err: {:?}, ent_val: {:?}, frq_ct_fail: {:?} }}",
            self.hw_err(),
            self.ent_val(),
            self.frq_ct_fail()
        )
    }
}
#[doc = "Miscellaneous Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctl(pub u32);
impl Mctl {
    #[doc = "Sample Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn samp_mode(&self) -> super::vals::SampMode {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SampMode::from_bits(val as u8)
    }
    #[doc = "Sample Mode"]
    #[inline(always)]
    pub const fn set_samp_mode(&mut self, val: super::vals::SampMode) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Oscillator Divide"]
    #[must_use]
    #[inline(always)]
    pub const fn osc_div(&self) -> super::vals::OscDiv {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::OscDiv::from_bits(val as u8)
    }
    #[doc = "Oscillator Divide"]
    #[inline(always)]
    pub const fn set_osc_div(&mut self, val: super::vals::OscDiv) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "TRNG Access Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_acc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG Access Mode"]
    #[inline(always)]
    pub const fn set_trng_acc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Reset Defaults"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_def(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Defaults"]
    #[inline(always)]
    pub const fn set_rst_def(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Force System Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn for_sclk(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force System Clock"]
    #[inline(always)]
    pub const fn set_for_sclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Read only: Frequency Count Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn fct_fail(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Read only: Frequency Count Fail"]
    #[inline(always)]
    pub const fn set_fct_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[must_use]
    #[inline(always)]
    pub const fn fct_val(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Read only: Frequency Count Valid. Indicates that a valid frequency count may be read from FRQCNT."]
    #[inline(always)]
    pub const fn set_fct_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Read only: Entropy Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_val(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Read only: Entropy Valid"]
    #[inline(always)]
    pub const fn set_ent_val(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Read only: Test point inside ring oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn tst_out(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Read only: Test point inside ring oscillator."]
    #[inline(always)]
    pub const fn set_tst_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Read: Error status"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Read: Error status"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TRNG_OK_TO_STOP"]
    #[must_use]
    #[inline(always)]
    pub const fn tstop_ok(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG_OK_TO_STOP"]
    #[inline(always)]
    pub const fn set_tstop_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Programming Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn prgm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Programming Mode Select"]
    #[inline(always)]
    pub const fn set_prgm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("samp_mode", &self.samp_mode())
            .field("osc_div", &self.osc_div())
            .field("trng_acc", &self.trng_acc())
            .field("rst_def", &self.rst_def())
            .field("for_sclk", &self.for_sclk())
            .field("fct_fail", &self.fct_fail())
            .field("fct_val", &self.fct_val())
            .field("ent_val", &self.ent_val())
            .field("tst_out", &self.tst_out())
            .field("err", &self.err())
            .field("tstop_ok", &self.tstop_ok())
            .field("prgm", &self.prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctl {{ samp_mode: {:?}, osc_div: {:?}, trng_acc: {=bool:?}, rst_def: {=bool:?}, for_sclk: {=bool:?}, fct_fail: {=bool:?}, fct_val: {=bool:?}, ent_val: {=bool:?}, tst_out: {=bool:?}, err: {=bool:?}, tstop_ok: {=bool:?}, prgm: {=bool:?} }}",
            self.samp_mode(),
            self.osc_div(),
            self.trng_acc(),
            self.rst_def(),
            self.for_sclk(),
            self.fct_fail(),
            self.fct_val(),
            self.ent_val(),
            self.tst_out(),
            self.err(),
            self.tstop_ok(),
            self.prgm()
        )
    }
}
#[doc = "Statistical Check Poker Count 1 and 0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcnt10(pub u32);
impl Pkrcnt10 {
    #[doc = "Poker 0h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 0h Count"]
    #[inline(always)]
    pub const fn set_pkr_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker 1h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 1h Count"]
    #[inline(always)]
    pub const fn set_pkr_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcnt10 {
    #[inline(always)]
    fn default() -> Pkrcnt10 {
        Pkrcnt10(0)
    }
}
impl core::fmt::Debug for Pkrcnt10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcnt10")
            .field("pkr_0_ct", &self.pkr_0_ct())
            .field("pkr_1_ct", &self.pkr_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcnt10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcnt10 {{ pkr_0_ct: {=u16:?}, pkr_1_ct: {=u16:?} }}",
            self.pkr_0_ct(),
            self.pkr_1_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count 3 and 2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcnt32(pub u32);
impl Pkrcnt32 {
    #[doc = "Poker 2h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_2_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 2h Count"]
    #[inline(always)]
    pub const fn set_pkr_2_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker 3h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_3_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 3h Count"]
    #[inline(always)]
    pub const fn set_pkr_3_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcnt32 {
    #[inline(always)]
    fn default() -> Pkrcnt32 {
        Pkrcnt32(0)
    }
}
impl core::fmt::Debug for Pkrcnt32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcnt32")
            .field("pkr_2_ct", &self.pkr_2_ct())
            .field("pkr_3_ct", &self.pkr_3_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcnt32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcnt32 {{ pkr_2_ct: {=u16:?}, pkr_3_ct: {=u16:?} }}",
            self.pkr_2_ct(),
            self.pkr_3_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count 5 and 4 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcnt54(pub u32);
impl Pkrcnt54 {
    #[doc = "Poker 4h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_4_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 4h Count"]
    #[inline(always)]
    pub const fn set_pkr_4_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker 5h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_5_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 5h Count"]
    #[inline(always)]
    pub const fn set_pkr_5_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcnt54 {
    #[inline(always)]
    fn default() -> Pkrcnt54 {
        Pkrcnt54(0)
    }
}
impl core::fmt::Debug for Pkrcnt54 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcnt54")
            .field("pkr_4_ct", &self.pkr_4_ct())
            .field("pkr_5_ct", &self.pkr_5_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcnt54 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcnt54 {{ pkr_4_ct: {=u16:?}, pkr_5_ct: {=u16:?} }}",
            self.pkr_4_ct(),
            self.pkr_5_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count 7 and 6 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcnt76(pub u32);
impl Pkrcnt76 {
    #[doc = "Poker 6h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_6_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 6h Count"]
    #[inline(always)]
    pub const fn set_pkr_6_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker 7h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_7_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 7h Count"]
    #[inline(always)]
    pub const fn set_pkr_7_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcnt76 {
    #[inline(always)]
    fn default() -> Pkrcnt76 {
        Pkrcnt76(0)
    }
}
impl core::fmt::Debug for Pkrcnt76 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcnt76")
            .field("pkr_6_ct", &self.pkr_6_ct())
            .field("pkr_7_ct", &self.pkr_7_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcnt76 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcnt76 {{ pkr_6_ct: {=u16:?}, pkr_7_ct: {=u16:?} }}",
            self.pkr_6_ct(),
            self.pkr_7_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count 9 and 8 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcnt98(pub u32);
impl Pkrcnt98 {
    #[doc = "Poker 8h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_8_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 8h Count"]
    #[inline(always)]
    pub const fn set_pkr_8_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker 9h Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_9_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker 9h Count"]
    #[inline(always)]
    pub const fn set_pkr_9_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcnt98 {
    #[inline(always)]
    fn default() -> Pkrcnt98 {
        Pkrcnt98(0)
    }
}
impl core::fmt::Debug for Pkrcnt98 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcnt98")
            .field("pkr_8_ct", &self.pkr_8_ct())
            .field("pkr_9_ct", &self.pkr_9_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcnt98 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcnt98 {{ pkr_8_ct: {=u16:?}, pkr_9_ct: {=u16:?} }}",
            self.pkr_8_ct(),
            self.pkr_9_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count B and A Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcntba(pub u32);
impl Pkrcntba {
    #[doc = "Poker Ah Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_a_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Ah Count"]
    #[inline(always)]
    pub const fn set_pkr_a_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker Bh Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_b_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Bh Count"]
    #[inline(always)]
    pub const fn set_pkr_b_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcntba {
    #[inline(always)]
    fn default() -> Pkrcntba {
        Pkrcntba(0)
    }
}
impl core::fmt::Debug for Pkrcntba {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcntba")
            .field("pkr_a_ct", &self.pkr_a_ct())
            .field("pkr_b_ct", &self.pkr_b_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcntba {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcntba {{ pkr_a_ct: {=u16:?}, pkr_b_ct: {=u16:?} }}",
            self.pkr_a_ct(),
            self.pkr_b_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count D and C Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcntdc(pub u32);
impl Pkrcntdc {
    #[doc = "Poker Ch Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_c_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Ch Count"]
    #[inline(always)]
    pub const fn set_pkr_c_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker Dh Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_d_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Dh Count"]
    #[inline(always)]
    pub const fn set_pkr_d_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcntdc {
    #[inline(always)]
    fn default() -> Pkrcntdc {
        Pkrcntdc(0)
    }
}
impl core::fmt::Debug for Pkrcntdc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcntdc")
            .field("pkr_c_ct", &self.pkr_c_ct())
            .field("pkr_d_ct", &self.pkr_d_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcntdc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcntdc {{ pkr_c_ct: {=u16:?}, pkr_d_ct: {=u16:?} }}",
            self.pkr_c_ct(),
            self.pkr_d_ct()
        )
    }
}
#[doc = "Statistical Check Poker Count F and E Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrcntfe(pub u32);
impl Pkrcntfe {
    #[doc = "Poker Eh Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_e_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Eh Count"]
    #[inline(always)]
    pub const fn set_pkr_e_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Poker Fh Count"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_f_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Fh Count"]
    #[inline(always)]
    pub const fn set_pkr_f_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pkrcntfe {
    #[inline(always)]
    fn default() -> Pkrcntfe {
        Pkrcntfe(0)
    }
}
impl core::fmt::Debug for Pkrcntfe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrcntfe")
            .field("pkr_e_ct", &self.pkr_e_ct())
            .field("pkr_f_ct", &self.pkr_f_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrcntfe {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pkrcntfe {{ pkr_e_ct: {=u16:?}, pkr_f_ct: {=u16:?} }}",
            self.pkr_e_ct(),
            self.pkr_f_ct()
        )
    }
}
#[doc = "Poker Maximum Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrmax(pub u32);
impl Pkrmax {
    #[doc = "Poker Maximum Limit."]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_max(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Poker Maximum Limit."]
    #[inline(always)]
    pub const fn set_pkr_max(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pkrmax {
    #[inline(always)]
    fn default() -> Pkrmax {
        Pkrmax(0)
    }
}
impl core::fmt::Debug for Pkrmax {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrmax")
            .field("pkr_max", &self.pkr_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrmax {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pkrmax {{ pkr_max: {=u32:?} }}", self.pkr_max())
    }
}
#[doc = "Poker Range Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrrng(pub u32);
impl Pkrrng {
    #[doc = "Poker Range"]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_rng(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Poker Range"]
    #[inline(always)]
    pub const fn set_pkr_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pkrrng {
    #[inline(always)]
    fn default() -> Pkrrng {
        Pkrrng(0)
    }
}
impl core::fmt::Debug for Pkrrng {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrrng")
            .field("pkr_rng", &self.pkr_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrrng {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pkrrng {{ pkr_rng: {=u16:?} }}", self.pkr_rng())
    }
}
#[doc = "Poker Square Calculation Result Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkrsq(pub u32);
impl Pkrsq {
    #[doc = "Poker Square Calculation Result."]
    #[must_use]
    #[inline(always)]
    pub const fn pkr_sq(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Poker Square Calculation Result."]
    #[inline(always)]
    pub const fn set_pkr_sq(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pkrsq {
    #[inline(always)]
    fn default() -> Pkrsq {
        Pkrsq(0)
    }
}
impl core::fmt::Debug for Pkrsq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pkrsq")
            .field("pkr_sq", &self.pkr_sq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pkrsq {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pkrsq {{ pkr_sq: {=u32:?} }}", self.pkr_sq())
    }
}
#[doc = "Sparse Bit Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sblim(pub u32);
impl Sblim {
    #[doc = "Sparse Bit Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_lim(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sparse Bit Limit"]
    #[inline(always)]
    pub const fn set_sb_lim(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Sblim {
    #[inline(always)]
    fn default() -> Sblim {
        Sblim(0)
    }
}
impl core::fmt::Debug for Sblim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sblim")
            .field("sb_lim", &self.sb_lim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sblim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sblim {{ sb_lim: {=u16:?} }}", self.sb_lim())
    }
}
#[doc = "Statistical Check Monobit Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmc(pub u32);
impl Scmc {
    #[doc = "Monobit Count"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Count"]
    #[inline(always)]
    pub const fn set_mono_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Scmc {
    #[inline(always)]
    fn default() -> Scmc {
        Scmc(0)
    }
}
impl core::fmt::Debug for Scmc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmc")
            .field("mono_ct", &self.mono_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scmc {{ mono_ct: {=u16:?} }}", self.mono_ct())
    }
}
#[doc = "Statistical Check Miscellaneous Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scmisc(pub u32);
impl Scmisc {
    #[doc = "LONG RUN MAX LIMIT"]
    #[must_use]
    #[inline(always)]
    pub const fn lrun_max(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub const fn set_lrun_max(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "RETRY COUNT"]
    #[must_use]
    #[inline(always)]
    pub const fn rty_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RETRY COUNT"]
    #[inline(always)]
    pub const fn set_rty_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Scmisc {
    #[inline(always)]
    fn default() -> Scmisc {
        Scmisc(0)
    }
}
impl core::fmt::Debug for Scmisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scmisc")
            .field("lrun_max", &self.lrun_max())
            .field("rty_ct", &self.rty_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scmisc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scmisc {{ lrun_max: {=u8:?}, rty_ct: {=u8:?} }}",
            self.lrun_max(),
            self.rty_ct()
        )
    }
}
#[doc = "Statistical Check Monobit Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scml(pub u32);
impl Scml {
    #[doc = "Monobit Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Maximum Limit"]
    #[inline(always)]
    pub const fn set_mono_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Monobit Range"]
    #[must_use]
    #[inline(always)]
    pub const fn mono_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Monobit Range"]
    #[inline(always)]
    pub const fn set_mono_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Scml {
    #[inline(always)]
    fn default() -> Scml {
        Scml(0)
    }
}
impl core::fmt::Debug for Scml {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scml")
            .field("mono_max", &self.mono_max())
            .field("mono_rng", &self.mono_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scml {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scml {{ mono_max: {=u16:?}, mono_rng: {=u16:?} }}",
            self.mono_max(),
            self.mono_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1c(pub u32);
impl Scr1c {
    #[doc = "Runs of Zero, Length 1 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r1_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 1 Count"]
    #[inline(always)]
    pub const fn set_r1_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Runs of One, Length 1 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r1_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Runs of One, Length 1 Count"]
    #[inline(always)]
    pub const fn set_r1_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1c {
    #[inline(always)]
    fn default() -> Scr1c {
        Scr1c(0)
    }
}
impl core::fmt::Debug for Scr1c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1c")
            .field("r1_0_ct", &self.r1_0_ct())
            .field("r1_1_ct", &self.r1_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1c {{ r1_0_ct: {=u16:?}, r1_1_ct: {=u16:?} }}",
            self.r1_0_ct(),
            self.r1_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 1 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr1l(pub u32);
impl Scr1l {
    #[doc = "Run Length 1 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run1_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run1_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Run Length 1 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run1_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "Run Length 1 Range"]
    #[inline(always)]
    pub const fn set_run1_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Scr1l {
    #[inline(always)]
    fn default() -> Scr1l {
        Scr1l(0)
    }
}
impl core::fmt::Debug for Scr1l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr1l")
            .field("run1_max", &self.run1_max())
            .field("run1_rng", &self.run1_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr1l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr1l {{ run1_max: {=u16:?}, run1_rng: {=u16:?} }}",
            self.run1_max(),
            self.run1_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2c(pub u32);
impl Scr2c {
    #[doc = "Runs of Zero, Length 2 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r2_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 2 Count"]
    #[inline(always)]
    pub const fn set_r2_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Runs of One, Length 2 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r2_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Runs of One, Length 2 Count"]
    #[inline(always)]
    pub const fn set_r2_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2c {
    #[inline(always)]
    fn default() -> Scr2c {
        Scr2c(0)
    }
}
impl core::fmt::Debug for Scr2c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2c")
            .field("r2_0_ct", &self.r2_0_ct())
            .field("r2_1_ct", &self.r2_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2c {{ r2_0_ct: {=u16:?}, r2_1_ct: {=u16:?} }}",
            self.r2_0_ct(),
            self.r2_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 2 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr2l(pub u32);
impl Scr2l {
    #[doc = "Run Length 2 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run2_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run2_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Run Length 2 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run2_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Run Length 2 Range"]
    #[inline(always)]
    pub const fn set_run2_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Scr2l {
    #[inline(always)]
    fn default() -> Scr2l {
        Scr2l(0)
    }
}
impl core::fmt::Debug for Scr2l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr2l")
            .field("run2_max", &self.run2_max())
            .field("run2_rng", &self.run2_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr2l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr2l {{ run2_max: {=u16:?}, run2_rng: {=u16:?} }}",
            self.run2_max(),
            self.run2_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3c(pub u32);
impl Scr3c {
    #[doc = "Runs of Zeroes, Length 3 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r3_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Zeroes, Length 3 Count"]
    #[inline(always)]
    pub const fn set_r3_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Runs of Ones, Length 3 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r3_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Runs of Ones, Length 3 Count"]
    #[inline(always)]
    pub const fn set_r3_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3c {
    #[inline(always)]
    fn default() -> Scr3c {
        Scr3c(0)
    }
}
impl core::fmt::Debug for Scr3c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3c")
            .field("r3_0_ct", &self.r3_0_ct())
            .field("r3_1_ct", &self.r3_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3c {{ r3_0_ct: {=u16:?}, r3_1_ct: {=u16:?} }}",
            self.r3_0_ct(),
            self.r3_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 3 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr3l(pub u32);
impl Scr3l {
    #[doc = "Run Length 3 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run3_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run3_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Run Length 3 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run3_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[doc = "Run Length 3 Range"]
    #[inline(always)]
    pub const fn set_run3_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for Scr3l {
    #[inline(always)]
    fn default() -> Scr3l {
        Scr3l(0)
    }
}
impl core::fmt::Debug for Scr3l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr3l")
            .field("run3_max", &self.run3_max())
            .field("run3_rng", &self.run3_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr3l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr3l {{ run3_max: {=u16:?}, run3_rng: {=u16:?} }}",
            self.run3_max(),
            self.run3_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 4 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr4c(pub u32);
impl Scr4c {
    #[doc = "Runs of Zero, Length 4 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r4_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 4 Count"]
    #[inline(always)]
    pub const fn set_r4_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Runs of One, Length 4 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r4_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Runs of One, Length 4 Count"]
    #[inline(always)]
    pub const fn set_r4_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Scr4c {
    #[inline(always)]
    fn default() -> Scr4c {
        Scr4c(0)
    }
}
impl core::fmt::Debug for Scr4c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr4c")
            .field("r4_0_ct", &self.r4_0_ct())
            .field("r4_1_ct", &self.r4_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr4c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr4c {{ r4_0_ct: {=u16:?}, r4_1_ct: {=u16:?} }}",
            self.r4_0_ct(),
            self.r4_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 4 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr4l(pub u32);
impl Scr4l {
    #[doc = "Run Length 4 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run4_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Run Length 4 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run4_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Run Length 4 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run4_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Run Length 4 Range"]
    #[inline(always)]
    pub const fn set_run4_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Scr4l {
    #[inline(always)]
    fn default() -> Scr4l {
        Scr4l(0)
    }
}
impl core::fmt::Debug for Scr4l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr4l")
            .field("run4_max", &self.run4_max())
            .field("run4_rng", &self.run4_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr4l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr4l {{ run4_max: {=u16:?}, run4_rng: {=u16:?} }}",
            self.run4_max(),
            self.run4_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 5 Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr5c(pub u32);
impl Scr5c {
    #[doc = "Runs of Zero, Length 5 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r5_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 5 Count"]
    #[inline(always)]
    pub const fn set_r5_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Runs of One, Length 5 Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r5_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Runs of One, Length 5 Count"]
    #[inline(always)]
    pub const fn set_r5_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Scr5c {
    #[inline(always)]
    fn default() -> Scr5c {
        Scr5c(0)
    }
}
impl core::fmt::Debug for Scr5c {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr5c")
            .field("r5_0_ct", &self.r5_0_ct())
            .field("r5_1_ct", &self.r5_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr5c {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr5c {{ r5_0_ct: {=u16:?}, r5_1_ct: {=u16:?} }}",
            self.r5_0_ct(),
            self.r5_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 5 Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr5l(pub u32);
impl Scr5l {
    #[doc = "Run Length 5 Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run5_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Run Length 5 Maximum Limit"]
    #[inline(always)]
    pub const fn set_run5_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Run Length 5 Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run5_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Run Length 5 Range"]
    #[inline(always)]
    pub const fn set_run5_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Scr5l {
    #[inline(always)]
    fn default() -> Scr5l {
        Scr5l(0)
    }
}
impl core::fmt::Debug for Scr5l {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr5l")
            .field("run5_max", &self.run5_max())
            .field("run5_rng", &self.run5_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr5l {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr5l {{ run5_max: {=u16:?}, run5_rng: {=u16:?} }}",
            self.run5_max(),
            self.run5_rng()
        )
    }
}
#[doc = "Statistical Check Run Length 6+ Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr6pc(pub u32);
impl Scr6pc {
    #[doc = "Runs of Zero, Length 6+ Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r6p_0_ct(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Runs of Zero, Length 6+ Count"]
    #[inline(always)]
    pub const fn set_r6p_0_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Runs of One, Length 6+ Count"]
    #[must_use]
    #[inline(always)]
    pub const fn r6p_1_ct(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Runs of One, Length 6+ Count"]
    #[inline(always)]
    pub const fn set_r6p_1_ct(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Scr6pc {
    #[inline(always)]
    fn default() -> Scr6pc {
        Scr6pc(0)
    }
}
impl core::fmt::Debug for Scr6pc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr6pc")
            .field("r6p_0_ct", &self.r6p_0_ct())
            .field("r6p_1_ct", &self.r6p_1_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr6pc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr6pc {{ r6p_0_ct: {=u16:?}, r6p_1_ct: {=u16:?} }}",
            self.r6p_0_ct(),
            self.r6p_1_ct()
        )
    }
}
#[doc = "Statistical Check Run Length 6+ Limit Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr6pl(pub u32);
impl Scr6pl {
    #[doc = "Run Length 6+ Maximum Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn run6p_max(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Run Length 6+ Maximum Limit"]
    #[inline(always)]
    pub const fn set_run6p_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Run Length 6+ Range"]
    #[must_use]
    #[inline(always)]
    pub const fn run6p_rng(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Run Length 6+ Range"]
    #[inline(always)]
    pub const fn set_run6p_rng(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Scr6pl {
    #[inline(always)]
    fn default() -> Scr6pl {
        Scr6pl(0)
    }
}
impl core::fmt::Debug for Scr6pl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr6pl")
            .field("run6p_max", &self.run6p_max())
            .field("run6p_rng", &self.run6p_rng())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr6pl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr6pl {{ run6p_max: {=u16:?}, run6p_rng: {=u16:?} }}",
            self.run6p_max(),
            self.run6p_rng()
        )
    }
}
#[doc = "Seed Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdctl(pub u32);
impl Sdctl {
    #[doc = "Sample Size"]
    #[must_use]
    #[inline(always)]
    pub const fn samp_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Sample Size"]
    #[inline(always)]
    pub const fn set_samp_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Entropy Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn ent_dly(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Entropy Delay"]
    #[inline(always)]
    pub const fn set_ent_dly(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sdctl {
    #[inline(always)]
    fn default() -> Sdctl {
        Sdctl(0)
    }
}
impl core::fmt::Debug for Sdctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sdctl")
            .field("samp_size", &self.samp_size())
            .field("ent_dly", &self.ent_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sdctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sdctl {{ samp_size: {=u16:?}, ent_dly: {=u16:?} }}",
            self.samp_size(),
            self.ent_dly()
        )
    }
}
#[doc = "Security Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SecCfg(pub u32);
impl SecCfg {
    #[doc = "If set, the TRNG registers cannot be programmed"]
    #[must_use]
    #[inline(always)]
    pub const fn no_prgm(&self) -> super::vals::NoPrgm {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::NoPrgm::from_bits(val as u8)
    }
    #[doc = "If set, the TRNG registers cannot be programmed"]
    #[inline(always)]
    pub const fn set_no_prgm(&mut self, val: super::vals::NoPrgm) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SecCfg {
    #[inline(always)]
    fn default() -> SecCfg {
        SecCfg(0)
    }
}
impl core::fmt::Debug for SecCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SecCfg")
            .field("no_prgm", &self.no_prgm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SecCfg {{ no_prgm: {:?} }}", self.no_prgm())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub const fn set_tf1br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf1br1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub const fn set_tf1br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub const fn set_tf2br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf2br1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub const fn set_tf2br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub const fn set_tf3br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf3br1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub const fn set_tf3br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf4br0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub const fn set_tf4br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf4br1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub const fn set_tf4br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf5br0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub const fn set_tf5br0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tf5br1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub const fn set_tf5br1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[must_use]
    #[inline(always)]
    pub const fn tf6pbr0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[inline(always)]
    pub const fn set_tf6pbr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[must_use]
    #[inline(always)]
    pub const fn tf6pbr1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[inline(always)]
    pub const fn set_tf6pbr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tfsb(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[inline(always)]
    pub const fn set_tfsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tflr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[inline(always)]
    pub const fn set_tflr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tfp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[inline(always)]
    pub const fn set_tfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[must_use]
    #[inline(always)]
    pub const fn tfmb(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[inline(always)]
    pub const fn set_tfmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "RETRY COUNT"]
    #[must_use]
    #[inline(always)]
    pub const fn retry_ct(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RETRY COUNT"]
    #[inline(always)]
    pub const fn set_retry_ct(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("tf1br0", &self.tf1br0())
            .field("tf1br1", &self.tf1br1())
            .field("tf2br0", &self.tf2br0())
            .field("tf2br1", &self.tf2br1())
            .field("tf3br0", &self.tf3br0())
            .field("tf3br1", &self.tf3br1())
            .field("tf4br0", &self.tf4br0())
            .field("tf4br1", &self.tf4br1())
            .field("tf5br0", &self.tf5br0())
            .field("tf5br1", &self.tf5br1())
            .field("tf6pbr0", &self.tf6pbr0())
            .field("tf6pbr1", &self.tf6pbr1())
            .field("tfsb", &self.tfsb())
            .field("tflr", &self.tflr())
            .field("tfp", &self.tfp())
            .field("tfmb", &self.tfmb())
            .field("retry_ct", &self.retry_ct())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ tf1br0: {=bool:?}, tf1br1: {=bool:?}, tf2br0: {=bool:?}, tf2br1: {=bool:?}, tf3br0: {=bool:?}, tf3br1: {=bool:?}, tf4br0: {=bool:?}, tf4br1: {=bool:?}, tf5br0: {=bool:?}, tf5br1: {=bool:?}, tf6pbr0: {=bool:?}, tf6pbr1: {=bool:?}, tfsb: {=bool:?}, tflr: {=bool:?}, tfp: {=bool:?}, tfmb: {=bool:?}, retry_ct: {=u8:?} }}",
            self.tf1br0(),
            self.tf1br1(),
            self.tf2br0(),
            self.tf2br1(),
            self.tf3br0(),
            self.tf3br1(),
            self.tf4br0(),
            self.tf4br1(),
            self.tf5br0(),
            self.tf5br1(),
            self.tf6pbr0(),
            self.tf6pbr1(),
            self.tfsb(),
            self.tflr(),
            self.tfp(),
            self.tfmb(),
            self.retry_ct()
        )
    }
}
#[doc = "Total Samples Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Totsam(pub u32);
impl Totsam {
    #[doc = "Total Samples"]
    #[must_use]
    #[inline(always)]
    pub const fn tot_sam(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Total Samples"]
    #[inline(always)]
    pub const fn set_tot_sam(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Totsam {
    #[inline(always)]
    fn default() -> Totsam {
        Totsam(0)
    }
}
impl core::fmt::Debug for Totsam {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Totsam")
            .field("tot_sam", &self.tot_sam())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Totsam {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Totsam {{ tot_sam: {=u32:?} }}", self.tot_sam())
    }
}
#[doc = "Version ID Register (MS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid1(pub u32);
impl Vid1 {
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn min_rev(&self) -> super::vals::MinRev {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::MinRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Minor revision of the TRNG."]
    #[inline(always)]
    pub const fn set_min_rev(&mut self, val: super::vals::MinRev) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's Major revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn maj_rev(&self) -> super::vals::MajRev {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::MajRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Major revision of the TRNG."]
    #[inline(always)]
    pub const fn set_maj_rev(&mut self, val: super::vals::MajRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the IP ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> super::vals::IpId {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::IpId::from_bits(val as u16)
    }
    #[doc = "Shows the IP ID."]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: super::vals::IpId) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Vid1 {
    #[inline(always)]
    fn default() -> Vid1 {
        Vid1(0)
    }
}
impl core::fmt::Debug for Vid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid1")
            .field("min_rev", &self.min_rev())
            .field("maj_rev", &self.maj_rev())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid1 {{ min_rev: {:?}, maj_rev: {:?}, ip_id: {:?} }}",
            self.min_rev(),
            self.maj_rev(),
            self.ip_id()
        )
    }
}
#[doc = "Version ID Register (LS)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vid2(pub u32);
impl Vid2 {
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn config_opt(&self) -> super::vals::ConfigOpt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::ConfigOpt::from_bits(val as u8)
    }
    #[doc = "Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub const fn set_config_opt(&mut self, val: super::vals::ConfigOpt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn eco_rev(&self) -> super::vals::EcoRev {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::EcoRev::from_bits(val as u8)
    }
    #[doc = "Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub const fn set_eco_rev(&mut self, val: super::vals::EcoRev) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn intg_opt(&self) -> super::vals::IntgOpt {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::IntgOpt::from_bits(val as u8)
    }
    #[doc = "Shows the integration options for the TRNG."]
    #[inline(always)]
    pub const fn set_intg_opt(&mut self, val: super::vals::IntgOpt) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Shows the compile options for the TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn era(&self) -> super::vals::Era {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Era::from_bits(val as u8)
    }
    #[doc = "Shows the compile options for the TRNG."]
    #[inline(always)]
    pub const fn set_era(&mut self, val: super::vals::Era) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Vid2 {
    #[inline(always)]
    fn default() -> Vid2 {
        Vid2(0)
    }
}
impl core::fmt::Debug for Vid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vid2")
            .field("config_opt", &self.config_opt())
            .field("eco_rev", &self.eco_rev())
            .field("intg_opt", &self.intg_opt())
            .field("era", &self.era())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vid2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Vid2 {{ config_opt: {:?}, eco_rev: {:?}, intg_opt: {:?}, era: {:?} }}",
            self.config_opt(),
            self.eco_rev(),
            self.intg_opt(),
            self.era()
        )
    }
}
