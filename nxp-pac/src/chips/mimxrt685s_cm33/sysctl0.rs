#[doc = "system controller 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysctl0 {
    ptr: *mut u8,
}
unsafe impl Send for Sysctl0 {}
unsafe impl Sync for Sysctl0 {}
impl Sysctl0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DSP stall register"]
    #[inline(always)]
    pub const fn dspstall(self) -> crate::common::Reg<regs::Dspstall, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AHB matrix priority"]
    #[inline(always)]
    pub const fn ahbmatrixprior(
        self,
    ) -> crate::common::Reg<regs::Ahbmatrixprior, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Packer enable for DSP RAM packer"]
    #[inline(always)]
    pub const fn packerenable(self) -> crate::common::Reg<regs::Packerenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "M33 nmi source selection"]
    #[inline(always)]
    pub const fn m33nmisrcsel(self) -> crate::common::Reg<regs::M33nmisrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "system stick calibration"]
    #[inline(always)]
    pub const fn system_stick_calib(
        self,
    ) -> crate::common::Reg<regs::SystemStickCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "system nstick calibration"]
    #[inline(always)]
    pub const fn system_nstick_calib(
        self,
    ) -> crate::common::Reg<regs::SystemNstickCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "product ID"]
    #[inline(always)]
    pub const fn product_id(self) -> crate::common::Reg<regs::ProductId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SILICONREV ID"]
    #[inline(always)]
    pub const fn siliconrev_id(self) -> crate::common::Reg<regs::SiliconrevId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "jtag ID"]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::common::Reg<regs::JtagId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "auto clock gating override 0"]
    #[inline(always)]
    pub const fn autoclkgateoverride0(
        self,
    ) -> crate::common::Reg<regs::Autoclkgateoverride0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "auto clock gating override 1"]
    #[inline(always)]
    pub const fn autoclkgateoverride1(
        self,
    ) -> crate::common::Reg<regs::Autoclkgateoverride1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Clock gate override 0"]
    #[inline(always)]
    pub const fn clkgateoverride0(
        self,
    ) -> crate::common::Reg<regs::Clkgateoverride0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "AHB SRAM access disable"]
    #[inline(always)]
    pub const fn ahb_sram_access_disable(
        self,
    ) -> crate::common::Reg<regs::AhbSramAccessDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "DSP SRAM access disable"]
    #[inline(always)]
    pub const fn dsp_sram_access_disable(
        self,
    ) -> crate::common::Reg<regs::DspSramAccessDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "AHB Flexspi access control"]
    #[inline(always)]
    pub const fn ahb_flexspi_access_disable(
        self,
    ) -> crate::common::Reg<regs::AhbFlexspiAccessDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "DSP Flexspi access control"]
    #[inline(always)]
    pub const fn dsp_flexspi_access_disable(
        self,
    ) -> crate::common::Reg<regs::DspFlexspiAccessDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "FLEXSPI NOR flash configure context register"]
    #[inline(always)]
    pub const fn flexspi_bootrom_scratch0(
        self,
    ) -> crate::common::Reg<regs::FlexspiBootromScratch0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "USB clock control"]
    #[inline(always)]
    pub const fn usbclkctrl(self) -> crate::common::Reg<regs::Usbclkctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "USB clock status"]
    #[inline(always)]
    pub const fn usbclkstat(self) -> crate::common::Reg<regs::Usbclkstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "USB PHY PLL0 lock time division 2"]
    #[inline(always)]
    pub const fn usbphypll0locktimediv2(
        self,
    ) -> crate::common::Reg<regs::Usbphypll0locktimediv2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Sleep configuration 0"]
    #[inline(always)]
    pub const fn pdsleepcfg0(self) -> crate::common::Reg<regs::Pdsleepcfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Sleep configuration 1"]
    #[inline(always)]
    pub const fn pdsleepcfg1(self) -> crate::common::Reg<regs::Pdsleepcfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "Sleep configuration 2"]
    #[inline(always)]
    pub const fn pdsleepcfg2(self) -> crate::common::Reg<regs::Pdsleepcfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "Sleep configuration 3"]
    #[inline(always)]
    pub const fn pdsleepcfg3(self) -> crate::common::Reg<regs::Pdsleepcfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "Run configuration 0"]
    #[inline(always)]
    pub const fn pdruncfg0(self) -> crate::common::Reg<regs::Pdruncfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Run configuration 1"]
    #[inline(always)]
    pub const fn pdruncfg1(self) -> crate::common::Reg<regs::Pdruncfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "Run configuration 2"]
    #[inline(always)]
    pub const fn pdruncfg2(self) -> crate::common::Reg<regs::Pdruncfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Run configuration 3"]
    #[inline(always)]
    pub const fn pdruncfg3(self) -> crate::common::Reg<regs::Pdruncfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Run configuration 0 set"]
    #[inline(always)]
    pub const fn pdruncfg0_set(self) -> crate::common::Reg<regs::Pdruncfg0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Run configuration 1 set"]
    #[inline(always)]
    pub const fn pdruncfg1_set(self) -> crate::common::Reg<regs::Pdruncfg1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "Run configuration 2 set"]
    #[inline(always)]
    pub const fn pdruncfg2_set(self) -> crate::common::Reg<regs::Pdruncfg2Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "Run configuration 3 set"]
    #[inline(always)]
    pub const fn pdruncfg3_set(self) -> crate::common::Reg<regs::Pdruncfg3Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "Run configuration 0 clear"]
    #[inline(always)]
    pub const fn pdruncfg0_clr(self) -> crate::common::Reg<regs::Pdruncfg0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "Run configuration 1 clear"]
    #[inline(always)]
    pub const fn pdruncfg1_clr(self) -> crate::common::Reg<regs::Pdruncfg1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "Run configuration 2 clear"]
    #[inline(always)]
    pub const fn pdruncfg2_clr(self) -> crate::common::Reg<regs::Pdruncfg2Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
    #[doc = "Run configuration 3 clear"]
    #[inline(always)]
    pub const fn pdruncfg3_clr(self) -> crate::common::Reg<regs::Pdruncfg3Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "PD Wake Configuration"]
    #[inline(always)]
    pub const fn pdwakecfg(self) -> crate::common::Reg<regs::Pdwakecfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "Start enable 0"]
    #[inline(always)]
    pub const fn starten0(self) -> crate::common::Reg<regs::Starten0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "Start enable 1"]
    #[inline(always)]
    pub const fn starten1(self) -> crate::common::Reg<regs::Starten1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "Start enable 0 set"]
    #[inline(always)]
    pub const fn starten0_set(self) -> crate::common::Reg<regs::Starten0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "Start enable 1 set"]
    #[inline(always)]
    pub const fn starten1_set(self) -> crate::common::Reg<regs::Starten1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a4usize) as _) }
    }
    #[doc = "Start enable 0 clear"]
    #[inline(always)]
    pub const fn starten0_clr(self) -> crate::common::Reg<regs::Starten0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "Start enable 1 clear"]
    #[inline(always)]
    pub const fn starten1_clr(self) -> crate::common::Reg<regs::Starten1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c4usize) as _) }
    }
    #[doc = "Main Clock Safety"]
    #[inline(always)]
    pub const fn mainclksafety(self) -> crate::common::Reg<regs::Mainclksafety, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "Hardware Wake-up control"]
    #[inline(always)]
    pub const fn hwwake(self) -> crate::common::Reg<regs::Hwwake, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "tempsensor ctrl"]
    #[inline(always)]
    pub const fn tempsensorctl(self) -> crate::common::Reg<regs::Tempsensorctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e0cusize) as _) }
    }
    #[doc = "boot state seed register"]
    #[inline(always)]
    pub const fn bootstateseed(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Bootstateseed, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e50usize + n * 4usize) as _)
        }
    }
    #[doc = "boot state hmac register"]
    #[inline(always)]
    pub const fn bootstatehmac(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Bootstatehmac, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e70usize + n * 4usize) as _)
        }
    }
    #[doc = "FLEXSPI IO pads ctrl register"]
    #[inline(always)]
    pub const fn flexspipadctrl(
        self,
    ) -> crate::common::Reg<regs::Flexspipadctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ef8usize) as _) }
    }
    #[doc = "sdio pad ctrl"]
    #[inline(always)]
    pub const fn sdiopadctl(self) -> crate::common::Reg<regs::Sdiopadctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0efcusize) as _) }
    }
    #[doc = "DICE General Purpose 32-Bit Data Register"]
    #[inline(always)]
    pub const fn dicehwreg(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dicehwreg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize + n * 4usize) as _)
        }
    }
    #[doc = "UUIDn 32-Bit Data Register"]
    #[inline(always)]
    pub const fn uuid(self, n: usize) -> crate::common::Reg<regs::Uuid, crate::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f50usize + n * 4usize) as _)
        }
    }
    #[doc = "AES key source selection"]
    #[inline(always)]
    pub const fn aeskey_srcsel(self) -> crate::common::Reg<regs::AeskeySrcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "Hash hardware key disable"]
    #[inline(always)]
    pub const fn hashhwkeydisable(
        self,
    ) -> crate::common::Reg<regs::Hashhwkeydisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "Debug Write Lock registers"]
    #[inline(always)]
    pub const fn dbg_locken(self) -> crate::common::Reg<regs::DbgLocken, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Debug features control for the CM33"]
    #[inline(always)]
    pub const fn dbg_features(self) -> crate::common::Reg<regs::DbgFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Debug features duplicate"]
    #[inline(always)]
    pub const fn dbg_features_dp(
        self,
    ) -> crate::common::Reg<regs::DbgFeaturesDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "HW unlock disable"]
    #[inline(always)]
    pub const fn hwunlock_disable(
        self,
    ) -> crate::common::Reg<regs::HwunlockDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0facusize) as _) }
    }
    #[doc = "Code Security for CPU0"]
    #[inline(always)]
    pub const fn cs_protcpu0(self) -> crate::common::Reg<regs::CsProtcpu0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "Code Security for CPU1"]
    #[inline(always)]
    pub const fn cs_protcpu1(self) -> crate::common::Reg<regs::CsProtcpu1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb8usize) as _) }
    }
    #[doc = "Debug authorization scratch"]
    #[inline(always)]
    pub const fn dbg_auth_scratch(
        self,
    ) -> crate::common::Reg<regs::DbgAuthScratch, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "Key block"]
    #[inline(always)]
    pub const fn key_block(self) -> crate::common::Reg<regs::KeyBlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
