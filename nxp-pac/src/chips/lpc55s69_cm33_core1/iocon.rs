#[doc = "I/O pin configuration (IOCON)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocon {
    ptr: *mut u8,
}
unsafe impl Send for Iocon {}
unsafe impl Sync for Iocon {}
impl Iocon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Digital I/O control for port 0 pins PIO0_0"]
    #[inline(always)]
    pub const fn pio0(self, n: usize) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Digital I/O control for port 1 pins PIO1_0"]
    #[inline(always)]
    pub const fn pio1(self, n: usize) -> crate::common::Reg<regs::Pio, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
