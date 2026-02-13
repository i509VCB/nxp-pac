#[doc = "CoolFlux BSP32"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsp320 {
    ptr: *mut u8,
}
unsafe impl Send for Bsp320 {}
unsafe impl Sync for Bsp320 {}
impl Bsp320 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Offset address register for program memory"]
    #[inline(always)]
    pub const fn offset_pmem(self) -> crate::common::Reg<regs::OffsetPmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Offset address register for X-data memory"]
    #[inline(always)]
    pub const fn offset_xmem(self) -> crate::common::Reg<regs::OffsetXmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Offset address register for Y-data memory"]
    #[inline(always)]
    pub const fn offset_ymem(self) -> crate::common::Reg<regs::OffsetYmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Offset address register for mailbox peripheral"]
    #[inline(always)]
    pub const fn offset_mailbox(
        self,
    ) -> crate::common::Reg<regs::OffsetMailbox, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "External interrupt register"]
    #[inline(always)]
    pub const fn interrupts_external(
        self,
    ) -> crate::common::Reg<regs::InterruptsExternal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt status register"]
    #[inline(always)]
    pub const fn interrupts_status(
        self,
    ) -> crate::common::Reg<regs::InterruptsStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CoolFlux BSP32 gating override"]
    #[inline(always)]
    pub const fn cf_gating_override(
        self,
    ) -> crate::common::Reg<regs::CfGatingOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT offset register"]
    #[inline(always)]
    pub const fn ivt_offset(self) -> crate::common::Reg<regs::IvtOffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CoolFlux BSP32 sleep mode register"]
    #[inline(always)]
    pub const fn sleep_mode(self) -> crate::common::Reg<regs::SleepMode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT register 0 content"]
    #[inline(always)]
    pub const fn ivt0(self) -> crate::common::Reg<regs::Ivt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT register 1 content"]
    #[inline(always)]
    pub const fn ivt1(self) -> crate::common::Reg<regs::Ivt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT register 2 content"]
    #[inline(always)]
    pub const fn ivt2(self) -> crate::common::Reg<regs::Ivt2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT register 3 content"]
    #[inline(always)]
    pub const fn ivt3(self) -> crate::common::Reg<regs::Ivt3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "CoolFlux BSP32 IVT disable register"]
    #[inline(always)]
    pub const fn ivt_disable(self) -> crate::common::Reg<regs::IvtDisable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
}
pub mod regs;
