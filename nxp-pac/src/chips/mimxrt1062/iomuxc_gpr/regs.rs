#[doc = "GPR1 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr1(pub u32);
impl Gpr1 {
    #[doc = "SAI1 MCLK1 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_mclk1_sel(&self) -> super::vals::Sai1Mclk1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sai1Mclk1Sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK1 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk1_sel(&mut self, val: super::vals::Sai1Mclk1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SAI1 MCLK2 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_mclk2_sel(&self) -> super::vals::Sai1Mclk2Sel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sai1Mclk2Sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK2 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk2_sel(&mut self, val: super::vals::Sai1Mclk2Sel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "SAI1 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_mclk3_sel(&self) -> super::vals::Sai1Mclk3Sel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sai1Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI1 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai1_mclk3_sel(&mut self, val: super::vals::Sai1Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "SAI2 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_mclk3_sel(&self) -> super::vals::Sai2Mclk3Sel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Sai2Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI2 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai2_mclk3_sel(&mut self, val: super::vals::Sai2Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "SAI3 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_mclk3_sel(&self) -> super::vals::Sai3Mclk3Sel {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Sai3Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI3 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai3_mclk3_sel(&mut self, val: super::vals::Sai3Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Global Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn gint(&self) -> super::vals::Gint {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Gint::from_bits(val as u8)
    }
    #[doc = "Global Interrupt"]
    #[inline(always)]
    pub const fn set_gint(&mut self, val: super::vals::Gint) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "ENET1 reference clock mode select."]
    #[must_use]
    #[inline(always)]
    pub const fn enet1_clk_sel(&self) -> super::vals::Enet1ClkSel {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Enet1ClkSel::from_bits(val as u8)
    }
    #[doc = "ENET1 reference clock mode select."]
    #[inline(always)]
    pub const fn set_enet1_clk_sel(&mut self, val: super::vals::Enet1ClkSel) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "ENET2 reference clock mode select."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_clk_sel(&self) -> super::vals::Enet2ClkSel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Enet2ClkSel::from_bits(val as u8)
    }
    #[doc = "ENET2 reference clock mode select."]
    #[inline(always)]
    pub const fn set_enet2_clk_sel(&mut self, val: super::vals::Enet2ClkSel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "ENET1_TX_CLK data direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn enet1_tx_clk_dir(&self) -> super::vals::Enet1TxClkDir {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Enet1TxClkDir::from_bits(val as u8)
    }
    #[doc = "ENET1_TX_CLK data direction control"]
    #[inline(always)]
    pub const fn set_enet1_tx_clk_dir(&mut self, val: super::vals::Enet1TxClkDir) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "ENET2_TX_CLK data direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_tx_clk_dir(&self) -> super::vals::Enet2TxClkDir {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Enet2TxClkDir::from_bits(val as u8)
    }
    #[doc = "ENET2_TX_CLK data direction control"]
    #[inline(always)]
    pub const fn set_enet2_tx_clk_dir(&mut self, val: super::vals::Enet2TxClkDir) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "sai1.MCLK signal direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_mclk_dir(&self) -> super::vals::Sai1MclkDir {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Sai1MclkDir::from_bits(val as u8)
    }
    #[doc = "sai1.MCLK signal direction control"]
    #[inline(always)]
    pub const fn set_sai1_mclk_dir(&mut self, val: super::vals::Sai1MclkDir) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "sai2.MCLK signal direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_mclk_dir(&self) -> super::vals::Sai2MclkDir {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Sai2MclkDir::from_bits(val as u8)
    }
    #[doc = "sai2.MCLK signal direction control"]
    #[inline(always)]
    pub const fn set_sai2_mclk_dir(&mut self, val: super::vals::Sai2MclkDir) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "sai3.MCLK signal direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_mclk_dir(&self) -> super::vals::Sai3MclkDir {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sai3MclkDir::from_bits(val as u8)
    }
    #[doc = "sai3.MCLK signal direction control"]
    #[inline(always)]
    pub const fn set_sai3_mclk_dir(&mut self, val: super::vals::Sai3MclkDir) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    #[must_use]
    #[inline(always)]
    pub const fn exc_mon(&self) -> super::vals::ExcMon {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::ExcMon::from_bits(val as u8)
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub const fn set_exc_mon(&mut self, val: super::vals::ExcMon) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "ENET and ENET2 ipg_clk_s clock gating enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_ipg_clk_s_en(&self) -> super::vals::EnetIpgClkSEn {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::EnetIpgClkSEn::from_bits(val as u8)
    }
    #[doc = "ENET and ENET2 ipg_clk_s clock gating enable"]
    #[inline(always)]
    pub const fn set_enet_ipg_clk_s_en(&mut self, val: super::vals::EnetIpgClkSEn) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_force_hclk_en(&self) -> super::vals::Cm7ForceHclkEn {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Cm7ForceHclkEn::from_bits(val as u8)
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    pub const fn set_cm7_force_hclk_en(&mut self, val: super::vals::Cm7ForceHclkEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr1 {
    #[inline(always)]
    fn default() -> Gpr1 {
        Gpr1(0)
    }
}
impl core::fmt::Debug for Gpr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr1")
            .field("sai1_mclk1_sel", &self.sai1_mclk1_sel())
            .field("sai1_mclk2_sel", &self.sai1_mclk2_sel())
            .field("sai1_mclk3_sel", &self.sai1_mclk3_sel())
            .field("sai2_mclk3_sel", &self.sai2_mclk3_sel())
            .field("sai3_mclk3_sel", &self.sai3_mclk3_sel())
            .field("gint", &self.gint())
            .field("enet1_clk_sel", &self.enet1_clk_sel())
            .field("enet2_clk_sel", &self.enet2_clk_sel())
            .field("enet1_tx_clk_dir", &self.enet1_tx_clk_dir())
            .field("enet2_tx_clk_dir", &self.enet2_tx_clk_dir())
            .field("sai1_mclk_dir", &self.sai1_mclk_dir())
            .field("sai2_mclk_dir", &self.sai2_mclk_dir())
            .field("sai3_mclk_dir", &self.sai3_mclk_dir())
            .field("exc_mon", &self.exc_mon())
            .field("enet_ipg_clk_s_en", &self.enet_ipg_clk_s_en())
            .field("cm7_force_hclk_en", &self.cm7_force_hclk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr1 {{ sai1_mclk1_sel: {:?}, sai1_mclk2_sel: {:?}, sai1_mclk3_sel: {:?}, sai2_mclk3_sel: {:?}, sai3_mclk3_sel: {:?}, gint: {:?}, enet1_clk_sel: {:?}, enet2_clk_sel: {:?}, enet1_tx_clk_dir: {:?}, enet2_tx_clk_dir: {:?}, sai1_mclk_dir: {:?}, sai2_mclk_dir: {:?}, sai3_mclk_dir: {:?}, exc_mon: {:?}, enet_ipg_clk_s_en: {:?}, cm7_force_hclk_en: {:?} }}",
            self.sai1_mclk1_sel(),
            self.sai1_mclk2_sel(),
            self.sai1_mclk3_sel(),
            self.sai2_mclk3_sel(),
            self.sai3_mclk3_sel(),
            self.gint(),
            self.enet1_clk_sel(),
            self.enet2_clk_sel(),
            self.enet1_tx_clk_dir(),
            self.enet2_tx_clk_dir(),
            self.sai1_mclk_dir(),
            self.sai2_mclk_dir(),
            self.sai3_mclk_dir(),
            self.exc_mon(),
            self.enet_ipg_clk_s_en(),
            self.cm7_force_hclk_en()
        )
    }
}
#[doc = "GPR10 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr10(pub u32);
impl Gpr10 {
    #[doc = "Arm non-secure (non-invasive) debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn niden(&self) -> super::vals::Niden {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Niden::from_bits(val as u8)
    }
    #[doc = "Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: super::vals::Niden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Arm invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> super::vals::DbgEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DbgEn::from_bits(val as u8)
    }
    #[doc = "Arm invasive debug enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: super::vals::DbgEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_err_resp(&self) -> super::vals::SecErrResp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SecErrResp::from_bits(val as u8)
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub const fn set_sec_err_resp(&mut self, val: super::vals::SecErrResp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DCP Key selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dcpkey_ocotp_or_keymux(&self) -> super::vals::DcpkeyOcotpOrKeymux {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DcpkeyOcotpOrKeymux::from_bits(val as u8)
    }
    #[doc = "DCP Key selection bit."]
    #[inline(always)]
    pub const fn set_dcpkey_ocotp_or_keymux(&mut self, val: super::vals::DcpkeyOcotpOrKeymux) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_tz_en(&self) -> super::vals::OcramTzEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::OcramTzEn::from_bits(val as u8)
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub const fn set_ocram_tz_en(&mut self, val: super::vals::OcramTzEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_tz_addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub const fn set_ocram_tz_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Lock NIDEN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_niden(&self) -> super::vals::LockNiden {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LockNiden::from_bits(val as u8)
    }
    #[doc = "Lock NIDEN field for changes"]
    #[inline(always)]
    pub const fn set_lock_niden(&mut self, val: super::vals::LockNiden) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_dbg_en(&self) -> super::vals::LockDbgEn {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::LockDbgEn::from_bits(val as u8)
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_dbg_en(&mut self, val: super::vals::LockDbgEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sec_err_resp(&self) -> super::vals::LockSecErrResp {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::LockSecErrResp::from_bits(val as u8)
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub const fn set_lock_sec_err_resp(&mut self, val: super::vals::LockSecErrResp) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_dcpkey_ocotp_or_keymux(&self) -> super::vals::LockDcpkeyOcotpOrKeymux {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::LockDcpkeyOcotpOrKeymux::from_bits(val as u8)
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub const fn set_lock_dcpkey_ocotp_or_keymux(
        &mut self,
        val: super::vals::LockDcpkeyOcotpOrKeymux,
    ) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram_tz_en(&self) -> super::vals::LockOcramTzEn {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::LockOcramTzEn::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_en(&mut self, val: super::vals::LockOcramTzEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram_tz_addr(&self) -> super::vals::LockOcramTzAddr {
        let val = (self.0 >> 25usize) & 0x7f;
        super::vals::LockOcramTzAddr::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_addr(&mut self, val: super::vals::LockOcramTzAddr) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val.to_bits() as u32) & 0x7f) << 25usize);
    }
}
impl Default for Gpr10 {
    #[inline(always)]
    fn default() -> Gpr10 {
        Gpr10(0)
    }
}
impl core::fmt::Debug for Gpr10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr10")
            .field("niden", &self.niden())
            .field("dbg_en", &self.dbg_en())
            .field("sec_err_resp", &self.sec_err_resp())
            .field("dcpkey_ocotp_or_keymux", &self.dcpkey_ocotp_or_keymux())
            .field("ocram_tz_en", &self.ocram_tz_en())
            .field("ocram_tz_addr", &self.ocram_tz_addr())
            .field("lock_niden", &self.lock_niden())
            .field("lock_dbg_en", &self.lock_dbg_en())
            .field("lock_sec_err_resp", &self.lock_sec_err_resp())
            .field(
                "lock_dcpkey_ocotp_or_keymux",
                &self.lock_dcpkey_ocotp_or_keymux(),
            )
            .field("lock_ocram_tz_en", &self.lock_ocram_tz_en())
            .field("lock_ocram_tz_addr", &self.lock_ocram_tz_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr10 {{ niden: {:?}, dbg_en: {:?}, sec_err_resp: {:?}, dcpkey_ocotp_or_keymux: {:?}, ocram_tz_en: {:?}, ocram_tz_addr: {=u8:?}, lock_niden: {:?}, lock_dbg_en: {:?}, lock_sec_err_resp: {:?}, lock_dcpkey_ocotp_or_keymux: {:?}, lock_ocram_tz_en: {:?}, lock_ocram_tz_addr: {:?} }}",
            self.niden(),
            self.dbg_en(),
            self.sec_err_resp(),
            self.dcpkey_ocotp_or_keymux(),
            self.ocram_tz_en(),
            self.ocram_tz_addr(),
            self.lock_niden(),
            self.lock_dbg_en(),
            self.lock_sec_err_resp(),
            self.lock_dcpkey_ocotp_or_keymux(),
            self.lock_ocram_tz_en(),
            self.lock_ocram_tz_addr()
        )
    }
}
#[doc = "GPR11 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr11(pub u32);
impl Gpr11 {
    #[doc = "Access control of memory region-0"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_ctrl(&self) -> super::vals::M7ApcAcR0Ctrl {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::M7ApcAcR0Ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_ctrl(&mut self, val: super::vals::M7ApcAcR0Ctrl) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Access control of memory region-1"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_ctrl(&self) -> super::vals::M7ApcAcR1Ctrl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::M7ApcAcR1Ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_ctrl(&mut self, val: super::vals::M7ApcAcR1Ctrl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Access control of memory region-2"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_ctrl(&self) -> super::vals::M7ApcAcR2Ctrl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::M7ApcAcR2Ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_ctrl(&mut self, val: super::vals::M7ApcAcR2Ctrl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Access control of memory region-3"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_ctrl(&self) -> super::vals::M7ApcAcR3Ctrl {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::M7ApcAcR3Ctrl::from_bits(val as u8)
    }
    #[doc = "Access control of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_ctrl(&mut self, val: super::vals::M7ApcAcR3Ctrl) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "BEE data decryption of memory region-n (n = 3 to 0)"]
    #[must_use]
    #[inline(always)]
    pub const fn bee_de_rx_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "BEE data decryption of memory region-n (n = 3 to 0)"]
    #[inline(always)]
    pub const fn set_bee_de_rx_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Gpr11 {
    #[inline(always)]
    fn default() -> Gpr11 {
        Gpr11(0)
    }
}
impl core::fmt::Debug for Gpr11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr11")
            .field("m7_apc_ac_r0_ctrl", &self.m7_apc_ac_r0_ctrl())
            .field("m7_apc_ac_r1_ctrl", &self.m7_apc_ac_r1_ctrl())
            .field("m7_apc_ac_r2_ctrl", &self.m7_apc_ac_r2_ctrl())
            .field("m7_apc_ac_r3_ctrl", &self.m7_apc_ac_r3_ctrl())
            .field("bee_de_rx_en", &self.bee_de_rx_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr11 {{ m7_apc_ac_r0_ctrl: {:?}, m7_apc_ac_r1_ctrl: {:?}, m7_apc_ac_r2_ctrl: {:?}, m7_apc_ac_r3_ctrl: {:?}, bee_de_rx_en: {=u8:?} }}",
            self.m7_apc_ac_r0_ctrl(),
            self.m7_apc_ac_r1_ctrl(),
            self.m7_apc_ac_r2_ctrl(),
            self.m7_apc_ac_r3_ctrl(),
            self.bee_de_rx_en()
        )
    }
}
#[doc = "GPR12 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr12(pub u32);
impl Gpr12 {
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_ipg_stop_mode(&self) -> super::vals::Flexio1IpgStopMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Flexio1IpgStopMode::from_bits(val as u8)
    }
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_flexio1_ipg_stop_mode(&mut self, val: super::vals::Flexio1IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_ipg_doze(&self) -> super::vals::Flexio1IpgDoze {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Flexio1IpgDoze::from_bits(val as u8)
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_flexio1_ipg_doze(&mut self, val: super::vals::Flexio1IpgDoze) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_ipg_stop_mode(&self) -> super::vals::Flexio2IpgStopMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Flexio2IpgStopMode::from_bits(val as u8)
    }
    #[doc = "FlexIO2 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_flexio2_ipg_stop_mode(&mut self, val: super::vals::Flexio2IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXIO2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_ipg_doze(&self) -> super::vals::Flexio2IpgDoze {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Flexio2IpgDoze::from_bits(val as u8)
    }
    #[doc = "FLEXIO2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_flexio2_ipg_doze(&mut self, val: super::vals::Flexio2IpgDoze) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn acmp_ipg_stop_mode(&self) -> super::vals::AcmpIpgStopMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AcmpIpgStopMode::from_bits(val as u8)
    }
    #[doc = "ACMP stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_acmp_ipg_stop_mode(&mut self, val: super::vals::AcmpIpgStopMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio3_ipg_stop_mode(&self) -> super::vals::Flexio3IpgStopMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Flexio3IpgStopMode::from_bits(val as u8)
    }
    #[doc = "FlexIO3 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_flexio3_ipg_stop_mode(&mut self, val: super::vals::Flexio3IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXIO3 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio3_ipg_doze(&self) -> super::vals::Flexio3IpgDoze {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Flexio3IpgDoze::from_bits(val as u8)
    }
    #[doc = "FLEXIO3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_flexio3_ipg_doze(&mut self, val: super::vals::Flexio3IpgDoze) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Gpr12 {
    #[inline(always)]
    fn default() -> Gpr12 {
        Gpr12(0)
    }
}
impl core::fmt::Debug for Gpr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr12")
            .field("flexio1_ipg_stop_mode", &self.flexio1_ipg_stop_mode())
            .field("flexio1_ipg_doze", &self.flexio1_ipg_doze())
            .field("flexio2_ipg_stop_mode", &self.flexio2_ipg_stop_mode())
            .field("flexio2_ipg_doze", &self.flexio2_ipg_doze())
            .field("acmp_ipg_stop_mode", &self.acmp_ipg_stop_mode())
            .field("flexio3_ipg_stop_mode", &self.flexio3_ipg_stop_mode())
            .field("flexio3_ipg_doze", &self.flexio3_ipg_doze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr12 {{ flexio1_ipg_stop_mode: {:?}, flexio1_ipg_doze: {:?}, flexio2_ipg_stop_mode: {:?}, flexio2_ipg_doze: {:?}, acmp_ipg_stop_mode: {:?}, flexio3_ipg_stop_mode: {:?}, flexio3_ipg_doze: {:?} }}",
            self.flexio1_ipg_stop_mode(),
            self.flexio1_ipg_doze(),
            self.flexio2_ipg_stop_mode(),
            self.flexio2_ipg_doze(),
            self.acmp_ipg_stop_mode(),
            self.flexio3_ipg_stop_mode(),
            self.flexio3_ipg_doze()
        )
    }
}
#[doc = "GPR13 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr13(pub u32);
impl Gpr13 {
    #[doc = "uSDHC block cacheable attribute value of AXI read transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn arcache_usdhc(&self) -> super::vals::ArcacheUsdhc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ArcacheUsdhc::from_bits(val as u8)
    }
    #[doc = "uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub const fn set_arcache_usdhc(&mut self, val: super::vals::ArcacheUsdhc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "uSDHC block cacheable attribute value of AXI write transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn awcache_usdhc(&self) -> super::vals::AwcacheUsdhc {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AwcacheUsdhc::from_bits(val as u8)
    }
    #[doc = "uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub const fn set_awcache_usdhc(&mut self, val: super::vals::AwcacheUsdhc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CANFD stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn canfd_stop_req(&self) -> super::vals::CanfdStopReq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::CanfdStopReq::from_bits(val as u8)
    }
    #[doc = "CANFD stop request."]
    #[inline(always)]
    pub const fn set_canfd_stop_req(&mut self, val: super::vals::CanfdStopReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ENET block cacheable attribute value of AXI transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_enet(&self) -> super::vals::CacheEnet {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::CacheEnet::from_bits(val as u8)
    }
    #[doc = "ENET block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub const fn set_cache_enet(&mut self, val: super::vals::CacheEnet) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_usb(&self) -> super::vals::CacheUsb {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::CacheUsb::from_bits(val as u8)
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub const fn set_cache_usb(&mut self, val: super::vals::CacheUsb) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "CANFD stop acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn canfd_stop_ack(&self) -> super::vals::CanfdStopAck {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::CanfdStopAck::from_bits(val as u8)
    }
    #[doc = "CANFD stop acknowledge."]
    #[inline(always)]
    pub const fn set_canfd_stop_ack(&mut self, val: super::vals::CanfdStopAck) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Gpr13 {
    #[inline(always)]
    fn default() -> Gpr13 {
        Gpr13(0)
    }
}
impl core::fmt::Debug for Gpr13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr13")
            .field("arcache_usdhc", &self.arcache_usdhc())
            .field("awcache_usdhc", &self.awcache_usdhc())
            .field("canfd_stop_req", &self.canfd_stop_req())
            .field("cache_enet", &self.cache_enet())
            .field("cache_usb", &self.cache_usb())
            .field("canfd_stop_ack", &self.canfd_stop_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr13 {{ arcache_usdhc: {:?}, awcache_usdhc: {:?}, canfd_stop_req: {:?}, cache_enet: {:?}, cache_usb: {:?}, canfd_stop_ack: {:?} }}",
            self.arcache_usdhc(),
            self.awcache_usdhc(),
            self.canfd_stop_req(),
            self.cache_enet(),
            self.cache_usb(),
            self.canfd_stop_ack()
        )
    }
}
#[doc = "GPR14 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr14(pub u32);
impl Gpr14 {
    #[doc = "reduces ACMP1 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1_cmp_igen_trim_dn(&self) -> super::vals::Acmp1CmpIgenTrimDn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Acmp1CmpIgenTrimDn::from_bits(val as u8)
    }
    #[doc = "reduces ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp1_cmp_igen_trim_dn(&mut self, val: super::vals::Acmp1CmpIgenTrimDn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "reduces ACMP2 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2_cmp_igen_trim_dn(&self) -> super::vals::Acmp2CmpIgenTrimDn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Acmp2CmpIgenTrimDn::from_bits(val as u8)
    }
    #[doc = "reduces ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp2_cmp_igen_trim_dn(&mut self, val: super::vals::Acmp2CmpIgenTrimDn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "reduces ACMP3 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3_cmp_igen_trim_dn(&self) -> super::vals::Acmp3CmpIgenTrimDn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Acmp3CmpIgenTrimDn::from_bits(val as u8)
    }
    #[doc = "reduces ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp3_cmp_igen_trim_dn(&mut self, val: super::vals::Acmp3CmpIgenTrimDn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "reduces ACMP4 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp4_cmp_igen_trim_dn(&self) -> super::vals::Acmp4CmpIgenTrimDn {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Acmp4CmpIgenTrimDn::from_bits(val as u8)
    }
    #[doc = "reduces ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp4_cmp_igen_trim_dn(&mut self, val: super::vals::Acmp4CmpIgenTrimDn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "increases ACMP1 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1_cmp_igen_trim_up(&self) -> super::vals::Acmp1CmpIgenTrimUp {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Acmp1CmpIgenTrimUp::from_bits(val as u8)
    }
    #[doc = "increases ACMP1 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp1_cmp_igen_trim_up(&mut self, val: super::vals::Acmp1CmpIgenTrimUp) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "increases ACMP2 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2_cmp_igen_trim_up(&self) -> super::vals::Acmp2CmpIgenTrimUp {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Acmp2CmpIgenTrimUp::from_bits(val as u8)
    }
    #[doc = "increases ACMP2 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp2_cmp_igen_trim_up(&mut self, val: super::vals::Acmp2CmpIgenTrimUp) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "increases ACMP3 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3_cmp_igen_trim_up(&self) -> super::vals::Acmp3CmpIgenTrimUp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Acmp3CmpIgenTrimUp::from_bits(val as u8)
    }
    #[doc = "increases ACMP3 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp3_cmp_igen_trim_up(&mut self, val: super::vals::Acmp3CmpIgenTrimUp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "increases ACMP4 internal bias current by 30%"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp4_cmp_igen_trim_up(&self) -> super::vals::Acmp4CmpIgenTrimUp {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Acmp4CmpIgenTrimUp::from_bits(val as u8)
    }
    #[doc = "increases ACMP4 internal bias current by 30%"]
    #[inline(always)]
    pub const fn set_acmp4_cmp_igen_trim_up(&mut self, val: super::vals::Acmp4CmpIgenTrimUp) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "ACMP1 sample_lv source select"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp1_sample_sync_en(&self) -> super::vals::Acmp1SampleSyncEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Acmp1SampleSyncEn::from_bits(val as u8)
    }
    #[doc = "ACMP1 sample_lv source select"]
    #[inline(always)]
    pub const fn set_acmp1_sample_sync_en(&mut self, val: super::vals::Acmp1SampleSyncEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "ACMP2 sample_lv source select"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp2_sample_sync_en(&self) -> super::vals::Acmp2SampleSyncEn {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Acmp2SampleSyncEn::from_bits(val as u8)
    }
    #[doc = "ACMP2 sample_lv source select"]
    #[inline(always)]
    pub const fn set_acmp2_sample_sync_en(&mut self, val: super::vals::Acmp2SampleSyncEn) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "ACMP3 sample_lv source select"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp3_sample_sync_en(&self) -> super::vals::Acmp3SampleSyncEn {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Acmp3SampleSyncEn::from_bits(val as u8)
    }
    #[doc = "ACMP3 sample_lv source select"]
    #[inline(always)]
    pub const fn set_acmp3_sample_sync_en(&mut self, val: super::vals::Acmp3SampleSyncEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ACMP4 sample_lv source select"]
    #[must_use]
    #[inline(always)]
    pub const fn acmp4_sample_sync_en(&self) -> super::vals::Acmp4SampleSyncEn {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Acmp4SampleSyncEn::from_bits(val as u8)
    }
    #[doc = "ACMP4 sample_lv source select"]
    #[inline(always)]
    pub const fn set_acmp4_sample_sync_en(&mut self, val: super::vals::Acmp4SampleSyncEn) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Gpr14 {
    #[inline(always)]
    fn default() -> Gpr14 {
        Gpr14(0)
    }
}
impl core::fmt::Debug for Gpr14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr14")
            .field("acmp1_cmp_igen_trim_dn", &self.acmp1_cmp_igen_trim_dn())
            .field("acmp2_cmp_igen_trim_dn", &self.acmp2_cmp_igen_trim_dn())
            .field("acmp3_cmp_igen_trim_dn", &self.acmp3_cmp_igen_trim_dn())
            .field("acmp4_cmp_igen_trim_dn", &self.acmp4_cmp_igen_trim_dn())
            .field("acmp1_cmp_igen_trim_up", &self.acmp1_cmp_igen_trim_up())
            .field("acmp2_cmp_igen_trim_up", &self.acmp2_cmp_igen_trim_up())
            .field("acmp3_cmp_igen_trim_up", &self.acmp3_cmp_igen_trim_up())
            .field("acmp4_cmp_igen_trim_up", &self.acmp4_cmp_igen_trim_up())
            .field("acmp1_sample_sync_en", &self.acmp1_sample_sync_en())
            .field("acmp2_sample_sync_en", &self.acmp2_sample_sync_en())
            .field("acmp3_sample_sync_en", &self.acmp3_sample_sync_en())
            .field("acmp4_sample_sync_en", &self.acmp4_sample_sync_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr14 {{ acmp1_cmp_igen_trim_dn: {:?}, acmp2_cmp_igen_trim_dn: {:?}, acmp3_cmp_igen_trim_dn: {:?}, acmp4_cmp_igen_trim_dn: {:?}, acmp1_cmp_igen_trim_up: {:?}, acmp2_cmp_igen_trim_up: {:?}, acmp3_cmp_igen_trim_up: {:?}, acmp4_cmp_igen_trim_up: {:?}, acmp1_sample_sync_en: {:?}, acmp2_sample_sync_en: {:?}, acmp3_sample_sync_en: {:?}, acmp4_sample_sync_en: {:?} }}",
            self.acmp1_cmp_igen_trim_dn(),
            self.acmp2_cmp_igen_trim_dn(),
            self.acmp3_cmp_igen_trim_dn(),
            self.acmp4_cmp_igen_trim_dn(),
            self.acmp1_cmp_igen_trim_up(),
            self.acmp2_cmp_igen_trim_up(),
            self.acmp3_cmp_igen_trim_up(),
            self.acmp4_cmp_igen_trim_up(),
            self.acmp1_sample_sync_en(),
            self.acmp2_sample_sync_en(),
            self.acmp3_sample_sync_en(),
            self.acmp4_sample_sync_en()
        )
    }
}
#[doc = "GPR16 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr16(pub u32);
impl Gpr16 {
    #[doc = "FlexRAM bank config source select"]
    #[must_use]
    #[inline(always)]
    pub const fn flexram_bank_cfg_sel(&self) -> super::vals::FlexramBankCfgSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::FlexramBankCfgSel::from_bits(val as u8)
    }
    #[doc = "FlexRAM bank config source select"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg_sel(&mut self, val: super::vals::FlexramBankCfgSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Vector table offset register out of reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_init_vtor(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Vector table offset register out of reset"]
    #[inline(always)]
    pub const fn set_cm7_init_vtor(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Gpr16 {
    #[inline(always)]
    fn default() -> Gpr16 {
        Gpr16(0)
    }
}
impl core::fmt::Debug for Gpr16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr16")
            .field("flexram_bank_cfg_sel", &self.flexram_bank_cfg_sel())
            .field("cm7_init_vtor", &self.cm7_init_vtor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr16 {{ flexram_bank_cfg_sel: {:?}, cm7_init_vtor: {=u32:?} }}",
            self.flexram_bank_cfg_sel(),
            self.cm7_init_vtor()
        )
    }
}
#[doc = "GPR17 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr17(pub u32);
impl Gpr17 {
    #[doc = "FlexRAM bank config value"]
    #[must_use]
    #[inline(always)]
    pub const fn flexram_bank_cfg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "FlexRAM bank config value"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpr17 {
    #[inline(always)]
    fn default() -> Gpr17 {
        Gpr17(0)
    }
}
impl core::fmt::Debug for Gpr17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr17")
            .field("flexram_bank_cfg", &self.flexram_bank_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr17 {{ flexram_bank_cfg: {=u32:?} }}",
            self.flexram_bank_cfg()
        )
    }
}
#[doc = "GPR18 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr18(pub u32);
impl Gpr18 {
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_bot(&self) -> super::vals::LockM7ApcAcR0Bot {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR0Bot::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_bot(&mut self, val: super::vals::LockM7ApcAcR0Bot) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-0"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr18 {
    #[inline(always)]
    fn default() -> Gpr18 {
        Gpr18(0)
    }
}
impl core::fmt::Debug for Gpr18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr18")
            .field("lock_m7_apc_ac_r0_bot", &self.lock_m7_apc_ac_r0_bot())
            .field("m7_apc_ac_r0_bot", &self.m7_apc_ac_r0_bot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr18 {{ lock_m7_apc_ac_r0_bot: {:?}, m7_apc_ac_r0_bot: {=u32:?} }}",
            self.lock_m7_apc_ac_r0_bot(),
            self.m7_apc_ac_r0_bot()
        )
    }
}
#[doc = "GPR19 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr19(pub u32);
impl Gpr19 {
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_top(&self) -> super::vals::LockM7ApcAcR0Top {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR0Top::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_top(&mut self, val: super::vals::LockM7ApcAcR0Top) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-0"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r0_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-0"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r0_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr19 {
    #[inline(always)]
    fn default() -> Gpr19 {
        Gpr19(0)
    }
}
impl core::fmt::Debug for Gpr19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr19")
            .field("lock_m7_apc_ac_r0_top", &self.lock_m7_apc_ac_r0_top())
            .field("m7_apc_ac_r0_top", &self.m7_apc_ac_r0_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr19 {{ lock_m7_apc_ac_r0_top: {:?}, m7_apc_ac_r0_top: {=u32:?} }}",
            self.lock_m7_apc_ac_r0_top(),
            self.m7_apc_ac_r0_top()
        )
    }
}
#[doc = "GPR2 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr2(pub u32);
impl Gpr2 {
    #[doc = "AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_l_ahbxl_high_priority(&self) -> super::vals::AxbsLAhbxlHighPriority {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::AxbsLAhbxlHighPriority::from_bits(val as u8)
    }
    #[doc = "AXBS_L AHBXL master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub const fn set_axbs_l_ahbxl_high_priority(
        &mut self,
        val: super::vals::AxbsLAhbxlHighPriority,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_l_dma_high_priority(&self) -> super::vals::AxbsLDmaHighPriority {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::AxbsLDmaHighPriority::from_bits(val as u8)
    }
    #[doc = "AXBS_L DMA master has higher priority.Do not set both DMA and AHBXL to high priority."]
    #[inline(always)]
    pub const fn set_axbs_l_dma_high_priority(&mut self, val: super::vals::AxbsLDmaHighPriority) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Force Round Robin in AXBS_L"]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_l_force_round_robin(&self) -> super::vals::AxbsLForceRoundRobin {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::AxbsLForceRoundRobin::from_bits(val as u8)
    }
    #[doc = "Force Round Robin in AXBS_L"]
    #[inline(always)]
    pub const fn set_axbs_l_force_round_robin(&mut self, val: super::vals::AxbsLForceRoundRobin) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_m0_high_priority(&self) -> super::vals::AxbsPM0HighPriority {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::AxbsPM0HighPriority::from_bits(val as u8)
    }
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m0_high_priority(&mut self, val: super::vals::AxbsPM0HighPriority) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_m1_high_priority(&self) -> super::vals::AxbsPM1HighPriority {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::AxbsPM1HighPriority::from_bits(val as u8)
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m1_high_priority(&mut self, val: super::vals::AxbsPM1HighPriority) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_force_round_robin(&self) -> super::vals::AxbsPForceRoundRobin {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::AxbsPForceRoundRobin::from_bits(val as u8)
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub const fn set_axbs_p_force_round_robin(&mut self, val: super::vals::AxbsPForceRoundRobin) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Disable CANFD filter"]
    #[must_use]
    #[inline(always)]
    pub const fn canfd_filter_bypass(&self) -> super::vals::CanfdFilterBypass {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::CanfdFilterBypass::from_bits(val as u8)
    }
    #[doc = "Disable CANFD filter"]
    #[inline(always)]
    pub const fn set_canfd_filter_bypass(&mut self, val: super::vals::CanfdFilterBypass) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "enable power saving features on L2 memory"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_mem_en_powersaving(&self) -> super::vals::L2MemEnPowersaving {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::L2MemEnPowersaving::from_bits(val as u8)
    }
    #[doc = "enable power saving features on L2 memory"]
    #[inline(always)]
    pub const fn set_l2_mem_en_powersaving(&mut self, val: super::vals::L2MemEnPowersaving) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[must_use]
    #[inline(always)]
    pub const fn ram_auto_clk_gating_en(&self) -> super::vals::RamAutoClkGatingEn {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::RamAutoClkGatingEn::from_bits(val as u8)
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub const fn set_ram_auto_clk_gating_en(&mut self, val: super::vals::RamAutoClkGatingEn) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_mem_deepsleep(&self) -> super::vals::L2MemDeepsleep {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::L2MemDeepsleep::from_bits(val as u8)
    }
    #[doc = "control how memory enter Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low)"]
    #[inline(always)]
    pub const fn set_l2_mem_deepsleep(&mut self, val: super::vals::L2MemDeepsleep) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_clk_div(&self) -> super::vals::MqsClkDiv {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::MqsClkDiv::from_bits(val as u8)
    }
    #[doc = "Divider ratio control for mclk from hmclk. mclk frequency = 1/(n+1) * hmclk frequency."]
    #[inline(always)]
    pub const fn set_mqs_clk_div(&mut self, val: super::vals::MqsClkDiv) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "MQS software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_sw_rst(&self) -> super::vals::MqsSwRst {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::MqsSwRst::from_bits(val as u8)
    }
    #[doc = "MQS software reset"]
    #[inline(always)]
    pub const fn set_mqs_sw_rst(&mut self, val: super::vals::MqsSwRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "MQS enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_en(&self) -> super::vals::MqsEn {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::MqsEn::from_bits(val as u8)
    }
    #[doc = "MQS enable."]
    #[inline(always)]
    pub const fn set_mqs_en(&mut self, val: super::vals::MqsEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Used to control the PWM oversampling rate compared with mclk."]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_oversample(&self) -> super::vals::MqsOversample {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MqsOversample::from_bits(val as u8)
    }
    #[doc = "Used to control the PWM oversampling rate compared with mclk."]
    #[inline(always)]
    pub const fn set_mqs_oversample(&mut self, val: super::vals::MqsOversample) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "QTIMER1 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr_cnts_freeze(&self) -> super::vals::Qtimer1TmrCntsFreeze {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Qtimer1TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER1 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer1TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "QTIMER2 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr_cnts_freeze(&self) -> super::vals::Qtimer2TmrCntsFreeze {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Qtimer2TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER2 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer2TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "QTIMER3 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr_cnts_freeze(&self) -> super::vals::Qtimer3TmrCntsFreeze {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Qtimer3TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER3 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer3TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "QTIMER4 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr_cnts_freeze(&self) -> super::vals::Qtimer4TmrCntsFreeze {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Qtimer4TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER4 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer4TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr2 {
    #[inline(always)]
    fn default() -> Gpr2 {
        Gpr2(0)
    }
}
impl core::fmt::Debug for Gpr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr2")
            .field(
                "axbs_l_ahbxl_high_priority",
                &self.axbs_l_ahbxl_high_priority(),
            )
            .field("axbs_l_dma_high_priority", &self.axbs_l_dma_high_priority())
            .field("axbs_l_force_round_robin", &self.axbs_l_force_round_robin())
            .field("axbs_p_m0_high_priority", &self.axbs_p_m0_high_priority())
            .field("axbs_p_m1_high_priority", &self.axbs_p_m1_high_priority())
            .field("axbs_p_force_round_robin", &self.axbs_p_force_round_robin())
            .field("canfd_filter_bypass", &self.canfd_filter_bypass())
            .field("l2_mem_en_powersaving", &self.l2_mem_en_powersaving())
            .field("ram_auto_clk_gating_en", &self.ram_auto_clk_gating_en())
            .field("l2_mem_deepsleep", &self.l2_mem_deepsleep())
            .field("mqs_clk_div", &self.mqs_clk_div())
            .field("mqs_sw_rst", &self.mqs_sw_rst())
            .field("mqs_en", &self.mqs_en())
            .field("mqs_oversample", &self.mqs_oversample())
            .field("qtimer1_tmr_cnts_freeze", &self.qtimer1_tmr_cnts_freeze())
            .field("qtimer2_tmr_cnts_freeze", &self.qtimer2_tmr_cnts_freeze())
            .field("qtimer3_tmr_cnts_freeze", &self.qtimer3_tmr_cnts_freeze())
            .field("qtimer4_tmr_cnts_freeze", &self.qtimer4_tmr_cnts_freeze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr2 {{ axbs_l_ahbxl_high_priority: {:?}, axbs_l_dma_high_priority: {:?}, axbs_l_force_round_robin: {:?}, axbs_p_m0_high_priority: {:?}, axbs_p_m1_high_priority: {:?}, axbs_p_force_round_robin: {:?}, canfd_filter_bypass: {:?}, l2_mem_en_powersaving: {:?}, ram_auto_clk_gating_en: {:?}, l2_mem_deepsleep: {:?}, mqs_clk_div: {:?}, mqs_sw_rst: {:?}, mqs_en: {:?}, mqs_oversample: {:?}, qtimer1_tmr_cnts_freeze: {:?}, qtimer2_tmr_cnts_freeze: {:?}, qtimer3_tmr_cnts_freeze: {:?}, qtimer4_tmr_cnts_freeze: {:?} }}",
            self.axbs_l_ahbxl_high_priority(),
            self.axbs_l_dma_high_priority(),
            self.axbs_l_force_round_robin(),
            self.axbs_p_m0_high_priority(),
            self.axbs_p_m1_high_priority(),
            self.axbs_p_force_round_robin(),
            self.canfd_filter_bypass(),
            self.l2_mem_en_powersaving(),
            self.ram_auto_clk_gating_en(),
            self.l2_mem_deepsleep(),
            self.mqs_clk_div(),
            self.mqs_sw_rst(),
            self.mqs_en(),
            self.mqs_oversample(),
            self.qtimer1_tmr_cnts_freeze(),
            self.qtimer2_tmr_cnts_freeze(),
            self.qtimer3_tmr_cnts_freeze(),
            self.qtimer4_tmr_cnts_freeze()
        )
    }
}
#[doc = "GPR20 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr20(pub u32);
impl Gpr20 {
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_bot(&self) -> super::vals::LockM7ApcAcR1Bot {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR1Bot::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_bot(&mut self, val: super::vals::LockM7ApcAcR1Bot) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-1"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr20 {
    #[inline(always)]
    fn default() -> Gpr20 {
        Gpr20(0)
    }
}
impl core::fmt::Debug for Gpr20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr20")
            .field("lock_m7_apc_ac_r1_bot", &self.lock_m7_apc_ac_r1_bot())
            .field("m7_apc_ac_r1_bot", &self.m7_apc_ac_r1_bot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr20 {{ lock_m7_apc_ac_r1_bot: {:?}, m7_apc_ac_r1_bot: {=u32:?} }}",
            self.lock_m7_apc_ac_r1_bot(),
            self.m7_apc_ac_r1_bot()
        )
    }
}
#[doc = "GPR21 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr21(pub u32);
impl Gpr21 {
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_top(&self) -> super::vals::LockM7ApcAcR1Top {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR1Top::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_top(&mut self, val: super::vals::LockM7ApcAcR1Top) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-1"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r1_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-1"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r1_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr21 {
    #[inline(always)]
    fn default() -> Gpr21 {
        Gpr21(0)
    }
}
impl core::fmt::Debug for Gpr21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr21")
            .field("lock_m7_apc_ac_r1_top", &self.lock_m7_apc_ac_r1_top())
            .field("m7_apc_ac_r1_top", &self.m7_apc_ac_r1_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr21 {{ lock_m7_apc_ac_r1_top: {:?}, m7_apc_ac_r1_top: {=u32:?} }}",
            self.lock_m7_apc_ac_r1_top(),
            self.m7_apc_ac_r1_top()
        )
    }
}
#[doc = "GPR22 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr22(pub u32);
impl Gpr22 {
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_bot(&self) -> super::vals::LockM7ApcAcR2Bot {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR2Bot::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_bot(&mut self, val: super::vals::LockM7ApcAcR2Bot) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-2"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr22 {
    #[inline(always)]
    fn default() -> Gpr22 {
        Gpr22(0)
    }
}
impl core::fmt::Debug for Gpr22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr22")
            .field("lock_m7_apc_ac_r2_bot", &self.lock_m7_apc_ac_r2_bot())
            .field("m7_apc_ac_r2_bot", &self.m7_apc_ac_r2_bot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr22 {{ lock_m7_apc_ac_r2_bot: {:?}, m7_apc_ac_r2_bot: {=u32:?} }}",
            self.lock_m7_apc_ac_r2_bot(),
            self.m7_apc_ac_r2_bot()
        )
    }
}
#[doc = "GPR23 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr23(pub u32);
impl Gpr23 {
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_top(&self) -> super::vals::LockM7ApcAcR2Top {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR2Top::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_top(&mut self, val: super::vals::LockM7ApcAcR2Top) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-2"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r2_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-2"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r2_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr23 {
    #[inline(always)]
    fn default() -> Gpr23 {
        Gpr23(0)
    }
}
impl core::fmt::Debug for Gpr23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr23")
            .field("lock_m7_apc_ac_r2_top", &self.lock_m7_apc_ac_r2_top())
            .field("m7_apc_ac_r2_top", &self.m7_apc_ac_r2_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr23 {{ lock_m7_apc_ac_r2_top: {:?}, m7_apc_ac_r2_top: {=u32:?} }}",
            self.lock_m7_apc_ac_r2_top(),
            self.m7_apc_ac_r2_top()
        )
    }
}
#[doc = "GPR24 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr24(pub u32);
impl Gpr24 {
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_bot(&self) -> super::vals::LockM7ApcAcR3Bot {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR3Bot::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_bot(&mut self, val: super::vals::LockM7ApcAcR3Bot) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC end address of memory region-3"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_bot(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC end address of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_bot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr24 {
    #[inline(always)]
    fn default() -> Gpr24 {
        Gpr24(0)
    }
}
impl core::fmt::Debug for Gpr24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr24")
            .field("lock_m7_apc_ac_r3_bot", &self.lock_m7_apc_ac_r3_bot())
            .field("m7_apc_ac_r3_bot", &self.m7_apc_ac_r3_bot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr24 {{ lock_m7_apc_ac_r3_bot: {:?}, m7_apc_ac_r3_bot: {=u32:?} }}",
            self.lock_m7_apc_ac_r3_bot(),
            self.m7_apc_ac_r3_bot()
        )
    }
}
#[doc = "GPR25 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr25(pub u32);
impl Gpr25 {
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_top(&self) -> super::vals::LockM7ApcAcR3Top {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LockM7ApcAcR3Top::from_bits(val as u8)
    }
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_top(&mut self, val: super::vals::LockM7ApcAcR3Top) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "APC start address of memory region-3"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_apc_ac_r3_top(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "APC start address of memory region-3"]
    #[inline(always)]
    pub const fn set_m7_apc_ac_r3_top(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Gpr25 {
    #[inline(always)]
    fn default() -> Gpr25 {
        Gpr25(0)
    }
}
impl core::fmt::Debug for Gpr25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr25")
            .field("lock_m7_apc_ac_r3_top", &self.lock_m7_apc_ac_r3_top())
            .field("m7_apc_ac_r3_top", &self.m7_apc_ac_r3_top())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr25 {{ lock_m7_apc_ac_r3_top: {:?}, m7_apc_ac_r3_top: {=u32:?} }}",
            self.lock_m7_apc_ac_r3_top(),
            self.m7_apc_ac_r3_top()
        )
    }
}
#[doc = "GPR26 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr26(pub u32);
impl Gpr26 {
    #[doc = "GPIO1 and GPIO6 share same IO MUX function, GPIO_MUX1 selects one GPIO function."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_mux1_gpio_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO1 and GPIO6 share same IO MUX function, GPIO_MUX1 selects one GPIO function."]
    #[inline(always)]
    pub const fn set_gpio_mux1_gpio_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpr26 {
    #[inline(always)]
    fn default() -> Gpr26 {
        Gpr26(0)
    }
}
impl core::fmt::Debug for Gpr26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr26")
            .field("gpio_mux1_gpio_sel", &self.gpio_mux1_gpio_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr26 {{ gpio_mux1_gpio_sel: {=u32:?} }}",
            self.gpio_mux1_gpio_sel()
        )
    }
}
#[doc = "GPR27 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr27(pub u32);
impl Gpr27 {
    #[doc = "GPIO2 and GPIO7 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_mux2_gpio_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO2 and GPIO7 share same IO MUX function, GPIO_MUX2 selects one GPIO function."]
    #[inline(always)]
    pub const fn set_gpio_mux2_gpio_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpr27 {
    #[inline(always)]
    fn default() -> Gpr27 {
        Gpr27(0)
    }
}
impl core::fmt::Debug for Gpr27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr27")
            .field("gpio_mux2_gpio_sel", &self.gpio_mux2_gpio_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr27 {{ gpio_mux2_gpio_sel: {=u32:?} }}",
            self.gpio_mux2_gpio_sel()
        )
    }
}
#[doc = "GPR28 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr28(pub u32);
impl Gpr28 {
    #[doc = "GPIO3 and GPIO8 share same IO MUX function, GPIO_MUX3 selects one GPIO function."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_mux3_gpio_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO3 and GPIO8 share same IO MUX function, GPIO_MUX3 selects one GPIO function."]
    #[inline(always)]
    pub const fn set_gpio_mux3_gpio_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpr28 {
    #[inline(always)]
    fn default() -> Gpr28 {
        Gpr28(0)
    }
}
impl core::fmt::Debug for Gpr28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr28")
            .field("gpio_mux3_gpio_sel", &self.gpio_mux3_gpio_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr28 {{ gpio_mux3_gpio_sel: {=u32:?} }}",
            self.gpio_mux3_gpio_sel()
        )
    }
}
#[doc = "GPR29 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr29(pub u32);
impl Gpr29 {
    #[doc = "GPIO4 and GPIO9 share same IO MUX function, GPIO_MUX4 selects one GPIO function."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_mux4_gpio_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "GPIO4 and GPIO9 share same IO MUX function, GPIO_MUX4 selects one GPIO function."]
    #[inline(always)]
    pub const fn set_gpio_mux4_gpio_sel(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpr29 {
    #[inline(always)]
    fn default() -> Gpr29 {
        Gpr29(0)
    }
}
impl core::fmt::Debug for Gpr29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr29")
            .field("gpio_mux4_gpio_sel", &self.gpio_mux4_gpio_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr29 {{ gpio_mux4_gpio_sel: {=u32:?} }}",
            self.gpio_mux4_gpio_sel()
        )
    }
}
#[doc = "GPR3 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr3(pub u32);
impl Gpr3 {
    #[doc = "OCRAM_CTL\\[3\\] - write address pipeline control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_ctl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "OCRAM_CTL\\[3\\] - write address pipeline control bit"]
    #[inline(always)]
    pub const fn set_ocram_ctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Select 128-bit dcp key from 256-bit key from SNVS Master Key"]
    #[must_use]
    #[inline(always)]
    pub const fn dcp_key_sel(&self) -> super::vals::DcpKeySel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DcpKeySel::from_bits(val as u8)
    }
    #[doc = "Select 128-bit dcp key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    pub const fn set_dcp_key_sel(&mut self, val: super::vals::DcpKeySel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM2_CTL\\[3\\] - write address pipeline control bit"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram2_ctl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "OCRAM2_CTL\\[3\\] - write address pipeline control bit"]
    #[inline(always)]
    pub const fn set_ocram2_ctl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Request to halt axbs_l"]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_l_halt_req(&self) -> super::vals::AxbsLHaltReq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::AxbsLHaltReq::from_bits(val as u8)
    }
    #[doc = "Request to halt axbs_l"]
    #[inline(always)]
    pub const fn set_axbs_l_halt_req(&mut self, val: super::vals::AxbsLHaltReq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "This field shows the OCRAM pipeline settings status, controlled by OCRAM_CTL bits respectively"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_status(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "This field shows the OCRAM pipeline settings status, controlled by OCRAM_CTL bits respectively"]
    #[inline(always)]
    pub const fn set_ocram_status(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "This field shows the OCRAM2 pipeline settings status, controlled by OCRAM2_CTL bits respectively"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram2_status(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "This field shows the OCRAM2 pipeline settings status, controlled by OCRAM2_CTL bits respectively"]
    #[inline(always)]
    pub const fn set_ocram2_status(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "This bit shows the status of axbs_l"]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_l_halted(&self) -> super::vals::AxbsLHalted {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::AxbsLHalted::from_bits(val as u8)
    }
    #[doc = "This bit shows the status of axbs_l"]
    #[inline(always)]
    pub const fn set_axbs_l_halted(&mut self, val: super::vals::AxbsLHalted) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr3 {
    #[inline(always)]
    fn default() -> Gpr3 {
        Gpr3(0)
    }
}
impl core::fmt::Debug for Gpr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr3")
            .field("ocram_ctl", &self.ocram_ctl())
            .field("dcp_key_sel", &self.dcp_key_sel())
            .field("ocram2_ctl", &self.ocram2_ctl())
            .field("axbs_l_halt_req", &self.axbs_l_halt_req())
            .field("ocram_status", &self.ocram_status())
            .field("ocram2_status", &self.ocram2_status())
            .field("axbs_l_halted", &self.axbs_l_halted())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr3 {{ ocram_ctl: {=u8:?}, dcp_key_sel: {:?}, ocram2_ctl: {=u8:?}, axbs_l_halt_req: {:?}, ocram_status: {=u8:?}, ocram2_status: {=u8:?}, axbs_l_halted: {:?} }}",
            self.ocram_ctl(),
            self.dcp_key_sel(),
            self.ocram2_ctl(),
            self.axbs_l_halt_req(),
            self.ocram_status(),
            self.ocram2_status(),
            self.axbs_l_halted()
        )
    }
}
#[doc = "GPR30 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr30(pub u32);
impl Gpr30 {
    #[doc = "Start address of flexspi1 and flexspi2"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_start(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr30 {
    #[inline(always)]
    fn default() -> Gpr30 {
        Gpr30(0)
    }
}
impl core::fmt::Debug for Gpr30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr30")
            .field("flexspi_remap_addr_start", &self.flexspi_remap_addr_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr30 {{ flexspi_remap_addr_start: {=u32:?} }}",
            self.flexspi_remap_addr_start()
        )
    }
}
#[doc = "GPR31 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr31(pub u32);
impl Gpr31 {
    #[doc = "End address of flexspi1 and flexspi2"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_end(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_end(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr31 {
    #[inline(always)]
    fn default() -> Gpr31 {
        Gpr31(0)
    }
}
impl core::fmt::Debug for Gpr31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr31")
            .field("flexspi_remap_addr_end", &self.flexspi_remap_addr_end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr31 {{ flexspi_remap_addr_end: {=u32:?} }}",
            self.flexspi_remap_addr_end()
        )
    }
}
#[doc = "GPR32 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr32(pub u32);
impl Gpr32 {
    #[doc = "Offset address of flexspi1 and flexspi2"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_offset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Offset address of flexspi1 and flexspi2"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Gpr32 {
    #[inline(always)]
    fn default() -> Gpr32 {
        Gpr32(0)
    }
}
impl core::fmt::Debug for Gpr32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr32")
            .field(
                "flexspi_remap_addr_offset",
                &self.flexspi_remap_addr_offset(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr32 {{ flexspi_remap_addr_offset: {=u32:?} }}",
            self.flexspi_remap_addr_offset()
        )
    }
}
#[doc = "GPR33 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr33(pub u32);
impl Gpr33 {
    #[doc = "OCRAM2 TrustZone (TZ) enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ocram2_tz_en(&self) -> super::vals::Ocram2TzEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ocram2TzEn::from_bits(val as u8)
    }
    #[doc = "OCRAM2 TrustZone (TZ) enable."]
    #[inline(always)]
    pub const fn set_ocram2_tz_en(&mut self, val: super::vals::Ocram2TzEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "OCRAM2 TrustZone (TZ) start address"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram2_tz_addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "OCRAM2 TrustZone (TZ) start address"]
    #[inline(always)]
    pub const fn set_ocram2_tz_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Lock OCRAM2_TZ_EN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram2_tz_en(&self) -> super::vals::LockOcram2TzEn {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::LockOcram2TzEn::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM2_TZ_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram2_tz_en(&mut self, val: super::vals::LockOcram2TzEn) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock OCRAM2_TZ_ADDR field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram2_tz_addr(&self) -> super::vals::LockOcram2TzAddr {
        let val = (self.0 >> 17usize) & 0x7f;
        super::vals::LockOcram2TzAddr::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM2_TZ_ADDR field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram2_tz_addr(&mut self, val: super::vals::LockOcram2TzAddr) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val.to_bits() as u32) & 0x7f) << 17usize);
    }
}
impl Default for Gpr33 {
    #[inline(always)]
    fn default() -> Gpr33 {
        Gpr33(0)
    }
}
impl core::fmt::Debug for Gpr33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr33")
            .field("ocram2_tz_en", &self.ocram2_tz_en())
            .field("ocram2_tz_addr", &self.ocram2_tz_addr())
            .field("lock_ocram2_tz_en", &self.lock_ocram2_tz_en())
            .field("lock_ocram2_tz_addr", &self.lock_ocram2_tz_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr33 {{ ocram2_tz_en: {:?}, ocram2_tz_addr: {=u8:?}, lock_ocram2_tz_en: {:?}, lock_ocram2_tz_addr: {:?} }}",
            self.ocram2_tz_en(),
            self.ocram2_tz_addr(),
            self.lock_ocram2_tz_en(),
            self.lock_ocram2_tz_addr()
        )
    }
}
#[doc = "GPR34 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr34(pub u32);
impl Gpr34 {
    #[doc = "Boot Pin select in SIP_TEST_MUX"]
    #[must_use]
    #[inline(always)]
    pub const fn sip_test_mux_boot_pin_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Boot Pin select in SIP_TEST_MUX"]
    #[inline(always)]
    pub const fn set_sip_test_mux_boot_pin_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable SIP_TEST_MUX"]
    #[must_use]
    #[inline(always)]
    pub const fn sip_test_mux_qspi_sip_en(&self) -> super::vals::SipTestMuxQspiSipEn {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::SipTestMuxQspiSipEn::from_bits(val as u8)
    }
    #[doc = "Enable SIP_TEST_MUX"]
    #[inline(always)]
    pub const fn set_sip_test_mux_qspi_sip_en(&mut self, val: super::vals::SipTestMuxQspiSipEn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Gpr34 {
    #[inline(always)]
    fn default() -> Gpr34 {
        Gpr34(0)
    }
}
impl core::fmt::Debug for Gpr34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr34")
            .field(
                "sip_test_mux_boot_pin_sel",
                &self.sip_test_mux_boot_pin_sel(),
            )
            .field("sip_test_mux_qspi_sip_en", &self.sip_test_mux_qspi_sip_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr34 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr34 {{ sip_test_mux_boot_pin_sel: {=u8:?}, sip_test_mux_qspi_sip_en: {:?} }}",
            self.sip_test_mux_boot_pin_sel(),
            self.sip_test_mux_qspi_sip_en()
        )
    }
}
#[doc = "GPR4 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr4(pub u32);
impl Gpr4 {
    #[doc = "EDMA stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn edma_stop_req(&self) -> super::vals::EdmaStopReq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::EdmaStopReq::from_bits(val as u8)
    }
    #[doc = "EDMA stop request."]
    #[inline(always)]
    pub const fn set_edma_stop_req(&mut self, val: super::vals::EdmaStopReq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CAN1 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_stop_req(&self) -> super::vals::Can1StopReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Can1StopReq::from_bits(val as u8)
    }
    #[doc = "CAN1 stop request."]
    #[inline(always)]
    pub const fn set_can1_stop_req(&mut self, val: super::vals::Can1StopReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CAN2 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn can2_stop_req(&self) -> super::vals::Can2StopReq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Can2StopReq::from_bits(val as u8)
    }
    #[doc = "CAN2 stop request."]
    #[inline(always)]
    pub const fn set_can2_stop_req(&mut self, val: super::vals::Can2StopReq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "TRNG stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_stop_req(&self) -> super::vals::TrngStopReq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::TrngStopReq::from_bits(val as u8)
    }
    #[doc = "TRNG stop request."]
    #[inline(always)]
    pub const fn set_trng_stop_req(&mut self, val: super::vals::TrngStopReq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ENET stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn enet_stop_req(&self) -> super::vals::EnetStopReq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::EnetStopReq::from_bits(val as u8)
    }
    #[doc = "ENET stop request."]
    #[inline(always)]
    pub const fn set_enet_stop_req(&mut self, val: super::vals::EnetStopReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SAI1 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_stop_req(&self) -> super::vals::Sai1StopReq {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Sai1StopReq::from_bits(val as u8)
    }
    #[doc = "SAI1 stop request."]
    #[inline(always)]
    pub const fn set_sai1_stop_req(&mut self, val: super::vals::Sai1StopReq) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SAI2 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_stop_req(&self) -> super::vals::Sai2StopReq {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Sai2StopReq::from_bits(val as u8)
    }
    #[doc = "SAI2 stop request."]
    #[inline(always)]
    pub const fn set_sai2_stop_req(&mut self, val: super::vals::Sai2StopReq) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "SAI3 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_stop_req(&self) -> super::vals::Sai3StopReq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Sai3StopReq::from_bits(val as u8)
    }
    #[doc = "SAI3 stop request."]
    #[inline(always)]
    pub const fn set_sai3_stop_req(&mut self, val: super::vals::Sai3StopReq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "ENET2 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_stop_req(&self) -> super::vals::Enet2StopReq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Enet2StopReq::from_bits(val as u8)
    }
    #[doc = "ENET2 stop request."]
    #[inline(always)]
    pub const fn set_enet2_stop_req(&mut self, val: super::vals::Enet2StopReq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SEMC stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn semc_stop_req(&self) -> super::vals::SemcStopReq {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::SemcStopReq::from_bits(val as u8)
    }
    #[doc = "SEMC stop request."]
    #[inline(always)]
    pub const fn set_semc_stop_req(&mut self, val: super::vals::SemcStopReq) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "PIT stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn pit_stop_req(&self) -> super::vals::PitStopReq {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::PitStopReq::from_bits(val as u8)
    }
    #[doc = "PIT stop request."]
    #[inline(always)]
    pub const fn set_pit_stop_req(&mut self, val: super::vals::PitStopReq) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FlexSPI stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_stop_req(&self) -> super::vals::FlexspiStopReq {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::FlexspiStopReq::from_bits(val as u8)
    }
    #[doc = "FlexSPI stop request."]
    #[inline(always)]
    pub const fn set_flexspi_stop_req(&mut self, val: super::vals::FlexspiStopReq) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FlexIO1 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_stop_req(&self) -> super::vals::Flexio1StopReq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Flexio1StopReq::from_bits(val as u8)
    }
    #[doc = "FlexIO1 stop request."]
    #[inline(always)]
    pub const fn set_flexio1_stop_req(&mut self, val: super::vals::Flexio1StopReq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FlexIO2 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_stop_req(&self) -> super::vals::Flexio2StopReq {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Flexio2StopReq::from_bits(val as u8)
    }
    #[doc = "FlexIO2 stop request."]
    #[inline(always)]
    pub const fn set_flexio2_stop_req(&mut self, val: super::vals::Flexio2StopReq) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "On-platform flexio3 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio3_stop_req(&self) -> super::vals::Flexio3StopReq {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Flexio3StopReq::from_bits(val as u8)
    }
    #[doc = "On-platform flexio3 stop request."]
    #[inline(always)]
    pub const fn set_flexio3_stop_req(&mut self, val: super::vals::Flexio3StopReq) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FlexSPI2 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi2_stop_req(&self) -> super::vals::Flexspi2StopReq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Flexspi2StopReq::from_bits(val as u8)
    }
    #[doc = "FlexSPI2 stop request."]
    #[inline(always)]
    pub const fn set_flexspi2_stop_req(&mut self, val: super::vals::Flexspi2StopReq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[must_use]
    #[inline(always)]
    pub const fn edma_stop_ack(&self) -> super::vals::EdmaStopAck {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::EdmaStopAck::from_bits(val as u8)
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline(always)]
    pub const fn set_edma_stop_ack(&mut self, val: super::vals::EdmaStopAck) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "CAN1 stop acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn can1_stop_ack(&self) -> super::vals::Can1StopAck {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Can1StopAck::from_bits(val as u8)
    }
    #[doc = "CAN1 stop acknowledge."]
    #[inline(always)]
    pub const fn set_can1_stop_ack(&mut self, val: super::vals::Can1StopAck) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "CAN2 stop acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn can2_stop_ack(&self) -> super::vals::Can2StopAck {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Can2StopAck::from_bits(val as u8)
    }
    #[doc = "CAN2 stop acknowledge."]
    #[inline(always)]
    pub const fn set_can2_stop_ack(&mut self, val: super::vals::Can2StopAck) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "TRNG stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_stop_ack(&self) -> super::vals::TrngStopAck {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::TrngStopAck::from_bits(val as u8)
    }
    #[doc = "TRNG stop acknowledge"]
    #[inline(always)]
    pub const fn set_trng_stop_ack(&mut self, val: super::vals::TrngStopAck) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "ENET stop acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn enet_stop_ack(&self) -> super::vals::EnetStopAck {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::EnetStopAck::from_bits(val as u8)
    }
    #[doc = "ENET stop acknowledge."]
    #[inline(always)]
    pub const fn set_enet_stop_ack(&mut self, val: super::vals::EnetStopAck) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SAI1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_stop_ack(&self) -> super::vals::Sai1StopAck {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Sai1StopAck::from_bits(val as u8)
    }
    #[doc = "SAI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai1_stop_ack(&mut self, val: super::vals::Sai1StopAck) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "SAI2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_stop_ack(&self) -> super::vals::Sai2StopAck {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Sai2StopAck::from_bits(val as u8)
    }
    #[doc = "SAI2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai2_stop_ack(&mut self, val: super::vals::Sai2StopAck) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "SAI3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_stop_ack(&self) -> super::vals::Sai3StopAck {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Sai3StopAck::from_bits(val as u8)
    }
    #[doc = "SAI3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai3_stop_ack(&mut self, val: super::vals::Sai3StopAck) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "ENET2 stop acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_stop_ack(&self) -> super::vals::Enet2StopAck {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Enet2StopAck::from_bits(val as u8)
    }
    #[doc = "ENET2 stop acknowledge."]
    #[inline(always)]
    pub const fn set_enet2_stop_ack(&mut self, val: super::vals::Enet2StopAck) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SEMC stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn semc_stop_ack(&self) -> super::vals::SemcStopAck {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::SemcStopAck::from_bits(val as u8)
    }
    #[doc = "SEMC stop acknowledge"]
    #[inline(always)]
    pub const fn set_semc_stop_ack(&mut self, val: super::vals::SemcStopAck) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "PIT stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn pit_stop_ack(&self) -> super::vals::PitStopAck {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PitStopAck::from_bits(val as u8)
    }
    #[doc = "PIT stop acknowledge"]
    #[inline(always)]
    pub const fn set_pit_stop_ack(&mut self, val: super::vals::PitStopAck) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_stop_ack(&self) -> super::vals::FlexspiStopAck {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::FlexspiStopAck::from_bits(val as u8)
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexspi_stop_ack(&mut self, val: super::vals::FlexspiStopAck) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_stop_ack(&self) -> super::vals::Flexio1StopAck {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Flexio1StopAck::from_bits(val as u8)
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexio1_stop_ack(&mut self, val: super::vals::Flexio1StopAck) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "FLEXIO2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio2_stop_ack(&self) -> super::vals::Flexio2StopAck {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Flexio2StopAck::from_bits(val as u8)
    }
    #[doc = "FLEXIO2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexio2_stop_ack(&mut self, val: super::vals::Flexio2StopAck) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "On-platform FLEXIO3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio3_stop_ack(&self) -> super::vals::Flexio3StopAck {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Flexio3StopAck::from_bits(val as u8)
    }
    #[doc = "On-platform FLEXIO3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexio3_stop_ack(&mut self, val: super::vals::Flexio3StopAck) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "FLEXSPI2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi2_stop_ack(&self) -> super::vals::Flexspi2StopAck {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Flexspi2StopAck::from_bits(val as u8)
    }
    #[doc = "FLEXSPI2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexspi2_stop_ack(&mut self, val: super::vals::Flexspi2StopAck) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr4 {
    #[inline(always)]
    fn default() -> Gpr4 {
        Gpr4(0)
    }
}
impl core::fmt::Debug for Gpr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr4")
            .field("edma_stop_req", &self.edma_stop_req())
            .field("can1_stop_req", &self.can1_stop_req())
            .field("can2_stop_req", &self.can2_stop_req())
            .field("trng_stop_req", &self.trng_stop_req())
            .field("enet_stop_req", &self.enet_stop_req())
            .field("sai1_stop_req", &self.sai1_stop_req())
            .field("sai2_stop_req", &self.sai2_stop_req())
            .field("sai3_stop_req", &self.sai3_stop_req())
            .field("enet2_stop_req", &self.enet2_stop_req())
            .field("semc_stop_req", &self.semc_stop_req())
            .field("pit_stop_req", &self.pit_stop_req())
            .field("flexspi_stop_req", &self.flexspi_stop_req())
            .field("flexio1_stop_req", &self.flexio1_stop_req())
            .field("flexio2_stop_req", &self.flexio2_stop_req())
            .field("flexio3_stop_req", &self.flexio3_stop_req())
            .field("flexspi2_stop_req", &self.flexspi2_stop_req())
            .field("edma_stop_ack", &self.edma_stop_ack())
            .field("can1_stop_ack", &self.can1_stop_ack())
            .field("can2_stop_ack", &self.can2_stop_ack())
            .field("trng_stop_ack", &self.trng_stop_ack())
            .field("enet_stop_ack", &self.enet_stop_ack())
            .field("sai1_stop_ack", &self.sai1_stop_ack())
            .field("sai2_stop_ack", &self.sai2_stop_ack())
            .field("sai3_stop_ack", &self.sai3_stop_ack())
            .field("enet2_stop_ack", &self.enet2_stop_ack())
            .field("semc_stop_ack", &self.semc_stop_ack())
            .field("pit_stop_ack", &self.pit_stop_ack())
            .field("flexspi_stop_ack", &self.flexspi_stop_ack())
            .field("flexio1_stop_ack", &self.flexio1_stop_ack())
            .field("flexio2_stop_ack", &self.flexio2_stop_ack())
            .field("flexio3_stop_ack", &self.flexio3_stop_ack())
            .field("flexspi2_stop_ack", &self.flexspi2_stop_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr4 {{ edma_stop_req: {:?}, can1_stop_req: {:?}, can2_stop_req: {:?}, trng_stop_req: {:?}, enet_stop_req: {:?}, sai1_stop_req: {:?}, sai2_stop_req: {:?}, sai3_stop_req: {:?}, enet2_stop_req: {:?}, semc_stop_req: {:?}, pit_stop_req: {:?}, flexspi_stop_req: {:?}, flexio1_stop_req: {:?}, flexio2_stop_req: {:?}, flexio3_stop_req: {:?}, flexspi2_stop_req: {:?}, edma_stop_ack: {:?}, can1_stop_ack: {:?}, can2_stop_ack: {:?}, trng_stop_ack: {:?}, enet_stop_ack: {:?}, sai1_stop_ack: {:?}, sai2_stop_ack: {:?}, sai3_stop_ack: {:?}, enet2_stop_ack: {:?}, semc_stop_ack: {:?}, pit_stop_ack: {:?}, flexspi_stop_ack: {:?}, flexio1_stop_ack: {:?}, flexio2_stop_ack: {:?}, flexio3_stop_ack: {:?}, flexspi2_stop_ack: {:?} }}",
            self.edma_stop_req(),
            self.can1_stop_req(),
            self.can2_stop_req(),
            self.trng_stop_req(),
            self.enet_stop_req(),
            self.sai1_stop_req(),
            self.sai2_stop_req(),
            self.sai3_stop_req(),
            self.enet2_stop_req(),
            self.semc_stop_req(),
            self.pit_stop_req(),
            self.flexspi_stop_req(),
            self.flexio1_stop_req(),
            self.flexio2_stop_req(),
            self.flexio3_stop_req(),
            self.flexspi2_stop_req(),
            self.edma_stop_ack(),
            self.can1_stop_ack(),
            self.can2_stop_ack(),
            self.trng_stop_ack(),
            self.enet_stop_ack(),
            self.sai1_stop_ack(),
            self.sai2_stop_ack(),
            self.sai3_stop_ack(),
            self.enet2_stop_ack(),
            self.semc_stop_ack(),
            self.pit_stop_ack(),
            self.flexspi_stop_ack(),
            self.flexio1_stop_ack(),
            self.flexio2_stop_ack(),
            self.flexio3_stop_ack(),
            self.flexspi2_stop_ack()
        )
    }
}
#[doc = "GPR5 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr5(pub u32);
impl Gpr5 {
    #[doc = "WDOG1 Timeout Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog1_mask(&self) -> super::vals::Wdog1Mask {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Wdog1Mask::from_bits(val as u8)
    }
    #[doc = "WDOG1 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog1_mask(&mut self, val: super::vals::Wdog1Mask) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_mask(&self) -> super::vals::Wdog2Mask {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wdog2Mask::from_bits(val as u8)
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog2_mask(&mut self, val: super::vals::Wdog2Mask) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "GPT2 input capture channel 1 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn gpt2_capin1_sel(&self) -> super::vals::Gpt2Capin1Sel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Gpt2Capin1Sel::from_bits(val as u8)
    }
    #[doc = "GPT2 input capture channel 1 source select"]
    #[inline(always)]
    pub const fn set_gpt2_capin1_sel(&mut self, val: super::vals::Gpt2Capin1Sel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "GPT2 input capture channel 2 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn gpt2_capin2_sel(&self) -> super::vals::Gpt2Capin2Sel {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Gpt2Capin2Sel::from_bits(val as u8)
    }
    #[doc = "GPT2 input capture channel 2 source select"]
    #[inline(always)]
    pub const fn set_gpt2_capin2_sel(&mut self, val: super::vals::Gpt2Capin2Sel) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "ENET input timer event3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn enet_event3in_sel(&self) -> super::vals::EnetEvent3inSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::EnetEvent3inSel::from_bits(val as u8)
    }
    #[doc = "ENET input timer event3 source select"]
    #[inline(always)]
    pub const fn set_enet_event3in_sel(&mut self, val: super::vals::EnetEvent3inSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "ENET2 input timer event3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn enet2_event3in_sel(&self) -> super::vals::Enet2Event3inSel {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Enet2Event3inSel::from_bits(val as u8)
    }
    #[doc = "ENET2 input timer event3 source select"]
    #[inline(always)]
    pub const fn set_enet2_event3in_sel(&mut self, val: super::vals::Enet2Event3inSel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt1(&self) -> super::vals::Vref1mClkGpt1 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Vref1mClkGpt1::from_bits(val as u8)
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt1(&mut self, val: super::vals::Vref1mClkGpt1) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt2(&self) -> super::vals::Vref1mClkGpt2 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Vref1mClkGpt2::from_bits(val as u8)
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt2(&mut self, val: super::vals::Vref1mClkGpt2) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Gpr5 {
    #[inline(always)]
    fn default() -> Gpr5 {
        Gpr5(0)
    }
}
impl core::fmt::Debug for Gpr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr5")
            .field("wdog1_mask", &self.wdog1_mask())
            .field("wdog2_mask", &self.wdog2_mask())
            .field("gpt2_capin1_sel", &self.gpt2_capin1_sel())
            .field("gpt2_capin2_sel", &self.gpt2_capin2_sel())
            .field("enet_event3in_sel", &self.enet_event3in_sel())
            .field("enet2_event3in_sel", &self.enet2_event3in_sel())
            .field("vref_1m_clk_gpt1", &self.vref_1m_clk_gpt1())
            .field("vref_1m_clk_gpt2", &self.vref_1m_clk_gpt2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr5 {{ wdog1_mask: {:?}, wdog2_mask: {:?}, gpt2_capin1_sel: {:?}, gpt2_capin2_sel: {:?}, enet_event3in_sel: {:?}, enet2_event3in_sel: {:?}, vref_1m_clk_gpt1: {:?}, vref_1m_clk_gpt2: {:?} }}",
            self.wdog1_mask(),
            self.wdog2_mask(),
            self.gpt2_capin1_sel(),
            self.gpt2_capin2_sel(),
            self.enet_event3in_sel(),
            self.enet2_event3in_sel(),
            self.vref_1m_clk_gpt1(),
            self.vref_1m_clk_gpt2()
        )
    }
}
#[doc = "GPR6 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr6(pub u32);
impl Gpr6 {
    #[doc = "QTIMER1 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_trm0_input_sel(&self) -> super::vals::Qtimer1Trm0InputSel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer1Trm0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_trm0_input_sel(&mut self, val: super::vals::Qtimer1Trm0InputSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QTIMER1 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_trm1_input_sel(&self) -> super::vals::Qtimer1Trm1InputSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Qtimer1Trm1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_trm1_input_sel(&mut self, val: super::vals::Qtimer1Trm1InputSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "QTIMER1 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_trm2_input_sel(&self) -> super::vals::Qtimer1Trm2InputSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Qtimer1Trm2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_trm2_input_sel(&mut self, val: super::vals::Qtimer1Trm2InputSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "QTIMER1 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_trm3_input_sel(&self) -> super::vals::Qtimer1Trm3InputSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Qtimer1Trm3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_trm3_input_sel(&mut self, val: super::vals::Qtimer1Trm3InputSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QTIMER2 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_trm0_input_sel(&self) -> super::vals::Qtimer2Trm0InputSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Qtimer2Trm0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_trm0_input_sel(&mut self, val: super::vals::Qtimer2Trm0InputSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QTIMER2 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_trm1_input_sel(&self) -> super::vals::Qtimer2Trm1InputSel {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Qtimer2Trm1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_trm1_input_sel(&mut self, val: super::vals::Qtimer2Trm1InputSel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "QTIMER2 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_trm2_input_sel(&self) -> super::vals::Qtimer2Trm2InputSel {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Qtimer2Trm2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_trm2_input_sel(&mut self, val: super::vals::Qtimer2Trm2InputSel) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "QTIMER2 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_trm3_input_sel(&self) -> super::vals::Qtimer2Trm3InputSel {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Qtimer2Trm3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_trm3_input_sel(&mut self, val: super::vals::Qtimer2Trm3InputSel) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "QTIMER3 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_trm0_input_sel(&self) -> super::vals::Qtimer3Trm0InputSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Qtimer3Trm0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_trm0_input_sel(&mut self, val: super::vals::Qtimer3Trm0InputSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "QTIMER3 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_trm1_input_sel(&self) -> super::vals::Qtimer3Trm1InputSel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Qtimer3Trm1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_trm1_input_sel(&mut self, val: super::vals::Qtimer3Trm1InputSel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "QTIMER3 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_trm2_input_sel(&self) -> super::vals::Qtimer3Trm2InputSel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Qtimer3Trm2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_trm2_input_sel(&mut self, val: super::vals::Qtimer3Trm2InputSel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "QTIMER3 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_trm3_input_sel(&self) -> super::vals::Qtimer3Trm3InputSel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Qtimer3Trm3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_trm3_input_sel(&mut self, val: super::vals::Qtimer3Trm3InputSel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "QTIMER4 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_trm0_input_sel(&self) -> super::vals::Qtimer4Trm0InputSel {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Qtimer4Trm0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_trm0_input_sel(&mut self, val: super::vals::Qtimer4Trm0InputSel) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "QTIMER4 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_trm1_input_sel(&self) -> super::vals::Qtimer4Trm1InputSel {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Qtimer4Trm1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_trm1_input_sel(&mut self, val: super::vals::Qtimer4Trm1InputSel) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "QTIMER4 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_trm2_input_sel(&self) -> super::vals::Qtimer4Trm2InputSel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Qtimer4Trm2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_trm2_input_sel(&mut self, val: super::vals::Qtimer4Trm2InputSel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "QTIMER4 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_trm3_input_sel(&self) -> super::vals::Qtimer4Trm3InputSel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Qtimer4Trm3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_trm3_input_sel(&mut self, val: super::vals::Qtimer4Trm3InputSel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_4(&self) -> super::vals::IomuxcXbarDirSel4 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::IomuxcXbarDirSel4::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_4(&mut self, val: super::vals::IomuxcXbarDirSel4) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_5(&self) -> super::vals::IomuxcXbarDirSel5 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::IomuxcXbarDirSel5::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_5(&mut self, val: super::vals::IomuxcXbarDirSel5) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_6(&self) -> super::vals::IomuxcXbarDirSel6 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::IomuxcXbarDirSel6::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_6(&mut self, val: super::vals::IomuxcXbarDirSel6) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_7(&self) -> super::vals::IomuxcXbarDirSel7 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::IomuxcXbarDirSel7::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_7(&mut self, val: super::vals::IomuxcXbarDirSel7) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_8(&self) -> super::vals::IomuxcXbarDirSel8 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::IomuxcXbarDirSel8::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_8(&mut self, val: super::vals::IomuxcXbarDirSel8) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_9(&self) -> super::vals::IomuxcXbarDirSel9 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::IomuxcXbarDirSel9::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_9(&mut self, val: super::vals::IomuxcXbarDirSel9) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_10(&self) -> super::vals::IomuxcXbarDirSel10 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::IomuxcXbarDirSel10::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_10(&mut self, val: super::vals::IomuxcXbarDirSel10) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_11(&self) -> super::vals::IomuxcXbarDirSel11 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::IomuxcXbarDirSel11::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_11(&mut self, val: super::vals::IomuxcXbarDirSel11) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_12(&self) -> super::vals::IomuxcXbarDirSel12 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::IomuxcXbarDirSel12::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_12(&mut self, val: super::vals::IomuxcXbarDirSel12) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_13(&self) -> super::vals::IomuxcXbarDirSel13 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::IomuxcXbarDirSel13::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_13(&mut self, val: super::vals::IomuxcXbarDirSel13) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_14(&self) -> super::vals::IomuxcXbarDirSel14 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::IomuxcXbarDirSel14::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_14(&mut self, val: super::vals::IomuxcXbarDirSel14) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_15(&self) -> super::vals::IomuxcXbarDirSel15 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::IomuxcXbarDirSel15::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_15(&mut self, val: super::vals::IomuxcXbarDirSel15) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_16(&self) -> super::vals::IomuxcXbarDirSel16 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::IomuxcXbarDirSel16::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_16(&mut self, val: super::vals::IomuxcXbarDirSel16) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_17(&self) -> super::vals::IomuxcXbarDirSel17 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::IomuxcXbarDirSel17::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_17(&mut self, val: super::vals::IomuxcXbarDirSel17) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_18(&self) -> super::vals::IomuxcXbarDirSel18 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::IomuxcXbarDirSel18::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_18(&mut self, val: super::vals::IomuxcXbarDirSel18) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_19(&self) -> super::vals::IomuxcXbarDirSel19 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::IomuxcXbarDirSel19::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_19(&mut self, val: super::vals::IomuxcXbarDirSel19) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr6 {
    #[inline(always)]
    fn default() -> Gpr6 {
        Gpr6(0)
    }
}
impl core::fmt::Debug for Gpr6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr6")
            .field("qtimer1_trm0_input_sel", &self.qtimer1_trm0_input_sel())
            .field("qtimer1_trm1_input_sel", &self.qtimer1_trm1_input_sel())
            .field("qtimer1_trm2_input_sel", &self.qtimer1_trm2_input_sel())
            .field("qtimer1_trm3_input_sel", &self.qtimer1_trm3_input_sel())
            .field("qtimer2_trm0_input_sel", &self.qtimer2_trm0_input_sel())
            .field("qtimer2_trm1_input_sel", &self.qtimer2_trm1_input_sel())
            .field("qtimer2_trm2_input_sel", &self.qtimer2_trm2_input_sel())
            .field("qtimer2_trm3_input_sel", &self.qtimer2_trm3_input_sel())
            .field("qtimer3_trm0_input_sel", &self.qtimer3_trm0_input_sel())
            .field("qtimer3_trm1_input_sel", &self.qtimer3_trm1_input_sel())
            .field("qtimer3_trm2_input_sel", &self.qtimer3_trm2_input_sel())
            .field("qtimer3_trm3_input_sel", &self.qtimer3_trm3_input_sel())
            .field("qtimer4_trm0_input_sel", &self.qtimer4_trm0_input_sel())
            .field("qtimer4_trm1_input_sel", &self.qtimer4_trm1_input_sel())
            .field("qtimer4_trm2_input_sel", &self.qtimer4_trm2_input_sel())
            .field("qtimer4_trm3_input_sel", &self.qtimer4_trm3_input_sel())
            .field("iomuxc_xbar_dir_sel_4", &self.iomuxc_xbar_dir_sel_4())
            .field("iomuxc_xbar_dir_sel_5", &self.iomuxc_xbar_dir_sel_5())
            .field("iomuxc_xbar_dir_sel_6", &self.iomuxc_xbar_dir_sel_6())
            .field("iomuxc_xbar_dir_sel_7", &self.iomuxc_xbar_dir_sel_7())
            .field("iomuxc_xbar_dir_sel_8", &self.iomuxc_xbar_dir_sel_8())
            .field("iomuxc_xbar_dir_sel_9", &self.iomuxc_xbar_dir_sel_9())
            .field("iomuxc_xbar_dir_sel_10", &self.iomuxc_xbar_dir_sel_10())
            .field("iomuxc_xbar_dir_sel_11", &self.iomuxc_xbar_dir_sel_11())
            .field("iomuxc_xbar_dir_sel_12", &self.iomuxc_xbar_dir_sel_12())
            .field("iomuxc_xbar_dir_sel_13", &self.iomuxc_xbar_dir_sel_13())
            .field("iomuxc_xbar_dir_sel_14", &self.iomuxc_xbar_dir_sel_14())
            .field("iomuxc_xbar_dir_sel_15", &self.iomuxc_xbar_dir_sel_15())
            .field("iomuxc_xbar_dir_sel_16", &self.iomuxc_xbar_dir_sel_16())
            .field("iomuxc_xbar_dir_sel_17", &self.iomuxc_xbar_dir_sel_17())
            .field("iomuxc_xbar_dir_sel_18", &self.iomuxc_xbar_dir_sel_18())
            .field("iomuxc_xbar_dir_sel_19", &self.iomuxc_xbar_dir_sel_19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr6 {{ qtimer1_trm0_input_sel: {:?}, qtimer1_trm1_input_sel: {:?}, qtimer1_trm2_input_sel: {:?}, qtimer1_trm3_input_sel: {:?}, qtimer2_trm0_input_sel: {:?}, qtimer2_trm1_input_sel: {:?}, qtimer2_trm2_input_sel: {:?}, qtimer2_trm3_input_sel: {:?}, qtimer3_trm0_input_sel: {:?}, qtimer3_trm1_input_sel: {:?}, qtimer3_trm2_input_sel: {:?}, qtimer3_trm3_input_sel: {:?}, qtimer4_trm0_input_sel: {:?}, qtimer4_trm1_input_sel: {:?}, qtimer4_trm2_input_sel: {:?}, qtimer4_trm3_input_sel: {:?}, iomuxc_xbar_dir_sel_4: {:?}, iomuxc_xbar_dir_sel_5: {:?}, iomuxc_xbar_dir_sel_6: {:?}, iomuxc_xbar_dir_sel_7: {:?}, iomuxc_xbar_dir_sel_8: {:?}, iomuxc_xbar_dir_sel_9: {:?}, iomuxc_xbar_dir_sel_10: {:?}, iomuxc_xbar_dir_sel_11: {:?}, iomuxc_xbar_dir_sel_12: {:?}, iomuxc_xbar_dir_sel_13: {:?}, iomuxc_xbar_dir_sel_14: {:?}, iomuxc_xbar_dir_sel_15: {:?}, iomuxc_xbar_dir_sel_16: {:?}, iomuxc_xbar_dir_sel_17: {:?}, iomuxc_xbar_dir_sel_18: {:?}, iomuxc_xbar_dir_sel_19: {:?} }}",
            self.qtimer1_trm0_input_sel(),
            self.qtimer1_trm1_input_sel(),
            self.qtimer1_trm2_input_sel(),
            self.qtimer1_trm3_input_sel(),
            self.qtimer2_trm0_input_sel(),
            self.qtimer2_trm1_input_sel(),
            self.qtimer2_trm2_input_sel(),
            self.qtimer2_trm3_input_sel(),
            self.qtimer3_trm0_input_sel(),
            self.qtimer3_trm1_input_sel(),
            self.qtimer3_trm2_input_sel(),
            self.qtimer3_trm3_input_sel(),
            self.qtimer4_trm0_input_sel(),
            self.qtimer4_trm1_input_sel(),
            self.qtimer4_trm2_input_sel(),
            self.qtimer4_trm3_input_sel(),
            self.iomuxc_xbar_dir_sel_4(),
            self.iomuxc_xbar_dir_sel_5(),
            self.iomuxc_xbar_dir_sel_6(),
            self.iomuxc_xbar_dir_sel_7(),
            self.iomuxc_xbar_dir_sel_8(),
            self.iomuxc_xbar_dir_sel_9(),
            self.iomuxc_xbar_dir_sel_10(),
            self.iomuxc_xbar_dir_sel_11(),
            self.iomuxc_xbar_dir_sel_12(),
            self.iomuxc_xbar_dir_sel_13(),
            self.iomuxc_xbar_dir_sel_14(),
            self.iomuxc_xbar_dir_sel_15(),
            self.iomuxc_xbar_dir_sel_16(),
            self.iomuxc_xbar_dir_sel_17(),
            self.iomuxc_xbar_dir_sel_18(),
            self.iomuxc_xbar_dir_sel_19()
        )
    }
}
#[doc = "GPR7 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr7(pub u32);
impl Gpr7 {
    #[doc = "LPI2C1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_stop_req(&self) -> super::vals::Lpi2c1StopReq {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c1StopReq::from_bits(val as u8)
    }
    #[doc = "LPI2C1 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_req(&mut self, val: super::vals::Lpi2c1StopReq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C2 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_stop_req(&self) -> super::vals::Lpi2c2StopReq {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpi2c2StopReq::from_bits(val as u8)
    }
    #[doc = "LPI2C2 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_req(&mut self, val: super::vals::Lpi2c2StopReq) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LPI2C3 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3_stop_req(&self) -> super::vals::Lpi2c3StopReq {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lpi2c3StopReq::from_bits(val as u8)
    }
    #[doc = "LPI2C3 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c3_stop_req(&mut self, val: super::vals::Lpi2c3StopReq) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LPI2C4 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4_stop_req(&self) -> super::vals::Lpi2c4StopReq {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lpi2c4StopReq::from_bits(val as u8)
    }
    #[doc = "LPI2C4 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c4_stop_req(&mut self, val: super::vals::Lpi2c4StopReq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPSPI1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_stop_req(&self) -> super::vals::Lpspi1StopReq {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lpspi1StopReq::from_bits(val as u8)
    }
    #[doc = "LPSPI1 stop request"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_req(&mut self, val: super::vals::Lpspi1StopReq) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI2 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_stop_req(&self) -> super::vals::Lpspi2StopReq {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Lpspi2StopReq::from_bits(val as u8)
    }
    #[doc = "LPSPI2 stop request"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_req(&mut self, val: super::vals::Lpspi2StopReq) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPSPI3 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3_stop_req(&self) -> super::vals::Lpspi3StopReq {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lpspi3StopReq::from_bits(val as u8)
    }
    #[doc = "LPSPI3 stop request"]
    #[inline(always)]
    pub const fn set_lpspi3_stop_req(&mut self, val: super::vals::Lpspi3StopReq) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI4 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4_stop_req(&self) -> super::vals::Lpspi4StopReq {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Lpspi4StopReq::from_bits(val as u8)
    }
    #[doc = "LPSPI4 stop request"]
    #[inline(always)]
    pub const fn set_lpspi4_stop_req(&mut self, val: super::vals::Lpspi4StopReq) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "LPUART1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_stop_req(&self) -> super::vals::Lpuart1StopReq {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lpuart1StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_req(&mut self, val: super::vals::Lpuart1StopReq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LPUART1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_stop_req(&self) -> super::vals::Lpuart2StopReq {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lpuart2StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_req(&mut self, val: super::vals::Lpuart2StopReq) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "LPUART3 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_stop_req(&self) -> super::vals::Lpuart3StopReq {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Lpuart3StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART3 stop request"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_req(&mut self, val: super::vals::Lpuart3StopReq) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "LPUART4 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_stop_req(&self) -> super::vals::Lpuart4StopReq {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Lpuart4StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART4 stop request"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_req(&mut self, val: super::vals::Lpuart4StopReq) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "LPUART5 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5_stop_req(&self) -> super::vals::Lpuart5StopReq {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Lpuart5StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART5 stop request"]
    #[inline(always)]
    pub const fn set_lpuart5_stop_req(&mut self, val: super::vals::Lpuart5StopReq) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "LPUART6 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart6_stop_req(&self) -> super::vals::Lpuart6StopReq {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Lpuart6StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART6 stop request"]
    #[inline(always)]
    pub const fn set_lpuart6_stop_req(&mut self, val: super::vals::Lpuart6StopReq) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "LPUART7 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart7_stop_req(&self) -> super::vals::Lpuart7StopReq {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Lpuart7StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART7 stop request"]
    #[inline(always)]
    pub const fn set_lpuart7_stop_req(&mut self, val: super::vals::Lpuart7StopReq) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "LPUART8 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart8_stop_req(&self) -> super::vals::Lpuart8StopReq {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Lpuart8StopReq::from_bits(val as u8)
    }
    #[doc = "LPUART8 stop request"]
    #[inline(always)]
    pub const fn set_lpuart8_stop_req(&mut self, val: super::vals::Lpuart8StopReq) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_stop_ack(&self) -> super::vals::Lpi2c1StopAck {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Lpi2c1StopAck::from_bits(val as u8)
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_ack(&mut self, val: super::vals::Lpi2c1StopAck) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_stop_ack(&self) -> super::vals::Lpi2c2StopAck {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Lpi2c2StopAck::from_bits(val as u8)
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_ack(&mut self, val: super::vals::Lpi2c2StopAck) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LPI2C3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3_stop_ack(&self) -> super::vals::Lpi2c3StopAck {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Lpi2c3StopAck::from_bits(val as u8)
    }
    #[doc = "LPI2C3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c3_stop_ack(&mut self, val: super::vals::Lpi2c3StopAck) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LPI2C4 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4_stop_ack(&self) -> super::vals::Lpi2c4StopAck {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Lpi2c4StopAck::from_bits(val as u8)
    }
    #[doc = "LPI2C4 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c4_stop_ack(&mut self, val: super::vals::Lpi2c4StopAck) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_stop_ack(&self) -> super::vals::Lpspi1StopAck {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lpspi1StopAck::from_bits(val as u8)
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_ack(&mut self, val: super::vals::Lpspi1StopAck) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_stop_ack(&self) -> super::vals::Lpspi2StopAck {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Lpspi2StopAck::from_bits(val as u8)
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_ack(&mut self, val: super::vals::Lpspi2StopAck) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "LPSPI3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3_stop_ack(&self) -> super::vals::Lpspi3StopAck {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Lpspi3StopAck::from_bits(val as u8)
    }
    #[doc = "LPSPI3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi3_stop_ack(&mut self, val: super::vals::Lpspi3StopAck) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "LPSPI4 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4_stop_ack(&self) -> super::vals::Lpspi4StopAck {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Lpspi4StopAck::from_bits(val as u8)
    }
    #[doc = "LPSPI4 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi4_stop_ack(&mut self, val: super::vals::Lpspi4StopAck) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_stop_ack(&self) -> super::vals::Lpuart1StopAck {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Lpuart1StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_ack(&mut self, val: super::vals::Lpuart1StopAck) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_stop_ack(&self) -> super::vals::Lpuart2StopAck {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Lpuart2StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_ack(&mut self, val: super::vals::Lpuart2StopAck) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_stop_ack(&self) -> super::vals::Lpuart3StopAck {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Lpuart3StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_ack(&mut self, val: super::vals::Lpuart3StopAck) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_stop_ack(&self) -> super::vals::Lpuart4StopAck {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Lpuart4StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_ack(&mut self, val: super::vals::Lpuart4StopAck) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART5 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5_stop_ack(&self) -> super::vals::Lpuart5StopAck {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Lpuart5StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART5 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart5_stop_ack(&mut self, val: super::vals::Lpuart5StopAck) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART6 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart6_stop_ack(&self) -> super::vals::Lpuart6StopAck {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Lpuart6StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART6 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart6_stop_ack(&mut self, val: super::vals::Lpuart6StopAck) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART7 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart7_stop_ack(&self) -> super::vals::Lpuart7StopAck {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Lpuart7StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART7 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart7_stop_ack(&mut self, val: super::vals::Lpuart7StopAck) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "LPUART8 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart8_stop_ack(&self) -> super::vals::Lpuart8StopAck {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lpuart8StopAck::from_bits(val as u8)
    }
    #[doc = "LPUART8 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart8_stop_ack(&mut self, val: super::vals::Lpuart8StopAck) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr7 {
    #[inline(always)]
    fn default() -> Gpr7 {
        Gpr7(0)
    }
}
impl core::fmt::Debug for Gpr7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr7")
            .field("lpi2c1_stop_req", &self.lpi2c1_stop_req())
            .field("lpi2c2_stop_req", &self.lpi2c2_stop_req())
            .field("lpi2c3_stop_req", &self.lpi2c3_stop_req())
            .field("lpi2c4_stop_req", &self.lpi2c4_stop_req())
            .field("lpspi1_stop_req", &self.lpspi1_stop_req())
            .field("lpspi2_stop_req", &self.lpspi2_stop_req())
            .field("lpspi3_stop_req", &self.lpspi3_stop_req())
            .field("lpspi4_stop_req", &self.lpspi4_stop_req())
            .field("lpuart1_stop_req", &self.lpuart1_stop_req())
            .field("lpuart2_stop_req", &self.lpuart2_stop_req())
            .field("lpuart3_stop_req", &self.lpuart3_stop_req())
            .field("lpuart4_stop_req", &self.lpuart4_stop_req())
            .field("lpuart5_stop_req", &self.lpuart5_stop_req())
            .field("lpuart6_stop_req", &self.lpuart6_stop_req())
            .field("lpuart7_stop_req", &self.lpuart7_stop_req())
            .field("lpuart8_stop_req", &self.lpuart8_stop_req())
            .field("lpi2c1_stop_ack", &self.lpi2c1_stop_ack())
            .field("lpi2c2_stop_ack", &self.lpi2c2_stop_ack())
            .field("lpi2c3_stop_ack", &self.lpi2c3_stop_ack())
            .field("lpi2c4_stop_ack", &self.lpi2c4_stop_ack())
            .field("lpspi1_stop_ack", &self.lpspi1_stop_ack())
            .field("lpspi2_stop_ack", &self.lpspi2_stop_ack())
            .field("lpspi3_stop_ack", &self.lpspi3_stop_ack())
            .field("lpspi4_stop_ack", &self.lpspi4_stop_ack())
            .field("lpuart1_stop_ack", &self.lpuart1_stop_ack())
            .field("lpuart2_stop_ack", &self.lpuart2_stop_ack())
            .field("lpuart3_stop_ack", &self.lpuart3_stop_ack())
            .field("lpuart4_stop_ack", &self.lpuart4_stop_ack())
            .field("lpuart5_stop_ack", &self.lpuart5_stop_ack())
            .field("lpuart6_stop_ack", &self.lpuart6_stop_ack())
            .field("lpuart7_stop_ack", &self.lpuart7_stop_ack())
            .field("lpuart8_stop_ack", &self.lpuart8_stop_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr7 {{ lpi2c1_stop_req: {:?}, lpi2c2_stop_req: {:?}, lpi2c3_stop_req: {:?}, lpi2c4_stop_req: {:?}, lpspi1_stop_req: {:?}, lpspi2_stop_req: {:?}, lpspi3_stop_req: {:?}, lpspi4_stop_req: {:?}, lpuart1_stop_req: {:?}, lpuart2_stop_req: {:?}, lpuart3_stop_req: {:?}, lpuart4_stop_req: {:?}, lpuart5_stop_req: {:?}, lpuart6_stop_req: {:?}, lpuart7_stop_req: {:?}, lpuart8_stop_req: {:?}, lpi2c1_stop_ack: {:?}, lpi2c2_stop_ack: {:?}, lpi2c3_stop_ack: {:?}, lpi2c4_stop_ack: {:?}, lpspi1_stop_ack: {:?}, lpspi2_stop_ack: {:?}, lpspi3_stop_ack: {:?}, lpspi4_stop_ack: {:?}, lpuart1_stop_ack: {:?}, lpuart2_stop_ack: {:?}, lpuart3_stop_ack: {:?}, lpuart4_stop_ack: {:?}, lpuart5_stop_ack: {:?}, lpuart6_stop_ack: {:?}, lpuart7_stop_ack: {:?}, lpuart8_stop_ack: {:?} }}",
            self.lpi2c1_stop_req(),
            self.lpi2c2_stop_req(),
            self.lpi2c3_stop_req(),
            self.lpi2c4_stop_req(),
            self.lpspi1_stop_req(),
            self.lpspi2_stop_req(),
            self.lpspi3_stop_req(),
            self.lpspi4_stop_req(),
            self.lpuart1_stop_req(),
            self.lpuart2_stop_req(),
            self.lpuart3_stop_req(),
            self.lpuart4_stop_req(),
            self.lpuart5_stop_req(),
            self.lpuart6_stop_req(),
            self.lpuart7_stop_req(),
            self.lpuart8_stop_req(),
            self.lpi2c1_stop_ack(),
            self.lpi2c2_stop_ack(),
            self.lpi2c3_stop_ack(),
            self.lpi2c4_stop_ack(),
            self.lpspi1_stop_ack(),
            self.lpspi2_stop_ack(),
            self.lpspi3_stop_ack(),
            self.lpspi4_stop_ack(),
            self.lpuart1_stop_ack(),
            self.lpuart2_stop_ack(),
            self.lpuart3_stop_ack(),
            self.lpuart4_stop_ack(),
            self.lpuart5_stop_ack(),
            self.lpuart6_stop_ack(),
            self.lpuart7_stop_ack(),
            self.lpuart8_stop_ack()
        )
    }
}
#[doc = "GPR8 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr8(pub u32);
impl Gpr8 {
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_ipg_stop_mode(&self) -> super::vals::Lpi2c1IpgStopMode {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpi2c1IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_stop_mode(&mut self, val: super::vals::Lpi2c1IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_ipg_doze(&self) -> super::vals::Lpi2c1IpgDoze {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpi2c1IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_doze(&mut self, val: super::vals::Lpi2c1IpgDoze) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_ipg_stop_mode(&self) -> super::vals::Lpi2c2IpgStopMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lpi2c2IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_stop_mode(&mut self, val: super::vals::Lpi2c2IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_ipg_doze(&self) -> super::vals::Lpi2c2IpgDoze {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lpi2c2IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_doze(&mut self, val: super::vals::Lpi2c2IpgDoze) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3_ipg_stop_mode(&self) -> super::vals::Lpi2c3IpgStopMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Lpi2c3IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPI2C3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c3_ipg_stop_mode(&mut self, val: super::vals::Lpi2c3IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPI2C3 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c3_ipg_doze(&self) -> super::vals::Lpi2c3IpgDoze {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Lpi2c3IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPI2C3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c3_ipg_doze(&mut self, val: super::vals::Lpi2c3IpgDoze) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4_ipg_stop_mode(&self) -> super::vals::Lpi2c4IpgStopMode {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Lpi2c4IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPI2C4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c4_ipg_stop_mode(&mut self, val: super::vals::Lpi2c4IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "LPI2C4 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c4_ipg_doze(&self) -> super::vals::Lpi2c4IpgDoze {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Lpi2c4IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPI2C4 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c4_ipg_doze(&mut self, val: super::vals::Lpi2c4IpgDoze) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_ipg_stop_mode(&self) -> super::vals::Lpspi1IpgStopMode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lpspi1IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_stop_mode(&mut self, val: super::vals::Lpspi1IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_ipg_doze(&self) -> super::vals::Lpspi1IpgDoze {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lpspi1IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_doze(&mut self, val: super::vals::Lpspi1IpgDoze) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_ipg_stop_mode(&self) -> super::vals::Lpspi2IpgStopMode {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Lpspi2IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_stop_mode(&mut self, val: super::vals::Lpspi2IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_ipg_doze(&self) -> super::vals::Lpspi2IpgDoze {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Lpspi2IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_doze(&mut self, val: super::vals::Lpspi2IpgDoze) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3_ipg_stop_mode(&self) -> super::vals::Lpspi3IpgStopMode {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Lpspi3IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPSPI3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi3_ipg_stop_mode(&mut self, val: super::vals::Lpspi3IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "LPSPI3 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi3_ipg_doze(&self) -> super::vals::Lpspi3IpgDoze {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Lpspi3IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPSPI3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi3_ipg_doze(&mut self, val: super::vals::Lpspi3IpgDoze) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4_ipg_stop_mode(&self) -> super::vals::Lpspi4IpgStopMode {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Lpspi4IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPSPI4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi4_ipg_stop_mode(&mut self, val: super::vals::Lpspi4IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "LPSPI4 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi4_ipg_doze(&self) -> super::vals::Lpspi4IpgDoze {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Lpspi4IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPSPI4 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi4_ipg_doze(&mut self, val: super::vals::Lpspi4IpgDoze) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_ipg_stop_mode(&self) -> super::vals::Lpuart1IpgStopMode {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Lpuart1IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_stop_mode(&mut self, val: super::vals::Lpuart1IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_ipg_doze(&self) -> super::vals::Lpuart1IpgDoze {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Lpuart1IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_doze(&mut self, val: super::vals::Lpuart1IpgDoze) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_ipg_stop_mode(&self) -> super::vals::Lpuart2IpgStopMode {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Lpuart2IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_stop_mode(&mut self, val: super::vals::Lpuart2IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_ipg_doze(&self) -> super::vals::Lpuart2IpgDoze {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Lpuart2IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_doze(&mut self, val: super::vals::Lpuart2IpgDoze) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_ipg_stop_mode(&self) -> super::vals::Lpuart3IpgStopMode {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Lpuart3IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_stop_mode(&mut self, val: super::vals::Lpuart3IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_ipg_doze(&self) -> super::vals::Lpuart3IpgDoze {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Lpuart3IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_doze(&mut self, val: super::vals::Lpuart3IpgDoze) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_ipg_stop_mode(&self) -> super::vals::Lpuart4IpgStopMode {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Lpuart4IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_stop_mode(&mut self, val: super::vals::Lpuart4IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_ipg_doze(&self) -> super::vals::Lpuart4IpgDoze {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Lpuart4IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_doze(&mut self, val: super::vals::Lpuart4IpgDoze) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5_ipg_stop_mode(&self) -> super::vals::Lpuart5IpgStopMode {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Lpuart5IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART5 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart5_ipg_stop_mode(&mut self, val: super::vals::Lpuart5IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART5 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart5_ipg_doze(&self) -> super::vals::Lpuart5IpgDoze {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Lpuart5IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART5 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart5_ipg_doze(&mut self, val: super::vals::Lpuart5IpgDoze) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart6_ipg_stop_mode(&self) -> super::vals::Lpuart6IpgStopMode {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Lpuart6IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART6 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart6_ipg_stop_mode(&mut self, val: super::vals::Lpuart6IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART6 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart6_ipg_doze(&self) -> super::vals::Lpuart6IpgDoze {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Lpuart6IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART6 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart6_ipg_doze(&mut self, val: super::vals::Lpuart6IpgDoze) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart7_ipg_stop_mode(&self) -> super::vals::Lpuart7IpgStopMode {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Lpuart7IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART7 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart7_ipg_stop_mode(&mut self, val: super::vals::Lpuart7IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "LPUART7 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart7_ipg_doze(&self) -> super::vals::Lpuart7IpgDoze {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Lpuart7IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART7 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart7_ipg_doze(&mut self, val: super::vals::Lpuart7IpgDoze) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart8_ipg_stop_mode(&self) -> super::vals::Lpuart8IpgStopMode {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Lpuart8IpgStopMode::from_bits(val as u8)
    }
    #[doc = "LPUART8 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart8_ipg_stop_mode(&mut self, val: super::vals::Lpuart8IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "LPUART8 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart8_ipg_doze(&self) -> super::vals::Lpuart8IpgDoze {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lpuart8IpgDoze::from_bits(val as u8)
    }
    #[doc = "LPUART8 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart8_ipg_doze(&mut self, val: super::vals::Lpuart8IpgDoze) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Gpr8 {
    #[inline(always)]
    fn default() -> Gpr8 {
        Gpr8(0)
    }
}
impl core::fmt::Debug for Gpr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpr8")
            .field("lpi2c1_ipg_stop_mode", &self.lpi2c1_ipg_stop_mode())
            .field("lpi2c1_ipg_doze", &self.lpi2c1_ipg_doze())
            .field("lpi2c2_ipg_stop_mode", &self.lpi2c2_ipg_stop_mode())
            .field("lpi2c2_ipg_doze", &self.lpi2c2_ipg_doze())
            .field("lpi2c3_ipg_stop_mode", &self.lpi2c3_ipg_stop_mode())
            .field("lpi2c3_ipg_doze", &self.lpi2c3_ipg_doze())
            .field("lpi2c4_ipg_stop_mode", &self.lpi2c4_ipg_stop_mode())
            .field("lpi2c4_ipg_doze", &self.lpi2c4_ipg_doze())
            .field("lpspi1_ipg_stop_mode", &self.lpspi1_ipg_stop_mode())
            .field("lpspi1_ipg_doze", &self.lpspi1_ipg_doze())
            .field("lpspi2_ipg_stop_mode", &self.lpspi2_ipg_stop_mode())
            .field("lpspi2_ipg_doze", &self.lpspi2_ipg_doze())
            .field("lpspi3_ipg_stop_mode", &self.lpspi3_ipg_stop_mode())
            .field("lpspi3_ipg_doze", &self.lpspi3_ipg_doze())
            .field("lpspi4_ipg_stop_mode", &self.lpspi4_ipg_stop_mode())
            .field("lpspi4_ipg_doze", &self.lpspi4_ipg_doze())
            .field("lpuart1_ipg_stop_mode", &self.lpuart1_ipg_stop_mode())
            .field("lpuart1_ipg_doze", &self.lpuart1_ipg_doze())
            .field("lpuart2_ipg_stop_mode", &self.lpuart2_ipg_stop_mode())
            .field("lpuart2_ipg_doze", &self.lpuart2_ipg_doze())
            .field("lpuart3_ipg_stop_mode", &self.lpuart3_ipg_stop_mode())
            .field("lpuart3_ipg_doze", &self.lpuart3_ipg_doze())
            .field("lpuart4_ipg_stop_mode", &self.lpuart4_ipg_stop_mode())
            .field("lpuart4_ipg_doze", &self.lpuart4_ipg_doze())
            .field("lpuart5_ipg_stop_mode", &self.lpuart5_ipg_stop_mode())
            .field("lpuart5_ipg_doze", &self.lpuart5_ipg_doze())
            .field("lpuart6_ipg_stop_mode", &self.lpuart6_ipg_stop_mode())
            .field("lpuart6_ipg_doze", &self.lpuart6_ipg_doze())
            .field("lpuart7_ipg_stop_mode", &self.lpuart7_ipg_stop_mode())
            .field("lpuart7_ipg_doze", &self.lpuart7_ipg_doze())
            .field("lpuart8_ipg_stop_mode", &self.lpuart8_ipg_stop_mode())
            .field("lpuart8_ipg_doze", &self.lpuart8_ipg_doze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr8 {{ lpi2c1_ipg_stop_mode: {:?}, lpi2c1_ipg_doze: {:?}, lpi2c2_ipg_stop_mode: {:?}, lpi2c2_ipg_doze: {:?}, lpi2c3_ipg_stop_mode: {:?}, lpi2c3_ipg_doze: {:?}, lpi2c4_ipg_stop_mode: {:?}, lpi2c4_ipg_doze: {:?}, lpspi1_ipg_stop_mode: {:?}, lpspi1_ipg_doze: {:?}, lpspi2_ipg_stop_mode: {:?}, lpspi2_ipg_doze: {:?}, lpspi3_ipg_stop_mode: {:?}, lpspi3_ipg_doze: {:?}, lpspi4_ipg_stop_mode: {:?}, lpspi4_ipg_doze: {:?}, lpuart1_ipg_stop_mode: {:?}, lpuart1_ipg_doze: {:?}, lpuart2_ipg_stop_mode: {:?}, lpuart2_ipg_doze: {:?}, lpuart3_ipg_stop_mode: {:?}, lpuart3_ipg_doze: {:?}, lpuart4_ipg_stop_mode: {:?}, lpuart4_ipg_doze: {:?}, lpuart5_ipg_stop_mode: {:?}, lpuart5_ipg_doze: {:?}, lpuart6_ipg_stop_mode: {:?}, lpuart6_ipg_doze: {:?}, lpuart7_ipg_stop_mode: {:?}, lpuart7_ipg_doze: {:?}, lpuart8_ipg_stop_mode: {:?}, lpuart8_ipg_doze: {:?} }}",
            self.lpi2c1_ipg_stop_mode(),
            self.lpi2c1_ipg_doze(),
            self.lpi2c2_ipg_stop_mode(),
            self.lpi2c2_ipg_doze(),
            self.lpi2c3_ipg_stop_mode(),
            self.lpi2c3_ipg_doze(),
            self.lpi2c4_ipg_stop_mode(),
            self.lpi2c4_ipg_doze(),
            self.lpspi1_ipg_stop_mode(),
            self.lpspi1_ipg_doze(),
            self.lpspi2_ipg_stop_mode(),
            self.lpspi2_ipg_doze(),
            self.lpspi3_ipg_stop_mode(),
            self.lpspi3_ipg_doze(),
            self.lpspi4_ipg_stop_mode(),
            self.lpspi4_ipg_doze(),
            self.lpuart1_ipg_stop_mode(),
            self.lpuart1_ipg_doze(),
            self.lpuart2_ipg_stop_mode(),
            self.lpuart2_ipg_doze(),
            self.lpuart3_ipg_stop_mode(),
            self.lpuart3_ipg_doze(),
            self.lpuart4_ipg_stop_mode(),
            self.lpuart4_ipg_doze(),
            self.lpuart5_ipg_stop_mode(),
            self.lpuart5_ipg_doze(),
            self.lpuart6_ipg_stop_mode(),
            self.lpuart6_ipg_doze(),
            self.lpuart7_ipg_stop_mode(),
            self.lpuart7_ipg_doze(),
            self.lpuart8_ipg_stop_mode(),
            self.lpuart8_ipg_doze()
        )
    }
}
