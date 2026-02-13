#[doc = "SYSCON"]
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
    #[doc = "AHB Matrix Remap Control"]
    #[inline(always)]
    pub const fn remap(self) -> crate::common::Reg<regs::Remap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Matrix Priority Control"]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::common::Reg<regs::Ahbmatprio, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration"]
    #[inline(always)]
    pub const fn cpu0nstckcal(self) -> crate::common::Reg<regs::Cpu0nstckcal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "NMI Source Select"]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::common::Reg<regs::Nmisrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "Protect Level Control"]
    #[inline(always)]
    pub const fn protlvl(self) -> crate::common::Reg<regs::Protlvl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider"]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::common::Reg<regs::Slowclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "BUS_CLK Clock Divider"]
    #[inline(always)]
    pub const fn busclkdiv(self) -> crate::common::Reg<regs::Busclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "System Clock Divider"]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::common::Reg<regs::Ahbclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider"]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::common::Reg<regs::Frohfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "FRO_LF_DIV Clock Divider"]
    #[inline(always)]
    pub const fn frolfdiv(self) -> crate::common::Reg<regs::Frolfdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "PLL1_CLK_DIV Clock Divider"]
    #[inline(always)]
    pub const fn pll1clkdiv(self) -> crate::common::Reg<regs::Pll1clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock"]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::common::Reg<regs::Clkunlock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "NVM Control"]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::common::Reg<regs::NvmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack"]
    #[inline(always)]
    pub const fn smart_dmaint(self) -> crate::common::Reg<regs::SmartDmaint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "Controls RAM Interleave Integration"]
    #[inline(always)]
    pub const fn ram_interleave(
        self,
    ) -> crate::common::Reg<regs::RamInterleave, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "CPU Status"]
    #[inline(always)]
    pub const fn cpustat(self) -> crate::common::Reg<regs::Cpustat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "LPCAC Control"]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::common::Reg<regs::LpcacCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "PWM0 Submodule Control"]
    #[inline(always)]
    pub const fn pwm0subctl(self) -> crate::common::Reg<regs::Pwm0subctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0938usize) as _) }
    }
    #[doc = "PWM1 Submodule Control"]
    #[inline(always)]
    pub const fn pwm1subctl(self) -> crate::common::Reg<regs::Pwm1subctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x093cusize) as _) }
    }
    #[doc = "CTIMER Global Start Enable"]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::common::Reg<regs::Ctimerglobalstarten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "RAM Control"]
    #[inline(always)]
    pub const fn ram_ctrl(self) -> crate::common::Reg<regs::RamCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0944usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[31:0\\]"]
    #[inline(always)]
    pub const fn gray_code_lsb(self) -> crate::common::Reg<regs::GrayCodeLsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray Code \\[41:32\\]"]
    #[inline(always)]
    pub const fn gray_code_msb(self) -> crate::common::Reg<regs::GrayCodeMsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b64usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]"]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeLsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]"]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::common::Reg<regs::BinaryCodeMsb, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "UDF Control"]
    #[inline(always)]
    pub const fn els_udf(self) -> crate::common::Reg<regs::ElsUdf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e10usize) as _) }
    }
    #[doc = "MSF Configuration"]
    #[inline(always)]
    pub const fn msfcfg(self) -> crate::common::Reg<regs::Msfcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e1cusize) as _) }
    }
    #[doc = "Device UID n"]
    #[inline(always)]
    pub const fn els_uid(self, n: usize) -> crate::common::Reg<regs::ElsUid, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e20usize + n * 4usize) as _)
        }
    }
    #[doc = "ROP State Register"]
    #[inline(always)]
    pub const fn rop_state(self) -> crate::common::Reg<regs::RopState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e3cusize) as _) }
    }
    #[doc = "RAM XEN Control"]
    #[inline(always)]
    pub const fn sram_xen(self) -> crate::common::Reg<regs::SramXen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e58usize) as _) }
    }
    #[doc = "RAM XEN Control (Duplicate)"]
    #[inline(always)]
    pub const fn sram_xen_dp(self) -> crate::common::Reg<regs::SramXenDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e5cusize) as _) }
    }
    #[doc = "Life Cycle State Register"]
    #[inline(always)]
    pub const fn els_otp_lc_state(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e80usize) as _) }
    }
    #[doc = "Life Cycle State Register (Duplicate)"]
    #[inline(always)]
    pub const fn els_otp_lc_state_dp(
        self,
    ) -> crate::common::Reg<regs::ElsOtpLcStateDp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e84usize) as _) }
    }
    #[doc = "Control Write Access to Security"]
    #[inline(always)]
    pub const fn debug_lock_en(self) -> crate::common::Reg<regs::DebugLockEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex Debug Features Control"]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::common::Reg<regs::DebugFeatures, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)"]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::common::Reg<regs::DebugFeaturesDp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "CPU0 Software Debug Access"]
    #[inline(always)]
    pub const fn swd_access_cpu0(
        self,
    ) -> crate::common::Reg<regs::SwdAccessCpu0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "Debug Authentication BEACON"]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::common::Reg<regs::DebugAuthBeacon, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "JTAG Chip ID"]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::common::Reg<regs::JtagId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Device Type"]
    #[inline(always)]
    pub const fn device_type(self) -> crate::common::Reg<regs::DeviceType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Device ID"]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::common::Reg<regs::DeviceId0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip Revision ID and Number"]
    #[inline(always)]
    pub const fn dieid(self) -> crate::common::Reg<regs::Dieid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
