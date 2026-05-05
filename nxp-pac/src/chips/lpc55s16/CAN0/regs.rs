#[doc = "CC Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CCCR(pub u32);
impl CCCR {
    #[doc = "Initialization."]
    #[must_use]
    #[inline(always)]
    pub const fn INIT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization."]
    #[inline(always)]
    pub const fn set_INIT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Configuration change enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CCE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Configuration change enable."]
    #[inline(always)]
    pub const fn set_CCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Restricted operational mode."]
    #[must_use]
    #[inline(always)]
    pub const fn ASM(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Restricted operational mode."]
    #[inline(always)]
    pub const fn set_ASM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock Stop Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn CSA(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Acknowledge."]
    #[inline(always)]
    pub const fn set_CSA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Stop Request."]
    #[must_use]
    #[inline(always)]
    pub const fn CSR(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop Request."]
    #[inline(always)]
    pub const fn set_CSR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Bus monitoring mode."]
    #[must_use]
    #[inline(always)]
    pub const fn MON(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Bus monitoring mode."]
    #[inline(always)]
    pub const fn set_MON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable automatic retransmission."]
    #[must_use]
    #[inline(always)]
    pub const fn DAR(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Disable automatic retransmission."]
    #[inline(always)]
    pub const fn set_DAR(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Test mode enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEST(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Test mode enable."]
    #[inline(always)]
    pub const fn set_TEST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "CAN FD operation enable."]
    #[must_use]
    #[inline(always)]
    pub const fn FDOE(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "CAN FD operation enable."]
    #[inline(always)]
    pub const fn set_FDOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When CAN FD operation is disabled, this bit is not evaluated."]
    #[must_use]
    #[inline(always)]
    pub const fn BRSE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When CAN FD operation is disabled, this bit is not evaluated."]
    #[inline(always)]
    pub const fn set_BRSE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Protocol exception handling disable."]
    #[must_use]
    #[inline(always)]
    pub const fn PXHD(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol exception handling disable."]
    #[inline(always)]
    pub const fn set_PXHD(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Edge filtering during bus integration."]
    #[must_use]
    #[inline(always)]
    pub const fn EFBI(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Edge filtering during bus integration."]
    #[inline(always)]
    pub const fn set_EFBI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Transmit pause."]
    #[must_use]
    #[inline(always)]
    pub const fn TXP(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit pause."]
    #[inline(always)]
    pub const fn set_TXP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Non ISO operation."]
    #[must_use]
    #[inline(always)]
    pub const fn NISO(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Non ISO operation."]
    #[inline(always)]
    pub const fn set_NISO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for CCCR {
    #[inline(always)]
    fn default() -> CCCR {
        CCCR(0)
    }
}
impl core::fmt::Debug for CCCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCCR")
            .field("INIT", &self.INIT())
            .field("CCE", &self.CCE())
            .field("ASM", &self.ASM())
            .field("CSA", &self.CSA())
            .field("CSR", &self.CSR())
            .field("MON", &self.MON())
            .field("DAR", &self.DAR())
            .field("TEST", &self.TEST())
            .field("FDOE", &self.FDOE())
            .field("BRSE", &self.BRSE())
            .field("PXHD", &self.PXHD())
            .field("EFBI", &self.EFBI())
            .field("TXP", &self.TXP())
            .field("NISO", &self.NISO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CCCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CCCR {{ INIT: {=bool:?}, CCE: {=bool:?}, ASM: {=bool:?}, CSA: {=bool:?}, CSR: {=bool:?}, MON: {=bool:?}, DAR: {=bool:?}, TEST: {=bool:?}, FDOE: {=bool:?}, BRSE: {=bool:?}, PXHD: {=bool:?}, EFBI: {=bool:?}, TXP: {=bool:?}, NISO: {=bool:?} }}",
            self.INIT(),
            self.CCE(),
            self.ASM(),
            self.CSA(),
            self.CSR(),
            self.MON(),
            self.DAR(),
            self.TEST(),
            self.FDOE(),
            self.BRSE(),
            self.PXHD(),
            self.EFBI(),
            self.TXP(),
            self.NISO()
        )
    }
}
#[doc = "Data Bit Timing Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DBTP(pub u32);
impl DBTP {
    #[doc = "Data (re)synchronization jump width."]
    #[must_use]
    #[inline(always)]
    pub const fn DSJW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data (re)synchronization jump width."]
    #[inline(always)]
    pub const fn set_DSJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Data time segment after sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data time segment after sample point."]
    #[inline(always)]
    pub const fn set_DTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data time segment before sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn DTSEG1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Data time segment before sample point."]
    #[inline(always)]
    pub const fn set_DTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Data bit rate prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn DBRP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data bit rate prescaler."]
    #[inline(always)]
    pub const fn set_DBRP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Transmitter delay compensation."]
    #[must_use]
    #[inline(always)]
    pub const fn TDC(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmitter delay compensation."]
    #[inline(always)]
    pub const fn set_TDC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for DBTP {
    #[inline(always)]
    fn default() -> DBTP {
        DBTP(0)
    }
}
impl core::fmt::Debug for DBTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBTP")
            .field("DSJW", &self.DSJW())
            .field("DTSEG2", &self.DTSEG2())
            .field("DTSEG1", &self.DTSEG1())
            .field("DBRP", &self.DBRP())
            .field("TDC", &self.TDC())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DBTP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DBTP {{ DSJW: {=u8:?}, DTSEG2: {=u8:?}, DTSEG1: {=u8:?}, DBRP: {=u8:?}, TDC: {=bool:?} }}",
            self.DSJW(),
            self.DTSEG2(),
            self.DTSEG1(),
            self.DBRP(),
            self.TDC()
        )
    }
}
#[doc = "Error Counter Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ECR(pub u32);
impl ECR {
    #[doc = "Transmit error counter."]
    #[must_use]
    #[inline(always)]
    pub const fn TEC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit error counter."]
    #[inline(always)]
    pub const fn set_TEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Receive error counter."]
    #[must_use]
    #[inline(always)]
    pub const fn REC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive error counter."]
    #[inline(always)]
    pub const fn set_REC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Receive error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn RP(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Receive error passive."]
    #[inline(always)]
    pub const fn set_RP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN error logging."]
    #[must_use]
    #[inline(always)]
    pub const fn CEL(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "CAN error logging."]
    #[inline(always)]
    pub const fn set_CEL(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ECR {
    #[inline(always)]
    fn default() -> ECR {
        ECR(0)
    }
}
impl core::fmt::Debug for ECR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECR")
            .field("TEC", &self.TEC())
            .field("REC", &self.REC())
            .field("RP", &self.RP())
            .field("CEL", &self.CEL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ECR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ECR {{ TEC: {=u8:?}, REC: {=u8:?}, RP: {=bool:?}, CEL: {=u8:?} }}",
            self.TEC(),
            self.REC(),
            self.RP(),
            self.CEL()
        )
    }
}
#[doc = "External Timestamp Counter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ETSCC(pub u32);
impl ETSCC {
    #[doc = "External timestamp prescaler value."]
    #[must_use]
    #[inline(always)]
    pub const fn ETCP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "External timestamp prescaler value."]
    #[inline(always)]
    pub const fn set_ETCP(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "External timestamp counter enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ETCE(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "External timestamp counter enable."]
    #[inline(always)]
    pub const fn set_ETCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ETSCC {
    #[inline(always)]
    fn default() -> ETSCC {
        ETSCC(0)
    }
}
impl core::fmt::Debug for ETSCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETSCC")
            .field("ETCP", &self.ETCP())
            .field("ETCE", &self.ETCE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ETSCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ETSCC {{ ETCP: {=u16:?}, ETCE: {=bool:?} }}",
            self.ETCP(),
            self.ETCE()
        )
    }
}
#[doc = "External Timestamp Counter Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ETSCV(pub u32);
impl ETSCV {
    #[doc = "External timestamp counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ETSC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "External timestamp counter."]
    #[inline(always)]
    pub const fn set_ETSC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ETSCV {
    #[inline(always)]
    fn default() -> ETSCV {
        ETSCV(0)
    }
}
impl core::fmt::Debug for ETSCV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETSCV").field("ETSC", &self.ETSC()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ETSCV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ETSCV {{ ETSC: {=u16:?} }}", self.ETSC())
    }
}
#[doc = "Global Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GFC(pub u32);
impl GFC {
    #[doc = "Reject remote frames extended."]
    #[must_use]
    #[inline(always)]
    pub const fn RRFE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reject remote frames extended."]
    #[inline(always)]
    pub const fn set_RRFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reject remote frames standard."]
    #[must_use]
    #[inline(always)]
    pub const fn RRFS(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reject remote frames standard."]
    #[inline(always)]
    pub const fn set_RRFS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Accept non-matching frames extended."]
    #[must_use]
    #[inline(always)]
    pub const fn ANFE(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Accept non-matching frames extended."]
    #[inline(always)]
    pub const fn set_ANFE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Accept non-matching frames standard."]
    #[must_use]
    #[inline(always)]
    pub const fn ANFS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Accept non-matching frames standard."]
    #[inline(always)]
    pub const fn set_ANFS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for GFC {
    #[inline(always)]
    fn default() -> GFC {
        GFC(0)
    }
}
impl core::fmt::Debug for GFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GFC")
            .field("RRFE", &self.RRFE())
            .field("RRFS", &self.RRFS())
            .field("ANFE", &self.ANFE())
            .field("ANFS", &self.ANFS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GFC {{ RRFE: {=bool:?}, RRFS: {=bool:?}, ANFE: {=u8:?}, ANFS: {=u8:?} }}",
            self.RRFE(),
            self.RRFS(),
            self.ANFE(),
            self.ANFS()
        )
    }
}
#[doc = "High Priority Message Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HPMS(pub u32);
impl HPMS {
    #[doc = "Buffer index."]
    #[must_use]
    #[inline(always)]
    pub const fn BIDX(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Buffer index."]
    #[inline(always)]
    pub const fn set_BIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Message storage indicator."]
    #[must_use]
    #[inline(always)]
    pub const fn MSI(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Message storage indicator."]
    #[inline(always)]
    pub const fn set_MSI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Filter index."]
    #[must_use]
    #[inline(always)]
    pub const fn FIDX(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Filter index."]
    #[inline(always)]
    pub const fn set_FIDX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Filter list."]
    #[must_use]
    #[inline(always)]
    pub const fn FLST(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Filter list."]
    #[inline(always)]
    pub const fn set_FLST(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for HPMS {
    #[inline(always)]
    fn default() -> HPMS {
        HPMS(0)
    }
}
impl core::fmt::Debug for HPMS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPMS")
            .field("BIDX", &self.BIDX())
            .field("MSI", &self.MSI())
            .field("FIDX", &self.FIDX())
            .field("FLST", &self.FLST())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HPMS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HPMS {{ BIDX: {=u8:?}, MSI: {=u8:?}, FIDX: {=u8:?}, FLST: {=bool:?} }}",
            self.BIDX(),
            self.MSI(),
            self.FIDX(),
            self.FLST()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IE(pub u32);
impl IE {
    #[doc = "Rx FIFO 0 new message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0NE(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message interrupt enable."]
    #[inline(always)]
    pub const fn set_RF0NE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0WE(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_RF0WE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0FE(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full interrupt enable."]
    #[inline(always)]
    pub const fn set_RF0FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0LE(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost interrupt enable."]
    #[inline(always)]
    pub const fn set_RF0LE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1NE(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message interrupt enable."]
    #[inline(always)]
    pub const fn set_RF1NE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1WE(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_RF1WE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1FE(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full interrupt enable."]
    #[inline(always)]
    pub const fn set_RF1FE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1LE(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost interrupt enable."]
    #[inline(always)]
    pub const fn set_RF1LE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn HPME(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message interrupt enable."]
    #[inline(always)]
    pub const fn set_HPME(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TCE(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed interrupt enable."]
    #[inline(always)]
    pub const fn set_TCE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TCFE(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished interrupt enable."]
    #[inline(always)]
    pub const fn set_TCFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TFEE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty interrupt enable."]
    #[inline(always)]
    pub const fn set_TFEE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFNE(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry interrupt enable."]
    #[inline(always)]
    pub const fn set_TEFNE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFWE(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached interrupt enable."]
    #[inline(always)]
    pub const fn set_TEFWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFFE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full interrupt enable."]
    #[inline(always)]
    pub const fn set_TEFFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFLE(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost interrupt enable."]
    #[inline(always)]
    pub const fn set_TEFLE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TSWE(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound interrupt enable."]
    #[inline(always)]
    pub const fn set_TSWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn MRAFE(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure interrupt enable."]
    #[inline(always)]
    pub const fn set_MRAFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TOOE(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred interrupt enable."]
    #[inline(always)]
    pub const fn set_TOOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DRXE(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt enable."]
    #[inline(always)]
    pub const fn set_DRXE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BECE(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected interrupt enable."]
    #[inline(always)]
    pub const fn set_BECE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BEUE(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected interrupt enable."]
    #[inline(always)]
    pub const fn set_BEUE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ELOE(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow interrupt enable."]
    #[inline(always)]
    pub const fn set_ELOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EPE(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt enable."]
    #[inline(always)]
    pub const fn set_EPE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn EWE(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status interrupt enable."]
    #[inline(always)]
    pub const fn set_EWE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn BOE(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status interrupt enable."]
    #[inline(always)]
    pub const fn set_BOE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn WDIE(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt enable."]
    #[inline(always)]
    pub const fn set_WDIE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PEAE(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase interrupt enable."]
    #[inline(always)]
    pub const fn set_PEAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn PEDE(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase interrupt enable."]
    #[inline(always)]
    pub const fn set_PEDE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ARAE(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address interrupt enable."]
    #[inline(always)]
    pub const fn set_ARAE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for IE {
    #[inline(always)]
    fn default() -> IE {
        IE(0)
    }
}
impl core::fmt::Debug for IE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IE")
            .field("RF0NE", &self.RF0NE())
            .field("RF0WE", &self.RF0WE())
            .field("RF0FE", &self.RF0FE())
            .field("RF0LE", &self.RF0LE())
            .field("RF1NE", &self.RF1NE())
            .field("RF1WE", &self.RF1WE())
            .field("RF1FE", &self.RF1FE())
            .field("RF1LE", &self.RF1LE())
            .field("HPME", &self.HPME())
            .field("TCE", &self.TCE())
            .field("TCFE", &self.TCFE())
            .field("TFEE", &self.TFEE())
            .field("TEFNE", &self.TEFNE())
            .field("TEFWE", &self.TEFWE())
            .field("TEFFE", &self.TEFFE())
            .field("TEFLE", &self.TEFLE())
            .field("TSWE", &self.TSWE())
            .field("MRAFE", &self.MRAFE())
            .field("TOOE", &self.TOOE())
            .field("DRXE", &self.DRXE())
            .field("BECE", &self.BECE())
            .field("BEUE", &self.BEUE())
            .field("ELOE", &self.ELOE())
            .field("EPE", &self.EPE())
            .field("EWE", &self.EWE())
            .field("BOE", &self.BOE())
            .field("WDIE", &self.WDIE())
            .field("PEAE", &self.PEAE())
            .field("PEDE", &self.PEDE())
            .field("ARAE", &self.ARAE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IE {{ RF0NE: {=bool:?}, RF0WE: {=bool:?}, RF0FE: {=bool:?}, RF0LE: {=bool:?}, RF1NE: {=bool:?}, RF1WE: {=bool:?}, RF1FE: {=bool:?}, RF1LE: {=bool:?}, HPME: {=bool:?}, TCE: {=bool:?}, TCFE: {=bool:?}, TFEE: {=bool:?}, TEFNE: {=bool:?}, TEFWE: {=bool:?}, TEFFE: {=bool:?}, TEFLE: {=bool:?}, TSWE: {=bool:?}, MRAFE: {=bool:?}, TOOE: {=bool:?}, DRXE: {=bool:?}, BECE: {=bool:?}, BEUE: {=bool:?}, ELOE: {=bool:?}, EPE: {=bool:?}, EWE: {=bool:?}, BOE: {=bool:?}, WDIE: {=bool:?}, PEAE: {=bool:?}, PEDE: {=bool:?}, ARAE: {=bool:?} }}",
            self.RF0NE(),
            self.RF0WE(),
            self.RF0FE(),
            self.RF0LE(),
            self.RF1NE(),
            self.RF1WE(),
            self.RF1FE(),
            self.RF1LE(),
            self.HPME(),
            self.TCE(),
            self.TCFE(),
            self.TFEE(),
            self.TEFNE(),
            self.TEFWE(),
            self.TEFFE(),
            self.TEFLE(),
            self.TSWE(),
            self.MRAFE(),
            self.TOOE(),
            self.DRXE(),
            self.BECE(),
            self.BEUE(),
            self.ELOE(),
            self.EPE(),
            self.EWE(),
            self.BOE(),
            self.WDIE(),
            self.PEAE(),
            self.PEDE(),
            self.ARAE()
        )
    }
}
#[doc = "Interrupt Line Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ILE(pub u32);
impl ILE {
    #[doc = "Enable interrupt line 0."]
    #[must_use]
    #[inline(always)]
    pub const fn EINT0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable interrupt line 0."]
    #[inline(always)]
    pub const fn set_EINT0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable interrupt line 1."]
    #[must_use]
    #[inline(always)]
    pub const fn EINT1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable interrupt line 1."]
    #[inline(always)]
    pub const fn set_EINT1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for ILE {
    #[inline(always)]
    fn default() -> ILE {
        ILE(0)
    }
}
impl core::fmt::Debug for ILE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILE")
            .field("EINT0", &self.EINT0())
            .field("EINT1", &self.EINT1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ILE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ILE {{ EINT0: {=bool:?}, EINT1: {=bool:?} }}",
            self.EINT0(),
            self.EINT1()
        )
    }
}
#[doc = "Interrupt Line Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ILS(pub u32);
impl ILS {
    #[doc = "Rx FIFO 0 new message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0NL(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message interrupt line."]
    #[inline(always)]
    pub const fn set_RF0NL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0WL(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_RF0WL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0FL(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full interrupt line."]
    #[inline(always)]
    pub const fn set_RF0FL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0LL(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost interrupt line."]
    #[inline(always)]
    pub const fn set_RF0LL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1NL(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message interrupt line."]
    #[inline(always)]
    pub const fn set_RF1NL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1WL(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_RF1WL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1FL(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full interrupt line."]
    #[inline(always)]
    pub const fn set_RF1FL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1LL(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost interrupt line."]
    #[inline(always)]
    pub const fn set_RF1LL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn HPML(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message interrupt line."]
    #[inline(always)]
    pub const fn set_HPML(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TCL(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed interrupt line."]
    #[inline(always)]
    pub const fn set_TCL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TCFL(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished interrupt line."]
    #[inline(always)]
    pub const fn set_TCFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TFEL(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty interrupt line."]
    #[inline(always)]
    pub const fn set_TFEL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFNL(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry interrupt line."]
    #[inline(always)]
    pub const fn set_TEFNL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFWL(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached interrupt line."]
    #[inline(always)]
    pub const fn set_TEFWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFFL(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full interrupt line."]
    #[inline(always)]
    pub const fn set_TEFFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFLL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost interrupt line."]
    #[inline(always)]
    pub const fn set_TEFLL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TSWL(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound interrupt line."]
    #[inline(always)]
    pub const fn set_TSWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn MRAFL(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure interrupt line."]
    #[inline(always)]
    pub const fn set_MRAFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn TOOL(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred interrupt line."]
    #[inline(always)]
    pub const fn set_TOOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn DRXL(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer interrupt line."]
    #[inline(always)]
    pub const fn set_DRXL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn BECL(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected interrupt line."]
    #[inline(always)]
    pub const fn set_BECL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn BEUL(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected interrupt line."]
    #[inline(always)]
    pub const fn set_BEUL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn ELOL(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow interrupt line."]
    #[inline(always)]
    pub const fn set_ELOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn EPL(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive interrupt line."]
    #[inline(always)]
    pub const fn set_EPL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn EWL(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status interrupt line."]
    #[inline(always)]
    pub const fn set_EWL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn BOL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status interrupt line."]
    #[inline(always)]
    pub const fn set_BOL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn WDIL(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt line."]
    #[inline(always)]
    pub const fn set_WDIL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn PEAL(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase interrupt line."]
    #[inline(always)]
    pub const fn set_PEAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn PEDL(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase interrupt line."]
    #[inline(always)]
    pub const fn set_PEDL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn ARAL(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address interrupt line."]
    #[inline(always)]
    pub const fn set_ARAL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ILS {
    #[inline(always)]
    fn default() -> ILS {
        ILS(0)
    }
}
impl core::fmt::Debug for ILS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ILS")
            .field("RF0NL", &self.RF0NL())
            .field("RF0WL", &self.RF0WL())
            .field("RF0FL", &self.RF0FL())
            .field("RF0LL", &self.RF0LL())
            .field("RF1NL", &self.RF1NL())
            .field("RF1WL", &self.RF1WL())
            .field("RF1FL", &self.RF1FL())
            .field("RF1LL", &self.RF1LL())
            .field("HPML", &self.HPML())
            .field("TCL", &self.TCL())
            .field("TCFL", &self.TCFL())
            .field("TFEL", &self.TFEL())
            .field("TEFNL", &self.TEFNL())
            .field("TEFWL", &self.TEFWL())
            .field("TEFFL", &self.TEFFL())
            .field("TEFLL", &self.TEFLL())
            .field("TSWL", &self.TSWL())
            .field("MRAFL", &self.MRAFL())
            .field("TOOL", &self.TOOL())
            .field("DRXL", &self.DRXL())
            .field("BECL", &self.BECL())
            .field("BEUL", &self.BEUL())
            .field("ELOL", &self.ELOL())
            .field("EPL", &self.EPL())
            .field("EWL", &self.EWL())
            .field("BOL", &self.BOL())
            .field("WDIL", &self.WDIL())
            .field("PEAL", &self.PEAL())
            .field("PEDL", &self.PEDL())
            .field("ARAL", &self.ARAL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ILS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ILS {{ RF0NL: {=bool:?}, RF0WL: {=bool:?}, RF0FL: {=bool:?}, RF0LL: {=bool:?}, RF1NL: {=bool:?}, RF1WL: {=bool:?}, RF1FL: {=bool:?}, RF1LL: {=bool:?}, HPML: {=bool:?}, TCL: {=bool:?}, TCFL: {=bool:?}, TFEL: {=bool:?}, TEFNL: {=bool:?}, TEFWL: {=bool:?}, TEFFL: {=bool:?}, TEFLL: {=bool:?}, TSWL: {=bool:?}, MRAFL: {=bool:?}, TOOL: {=bool:?}, DRXL: {=bool:?}, BECL: {=bool:?}, BEUL: {=bool:?}, ELOL: {=bool:?}, EPL: {=bool:?}, EWL: {=bool:?}, BOL: {=bool:?}, WDIL: {=bool:?}, PEAL: {=bool:?}, PEDL: {=bool:?}, ARAL: {=bool:?} }}",
            self.RF0NL(),
            self.RF0WL(),
            self.RF0FL(),
            self.RF0LL(),
            self.RF1NL(),
            self.RF1WL(),
            self.RF1FL(),
            self.RF1LL(),
            self.HPML(),
            self.TCL(),
            self.TCFL(),
            self.TFEL(),
            self.TEFNL(),
            self.TEFWL(),
            self.TEFFL(),
            self.TEFLL(),
            self.TSWL(),
            self.MRAFL(),
            self.TOOL(),
            self.DRXL(),
            self.BECL(),
            self.BEUL(),
            self.ELOL(),
            self.EPL(),
            self.EWL(),
            self.BOL(),
            self.WDIL(),
            self.PEAL(),
            self.PEDL(),
            self.ARAL()
        )
    }
}
#[doc = "Interrupt Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IR(pub u32);
impl IR {
    #[doc = "Rx FIFO 0 new message."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0N(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 new message."]
    #[inline(always)]
    pub const fn set_RF0N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx FIFO 0 watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0W(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 watermark reached."]
    #[inline(always)]
    pub const fn set_RF0W(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Rx FIFO 0 full."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0F(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full."]
    #[inline(always)]
    pub const fn set_RF0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0L(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[inline(always)]
    pub const fn set_RF0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rx FIFO 1 new message."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1N(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 new message."]
    #[inline(always)]
    pub const fn set_RF1N(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Rx FIFO 1 watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1W(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 watermark reached."]
    #[inline(always)]
    pub const fn set_RF1W(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Rx FIFO 1 full."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1F(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full."]
    #[inline(always)]
    pub const fn set_RF1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1L(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[inline(always)]
    pub const fn set_RF1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "High priority message."]
    #[must_use]
    #[inline(always)]
    pub const fn HPM(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "High priority message."]
    #[inline(always)]
    pub const fn set_HPM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmission completed."]
    #[must_use]
    #[inline(always)]
    pub const fn TC(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission completed."]
    #[inline(always)]
    pub const fn set_TC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Transmission cancellation finished."]
    #[must_use]
    #[inline(always)]
    pub const fn TCF(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission cancellation finished."]
    #[inline(always)]
    pub const fn set_TCF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Tx FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn TFE(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO empty."]
    #[inline(always)]
    pub const fn set_TFE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Tx event FIFO new entry."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFN(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO new entry."]
    #[inline(always)]
    pub const fn set_TEFN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Tx event FIFO watermark reached."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFW(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO watermark reached."]
    #[inline(always)]
    pub const fn set_TEFW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Tx event FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFF(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO full."]
    #[inline(always)]
    pub const fn set_TEFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx event FIFO element lost."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFL(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost."]
    #[inline(always)]
    pub const fn set_TEFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timestamp wraparound."]
    #[must_use]
    #[inline(always)]
    pub const fn TSW(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp wraparound."]
    #[inline(always)]
    pub const fn set_TSW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Message RAM access failure."]
    #[must_use]
    #[inline(always)]
    pub const fn MRAF(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Message RAM access failure."]
    #[inline(always)]
    pub const fn set_MRAF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timeout occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn TOO(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout occurred."]
    #[inline(always)]
    pub const fn set_TOO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Message stored in dedicated Rx buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn DRX(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Message stored in dedicated Rx buffer."]
    #[inline(always)]
    pub const fn set_DRX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Bit error corrected."]
    #[must_use]
    #[inline(always)]
    pub const fn BEC(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error corrected."]
    #[inline(always)]
    pub const fn set_BEC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Bit error uncorrected."]
    #[must_use]
    #[inline(always)]
    pub const fn BEU(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Bit error uncorrected."]
    #[inline(always)]
    pub const fn set_BEU(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Error logging overflow."]
    #[must_use]
    #[inline(always)]
    pub const fn ELO(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Error logging overflow."]
    #[inline(always)]
    pub const fn set_ELO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Error passive."]
    #[must_use]
    #[inline(always)]
    pub const fn EP(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Error passive."]
    #[inline(always)]
    pub const fn set_EP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Warning status."]
    #[must_use]
    #[inline(always)]
    pub const fn EW(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status."]
    #[inline(always)]
    pub const fn set_EW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bus_Off Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BO(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bus_Off Status."]
    #[inline(always)]
    pub const fn set_BO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Watchdog interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn WDI(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog interrupt."]
    #[inline(always)]
    pub const fn set_WDI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Protocol error in arbitration phase."]
    #[must_use]
    #[inline(always)]
    pub const fn PEA(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in arbitration phase."]
    #[inline(always)]
    pub const fn set_PEA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Protocol error in data phase."]
    #[must_use]
    #[inline(always)]
    pub const fn PED(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol error in data phase."]
    #[inline(always)]
    pub const fn set_PED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Access to reserved address."]
    #[must_use]
    #[inline(always)]
    pub const fn ARA(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Access to reserved address."]
    #[inline(always)]
    pub const fn set_ARA(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for IR {
    #[inline(always)]
    fn default() -> IR {
        IR(0)
    }
}
impl core::fmt::Debug for IR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IR")
            .field("RF0N", &self.RF0N())
            .field("RF0W", &self.RF0W())
            .field("RF0F", &self.RF0F())
            .field("RF0L", &self.RF0L())
            .field("RF1N", &self.RF1N())
            .field("RF1W", &self.RF1W())
            .field("RF1F", &self.RF1F())
            .field("RF1L", &self.RF1L())
            .field("HPM", &self.HPM())
            .field("TC", &self.TC())
            .field("TCF", &self.TCF())
            .field("TFE", &self.TFE())
            .field("TEFN", &self.TEFN())
            .field("TEFW", &self.TEFW())
            .field("TEFF", &self.TEFF())
            .field("TEFL", &self.TEFL())
            .field("TSW", &self.TSW())
            .field("MRAF", &self.MRAF())
            .field("TOO", &self.TOO())
            .field("DRX", &self.DRX())
            .field("BEC", &self.BEC())
            .field("BEU", &self.BEU())
            .field("ELO", &self.ELO())
            .field("EP", &self.EP())
            .field("EW", &self.EW())
            .field("BO", &self.BO())
            .field("WDI", &self.WDI())
            .field("PEA", &self.PEA())
            .field("PED", &self.PED())
            .field("ARA", &self.ARA())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IR {{ RF0N: {=bool:?}, RF0W: {=bool:?}, RF0F: {=bool:?}, RF0L: {=bool:?}, RF1N: {=bool:?}, RF1W: {=bool:?}, RF1F: {=bool:?}, RF1L: {=bool:?}, HPM: {=bool:?}, TC: {=bool:?}, TCF: {=bool:?}, TFE: {=bool:?}, TEFN: {=bool:?}, TEFW: {=bool:?}, TEFF: {=bool:?}, TEFL: {=bool:?}, TSW: {=bool:?}, MRAF: {=bool:?}, TOO: {=bool:?}, DRX: {=bool:?}, BEC: {=bool:?}, BEU: {=bool:?}, ELO: {=bool:?}, EP: {=bool:?}, EW: {=bool:?}, BO: {=bool:?}, WDI: {=bool:?}, PEA: {=bool:?}, PED: {=bool:?}, ARA: {=bool:?} }}",
            self.RF0N(),
            self.RF0W(),
            self.RF0F(),
            self.RF0L(),
            self.RF1N(),
            self.RF1W(),
            self.RF1F(),
            self.RF1L(),
            self.HPM(),
            self.TC(),
            self.TCF(),
            self.TFE(),
            self.TEFN(),
            self.TEFW(),
            self.TEFF(),
            self.TEFL(),
            self.TSW(),
            self.MRAF(),
            self.TOO(),
            self.DRX(),
            self.BEC(),
            self.BEU(),
            self.ELO(),
            self.EP(),
            self.EW(),
            self.BO(),
            self.WDI(),
            self.PEA(),
            self.PED(),
            self.ARA()
        )
    }
}
#[doc = "CAN Message RAM Base Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MRBA(pub u32);
impl MRBA {
    #[doc = "Base address for the message RAM in the chip memory map."]
    #[must_use]
    #[inline(always)]
    pub const fn BA(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base address for the message RAM in the chip memory map."]
    #[inline(always)]
    pub const fn set_BA(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MRBA {
    #[inline(always)]
    fn default() -> MRBA {
        MRBA(0)
    }
}
impl core::fmt::Debug for MRBA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MRBA").field("BA", &self.BA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MRBA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MRBA {{ BA: {=u16:?} }}", self.BA())
    }
}
#[doc = "Nominal Bit Timing and Prescaler Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NBTP(pub u32);
impl NBTP {
    #[doc = "Nominal time segment after sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal time segment after sample point."]
    #[inline(always)]
    pub const fn set_NTSEG2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Nominal time segment before sample point."]
    #[must_use]
    #[inline(always)]
    pub const fn NTSEG1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Nominal time segment before sample point."]
    #[inline(always)]
    pub const fn set_NTSEG1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Nominal bit rate prescaler."]
    #[must_use]
    #[inline(always)]
    pub const fn NBRP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Nominal bit rate prescaler."]
    #[inline(always)]
    pub const fn set_NBRP(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Nominal (re)synchronization jump width."]
    #[must_use]
    #[inline(always)]
    pub const fn NSJW(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Nominal (re)synchronization jump width."]
    #[inline(always)]
    pub const fn set_NSJW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for NBTP {
    #[inline(always)]
    fn default() -> NBTP {
        NBTP(0)
    }
}
impl core::fmt::Debug for NBTP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NBTP")
            .field("NTSEG2", &self.NTSEG2())
            .field("NTSEG1", &self.NTSEG1())
            .field("NBRP", &self.NBRP())
            .field("NSJW", &self.NSJW())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NBTP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NBTP {{ NTSEG2: {=u8:?}, NTSEG1: {=u8:?}, NBRP: {=u16:?}, NSJW: {=u8:?} }}",
            self.NTSEG2(),
            self.NTSEG1(),
            self.NBRP(),
            self.NSJW()
        )
    }
}
#[doc = "New Data 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NDAT1(pub u32);
impl NDAT1 {
    #[doc = "New Data."]
    #[must_use]
    #[inline(always)]
    pub const fn ND(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data."]
    #[inline(always)]
    pub const fn set_ND(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NDAT1 {
    #[inline(always)]
    fn default() -> NDAT1 {
        NDAT1(0)
    }
}
impl core::fmt::Debug for NDAT1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NDAT1").field("ND", &self.ND()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NDAT1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NDAT1 {{ ND: {=u32:?} }}", self.ND())
    }
}
#[doc = "New Data 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NDAT2(pub u32);
impl NDAT2 {
    #[doc = "New Data."]
    #[must_use]
    #[inline(always)]
    pub const fn ND(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "New Data."]
    #[inline(always)]
    pub const fn set_ND(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NDAT2 {
    #[inline(always)]
    fn default() -> NDAT2 {
        NDAT2(0)
    }
}
impl core::fmt::Debug for NDAT2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NDAT2").field("ND", &self.ND()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NDAT2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NDAT2 {{ ND: {=u32:?} }}", self.ND())
    }
}
#[doc = "Protocol Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PSR(pub u32);
impl PSR {
    #[doc = "Last error code."]
    #[must_use]
    #[inline(always)]
    pub const fn LEC(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Last error code."]
    #[inline(always)]
    pub const fn set_LEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Activity."]
    #[must_use]
    #[inline(always)]
    pub const fn ACT(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Activity."]
    #[inline(always)]
    pub const fn set_ACT(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Error Passive."]
    #[must_use]
    #[inline(always)]
    pub const fn EP(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Error Passive."]
    #[inline(always)]
    pub const fn set_EP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Warning status."]
    #[must_use]
    #[inline(always)]
    pub const fn EW(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Warning status."]
    #[inline(always)]
    pub const fn set_EW(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Bus Off Status."]
    #[must_use]
    #[inline(always)]
    pub const fn BO(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Bus Off Status."]
    #[inline(always)]
    pub const fn set_BO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Data phase last error code."]
    #[must_use]
    #[inline(always)]
    pub const fn DLEC(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Data phase last error code."]
    #[inline(always)]
    pub const fn set_DLEC(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "ESI flag of the last received CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn RESI(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "ESI flag of the last received CAN FD message."]
    #[inline(always)]
    pub const fn set_RESI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "BRS flag of last received CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn RBRS(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "BRS flag of last received CAN FD message."]
    #[inline(always)]
    pub const fn set_RBRS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Received a CAN FD message."]
    #[must_use]
    #[inline(always)]
    pub const fn RFDF(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Received a CAN FD message."]
    #[inline(always)]
    pub const fn set_RFDF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Protocol exception event."]
    #[must_use]
    #[inline(always)]
    pub const fn PXE(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Protocol exception event."]
    #[inline(always)]
    pub const fn set_PXE(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Transmitter delay compensation value."]
    #[must_use]
    #[inline(always)]
    pub const fn TDCV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation value."]
    #[inline(always)]
    pub const fn set_TDCV(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
}
impl Default for PSR {
    #[inline(always)]
    fn default() -> PSR {
        PSR(0)
    }
}
impl core::fmt::Debug for PSR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("LEC", &self.LEC())
            .field("ACT", &self.ACT())
            .field("EP", &self.EP())
            .field("EW", &self.EW())
            .field("BO", &self.BO())
            .field("DLEC", &self.DLEC())
            .field("RESI", &self.RESI())
            .field("RBRS", &self.RBRS())
            .field("RFDF", &self.RFDF())
            .field("PXE", &self.PXE())
            .field("TDCV", &self.TDCV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PSR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PSR {{ LEC: {=u8:?}, ACT: {=u8:?}, EP: {=bool:?}, EW: {=bool:?}, BO: {=bool:?}, DLEC: {=u8:?}, RESI: {=bool:?}, RBRS: {=bool:?}, RFDF: {=bool:?}, PXE: {=bool:?}, TDCV: {=u8:?} }}",
            self.LEC(),
            self.ACT(),
            self.EP(),
            self.EW(),
            self.BO(),
            self.DLEC(),
            self.RESI(),
            self.RBRS(),
            self.RFDF(),
            self.PXE(),
            self.TDCV()
        )
    }
}
#[doc = "Rx Buffer Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXBC(pub u32);
impl RXBC {
    #[doc = "Rx buffer start address."]
    #[must_use]
    #[inline(always)]
    pub const fn RBSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx buffer start address."]
    #[inline(always)]
    pub const fn set_RBSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
}
impl Default for RXBC {
    #[inline(always)]
    fn default() -> RXBC {
        RXBC(0)
    }
}
impl core::fmt::Debug for RXBC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXBC").field("RBSA", &self.RBSA()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXBC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RXBC {{ RBSA: {=u16:?} }}", self.RBSA())
    }
}
#[doc = "Rx Buffer and FIFO Element Size Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXESC(pub u32);
impl RXESC {
    #[doc = "Rx FIFO 0 data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn F0DS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 0 data field size."]
    #[inline(always)]
    pub const fn set_F0DS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Rx FIFO 1 data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn F1DS(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Rx FIFO 1 data field size."]
    #[inline(always)]
    pub const fn set_F1DS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "."]
    #[must_use]
    #[inline(always)]
    pub const fn RBDS(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn set_RBDS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for RXESC {
    #[inline(always)]
    fn default() -> RXESC {
        RXESC(0)
    }
}
impl core::fmt::Debug for RXESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXESC")
            .field("F0DS", &self.F0DS())
            .field("F1DS", &self.F1DS())
            .field("RBDS", &self.RBDS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXESC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RXESC {{ F0DS: {=u8:?}, F1DS: {=u8:?}, RBDS: {=u8:?} }}",
            self.F0DS(),
            self.F1DS(),
            self.RBDS()
        )
    }
}
#[doc = "Rx FIFO 0 Acknowledge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF0A(pub u32);
impl RXF0A {
    #[doc = "Rx FIFO 0 acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn F0AI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 acknowledge index."]
    #[inline(always)]
    pub const fn set_F0AI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RXF0A {
    #[inline(always)]
    fn default() -> RXF0A {
        RXF0A(0)
    }
}
impl core::fmt::Debug for RXF0A {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0A").field("F0AI", &self.F0AI()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF0A {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RXF0A {{ F0AI: {=u8:?} }}", self.F0AI())
    }
}
#[doc = "Rx FIFO 0 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF0C(pub u32);
impl RXF0C {
    #[doc = "Rx FIFO 0 start address."]
    #[must_use]
    #[inline(always)]
    pub const fn F0SA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 0 start address."]
    #[inline(always)]
    pub const fn set_F0SA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 0 size."]
    #[must_use]
    #[inline(always)]
    pub const fn F0S(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 size."]
    #[inline(always)]
    pub const fn set_F0S(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 0 watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn F0WM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_F0WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 0 operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn F0OM(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 0 operation mode."]
    #[inline(always)]
    pub const fn set_F0OM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RXF0C {
    #[inline(always)]
    fn default() -> RXF0C {
        RXF0C(0)
    }
}
impl core::fmt::Debug for RXF0C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0C")
            .field("F0SA", &self.F0SA())
            .field("F0S", &self.F0S())
            .field("F0WM", &self.F0WM())
            .field("F0OM", &self.F0OM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF0C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RXF0C {{ F0SA: {=u16:?}, F0S: {=u8:?}, F0WM: {=u8:?}, F0OM: {=bool:?} }}",
            self.F0SA(),
            self.F0S(),
            self.F0WM(),
            self.F0OM()
        )
    }
}
#[doc = "Rx FIFO 0 Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF0S(pub u32);
impl RXF0S {
    #[doc = "Rx FIFO 0 fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn F0FL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 0 fill level."]
    #[inline(always)]
    pub const fn set_F0FL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 0 get index."]
    #[must_use]
    #[inline(always)]
    pub const fn F0GI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 get index."]
    #[inline(always)]
    pub const fn set_F0GI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 0 put index."]
    #[must_use]
    #[inline(always)]
    pub const fn F0PI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 0 put index."]
    #[inline(always)]
    pub const fn set_F0PI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 0 full."]
    #[must_use]
    #[inline(always)]
    pub const fn F0F(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 full."]
    #[inline(always)]
    pub const fn set_F0F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn RF0L(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 0 message lost."]
    #[inline(always)]
    pub const fn set_RF0L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for RXF0S {
    #[inline(always)]
    fn default() -> RXF0S {
        RXF0S(0)
    }
}
impl core::fmt::Debug for RXF0S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF0S")
            .field("F0FL", &self.F0FL())
            .field("F0GI", &self.F0GI())
            .field("F0PI", &self.F0PI())
            .field("F0F", &self.F0F())
            .field("RF0L", &self.RF0L())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF0S {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RXF0S {{ F0FL: {=u8:?}, F0GI: {=u8:?}, F0PI: {=u8:?}, F0F: {=bool:?}, RF0L: {=bool:?} }}",
            self.F0FL(),
            self.F0GI(),
            self.F0PI(),
            self.F0F(),
            self.RF0L()
        )
    }
}
#[doc = "Rx FIFO 1 Acknowledge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF1A(pub u32);
impl RXF1A {
    #[doc = "Rx FIFO 1 acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn F1AI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 acknowledge index."]
    #[inline(always)]
    pub const fn set_F1AI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RXF1A {
    #[inline(always)]
    fn default() -> RXF1A {
        RXF1A(0)
    }
}
impl core::fmt::Debug for RXF1A {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1A").field("F1AI", &self.F1AI()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF1A {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RXF1A {{ F1AI: {=u8:?} }}", self.F1AI())
    }
}
#[doc = "Rx FIFO 1 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF1C(pub u32);
impl RXF1C {
    #[doc = "Rx FIFO 1 start address."]
    #[must_use]
    #[inline(always)]
    pub const fn F1SA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Rx FIFO 1 start address."]
    #[inline(always)]
    pub const fn set_F1SA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[must_use]
    #[inline(always)]
    pub const fn F1S(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 size 0 = No Rx FIFO 1."]
    #[inline(always)]
    pub const fn set_F1S(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn F1WM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_F1WM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "FIFO 1 operation mode."]
    #[must_use]
    #[inline(always)]
    pub const fn F1OM(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 1 operation mode."]
    #[inline(always)]
    pub const fn set_F1OM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RXF1C {
    #[inline(always)]
    fn default() -> RXF1C {
        RXF1C(0)
    }
}
impl core::fmt::Debug for RXF1C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1C")
            .field("F1SA", &self.F1SA())
            .field("F1S", &self.F1S())
            .field("F1WM", &self.F1WM())
            .field("F1OM", &self.F1OM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF1C {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RXF1C {{ F1SA: {=u16:?}, F1S: {=u8:?}, F1WM: {=u8:?}, F1OM: {=bool:?} }}",
            self.F1SA(),
            self.F1S(),
            self.F1WM(),
            self.F1OM()
        )
    }
}
#[doc = "Rx FIFO 1 Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RXF1S(pub u32);
impl RXF1S {
    #[doc = "Rx FIFO 1 fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn F1FL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Rx FIFO 1 fill level."]
    #[inline(always)]
    pub const fn set_F1FL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Rx FIFO 1 get index."]
    #[must_use]
    #[inline(always)]
    pub const fn F1GI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 get index."]
    #[inline(always)]
    pub const fn set_F1GI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Rx FIFO 1 put index."]
    #[must_use]
    #[inline(always)]
    pub const fn F1PI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Rx FIFO 1 put index."]
    #[inline(always)]
    pub const fn set_F1PI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Rx FIFO 1 full."]
    #[must_use]
    #[inline(always)]
    pub const fn F1F(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 full."]
    #[inline(always)]
    pub const fn set_F1F(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[must_use]
    #[inline(always)]
    pub const fn RF1L(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Rx FIFO 1 message lost."]
    #[inline(always)]
    pub const fn set_RF1L(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for RXF1S {
    #[inline(always)]
    fn default() -> RXF1S {
        RXF1S(0)
    }
}
impl core::fmt::Debug for RXF1S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXF1S")
            .field("F1FL", &self.F1FL())
            .field("F1GI", &self.F1GI())
            .field("F1PI", &self.F1PI())
            .field("F1F", &self.F1F())
            .field("RF1L", &self.RF1L())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RXF1S {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RXF1S {{ F1FL: {=u8:?}, F1GI: {=u8:?}, F1PI: {=u8:?}, F1F: {=bool:?}, RF1L: {=bool:?} }}",
            self.F1FL(),
            self.F1GI(),
            self.F1PI(),
            self.F1F(),
            self.RF1L()
        )
    }
}
#[doc = "Standard ID Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SIDFC(pub u32);
impl SIDFC {
    #[doc = "Filter list standard start address."]
    #[must_use]
    #[inline(always)]
    pub const fn FLSSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter list standard start address."]
    #[inline(always)]
    pub const fn set_FLSSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List size standard 0 = No standard message ID filter."]
    #[must_use]
    #[inline(always)]
    pub const fn LSS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List size standard 0 = No standard message ID filter."]
    #[inline(always)]
    pub const fn set_LSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for SIDFC {
    #[inline(always)]
    fn default() -> SIDFC {
        SIDFC(0)
    }
}
impl core::fmt::Debug for SIDFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIDFC")
            .field("FLSSA", &self.FLSSA())
            .field("LSS", &self.LSS())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SIDFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SIDFC {{ FLSSA: {=u16:?}, LSS: {=u8:?} }}",
            self.FLSSA(),
            self.LSS()
        )
    }
}
#[doc = "Transmitter Delay Compensator Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TDCR(pub u32);
impl TDCR {
    #[doc = "Transmitter delay compensation filter window length."]
    #[must_use]
    #[inline(always)]
    pub const fn TDCF(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation filter window length."]
    #[inline(always)]
    pub const fn set_TDCF(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Transmitter delay compensation offset."]
    #[must_use]
    #[inline(always)]
    pub const fn TDCO(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmitter delay compensation offset."]
    #[inline(always)]
    pub const fn set_TDCO(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for TDCR {
    #[inline(always)]
    fn default() -> TDCR {
        TDCR(0)
    }
}
impl core::fmt::Debug for TDCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDCR")
            .field("TDCF", &self.TDCF())
            .field("TDCO", &self.TDCO())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TDCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TDCR {{ TDCF: {=u8:?}, TDCO: {=u8:?} }}",
            self.TDCF(),
            self.TDCO()
        )
    }
}
#[doc = "Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TEST(pub u32);
impl TEST {
    #[doc = "Loop back mode."]
    #[must_use]
    #[inline(always)]
    pub const fn LBCK(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Loop back mode."]
    #[inline(always)]
    pub const fn set_LBCK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control of transmit pin."]
    #[must_use]
    #[inline(always)]
    pub const fn TX(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Control of transmit pin."]
    #[inline(always)]
    pub const fn set_TX(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Monitors the actual value of the CAN_RXD."]
    #[must_use]
    #[inline(always)]
    pub const fn RX(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the actual value of the CAN_RXD."]
    #[inline(always)]
    pub const fn set_RX(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for TEST {
    #[inline(always)]
    fn default() -> TEST {
        TEST(0)
    }
}
impl core::fmt::Debug for TEST {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST")
            .field("LBCK", &self.LBCK())
            .field("TX", &self.TX())
            .field("RX", &self.RX())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TEST {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TEST {{ LBCK: {=bool:?}, TX: {=u8:?}, RX: {=bool:?} }}",
            self.LBCK(),
            self.TX(),
            self.RX()
        )
    }
}
#[doc = "Timeout Counter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TOCC(pub u32);
impl TOCC {
    #[doc = "Enable timeout counter."]
    #[must_use]
    #[inline(always)]
    pub const fn ETOC(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable timeout counter."]
    #[inline(always)]
    pub const fn set_ETOC(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timeout select."]
    #[must_use]
    #[inline(always)]
    pub const fn TOS(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "Timeout select."]
    #[inline(always)]
    pub const fn set_TOS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "Timeout period."]
    #[must_use]
    #[inline(always)]
    pub const fn TOP(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout period."]
    #[inline(always)]
    pub const fn set_TOP(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for TOCC {
    #[inline(always)]
    fn default() -> TOCC {
        TOCC(0)
    }
}
impl core::fmt::Debug for TOCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCC")
            .field("ETOC", &self.ETOC())
            .field("TOS", &self.TOS())
            .field("TOP", &self.TOP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TOCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TOCC {{ ETOC: {=bool:?}, TOS: {=u8:?}, TOP: {=u16:?} }}",
            self.ETOC(),
            self.TOS(),
            self.TOP()
        )
    }
}
#[doc = "Timeout Counter Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TOCV(pub u32);
impl TOCV {
    #[doc = "Timeout counter."]
    #[must_use]
    #[inline(always)]
    pub const fn TOC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timeout counter."]
    #[inline(always)]
    pub const fn set_TOC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TOCV {
    #[inline(always)]
    fn default() -> TOCV {
        TOCV(0)
    }
}
impl core::fmt::Debug for TOCV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCV").field("TOC", &self.TOC()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TOCV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TOCV {{ TOC: {=u16:?} }}", self.TOC())
    }
}
#[doc = "Timestamp Counter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TSCC(pub u32);
impl TSCC {
    #[doc = "Timestamp select."]
    #[must_use]
    #[inline(always)]
    pub const fn TSS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Timestamp select."]
    #[inline(always)]
    pub const fn set_TSS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times."]
    #[must_use]
    #[inline(always)]
    pub const fn TCP(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Timestamp counter prescaler Configures the timestamp and timeout counters time unit in multiple of CAN bit times."]
    #[inline(always)]
    pub const fn set_TCP(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for TSCC {
    #[inline(always)]
    fn default() -> TSCC {
        TSCC(0)
    }
}
impl core::fmt::Debug for TSCC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCC")
            .field("TSS", &self.TSS())
            .field("TCP", &self.TCP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TSCC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TSCC {{ TSS: {=u8:?}, TCP: {=u8:?} }}",
            self.TSS(),
            self.TCP()
        )
    }
}
#[doc = "Timestamp Counter Value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TSCV(pub u32);
impl TSCV {
    #[doc = "Timestamp counter."]
    #[must_use]
    #[inline(always)]
    pub const fn TSC(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timestamp counter."]
    #[inline(always)]
    pub const fn set_TSC(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for TSCV {
    #[inline(always)]
    fn default() -> TSCV {
        TSCV(0)
    }
}
impl core::fmt::Debug for TSCV {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCV").field("TSC", &self.TSC()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TSCV {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TSCV {{ TSC: {=u16:?} }}", self.TSC())
    }
}
#[doc = "Tx Buffer Add Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBAR(pub u32);
impl TXBAR {
    #[doc = "Add request."]
    #[must_use]
    #[inline(always)]
    pub const fn AR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Add request."]
    #[inline(always)]
    pub const fn set_AR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBAR {
    #[inline(always)]
    fn default() -> TXBAR {
        TXBAR(0)
    }
}
impl core::fmt::Debug for TXBAR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBAR").field("AR", &self.AR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBAR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBAR {{ AR: {=u32:?} }}", self.AR())
    }
}
#[doc = "Tx Buffer Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBC(pub u32);
impl TXBC {
    #[doc = "Tx buffers start address."]
    #[must_use]
    #[inline(always)]
    pub const fn TBSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Tx buffers start address."]
    #[inline(always)]
    pub const fn set_TBSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[must_use]
    #[inline(always)]
    pub const fn NDTB(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of dedicated transmit buffers 0 = No dedicated Tx buffers."]
    #[inline(always)]
    pub const fn set_NDTB(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[must_use]
    #[inline(always)]
    pub const fn TFQS(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Transmit FIFO/queue size 0 = No tx FIFO/Queue."]
    #[inline(always)]
    pub const fn set_TFQS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Tx FIFO/queue mode."]
    #[must_use]
    #[inline(always)]
    pub const fn TFQM(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/queue mode."]
    #[inline(always)]
    pub const fn set_TFQM(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for TXBC {
    #[inline(always)]
    fn default() -> TXBC {
        TXBC(0)
    }
}
impl core::fmt::Debug for TXBC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBC")
            .field("TBSA", &self.TBSA())
            .field("NDTB", &self.NDTB())
            .field("TFQS", &self.TFQS())
            .field("TFQM", &self.TFQM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TXBC {{ TBSA: {=u16:?}, NDTB: {=u8:?}, TFQS: {=u8:?}, TFQM: {=bool:?} }}",
            self.TBSA(),
            self.NDTB(),
            self.TFQS(),
            self.TFQM()
        )
    }
}
#[doc = "Tx Buffer Cancellation Finished."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBCF(pub u32);
impl TXBCF {
    #[doc = "Cancellation finished."]
    #[must_use]
    #[inline(always)]
    pub const fn TO(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation finished."]
    #[inline(always)]
    pub const fn set_TO(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBCF {
    #[inline(always)]
    fn default() -> TXBCF {
        TXBCF(0)
    }
}
impl core::fmt::Debug for TXBCF {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCF").field("TO", &self.TO()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBCF {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBCF {{ TO: {=u32:?} }}", self.TO())
    }
}
#[doc = "Tx Buffer Cancellation Finished Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBCIE(pub u32);
impl TXBCIE {
    #[doc = "Cancellation finished interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn CFIE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation finished interrupt enable."]
    #[inline(always)]
    pub const fn set_CFIE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBCIE {
    #[inline(always)]
    fn default() -> TXBCIE {
        TXBCIE(0)
    }
}
impl core::fmt::Debug for TXBCIE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCIE")
            .field("CFIE", &self.CFIE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBCIE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBCIE {{ CFIE: {=u32:?} }}", self.CFIE())
    }
}
#[doc = "Tx Buffer Cancellation Request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBCR(pub u32);
impl TXBCR {
    #[doc = "Cancellation request."]
    #[must_use]
    #[inline(always)]
    pub const fn CR(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cancellation request."]
    #[inline(always)]
    pub const fn set_CR(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBCR {
    #[inline(always)]
    fn default() -> TXBCR {
        TXBCR(0)
    }
}
impl core::fmt::Debug for TXBCR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBCR").field("CR", &self.CR()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBCR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBCR {{ CR: {=u32:?} }}", self.CR())
    }
}
#[doc = "Tx Buffer Request Pending."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBRP(pub u32);
impl TXBRP {
    #[doc = "Transmission request pending."]
    #[must_use]
    #[inline(always)]
    pub const fn TRP(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission request pending."]
    #[inline(always)]
    pub const fn set_TRP(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBRP {
    #[inline(always)]
    fn default() -> TXBRP {
        TXBRP(0)
    }
}
impl core::fmt::Debug for TXBRP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBRP").field("TRP", &self.TRP()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBRP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBRP {{ TRP: {=u32:?} }}", self.TRP())
    }
}
#[doc = "Tx Buffer Transmission Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBTIE(pub u32);
impl TXBTIE {
    #[doc = "Transmission interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn TIE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission interrupt enable."]
    #[inline(always)]
    pub const fn set_TIE(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBTIE {
    #[inline(always)]
    fn default() -> TXBTIE {
        TXBTIE(0)
    }
}
impl core::fmt::Debug for TXBTIE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTIE").field("TIE", &self.TIE()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBTIE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBTIE {{ TIE: {=u32:?} }}", self.TIE())
    }
}
#[doc = "Tx Buffer Transmission Occurred."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXBTO(pub u32);
impl TXBTO {
    #[doc = "Transmission occurred."]
    #[must_use]
    #[inline(always)]
    pub const fn TO(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmission occurred."]
    #[inline(always)]
    pub const fn set_TO(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TXBTO {
    #[inline(always)]
    fn default() -> TXBTO {
        TXBTO(0)
    }
}
impl core::fmt::Debug for TXBTO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXBTO").field("TO", &self.TO()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXBTO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXBTO {{ TO: {=u32:?} }}", self.TO())
    }
}
#[doc = "Tx Event FIFO Acknowledge."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXEFA(pub u32);
impl TXEFA {
    #[doc = "Event FIFO acknowledge index."]
    #[must_use]
    #[inline(always)]
    pub const fn EFAI(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO acknowledge index."]
    #[inline(always)]
    pub const fn set_EFAI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for TXEFA {
    #[inline(always)]
    fn default() -> TXEFA {
        TXEFA(0)
    }
}
impl core::fmt::Debug for TXEFA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFA").field("EFAI", &self.EFAI()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXEFA {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXEFA {{ EFAI: {=u8:?} }}", self.EFAI())
    }
}
#[doc = "Tx Event FIFO Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXEFC(pub u32);
impl TXEFC {
    #[doc = "Event FIFO start address."]
    #[must_use]
    #[inline(always)]
    pub const fn EFSA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Event FIFO start address."]
    #[inline(always)]
    pub const fn set_EFSA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "Event FIFO size 0 = Tx event FIFO disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn EFS(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO size 0 = Tx event FIFO disabled."]
    #[inline(always)]
    pub const fn set_EFS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[must_use]
    #[inline(always)]
    pub const fn EFWM(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO watermark 0 = Watermark interrupt disabled."]
    #[inline(always)]
    pub const fn set_EFWM(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for TXEFC {
    #[inline(always)]
    fn default() -> TXEFC {
        TXEFC(0)
    }
}
impl core::fmt::Debug for TXEFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFC")
            .field("EFSA", &self.EFSA())
            .field("EFS", &self.EFS())
            .field("EFWM", &self.EFWM())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXEFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TXEFC {{ EFSA: {=u16:?}, EFS: {=u8:?}, EFWM: {=u8:?} }}",
            self.EFSA(),
            self.EFS(),
            self.EFWM()
        )
    }
}
#[doc = "Tx Event FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXEFS(pub u32);
impl TXEFS {
    #[doc = "Event FIFO fill level."]
    #[must_use]
    #[inline(always)]
    pub const fn EFFL(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO fill level."]
    #[inline(always)]
    pub const fn set_EFFL(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Event FIFO get index."]
    #[must_use]
    #[inline(always)]
    pub const fn EFGI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Event FIFO get index."]
    #[inline(always)]
    pub const fn set_EFGI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Event FIFO put index."]
    #[must_use]
    #[inline(always)]
    pub const fn EFPI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Event FIFO put index."]
    #[inline(always)]
    pub const fn set_EFPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Event FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn EFF(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event FIFO full."]
    #[inline(always)]
    pub const fn set_EFF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Tx event FIFO element lost."]
    #[must_use]
    #[inline(always)]
    pub const fn TEFL(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Tx event FIFO element lost."]
    #[inline(always)]
    pub const fn set_TEFL(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for TXEFS {
    #[inline(always)]
    fn default() -> TXEFS {
        TXEFS(0)
    }
}
impl core::fmt::Debug for TXEFS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXEFS")
            .field("EFFL", &self.EFFL())
            .field("EFGI", &self.EFGI())
            .field("EFPI", &self.EFPI())
            .field("EFF", &self.EFF())
            .field("TEFL", &self.TEFL())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXEFS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TXEFS {{ EFFL: {=u8:?}, EFGI: {=u8:?}, EFPI: {=u8:?}, EFF: {=bool:?}, TEFL: {=bool:?} }}",
            self.EFFL(),
            self.EFGI(),
            self.EFPI(),
            self.EFF(),
            self.TEFL()
        )
    }
}
#[doc = "Tx Buffer Element Size Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXESC(pub u32);
impl TXESC {
    #[doc = "Tx buffer data field size."]
    #[must_use]
    #[inline(always)]
    pub const fn TBDS(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tx buffer data field size."]
    #[inline(always)]
    pub const fn set_TBDS(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for TXESC {
    #[inline(always)]
    fn default() -> TXESC {
        TXESC(0)
    }
}
impl core::fmt::Debug for TXESC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXESC").field("TBDS", &self.TBDS()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXESC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TXESC {{ TBDS: {=u8:?} }}", self.TBDS())
    }
}
#[doc = "Tx FIFO/Queue Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TXFQS(pub u32);
impl TXFQS {
    #[doc = "Tx FIFO get index."]
    #[must_use]
    #[inline(always)]
    pub const fn TFGI(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO get index."]
    #[inline(always)]
    pub const fn set_TFGI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Tx FIFO/queue put index."]
    #[must_use]
    #[inline(always)]
    pub const fn TFQPI(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Tx FIFO/queue put index."]
    #[inline(always)]
    pub const fn set_TFQPI(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Tx FIFO/queue full."]
    #[must_use]
    #[inline(always)]
    pub const fn TFQF(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx FIFO/queue full."]
    #[inline(always)]
    pub const fn set_TFQF(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for TXFQS {
    #[inline(always)]
    fn default() -> TXFQS {
        TXFQS(0)
    }
}
impl core::fmt::Debug for TXFQS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXFQS")
            .field("TFGI", &self.TFGI())
            .field("TFQPI", &self.TFQPI())
            .field("TFQF", &self.TFQF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TXFQS {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TXFQS {{ TFGI: {=u8:?}, TFQPI: {=u8:?}, TFQF: {=bool:?} }}",
            self.TFGI(),
            self.TFQPI(),
            self.TFQF()
        )
    }
}
#[doc = "Extended ID AND Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XIDAM(pub u32);
impl XIDAM {
    #[doc = "Extended ID mask."]
    #[must_use]
    #[inline(always)]
    pub const fn EIDM(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Extended ID mask."]
    #[inline(always)]
    pub const fn set_EIDM(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for XIDAM {
    #[inline(always)]
    fn default() -> XIDAM {
        XIDAM(0)
    }
}
impl core::fmt::Debug for XIDAM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIDAM").field("EIDM", &self.EIDM()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XIDAM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "XIDAM {{ EIDM: {=u32:?} }}", self.EIDM())
    }
}
#[doc = "Extended ID Filter Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XIDFC(pub u32);
impl XIDFC {
    #[doc = "Filter list extended start address."]
    #[must_use]
    #[inline(always)]
    pub const fn FLESA(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x3fff;
        val as u16
    }
    #[doc = "Filter list extended start address."]
    #[inline(always)]
    pub const fn set_FLESA(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
    }
    #[doc = "List size extended 0 = No extended message ID filter."]
    #[must_use]
    #[inline(always)]
    pub const fn LSE(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "List size extended 0 = No extended message ID filter."]
    #[inline(always)]
    pub const fn set_LSE(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for XIDFC {
    #[inline(always)]
    fn default() -> XIDFC {
        XIDFC(0)
    }
}
impl core::fmt::Debug for XIDFC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XIDFC")
            .field("FLESA", &self.FLESA())
            .field("LSE", &self.LSE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XIDFC {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "XIDFC {{ FLESA: {=u16:?}, LSE: {=u8:?} }}",
            self.FLESA(),
            self.LSE()
        )
    }
}
