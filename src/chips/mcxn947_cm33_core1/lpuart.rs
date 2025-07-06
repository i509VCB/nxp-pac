#[doc = "LPUART"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpuart {
    ptr: *mut u8,
}
unsafe impl Send for Lpuart {}
unsafe impl Sync for Lpuart {}
impl Lpuart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Global"]
    #[inline(always)]
    pub const fn global(self) -> crate::common::Reg<regs::Global, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Pin Configuration"]
    #[inline(always)]
    pub const fn pincfg(self) -> crate::common::Reg<regs::Pincfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Baud Rate"]
    #[inline(always)]
    pub const fn baud(self) -> crate::common::Reg<regs::Baud, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Match Address"]
    #[inline(always)]
    pub const fn match_(self) -> crate::common::Reg<regs::Match, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MODEM IrDA"]
    #[inline(always)]
    pub const fn modir(self) -> crate::common::Reg<regs::Modir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FIFO"]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Watermark"]
    #[inline(always)]
    pub const fn water(self) -> crate::common::Reg<regs::Water, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Data Read-Only"]
    #[inline(always)]
    pub const fn dataro(self) -> crate::common::Reg<regs::Dataro, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "MODEM Control"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "MODEM Status"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Receiver Extended Idle"]
    #[inline(always)]
    pub const fn reir(self) -> crate::common::Reg<regs::Reir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Transmitter Extended Idle"]
    #[inline(always)]
    pub const fn teir(self) -> crate::common::Reg<regs::Teir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Half Duplex Control"]
    #[inline(always)]
    pub const fn hdcr(self) -> crate::common::Reg<regs::Hdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Timeout Control"]
    #[inline(always)]
    pub const fn tocr(self) -> crate::common::Reg<regs::Tocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Timeout Status"]
    #[inline(always)]
    pub const fn tosr(self) -> crate::common::Reg<regs::Tosr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Timeout N"]
    #[inline(always)]
    pub const fn timeout(self, n: usize) -> crate::common::Reg<regs::Timeout, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Transmit Command Burst"]
    #[inline(always)]
    pub const fn tcbr(self, n: usize) -> crate::common::Reg<regs::Tcbr, crate::common::W> {
        assert!(n < 128usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "Transmit Data Burst"]
    #[inline(always)]
    pub const fn tdbr(self, n: usize) -> crate::common::Reg<regs::Tdbr, crate::common::W> {
        assert!(n < 256usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
