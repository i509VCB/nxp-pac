#[doc = "Array of registers: LUT_INP_MUX%s"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut {
    ptr: *mut u8,
}
unsafe impl Send for Lut {}
unsafe impl Sync for Lut {}
impl Lut {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Input select register for LUTn (0 to 25), Inputx (5 inputs)"]
    #[inline(always)]
    pub const fn lut_inp_mux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LutInpMux, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "Programmable Logic Unit (PLU)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plu0 {
    ptr: *mut u8,
}
unsafe impl Send for Plu0 {}
unsafe impl Sync for Plu0 {}
impl Plu0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: LUT_INP_MUX%s"]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> Lut {
        assert!(n < 26usize);
        unsafe { Lut::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "PLU LUT truth table"]
    #[inline(always)]
    pub const fn lut_truth(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LutTruth, crate::common::RW> {
        assert!(n < 26usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize + n * 4usize) as _)
        }
    }
    #[doc = "PLU outputs"]
    #[inline(always)]
    pub const fn outputs(self) -> crate::common::Reg<regs::Outputs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Wakeup interrupt control"]
    #[inline(always)]
    pub const fn wakeint_ctrl(self) -> crate::common::Reg<regs::WakeintCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "PLU output multiplexer"]
    #[inline(always)]
    pub const fn output_mux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::OutputMux, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
