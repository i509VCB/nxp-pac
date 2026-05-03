#[doc = "Hash-Crypt peripheral."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HASHCRYPT {
    ptr: *mut u8,
}
unsafe impl Send for HASHCRYPT {}
unsafe impl Sync for HASHCRYPT {}
impl HASHCRYPT {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register to enable and operate Hash and Crypto."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Indicates status of Hash peripheral."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Write 1 to enable interrupts; reads back with which are set."]
    #[inline(always)]
    pub const fn INTENSET(self) -> crate::common::Reg<regs::INTENSET, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Write 1 to clear interrupts."]
    #[inline(always)]
    pub const fn INTENCLR(self) -> crate::common::Reg<regs::INTENCLR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Setup Master to access memory (if available)."]
    #[inline(always)]
    pub const fn MEMCTRL(self) -> crate::common::Reg<regs::MEMCTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Address to start memory access from (if available)."]
    #[inline(always)]
    pub const fn MEMADDR(self) -> crate::common::Reg<regs::MEMADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Input of 16 words at a time to load up buffer."]
    #[inline(always)]
    pub const fn INDATA(self) -> crate::common::Reg<regs::INDATA, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn ALIAS(self, n: usize) -> crate::common::Reg<regs::ALIAS, crate::common::W> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn DIGEST0(self, n: usize) -> crate::common::Reg<regs::DIGEST0, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Crypto settings for AES and Salsa and ChaCha."]
    #[inline(always)]
    pub const fn CRYPTCFG(self) -> crate::common::Reg<regs::CRYPTCFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
    #[inline(always)]
    pub const fn CONFIG(self) -> crate::common::Reg<regs::CONFIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
    #[inline(always)]
    pub const fn LOCK(self) -> crate::common::Reg<regs::LOCK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available."]
    #[inline(always)]
    pub const fn MASK(self, n: usize) -> crate::common::Reg<regs::MASK, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
