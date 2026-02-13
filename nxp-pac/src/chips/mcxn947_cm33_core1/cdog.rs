#[doc = "CDOG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdog {
    ptr: *mut u8,
}
unsafe impl Send for Cdog {}
unsafe impl Sync for Cdog {}
impl Cdog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Instruction Timer Reload Register"]
    #[inline(always)]
    pub const fn reload(self) -> crate::common::Reg<regs::Reload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Instruction Timer Register"]
    #[inline(always)]
    pub const fn instruction_timer(
        self,
    ) -> crate::common::Reg<regs::InstructionTimer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Status 1 Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status 2 Register"]
    #[inline(always)]
    pub const fn status2(self) -> crate::common::Reg<regs::Status2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Flags Register"]
    #[inline(always)]
    pub const fn flags(self) -> crate::common::Reg<regs::Flags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Persistent Data Storage Register"]
    #[inline(always)]
    pub const fn persistent(self) -> crate::common::Reg<regs::Persistent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "START Command Register"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "STOP Command Register"]
    #[inline(always)]
    pub const fn stop(self) -> crate::common::Reg<regs::Stop, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RESTART Command Register"]
    #[inline(always)]
    pub const fn restart(self) -> crate::common::Reg<regs::Restart, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "ADD Command Register"]
    #[inline(always)]
    pub const fn add(self) -> crate::common::Reg<regs::Add, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "ADD1 Command Register"]
    #[inline(always)]
    pub const fn add1(self) -> crate::common::Reg<regs::Add1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "ADD16 Command Register"]
    #[inline(always)]
    pub const fn add16(self) -> crate::common::Reg<regs::Add16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "ADD256 Command Register"]
    #[inline(always)]
    pub const fn add256(self) -> crate::common::Reg<regs::Add256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SUB Command Register"]
    #[inline(always)]
    pub const fn sub(self) -> crate::common::Reg<regs::Sub, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SUB1 Command Register"]
    #[inline(always)]
    pub const fn sub1(self) -> crate::common::Reg<regs::Sub1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SUB16 Command Register"]
    #[inline(always)]
    pub const fn sub16(self) -> crate::common::Reg<regs::Sub16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SUB256 Command Register"]
    #[inline(always)]
    pub const fn sub256(self) -> crate::common::Reg<regs::Sub256, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "ASSERT16 Command Register"]
    #[inline(always)]
    pub const fn assert16(self) -> crate::common::Reg<regs::Assert16, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
