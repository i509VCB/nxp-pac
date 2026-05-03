#[doc = "SDMMC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SDIF {
    ptr: *mut u8,
}
unsafe impl Send for SDIF {}
unsafe impl Sync for SDIF {}
impl SDIF {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Power Enable register."]
    #[inline(always)]
    pub const fn PWREN(self) -> crate::common::Reg<regs::PWREN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Clock Divider register."]
    #[inline(always)]
    pub const fn CLKDIV(self) -> crate::common::Reg<regs::CLKDIV, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Clock Enable register."]
    #[inline(always)]
    pub const fn CLKENA(self) -> crate::common::Reg<regs::CLKENA, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Time-out register."]
    #[inline(always)]
    pub const fn TMOUT(self) -> crate::common::Reg<regs::TMOUT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Card Type register."]
    #[inline(always)]
    pub const fn CTYPE(self) -> crate::common::Reg<regs::CTYPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Block Size register."]
    #[inline(always)]
    pub const fn BLKSIZ(self) -> crate::common::Reg<regs::BLKSIZ, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Byte Count register."]
    #[inline(always)]
    pub const fn BYTCNT(self) -> crate::common::Reg<regs::BYTCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Interrupt Mask register."]
    #[inline(always)]
    pub const fn INTMASK(self) -> crate::common::Reg<regs::INTMASK, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Command Argument register."]
    #[inline(always)]
    pub const fn CMDARG(self) -> crate::common::Reg<regs::CMDARG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Command register."]
    #[inline(always)]
    pub const fn CMD(self) -> crate::common::Reg<regs::CMD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Response register."]
    #[inline(always)]
    pub const fn RESP(self, n: usize) -> crate::common::Reg<regs::RESP, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 4usize) as _) }
    }
    #[doc = "Masked Interrupt Status register."]
    #[inline(always)]
    pub const fn MINTSTS(self) -> crate::common::Reg<regs::MINTSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Raw Interrupt Status register."]
    #[inline(always)]
    pub const fn RINTSTS(self) -> crate::common::Reg<regs::RINTSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Status register."]
    #[inline(always)]
    pub const fn STATUS(self) -> crate::common::Reg<regs::STATUS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "FIFO Threshold Watermark register."]
    #[inline(always)]
    pub const fn FIFOTH(self) -> crate::common::Reg<regs::FIFOTH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Card Detect register."]
    #[inline(always)]
    pub const fn CDETECT(self) -> crate::common::Reg<regs::CDETECT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Write Protect register."]
    #[inline(always)]
    pub const fn WRTPRT(self) -> crate::common::Reg<regs::WRTPRT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Transferred CIU Card Byte Count register."]
    #[inline(always)]
    pub const fn TCBCNT(self) -> crate::common::Reg<regs::TCBCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Transferred Host to BIU-FIFO Byte Count register."]
    #[inline(always)]
    pub const fn TBBCNT(self) -> crate::common::Reg<regs::TBBCNT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Debounce Count register."]
    #[inline(always)]
    pub const fn DEBNCE(self) -> crate::common::Reg<regs::DEBNCE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Hardware Reset."]
    #[inline(always)]
    pub const fn RST_N(self) -> crate::common::Reg<regs::RST_N, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Bus Mode register."]
    #[inline(always)]
    pub const fn BMOD(self) -> crate::common::Reg<regs::BMOD, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Poll Demand register."]
    #[inline(always)]
    pub const fn PLDMND(self) -> crate::common::Reg<regs::PLDMND, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Descriptor List Base Address register."]
    #[inline(always)]
    pub const fn DBADDR(self) -> crate::common::Reg<regs::DBADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Internal DMAC Status register."]
    #[inline(always)]
    pub const fn IDSTS(self) -> crate::common::Reg<regs::IDSTS, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Internal DMAC Interrupt Enable register."]
    #[inline(always)]
    pub const fn IDINTEN(self) -> crate::common::Reg<regs::IDINTEN, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Current Host Descriptor Address register."]
    #[inline(always)]
    pub const fn DSCADDR(self) -> crate::common::Reg<regs::DSCADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Current Buffer Descriptor Address register."]
    #[inline(always)]
    pub const fn BUFADDR(self) -> crate::common::Reg<regs::BUFADDR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Card Threshold Control."]
    #[inline(always)]
    pub const fn CARDTHRCTL(self) -> crate::common::Reg<regs::CARDTHRCTL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Power control."]
    #[inline(always)]
    pub const fn BACKENDPWR(self) -> crate::common::Reg<regs::BACKENDPWR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SDIF FIFO."]
    #[inline(always)]
    pub const fn FIFO(self, n: usize) -> crate::common::Reg<regs::FIFO, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
