#[doc = "no description available"]
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
    #[doc = "LUTn input x MUX"]
    #[inline(always)]
    pub const fn lut_inp_mux(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::LutInpMux, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "LPC80X Programmable Logic Unit (PLU)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plu {
    ptr: *mut u8,
}
unsafe impl Send for Plu {}
unsafe impl Sync for Plu {}
impl Plu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> Lut {
        assert!(n < 26usize);
        unsafe { Lut::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Specifies the Truth Table contents for LUTLUTn"]
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
    #[doc = "Provides the current state of the 8 designated PLU Outputs."]
    #[inline(always)]
    pub const fn outputs(self) -> crate::common::Reg<regs::Outputs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Wakeup interrupt control for PLU"]
    #[inline(always)]
    pub const fn wakeint_ctrl(self) -> crate::common::Reg<regs::WakeintCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0904usize) as _) }
    }
    #[doc = "Selects the source to be connected to PLU Output OUTPUT_n"]
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
