#[doc = "SRC General Purpose Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr1(pub u32);
impl Gpr1 {
    #[doc = "Holds entry function for core0 for waking-up from low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn persistent_entry0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Holds entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    pub const fn set_persistent_entry0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("persistent_entry0", &self.persistent_entry0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr1 {{ persistent_entry0: {=u32:?} }}",
            self.persistent_entry0()
        )
    }
}
#[doc = "SRC General Purpose Register 10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr10(pub u32);
impl Gpr10 {
    #[doc = "This field identifies which image must be used - 0/1/2/3"]
    #[must_use]
    #[inline(always)]
    pub const fn persist_redundant_boot(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "This field identifies which image must be used - 0/1/2/3"]
    #[inline(always)]
    pub const fn set_persist_redundant_boot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "This bit identifies which image must be used - primary and secondary"]
    #[must_use]
    #[inline(always)]
    pub const fn persist_secondary_boot(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This bit identifies which image must be used - primary and secondary"]
    #[inline(always)]
    pub const fn set_persist_secondary_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("persist_redundant_boot", &self.persist_redundant_boot())
            .field("persist_secondary_boot", &self.persist_secondary_boot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr10 {{ persist_redundant_boot: {=u8:?}, persist_secondary_boot: {=bool:?} }}",
            self.persist_redundant_boot(),
            self.persist_secondary_boot()
        )
    }
}
#[doc = "SRC General Purpose Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpr2(pub u32);
impl Gpr2 {
    #[doc = "Holds argument of entry function for core0 for waking-up from low power mode"]
    #[must_use]
    #[inline(always)]
    pub const fn persistent_arg0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Holds argument of entry function for core0 for waking-up from low power mode"]
    #[inline(always)]
    pub const fn set_persistent_arg0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            .field("persistent_arg0", &self.persistent_arg0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Gpr2 {{ persistent_arg0: {=u32:?} }}",
            self.persistent_arg0()
        )
    }
}
#[doc = "SRC Boot Mode Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbmr1(pub u32);
impl Sbmr1 {
    #[doc = "Refer to fusemap."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Refer to fusemap."]
    #[inline(always)]
    pub const fn set_boot_cfg1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Refer to fusemap."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Refer to fusemap."]
    #[inline(always)]
    pub const fn set_boot_cfg2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Refer to fusemap."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg3(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Refer to fusemap."]
    #[inline(always)]
    pub const fn set_boot_cfg3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Refer to fusemap."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_cfg4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Refer to fusemap."]
    #[inline(always)]
    pub const fn set_boot_cfg4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Sbmr1 {
    #[inline(always)]
    fn default() -> Sbmr1 {
        Sbmr1(0)
    }
}
impl core::fmt::Debug for Sbmr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbmr1")
            .field("boot_cfg1", &self.boot_cfg1())
            .field("boot_cfg2", &self.boot_cfg2())
            .field("boot_cfg3", &self.boot_cfg3())
            .field("boot_cfg4", &self.boot_cfg4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbmr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sbmr1 {{ boot_cfg1: {=u8:?}, boot_cfg2: {=u8:?}, boot_cfg3: {=u8:?}, boot_cfg4: {=u8:?} }}",
            self.boot_cfg1(),
            self.boot_cfg2(),
            self.boot_cfg3(),
            self.boot_cfg4()
        )
    }
}
#[doc = "SRC Boot Mode Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbmr2(pub u32);
impl Sbmr2 {
    #[doc = "SECONFIG\\[1\\] shows the state of the SECONFIG\\[1\\] fuse"]
    #[must_use]
    #[inline(always)]
    pub const fn sec_config(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "SECONFIG\\[1\\] shows the state of the SECONFIG\\[1\\] fuse"]
    #[inline(always)]
    pub const fn set_sec_config(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_fuse_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BT_FUSE_SEL (connected to gpio bt_fuse_sel) shows the state of the BT_FUSE_SEL fuse"]
    #[inline(always)]
    pub const fn set_bt_fuse_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "BMOD\\[1:0\\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    #[must_use]
    #[inline(always)]
    pub const fn bmod(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "BMOD\\[1:0\\] shows the latched state of the BOOT_MODE1 and BOOT_MODE0 signals on the rising edge of POR_B"]
    #[inline(always)]
    pub const fn set_bmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Sbmr2 {
    #[inline(always)]
    fn default() -> Sbmr2 {
        Sbmr2(0)
    }
}
impl core::fmt::Debug for Sbmr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbmr2")
            .field("sec_config", &self.sec_config())
            .field("bt_fuse_sel", &self.bt_fuse_sel())
            .field("bmod", &self.bmod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbmr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sbmr2 {{ sec_config: {=u8:?}, bt_fuse_sel: {=bool:?}, bmod: {=u8:?} }}",
            self.sec_config(),
            self.bt_fuse_sel(),
            self.bmod()
        )
    }
}
#[doc = "SRC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr(pub u32);
impl Scr {
    #[doc = "lockup reset enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn lockup_rst(&self) -> super::vals::LockupRst {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::LockupRst::from_bits(val as u8)
    }
    #[doc = "lockup reset enable bit"]
    #[inline(always)]
    pub const fn set_lockup_rst(&mut self, val: super::vals::LockupRst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Mask wdog_rst_b source"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_wdog_rst(&self) -> super::vals::MaskWdogRst {
        let val = (self.0 >> 7usize) & 0x0f;
        super::vals::MaskWdogRst::from_bits(val as u8)
    }
    #[doc = "Mask wdog_rst_b source"]
    #[inline(always)]
    pub const fn set_mask_wdog_rst(&mut self, val: super::vals::MaskWdogRst) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val.to_bits() as u32) & 0x0f) << 7usize);
    }
    #[doc = "Software reset for core0 only"]
    #[must_use]
    #[inline(always)]
    pub const fn core0_rst(&self) -> super::vals::Core0Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Core0Rst::from_bits(val as u8)
    }
    #[doc = "Software reset for core0 only"]
    #[inline(always)]
    pub const fn set_core0_rst(&mut self, val: super::vals::Core0Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Software reset for core0 debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn core0_dbg_rst(&self) -> super::vals::Core0DbgRst {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Core0DbgRst::from_bits(val as u8)
    }
    #[doc = "Software reset for core0 debug only"]
    #[inline(always)]
    pub const fn set_core0_dbg_rst(&mut self, val: super::vals::Core0DbgRst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Do not assert debug resets after power gating event of core"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_rst_msk_pg(&self) -> super::vals::DbgRstMskPg {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::DbgRstMskPg::from_bits(val as u8)
    }
    #[doc = "Do not assert debug resets after power gating event of core"]
    #[inline(always)]
    pub const fn set_dbg_rst_msk_pg(&mut self, val: super::vals::DbgRstMskPg) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Mask wdog3_rst_b source"]
    #[must_use]
    #[inline(always)]
    pub const fn mask_wdog3_rst(&self) -> super::vals::MaskWdog3Rst {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::MaskWdog3Rst::from_bits(val as u8)
    }
    #[doc = "Mask wdog3_rst_b source"]
    #[inline(always)]
    pub const fn set_mask_wdog3_rst(&mut self, val: super::vals::MaskWdog3Rst) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        Scr(0)
    }
}
impl core::fmt::Debug for Scr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scr")
            .field("lockup_rst", &self.lockup_rst())
            .field("mask_wdog_rst", &self.mask_wdog_rst())
            .field("core0_rst", &self.core0_rst())
            .field("core0_dbg_rst", &self.core0_dbg_rst())
            .field("dbg_rst_msk_pg", &self.dbg_rst_msk_pg())
            .field("mask_wdog3_rst", &self.mask_wdog3_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scr {{ lockup_rst: {:?}, mask_wdog_rst: {:?}, core0_rst: {:?}, core0_dbg_rst: {:?}, dbg_rst_msk_pg: {:?}, mask_wdog3_rst: {:?} }}",
            self.lockup_rst(),
            self.mask_wdog_rst(),
            self.core0_rst(),
            self.core0_dbg_rst(),
            self.dbg_rst_msk_pg(),
            self.mask_wdog3_rst()
        )
    }
}
#[doc = "SRC Reset Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srsr(pub u32);
impl Srsr {
    #[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_reset_b(&self) -> super::vals::IppResetB {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IppResetB::from_bits(val as u8)
    }
    #[doc = "Indicates whether reset was the result of ipp_reset_b pin (Power-up sequence)"]
    #[inline(always)]
    pub const fn set_ipp_reset_b(&mut self, val: super::vals::IppResetB) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates a reset has been caused by CPU lockup."]
    #[must_use]
    #[inline(always)]
    pub const fn lockup(&self) -> super::vals::Lockup {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lockup::from_bits(val as u8)
    }
    #[doc = "Indicates a reset has been caused by CPU lockup."]
    #[inline(always)]
    pub const fn set_lockup(&mut self, val: super::vals::Lockup) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates whether the reset was the result of the csu_reset_b input."]
    #[must_use]
    #[inline(always)]
    pub const fn csu_reset_b(&self) -> super::vals::CsuResetB {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CsuResetB::from_bits(val as u8)
    }
    #[doc = "Indicates whether the reset was the result of the csu_reset_b input."]
    #[inline(always)]
    pub const fn set_csu_reset_b(&mut self, val: super::vals::CsuResetB) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_user_reset_b(&self) -> super::vals::IppUserResetB {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::IppUserResetB::from_bits(val as u8)
    }
    #[doc = "Indicates whether the reset was the result of the ipp_user_reset_b qualified reset."]
    #[inline(always)]
    pub const fn set_ipp_user_reset_b(&mut self, val: super::vals::IppUserResetB) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IC Watchdog Time-out reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog_rst_b(&self) -> super::vals::WdogRstB {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::WdogRstB::from_bits(val as u8)
    }
    #[doc = "IC Watchdog Time-out reset"]
    #[inline(always)]
    pub const fn set_wdog_rst_b(&mut self, val: super::vals::WdogRstB) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_rst_b(&self) -> super::vals::JtagRstB {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::JtagRstB::from_bits(val as u8)
    }
    #[doc = "HIGH - Z JTAG reset. Indicates whether the reset was the result of HIGH-Z reset from JTAG."]
    #[inline(always)]
    pub const fn set_jtag_rst_b(&mut self, val: super::vals::JtagRstB) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "JTAG software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_sw_rst(&self) -> super::vals::JtagSwRst {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::JtagSwRst::from_bits(val as u8)
    }
    #[doc = "JTAG software reset"]
    #[inline(always)]
    pub const fn set_jtag_sw_rst(&mut self, val: super::vals::JtagSwRst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "IC Watchdog3 Time-out reset"]
    #[must_use]
    #[inline(always)]
    pub const fn wdog3_rst_b(&self) -> super::vals::Wdog3RstB {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Wdog3RstB::from_bits(val as u8)
    }
    #[doc = "IC Watchdog3 Time-out reset"]
    #[inline(always)]
    pub const fn set_wdog3_rst_b(&mut self, val: super::vals::Wdog3RstB) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Temper Sensor software reset"]
    #[must_use]
    #[inline(always)]
    pub const fn tempsense_rst_b(&self) -> super::vals::TempsenseRstB {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::TempsenseRstB::from_bits(val as u8)
    }
    #[doc = "Temper Sensor software reset"]
    #[inline(always)]
    pub const fn set_tempsense_rst_b(&mut self, val: super::vals::TempsenseRstB) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Srsr {
    #[inline(always)]
    fn default() -> Srsr {
        Srsr(0)
    }
}
impl core::fmt::Debug for Srsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Srsr")
            .field("ipp_reset_b", &self.ipp_reset_b())
            .field("lockup", &self.lockup())
            .field("csu_reset_b", &self.csu_reset_b())
            .field("ipp_user_reset_b", &self.ipp_user_reset_b())
            .field("wdog_rst_b", &self.wdog_rst_b())
            .field("jtag_rst_b", &self.jtag_rst_b())
            .field("jtag_sw_rst", &self.jtag_sw_rst())
            .field("wdog3_rst_b", &self.wdog3_rst_b())
            .field("tempsense_rst_b", &self.tempsense_rst_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Srsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Srsr {{ ipp_reset_b: {:?}, lockup: {:?}, csu_reset_b: {:?}, ipp_user_reset_b: {:?}, wdog_rst_b: {:?}, jtag_rst_b: {:?}, jtag_sw_rst: {:?}, wdog3_rst_b: {:?}, tempsense_rst_b: {:?} }}",
            self.ipp_reset_b(),
            self.lockup(),
            self.csu_reset_b(),
            self.ipp_user_reset_b(),
            self.wdog_rst_b(),
            self.jtag_rst_b(),
            self.jtag_sw_rst(),
            self.wdog3_rst_b(),
            self.tempsense_rst_b()
        )
    }
}
