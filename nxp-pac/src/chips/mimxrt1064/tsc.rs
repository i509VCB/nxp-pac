#[doc = "Touch Screen Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsc {
    ptr: *mut u8,
}
unsafe impl Send for Tsc {}
unsafe impl Sync for Tsc {}
impl Tsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Basic Setting"]
    #[inline(always)]
    pub const fn basic_setting(self) -> crate::common::Reg<regs::BasicSetting, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pre-charge Time"]
    #[inline(always)]
    pub const fn pre_charge_time(
        self,
    ) -> crate::common::Reg<regs::PreChargeTime, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Flow Control"]
    #[inline(always)]
    pub const fn flow_control(self) -> crate::common::Reg<regs::FlowControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Measure Value"]
    #[inline(always)]
    pub const fn measeure_value(self) -> crate::common::Reg<regs::MeaseureValue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn int_sig_en(self) -> crate::common::Reg<regs::IntSigEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Intterrupt Status"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Debug Mode Register"]
    #[inline(always)]
    pub const fn debug_mode(self) -> crate::common::Reg<regs::DebugMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Debug Mode Register 2"]
    #[inline(always)]
    pub const fn debug_mode2(self) -> crate::common::Reg<regs::DebugMode2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
}
pub mod regs;
pub mod vals;
