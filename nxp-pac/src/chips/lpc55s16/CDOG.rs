#[doc = "CDOG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CDOG {
    ptr: *mut u8,
}
unsafe impl Send for CDOG {}
unsafe impl Sync for CDOG {}
impl CDOG {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "The control fields, which constitute CONTROL, control all controllable attributes of the module, including those of CONTROL itself."]
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Instruction timer reload."]
    #[inline(always)]
    pub const fn RELOAD(self) -> crate::common::Reg<regs::RELOAD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "The INSTRUCTION TIMER itself."]
    #[inline(always)]
    pub const fn INSTRUCTION_TIMER(
        self,
    ) -> crate::common::Reg<regs::INSTRUCTION_TIMER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Also known as SEC_CNT."]
    #[inline(always)]
    pub const fn SECURE_COUNTER(
        self,
    ) -> crate::common::Reg<regs::SECURE_COUNTER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Status register (1 of 2)."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "STATUS register (2 of 2)."]
    #[inline(always)]
    pub const fn STATUS2(self) -> crate::common::Reg<regs::STATUS2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Hardware flags."]
    #[inline(always)]
    pub const fn FLAGS(self) -> crate::common::Reg<regs::FLAGS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Persistent (Ad. Hoc., quasi-NV) data storage."]
    #[inline(always)]
    pub const fn PERSISTENT(self) -> crate::common::Reg<regs::PERSISTENT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Write address for issuing the START command."]
    #[inline(always)]
    pub const fn START(self) -> crate::common::Reg<regs::START, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Write address for issuing the STOP command."]
    #[inline(always)]
    pub const fn STOP(self) -> crate::common::Reg<regs::STOP, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Write address for issuing the RESTART command."]
    #[inline(always)]
    pub const fn RESTART(self) -> crate::common::Reg<regs::RESTART, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Write address for issuing the ADD command."]
    #[inline(always)]
    pub const fn ADD(self) -> crate::common::Reg<regs::ADD, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Write address for issuing the ADD1 command."]
    #[inline(always)]
    pub const fn ADD1(self) -> crate::common::Reg<regs::ADD1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn ADD16(self) -> crate::common::Reg<regs::ADD16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Write address for issuing the ADD16 command."]
    #[inline(always)]
    pub const fn ADD256(self) -> crate::common::Reg<regs::ADD256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Write address for issuing the SUB command."]
    #[inline(always)]
    pub const fn SUB(self) -> crate::common::Reg<regs::SUB, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Write address for issuing the SUB1 command."]
    #[inline(always)]
    pub const fn SUB1(self) -> crate::common::Reg<regs::SUB1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Write address for issuing the SUB16 command."]
    #[inline(always)]
    pub const fn SUB16(self) -> crate::common::Reg<regs::SUB16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Write address for issuing the SUB256 command."]
    #[inline(always)]
    pub const fn SUB256(self) -> crate::common::Reg<regs::SUB256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs;
