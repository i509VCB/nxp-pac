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
    pub const fn gint(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Global Interrupt"]
    #[inline(always)]
    pub const fn set_gint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    pub const fn exc_mon(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub const fn set_exc_mon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cm7_force_hclk_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    pub const fn set_cm7_force_hclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("sai3_mclk3_sel", &self.sai3_mclk3_sel())
            .field("gint", &self.gint())
            .field("sai1_mclk_dir", &self.sai1_mclk_dir())
            .field("sai3_mclk_dir", &self.sai3_mclk_dir())
            .field("exc_mon", &self.exc_mon())
            .field("cm7_force_hclk_en", &self.cm7_force_hclk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr1 {{ sai1_mclk1_sel: {:?}, sai1_mclk2_sel: {:?}, sai1_mclk3_sel: {:?}, sai3_mclk3_sel: {:?}, gint: {=bool:?}, sai1_mclk_dir: {:?}, sai3_mclk_dir: {:?}, exc_mon: {=bool:?}, cm7_force_hclk_en: {=bool:?} }}",
            self.sai1_mclk1_sel(),
            self.sai1_mclk2_sel(),
            self.sai1_mclk3_sel(),
            self.sai3_mclk3_sel(),
            self.gint(),
            self.sai1_mclk_dir(),
            self.sai3_mclk_dir(),
            self.exc_mon(),
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
    pub const fn niden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Arm non-secure (non-invasive) debug enable"]
    #[inline(always)]
    pub const fn set_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Arm invasive debug enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Arm invasive debug enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_err_resp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Security error response enable for all security gaskets (on both AHB and AXI buses)"]
    #[inline(always)]
    pub const fn set_sec_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DCP Key selection bit."]
    #[must_use]
    #[inline(always)]
    pub const fn dcpkey_ocotp_or_keymux(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DCP Key selection bit."]
    #[inline(always)]
    pub const fn set_dcpkey_ocotp_or_keymux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_tz_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "OCRAM TrustZone (TZ) enable."]
    #[inline(always)]
    pub const fn set_ocram_tz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[must_use]
    #[inline(always)]
    pub const fn ocram_tz_addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "OCRAM TrustZone (TZ) start address"]
    #[inline(always)]
    pub const fn set_ocram_tz_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "Lock NIDEN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_niden(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Lock NIDEN field for changes"]
    #[inline(always)]
    pub const fn set_lock_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_dbg_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Lock DBG_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_dbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_sec_err_resp(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Lock SEC_ERR_RESP field for changes"]
    #[inline(always)]
    pub const fn set_lock_sec_err_resp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_dcpkey_ocotp_or_keymux(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Lock DCP Key OCOTP/Key MUX selection bit"]
    #[inline(always)]
    pub const fn set_lock_dcpkey_ocotp_or_keymux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram_tz_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Lock OCRAM_TZ_EN field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ocram_tz_addr(&self) -> super::vals::LockOcramTzAddr {
        let val = (self.0 >> 25usize) & 0x1f;
        super::vals::LockOcramTzAddr::from_bits(val as u8)
    }
    #[doc = "Lock OCRAM_TZ_ADDR field for changes"]
    #[inline(always)]
    pub const fn set_lock_ocram_tz_addr(&mut self, val: super::vals::LockOcramTzAddr) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val.to_bits() as u32) & 0x1f) << 25usize);
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
            "Gpr10 {{ niden: {=bool:?}, dbg_en: {=bool:?}, sec_err_resp: {=bool:?}, dcpkey_ocotp_or_keymux: {=bool:?}, ocram_tz_en: {=bool:?}, ocram_tz_addr: {=u8:?}, lock_niden: {=bool:?}, lock_dbg_en: {=bool:?}, lock_sec_err_resp: {=bool:?}, lock_dcpkey_ocotp_or_keymux: {=bool:?}, lock_ocram_tz_en: {=bool:?}, lock_ocram_tz_addr: {:?} }}",
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
    #[doc = "Lock M7_APC_AC_R0_CTRL field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r0_ctrl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R0_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Lock M7_APC_AC_R1_CTRL field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r1_ctrl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R1_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Lock M7_APC_AC_R2_CTRL field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r2_ctrl(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R2_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Lock M7_APC_AC_R3_CTRL field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_m7_apc_ac_r3_ctrl(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Lock M7_APC_AC_R3_CTRL field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
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
            .field("lock_m7_apc_ac_r0_ctrl", &self.lock_m7_apc_ac_r0_ctrl())
            .field("lock_m7_apc_ac_r1_ctrl", &self.lock_m7_apc_ac_r1_ctrl())
            .field("lock_m7_apc_ac_r2_ctrl", &self.lock_m7_apc_ac_r2_ctrl())
            .field("lock_m7_apc_ac_r3_ctrl", &self.lock_m7_apc_ac_r3_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr11 {{ m7_apc_ac_r0_ctrl: {:?}, m7_apc_ac_r1_ctrl: {:?}, m7_apc_ac_r2_ctrl: {:?}, m7_apc_ac_r3_ctrl: {:?}, lock_m7_apc_ac_r0_ctrl: {=u8:?}, lock_m7_apc_ac_r1_ctrl: {=u8:?}, lock_m7_apc_ac_r2_ctrl: {=u8:?}, lock_m7_apc_ac_r3_ctrl: {=u8:?} }}",
            self.m7_apc_ac_r0_ctrl(),
            self.m7_apc_ac_r1_ctrl(),
            self.m7_apc_ac_r2_ctrl(),
            self.m7_apc_ac_r3_ctrl(),
            self.lock_m7_apc_ac_r0_ctrl(),
            self.lock_m7_apc_ac_r1_ctrl(),
            self.lock_m7_apc_ac_r2_ctrl(),
            self.lock_m7_apc_ac_r3_ctrl()
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
    pub const fn flexio1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO1 stop mode selection. Cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_flexio1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_flexio1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr12 {{ flexio1_ipg_stop_mode: {=bool:?}, flexio1_ipg_doze: {=bool:?} }}",
            self.flexio1_ipg_stop_mode(),
            self.flexio1_ipg_doze()
        )
    }
}
#[doc = "GPR13 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr13(pub u32);
impl Gpr13 {
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_usb(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub const fn set_cache_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("cache_usb", &self.cache_usb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpr13 {{ cache_usb: {=bool:?} }}", self.cache_usb())
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
    pub const fn flexram_bank_cfg_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FlexRAM bank config source select"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Lock CM7_INIT_VTOR field for changes"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_vtor(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Lock CM7_INIT_VTOR field for changes"]
    #[inline(always)]
    pub const fn set_lock_vtor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
            .field("lock_vtor", &self.lock_vtor())
            .field("cm7_init_vtor", &self.cm7_init_vtor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr16 {{ flexram_bank_cfg_sel: {=bool:?}, lock_vtor: {=bool:?}, cm7_init_vtor: {=u32:?} }}",
            self.flexram_bank_cfg_sel(),
            self.lock_vtor(),
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
    pub const fn flexram_bank_cfg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "FlexRAM bank config value"]
    #[inline(always)]
    pub const fn set_flexram_bank_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            "Gpr17 {{ flexram_bank_cfg: {=u8:?} }}",
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
    pub const fn lock_m7_apc_ac_r0_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R0_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr18 {{ lock_m7_apc_ac_r0_bot: {=bool:?}, m7_apc_ac_r0_bot: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r0_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R0_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r0_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr19 {{ lock_m7_apc_ac_r0_top: {=bool:?}, m7_apc_ac_r0_top: {=u32:?} }}",
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
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_m0_high_priority(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "AXBS_P M0 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m0_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_m1_high_priority(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AXBS_P M1 master has higher priority.Do not set both M1 and M0 to high priority."]
    #[inline(always)]
    pub const fn set_axbs_p_m1_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn axbs_p_force_round_robin(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Force Round Robin in AXBS_P. This bit can override master M0 M1 high priority configuration."]
    #[inline(always)]
    pub const fn set_axbs_p_force_round_robin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable power saving features on L2 memory"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_mem_en_powersaving(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable power saving features on L2 memory"]
    #[inline(always)]
    pub const fn set_l2_mem_en_powersaving(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[must_use]
    #[inline(always)]
    pub const fn ram_auto_clk_gating_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Automatically gate off RAM clock when RAM is not accessed."]
    #[inline(always)]
    pub const fn set_ram_auto_clk_gating_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_mem_deepsleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This bit controls how memory (OCRAM) enters Deep Sleep mode (shutdown periphery power, but maintain memory contents, outputs of memory are pulled low"]
    #[inline(always)]
    pub const fn set_l2_mem_deepsleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Divider ratio control for mclk from hmclk"]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_clk_div(&self) -> super::vals::MqsClkDiv {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::MqsClkDiv::from_bits(val as u8)
    }
    #[doc = "Divider ratio control for mclk from hmclk"]
    #[inline(always)]
    pub const fn set_mqs_clk_div(&mut self, val: super::vals::MqsClkDiv) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "MQS software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_sw_rst(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "MQS software reset"]
    #[inline(always)]
    pub const fn set_mqs_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "MQS enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "MQS enable."]
    #[inline(always)]
    pub const fn set_mqs_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Medium Quality Sound (MQS) Oversample"]
    #[must_use]
    #[inline(always)]
    pub const fn mqs_oversample(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Medium Quality Sound (MQS) Oversample"]
    #[inline(always)]
    pub const fn set_mqs_oversample(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
            .field("axbs_p_m0_high_priority", &self.axbs_p_m0_high_priority())
            .field("axbs_p_m1_high_priority", &self.axbs_p_m1_high_priority())
            .field("axbs_p_force_round_robin", &self.axbs_p_force_round_robin())
            .field("l2_mem_en_powersaving", &self.l2_mem_en_powersaving())
            .field("ram_auto_clk_gating_en", &self.ram_auto_clk_gating_en())
            .field("l2_mem_deepsleep", &self.l2_mem_deepsleep())
            .field("mqs_clk_div", &self.mqs_clk_div())
            .field("mqs_sw_rst", &self.mqs_sw_rst())
            .field("mqs_en", &self.mqs_en())
            .field("mqs_oversample", &self.mqs_oversample())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr2 {{ axbs_p_m0_high_priority: {=bool:?}, axbs_p_m1_high_priority: {=bool:?}, axbs_p_force_round_robin: {=bool:?}, l2_mem_en_powersaving: {=bool:?}, ram_auto_clk_gating_en: {=bool:?}, l2_mem_deepsleep: {=bool:?}, mqs_clk_div: {:?}, mqs_sw_rst: {=bool:?}, mqs_en: {=bool:?}, mqs_oversample: {=bool:?} }}",
            self.axbs_p_m0_high_priority(),
            self.axbs_p_m1_high_priority(),
            self.axbs_p_force_round_robin(),
            self.l2_mem_en_powersaving(),
            self.ram_auto_clk_gating_en(),
            self.l2_mem_deepsleep(),
            self.mqs_clk_div(),
            self.mqs_sw_rst(),
            self.mqs_en(),
            self.mqs_oversample()
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
    pub const fn lock_m7_apc_ac_r1_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R1_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr20 {{ lock_m7_apc_ac_r1_bot: {=bool:?}, m7_apc_ac_r1_bot: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r1_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R1_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r1_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr21 {{ lock_m7_apc_ac_r1_top: {=bool:?}, m7_apc_ac_r1_top: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r2_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R2_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr22 {{ lock_m7_apc_ac_r2_bot: {=bool:?}, m7_apc_ac_r2_bot: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r2_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R2_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r2_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr23 {{ lock_m7_apc_ac_r2_top: {=bool:?}, m7_apc_ac_r2_top: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r3_bot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R3_BOT field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_bot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr24 {{ lock_m7_apc_ac_r3_bot: {=bool:?}, m7_apc_ac_r3_bot: {=u32:?} }}",
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
    pub const fn lock_m7_apc_ac_r3_top(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "lock M7_APC_AC_R3_TOP field for changes"]
    #[inline(always)]
    pub const fn set_lock_m7_apc_ac_r3_top(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
            "Gpr25 {{ lock_m7_apc_ac_r3_top: {=bool:?}, m7_apc_ac_r3_top: {=u32:?} }}",
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
    #[doc = "Select GPIO1 or GPIO2"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sel(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Select GPIO1 or GPIO2"]
    #[inline(always)]
    pub const fn set_gpio_sel(&mut self, val: u32) {
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
            .field("gpio_sel", &self.gpio_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpr26 {{ gpio_sel: {=u32:?} }}", self.gpio_sel())
    }
}
#[doc = "GPR27 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr27(pub u32);
impl Gpr27 {
    #[doc = "Start address of flexspi1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_start(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_start(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
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
            .field("flexspi_remap_addr_start", &self.flexspi_remap_addr_start())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr27 {{ flexspi_remap_addr_start: {=u32:?} }}",
            self.flexspi_remap_addr_start()
        )
    }
}
#[doc = "GPR28 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr28(pub u32);
impl Gpr28 {
    #[doc = "End address of flexspi1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_end(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_end(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
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
            .field("flexspi_remap_addr_end", &self.flexspi_remap_addr_end())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr28 {{ flexspi_remap_addr_end: {=u32:?} }}",
            self.flexspi_remap_addr_end()
        )
    }
}
#[doc = "GPR29 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr29(pub u32);
impl Gpr29 {
    #[doc = "Offset address of flexspi1"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_remap_addr_offset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Offset address of flexspi1"]
    #[inline(always)]
    pub const fn set_flexspi_remap_addr_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
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
            .field(
                "flexspi_remap_addr_offset",
                &self.flexspi_remap_addr_offset(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr29 {{ flexspi_remap_addr_offset: {=u32:?} }}",
            self.flexspi_remap_addr_offset()
        )
    }
}
#[doc = "GPR3 General Purpose Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr3(pub u32);
impl Gpr3 {
    #[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[must_use]
    #[inline(always)]
    pub const fn dcp_key_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Select 128-bit DCP key from 256-bit key from SNVS Master Key"]
    #[inline(always)]
    pub const fn set_dcp_key_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("dcp_key_sel", &self.dcp_key_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gpr3 {{ dcp_key_sel: {=bool:?} }}", self.dcp_key_sel())
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
    pub const fn edma_stop_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "EDMA stop request."]
    #[inline(always)]
    pub const fn set_edma_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TRNG stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_stop_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG stop request."]
    #[inline(always)]
    pub const fn set_trng_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SAI1 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_stop_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 stop request."]
    #[inline(always)]
    pub const fn set_sai1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SAI3 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_stop_req(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 stop request."]
    #[inline(always)]
    pub const fn set_sai3_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "PIT stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn pit_stop_req(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "PIT stop request."]
    #[inline(always)]
    pub const fn set_pit_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "FlexSPI stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_stop_req(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "FlexSPI stop request."]
    #[inline(always)]
    pub const fn set_flexspi_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "FlexIO1 stop request."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_stop_req(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "FlexIO1 stop request."]
    #[inline(always)]
    pub const fn set_flexio1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[must_use]
    #[inline(always)]
    pub const fn edma_stop_ack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "EDMA stop acknowledge. This is a status (read-only) bit"]
    #[inline(always)]
    pub const fn set_edma_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "TRNG stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn trng_stop_ack(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "TRNG stop acknowledge"]
    #[inline(always)]
    pub const fn set_trng_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SAI1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_stop_ack(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SAI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SAI3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_stop_ack(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SAI3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_sai3_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "PIT stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn pit_stop_ack(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "PIT stop acknowledge"]
    #[inline(always)]
    pub const fn set_pit_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_stop_ack(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXSPI stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexspi_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn flexio1_stop_ack(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_flexio1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("trng_stop_req", &self.trng_stop_req())
            .field("sai1_stop_req", &self.sai1_stop_req())
            .field("sai3_stop_req", &self.sai3_stop_req())
            .field("pit_stop_req", &self.pit_stop_req())
            .field("flexspi_stop_req", &self.flexspi_stop_req())
            .field("flexio1_stop_req", &self.flexio1_stop_req())
            .field("edma_stop_ack", &self.edma_stop_ack())
            .field("trng_stop_ack", &self.trng_stop_ack())
            .field("sai1_stop_ack", &self.sai1_stop_ack())
            .field("sai3_stop_ack", &self.sai3_stop_ack())
            .field("pit_stop_ack", &self.pit_stop_ack())
            .field("flexspi_stop_ack", &self.flexspi_stop_ack())
            .field("flexio1_stop_ack", &self.flexio1_stop_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr4 {{ edma_stop_req: {=bool:?}, trng_stop_req: {=bool:?}, sai1_stop_req: {=bool:?}, sai3_stop_req: {=bool:?}, pit_stop_req: {=bool:?}, flexspi_stop_req: {=bool:?}, flexio1_stop_req: {=bool:?}, edma_stop_ack: {=bool:?}, trng_stop_ack: {=bool:?}, sai1_stop_ack: {=bool:?}, sai3_stop_ack: {=bool:?}, pit_stop_ack: {=bool:?}, flexspi_stop_ack: {=bool:?}, flexio1_stop_ack: {=bool:?} }}",
            self.edma_stop_req(),
            self.trng_stop_req(),
            self.sai1_stop_req(),
            self.sai3_stop_req(),
            self.pit_stop_req(),
            self.flexspi_stop_req(),
            self.flexio1_stop_req(),
            self.edma_stop_ack(),
            self.trng_stop_ack(),
            self.sai1_stop_ack(),
            self.sai3_stop_ack(),
            self.pit_stop_ack(),
            self.flexspi_stop_ack(),
            self.flexio1_stop_ack()
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
    pub const fn wdog1_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG1 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog1_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog2_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG2 Timeout Mask"]
    #[inline(always)]
    pub const fn set_wdog2_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt1(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_1m_clk_gpt2(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub const fn set_vref_1m_clk_gpt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
            "Gpr5 {{ wdog1_mask: {=bool:?}, wdog2_mask: {=bool:?}, vref_1m_clk_gpt1: {=bool:?}, vref_1m_clk_gpt2: {=bool:?} }}",
            self.wdog1_mask(),
            self.wdog2_mask(),
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
    #[doc = "IOMUXC XBAR_INOUT2 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_2(&self) -> super::vals::IomuxcXbarDirSel2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::IomuxcXbarDirSel2::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT2 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_2(&mut self, val: super::vals::IomuxcXbarDirSel2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "IOMUXC XBAR_INOUT3 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_3(&self) -> super::vals::IomuxcXbarDirSel3 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::IomuxcXbarDirSel3::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT3 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_3(&mut self, val: super::vals::IomuxcXbarDirSel3) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
            .field("iomuxc_xbar_dir_sel_2", &self.iomuxc_xbar_dir_sel_2())
            .field("iomuxc_xbar_dir_sel_3", &self.iomuxc_xbar_dir_sel_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr6 {{ iomuxc_xbar_dir_sel_2: {:?}, iomuxc_xbar_dir_sel_3: {:?} }}",
            self.iomuxc_xbar_dir_sel_2(),
            self.iomuxc_xbar_dir_sel_3()
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
    pub const fn lpi2c1_stop_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C2 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_stop_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop request"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSPI1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_stop_req(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop request"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "LPSPI2 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_stop_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop request"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPUART1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_stop_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPUART1 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_stop_req(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop request"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPUART3 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_stop_req(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop request"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "LPUART4 stop request"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_stop_req(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop request"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_stop_ack(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_stop_ack(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpi2c2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_stop_ack(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_stop_ack(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpspi2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_stop_ack(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart1_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_stop_ack(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart2_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_stop_ack(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart3_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_stop_ack(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop acknowledge"]
    #[inline(always)]
    pub const fn set_lpuart4_stop_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
            .field("lpspi1_stop_req", &self.lpspi1_stop_req())
            .field("lpspi2_stop_req", &self.lpspi2_stop_req())
            .field("lpuart1_stop_req", &self.lpuart1_stop_req())
            .field("lpuart2_stop_req", &self.lpuart2_stop_req())
            .field("lpuart3_stop_req", &self.lpuart3_stop_req())
            .field("lpuart4_stop_req", &self.lpuart4_stop_req())
            .field("lpi2c1_stop_ack", &self.lpi2c1_stop_ack())
            .field("lpi2c2_stop_ack", &self.lpi2c2_stop_ack())
            .field("lpspi1_stop_ack", &self.lpspi1_stop_ack())
            .field("lpspi2_stop_ack", &self.lpspi2_stop_ack())
            .field("lpuart1_stop_ack", &self.lpuart1_stop_ack())
            .field("lpuart2_stop_ack", &self.lpuart2_stop_ack())
            .field("lpuart3_stop_ack", &self.lpuart3_stop_ack())
            .field("lpuart4_stop_ack", &self.lpuart4_stop_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr7 {{ lpi2c1_stop_req: {=bool:?}, lpi2c2_stop_req: {=bool:?}, lpspi1_stop_req: {=bool:?}, lpspi2_stop_req: {=bool:?}, lpuart1_stop_req: {=bool:?}, lpuart2_stop_req: {=bool:?}, lpuart3_stop_req: {=bool:?}, lpuart4_stop_req: {=bool:?}, lpi2c1_stop_ack: {=bool:?}, lpi2c2_stop_ack: {=bool:?}, lpspi1_stop_ack: {=bool:?}, lpspi2_stop_ack: {=bool:?}, lpuart1_stop_ack: {=bool:?}, lpuart2_stop_ack: {=bool:?}, lpuart3_stop_ack: {=bool:?}, lpuart4_stop_ack: {=bool:?} }}",
            self.lpi2c1_stop_req(),
            self.lpi2c2_stop_req(),
            self.lpspi1_stop_req(),
            self.lpspi2_stop_req(),
            self.lpuart1_stop_req(),
            self.lpuart2_stop_req(),
            self.lpuart3_stop_req(),
            self.lpuart4_stop_req(),
            self.lpi2c1_stop_ack(),
            self.lpi2c2_stop_ack(),
            self.lpspi1_stop_ack(),
            self.lpspi2_stop_ack(),
            self.lpuart1_stop_ack(),
            self.lpuart2_stop_ack(),
            self.lpuart3_stop_ack(),
            self.lpuart4_stop_ack()
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
    pub const fn lpi2c1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpi2c2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "LPI2C2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpi2c2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpspi2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPSPI2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpspi2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart1_ipg_doze(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART1 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart1_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart2_ipg_doze(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART2 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart2_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart3_ipg_doze(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART3 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart3_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_ipg_stop_mode(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 stop mode selection, cannot change when ipg_stop is asserted."]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_stop_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpuart4_ipg_doze(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "LPUART4 ipg_doze mode"]
    #[inline(always)]
    pub const fn set_lpuart4_ipg_doze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
            .field("lpspi1_ipg_stop_mode", &self.lpspi1_ipg_stop_mode())
            .field("lpspi1_ipg_doze", &self.lpspi1_ipg_doze())
            .field("lpspi2_ipg_stop_mode", &self.lpspi2_ipg_stop_mode())
            .field("lpspi2_ipg_doze", &self.lpspi2_ipg_doze())
            .field("lpuart1_ipg_stop_mode", &self.lpuart1_ipg_stop_mode())
            .field("lpuart1_ipg_doze", &self.lpuart1_ipg_doze())
            .field("lpuart2_ipg_stop_mode", &self.lpuart2_ipg_stop_mode())
            .field("lpuart2_ipg_doze", &self.lpuart2_ipg_doze())
            .field("lpuart3_ipg_stop_mode", &self.lpuart3_ipg_stop_mode())
            .field("lpuart3_ipg_doze", &self.lpuart3_ipg_doze())
            .field("lpuart4_ipg_stop_mode", &self.lpuart4_ipg_stop_mode())
            .field("lpuart4_ipg_doze", &self.lpuart4_ipg_doze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr8 {{ lpi2c1_ipg_stop_mode: {=bool:?}, lpi2c1_ipg_doze: {=bool:?}, lpi2c2_ipg_stop_mode: {=bool:?}, lpi2c2_ipg_doze: {=bool:?}, lpspi1_ipg_stop_mode: {=bool:?}, lpspi1_ipg_doze: {=bool:?}, lpspi2_ipg_stop_mode: {=bool:?}, lpspi2_ipg_doze: {=bool:?}, lpuart1_ipg_stop_mode: {=bool:?}, lpuart1_ipg_doze: {=bool:?}, lpuart2_ipg_stop_mode: {=bool:?}, lpuart2_ipg_doze: {=bool:?}, lpuart3_ipg_stop_mode: {=bool:?}, lpuart3_ipg_doze: {=bool:?}, lpuart4_ipg_stop_mode: {=bool:?}, lpuart4_ipg_doze: {=bool:?} }}",
            self.lpi2c1_ipg_stop_mode(),
            self.lpi2c1_ipg_doze(),
            self.lpi2c2_ipg_stop_mode(),
            self.lpi2c2_ipg_doze(),
            self.lpspi1_ipg_stop_mode(),
            self.lpspi1_ipg_doze(),
            self.lpspi2_ipg_stop_mode(),
            self.lpspi2_ipg_doze(),
            self.lpuart1_ipg_stop_mode(),
            self.lpuart1_ipg_doze(),
            self.lpuart2_ipg_stop_mode(),
            self.lpuart2_ipg_doze(),
            self.lpuart3_ipg_stop_mode(),
            self.lpuart3_ipg_doze(),
            self.lpuart4_ipg_stop_mode(),
            self.lpuart4_ipg_doze()
        )
    }
}
