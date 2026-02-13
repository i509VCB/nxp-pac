#[doc = "OTP clock divider register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpClkDiv(pub u32);
impl OtpClkDiv {
    #[doc = "Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> super::vals::Div {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Div::from_bits(val as u8)
    }
    #[doc = "Clock divider value by -1 encoding. It's used to generate the clock to OTP memory (otp_clk) with apb_clk. The maximum otp_clk frequency is 120Mhz. 0: Divide by 1"]
    #[inline(always)]
    pub const fn set_div(&mut self, val: super::vals::Div) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for OtpClkDiv {
    #[inline(always)]
    fn default() -> OtpClkDiv {
        OtpClkDiv(0)
    }
}
impl core::fmt::Debug for OtpClkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpClkDiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("reqflag", &self.reqflag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpClkDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpClkDiv {{ div: {:?}, reset: {=bool:?}, halt: {=bool:?}, reqflag: {=bool:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.reqflag()
        )
    }
}
#[doc = "CRC address range register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpCrcAddr(pub u32);
impl OtpCrcAddr {
    #[doc = "CRC starting fuse word address"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_start_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "CRC starting fuse word address"]
    #[inline(always)]
    pub const fn set_crc_start_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "CRC ending fuse word address"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_end_addr(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x01ff;
        val as u16
    }
    #[doc = "CRC ending fuse word address"]
    #[inline(always)]
    pub const fn set_crc_end_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12usize)) | (((val as u32) & 0x01ff) << 12usize);
    }
    #[doc = "Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_ref_addr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Specify which of the 8 CRC reference value to use for CRC calculation. When the CRC result for the fuse data from CRC_START_ADDR to CRC_AND_ADDR and this CRC reference value is 0, the CRC check passes."]
    #[inline(always)]
    pub const fn set_crc_ref_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for OtpCrcAddr {
    #[inline(always)]
    fn default() -> OtpCrcAddr {
        OtpCrcAddr(0)
    }
}
impl core::fmt::Debug for OtpCrcAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpCrcAddr")
            .field("crc_start_addr", &self.crc_start_addr())
            .field("crc_end_addr", &self.crc_end_addr())
            .field("crc_ref_addr", &self.crc_ref_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpCrcAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpCrcAddr {{ crc_start_addr: {=u16:?}, crc_end_addr: {=u16:?}, crc_ref_addr: {=u8:?} }}",
            self.crc_start_addr(),
            self.crc_end_addr(),
            self.crc_ref_addr()
        )
    }
}
#[doc = "CRC result register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpCrcValue(pub u32);
impl OtpCrcValue {
    #[doc = "The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "The CRC result value. When it is locked, reading from it returns value 32hBADA_BADA"]
    #[inline(always)]
    pub const fn set_crc_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OtpCrcValue {
    #[inline(always)]
    fn default() -> OtpCrcValue {
        OtpCrcValue(0)
    }
}
impl core::fmt::Debug for OtpCrcValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpCrcValue")
            .field("crc_value", &self.crc_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpCrcValue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtpCrcValue {{ crc_value: {=u32:?} }}", self.crc_value())
    }
}
#[doc = "Control/address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpCtrl(pub u32);
impl OtpCtrl {
    #[doc = "OTP word address for read/programming"]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "OTP word address for read/programming"]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
    #[must_use]
    #[inline(always)]
    pub const fn reload_shadows(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set to force re-loading the shadow registers (HW/SW capability and LOCK). This operation will automatically set OTP_STATUS.BUSY. Once the shadow registers have been re-loaded, OTP_STATUS.BUSY and RELOAD_SHADOWS are automatically cleared by the controller"]
    #[inline(always)]
    pub const fn set_reload_shadows(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
    #[must_use]
    #[inline(always)]
    pub const fn crc_test(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Set to start CRC calculation. This operation will automatically set OTP_STATUS.BUSY. Once CRC is calculation done, OTP_STATUS.BUSY and CRC_TEST are automatically cleared by the controller"]
    #[inline(always)]
    pub const fn set_crc_test(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
    #[must_use]
    #[inline(always)]
    pub const fn wordlock(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set to write-lock the fuse word when it's being programming. When programming with ECC mode, it recommends to set this bit."]
    #[inline(always)]
    pub const fn set_wordlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_unlock(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write 0x3E77 to enable OTP write accesses. NOTE: The write operation must be unlocked for each word by writing 0x3E77 to WR_UNLOCK field. Then writing to OTP_WRITE_DATA register will automatically start the programming procedure."]
    #[inline(always)]
    pub const fn set_wr_unlock(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for OtpCtrl {
    #[inline(always)]
    fn default() -> OtpCtrl {
        OtpCtrl(0)
    }
}
impl core::fmt::Debug for OtpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpCtrl")
            .field("addr", &self.addr())
            .field("reload_shadows", &self.reload_shadows())
            .field("crc_test", &self.crc_test())
            .field("wordlock", &self.wordlock())
            .field("wr_unlock", &self.wr_unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpCtrl {{ addr: {=u16:?}, reload_shadows: {=bool:?}, crc_test: {=bool:?}, wordlock: {=bool:?}, wr_unlock: {=u16:?} }}",
            self.addr(),
            self.reload_shadows(),
            self.crc_test(),
            self.wordlock(),
            self.wr_unlock()
        )
    }
}
#[doc = "Power-down register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpPdn(pub u32);
impl OtpPdn {
    #[doc = "This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
    #[must_use]
    #[inline(always)]
    pub const fn pdn(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates the PDN value of OTP memory. Writing 1 to the bit to clear PDN. Writing 0 has no effect. Note: Software need to write 1 to this bit to shut off power of OTP memory after system power up. At the beginning of every fuse operation, the controller will automatically turn-on power to the OPT memory. After every fuse operation, software also need to write 1 to this bit to shut off power to the OTP memory to reduce power consumption."]
    #[inline(always)]
    pub const fn set_pdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for OtpPdn {
    #[inline(always)]
    fn default() -> OtpPdn {
        OtpPdn(0)
    }
}
impl core::fmt::Debug for OtpPdn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpPdn").field("pdn", &self.pdn()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpPdn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtpPdn {{ pdn: {=bool:?} }}", self.pdn())
    }
}
#[doc = "OTP read start register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpReadCtrl(pub u32);
impl OtpReadCtrl {
    #[doc = "no description available"]
    #[must_use]
    #[inline(always)]
    pub const fn read(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn set_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for OtpReadCtrl {
    #[inline(always)]
    fn default() -> OtpReadCtrl {
        OtpReadCtrl(0)
    }
}
impl core::fmt::Debug for OtpReadCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpReadCtrl")
            .field("read", &self.read())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpReadCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtpReadCtrl {{ read: {=bool:?} }}", self.read())
    }
}
#[doc = "OTP read data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpReadData(pub u32);
impl OtpReadData {
    #[doc = "Fuse word read data from read operation"]
    #[must_use]
    #[inline(always)]
    pub const fn read_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fuse word read data from read operation"]
    #[inline(always)]
    pub const fn set_read_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OtpReadData {
    #[inline(always)]
    fn default() -> OtpReadData {
        OtpReadData(0)
    }
}
impl core::fmt::Debug for OtpReadData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpReadData")
            .field("read_data", &self.read_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpReadData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtpReadData {{ read_data: {=u32:?} }}", self.read_data())
    }
}
#[doc = "OTP shadow register N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpShadow(pub u32);
impl OtpShadow {
    #[doc = "OTP shadow register"]
    #[must_use]
    #[inline(always)]
    pub const fn shadow(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "OTP shadow register"]
    #[inline(always)]
    pub const fn set_shadow(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OtpShadow {
    #[inline(always)]
    fn default() -> OtpShadow {
        OtpShadow(0)
    }
}
impl core::fmt::Debug for OtpShadow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpShadow")
            .field("shadow", &self.shadow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpShadow {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OtpShadow {{ shadow: {=u32:?} }}", self.shadow())
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpStatus(pub u32);
impl OtpStatus {
    #[doc = "OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn sec(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "OTP Single Error Corrected status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ded(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "OTP Double Error Detection status of ECC during read operation. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_ded(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn locked(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "OTP LOCKED status during read/write operation. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_locked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "OTP PROGFAIL status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn progfail(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "OTP PROGFAIL status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_progfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "OTP ACK value"]
    #[must_use]
    #[inline(always)]
    pub const fn ack(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "OTP ACK value"]
    #[inline(always)]
    pub const fn set_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "OTP Power OK status. Indicate that power VDD are in the operating range."]
    #[must_use]
    #[inline(always)]
    pub const fn pwok(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "OTP Power OK status. Indicate that power VDD are in the operating range."]
    #[inline(always)]
    pub const fn set_pwok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_reload(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "OTP Single Error Corrected status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_sec_reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn ded_reload(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "OTP Double Error Detect status of ECC during reload process. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_ded_reload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "OTP controller status bit. When active, no new write or read access to OTP (including RELOAD_SHADOWS) can be performed. Cleared by the controller when the access completes. After reset (or after setting RELOAD_SHADOWS), this bit is set by the controller and cleared after all the shadow registers are successfully loaded."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "OTP controller status bit. When active, no new write or read access to OTP (including RELOAD_SHADOWS) can be performed. Cleared by the controller when the access completes. After reset (or after setting RELOAD_SHADOWS), this bit is set by the controller and cleared after all the shadow registers are successfully loaded."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set by the controller when a read/write access to a locked region (OTP or shadow register) is requested. Writing 1 to clear it before any further access can be performed. This bit can only be set by the controller."]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_fail(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "CRC failed when set by hardware for CRC operation. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_crc_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fuse_latched(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Indicate all shadows registers have been loaded with their corresponding fuse words when set by the controller after reset."]
    #[inline(always)]
    pub const fn set_fuse_latched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for OtpStatus {
    #[inline(always)]
    fn default() -> OtpStatus {
        OtpStatus(0)
    }
}
impl core::fmt::Debug for OtpStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpStatus")
            .field("sec", &self.sec())
            .field("ded", &self.ded())
            .field("locked", &self.locked())
            .field("progfail", &self.progfail())
            .field("ack", &self.ack())
            .field("pwok", &self.pwok())
            .field("sec_reload", &self.sec_reload())
            .field("ded_reload", &self.ded_reload())
            .field("busy", &self.busy())
            .field("error", &self.error())
            .field("crc_fail", &self.crc_fail())
            .field("fuse_latched", &self.fuse_latched())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpStatus {{ sec: {=bool:?}, ded: {=bool:?}, locked: {=bool:?}, progfail: {=bool:?}, ack: {=bool:?}, pwok: {=bool:?}, sec_reload: {=bool:?}, ded_reload: {=bool:?}, busy: {=bool:?}, error: {=bool:?}, crc_fail: {=bool:?}, fuse_latched: {=bool:?} }}",
            self.sec(),
            self.ded(),
            self.locked(),
            self.progfail(),
            self.ack(),
            self.pwok(),
            self.sec_reload(),
            self.ded_reload(),
            self.busy(),
            self.error(),
            self.crc_fail(),
            self.fuse_latched()
        )
    }
}
#[doc = "VERSION ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpVersion(pub u32);
impl OtpVersion {
    #[doc = "OTP controller step version"]
    #[must_use]
    #[inline(always)]
    pub const fn step_ver(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "OTP controller step version"]
    #[inline(always)]
    pub const fn set_step_ver(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "OTP controller minor version"]
    #[must_use]
    #[inline(always)]
    pub const fn minor_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OTP controller minor version"]
    #[inline(always)]
    pub const fn set_minor_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "OTP controller major version"]
    #[must_use]
    #[inline(always)]
    pub const fn major_ver(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "OTP controller major version"]
    #[inline(always)]
    pub const fn set_major_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for OtpVersion {
    #[inline(always)]
    fn default() -> OtpVersion {
        OtpVersion(0)
    }
}
impl core::fmt::Debug for OtpVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpVersion")
            .field("step_ver", &self.step_ver())
            .field("minor_ver", &self.minor_ver())
            .field("major_ver", &self.major_ver())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpVersion {{ step_ver: {=u16:?}, minor_ver: {=u8:?}, major_ver: {=u8:?} }}",
            self.step_ver(),
            self.minor_ver(),
            self.major_ver()
        )
    }
}
#[doc = "OTP programming data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpWriteData(pub u32);
impl OtpWriteData {
    #[doc = "Fuse word programming data. After the write operation is unlocked in OTP_CTRL register, writing data to this register automatically start the programming procedure."]
    #[must_use]
    #[inline(always)]
    pub const fn write_data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fuse word programming data. After the write operation is unlocked in OTP_CTRL register, writing data to this register automatically start the programming procedure."]
    #[inline(always)]
    pub const fn set_write_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for OtpWriteData {
    #[inline(always)]
    fn default() -> OtpWriteData {
        OtpWriteData(0)
    }
}
impl core::fmt::Debug for OtpWriteData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OtpWriteData")
            .field("write_data", &self.write_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OtpWriteData {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "OtpWriteData {{ write_data: {=u32:?} }}",
            self.write_data()
        )
    }
}
