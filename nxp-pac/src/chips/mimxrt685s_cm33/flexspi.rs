#[doc = "FlexSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi {}
unsafe impl Sync for Flexspi {}
impl Flexspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Control Register 0"]
    #[inline(always)]
    pub const fn mcr0(self) -> crate::common::Reg<regs::Mcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Module Control Register 1"]
    #[inline(always)]
    pub const fn mcr1(self) -> crate::common::Reg<regs::Mcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Module Control Register 2"]
    #[inline(always)]
    pub const fn mcr2(self) -> crate::common::Reg<regs::Mcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AHB Bus Control Register"]
    #[inline(always)]
    pub const fn ahbcr(self) -> crate::common::Reg<regs::Ahbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LUT Key Register"]
    #[inline(always)]
    pub const fn lutkey(self) -> crate::common::Reg<regs::Lutkey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LUT Control Register"]
    #[inline(always)]
    pub const fn lutcr(self) -> crate::common::Reg<regs::Lutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "AHB RX Buffer 0 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf0cr0(self) -> crate::common::Reg<regs::Ahbrxbuf0cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "AHB RX Buffer 1 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf1cr0(self) -> crate::common::Reg<regs::Ahbrxbuf1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB RX Buffer 2 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf2cr0(self) -> crate::common::Reg<regs::Ahbrxbuf2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AHB RX Buffer 3 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf3cr0(self) -> crate::common::Reg<regs::Ahbrxbuf3cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "AHB RX Buffer 4 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf4cr0(self) -> crate::common::Reg<regs::Ahbrxbuf4cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "AHB RX Buffer 5 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf5cr0(self) -> crate::common::Reg<regs::Ahbrxbuf5cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "AHB RX Buffer 6 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf6cr0(self) -> crate::common::Reg<regs::Ahbrxbuf6cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "AHB RX Buffer 7 Control Register 0"]
    #[inline(always)]
    pub const fn ahbrxbuf7cr0(self) -> crate::common::Reg<regs::Ahbrxbuf7cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha1cr0(self) -> crate::common::Reg<regs::Flsha1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flsha2cr0(self) -> crate::common::Reg<regs::Flsha2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb1cr0(self) -> crate::common::Reg<regs::Flshb1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Flash Control Register 0"]
    #[inline(always)]
    pub const fn flshb2cr0(self) -> crate::common::Reg<regs::Flshb2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Flash Control Register 1"]
    #[inline(always)]
    pub const fn flshcr1(self, n: usize) -> crate::common::Reg<regs::Flshcr1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control Register 2"]
    #[inline(always)]
    pub const fn flshcr2(self, n: usize) -> crate::common::Reg<regs::Flshcr2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control Register 4"]
    #[inline(always)]
    pub const fn flshcr4(self) -> crate::common::Reg<regs::Flshcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Control Register 0"]
    #[inline(always)]
    pub const fn ipcr0(self) -> crate::common::Reg<regs::Ipcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "IP Control Register 1"]
    #[inline(always)]
    pub const fn ipcr1(self) -> crate::common::Reg<regs::Ipcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "IP Command Register"]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::common::Reg<regs::Ipcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Data Learn Pattern Register"]
    #[inline(always)]
    pub const fn dlpr(self) -> crate::common::Reg<regs::Dlpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "IP RX FIFO Control Register"]
    #[inline(always)]
    pub const fn iprxfcr(self) -> crate::common::Reg<regs::Iprxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "IP TX FIFO Control Register"]
    #[inline(always)]
    pub const fn iptxfcr(self) -> crate::common::Reg<regs::Iptxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "DLL Control Register 0"]
    #[inline(always)]
    pub const fn dllcr(self, n: usize) -> crate::common::Reg<regs::Dllcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Status Register 0"]
    #[inline(always)]
    pub const fn sts0(self) -> crate::common::Reg<regs::Sts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Status Register 1"]
    #[inline(always)]
    pub const fn sts1(self) -> crate::common::Reg<regs::Sts1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Status Register 2"]
    #[inline(always)]
    pub const fn sts2(self) -> crate::common::Reg<regs::Sts2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "AHB Suspend Status Register"]
    #[inline(always)]
    pub const fn ahbspndsts(self) -> crate::common::Reg<regs::Ahbspndsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "IP RX FIFO Status Register"]
    #[inline(always)]
    pub const fn iprxfsts(self) -> crate::common::Reg<regs::Iprxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "IP TX FIFO Status Register"]
    #[inline(always)]
    pub const fn iptxfsts(self) -> crate::common::Reg<regs::Iptxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "IP RX FIFO Data Register x"]
    #[inline(always)]
    pub const fn rfdr(self, n: usize) -> crate::common::Reg<regs::Rfdr, crate::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "IP TX FIFO Data Register x"]
    #[inline(always)]
    pub const fn tfdr(self, n: usize) -> crate::common::Reg<regs::Tfdr, crate::common::W> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "LUT x"]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> crate::common::Reg<regs::Lut, crate::common::RW> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "HADDR REMAP START ADDR"]
    #[inline(always)]
    pub const fn haddrstart(self) -> crate::common::Reg<regs::Haddrstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "HADDR REMAP END ADDR"]
    #[inline(always)]
    pub const fn haddrend(self) -> crate::common::Reg<regs::Haddrend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "HADDR REMAP OFFSET"]
    #[inline(always)]
    pub const fn haddroffset(self) -> crate::common::Reg<regs::Haddroffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
}
pub mod regs;
pub mod vals;
