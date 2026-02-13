#[doc = "FLASH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "command register"]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "event register"]
    #[inline(always)]
    pub const fn event(self) -> crate::common::Reg<regs::Event, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "start (or only) address for next flash command"]
    #[inline(always)]
    pub const fn starta(self) -> crate::common::Reg<regs::Starta, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "end address for next flash command, if command operates on address ranges"]
    #[inline(always)]
    pub const fn stopa(self) -> crate::common::Reg<regs::Stopa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "data register, word 0-7; Memory data, or command parameter, or command result."]
    #[inline(always)]
    pub const fn dataw(self, n: usize) -> crate::common::Reg<regs::Dataw, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Clear interrupt enable bits"]
    #[inline(always)]
    pub const fn int_clr_enable(self) -> crate::common::Reg<regs::IntClrEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd8usize) as _) }
    }
    #[doc = "Set interrupt enable bits"]
    #[inline(always)]
    pub const fn int_set_enable(self) -> crate::common::Reg<regs::IntSetEnable, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fdcusize) as _) }
    }
    #[doc = "Interrupt status bits"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "Interrupt enable bits"]
    #[inline(always)]
    pub const fn int_enable(self) -> crate::common::Reg<regs::IntEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "Clear interrupt status bits"]
    #[inline(always)]
    pub const fn int_clr_status(self) -> crate::common::Reg<regs::IntClrStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "Set interrupt status bits"]
    #[inline(always)]
    pub const fn int_set_status(self) -> crate::common::Reg<regs::IntSetStatus, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "Controller+Memory module identification"]
    #[inline(always)]
    pub const fn module_id(self) -> crate::common::Reg<regs::ModuleId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
