#[doc = "FlexSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspi0 {
    ptr: *mut u8,
}
unsafe impl Send for Flexspi0 {}
unsafe impl Sync for Flexspi0 {}
impl Flexspi0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Module Control 0"]
    #[inline(always)]
    pub const fn mcr0(self) -> crate::common::Reg<regs::Mcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Module Control 1"]
    #[inline(always)]
    pub const fn mcr1(self) -> crate::common::Reg<regs::Mcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Module Control 2"]
    #[inline(always)]
    pub const fn mcr2(self) -> crate::common::Reg<regs::Mcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AHB Bus Control"]
    #[inline(always)]
    pub const fn ahbcr(self) -> crate::common::Reg<regs::Ahbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LUT Key"]
    #[inline(always)]
    pub const fn lutkey(self) -> crate::common::Reg<regs::Lutkey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LUT Control"]
    #[inline(always)]
    pub const fn lutcr(self) -> crate::common::Reg<regs::Lutcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "AHB Receive Buffer 0 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf0cr0(self) -> crate::common::Reg<regs::Ahbrxbuf0cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "AHB Receive Buffer 1 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf1cr0(self) -> crate::common::Reg<regs::Ahbrxbuf1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB Receive Buffer 2 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf2cr0(self) -> crate::common::Reg<regs::Ahbrxbuf2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AHB Receive Buffer 3 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf3cr0(self) -> crate::common::Reg<regs::Ahbrxbuf3cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "AHB Receive Buffer 4 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf4cr0(self) -> crate::common::Reg<regs::Ahbrxbuf4cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "AHB Receive Buffer 5 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf5cr0(self) -> crate::common::Reg<regs::Ahbrxbuf5cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "AHB Receive Buffer 6 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf6cr0(self) -> crate::common::Reg<regs::Ahbrxbuf6cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "AHB Receive Buffer 7 Control 0"]
    #[inline(always)]
    pub const fn ahbrxbuf7cr0(self) -> crate::common::Reg<regs::Ahbrxbuf7cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Flash Control 0"]
    #[inline(always)]
    pub const fn flsha1cr0(self) -> crate::common::Reg<regs::Flsha1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Flash Control 0"]
    #[inline(always)]
    pub const fn flsha2cr0(self) -> crate::common::Reg<regs::Flsha2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Flash Control 0"]
    #[inline(always)]
    pub const fn flshb1cr0(self) -> crate::common::Reg<regs::Flshb1cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Flash Control 0"]
    #[inline(always)]
    pub const fn flshb2cr0(self) -> crate::common::Reg<regs::Flshb2cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Flash Control 1"]
    #[inline(always)]
    pub const fn flshcr1(self, n: usize) -> crate::common::Reg<regs::Flshcr1, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control 2"]
    #[inline(always)]
    pub const fn flshcr2(self, n: usize) -> crate::common::Reg<regs::Flshcr2, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Flash Control 4"]
    #[inline(always)]
    pub const fn flshcr4(self) -> crate::common::Reg<regs::Flshcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "IP Control 0"]
    #[inline(always)]
    pub const fn ipcr0(self) -> crate::common::Reg<regs::Ipcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "IP Control 1"]
    #[inline(always)]
    pub const fn ipcr1(self) -> crate::common::Reg<regs::Ipcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "IP Control 2"]
    #[inline(always)]
    pub const fn ipcr2(self) -> crate::common::Reg<regs::Ipcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "IP Command"]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::common::Reg<regs::Ipcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Data Learning Pattern"]
    #[inline(always)]
    pub const fn dlpr(self) -> crate::common::Reg<regs::Dlpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "IP Receive FIFO Control"]
    #[inline(always)]
    pub const fn iprxfcr(self) -> crate::common::Reg<regs::Iprxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "IP Transmit FIFO Control"]
    #[inline(always)]
    pub const fn iptxfcr(self) -> crate::common::Reg<regs::Iptxfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "DLL Control 0"]
    #[inline(always)]
    pub const fn dllcr(self, n: usize) -> crate::common::Reg<regs::Dllcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Status 0"]
    #[inline(always)]
    pub const fn sts0(self) -> crate::common::Reg<regs::Sts0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Status 1"]
    #[inline(always)]
    pub const fn sts1(self) -> crate::common::Reg<regs::Sts1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Status 2"]
    #[inline(always)]
    pub const fn sts2(self) -> crate::common::Reg<regs::Sts2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "AHB Suspend Status"]
    #[inline(always)]
    pub const fn ahbspndsts(self) -> crate::common::Reg<regs::Ahbspndsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "IP Receive FIFO Status"]
    #[inline(always)]
    pub const fn iprxfsts(self) -> crate::common::Reg<regs::Iprxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "IP Transmit FIFO Status"]
    #[inline(always)]
    pub const fn iptxfsts(self) -> crate::common::Reg<regs::Iptxfsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "IP Receive FIFO Data x"]
    #[inline(always)]
    pub const fn rfdr(self, n: usize) -> crate::common::Reg<regs::Rfdr, crate::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "IP TX FIFO Data x"]
    #[inline(always)]
    pub const fn tfdr(self, n: usize) -> crate::common::Reg<regs::Tfdr, crate::common::W> {
        assert!(n < 32usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "Lookup Table x"]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> crate::common::Reg<regs::Lut, crate::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "HADDR REMAP Start Address"]
    #[inline(always)]
    pub const fn haddrstart(self) -> crate::common::Reg<regs::Haddrstart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "HADDR REMAP END ADDR"]
    #[inline(always)]
    pub const fn haddrend(self) -> crate::common::Reg<regs::Haddrend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "HADDR Remap Offset"]
    #[inline(always)]
    pub const fn haddroffset(self) -> crate::common::Reg<regs::Haddroffset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "IPED Function Control"]
    #[inline(always)]
    pub const fn ipedctrl(self) -> crate::common::Reg<regs::Ipedctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "IPS Nonsecure Region 0 Start Address"]
    #[inline(always)]
    pub const fn ipsnszstart0(self) -> crate::common::Reg<regs::Ipsnszstart0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 0 End Address"]
    #[inline(always)]
    pub const fn ipsnszend0(self) -> crate::common::Reg<regs::Ipsnszend0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 1 Start Address"]
    #[inline(always)]
    pub const fn ipsnszstart1(self) -> crate::common::Reg<regs::Ipsnszstart1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 1 End Address"]
    #[inline(always)]
    pub const fn ipsnszend1(self) -> crate::common::Reg<regs::Ipsnszend1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 0"]
    #[inline(always)]
    pub const fn ahbbufregionstart0(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionstart0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Receive Buffer Region 0 End Address"]
    #[inline(always)]
    pub const fn ahbbufregionend0(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionend0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 1"]
    #[inline(always)]
    pub const fn ahbbufregionstart1(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionstart1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Receive Buffer Region 1 End Address"]
    #[inline(always)]
    pub const fn ahbbufregionend1(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionend1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 2"]
    #[inline(always)]
    pub const fn ahbbufregionstart2(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionstart2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Receive Buffer Region 2 End Address"]
    #[inline(always)]
    pub const fn ahbbufregionend2(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionend2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 3"]
    #[inline(always)]
    pub const fn ahbbufregionstart3(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionstart3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Receive Buffer Region 3 End Address"]
    #[inline(always)]
    pub const fn ahbbufregionend3(
        self,
    ) -> crate::common::Reg<regs::Ahbbufregionend3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "IPED context control 0"]
    #[inline(always)]
    pub const fn ipedctxctrl0(self) -> crate::common::Reg<regs::Ipedctxctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "IPED context control 1"]
    #[inline(always)]
    pub const fn ipedctxctrl1(self) -> crate::common::Reg<regs::Ipedctxctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "IPED Context0 IV0"]
    #[inline(always)]
    pub const fn ipedctx0iv0(self) -> crate::common::Reg<regs::Ipedctx0iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "IPED Context0 IV1"]
    #[inline(always)]
    pub const fn ipedctx0iv1(self) -> crate::common::Reg<regs::Ipedctx0iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx0start(self) -> crate::common::Reg<regs::Ipedctx0start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx0end(self) -> crate::common::Reg<regs::Ipedctx0end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "IPED Context0 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx0aad0(self) -> crate::common::Reg<regs::Ipedctx0aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "IPED Context0 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx0aad1(self) -> crate::common::Reg<regs::Ipedctx0aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "IPED Context1 IV0"]
    #[inline(always)]
    pub const fn ipedctx1iv0(self) -> crate::common::Reg<regs::Ipedctx1iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "IPED Context1 IV1"]
    #[inline(always)]
    pub const fn ipedctx1iv1(self) -> crate::common::Reg<regs::Ipedctx1iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx1start(self) -> crate::common::Reg<regs::Ipedctx1start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx1end(self) -> crate::common::Reg<regs::Ipedctx1end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "IPED Context1 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx1aad0(self) -> crate::common::Reg<regs::Ipedctx1aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "IPED Context1 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx1aad1(self) -> crate::common::Reg<regs::Ipedctx1aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "IPED Context2 IV0"]
    #[inline(always)]
    pub const fn ipedctx2iv0(self) -> crate::common::Reg<regs::Ipedctx2iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "IPED Context2 IV1"]
    #[inline(always)]
    pub const fn ipedctx2iv1(self) -> crate::common::Reg<regs::Ipedctx2iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx2start(self) -> crate::common::Reg<regs::Ipedctx2start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx2end(self) -> crate::common::Reg<regs::Ipedctx2end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "IPED Context2 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx2aad0(self) -> crate::common::Reg<regs::Ipedctx2aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "IPED Context2 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx2aad1(self) -> crate::common::Reg<regs::Ipedctx2aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "IPED Context3 IV0"]
    #[inline(always)]
    pub const fn ipedctx3iv0(self) -> crate::common::Reg<regs::Ipedctx3iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "IPED Context3 IV1"]
    #[inline(always)]
    pub const fn ipedctx3iv1(self) -> crate::common::Reg<regs::Ipedctx3iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx3start(self) -> crate::common::Reg<regs::Ipedctx3start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx3end(self) -> crate::common::Reg<regs::Ipedctx3end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "IPED Context3 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx3aad0(self) -> crate::common::Reg<regs::Ipedctx3aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "IPED Context3 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx3aad1(self) -> crate::common::Reg<regs::Ipedctx3aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "IPED Context4 IV0"]
    #[inline(always)]
    pub const fn ipedctx4iv0(self) -> crate::common::Reg<regs::Ipedctx4iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "IPED Context4 IV1"]
    #[inline(always)]
    pub const fn ipedctx4iv1(self) -> crate::common::Reg<regs::Ipedctx4iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx4start(self) -> crate::common::Reg<regs::Ipedctx4start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx4end(self) -> crate::common::Reg<regs::Ipedctx4end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "IPED Context4 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx4aad0(self) -> crate::common::Reg<regs::Ipedctx4aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "IPED Context4 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx4aad1(self) -> crate::common::Reg<regs::Ipedctx4aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "IPED Context5 IV0"]
    #[inline(always)]
    pub const fn ipedctx5iv0(self) -> crate::common::Reg<regs::Ipedctx5iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "IPED Context5 IV1"]
    #[inline(always)]
    pub const fn ipedctx5iv1(self) -> crate::common::Reg<regs::Ipedctx5iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx5start(self) -> crate::common::Reg<regs::Ipedctx5start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx5end(self) -> crate::common::Reg<regs::Ipedctx5end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "IPED Context5 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx5aad0(self) -> crate::common::Reg<regs::Ipedctx5aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "IPED Context5 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx5aad1(self) -> crate::common::Reg<regs::Ipedctx5aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "IPED Context6 IV0"]
    #[inline(always)]
    pub const fn ipedctx6iv0(self) -> crate::common::Reg<regs::Ipedctx6iv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "IPED Context6 IV1"]
    #[inline(always)]
    pub const fn ipedctx6iv1(self) -> crate::common::Reg<regs::Ipedctx6iv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "Start Address of Region"]
    #[inline(always)]
    pub const fn ipedctx6start(self) -> crate::common::Reg<regs::Ipedctx6start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "End Address of Region"]
    #[inline(always)]
    pub const fn ipedctx6end(self) -> crate::common::Reg<regs::Ipedctx6end, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "IPED Context6 Additional Authenticated Data0"]
    #[inline(always)]
    pub const fn ipedctx6aad0(self) -> crate::common::Reg<regs::Ipedctx6aad0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "IPED Context6 Additional Authenticated Data1"]
    #[inline(always)]
    pub const fn ipedctx6aad1(self) -> crate::common::Reg<regs::Ipedctx6aad1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
