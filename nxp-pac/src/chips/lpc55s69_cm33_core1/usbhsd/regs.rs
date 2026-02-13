#[doc = "USB Data buffer start address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Databufstart(pub u32);
impl Databufstart {
    #[doc = "Start address of the memory page where all endpoint data buffers are located."]
    #[must_use]
    #[inline(always)]
    pub const fn da_buf(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub const fn set_da_buf(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Databufstart {
    #[inline(always)]
    fn default() -> Databufstart {
        Databufstart(0)
    }
}
impl core::fmt::Debug for Databufstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Databufstart")
            .field("da_buf", &self.da_buf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Databufstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Databufstart {{ da_buf: {=u32:?} }}", self.da_buf())
    }
}
#[doc = "USB Device Command/Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devcmdstat(pub u32);
impl Devcmdstat {
    #[doc = "USB device address."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn set_dev_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "USB device enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB device enable."]
    #[inline(always)]
    pub const fn set_dev_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SETUP token received."]
    #[must_use]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP token received."]
    #[inline(always)]
    pub const fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[must_use]
    #[inline(always)]
    pub const fn force_needclk(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub const fn set_force_needclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPM Supported:."]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_sup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Supported:."]
    #[inline(always)]
    pub const fn set_lpm_sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn intonnak_ao(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub const fn set_intonnak_ao(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn intonnak_ai(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub const fn set_intonnak_ai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn intonnak_co(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub const fn set_intonnak_co(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[must_use]
    #[inline(always)]
    pub const fn intonnak_ci(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub const fn set_intonnak_ci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Device status - connect."]
    #[must_use]
    #[inline(always)]
    pub const fn dcon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect."]
    #[inline(always)]
    pub const fn set_dcon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device status - suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn dsus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend."]
    #[inline(always)]
    pub const fn set_dsus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Device status - LPM Suspend."]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_sus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - LPM Suspend."]
    #[inline(always)]
    pub const fn set_lpm_sus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_rewp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub const fn set_lpm_rewp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[must_use]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub const fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Device status - connect change."]
    #[must_use]
    #[inline(always)]
    pub const fn dcon_c(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect change."]
    #[inline(always)]
    pub const fn set_dcon_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device status - suspend change."]
    #[must_use]
    #[inline(always)]
    pub const fn dsus_c(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend change."]
    #[inline(always)]
    pub const fn set_dsus_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Device status - reset change."]
    #[must_use]
    #[inline(always)]
    pub const fn dres_c(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - reset change."]
    #[inline(always)]
    pub const fn set_dres_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[must_use]
    #[inline(always)]
    pub const fn vbus_debounced(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub const fn set_vbus_debounced(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[must_use]
    #[inline(always)]
    pub const fn phy_test_mode(&self) -> super::vals::PhyTestMode {
        let val = (self.0 >> 29usize) & 0x07;
        super::vals::PhyTestMode::from_bits(val as u8)
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub const fn set_phy_test_mode(&mut self, val: super::vals::PhyTestMode) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
    }
}
impl Default for Devcmdstat {
    #[inline(always)]
    fn default() -> Devcmdstat {
        Devcmdstat(0)
    }
}
impl core::fmt::Debug for Devcmdstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devcmdstat")
            .field("dev_addr", &self.dev_addr())
            .field("dev_en", &self.dev_en())
            .field("setup", &self.setup())
            .field("force_needclk", &self.force_needclk())
            .field("lpm_sup", &self.lpm_sup())
            .field("intonnak_ao", &self.intonnak_ao())
            .field("intonnak_ai", &self.intonnak_ai())
            .field("intonnak_co", &self.intonnak_co())
            .field("intonnak_ci", &self.intonnak_ci())
            .field("dcon", &self.dcon())
            .field("dsus", &self.dsus())
            .field("lpm_sus", &self.lpm_sus())
            .field("lpm_rewp", &self.lpm_rewp())
            .field("speed", &self.speed())
            .field("dcon_c", &self.dcon_c())
            .field("dsus_c", &self.dsus_c())
            .field("dres_c", &self.dres_c())
            .field("vbus_debounced", &self.vbus_debounced())
            .field("phy_test_mode", &self.phy_test_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devcmdstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Devcmdstat {{ dev_addr: {=u8:?}, dev_en: {=bool:?}, setup: {=bool:?}, force_needclk: {=bool:?}, lpm_sup: {=bool:?}, intonnak_ao: {=bool:?}, intonnak_ai: {=bool:?}, intonnak_co: {=bool:?}, intonnak_ci: {=bool:?}, dcon: {=bool:?}, dsus: {=bool:?}, lpm_sus: {=bool:?}, lpm_rewp: {=bool:?}, speed: {=u8:?}, dcon_c: {=bool:?}, dsus_c: {=bool:?}, dres_c: {=bool:?}, vbus_debounced: {=bool:?}, phy_test_mode: {:?} }}",
            self.dev_addr(),
            self.dev_en(),
            self.setup(),
            self.force_needclk(),
            self.lpm_sup(),
            self.intonnak_ao(),
            self.intonnak_ai(),
            self.intonnak_co(),
            self.intonnak_ci(),
            self.dcon(),
            self.dsus(),
            self.lpm_sus(),
            self.lpm_rewp(),
            self.speed(),
            self.dcon_c(),
            self.dsus_c(),
            self.dres_c(),
            self.vbus_debounced(),
            self.phy_test_mode()
        )
    }
}
#[doc = "USB Endpoint Buffer Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epbufcfg(pub u32);
impl Epbufcfg {
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn buf_sb(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn set_buf_sb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Epbufcfg {
    #[inline(always)]
    fn default() -> Epbufcfg {
        Epbufcfg(0)
    }
}
impl core::fmt::Debug for Epbufcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Epbufcfg")
            .field("buf_sb", &self.buf_sb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Epbufcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Epbufcfg {{ buf_sb: {=u16:?} }}", self.buf_sb())
    }
}
#[doc = "USB Endpoint Buffer in use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epinuse(pub u32);
impl Epinuse {
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn buf(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn set_buf(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Epinuse {
    #[inline(always)]
    fn default() -> Epinuse {
        Epinuse(0)
    }
}
impl core::fmt::Debug for Epinuse {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Epinuse").field("buf", &self.buf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Epinuse {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Epinuse {{ buf: {=u16:?} }}", self.buf())
    }
}
#[doc = "USB EP Command/Status List start address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epliststart(pub u32);
impl Epliststart {
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[must_use]
    #[inline(always)]
    pub const fn ep_list_prg(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn set_ep_list_prg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[must_use]
    #[inline(always)]
    pub const fn ep_list_fixed(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn set_ep_list_fixed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Epliststart {
    #[inline(always)]
    fn default() -> Epliststart {
        Epliststart(0)
    }
}
impl core::fmt::Debug for Epliststart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Epliststart")
            .field("ep_list_prg", &self.ep_list_prg())
            .field("ep_list_fixed", &self.ep_list_fixed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Epliststart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Epliststart {{ ep_list_prg: {=u16:?}, ep_list_fixed: {=u16:?} }}",
            self.ep_list_prg(),
            self.ep_list_fixed()
        )
    }
}
#[doc = "USB Endpoint skip"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epskip(pub u32);
impl Epskip {
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[must_use]
    #[inline(always)]
    pub const fn skip(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub const fn set_skip(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Epskip {
    #[inline(always)]
    fn default() -> Epskip {
        Epskip(0)
    }
}
impl core::fmt::Debug for Epskip {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Epskip")
            .field("skip", &self.skip())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Epskip {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Epskip {{ skip: {=u16:?} }}", self.skip())
    }
}
#[doc = "USB Endpoint toggle register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eptoggle(pub u32);
impl Eptoggle {
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[must_use]
    #[inline(always)]
    pub const fn toggle(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub const fn set_toggle(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Eptoggle {
    #[inline(always)]
    fn default() -> Eptoggle {
        Eptoggle(0)
    }
}
impl core::fmt::Debug for Eptoggle {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eptoggle")
            .field("toggle", &self.toggle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eptoggle {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Eptoggle {{ toggle: {=u32:?} }}", self.toggle())
    }
}
#[doc = "USB Info register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info(pub u32);
impl Info {
    #[doc = "Frame number."]
    #[must_use]
    #[inline(always)]
    pub const fn frame_nr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frame number."]
    #[inline(always)]
    pub const fn set_frame_nr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "The error code which last occurred:."]
    #[must_use]
    #[inline(always)]
    pub const fn err_code(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "The error code which last occurred:."]
    #[inline(always)]
    pub const fn set_err_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Minor revision."]
    #[must_use]
    #[inline(always)]
    pub const fn minrev(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision."]
    #[inline(always)]
    pub const fn set_minrev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major revision."]
    #[must_use]
    #[inline(always)]
    pub const fn majrev(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision."]
    #[inline(always)]
    pub const fn set_majrev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Info {
    #[inline(always)]
    fn default() -> Info {
        Info(0)
    }
}
impl core::fmt::Debug for Info {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Info")
            .field("frame_nr", &self.frame_nr())
            .field("err_code", &self.err_code())
            .field("minrev", &self.minrev())
            .field("majrev", &self.majrev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Info {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Info {{ frame_nr: {=u16:?}, err_code: {=u8:?}, minrev: {=u8:?}, majrev: {=u8:?} }}",
            self.frame_nr(),
            self.err_code(),
            self.minrev(),
            self.majrev()
        )
    }
}
#[doc = "USB interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn ep_int_en(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_ep_int_en(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn frame_int_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_frame_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_int_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn set_dev_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("ep_int_en", &self.ep_int_en())
            .field("frame_int_en", &self.frame_int_en())
            .field("dev_int_en", &self.dev_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ep_int_en: {=u16:?}, frame_int_en: {=bool:?}, dev_int_en: {=bool:?} }}",
            self.ep_int_en(),
            self.frame_int_en(),
            self.dev_int_en()
        )
    }
}
#[doc = "USB set interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsetstat(pub u32);
impl Intsetstat {
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn ep_set_int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_ep_set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn frame_set_int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_frame_set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_set_int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn set_dev_set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Intsetstat {
    #[inline(always)]
    fn default() -> Intsetstat {
        Intsetstat(0)
    }
}
impl core::fmt::Debug for Intsetstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intsetstat")
            .field("ep_set_int", &self.ep_set_int())
            .field("frame_set_int", &self.frame_set_int())
            .field("dev_set_int", &self.dev_set_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intsetstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intsetstat {{ ep_set_int: {=u16:?}, frame_set_int: {=bool:?}, dev_set_int: {=bool:?} }}",
            self.ep_set_int(),
            self.frame_set_int(),
            self.dev_set_int()
        )
    }
}
#[doc = "USB interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep0out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub const fn set_ep0out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep0in(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub const fn set_ep0in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep1out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub const fn set_ep1out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep1in(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub const fn set_ep1in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep2out(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub const fn set_ep2out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep2in(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub const fn set_ep2in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep3out(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub const fn set_ep3out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep3in(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub const fn set_ep3in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep4out(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub const fn set_ep4out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep4in(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub const fn set_ep4in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep5out(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub const fn set_ep5out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[must_use]
    #[inline(always)]
    pub const fn ep5in(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub const fn set_ep5in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Frame interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frame_int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Frame interrupt."]
    #[inline(always)]
    pub const fn set_frame_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Device status interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dev_int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Device status interrupt."]
    #[inline(always)]
    pub const fn set_dev_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("ep0out", &self.ep0out())
            .field("ep0in", &self.ep0in())
            .field("ep1out", &self.ep1out())
            .field("ep1in", &self.ep1in())
            .field("ep2out", &self.ep2out())
            .field("ep2in", &self.ep2in())
            .field("ep3out", &self.ep3out())
            .field("ep3in", &self.ep3in())
            .field("ep4out", &self.ep4out())
            .field("ep4in", &self.ep4in())
            .field("ep5out", &self.ep5out())
            .field("ep5in", &self.ep5in())
            .field("frame_int", &self.frame_int())
            .field("dev_int", &self.dev_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intstat {{ ep0out: {=bool:?}, ep0in: {=bool:?}, ep1out: {=bool:?}, ep1in: {=bool:?}, ep2out: {=bool:?}, ep2in: {=bool:?}, ep3out: {=bool:?}, ep3in: {=bool:?}, ep4out: {=bool:?}, ep4in: {=bool:?}, ep5out: {=bool:?}, ep5in: {=bool:?}, frame_int: {=bool:?}, dev_int: {=bool:?} }}",
            self.ep0out(),
            self.ep0in(),
            self.ep1out(),
            self.ep1in(),
            self.ep2out(),
            self.ep2in(),
            self.ep3out(),
            self.ep3in(),
            self.ep4out(),
            self.ep4in(),
            self.ep5out(),
            self.ep5in(),
            self.frame_int(),
            self.dev_int()
        )
    }
}
#[doc = "USB Link Power Management register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpm(pub u32);
impl Lpm {
    #[doc = "Host Initiated Resume Duration - HW."]
    #[must_use]
    #[inline(always)]
    pub const fn hird_hw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub const fn set_hird_hw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[must_use]
    #[inline(always)]
    pub const fn hird_sw(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub const fn set_hird_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[must_use]
    #[inline(always)]
    pub const fn data_pending(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub const fn set_data_pending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Lpm {
    #[inline(always)]
    fn default() -> Lpm {
        Lpm(0)
    }
}
impl core::fmt::Debug for Lpm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lpm")
            .field("hird_hw", &self.hird_hw())
            .field("hird_sw", &self.hird_sw())
            .field("data_pending", &self.data_pending())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lpm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lpm {{ hird_hw: {=u8:?}, hird_sw: {=u8:?}, data_pending: {=bool:?} }}",
            self.hird_hw(),
            self.hird_sw(),
            self.data_pending()
        )
    }
}
