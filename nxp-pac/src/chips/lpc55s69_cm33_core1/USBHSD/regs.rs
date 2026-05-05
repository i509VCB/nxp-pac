#[doc = "USB Data buffer start address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DATABUFSTART(pub u32);
impl DATABUFSTART {
    #[doc = "Start address of the memory page where all endpoint data buffers are located."]
    #[must_use]
    #[inline(always)]
    pub const fn DA_BUF(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub const fn set_DA_BUF(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DATABUFSTART {
    #[inline(always)]
    fn default() -> DATABUFSTART {
        DATABUFSTART(0)
    }
}
impl core::fmt::Debug for DATABUFSTART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATABUFSTART")
            .field("DA_BUF", &self.DA_BUF())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DATABUFSTART {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DATABUFSTART {{ DA_BUF: {=u32:?} }}", self.DA_BUF())
    }
}
#[doc = "USB Device Command/Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DEVCMDSTAT(pub u32);
impl DEVCMDSTAT {
    #[doc = "USB device address."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_ADDR(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn set_DEV_ADDR(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "USB device enable."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_EN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB device enable."]
    #[inline(always)]
    pub const fn set_DEV_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SETUP token received."]
    #[must_use]
    #[inline(always)]
    pub const fn SETUP(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP token received."]
    #[inline(always)]
    pub const fn set_SETUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[must_use]
    #[inline(always)]
    pub const fn FORCE_NEEDCLK(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub const fn set_FORCE_NEEDCLK(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPM Supported:."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_SUP(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Supported:."]
    #[inline(always)]
    pub const fn set_LPM_SUP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn INTONNAK_AO(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub const fn set_INTONNAK_AO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn INTONNAK_AI(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub const fn set_INTONNAK_AI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn INTONNAK_CO(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub const fn set_INTONNAK_CO(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn INTONNAK_CI(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub const fn set_INTONNAK_CI(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Device status - connect."]
    #[must_use]
    #[inline(always)]
    pub const fn DCON(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect."]
    #[inline(always)]
    pub const fn set_DCON(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device status - suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn DSUS(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend."]
    #[inline(always)]
    pub const fn set_DSUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Device status - LPM Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_SUS(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - LPM Suspend."]
    #[inline(always)]
    pub const fn set_LPM_SUS(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[must_use]
    #[inline(always)]
    pub const fn LPM_REWP(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub const fn set_LPM_REWP(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[must_use]
    #[inline(always)]
    pub const fn Speed(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub const fn set_Speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Device status - connect change."]
    #[must_use]
    #[inline(always)]
    pub const fn DCON_C(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect change."]
    #[inline(always)]
    pub const fn set_DCON_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device status - suspend change."]
    #[must_use]
    #[inline(always)]
    pub const fn DSUS_C(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend change."]
    #[inline(always)]
    pub const fn set_DSUS_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Device status - reset change."]
    #[must_use]
    #[inline(always)]
    pub const fn DRES_C(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - reset change."]
    #[inline(always)]
    pub const fn set_DRES_C(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[must_use]
    #[inline(always)]
    pub const fn VBUS_DEBOUNCED(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub const fn set_VBUS_DEBOUNCED(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[must_use]
    #[inline(always)]
    pub const fn PHY_TEST_MODE(&self) -> super::vals::PHY_TEST_MODE {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::PHY_TEST_MODE::from_bits(val as u8)
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub const fn set_PHY_TEST_MODE(&mut self, val: super::vals::PHY_TEST_MODE) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for DEVCMDSTAT {
    #[inline(always)]
    fn default() -> DEVCMDSTAT {
        DEVCMDSTAT(0)
    }
}
impl core::fmt::Debug for DEVCMDSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVCMDSTAT")
            .field("DEV_ADDR", &self.DEV_ADDR())
            .field("DEV_EN", &self.DEV_EN())
            .field("SETUP", &self.SETUP())
            .field("FORCE_NEEDCLK", &self.FORCE_NEEDCLK())
            .field("LPM_SUP", &self.LPM_SUP())
            .field("INTONNAK_AO", &self.INTONNAK_AO())
            .field("INTONNAK_AI", &self.INTONNAK_AI())
            .field("INTONNAK_CO", &self.INTONNAK_CO())
            .field("INTONNAK_CI", &self.INTONNAK_CI())
            .field("DCON", &self.DCON())
            .field("DSUS", &self.DSUS())
            .field("LPM_SUS", &self.LPM_SUS())
            .field("LPM_REWP", &self.LPM_REWP())
            .field("Speed", &self.Speed())
            .field("DCON_C", &self.DCON_C())
            .field("DSUS_C", &self.DSUS_C())
            .field("DRES_C", &self.DRES_C())
            .field("VBUS_DEBOUNCED", &self.VBUS_DEBOUNCED())
            .field("PHY_TEST_MODE", &self.PHY_TEST_MODE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DEVCMDSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DEVCMDSTAT {{ DEV_ADDR: {=u8:?}, DEV_EN: {=bool:?}, SETUP: {=bool:?}, FORCE_NEEDCLK: {=bool:?}, LPM_SUP: {=bool:?}, INTONNAK_AO: {=bool:?}, INTONNAK_AI: {=bool:?}, INTONNAK_CO: {=bool:?}, INTONNAK_CI: {=bool:?}, DCON: {=bool:?}, DSUS: {=bool:?}, LPM_SUS: {=bool:?}, LPM_REWP: {=bool:?}, Speed: {=u8:?}, DCON_C: {=bool:?}, DSUS_C: {=bool:?}, DRES_C: {=bool:?}, VBUS_DEBOUNCED: {=bool:?}, PHY_TEST_MODE: {:?} }}",
            self.DEV_ADDR(),
            self.DEV_EN(),
            self.SETUP(),
            self.FORCE_NEEDCLK(),
            self.LPM_SUP(),
            self.INTONNAK_AO(),
            self.INTONNAK_AI(),
            self.INTONNAK_CO(),
            self.INTONNAK_CI(),
            self.DCON(),
            self.DSUS(),
            self.LPM_SUS(),
            self.LPM_REWP(),
            self.Speed(),
            self.DCON_C(),
            self.DSUS_C(),
            self.DRES_C(),
            self.VBUS_DEBOUNCED(),
            self.PHY_TEST_MODE()
        )
    }
}
#[doc = "USB Endpoint Buffer Configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EPBUFCFG(pub u32);
impl EPBUFCFG {
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn BUF_SB(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn set_BUF_SB(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for EPBUFCFG {
    #[inline(always)]
    fn default() -> EPBUFCFG {
        EPBUFCFG(0)
    }
}
impl core::fmt::Debug for EPBUFCFG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPBUFCFG")
            .field("BUF_SB", &self.BUF_SB())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPBUFCFG {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EPBUFCFG {{ BUF_SB: {=u16:?} }}", self.BUF_SB())
    }
}
#[doc = "USB Endpoint Buffer in use."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EPINUSE(pub u32);
impl EPINUSE {
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn BUF(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn set_BUF(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for EPINUSE {
    #[inline(always)]
    fn default() -> EPINUSE {
        EPINUSE(0)
    }
}
impl core::fmt::Debug for EPINUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPINUSE").field("BUF", &self.BUF()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPINUSE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EPINUSE {{ BUF: {=u16:?} }}", self.BUF())
    }
}
#[doc = "USB EP Command/Status List start address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EPLISTSTART(pub u32);
impl EPLISTSTART {
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[must_use]
    #[inline(always)]
    pub const fn EP_LIST_PRG(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn set_EP_LIST_PRG(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[must_use]
    #[inline(always)]
    pub const fn EP_LIST_FIXED(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn set_EP_LIST_FIXED(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for EPLISTSTART {
    #[inline(always)]
    fn default() -> EPLISTSTART {
        EPLISTSTART(0)
    }
}
impl core::fmt::Debug for EPLISTSTART {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPLISTSTART")
            .field("EP_LIST_PRG", &self.EP_LIST_PRG())
            .field("EP_LIST_FIXED", &self.EP_LIST_FIXED())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPLISTSTART {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EPLISTSTART {{ EP_LIST_PRG: {=u16:?}, EP_LIST_FIXED: {=u16:?} }}",
            self.EP_LIST_PRG(),
            self.EP_LIST_FIXED()
        )
    }
}
#[doc = "USB Endpoint skip."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EPSKIP(pub u32);
impl EPSKIP {
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[must_use]
    #[inline(always)]
    pub const fn SKIP(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub const fn set_SKIP(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for EPSKIP {
    #[inline(always)]
    fn default() -> EPSKIP {
        EPSKIP(0)
    }
}
impl core::fmt::Debug for EPSKIP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPSKIP")
            .field("SKIP", &self.SKIP())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPSKIP {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EPSKIP {{ SKIP: {=u16:?} }}", self.SKIP())
    }
}
#[doc = "USB Endpoint toggle register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EPTOGGLE(pub u32);
impl EPTOGGLE {
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn TOGGLE(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub const fn set_TOGGLE(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for EPTOGGLE {
    #[inline(always)]
    fn default() -> EPTOGGLE {
        EPTOGGLE(0)
    }
}
impl core::fmt::Debug for EPTOGGLE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPTOGGLE")
            .field("TOGGLE", &self.TOGGLE())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EPTOGGLE {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EPTOGGLE {{ TOGGLE: {=u32:?} }}", self.TOGGLE())
    }
}
#[doc = "USB Info register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INFO(pub u32);
impl INFO {
    #[doc = "Frame number."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAME_NR(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frame number."]
    #[inline(always)]
    pub const fn set_FRAME_NR(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "The error code which last occurred:."]
    #[must_use]
    #[inline(always)]
    pub const fn ERR_CODE(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "The error code which last occurred:."]
    #[inline(always)]
    pub const fn set_ERR_CODE(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Minor revision."]
    #[must_use]
    #[inline(always)]
    pub const fn MINREV(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision."]
    #[inline(always)]
    pub const fn set_MINREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major revision."]
    #[must_use]
    #[inline(always)]
    pub const fn MAJREV(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision."]
    #[inline(always)]
    pub const fn set_MAJREV(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for INFO {
    #[inline(always)]
    fn default() -> INFO {
        INFO(0)
    }
}
impl core::fmt::Debug for INFO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INFO")
            .field("FRAME_NR", &self.FRAME_NR())
            .field("ERR_CODE", &self.ERR_CODE())
            .field("MINREV", &self.MINREV())
            .field("MAJREV", &self.MAJREV())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INFO {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INFO {{ FRAME_NR: {=u16:?}, ERR_CODE: {=u8:?}, MINREV: {=u8:?}, MAJREV: {=u8:?} }}",
            self.FRAME_NR(),
            self.ERR_CODE(),
            self.MINREV(),
            self.MAJREV()
        )
    }
}
#[doc = "USB interrupt enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTEN(pub u32);
impl INTEN {
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn EP_INT_EN(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_EP_INT_EN(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAME_INT_EN(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_FRAME_INT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_INT_EN(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_DEV_INT_EN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for INTEN {
    #[inline(always)]
    fn default() -> INTEN {
        INTEN(0)
    }
}
impl core::fmt::Debug for INTEN {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("EP_INT_EN", &self.EP_INT_EN())
            .field("FRAME_INT_EN", &self.FRAME_INT_EN())
            .field("DEV_INT_EN", &self.DEV_INT_EN())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTEN {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTEN {{ EP_INT_EN: {=u16:?}, FRAME_INT_EN: {=bool:?}, DEV_INT_EN: {=bool:?} }}",
            self.EP_INT_EN(),
            self.FRAME_INT_EN(),
            self.DEV_INT_EN()
        )
    }
}
#[doc = "USB set interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSETSTAT(pub u32);
impl INTSETSTAT {
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn EP_SET_INT(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_EP_SET_INT(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAME_SET_INT(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_FRAME_SET_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_SET_INT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_DEV_SET_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for INTSETSTAT {
    #[inline(always)]
    fn default() -> INTSETSTAT {
        INTSETSTAT(0)
    }
}
impl core::fmt::Debug for INTSETSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSETSTAT")
            .field("EP_SET_INT", &self.EP_SET_INT())
            .field("FRAME_SET_INT", &self.FRAME_SET_INT())
            .field("DEV_SET_INT", &self.DEV_SET_INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSETSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSETSTAT {{ EP_SET_INT: {=u16:?}, FRAME_SET_INT: {=bool:?}, DEV_SET_INT: {=bool:?} }}",
            self.EP_SET_INT(),
            self.FRAME_SET_INT(),
            self.DEV_SET_INT()
        )
    }
}
#[doc = "USB interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct INTSTAT(pub u32);
impl INTSTAT {
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP0OUT(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub const fn set_EP0OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP0IN(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub const fn set_EP0IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP1OUT(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub const fn set_EP1OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP1IN(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub const fn set_EP1IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP2OUT(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub const fn set_EP2OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP2IN(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub const fn set_EP2IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP3OUT(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub const fn set_EP3OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP3IN(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub const fn set_EP3IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP4OUT(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub const fn set_EP4OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP4IN(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub const fn set_EP4IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP5OUT(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub const fn set_EP5OUT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn EP5IN(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub const fn set_EP5IN(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn FRAME_INT(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Frame interrupt."]
    #[inline(always)]
    pub const fn set_FRAME_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Device status interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn DEV_INT(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Device status interrupt."]
    #[inline(always)]
    pub const fn set_DEV_INT(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for INTSTAT {
    #[inline(always)]
    fn default() -> INTSTAT {
        INTSTAT(0)
    }
}
impl core::fmt::Debug for INTSTAT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTAT")
            .field("EP0OUT", &self.EP0OUT())
            .field("EP0IN", &self.EP0IN())
            .field("EP1OUT", &self.EP1OUT())
            .field("EP1IN", &self.EP1IN())
            .field("EP2OUT", &self.EP2OUT())
            .field("EP2IN", &self.EP2IN())
            .field("EP3OUT", &self.EP3OUT())
            .field("EP3IN", &self.EP3IN())
            .field("EP4OUT", &self.EP4OUT())
            .field("EP4IN", &self.EP4IN())
            .field("EP5OUT", &self.EP5OUT())
            .field("EP5IN", &self.EP5IN())
            .field("FRAME_INT", &self.FRAME_INT())
            .field("DEV_INT", &self.DEV_INT())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for INTSTAT {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "INTSTAT {{ EP0OUT: {=bool:?}, EP0IN: {=bool:?}, EP1OUT: {=bool:?}, EP1IN: {=bool:?}, EP2OUT: {=bool:?}, EP2IN: {=bool:?}, EP3OUT: {=bool:?}, EP3IN: {=bool:?}, EP4OUT: {=bool:?}, EP4IN: {=bool:?}, EP5OUT: {=bool:?}, EP5IN: {=bool:?}, FRAME_INT: {=bool:?}, DEV_INT: {=bool:?} }}",
            self.EP0OUT(),
            self.EP0IN(),
            self.EP1OUT(),
            self.EP1IN(),
            self.EP2OUT(),
            self.EP2IN(),
            self.EP3OUT(),
            self.EP3IN(),
            self.EP4OUT(),
            self.EP4IN(),
            self.EP5OUT(),
            self.EP5IN(),
            self.FRAME_INT(),
            self.DEV_INT()
        )
    }
}
#[doc = "USB Link Power Management register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LPM(pub u32);
impl LPM {
    #[doc = "Host Initiated Resume Duration - HW."]
    #[must_use]
    #[inline(always)]
    pub const fn HIRD_HW(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub const fn set_HIRD_HW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[must_use]
    #[inline(always)]
    pub const fn HIRD_SW(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub const fn set_HIRD_SW(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[must_use]
    #[inline(always)]
    pub const fn DATA_PENDING(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub const fn set_DATA_PENDING(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LPM {
    #[inline(always)]
    fn default() -> LPM {
        LPM(0)
    }
}
impl core::fmt::Debug for LPM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPM")
            .field("HIRD_HW", &self.HIRD_HW())
            .field("HIRD_SW", &self.HIRD_SW())
            .field("DATA_PENDING", &self.DATA_PENDING())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LPM {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LPM {{ HIRD_HW: {=u8:?}, HIRD_SW: {=u8:?}, DATA_PENDING: {=bool:?} }}",
            self.HIRD_HW(),
            self.HIRD_SW(),
            self.DATA_PENDING()
        )
    }
}
