#[doc = "Digital Signal Co-Processing companion to a Cortex-M v8M CPU core."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct POWERQUAD {
    ptr: *mut u8,
}
unsafe impl Send for POWERQUAD {}
unsafe impl Sync for POWERQUAD {}
impl POWERQUAD {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Base address register for output region."]
    #[inline(always)]
    pub const fn OUTBASE(self) -> crate::common::Reg<regs::OUTBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output format."]
    #[inline(always)]
    pub const fn OUTFORMAT(self) -> crate::common::Reg<regs::OUTFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Base address register for temp region."]
    #[inline(always)]
    pub const fn TMPBASE(self) -> crate::common::Reg<regs::TMPBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Temp format."]
    #[inline(always)]
    pub const fn TMPFORMAT(self) -> crate::common::Reg<regs::TMPFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Base address register for input A region."]
    #[inline(always)]
    pub const fn INABASE(self) -> crate::common::Reg<regs::INABASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Input A format."]
    #[inline(always)]
    pub const fn INAFORMAT(self) -> crate::common::Reg<regs::INAFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Base address register for input B region."]
    #[inline(always)]
    pub const fn INBBASE(self) -> crate::common::Reg<regs::INBBASE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input B format."]
    #[inline(always)]
    pub const fn INBFORMAT(self) -> crate::common::Reg<regs::INBFORMAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "PowerQuad Control register."]
    #[inline(always)]
    pub const fn CONTROL(self) -> crate::common::Reg<regs::CONTROL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Length register."]
    #[inline(always)]
    pub const fn LENGTH(self) -> crate::common::Reg<regs::LENGTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Pre-scale register."]
    #[inline(always)]
    pub const fn CPPRE(self) -> crate::common::Reg<regs::CPPRE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Misc register."]
    #[inline(always)]
    pub const fn MISC(self) -> crate::common::Reg<regs::MISC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Cursory register."]
    #[inline(always)]
    pub const fn CURSORY(self) -> crate::common::Reg<regs::CURSORY, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Cordic input X register."]
    #[inline(always)]
    pub const fn CORDIC_X(self) -> crate::common::Reg<regs::CORDIC_X, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Cordic input Y register."]
    #[inline(always)]
    pub const fn CORDIC_Y(self) -> crate::common::Reg<regs::CORDIC_Y, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Cordic input Z register."]
    #[inline(always)]
    pub const fn CORDIC_Z(self) -> crate::common::Reg<regs::CORDIC_Z, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "Read/Write register where error statuses are captured (sticky)."]
    #[inline(always)]
    pub const fn ERRSTAT(self) -> crate::common::Reg<regs::ERRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "INTERRUPT enable register."]
    #[inline(always)]
    pub const fn INTREN(self) -> crate::common::Reg<regs::INTREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "Event Enable register."]
    #[inline(always)]
    pub const fn EVENTEN(self) -> crate::common::Reg<regs::EVENTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "INTERRUPT STATUS register."]
    #[inline(always)]
    pub const fn INTRSTAT(self) -> crate::common::Reg<regs::INTRSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "General purpose register bank N."]
    #[inline(always)]
    pub const fn gpreg(self, n: usize) -> crate::common::Reg<regs::gpreg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Compute register bank."]
    #[inline(always)]
    pub const fn compreg(self, n: usize) -> crate::common::Reg<regs::compreg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
