#[doc = "PowerQuad"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powerquad {
    ptr: *mut u8,
}
unsafe impl Send for Powerquad {}
unsafe impl Sync for Powerquad {}
impl Powerquad {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output Base"]
    #[inline(always)]
    pub const fn outbase(self) -> crate::common::Reg<regs::Outbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output Format"]
    #[inline(always)]
    pub const fn outformat(self) -> crate::common::Reg<regs::Outformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Temporary Base"]
    #[inline(always)]
    pub const fn tmpbase(self) -> crate::common::Reg<regs::Tmpbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Temporary Format"]
    #[inline(always)]
    pub const fn tmpformat(self) -> crate::common::Reg<regs::Tmpformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Input A Base"]
    #[inline(always)]
    pub const fn inabase(self) -> crate::common::Reg<regs::Inabase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Input A Format"]
    #[inline(always)]
    pub const fn inaformat(self) -> crate::common::Reg<regs::Inaformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Input B Base"]
    #[inline(always)]
    pub const fn inbbase(self) -> crate::common::Reg<regs::Inbbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input B Format"]
    #[inline(always)]
    pub const fn inbformat(self) -> crate::common::Reg<regs::Inbformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn length(self) -> crate::common::Reg<regs::Length, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Coprocessor Prescale"]
    #[inline(always)]
    pub const fn cppre(self) -> crate::common::Reg<regs::Cppre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Miscellaneous"]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Cursory"]
    #[inline(always)]
    pub const fn cursory(self) -> crate::common::Reg<regs::Cursory, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "CORDIC Input X"]
    #[inline(always)]
    pub const fn cordic_x(self) -> crate::common::Reg<regs::CordicX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "CORDIC Input Y"]
    #[inline(always)]
    pub const fn cordic_y(self) -> crate::common::Reg<regs::CordicY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "CORDIC Input Z"]
    #[inline(always)]
    pub const fn cordic_z(self) -> crate::common::Reg<regs::CordicZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub const fn errstat(self) -> crate::common::Reg<regs::Errstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn intren(self) -> crate::common::Reg<regs::Intren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Event Enable"]
    #[inline(always)]
    pub const fn eventen(self) -> crate::common::Reg<regs::Eventen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn intrstat(self) -> crate::common::Reg<regs::Intrstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "General Purpose Register Bank n"]
    #[inline(always)]
    pub const fn gpreg(self, n: usize) -> crate::common::Reg<regs::Gpreg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Compute Register Bank n"]
    #[inline(always)]
    pub const fn compreg(self, n: usize) -> crate::common::Reg<regs::Compreg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
