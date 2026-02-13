#[doc = "SDMMC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdif {
    ptr: *mut u8,
}
unsafe impl Send for Sdif {}
unsafe impl Sync for Sdif {}
impl Sdif {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Enable register"]
    #[inline(always)]
    pub const fn pwren(self) -> crate::common::Reg<regs::Pwren, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Divider register"]
    #[inline(always)]
    pub const fn clkdiv(self) -> crate::common::Reg<regs::Clkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Enable register"]
    #[inline(always)]
    pub const fn clkena(self) -> crate::common::Reg<regs::Clkena, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Time-out register"]
    #[inline(always)]
    pub const fn tmout(self) -> crate::common::Reg<regs::Tmout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Card Type register"]
    #[inline(always)]
    pub const fn ctype(self) -> crate::common::Reg<regs::Ctype, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Block Size register"]
    #[inline(always)]
    pub const fn blksiz(self) -> crate::common::Reg<regs::Blksiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Byte Count register"]
    #[inline(always)]
    pub const fn bytcnt(self) -> crate::common::Reg<regs::Bytcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Mask register"]
    #[inline(always)]
    pub const fn intmask(self) -> crate::common::Reg<regs::Intmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Command Argument register"]
    #[inline(always)]
    pub const fn cmdarg(self) -> crate::common::Reg<regs::Cmdarg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Command register"]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Response register"]
    #[inline(always)]
    pub const fn resp(self, n: usize) -> crate::common::Reg<regs::Resp, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Masked Interrupt Status register"]
    #[inline(always)]
    pub const fn mintsts(self) -> crate::common::Reg<regs::Mintsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Raw Interrupt Status register"]
    #[inline(always)]
    pub const fn rintsts(self) -> crate::common::Reg<regs::Rintsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "FIFO Threshold Watermark register"]
    #[inline(always)]
    pub const fn fifoth(self) -> crate::common::Reg<regs::Fifoth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Card Detect register"]
    #[inline(always)]
    pub const fn cdetect(self) -> crate::common::Reg<regs::Cdetect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Write Protect register"]
    #[inline(always)]
    pub const fn wrtprt(self) -> crate::common::Reg<regs::Wrtprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Transferred CIU Card Byte Count register"]
    #[inline(always)]
    pub const fn tcbcnt(self) -> crate::common::Reg<regs::Tcbcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Transferred Host to BIU-FIFO Byte Count register"]
    #[inline(always)]
    pub const fn tbbcnt(self) -> crate::common::Reg<regs::Tbbcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Debounce Count register"]
    #[inline(always)]
    pub const fn debnce(self) -> crate::common::Reg<regs::Debnce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Hardware Reset"]
    #[inline(always)]
    pub const fn rst_n(self) -> crate::common::Reg<regs::RstN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Bus Mode register"]
    #[inline(always)]
    pub const fn bmod(self) -> crate::common::Reg<regs::Bmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Poll Demand register"]
    #[inline(always)]
    pub const fn pldmnd(self) -> crate::common::Reg<regs::Pldmnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Descriptor List Base Address register"]
    #[inline(always)]
    pub const fn dbaddr(self) -> crate::common::Reg<regs::Dbaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Internal DMAC Status register"]
    #[inline(always)]
    pub const fn idsts(self) -> crate::common::Reg<regs::Idsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Internal DMAC Interrupt Enable register"]
    #[inline(always)]
    pub const fn idinten(self) -> crate::common::Reg<regs::Idinten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Current Host Descriptor Address register"]
    #[inline(always)]
    pub const fn dscaddr(self) -> crate::common::Reg<regs::Dscaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Current Buffer Descriptor Address register"]
    #[inline(always)]
    pub const fn bufaddr(self) -> crate::common::Reg<regs::Bufaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Card Threshold Control"]
    #[inline(always)]
    pub const fn cardthrctl(self) -> crate::common::Reg<regs::Cardthrctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Power control"]
    #[inline(always)]
    pub const fn backendpwr(self) -> crate::common::Reg<regs::Backendpwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SDIF FIFO"]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
