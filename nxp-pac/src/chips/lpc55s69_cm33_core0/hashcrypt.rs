#[doc = "Hash-Crypt peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hashcrypt {
    ptr: *mut u8,
}
unsafe impl Send for Hashcrypt {}
unsafe impl Sync for Hashcrypt {}
impl Hashcrypt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register to enable and operate Hash and Crypto"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Indicates status of Hash peripheral."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Write 1 to enable interrupts; reads back with which are set."]
    #[inline(always)]
    pub const fn intenset(self) -> crate::common::Reg<regs::Intenset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Write 1 to clear interrupts."]
    #[inline(always)]
    pub const fn intenclr(self) -> crate::common::Reg<regs::Intenclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Setup Master to access memory (if available)"]
    #[inline(always)]
    pub const fn memctrl(self) -> crate::common::Reg<regs::Memctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Address to start memory access from (if available)."]
    #[inline(always)]
    pub const fn memaddr(self) -> crate::common::Reg<regs::Memaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Input of 16 words at a time to load up buffer."]
    #[inline(always)]
    pub const fn indata(self) -> crate::common::Reg<regs::Indata, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn alias(self, n: usize) -> crate::common::Reg<regs::Alias, crate::common::W> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn digest0(self, n: usize) -> crate::common::Reg<regs::Digest0, crate::common::R> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Crypto settings for AES and Salsa and ChaCha"]
    #[inline(always)]
    pub const fn cryptcfg(self) -> crate::common::Reg<regs::Cryptcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Returns the configuration of this block in this chip - indicates what services are available."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn mask(self, n: usize) -> crate::common::Reg<regs::Mask, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
