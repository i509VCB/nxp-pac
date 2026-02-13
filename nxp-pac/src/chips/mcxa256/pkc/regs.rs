#[doc = "Access Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcAccessErr(pub u32);
impl PkcAccessErr {
    #[doc = "APB Error"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_notav(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "APB Error"]
    #[inline(always)]
    pub const fn set_apb_notav(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "APB Error"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_wrgmd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "APB Error"]
    #[inline(always)]
    pub const fn set_apb_wrgmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APB Master that triggered first APB error (APB_WRGMD or APB_NOTAV)"]
    #[must_use]
    #[inline(always)]
    pub const fn apb_master(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "APB Master that triggered first APB error (APB_WRGMD or APB_NOTAV)"]
    #[inline(always)]
    pub const fn set_apb_master(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "AHB Error"]
    #[must_use]
    #[inline(always)]
    pub const fn ahb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Error"]
    #[inline(always)]
    pub const fn set_ahb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Error in PKC coprocessor kernel"]
    #[must_use]
    #[inline(always)]
    pub const fn pkcc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Error in PKC coprocessor kernel"]
    #[inline(always)]
    pub const fn set_pkcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Error due to error detection circuitry"]
    #[must_use]
    #[inline(always)]
    pub const fn fdet(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Error due to error detection circuitry"]
    #[inline(always)]
    pub const fn set_fdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Error in PKC software control"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Error in PKC software control"]
    #[inline(always)]
    pub const fn set_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Error in layer2 CRC check."]
    #[must_use]
    #[inline(always)]
    pub const fn ucrc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Error in layer2 CRC check."]
    #[inline(always)]
    pub const fn set_ucrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for PkcAccessErr {
    #[inline(always)]
    fn default() -> PkcAccessErr {
        PkcAccessErr(0)
    }
}
impl core::fmt::Debug for PkcAccessErr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcAccessErr")
            .field("apb_notav", &self.apb_notav())
            .field("apb_wrgmd", &self.apb_wrgmd())
            .field("apb_master", &self.apb_master())
            .field("ahb", &self.ahb())
            .field("pkcc", &self.pkcc())
            .field("fdet", &self.fdet())
            .field("ctrl", &self.ctrl())
            .field("ucrc", &self.ucrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcAccessErr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcAccessErr {{ apb_notav: {=bool:?}, apb_wrgmd: {=bool:?}, apb_master: {=u8:?}, ahb: {=bool:?}, pkcc: {=bool:?}, fdet: {=bool:?}, ctrl: {=bool:?}, ucrc: {=bool:?} }}",
            self.apb_notav(),
            self.apb_wrgmd(),
            self.apb_master(),
            self.ahb(),
            self.pkcc(),
            self.fdet(),
            self.ctrl(),
            self.ucrc()
        )
    }
}
#[doc = "Clear Access Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcAccessErrClr(pub u32);
impl PkcAccessErrClr {
    #[doc = "Write 1 to reset PKC_ACCESS_ERR SFR."]
    #[must_use]
    #[inline(always)]
    pub const fn err_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to reset PKC_ACCESS_ERR SFR."]
    #[inline(always)]
    pub const fn set_err_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcAccessErrClr {
    #[inline(always)]
    fn default() -> PkcAccessErrClr {
        PkcAccessErrClr(0)
    }
}
impl core::fmt::Debug for PkcAccessErrClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcAccessErrClr")
            .field("err_clr", &self.err_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcAccessErrClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcAccessErrClr {{ err_clr: {=bool:?} }}",
            self.err_clr()
        )
    }
}
#[doc = "Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcCfg(pub u32);
impl PkcCfg {
    #[doc = "Idle operation feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn idleop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Idle operation feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_idleop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_rfu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RFU"]
    #[must_use]
    #[inline(always)]
    pub const fn rfu2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RFU"]
    #[inline(always)]
    pub const fn set_rfu2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clock randomization feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn clkrnd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clock randomization feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_clkrnd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Noise in reduced multiplier mode feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn redmulnoise(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Noise in reduced multiplier mode feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_redmulnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Random delay feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn rnddly(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Random delay feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_rnddly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn sbxnoise(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_sbxnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn alpnoise(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_alpnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[must_use]
    #[inline(always)]
    pub const fn fmulnoise(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Noise feature not available in this version (flag is don't care)."]
    #[inline(always)]
    pub const fn set_fmulnoise(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for PkcCfg {
    #[inline(always)]
    fn default() -> PkcCfg {
        PkcCfg(0)
    }
}
impl core::fmt::Debug for PkcCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcCfg")
            .field("idleop", &self.idleop())
            .field("rfu1", &self.rfu1())
            .field("rfu2", &self.rfu2())
            .field("clkrnd", &self.clkrnd())
            .field("redmulnoise", &self.redmulnoise())
            .field("rnddly", &self.rnddly())
            .field("sbxnoise", &self.sbxnoise())
            .field("alpnoise", &self.alpnoise())
            .field("fmulnoise", &self.fmulnoise())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcCfg {{ idleop: {=bool:?}, rfu1: {=bool:?}, rfu2: {=bool:?}, clkrnd: {=bool:?}, redmulnoise: {=bool:?}, rnddly: {=u8:?}, sbxnoise: {=bool:?}, alpnoise: {=bool:?}, fmulnoise: {=bool:?} }}",
            self.idleop(),
            self.rfu1(),
            self.rfu2(),
            self.clkrnd(),
            self.redmulnoise(),
            self.rnddly(),
            self.sbxnoise(),
            self.alpnoise(),
            self.fmulnoise()
        )
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcCtrl(pub u32);
impl PkcCtrl {
    #[doc = "PKC reset control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PKC reset control bit"]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Freeze PKC calculation"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze PKC calculation"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Control bit to start direct operation using parameter set 1"]
    #[must_use]
    #[inline(always)]
    pub const fn god1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to start direct operation using parameter set 1"]
    #[inline(always)]
    pub const fn set_god1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Control bit to start direct operation using parameter set 2"]
    #[must_use]
    #[inline(always)]
    pub const fn god2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to start direct operation using parameter set 2"]
    #[inline(always)]
    pub const fn set_god2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Control bit to start MC pattern using parameter set 1"]
    #[must_use]
    #[inline(always)]
    pub const fn gom1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to start MC pattern using parameter set 1"]
    #[inline(always)]
    pub const fn set_gom1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Control bit to start MC pattern using parameter set 2"]
    #[must_use]
    #[inline(always)]
    pub const fn gom2(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to start MC pattern using parameter set 2"]
    #[inline(always)]
    pub const fn set_gom2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Control bit to start pipe operation"]
    #[must_use]
    #[inline(always)]
    pub const fn gou(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Control bit to start pipe operation"]
    #[inline(always)]
    pub const fn set_gou(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Convert to GF2 calculation modes"]
    #[must_use]
    #[inline(always)]
    pub const fn gf2conv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Convert to GF2 calculation modes"]
    #[inline(always)]
    pub const fn set_gf2conv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Clear universal pointer cache"]
    #[must_use]
    #[inline(always)]
    pub const fn clrcache(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Clear universal pointer cache"]
    #[inline(always)]
    pub const fn set_clrcache(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable universal pointer cache"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable universal pointer cache"]
    #[inline(always)]
    pub const fn set_cache_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Reduced multiplier mode"]
    #[must_use]
    #[inline(always)]
    pub const fn redmul(&self) -> super::vals::Redmul {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Redmul::from_bits(val as u8)
    }
    #[doc = "Reduced multiplier mode"]
    #[inline(always)]
    pub const fn set_redmul(&mut self, val: super::vals::Redmul) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for PkcCtrl {
    #[inline(always)]
    fn default() -> PkcCtrl {
        PkcCtrl(0)
    }
}
impl core::fmt::Debug for PkcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcCtrl")
            .field("reset", &self.reset())
            .field("stop", &self.stop())
            .field("god1", &self.god1())
            .field("god2", &self.god2())
            .field("gom1", &self.gom1())
            .field("gom2", &self.gom2())
            .field("gou", &self.gou())
            .field("gf2conv", &self.gf2conv())
            .field("clrcache", &self.clrcache())
            .field("cache_en", &self.cache_en())
            .field("redmul", &self.redmul())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcCtrl {{ reset: {=bool:?}, stop: {=bool:?}, god1: {=bool:?}, god2: {=bool:?}, gom1: {=bool:?}, gom2: {=bool:?}, gou: {=bool:?}, gf2conv: {=bool:?}, clrcache: {=bool:?}, cache_en: {=bool:?}, redmul: {:?} }}",
            self.reset(),
            self.stop(),
            self.god1(),
            self.god2(),
            self.gom1(),
            self.gom2(),
            self.gou(),
            self.gf2conv(),
            self.clrcache(),
            self.cache_en(),
            self.redmul()
        )
    }
}
#[doc = "Interrupt enable clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntClrEnable(pub u32);
impl PkcIntClrEnable {
    #[doc = "Write to clear PDONE interrupt enable flag (PKC_INT_ENABLE\\[EN_PDONE\\]=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn en_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear PDONE interrupt enable flag (PKC_INT_ENABLE\\[EN_PDONE\\]=0)."]
    #[inline(always)]
    pub const fn set_en_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntClrEnable {
    #[inline(always)]
    fn default() -> PkcIntClrEnable {
        PkcIntClrEnable(0)
    }
}
impl core::fmt::Debug for PkcIntClrEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntClrEnable")
            .field("en_pdone", &self.en_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntClrEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcIntClrEnable {{ en_pdone: {=bool:?} }}",
            self.en_pdone()
        )
    }
}
#[doc = "Interrupt status clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntClrStatus(pub u32);
impl PkcIntClrStatus {
    #[doc = "Write to clear End-of-computation status flag (PKC_INT_STATUS\\[INT_PDONE\\]=0)."]
    #[must_use]
    #[inline(always)]
    pub const fn int_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to clear End-of-computation status flag (PKC_INT_STATUS\\[INT_PDONE\\]=0)."]
    #[inline(always)]
    pub const fn set_int_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntClrStatus {
    #[inline(always)]
    fn default() -> PkcIntClrStatus {
        PkcIntClrStatus(0)
    }
}
impl core::fmt::Debug for PkcIntClrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntClrStatus")
            .field("int_pdone", &self.int_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntClrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcIntClrStatus {{ int_pdone: {=bool:?} }}",
            self.int_pdone()
        )
    }
}
#[doc = "Interrupt enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntEnable(pub u32);
impl PkcIntEnable {
    #[doc = "PDONE interrupt enable flag"]
    #[must_use]
    #[inline(always)]
    pub const fn en_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PDONE interrupt enable flag"]
    #[inline(always)]
    pub const fn set_en_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntEnable {
    #[inline(always)]
    fn default() -> PkcIntEnable {
        PkcIntEnable(0)
    }
}
impl core::fmt::Debug for PkcIntEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntEnable")
            .field("en_pdone", &self.en_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcIntEnable {{ en_pdone: {=bool:?} }}", self.en_pdone())
    }
}
#[doc = "Interrupt enable set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntSetEnable(pub u32);
impl PkcIntSetEnable {
    #[doc = "Write to set PDONE interrupt enable flag (PKC_INT_ENABLE\\[EN_PDONE\\]=1)."]
    #[must_use]
    #[inline(always)]
    pub const fn en_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to set PDONE interrupt enable flag (PKC_INT_ENABLE\\[EN_PDONE\\]=1)."]
    #[inline(always)]
    pub const fn set_en_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntSetEnable {
    #[inline(always)]
    fn default() -> PkcIntSetEnable {
        PkcIntSetEnable(0)
    }
}
impl core::fmt::Debug for PkcIntSetEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntSetEnable")
            .field("en_pdone", &self.en_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntSetEnable {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcIntSetEnable {{ en_pdone: {=bool:?} }}",
            self.en_pdone()
        )
    }
}
#[doc = "Interrupt status set"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntSetStatus(pub u32);
impl PkcIntSetStatus {
    #[doc = "Write to set End-of-computation status flag (PKC_INT_STATUS\\[INT_PDONE\\]=1) to trigger a PKC interrupt via software, e"]
    #[must_use]
    #[inline(always)]
    pub const fn int_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to set End-of-computation status flag (PKC_INT_STATUS\\[INT_PDONE\\]=1) to trigger a PKC interrupt via software, e"]
    #[inline(always)]
    pub const fn set_int_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntSetStatus {
    #[inline(always)]
    fn default() -> PkcIntSetStatus {
        PkcIntSetStatus(0)
    }
}
impl core::fmt::Debug for PkcIntSetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntSetStatus")
            .field("int_pdone", &self.int_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntSetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcIntSetStatus {{ int_pdone: {=bool:?} }}",
            self.int_pdone()
        )
    }
}
#[doc = "Interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcIntStatus(pub u32);
impl PkcIntStatus {
    #[doc = "End-of-computation status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn int_pdone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "End-of-computation status flag"]
    #[inline(always)]
    pub const fn set_int_pdone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcIntStatus {
    #[inline(always)]
    fn default() -> PkcIntStatus {
        PkcIntStatus(0)
    }
}
impl core::fmt::Debug for PkcIntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcIntStatus")
            .field("int_pdone", &self.int_pdone())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcIntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcIntStatus {{ int_pdone: {=bool:?} }}",
            self.int_pdone()
        )
    }
}
#[doc = "Length register, parameter set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcLen1(pub u32);
impl PkcLen1 {
    #[doc = "Operand length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Operand length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Loop counter for microcode pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn mclen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Loop counter for microcode pattern"]
    #[inline(always)]
    pub const fn set_mclen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcLen1 {
    #[inline(always)]
    fn default() -> PkcLen1 {
        PkcLen1(0)
    }
}
impl core::fmt::Debug for PkcLen1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcLen1")
            .field("len", &self.len())
            .field("mclen", &self.mclen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcLen1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcLen1 {{ len: {=u16:?}, mclen: {=u16:?} }}",
            self.len(),
            self.mclen()
        )
    }
}
#[doc = "Length register, parameter set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcLen2(pub u32);
impl PkcLen2 {
    #[doc = "Operand length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Operand length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Loop counter for microcode pattern"]
    #[must_use]
    #[inline(always)]
    pub const fn mclen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Loop counter for microcode pattern"]
    #[inline(always)]
    pub const fn set_mclen(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcLen2 {
    #[inline(always)]
    fn default() -> PkcLen2 {
        PkcLen2(0)
    }
}
impl core::fmt::Debug for PkcLen2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcLen2")
            .field("len", &self.len())
            .field("mclen", &self.mclen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcLen2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcLen2 {{ len: {=u16:?}, mclen: {=u16:?} }}",
            self.len(),
            self.mclen()
        )
    }
}
#[doc = "MC pattern data interface"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcMcdata(pub u32);
impl PkcMcdata {
    #[doc = "Microcode read/write data"]
    #[must_use]
    #[inline(always)]
    pub const fn mcdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Microcode read/write data"]
    #[inline(always)]
    pub const fn set_mcdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PkcMcdata {
    #[inline(always)]
    fn default() -> PkcMcdata {
        PkcMcdata(0)
    }
}
impl core::fmt::Debug for PkcMcdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcMcdata")
            .field("mcdata", &self.mcdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcMcdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcMcdata {{ mcdata: {=u32:?} }}", self.mcdata())
    }
}
#[doc = "Mode register, parameter set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcMode1(pub u32);
impl PkcMode1 {
    #[doc = "Calculation Mode / MC Start address"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Calculation Mode / MC Start address"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PkcMode1 {
    #[inline(always)]
    fn default() -> PkcMode1 {
        PkcMode1(0)
    }
}
impl core::fmt::Debug for PkcMode1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcMode1")
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcMode1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcMode1 {{ mode: {=u8:?} }}", self.mode())
    }
}
#[doc = "Mode register, parameter set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcMode2(pub u32);
impl PkcMode2 {
    #[doc = "Calculation Mode / MC Start address"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Calculation Mode / MC Start address"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PkcMode2 {
    #[inline(always)]
    fn default() -> PkcMode2 {
        PkcMode2(0)
    }
}
impl core::fmt::Debug for PkcMode2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcMode2")
            .field("mode", &self.mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcMode2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcMode2 {{ mode: {=u8:?} }}", self.mode())
    }
}
#[doc = "Module ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcModuleId(pub u32);
impl PkcModuleId {
    #[doc = "Address space of the IP"]
    #[must_use]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Address space of the IP"]
    #[inline(always)]
    pub const fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision"]
    #[inline(always)]
    pub const fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision"]
    #[inline(always)]
    pub const fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module ID"]
    #[must_use]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module ID"]
    #[inline(always)]
    pub const fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcModuleId {
    #[inline(always)]
    fn default() -> PkcModuleId {
        PkcModuleId(0)
    }
}
impl core::fmt::Debug for PkcModuleId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcModuleId")
            .field("size", &self.size())
            .field("minor_rev", &self.minor_rev())
            .field("major_rev", &self.major_rev())
            .field("id", &self.id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcModuleId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcModuleId {{ size: {=u8:?}, minor_rev: {=u8:?}, major_rev: {=u8:?}, id: {=u16:?} }}",
            self.size(),
            self.minor_rev(),
            self.major_rev(),
            self.id()
        )
    }
}
#[doc = "Software reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcSoftRst(pub u32);
impl PkcSoftRst {
    #[doc = "Write 1 to reset module (0 has no effect)"]
    #[must_use]
    #[inline(always)]
    pub const fn soft_rst(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to reset module (0 has no effect)"]
    #[inline(always)]
    pub const fn set_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PkcSoftRst {
    #[inline(always)]
    fn default() -> PkcSoftRst {
        PkcSoftRst(0)
    }
}
impl core::fmt::Debug for PkcSoftRst {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcSoftRst")
            .field("soft_rst", &self.soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcSoftRst {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcSoftRst {{ soft_rst: {=bool:?} }}", self.soft_rst())
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcStatus(pub u32);
impl PkcStatus {
    #[doc = "PKC ACTIV"]
    #[must_use]
    #[inline(always)]
    pub const fn activ(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PKC ACTIV"]
    #[inline(always)]
    pub const fn set_activ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Carry overflow flag"]
    #[must_use]
    #[inline(always)]
    pub const fn carry(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Carry overflow flag"]
    #[inline(always)]
    pub const fn set_carry(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Zero result flag"]
    #[must_use]
    #[inline(always)]
    pub const fn zero(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Zero result flag"]
    #[inline(always)]
    pub const fn set_zero(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Combined GO status flag"]
    #[must_use]
    #[inline(always)]
    pub const fn goany(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Combined GO status flag"]
    #[inline(always)]
    pub const fn set_goany(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Parameter set locked"]
    #[must_use]
    #[inline(always)]
    pub const fn locked(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Parameter set locked"]
    #[inline(always)]
    pub const fn set_locked(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for PkcStatus {
    #[inline(always)]
    fn default() -> PkcStatus {
        PkcStatus(0)
    }
}
impl core::fmt::Debug for PkcStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcStatus")
            .field("activ", &self.activ())
            .field("carry", &self.carry())
            .field("zero", &self.zero())
            .field("goany", &self.goany())
            .field("locked", &self.locked())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcStatus {{ activ: {=bool:?}, carry: {=bool:?}, zero: {=bool:?}, goany: {=bool:?}, locked: {=u8:?} }}",
            self.activ(),
            self.carry(),
            self.zero(),
            self.goany(),
            self.locked()
        )
    }
}
#[doc = "Universal pointer length"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcUlen(pub u32);
impl PkcUlen {
    #[doc = "Length of universal pointer calculation"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Length of universal pointer calculation"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PkcUlen {
    #[inline(always)]
    fn default() -> PkcUlen {
        PkcUlen(0)
    }
}
impl core::fmt::Debug for PkcUlen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcUlen").field("len", &self.len()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcUlen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcUlen {{ len: {=u8:?} }}", self.len())
    }
}
#[doc = "Universal pointer FUP program"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcUptr(pub u32);
impl PkcUptr {
    #[doc = "Pointer to start address of PKC FUP program"]
    #[must_use]
    #[inline(always)]
    pub const fn ptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pointer to start address of PKC FUP program"]
    #[inline(always)]
    pub const fn set_ptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PkcUptr {
    #[inline(always)]
    fn default() -> PkcUptr {
        PkcUptr(0)
    }
}
impl core::fmt::Debug for PkcUptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcUptr").field("ptr", &self.ptr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcUptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcUptr {{ ptr: {=u32:?} }}", self.ptr())
    }
}
#[doc = "Universal pointer FUP table"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcUptrt(pub u32);
impl PkcUptrt {
    #[doc = "Pointer to start address of PKC FUP table"]
    #[must_use]
    #[inline(always)]
    pub const fn ptr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Pointer to start address of PKC FUP table"]
    #[inline(always)]
    pub const fn set_ptr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PkcUptrt {
    #[inline(always)]
    fn default() -> PkcUptrt {
        PkcUptrt(0)
    }
}
impl core::fmt::Debug for PkcUptrt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcUptrt")
            .field("ptr", &self.ptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcUptrt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PkcUptrt {{ ptr: {=u32:?} }}", self.ptr())
    }
}
#[doc = "PKC version register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcVersion(pub u32);
impl PkcVersion {
    #[doc = "Native multiplier size and operand granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn mulsize(&self) -> super::vals::Mulsize {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mulsize::from_bits(val as u8)
    }
    #[doc = "Native multiplier size and operand granularity"]
    #[inline(always)]
    pub const fn set_mulsize(&mut self, val: super::vals::Mulsize) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "MC feature (layer1 calculation) is available"]
    #[must_use]
    #[inline(always)]
    pub const fn mcavail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "MC feature (layer1 calculation) is available"]
    #[inline(always)]
    pub const fn set_mcavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UP feature (layer2 calculation) is available"]
    #[must_use]
    #[inline(always)]
    pub const fn upavail(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UP feature (layer2 calculation) is available"]
    #[inline(always)]
    pub const fn set_upavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "UP cache is available"]
    #[must_use]
    #[inline(always)]
    pub const fn upcacheavail(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "UP cache is available"]
    #[inline(always)]
    pub const fn set_upcacheavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GF2 calculation modes are available"]
    #[must_use]
    #[inline(always)]
    pub const fn gf2avail(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GF2 calculation modes are available"]
    #[inline(always)]
    pub const fn set_gf2avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Number of parameter sets for real calculation"]
    #[must_use]
    #[inline(always)]
    pub const fn paramnum(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Number of parameter sets for real calculation"]
    #[inline(always)]
    pub const fn set_paramnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "SBX0 operation is available"]
    #[must_use]
    #[inline(always)]
    pub const fn sbx0avail(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SBX0 operation is available"]
    #[inline(always)]
    pub const fn set_sbx0avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SBX1 operation is available"]
    #[must_use]
    #[inline(always)]
    pub const fn sbx1avail(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SBX1 operation is available"]
    #[inline(always)]
    pub const fn set_sbx1avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SBX2 operation is available"]
    #[must_use]
    #[inline(always)]
    pub const fn sbx2avail(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SBX2 operation is available"]
    #[inline(always)]
    pub const fn set_sbx2avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SBX3 operation is available"]
    #[must_use]
    #[inline(always)]
    pub const fn sbx3avail(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SBX3 operation is available"]
    #[inline(always)]
    pub const fn set_sbx3avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Size of reconfigurable MC table in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn mcreconf_size(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0xff;
        val as u8
    }
    #[doc = "Size of reconfigurable MC table in bytes."]
    #[inline(always)]
    pub const fn set_mcreconf_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 12usize)) | (((val as u32) & 0xff) << 12usize);
    }
}
impl Default for PkcVersion {
    #[inline(always)]
    fn default() -> PkcVersion {
        PkcVersion(0)
    }
}
impl core::fmt::Debug for PkcVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcVersion")
            .field("mulsize", &self.mulsize())
            .field("mcavail", &self.mcavail())
            .field("upavail", &self.upavail())
            .field("upcacheavail", &self.upcacheavail())
            .field("gf2avail", &self.gf2avail())
            .field("paramnum", &self.paramnum())
            .field("sbx0avail", &self.sbx0avail())
            .field("sbx1avail", &self.sbx1avail())
            .field("sbx2avail", &self.sbx2avail())
            .field("sbx3avail", &self.sbx3avail())
            .field("mcreconf_size", &self.mcreconf_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcVersion {{ mulsize: {:?}, mcavail: {=bool:?}, upavail: {=bool:?}, upcacheavail: {=bool:?}, gf2avail: {=bool:?}, paramnum: {=u8:?}, sbx0avail: {=bool:?}, sbx1avail: {=bool:?}, sbx2avail: {=bool:?}, sbx3avail: {=bool:?}, mcreconf_size: {=u8:?} }}",
            self.mulsize(),
            self.mcavail(),
            self.upavail(),
            self.upcacheavail(),
            self.gf2avail(),
            self.paramnum(),
            self.sbx0avail(),
            self.sbx1avail(),
            self.sbx2avail(),
            self.sbx3avail(),
            self.mcreconf_size()
        )
    }
}
#[doc = "X+Y pointer register, parameter set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcXyptr1(pub u32);
impl PkcXyptr1 {
    #[doc = "Start address of X operand in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn xptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of X operand in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_xptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn yptr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_yptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcXyptr1 {
    #[inline(always)]
    fn default() -> PkcXyptr1 {
        PkcXyptr1(0)
    }
}
impl core::fmt::Debug for PkcXyptr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcXyptr1")
            .field("xptr", &self.xptr())
            .field("yptr", &self.yptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcXyptr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcXyptr1 {{ xptr: {=u16:?}, yptr: {=u16:?} }}",
            self.xptr(),
            self.yptr()
        )
    }
}
#[doc = "X+Y pointer register, parameter set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcXyptr2(pub u32);
impl PkcXyptr2 {
    #[doc = "Start address of X operand in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn xptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of X operand in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_xptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn yptr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of Y operand in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_yptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcXyptr2 {
    #[inline(always)]
    fn default() -> PkcXyptr2 {
        PkcXyptr2(0)
    }
}
impl core::fmt::Debug for PkcXyptr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcXyptr2")
            .field("xptr", &self.xptr())
            .field("yptr", &self.yptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcXyptr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcXyptr2 {{ xptr: {=u16:?}, yptr: {=u16:?} }}",
            self.xptr(),
            self.yptr()
        )
    }
}
#[doc = "Z+R pointer register, parameter set 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcZrptr1(pub u32);
impl PkcZrptr1 {
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST"]
    #[must_use]
    #[inline(always)]
    pub const fn zptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST"]
    #[inline(always)]
    pub const fn set_zptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn rptr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_rptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcZrptr1 {
    #[inline(always)]
    fn default() -> PkcZrptr1 {
        PkcZrptr1(0)
    }
}
impl core::fmt::Debug for PkcZrptr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcZrptr1")
            .field("zptr", &self.zptr())
            .field("rptr", &self.rptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcZrptr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcZrptr1 {{ zptr: {=u16:?}, rptr: {=u16:?} }}",
            self.zptr(),
            self.rptr()
        )
    }
}
#[doc = "Z+R pointer register, parameter set 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcZrptr2(pub u32);
impl PkcZrptr2 {
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST"]
    #[must_use]
    #[inline(always)]
    pub const fn zptr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of Z operand in PKCRAM with byte granularity or constant for calculation modes using CONST"]
    #[inline(always)]
    pub const fn set_zptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity"]
    #[must_use]
    #[inline(always)]
    pub const fn rptr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Start address of R result in PKCRAM with byte granularity"]
    #[inline(always)]
    pub const fn set_rptr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PkcZrptr2 {
    #[inline(always)]
    fn default() -> PkcZrptr2 {
        PkcZrptr2(0)
    }
}
impl core::fmt::Debug for PkcZrptr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PkcZrptr2")
            .field("zptr", &self.zptr())
            .field("rptr", &self.rptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PkcZrptr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PkcZrptr2 {{ zptr: {=u16:?}, rptr: {=u16:?} }}",
            self.zptr(),
            self.rptr()
        )
    }
}
