#[doc = "Digital Signal Co-Processing companion to a Cortex-M v8M CPU core"]
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
    #[doc = "Base address register for output region"]
    #[inline(always)]
    pub const fn outbase(self) -> crate::common::Reg<regs::Outbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output format"]
    #[inline(always)]
    pub const fn outformat(self) -> crate::common::Reg<regs::Outformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Base address register for temp region"]
    #[inline(always)]
    pub const fn tmpbase(self) -> crate::common::Reg<regs::Tmpbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Temp format"]
    #[inline(always)]
    pub const fn tmpformat(self) -> crate::common::Reg<regs::Tmpformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Base address register for input A region"]
    #[inline(always)]
    pub const fn inabase(self) -> crate::common::Reg<regs::Inabase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Input A format"]
    #[inline(always)]
    pub const fn inaformat(self) -> crate::common::Reg<regs::Inaformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Base address register for input B region"]
    #[inline(always)]
    pub const fn inbbase(self) -> crate::common::Reg<regs::Inbbase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input B format"]
    #[inline(always)]
    pub const fn inbformat(self) -> crate::common::Reg<regs::Inbformat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "PowerQuad Control register"]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Length register"]
    #[inline(always)]
    pub const fn length(self) -> crate::common::Reg<regs::Length, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Pre-scale register"]
    #[inline(always)]
    pub const fn cppre(self) -> crate::common::Reg<regs::Cppre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Misc register"]
    #[inline(always)]
    pub const fn misc(self) -> crate::common::Reg<regs::Misc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Cursory register"]
    #[inline(always)]
    pub const fn cursory(self) -> crate::common::Reg<regs::Cursory, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Cordic input X register"]
    #[inline(always)]
    pub const fn cordic_x(self) -> crate::common::Reg<regs::CordicX, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Cordic input Y register"]
    #[inline(always)]
    pub const fn cordic_y(self) -> crate::common::Reg<regs::CordicY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Cordic input Z register"]
    #[inline(always)]
    pub const fn cordic_z(self) -> crate::common::Reg<regs::CordicZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Read/Write register where error statuses are captured (sticky)"]
    #[inline(always)]
    pub const fn errstat(self) -> crate::common::Reg<regs::Errstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "INTERRUPT enable register"]
    #[inline(always)]
    pub const fn intren(self) -> crate::common::Reg<regs::Intren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Event Enable register"]
    #[inline(always)]
    pub const fn eventen(self) -> crate::common::Reg<regs::Eventen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "INTERRUPT STATUS register"]
    #[inline(always)]
    pub const fn intrstat(self) -> crate::common::Reg<regs::Intrstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "General purpose register bank N."]
    #[inline(always)]
    pub const fn gpreg(self, n: usize) -> crate::common::Reg<regs::Gpreg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Compute register bank"]
    #[inline(always)]
    pub const fn compreg(self, n: usize) -> crate::common::Reg<regs::Compreg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
