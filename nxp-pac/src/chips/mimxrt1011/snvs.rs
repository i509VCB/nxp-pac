#[doc = "SNVS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Snvs {
    ptr: *mut u8,
}
unsafe impl Send for Snvs {}
unsafe impl Sync for Snvs {}
impl Snvs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SNVS_HP Lock Register"]
    #[inline(always)]
    pub const fn hplr(self) -> crate::common::Reg<regs::Hplr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SNVS_HP Command Register"]
    #[inline(always)]
    pub const fn hpcomr(self) -> crate::common::Reg<regs::Hpcomr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SNVS_HP Control Register"]
    #[inline(always)]
    pub const fn hpcr(self) -> crate::common::Reg<regs::Hpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SNVS_HP Security Interrupt Control Register"]
    #[inline(always)]
    pub const fn hpsicr(self) -> crate::common::Reg<regs::Hpsicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SNVS_HP Security Violation Control Register"]
    #[inline(always)]
    pub const fn hpsvcr(self) -> crate::common::Reg<regs::Hpsvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SNVS_HP Status Register"]
    #[inline(always)]
    pub const fn hpsr(self) -> crate::common::Reg<regs::Hpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SNVS_HP Security Violation Status Register"]
    #[inline(always)]
    pub const fn hpsvsr(self) -> crate::common::Reg<regs::Hpsvsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SNVS_HP High Assurance Counter IV Register"]
    #[inline(always)]
    pub const fn hphacivr(self) -> crate::common::Reg<regs::Hphacivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SNVS_HP High Assurance Counter Register"]
    #[inline(always)]
    pub const fn hphacr(self) -> crate::common::Reg<regs::Hphacr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SNVS_HP Real Time Counter MSB Register"]
    #[inline(always)]
    pub const fn hprtcmr(self) -> crate::common::Reg<regs::Hprtcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SNVS_HP Real Time Counter LSB Register"]
    #[inline(always)]
    pub const fn hprtclr(self) -> crate::common::Reg<regs::Hprtclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SNVS_HP Time Alarm MSB Register"]
    #[inline(always)]
    pub const fn hptamr(self) -> crate::common::Reg<regs::Hptamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SNVS_HP Time Alarm LSB Register"]
    #[inline(always)]
    pub const fn hptalr(self) -> crate::common::Reg<regs::Hptalr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SNVS_LP Lock Register"]
    #[inline(always)]
    pub const fn lplr(self) -> crate::common::Reg<regs::Lplr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SNVS_LP Control Register"]
    #[inline(always)]
    pub const fn lpcr(self) -> crate::common::Reg<regs::Lpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SNVS_LP Master Key Control Register"]
    #[inline(always)]
    pub const fn lpmkcr(self) -> crate::common::Reg<regs::Lpmkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SNVS_LP Security Violation Control Register"]
    #[inline(always)]
    pub const fn lpsvcr(self) -> crate::common::Reg<regs::Lpsvcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SNVS_LP Tamper Glitch Filters Configuration Register"]
    #[inline(always)]
    pub const fn lptgfcr(self) -> crate::common::Reg<regs::Lptgfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SNVS_LP Tamper Detect Configuration Register"]
    #[inline(always)]
    pub const fn lptdcr(self) -> crate::common::Reg<regs::Lptdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SNVS_LP Status Register"]
    #[inline(always)]
    pub const fn lpsr(self) -> crate::common::Reg<regs::Lpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "SNVS_LP Secure Real Time Counter MSB Register"]
    #[inline(always)]
    pub const fn lpsrtcmr(self) -> crate::common::Reg<regs::Lpsrtcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SNVS_LP Secure Real Time Counter LSB Register"]
    #[inline(always)]
    pub const fn lpsrtclr(self) -> crate::common::Reg<regs::Lpsrtclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SNVS_LP Time Alarm Register"]
    #[inline(always)]
    pub const fn lptar(self) -> crate::common::Reg<regs::Lptar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter MSB Register"]
    #[inline(always)]
    pub const fn lpsmcmr(self) -> crate::common::Reg<regs::Lpsmcmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "SNVS_LP Secure Monotonic Counter LSB Register"]
    #[inline(always)]
    pub const fn lpsmclr(self) -> crate::common::Reg<regs::Lpsmclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "SNVS_LP Digital Low-Voltage Detector Register"]
    #[inline(always)]
    pub const fn lplvdr(self) -> crate::common::Reg<regs::Lplvdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Register 0 (legacy alias)"]
    #[inline(always)]
    pub const fn lpgpr0_legacy_alias(
        self,
    ) -> crate::common::Reg<regs::Lpgpr0LegacyAlias, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "SNVS_LP Zeroizable Master Key Register"]
    #[inline(always)]
    pub const fn lpzmkr(self, n: usize) -> crate::common::Reg<regs::Lpzmkr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize + n * 4usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    #[inline(always)]
    pub const fn lpgpr_alias(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LpgprAlias, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "SNVS_LP General Purpose Registers 0 .. 3"]
    #[inline(always)]
    pub const fn lpgpr(self, n: usize) -> crate::common::Reg<regs::Lpgpr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "SNVS_HP Version ID Register 1"]
    #[inline(always)]
    pub const fn hpvidr1(self) -> crate::common::Reg<regs::Hpvidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bf8usize) as _) }
    }
    #[doc = "SNVS_HP Version ID Register 2"]
    #[inline(always)]
    pub const fn hpvidr2(self) -> crate::common::Reg<regs::Hpvidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0bfcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
