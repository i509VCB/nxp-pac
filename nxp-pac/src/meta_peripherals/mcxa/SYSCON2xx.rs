#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "SYSCON."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon {
    ptr: *mut u8,
}
unsafe impl Send for Syscon {}
unsafe impl Sync for Syscon {}
impl Syscon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AHB Matrix Remap Control."]
    #[inline(always)]
    pub const fn remap(self) -> crate::pac::common::Reg<Remap, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Matrix Priority Control."]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::pac::common::Reg<Ahbmatprio, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration."]
    #[inline(always)]
    pub const fn cpu0nstckcal(
        self,
    ) -> crate::pac::common::Reg<Cpu0nstckcal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "NMI Source Select."]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::pac::common::Reg<Nmisrc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Protect Level Control."]
    #[inline(always)]
    pub const fn protlvl(self) -> crate::pac::common::Reg<Protlvl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider."]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::pac::common::Reg<Slowclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "BUS_CLK Clock Divider."]
    #[inline(always)]
    pub const fn busclkdiv(self) -> crate::pac::common::Reg<Busclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "System Clock Divider."]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::pac::common::Reg<Ahbclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider."]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::pac::common::Reg<Frohfdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "FRO_LF_DIV Clock Divider."]
    #[inline(always)]
    pub const fn frolfdiv(self) -> crate::pac::common::Reg<Frolfdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "PLL1_CLK_DIV Clock Divider."]
    #[inline(always)]
    pub const fn pll1clkdiv(self) -> crate::pac::common::Reg<Pll1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock."]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::pac::common::Reg<Clkunlock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "NVM Control."]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::pac::common::Reg<NvmCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack."]
    #[inline(always)]
    pub const fn smart_dmaint(
        self,
    ) -> crate::pac::common::Reg<SmartDmaint, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Controls RAM Interleave Integration."]
    #[inline(always)]
    pub const fn ram_interleave(
        self,
    ) -> crate::pac::common::Reg<RamInterleave, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "CPU Status."]
    #[inline(always)]
    pub const fn cpustat(self) -> crate::pac::common::Reg<Cpustat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "LPCAC Control."]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::pac::common::Reg<LpcacCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "PWM0 Submodule Control."]
    #[inline(always)]
    pub const fn pwm0subctl(self) -> crate::pac::common::Reg<Pwm0subctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0938usize) as _) }
    }
    #[doc = "PWM1 Submodule Control."]
    #[inline(always)]
    pub const fn pwm1subctl(self) -> crate::pac::common::Reg<Pwm1subctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x093cusize) as _) }
    }
    #[doc = "CTIMER Global Start Enable."]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::pac::common::Reg<Ctimerglobalstarten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "RAM Control."]
    #[inline(always)]
    pub const fn ram_ctrl(self) -> crate::pac::common::Reg<RamCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[31:0\\]."]
    #[inline(always)]
    pub const fn gray_code_lsb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeLsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[41:32\\]."]
    #[inline(always)]
    pub const fn gray_code_msb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeMsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b64usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeLsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeMsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "UDF Control."]
    #[inline(always)]
    pub const fn els_udf(self) -> crate::pac::common::Reg<ElsUdf, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "MSF Configuration."]
    #[inline(always)]
    pub const fn msfcfg(self) -> crate::pac::common::Reg<Msfcfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e1cusize) as _) }
    }
    #[doc = "Device UID n."]
    #[inline(always)]
    pub const fn els_uid(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ElsUid, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize + n * 4usize) as _)
        }
    }
    #[doc = "ROP State Register."]
    #[inline(always)]
    pub const fn rop_state(self) -> crate::pac::common::Reg<RopState, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e3cusize) as _) }
    }
    #[doc = "RAM XEN Control."]
    #[inline(always)]
    pub const fn sram_xen(self) -> crate::pac::common::Reg<SramXen, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e58usize) as _) }
    }
    #[doc = "RAM XEN Control (Duplicate)."]
    #[inline(always)]
    pub const fn sram_xen_dp(self) -> crate::pac::common::Reg<SramXenDp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e5cusize) as _) }
    }
    #[doc = "Life Cycle State Register."]
    #[inline(always)]
    pub const fn els_otp_lc_state(
        self,
    ) -> crate::pac::common::Reg<ElsOtpLcState, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize) as _) }
    }
    #[doc = "Life Cycle State Register (Duplicate)."]
    #[inline(always)]
    pub const fn els_otp_lc_state_dp(
        self,
    ) -> crate::pac::common::Reg<ElsOtpLcStateDp, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e84usize) as _) }
    }
    #[doc = "Control Write Access to Security."]
    #[inline(always)]
    pub const fn debug_lock_en(
        self,
    ) -> crate::pac::common::Reg<DebugLockEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex Debug Features Control."]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::pac::common::Reg<DebugFeatures, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)."]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::pac::common::Reg<DebugFeaturesDp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "CPU0 Software Debug Access."]
    #[inline(always)]
    pub const fn swd_access_cpu0(
        self,
    ) -> crate::pac::common::Reg<SwdAccessCpu0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "Debug Authentication BEACON."]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::pac::common::Reg<DebugAuthBeacon, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "JTAG Chip ID."]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::pac::common::Reg<JtagId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Device Type."]
    #[inline(always)]
    pub const fn device_type(self) -> crate::pac::common::Reg<DeviceType, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Device ID."]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::pac::common::Reg<DeviceId0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip Revision ID and Number."]
    #[inline(always)]
    pub const fn dieid(self) -> crate::pac::common::Reg<Dieid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "System Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: AhbclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_cbus(&self) -> Cpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        Cpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu0_cbus(&mut self, val: Cpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> AhbmatprioCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        AhbmatprioCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: AhbmatprioCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "SmartDMA-I bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_cbus_smart_dma_i(&self) -> Cpu1CbusSmartDmaI {
        let val = (self.0 >> 4usize) & 0x03;
        Cpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "SmartDMA-I bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu1_cbus_smart_dma_i(&mut self, val: Cpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "SmartDMA-D bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_sbus_smart_dma_d(&self) -> Cpu1SbusSmartDmaD {
        let val = (self.0 >> 6usize) & 0x03;
        Cpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "SmartDMA-D bus master priority level."]
    #[inline(always)]
    pub const fn set_cpu1_sbus_smart_dma_d(&mut self, val: Cpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_els(&self) -> PkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        PkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[inline(always)]
    pub const fn set_pkc_els(&mut self, val: PkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "USB-FS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_fs_enet(&self) -> UsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        UsbFsEnet::from_bits(val as u8)
    }
    #[doc = "USB-FS bus master priority level."]
    #[inline(always)]
    pub const fn set_usb_fs_enet(&mut self, val: UsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("cpu0_cbus", &self.cpu0_cbus())
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("cpu1_cbus_smart_dma_i", &self.cpu1_cbus_smart_dma_i())
            .field("cpu1_sbus_smart_dma_d", &self.cpu1_sbus_smart_dma_d())
            .field("dma0", &self.dma0())
            .field("pkc_els", &self.pkc_els())
            .field("usb_fs_enet", &self.usb_fs_enet())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ cpu0_cbus: {:?}, cpu0_sbus: {:?}, cpu1_cbus_smart_dma_i: {:?}, cpu1_sbus_smart_dma_d: {:?}, dma0: {:?}, pkc_els: {:?}, usb_fs_enet: {:?} }}",
            self.cpu0_cbus(),
            self.cpu0_sbus(),
            self.cpu1_cbus_smart_dma_i(),
            self.cpu1_sbus_smart_dma_d(),
            self.dma0(),
            self.pkc_els(),
            self.usb_fs_enet()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "BUS_CLK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busclkdiv(pub u32);
impl Busclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> BusclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        BusclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: BusclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> BusclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        BusclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: BusclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> BusclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        BusclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: BusclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Busclkdiv {
    #[inline(always)]
    fn default() -> Busclkdiv {
        Busclkdiv(0)
    }
}
impl core::fmt::Debug for Busclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Busclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Busclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Busclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Clock Configuration Unlock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, SLOWCLKDIV, BUSCLKDIV, AHBCLKDIV, FROHFDIV, FROLFDIV, PLLxCLKDIV, MRCC_xxx_CLKDIV, MRCC_xxx_CLKSEL, MRCC_GLB_xxx)."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Skew {
        let val = (self.0 >> 24usize) & 0x01;
        Skew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Skew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Noref {
        let val = (self.0 >> 25usize) & 0x01;
        Noref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Noref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU0 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state."]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu0lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu0lockup()
        )
    }
}
#[doc = "CTIMER Global Start Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER0 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer0_clk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER0 function clock."]
    #[inline(always)]
    pub const fn set_ctimer0_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the CTIMER1 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer1_clk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER1 function clock."]
    #[inline(always)]
    pub const fn set_ctimer1_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the CTIMER2 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer2_clk_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER2 function clock."]
    #[inline(always)]
    pub const fn set_ctimer2_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the CTIMER3 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer3_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER3 function clock."]
    #[inline(always)]
    pub const fn set_ctimer3_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the CTIMER4 function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer4_clk_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER4 function clock."]
    #[inline(always)]
    pub const fn set_ctimer4_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer0_clk_en", &self.ctimer0_clk_en())
            .field("ctimer1_clk_en", &self.ctimer1_clk_en())
            .field("ctimer2_clk_en", &self.ctimer2_clk_en())
            .field("ctimer3_clk_en", &self.ctimer3_clk_en())
            .field("ctimer4_clk_en", &self.ctimer4_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer0_clk_en: {=bool:?}, ctimer1_clk_en: {=bool:?}, ctimer2_clk_en: {=bool:?}, ctimer3_clk_en: {=bool:?}, ctimer4_clk_en: {=bool:?} }}",
            self.ctimer0_clk_en(),
            self.ctimer1_clk_en(),
            self.ctimer2_clk_en(),
            self.ctimer3_clk_en(),
            self.ctimer4_clk_en()
        )
    }
}
#[doc = "Debug Authentication BEACON."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[inline(always)]
    pub const fn set_beacon(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DebugAuthBeacon {
    #[inline(always)]
    fn default() -> DebugAuthBeacon {
        DebugAuthBeacon(0)
    }
}
impl core::fmt::Debug for DebugAuthBeacon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugAuthBeacon")
            .field("beacon", &self.beacon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugAuthBeacon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugAuthBeacon {{ beacon: {=u32:?} }}", self.beacon())
    }
}
#[doc = "Cortex Debug Features Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for DebugFeatures {
    #[inline(always)]
    fn default() -> DebugFeatures {
        DebugFeatures(0)
    }
}
impl core::fmt::Debug for DebugFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeatures")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden()
        )
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for DebugFeaturesDp {
    #[inline(always)]
    fn default() -> DebugFeaturesDp {
        DebugFeaturesDp(0)
    }
}
impl core::fmt::Debug for DebugFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeaturesDp")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden()
        )
    }
}
#[doc = "Control Write Access to Security."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Controls write access to the security registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        LockAll::from_bits(val as u8)
    }
    #[doc = "Controls write access to the security registers."]
    #[inline(always)]
    pub const fn set_lock_all(&mut self, val: LockAll) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DebugLockEn {
    #[inline(always)]
    fn default() -> DebugLockEn {
        DebugLockEn(0)
    }
}
impl core::fmt::Debug for DebugLockEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLockEn")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLockEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugLockEn {{ lock_all: {:?} }}", self.lock_all())
    }
}
#[doc = "Device ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "Indicates the device's ram size."]
    #[must_use]
    #[inline(always)]
    pub const fn ram_size(&self) -> RamSize {
        let val = (self.0 >> 0usize) & 0x0f;
        RamSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's ram size."]
    #[inline(always)]
    pub const fn set_ram_size(&mut self, val: RamSize) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Indicates the device's flash size."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_size(&self) -> FlashSize {
        let val = (self.0 >> 4usize) & 0x0f;
        FlashSize::from_bits(val as u8)
    }
    #[doc = "Indicates the device's flash size."]
    #[inline(always)]
    pub const fn set_flash_size(&mut self, val: FlashSize) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Indicates the device's ROM revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the device's ROM revision."]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "no description available."]
    #[must_use]
    #[inline(always)]
    pub const fn security(&self) -> Security {
        let val = (self.0 >> 24usize) & 0x0f;
        Security::from_bits(val as u8)
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn set_security(&mut self, val: Security) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("ram_size", &self.ram_size())
            .field("flash_size", &self.flash_size())
            .field("rom_rev_minor", &self.rom_rev_minor())
            .field("security", &self.security())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ ram_size: {:?}, flash_size: {:?}, rom_rev_minor: {=u8:?}, security: {:?} }}",
            self.ram_size(),
            self.flash_size(),
            self.rom_rev_minor(),
            self.security()
        )
    }
}
#[doc = "Device Type."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates the device part number."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_num(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the device part number."]
    #[inline(always)]
    pub const fn set_device_type_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates the device type."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_sec(&self) -> DeviceTypeSec {
        let val = (self.0 >> 16usize) & 0x01;
        DeviceTypeSec::from_bits(val as u8)
    }
    #[doc = "Indicates the device type."]
    #[inline(always)]
    pub const fn set_device_type_sec(&mut self, val: DeviceTypeSec) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the device's package type."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pkg(&self) -> DeviceTypePkg {
        let val = (self.0 >> 20usize) & 0x0f;
        DeviceTypePkg::from_bits(val as u8)
    }
    #[doc = "Indicates the device's package type."]
    #[inline(always)]
    pub const fn set_device_type_pkg(&mut self, val: DeviceTypePkg) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Indicates the device's pin number."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the device's pin number."]
    #[inline(always)]
    pub const fn set_device_type_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type_num", &self.device_type_num())
            .field("device_type_sec", &self.device_type_sec())
            .field("device_type_pkg", &self.device_type_pkg())
            .field("device_type_pin", &self.device_type_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type_num: {=u16:?}, device_type_sec: {:?}, device_type_pkg: {:?}, device_type_pin: {=u8:?} }}",
            self.device_type_num(),
            self.device_type_sec(),
            self.device_type_pkg(),
            self.device_type_pin()
        )
    }
}
#[doc = "Chip Revision ID and Number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieid(pub u32);
impl Dieid {
    #[doc = "Chip minor revision."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_revision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip minor revision."]
    #[inline(always)]
    pub const fn set_minor_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip major revision."]
    #[must_use]
    #[inline(always)]
    pub const fn major_revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip major revision."]
    #[inline(always)]
    pub const fn set_major_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Chip number."]
    #[must_use]
    #[inline(always)]
    pub const fn mco_num_in_die_id(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip number."]
    #[inline(always)]
    pub const fn set_mco_num_in_die_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
    }
}
impl Default for Dieid {
    #[inline(always)]
    fn default() -> Dieid {
        Dieid(0)
    }
}
impl core::fmt::Debug for Dieid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieid")
            .field("minor_revision", &self.minor_revision())
            .field("major_revision", &self.major_revision())
            .field("mco_num_in_die_id", &self.mco_num_in_die_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieid {{ minor_revision: {=u8:?}, major_revision: {=u8:?}, mco_num_in_die_id: {=u32:?} }}",
            self.minor_revision(),
            self.major_revision(),
            self.mco_num_in_die_id()
        )
    }
}
#[doc = "Life Cycle State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcState(pub u32);
impl ElsOtpLcState {
    #[doc = "OTP life cycle state."]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state."]
    #[inline(always)]
    pub const fn set_otp_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcState {
    #[inline(always)]
    fn default() -> ElsOtpLcState {
        ElsOtpLcState(0)
    }
}
impl core::fmt::Debug for ElsOtpLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcState")
            .field("otp_lc_state", &self.otp_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcState {{ otp_lc_state: {=u8:?} }}",
            self.otp_lc_state()
        )
    }
}
#[doc = "Life Cycle State Register (Duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcStateDp(pub u32);
impl ElsOtpLcStateDp {
    #[doc = "OTP life cycle state."]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state."]
    #[inline(always)]
    pub const fn set_otp_lc_state_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcStateDp {
    #[inline(always)]
    fn default() -> ElsOtpLcStateDp {
        ElsOtpLcStateDp(0)
    }
}
impl core::fmt::Debug for ElsOtpLcStateDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcStateDp")
            .field("otp_lc_state_dp", &self.otp_lc_state_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcStateDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcStateDp {{ otp_lc_state_dp: {=u8:?} }}",
            self.otp_lc_state_dp()
        )
    }
}
#[doc = "UDF Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsUdf(pub u32);
impl ElsUdf {
    #[doc = "UDF KEY Select."]
    #[must_use]
    #[inline(always)]
    pub const fn key_sel(&self) -> KeySel {
        let val = (self.0 >> 0usize) & 0x03;
        KeySel::from_bits(val as u8)
    }
    #[doc = "UDF KEY Select."]
    #[inline(always)]
    pub const fn set_key_sel(&mut self, val: KeySel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "UID register hidden control. Write values other than 1010b, locked the write of UID_HIDDEN until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn uid_hidden(&self) -> UidHidden {
        let val = (self.0 >> 24usize) & 0x0f;
        UidHidden::from_bits(val as u8)
    }
    #[doc = "UID register hidden control. Write values other than 1010b, locked the write of UID_HIDDEN until a system reset."]
    #[inline(always)]
    pub const fn set_uid_hidden(&mut self, val: UidHidden) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "UDF register hidden control. Write values other than 1010b, locked the write of UDF_HIDDEN until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn udf_hidden(&self) -> UdfHidden {
        let val = (self.0 >> 28usize) & 0x0f;
        UdfHidden::from_bits(val as u8)
    }
    #[doc = "UDF register hidden control. Write values other than 1010b, locked the write of UDF_HIDDEN until a system reset."]
    #[inline(always)]
    pub const fn set_udf_hidden(&mut self, val: UdfHidden) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for ElsUdf {
    #[inline(always)]
    fn default() -> ElsUdf {
        ElsUdf(0)
    }
}
impl core::fmt::Debug for ElsUdf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsUdf")
            .field("key_sel", &self.key_sel())
            .field("uid_hidden", &self.uid_hidden())
            .field("udf_hidden", &self.udf_hidden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsUdf {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsUdf {{ key_sel: {:?}, uid_hidden: {:?}, udf_hidden: {:?} }}",
            self.key_sel(),
            self.uid_hidden(),
            self.udf_hidden()
        )
    }
}
#[doc = "Device UID n."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsUid(pub u32);
impl ElsUid {
    #[doc = "UID."]
    #[must_use]
    #[inline(always)]
    pub const fn uid0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "UID."]
    #[inline(always)]
    pub const fn set_uid0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsUid {
    #[inline(always)]
    fn default() -> ElsUid {
        ElsUid(0)
    }
}
impl core::fmt::Debug for ElsUid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsUid")
            .field("uid0", &self.uid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsUid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsUid {{ uid0: {=u32:?} }}", self.uid0())
    }
}
#[doc = "FRO_HF_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FrohfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FrohfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FrohfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FrohfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FRO_LF_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frolfdiv(pub u32);
impl Frolfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FrolfdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FrolfdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FrolfdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FrolfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FrolfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FrolfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FrolfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FrolfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FrolfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frolfdiv {
    #[inline(always)]
    fn default() -> Frolfdiv {
        Frolfdiv(0)
    }
}
impl core::fmt::Debug for Frolfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frolfdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frolfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frolfdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray Code \\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "JTAG Chip ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID."]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "LPCAC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Request LPCAC memories."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_mem_req(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Request LPCAC memories."]
    #[inline(always)]
    pub const fn set_lpcac_mem_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("lpcac_xom", &self.lpcac_xom())
            .field("lpcac_mem_req", &self.lpcac_mem_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, lpcac_xom: {=bool:?}, lpcac_mem_req: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.lpcac_xom(),
            self.lpcac_mem_req()
        )
    }
}
#[doc = "MSF Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msfcfg(pub u32);
impl Msfcfg {
    #[doc = "user IFR sector 0 erase control."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis0(&self) -> IfrEraseDis0 {
        let val = (self.0 >> 0usize) & 0x01;
        IfrEraseDis0::from_bits(val as u8)
    }
    #[doc = "user IFR sector 0 erase control."]
    #[inline(always)]
    pub const fn set_ifr_erase_dis0(&mut self, val: IfrEraseDis0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "user IFR sector 1 erase control."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis1(&self) -> IfrEraseDis1 {
        let val = (self.0 >> 1usize) & 0x01;
        IfrEraseDis1::from_bits(val as u8)
    }
    #[doc = "user IFR sector 1 erase control."]
    #[inline(always)]
    pub const fn set_ifr_erase_dis1(&mut self, val: IfrEraseDis1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "user IFR sector 2 erase control."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis2(&self) -> IfrEraseDis2 {
        let val = (self.0 >> 2usize) & 0x01;
        IfrEraseDis2::from_bits(val as u8)
    }
    #[doc = "user IFR sector 2 erase control."]
    #[inline(always)]
    pub const fn set_ifr_erase_dis2(&mut self, val: IfrEraseDis2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "user IFR sector 3 erase control."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_erase_dis3(&self) -> IfrEraseDis3 {
        let val = (self.0 >> 3usize) & 0x01;
        IfrEraseDis3::from_bits(val as u8)
    }
    #[doc = "user IFR sector 3 erase control."]
    #[inline(always)]
    pub const fn set_ifr_erase_dis3(&mut self, val: IfrEraseDis3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Mass erase control."]
    #[must_use]
    #[inline(always)]
    pub const fn mass_erase_dis(&self) -> MassEraseDis {
        let val = (self.0 >> 8usize) & 0x01;
        MassEraseDis::from_bits(val as u8)
    }
    #[doc = "Mass erase control."]
    #[inline(always)]
    pub const fn set_mass_erase_dis(&mut self, val: MassEraseDis) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Msfcfg {
    #[inline(always)]
    fn default() -> Msfcfg {
        Msfcfg(0)
    }
}
impl core::fmt::Debug for Msfcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msfcfg")
            .field("ifr_erase_dis0", &self.ifr_erase_dis0())
            .field("ifr_erase_dis1", &self.ifr_erase_dis1())
            .field("ifr_erase_dis2", &self.ifr_erase_dis2())
            .field("ifr_erase_dis3", &self.ifr_erase_dis3())
            .field("mass_erase_dis", &self.mass_erase_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msfcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Msfcfg {{ ifr_erase_dis0: {:?}, ifr_erase_dis1: {:?}, ifr_erase_dis2: {:?}, ifr_erase_dis3: {:?}, mass_erase_dis: {:?} }}",
            self.ifr_erase_dis0(),
            self.ifr_erase_dis1(),
            self.ifr_erase_dis2(),
            self.ifr_erase_dis3(),
            self.mass_erase_dis()
        )
    }
}
#[doc = "NMI Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.nmiencpu0()
        )
    }
}
#[doc = "NVM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control."]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control."]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLASH stall on busy control."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control."]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control Set this field to 0 if you want to enable flash speculative."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "PLL1_CLK_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clkdiv(pub u32);
impl Pll1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Pll1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Pll1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Pll1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Pll1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Pll1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Pll1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Pll1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Pll1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Pll1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clkdiv {
    #[inline(always)]
    fn default() -> Pll1clkdiv {
        Pll1clkdiv(0)
    }
}
impl core::fmt::Debug for Pll1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Protect Level Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Protlvl(pub u32);
impl Protlvl {
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG."]
    #[must_use]
    #[inline(always)]
    pub const fn priv_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control privileged access of EIM, ERM, Flexcan, MBC, SCG."]
    #[inline(always)]
    pub const fn set_priv_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[must_use]
    #[inline(always)]
    pub const fn locknsmpu(&self) -> Locknsmpu {
        let val = (self.0 >> 16usize) & 0x01;
        Locknsmpu::from_bits(val as u8)
    }
    #[doc = "Control write access to Nonsecure MPU memory regions."]
    #[inline(always)]
    pub const fn set_locknsmpu(&mut self, val: Locknsmpu) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> ProtlvlLock {
        let val = (self.0 >> 31usize) & 0x01;
        ProtlvlLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: ProtlvlLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Protlvl {
    #[inline(always)]
    fn default() -> Protlvl {
        Protlvl(0)
    }
}
impl core::fmt::Debug for Protlvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Protlvl")
            .field("priv_", &self.priv_())
            .field("locknsmpu", &self.locknsmpu())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Protlvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Protlvl {{ priv_: {=bool:?}, locknsmpu: {:?}, lock: {:?} }}",
            self.priv_(),
            self.locknsmpu(),
            self.lock()
        )
    }
}
#[doc = "PWM0 Submodule Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0subctl(pub u32);
impl Pwm0subctl {
    #[doc = "Enables PWM0 SUB Clock0."]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock0."]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM0 SUB Clock1."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock1."]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM0 SUB Clock2."]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock2."]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM0 SUB Clock3."]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock3."]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwm0subctl {
    #[inline(always)]
    fn default() -> Pwm0subctl {
        Pwm0subctl(0)
    }
}
impl core::fmt::Debug for Pwm0subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm0subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en()
        )
    }
}
#[doc = "PWM1 Submodule Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1subctl(pub u32);
impl Pwm1subctl {
    #[doc = "Enables PWM1 SUB Clock0."]
    #[must_use]
    #[inline(always)]
    pub const fn clk0_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock0."]
    #[inline(always)]
    pub const fn set_clk0_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables PWM1 SUB Clock1."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock1."]
    #[inline(always)]
    pub const fn set_clk1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables PWM1 SUB Clock2."]
    #[must_use]
    #[inline(always)]
    pub const fn clk2_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock2."]
    #[inline(always)]
    pub const fn set_clk2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables PWM1 SUB Clock3."]
    #[must_use]
    #[inline(always)]
    pub const fn clk3_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock3."]
    #[inline(always)]
    pub const fn set_clk3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pwm1subctl {
    #[inline(always)]
    fn default() -> Pwm1subctl {
        Pwm1subctl(0)
    }
}
impl core::fmt::Debug for Pwm1subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1subctl")
            .field("clk0_en", &self.clk0_en())
            .field("clk1_en", &self.clk1_en())
            .field("clk2_en", &self.clk2_en())
            .field("clk3_en", &self.clk3_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm1subctl {{ clk0_en: {=bool:?}, clk1_en: {=bool:?}, clk2_en: {=bool:?}, clk3_en: {=bool:?} }}",
            self.clk0_en(),
            self.clk1_en(),
            self.clk2_en(),
            self.clk3_en()
        )
    }
}
#[doc = "RAM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamCtrl(pub u32);
impl RamCtrl {
    #[doc = "RAMA ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable."]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_cg_override(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA bank clock gating control, only avaiable when RAMA_ECC_ENABLE = 0."]
    #[inline(always)]
    pub const fn set_rama_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "RAMX bank clock gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx_cg_override(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX bank clock gating control."]
    #[inline(always)]
    pub const fn set_ramx_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "RAMB bank clock gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_cg_override(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB bank clock gating control."]
    #[inline(always)]
    pub const fn set_ramb_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "RAMC bank clock gating control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_cg_override(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "RAMC bank clock gating control."]
    #[inline(always)]
    pub const fn set_ramc_cg_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for RamCtrl {
    #[inline(always)]
    fn default() -> RamCtrl {
        RamCtrl(0)
    }
}
impl core::fmt::Debug for RamCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("rama_cg_override", &self.rama_cg_override())
            .field("ramx_cg_override", &self.ramx_cg_override())
            .field("ramb_cg_override", &self.ramb_cg_override())
            .field("ramc_cg_override", &self.ramc_cg_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RamCtrl {{ rama_ecc_enable: {=bool:?}, rama_cg_override: {=bool:?}, ramx_cg_override: {=bool:?}, ramb_cg_override: {=bool:?}, ramc_cg_override: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.rama_cg_override(),
            self.ramx_cg_override(),
            self.ramb_cg_override(),
            self.ramc_cg_override()
        )
    }
}
#[doc = "Controls RAM Interleave Integration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamInterleave(pub u32);
impl RamInterleave {
    #[doc = "Controls RAM access for RAMA1 and RAMA2."]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        Interleave::from_bits(val as u8)
    }
    #[doc = "Controls RAM access for RAMA1 and RAMA2."]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RamInterleave {
    #[inline(always)]
    fn default() -> RamInterleave {
        RamInterleave(0)
    }
}
impl core::fmt::Debug for RamInterleave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamInterleave")
            .field("interleave", &self.interleave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamInterleave {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RamInterleave {{ interleave: {:?} }}", self.interleave())
    }
}
#[doc = "AHB Matrix Remap Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Remap(pub u32);
impl Remap {
    #[doc = "RAMX0 address remap for CPU System bus."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_sbus(&self) -> RemapCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        RemapCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for CPU System bus."]
    #[inline(always)]
    pub const fn set_cpu0_sbus(&mut self, val: RemapCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_d(&self) -> SmartDmaD {
        let val = (self.0 >> 4usize) & 0x03;
        SmartDmaD::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA D-BUS."]
    #[inline(always)]
    pub const fn set_smart_dma_d(&mut self, val: SmartDmaD) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_i(&self) -> SmartDmaI {
        let val = (self.0 >> 6usize) & 0x03;
        SmartDmaI::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for SmartDMA I-BUS."]
    #[inline(always)]
    pub const fn set_smart_dma_i(&mut self, val: SmartDmaI) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "RAMX0 address remap for DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> RemapDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        RemapDma0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: RemapDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "RAMX0 address remap for PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> Pkc {
        let val = (self.0 >> 12usize) & 0x03;
        Pkc::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: Pkc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "RAMX0 address remap for USB0."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0(&self) -> Usb0 {
        let val = (self.0 >> 24usize) & 0x03;
        Usb0::from_bits(val as u8)
    }
    #[doc = "RAMX0 address remap for USB0."]
    #[inline(always)]
    pub const fn set_usb0(&mut self, val: Usb0) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> RemapLock {
        let val = (self.0 >> 31usize) & 0x01;
        RemapLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: RemapLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Remap {
    #[inline(always)]
    fn default() -> Remap {
        Remap(0)
    }
}
impl core::fmt::Debug for Remap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Remap")
            .field("cpu0_sbus", &self.cpu0_sbus())
            .field("smart_dma_d", &self.smart_dma_d())
            .field("smart_dma_i", &self.smart_dma_i())
            .field("dma0", &self.dma0())
            .field("pkc", &self.pkc())
            .field("usb0", &self.usb0())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Remap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Remap {{ cpu0_sbus: {:?}, smart_dma_d: {:?}, smart_dma_i: {:?}, dma0: {:?}, pkc: {:?}, usb0: {:?}, lock: {:?} }}",
            self.cpu0_sbus(),
            self.smart_dma_d(),
            self.smart_dma_i(),
            self.dma0(),
            self.pkc(),
            self.usb0(),
            self.lock()
        )
    }
}
#[doc = "ROP State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RopState(pub u32);
impl RopState {
    #[doc = "ROP state."]
    #[must_use]
    #[inline(always)]
    pub const fn rop_state(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ROP state."]
    #[inline(always)]
    pub const fn set_rop_state(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RopState {
    #[inline(always)]
    fn default() -> RopState {
        RopState(0)
    }
}
impl core::fmt::Debug for RopState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RopState")
            .field("rop_state", &self.rop_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RopState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RopState {{ rop_state: {=u32:?} }}", self.rop_state())
    }
}
#[doc = "SLOW_CLK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ1."]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ1."]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ17."]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ17."]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ18."]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ18."]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ30."]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ30."]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ35."]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ35."]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ36."]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ36."]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ37."]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ37."]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ38."]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ38."]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ42."]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ42."]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ45."]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ45."]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ47."]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ47."]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ50."]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ50."]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ51."]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ51."]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ66."]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ66."]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ67."]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ67."]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ77."]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ77."]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
#[doc = "RAM XEN Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramXen(pub u32);
impl SramXen {
    #[doc = "RAMX0 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX0 Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMX1 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMX1 Execute permission control."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMA0 Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA0 Execute permission control."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMAx (excepts RAMA0) Execute permission control."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RAMBx Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RAMBx Execute permission control."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RAMCx Execute permission control."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_xen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RAMCx Execute permission control."]
    #[inline(always)]
    pub const fn set_ramc_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register (and SRAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> SramXenLock {
        let val = (self.0 >> 31usize) & 0x01;
        SramXenLock::from_bits(val as u8)
    }
    #[doc = "This 1-bit field provides a mechanism to limit writes to the this register (and SRAM_XEN_DP) to protect its contents. Once set, this bit remains asserted until a system reset."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: SramXenLock) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SramXen {
    #[inline(always)]
    fn default() -> SramXen {
        SramXen(0)
    }
}
impl core::fmt::Debug for SramXen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramXen")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .field("ramc_xen", &self.ramc_xen())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramXen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramXen {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?}, ramc_xen: {=bool:?}, lock: {:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen(),
            self.ramc_xen(),
            self.lock()
        )
    }
}
#[doc = "RAM XEN Control (Duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SramXenDp(pub u32);
impl SramXenDp {
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx0_xen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx1_xen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramx1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama0_xen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama0_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn rama1_xen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_rama1_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_xen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramb_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_xen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Refer to SRAM_XEN for more details."]
    #[inline(always)]
    pub const fn set_ramc_xen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for SramXenDp {
    #[inline(always)]
    fn default() -> SramXenDp {
        SramXenDp(0)
    }
}
impl core::fmt::Debug for SramXenDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SramXenDp")
            .field("ramx0_xen", &self.ramx0_xen())
            .field("ramx1_xen", &self.ramx1_xen())
            .field("rama0_xen", &self.rama0_xen())
            .field("rama1_xen", &self.rama1_xen())
            .field("ramb_xen", &self.ramb_xen())
            .field("ramc_xen", &self.ramc_xen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SramXenDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SramXenDp {{ ramx0_xen: {=bool:?}, ramx1_xen: {=bool:?}, rama0_xen: {=bool:?}, rama1_xen: {=bool:?}, ramb_xen: {=bool:?}, ramc_xen: {=bool:?} }}",
            self.ramx0_xen(),
            self.ramx1_xen(),
            self.rama0_xen(),
            self.rama1_xen(),
            self.ramb_xen(),
            self.ramc_xen()
        )
    }
}
#[doc = "CPU0 Software Debug Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu0(pub u32);
impl SwdAccessCpu0 {
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        SecCode::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu0 {
    #[inline(always)]
    fn default() -> SwdAccessCpu0 {
        SwdAccessCpu0(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu0")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu0 {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioCpu0Sbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioCpu0Sbus {
        AhbmatprioCpu0Sbus::from_bits(val)
    }
}
impl From<AhbmatprioCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioCpu0Sbus) -> u8 {
        AhbmatprioCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl BusclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivHalt {
        BusclkdivHalt::from_bits(val)
    }
}
impl From<BusclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivHalt) -> u8 {
        BusclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl BusclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivReset {
        BusclkdivReset::from_bits(val)
    }
}
impl From<BusclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivReset) -> u8 {
        BusclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BusclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl BusclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BusclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BusclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> BusclkdivUnstab {
        BusclkdivUnstab::from_bits(val)
    }
}
impl From<BusclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: BusclkdivUnstab) -> u8 {
        BusclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache."]
    Enable = 0x0,
    #[doc = "Clears the cache."]
    Disable = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0Cbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl Cpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> Cpu0Cbus {
        Cpu0Cbus::from_bits(val)
    }
}
impl From<Cpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: Cpu0Cbus) -> u8 {
        Cpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup."]
    Awake = 0x0,
    #[doc = "CPU is in lockup."]
    Sleeping = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping."]
    Awake = 0x0,
    #[doc = "CPU is sleeping."]
    Sleeping = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1CbusSmartDmaI {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl Cpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> Cpu1CbusSmartDmaI {
        Cpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<Cpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: Cpu1CbusSmartDmaI) -> u8 {
        Cpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1SbusSmartDmaD {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl Cpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> Cpu1SbusSmartDmaD {
        Cpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<Cpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: Cpu1SbusSmartDmaD) -> u8 {
        Cpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypePkg {
    #[doc = "HLQFP."]
    Hlqfp = 0x0,
    #[doc = "HTQFP."]
    Htqfp = 0x01,
    #[doc = "BGA."]
    Bga = 0x02,
    #[doc = "HDQFP."]
    Hdqfp = 0x03,
    #[doc = "QFN."]
    Qfn = 0x04,
    #[doc = "CSP."]
    Csp = 0x05,
    #[doc = "LQFP."]
    Lqfp = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DeviceTypePkg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypePkg {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypePkg {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypePkg {
        DeviceTypePkg::from_bits(val)
    }
}
impl From<DeviceTypePkg> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypePkg) -> u8 {
        DeviceTypePkg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DeviceTypeSec {
    #[doc = "Non Secure."]
    NonSec = 0x0,
    #[doc = "Secure."]
    Sec = 0x01,
}
impl DeviceTypeSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DeviceTypeSec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DeviceTypeSec {
    #[inline(always)]
    fn from(val: u8) -> DeviceTypeSec {
        DeviceTypeSec::from_bits(val)
    }
}
impl From<DeviceTypeSec> for u8 {
    #[inline(always)]
    fn from(val: DeviceTypeSec) -> u8 {
        DeviceTypeSec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation."]
    Enable = 0x0,
    #[doc = "Disables data speculation."]
    Disable = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation."]
    Enable = 0x0,
    #[doc = "Disables flash speculation."]
    Disable = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data."]
    Disable = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction."]
    Disable = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashSize {
    #[doc = "32KB."]
    Size32kb = 0x0,
    #[doc = "64KB."]
    Size64kb = 0x01,
    #[doc = "128KB."]
    Size128kb = 0x02,
    #[doc = "256KB."]
    Size256kb = 0x03,
    #[doc = "512KB."]
    Size512kb = 0x04,
    #[doc = "768KB."]
    Size768kb = 0x05,
    #[doc = "1MB."]
    Size1mb = 0x06,
    #[doc = "1.5MB."]
    Size1p5mb = 0x07,
    #[doc = "2MB."]
    Size2mb = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl FlashSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashSize {
    #[inline(always)]
    fn from(val: u8) -> FlashSize {
        FlashSize::from_bits(val)
    }
}
impl From<FlashSize> for u8 {
    #[inline(always)]
    fn from(val: FlashSize) -> u8 {
        FlashSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy."]
    Enable = 0x0,
    #[doc = "Stall on FLASH busy."]
    Disable = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation."]
    Enable = 0x0,
    #[doc = "Forces no allocation."]
    Disable = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FrohfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivReset {
        FrohfdivReset::from_bits(val)
    }
}
impl From<FrohfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivReset) -> u8 {
        FrohfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FrolfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivHalt {
        FrolfdivHalt::from_bits(val)
    }
}
impl From<FrolfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivHalt) -> u8 {
        FrolfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FrolfdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivReset {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivReset {
        FrolfdivReset::from_bits(val)
    }
}
impl From<FrolfdivReset> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivReset) -> u8 {
        FrolfdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrolfdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FrolfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrolfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrolfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrolfdivUnstab {
        FrolfdivUnstab::from_bits(val)
    }
}
impl From<FrolfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrolfdivUnstab) -> u8 {
        FrolfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrEraseDis0 {
    #[doc = "Enable IFR sector erase."]
    Enable = 0x0,
    #[doc = "Disable IFR sector erase, write one lock until a system reset."]
    Disable = 0x01,
}
impl IfrEraseDis0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrEraseDis0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrEraseDis0 {
    #[inline(always)]
    fn from(val: u8) -> IfrEraseDis0 {
        IfrEraseDis0::from_bits(val)
    }
}
impl From<IfrEraseDis0> for u8 {
    #[inline(always)]
    fn from(val: IfrEraseDis0) -> u8 {
        IfrEraseDis0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrEraseDis1 {
    #[doc = "Enable IFR sector erase."]
    Enable = 0x0,
    #[doc = "Disable IFR sector erase, write one lock until a system reset."]
    Disable = 0x01,
}
impl IfrEraseDis1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrEraseDis1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrEraseDis1 {
    #[inline(always)]
    fn from(val: u8) -> IfrEraseDis1 {
        IfrEraseDis1::from_bits(val)
    }
}
impl From<IfrEraseDis1> for u8 {
    #[inline(always)]
    fn from(val: IfrEraseDis1) -> u8 {
        IfrEraseDis1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrEraseDis2 {
    #[doc = "Enable IFR sector erase."]
    Enable = 0x0,
    #[doc = "Disable IFR sector erase, write one lock until a system reset."]
    Disable = 0x01,
}
impl IfrEraseDis2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrEraseDis2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrEraseDis2 {
    #[inline(always)]
    fn from(val: u8) -> IfrEraseDis2 {
        IfrEraseDis2::from_bits(val)
    }
}
impl From<IfrEraseDis2> for u8 {
    #[inline(always)]
    fn from(val: IfrEraseDis2) -> u8 {
        IfrEraseDis2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrEraseDis3 {
    #[doc = "Enable IFR sector erase."]
    Enable = 0x0,
    #[doc = "Disable IFR sector erase, write one lock until a system reset."]
    Disable = 0x01,
}
impl IfrEraseDis3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrEraseDis3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrEraseDis3 {
    #[inline(always)]
    fn from(val: u8) -> IfrEraseDis3 {
        IfrEraseDis3::from_bits(val)
    }
}
impl From<IfrEraseDis3> for u8 {
    #[inline(always)]
    fn from(val: IfrEraseDis3) -> u8 {
        IfrEraseDis3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access is consecutive."]
    Normal = 0x0,
    #[doc = "RAM access is interleaved. This setting is need for PKC L0 memory access."]
    Interleave = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum KeySel {
    #[doc = "DUK: UID\\[127:0\\]^RTL_CONST1\\[127:0\\]."]
    Duk0 = 0x0,
    #[doc = "DUK: UID\\[127:0\\]^RTL_CONST1\\[127:0\\]."]
    Duk1 = 0x01,
    #[doc = "DeviceHSM."]
    DeviceHsm = 0x02,
    #[doc = "NXP_mRoT."]
    NxpMRoT = 0x03,
}
impl KeySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> KeySel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for KeySel {
    #[inline(always)]
    fn from(val: u8) -> KeySel {
        KeySel::from_bits(val)
    }
}
impl From<KeySel> for u8 {
    #[inline(always)]
    fn from(val: KeySel) -> u8 {
        KeySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers."]
    Disable = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers."]
    Enable = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Locknsmpu {
    #[doc = "Unlock these registers. privileged access to Nonsecure MPU memory regions is allowed."]
    Enable = 0x0,
    #[doc = "Disable writes to the MPU_CTRL_NS, MPU_RNR_NS, MPU_RBAR_NS, MPU_RLAR_NS, MPU_RBAR_A_NSn and MPU_RLAR_A_NSn. All writes to the registers are ignored."]
    Disable = 0x01,
}
impl Locknsmpu {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Locknsmpu {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Locknsmpu {
    #[inline(always)]
    fn from(val: u8) -> Locknsmpu {
        Locknsmpu::from_bits(val)
    }
}
impl From<Locknsmpu> for u8 {
    #[inline(always)]
    fn from(val: Locknsmpu) -> u8 {
        Locknsmpu::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MassEraseDis {
    #[doc = "Enables mass erase."]
    Enable = 0x0,
    #[doc = "Disables mass erase, write one lock until a system reset."]
    Disable = 0x01,
}
impl MassEraseDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MassEraseDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MassEraseDis {
    #[inline(always)]
    fn from(val: u8) -> MassEraseDis {
        MassEraseDis::from_bits(val)
    }
}
impl From<MassEraseDis> for u8 {
    #[inline(always)]
    fn from(val: MassEraseDis) -> u8 {
        MassEraseDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Noref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Noref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Noref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Noref {
    #[inline(always)]
    fn from(val: u8) -> Noref {
        Noref::from_bits(val)
    }
}
impl From<Noref> for u8 {
    #[inline(always)]
    fn from(val: Noref) -> u8 {
        Noref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pkc {
    #[doc = "RAMX0: alias space is disabled."]
    Pkc0 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    Pkc1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pkc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pkc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pkc {
    #[inline(always)]
    fn from(val: u8) -> Pkc {
        Pkc::from_bits(val)
    }
}
impl From<Pkc> for u8 {
    #[inline(always)]
    fn from(val: Pkc) -> u8 {
        Pkc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcEls {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcEls {
    #[inline(always)]
    fn from(val: u8) -> PkcEls {
        PkcEls::from_bits(val)
    }
}
impl From<PkcEls> for u8 {
    #[inline(always)]
    fn from(val: PkcEls) -> u8 {
        PkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Pll1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivHalt {
        Pll1clkdivHalt::from_bits(val)
    }
}
impl From<Pll1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivHalt) -> u8 {
        Pll1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Pll1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivReset {
        Pll1clkdivReset::from_bits(val)
    }
}
impl From<Pll1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivReset) -> u8 {
        Pll1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Pll1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clkdivUnstab {
        Pll1clkdivUnstab::from_bits(val)
    }
}
impl From<Pll1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clkdivUnstab) -> u8 {
        Pll1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProtlvlLock {
    #[doc = "This register is not locked and can be altered."]
    Lock0 = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    Lock1 = 0x01,
}
impl ProtlvlLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProtlvlLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProtlvlLock {
    #[inline(always)]
    fn from(val: u8) -> ProtlvlLock {
        ProtlvlLock::from_bits(val)
    }
}
impl From<ProtlvlLock> for u8 {
    #[inline(always)]
    fn from(val: ProtlvlLock) -> u8 {
        ProtlvlLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RamSize {
    #[doc = "8KB."]
    Size8kb = 0x0,
    #[doc = "16KB."]
    Size16kb = 0x01,
    #[doc = "32KB."]
    Size32kb = 0x02,
    #[doc = "64KB."]
    Size64kb = 0x03,
    #[doc = "96KB."]
    Size96kb = 0x04,
    #[doc = "128KB."]
    Size128kb = 0x05,
    #[doc = "160KB."]
    Size160kb = 0x06,
    #[doc = "192KB."]
    Size192kb = 0x07,
    #[doc = "256KB."]
    Size256kb = 0x08,
    #[doc = "288KB."]
    Size288kb = 0x09,
    #[doc = "352KB."]
    Size352kb = 0x0a,
    #[doc = "512KB."]
    Size512kb = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl RamSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RamSize {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RamSize {
    #[inline(always)]
    fn from(val: u8) -> RamSize {
        RamSize::from_bits(val)
    }
}
impl From<RamSize> for u8 {
    #[inline(always)]
    fn from(val: RamSize) -> u8 {
        RamSize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapCpu0Sbus {
    #[doc = "RAMX0: alias space is disabled."]
    Cpu0Sbus0 = 0x0,
    #[doc = "RAMX0: alias space is enabled. It's linear address space from bottom of system ram. The start address is 0x20000000 + (system ram size - RAMX size)*1024."]
    Cpu0Sbus1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> RemapCpu0Sbus {
        RemapCpu0Sbus::from_bits(val)
    }
}
impl From<RemapCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: RemapCpu0Sbus) -> u8 {
        RemapCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapDma0 {
    #[doc = "RAMX0: alias space is disabled."]
    Dma00 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    Dma01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl RemapDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapDma0 {
    #[inline(always)]
    fn from(val: u8) -> RemapDma0 {
        RemapDma0::from_bits(val)
    }
}
impl From<RemapDma0> for u8 {
    #[inline(always)]
    fn from(val: RemapDma0) -> u8 {
        RemapDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RemapLock {
    #[doc = "This register is not locked and can be altered."]
    Lock0 = 0x0,
    #[doc = "This register is locked and cannot be altered until a system reset."]
    Lock1 = 0x01,
}
impl RemapLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RemapLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RemapLock {
    #[inline(always)]
    fn from(val: u8) -> RemapLock {
        RemapLock::from_bits(val)
    }
}
impl From<RemapLock> for u8 {
    #[inline(always)]
    fn from(val: RemapLock) -> u8 {
        RemapLock::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SecCode(u32);
impl SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SecCode {
    pub const fn from_bits(val: u32) -> SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SecCode {
    #[inline(always)]
    fn from(val: u32) -> SecCode {
        SecCode::from_bits(val)
    }
}
impl From<SecCode> for u32 {
    #[inline(always)]
    fn from(val: SecCode) -> u32 {
        SecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Security {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Secure version."]
    NonSec = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Non secure version."]
    Security10 = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Security {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Security {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Security {
    #[inline(always)]
    fn from(val: u8) -> Security {
        Security::from_bits(val)
    }
}
impl From<Security> for u8 {
    #[inline(always)]
    fn from(val: Security) -> u8 {
        Security::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Skew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Skew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Skew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Skew {
    #[inline(always)]
    fn from(val: u8) -> Skew {
        Skew::from_bits(val)
    }
}
impl From<Skew> for u8 {
    #[inline(always)]
    fn from(val: Skew) -> u8 {
        Skew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaD {
    #[doc = "RAMX0: alias space is disabled."]
    SmartDmaD0 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    SmartDmaD1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaD {
        SmartDmaD::from_bits(val)
    }
}
impl From<SmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaD) -> u8 {
        SmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaI {
    #[doc = "RAMX0: alias space is disabled."]
    SmartDmaI0 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    SmartDmaI1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaI {
        SmartDmaI::from_bits(val)
    }
}
impl From<SmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaI) -> u8 {
        SmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SramXenLock {
    #[doc = "This register is not locked and can be altered."]
    Lock0 = 0x0,
    #[doc = "This register is locked and cannot be altered."]
    Lock1 = 0x01,
}
impl SramXenLock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SramXenLock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SramXenLock {
    #[inline(always)]
    fn from(val: u8) -> SramXenLock {
        SramXenLock::from_bits(val)
    }
}
impl From<SramXenLock> for u8 {
    #[inline(always)]
    fn from(val: SramXenLock) -> u8 {
        SramXenLock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UdfHidden {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enable the access of UDF register from APB bus. All other value, disable the read/write of UDF register from UDF APB bus."]
    UdfHidden = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl UdfHidden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UdfHidden {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UdfHidden {
    #[inline(always)]
    fn from(val: u8) -> UdfHidden {
        UdfHidden::from_bits(val)
    }
}
impl From<UdfHidden> for u8 {
    #[inline(always)]
    fn from(val: UdfHidden) -> u8 {
        UdfHidden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UidHidden {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enable the access of UID\\[127:0\\] register. All other value, disable the read/write of UID\\[127:0\\] register."]
    UidHidden = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl UidHidden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UidHidden {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UidHidden {
    #[inline(always)]
    fn from(val: u8) -> UidHidden {
        UidHidden::from_bits(val)
    }
}
impl From<UidHidden> for u8 {
    #[inline(always)]
    fn from(val: UidHidden) -> u8 {
        UidHidden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers."]
    Enable = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    Freeze = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0 {
    #[doc = "RAMX0: alias space is disabled."]
    Usb00 = 0x0,
    #[doc = "RAMX0: same alias space as CPU0_SBUS."]
    Usb01 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Usb0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0 {
    #[inline(always)]
    fn from(val: u8) -> Usb0 {
        Usb0::from_bits(val)
    }
}
impl From<Usb0> for u8 {
    #[inline(always)]
    fn from(val: Usb0) -> u8 {
        Usb0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbFsEnet {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl UsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> UsbFsEnet {
        UsbFsEnet::from_bits(val)
    }
}
impl From<UsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: UsbFsEnet) -> u8 {
        UsbFsEnet::to_bits(val)
    }
}
