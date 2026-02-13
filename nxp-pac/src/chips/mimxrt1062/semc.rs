#[doc = "SEMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Semc {
    ptr: *mut u8,
}
unsafe impl Send for Semc {}
unsafe impl Sync for Semc {}
impl Semc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Control Register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "IO MUX Control Register"]
    #[inline(always)]
    pub const fn iocr(self) -> crate::common::Reg<regs::Iocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Bus (AXI) Master Control Register 0"]
    #[inline(always)]
    pub const fn bmcr0(self) -> crate::common::Reg<regs::Bmcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Bus (AXI) Master Control Register 1"]
    #[inline(always)]
    pub const fn bmcr1(self) -> crate::common::Reg<regs::Bmcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Base Register n"]
    #[inline(always)]
    pub const fn br(self, n: usize) -> crate::common::Reg<regs::Br, crate::common::RW> {
        assert!(n < 9usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(self) -> crate::common::Reg<regs::Dllcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Interrupt Register"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SDRAM Control Register 0"]
    #[inline(always)]
    pub const fn sdramcr0(self) -> crate::common::Reg<regs::Sdramcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SDRAM Control Register 1"]
    #[inline(always)]
    pub const fn sdramcr1(self) -> crate::common::Reg<regs::Sdramcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SDRAM Control Register 2"]
    #[inline(always)]
    pub const fn sdramcr2(self) -> crate::common::Reg<regs::Sdramcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "SDRAM Control Register 3"]
    #[inline(always)]
    pub const fn sdramcr3(self) -> crate::common::Reg<regs::Sdramcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "NAND Control Register 0"]
    #[inline(always)]
    pub const fn nandcr0(self) -> crate::common::Reg<regs::Nandcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "NAND Control Register 1"]
    #[inline(always)]
    pub const fn nandcr1(self) -> crate::common::Reg<regs::Nandcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "NAND Control Register 2"]
    #[inline(always)]
    pub const fn nandcr2(self) -> crate::common::Reg<regs::Nandcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "NAND Control Register 3"]
    #[inline(always)]
    pub const fn nandcr3(self) -> crate::common::Reg<regs::Nandcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "NOR Control Register 0"]
    #[inline(always)]
    pub const fn norcr0(self) -> crate::common::Reg<regs::Norcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "NOR Control Register 1"]
    #[inline(always)]
    pub const fn norcr1(self) -> crate::common::Reg<regs::Norcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "NOR Control Register 2"]
    #[inline(always)]
    pub const fn norcr2(self) -> crate::common::Reg<regs::Norcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "NOR Control Register 3"]
    #[inline(always)]
    pub const fn norcr3(self) -> crate::common::Reg<regs::Norcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "SRAM Control Register 0"]
    #[inline(always)]
    pub const fn sramcr0(self) -> crate::common::Reg<regs::Sramcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "SRAM Control Register 1"]
    #[inline(always)]
    pub const fn sramcr1(self) -> crate::common::Reg<regs::Sramcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "SRAM Control Register 2"]
    #[inline(always)]
    pub const fn sramcr2(self) -> crate::common::Reg<regs::Sramcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "SRAM Control Register 3"]
    #[inline(always)]
    pub const fn sramcr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "DBI-B Control Register 0"]
    #[inline(always)]
    pub const fn dbicr0(self) -> crate::common::Reg<regs::Dbicr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "DBI-B Control Register 1"]
    #[inline(always)]
    pub const fn dbicr1(self) -> crate::common::Reg<regs::Dbicr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "IP Command Control Register 0"]
    #[inline(always)]
    pub const fn ipcr0(self) -> crate::common::Reg<regs::Ipcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "IP Command Control Register 1"]
    #[inline(always)]
    pub const fn ipcr1(self) -> crate::common::Reg<regs::Ipcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Command Control Register 2"]
    #[inline(always)]
    pub const fn ipcr2(self) -> crate::common::Reg<regs::Ipcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "IP Command Register"]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::common::Reg<regs::Ipcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "TX DATA Register"]
    #[inline(always)]
    pub const fn iptxdat(self) -> crate::common::Reg<regs::Iptxdat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RX DATA Register"]
    #[inline(always)]
    pub const fn iprxdat(self) -> crate::common::Reg<regs::Iprxdat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Status Register 0"]
    #[inline(always)]
    pub const fn sts0(self) -> crate::common::Reg<regs::Sts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Status Register 1"]
    #[inline(always)]
    pub const fn sts1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Status Register 2"]
    #[inline(always)]
    pub const fn sts2(self) -> crate::common::Reg<regs::Sts2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Status Register 3"]
    #[inline(always)]
    pub const fn sts3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Status Register 4"]
    #[inline(always)]
    pub const fn sts4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Status Register 5"]
    #[inline(always)]
    pub const fn sts5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Status Register 6"]
    #[inline(always)]
    pub const fn sts6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Status Register 7"]
    #[inline(always)]
    pub const fn sts7(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Status Register 8"]
    #[inline(always)]
    pub const fn sts8(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Status Register 9"]
    #[inline(always)]
    pub const fn sts9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Status Register 10"]
    #[inline(always)]
    pub const fn sts10(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Status Register 11"]
    #[inline(always)]
    pub const fn sts11(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Status Register 12"]
    #[inline(always)]
    pub const fn sts12(self) -> crate::common::Reg<regs::Sts12, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Status Register 13"]
    #[inline(always)]
    pub const fn sts13(self) -> crate::common::Reg<regs::Sts13, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Status Register 14"]
    #[inline(always)]
    pub const fn sts14(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Status Register 15"]
    #[inline(always)]
    pub const fn sts15(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
