#[doc = "PMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmu {
    ptr: *mut u8,
}
unsafe impl Send for Pmu {}
unsafe impl Sync for Pmu {}
impl Pmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1(self) -> crate::common::Reg<regs::Reg1p1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_set(self) -> crate::common::Reg<regs::Reg1p1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_clr(self) -> crate::common::Reg<regs::Reg1p1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Regulator 1P1 Register"]
    #[inline(always)]
    pub const fn reg_1p1_tog(self) -> crate::common::Reg<regs::Reg1p1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0(self) -> crate::common::Reg<regs::Reg3p0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_set(self) -> crate::common::Reg<regs::Reg3p0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_clr(self) -> crate::common::Reg<regs::Reg3p0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Regulator 3P0 Register"]
    #[inline(always)]
    pub const fn reg_3p0_tog(self) -> crate::common::Reg<regs::Reg3p0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5(self) -> crate::common::Reg<regs::Reg2p5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_set(self) -> crate::common::Reg<regs::Reg2p5Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_clr(self) -> crate::common::Reg<regs::Reg2p5Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Regulator 2P5 Register"]
    #[inline(always)]
    pub const fn reg_2p5_tog(self) -> crate::common::Reg<regs::Reg2p5Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core(self) -> crate::common::Reg<regs::RegCore, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_set(self) -> crate::common::Reg<regs::RegCoreSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_clr(self) -> crate::common::Reg<regs::RegCoreClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Digital Regulator Core Register"]
    #[inline(always)]
    pub const fn reg_core_tog(self) -> crate::common::Reg<regs::RegCoreTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0(self) -> crate::common::Reg<regs::Misc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_set(self) -> crate::common::Reg<regs::Misc0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_clr(self) -> crate::common::Reg<regs::Misc0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_tog(self) -> crate::common::Reg<regs::Misc0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1(self) -> crate::common::Reg<regs::Misc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_set(self) -> crate::common::Reg<regs::Misc1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_clr(self) -> crate::common::Reg<regs::Misc1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Miscellaneous Register 1"]
    #[inline(always)]
    pub const fn misc1_tog(self) -> crate::common::Reg<regs::Misc1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2(self) -> crate::common::Reg<regs::Misc2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_set(self) -> crate::common::Reg<regs::Misc2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_clr(self) -> crate::common::Reg<regs::Misc2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn misc2_tog(self) -> crate::common::Reg<regs::Misc2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
