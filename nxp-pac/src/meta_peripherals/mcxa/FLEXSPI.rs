#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Flexible Serial Peripheral Interface."]
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
    #[doc = "Module Control 0."]
    #[inline(always)]
    pub const fn mcr0(self) -> crate::pac::common::Reg<Mcr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Module Control 1."]
    #[inline(always)]
    pub const fn mcr1(self) -> crate::pac::common::Reg<Mcr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Module Control 2."]
    #[inline(always)]
    pub const fn mcr2(self) -> crate::pac::common::Reg<Mcr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "AHB Bus Control."]
    #[inline(always)]
    pub const fn ahbcr(self) -> crate::pac::common::Reg<Ahbcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::pac::common::Reg<Inten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Interrupt."]
    #[inline(always)]
    pub const fn intr(self) -> crate::pac::common::Reg<Intr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LUT Key."]
    #[inline(always)]
    pub const fn lutkey(self) -> crate::pac::common::Reg<Lutkey, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LUT Control."]
    #[inline(always)]
    pub const fn lutcr(self) -> crate::pac::common::Reg<Lutcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "AHB Receive Buffer 0 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf0cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf0cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "AHB Receive Buffer 1 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf1cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf1cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB Receive Buffer 2 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf2cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf2cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "AHB Receive Buffer 3 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf3cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf3cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "AHB Receive Buffer 4 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf4cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf4cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "AHB Receive Buffer 5 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf5cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf5cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "AHB Receive Buffer 6 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf6cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf6cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "AHB Receive Buffer 7 Control 0."]
    #[inline(always)]
    pub const fn ahbrxbuf7cr0(
        self,
    ) -> crate::pac::common::Reg<Ahbrxbuf7cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "AHB Receive Buffer n Control 1."]
    #[inline(always)]
    pub const fn ahbrxbufcr1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Control 0."]
    #[inline(always)]
    pub const fn flsha1cr0(self) -> crate::pac::common::Reg<Flsha1cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Flash Control 0."]
    #[inline(always)]
    pub const fn flsha2cr0(self) -> crate::pac::common::Reg<Flsha2cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Flash Control 0."]
    #[inline(always)]
    pub const fn flshb1cr0(self) -> crate::pac::common::Reg<Flshb1cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Flash Control 0."]
    #[inline(always)]
    pub const fn flshb2cr0(self) -> crate::pac::common::Reg<Flshb2cr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Flash Control 1."]
    #[inline(always)]
    pub const fn flshcr1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flshcr1, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Control 2."]
    #[inline(always)]
    pub const fn flshcr2(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flshcr2, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _)
        }
    }
    #[doc = "Flash Control 3."]
    #[inline(always)]
    pub const fn flshcr3(self) -> crate::pac::common::Reg<Flshcr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Flash Control 4."]
    #[inline(always)]
    pub const fn flshcr4(self) -> crate::pac::common::Reg<Flshcr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Flash Control 5."]
    #[inline(always)]
    pub const fn flshcr5(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Flash Control 6."]
    #[inline(always)]
    pub const fn flshcr6(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "IP Control 0."]
    #[inline(always)]
    pub const fn ipcr0(self) -> crate::pac::common::Reg<Ipcr0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "IP Control 1."]
    #[inline(always)]
    pub const fn ipcr1(self) -> crate::pac::common::Reg<Ipcr1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "IP Control 2."]
    #[inline(always)]
    pub const fn ipcr2(self) -> crate::pac::common::Reg<Ipcr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "IP Control 3."]
    #[inline(always)]
    pub const fn ipcr3(self) -> crate::pac::common::Reg<u32, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "IP Command."]
    #[inline(always)]
    pub const fn ipcmd(self) -> crate::pac::common::Reg<Ipcmd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Data Learning Pattern."]
    #[inline(always)]
    pub const fn dlpr(self) -> crate::pac::common::Reg<Dlpr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "IP Receive FIFO Control."]
    #[inline(always)]
    pub const fn iprxfcr(self) -> crate::pac::common::Reg<Iprxfcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "IP Transmit FIFO Control."]
    #[inline(always)]
    pub const fn iptxfcr(self) -> crate::pac::common::Reg<Iptxfcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "DLL Control 0."]
    #[inline(always)]
    pub const fn dllcr(self, n: usize) -> crate::pac::common::Reg<Dllcr, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _)
        }
    }
    #[doc = "Misc Control 2."]
    #[inline(always)]
    pub const fn misccr2(self) -> crate::pac::common::Reg<Misccr2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Misc Control 3."]
    #[inline(always)]
    pub const fn misccr3(self) -> crate::pac::common::Reg<Misccr3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Status 0."]
    #[inline(always)]
    pub const fn sts0(self) -> crate::pac::common::Reg<Sts0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Status 1."]
    #[inline(always)]
    pub const fn sts1(self) -> crate::pac::common::Reg<Sts1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Status 2."]
    #[inline(always)]
    pub const fn sts2(self) -> crate::pac::common::Reg<Sts2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "AHB Suspend Status."]
    #[inline(always)]
    pub const fn ahbspndsts(self) -> crate::pac::common::Reg<Ahbspndsts, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "IP Receive FIFO Status."]
    #[inline(always)]
    pub const fn iprxfsts(self) -> crate::pac::common::Reg<Iprxfsts, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "IP Transmit FIFO Status."]
    #[inline(always)]
    pub const fn iptxfsts(self) -> crate::pac::common::Reg<Iptxfsts, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "IP Receive FIFO Data x."]
    #[inline(always)]
    pub const fn rfdr(self, n: usize) -> crate::pac::common::Reg<Rfdr, crate::pac::common::R> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "IP TX FIFO Data x."]
    #[inline(always)]
    pub const fn tfdr(self, n: usize) -> crate::pac::common::Reg<Tfdr, crate::pac::common::W> {
        assert!(n < 32usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize + n * 4usize) as _)
        }
    }
    #[doc = "Lookup Table x."]
    #[inline(always)]
    pub const fn lut(self, n: usize) -> crate::pac::common::Reg<Lut, crate::pac::common::RW> {
        assert!(n < 64usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "HADDR REMAP Start Address."]
    #[inline(always)]
    pub const fn haddrstart(self) -> crate::pac::common::Reg<Haddrstart, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "HADDR REMAP END ADDR."]
    #[inline(always)]
    pub const fn haddrend(self) -> crate::pac::common::Reg<Haddrend, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "HADDR Remap Offset."]
    #[inline(always)]
    pub const fn haddroffset(self) -> crate::pac::common::Reg<Haddroffset, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "IPED Function Control."]
    #[inline(always)]
    pub const fn ipedctrl(self) -> crate::pac::common::Reg<Ipedctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "IPS Nonsecure Region 0 Start Address."]
    #[inline(always)]
    pub const fn ipsnszstart0(
        self,
    ) -> crate::pac::common::Reg<Ipsnszstart0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 0 End Address."]
    #[inline(always)]
    pub const fn ipsnszend0(self) -> crate::pac::common::Reg<Ipsnszend0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 1 Start Address."]
    #[inline(always)]
    pub const fn ipsnszstart1(
        self,
    ) -> crate::pac::common::Reg<Ipsnszstart1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "IPS Nonsecure Region 1 End Address."]
    #[inline(always)]
    pub const fn ipsnszend1(self) -> crate::pac::common::Reg<Ipsnszend1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 0."]
    #[inline(always)]
    pub const fn ahbbufregionstart0(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionstart0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Receive Buffer Region 0 End Address."]
    #[inline(always)]
    pub const fn ahbbufregionend0(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionend0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 1."]
    #[inline(always)]
    pub const fn ahbbufregionstart1(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionstart1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "Receive Buffer Region 1 End Address."]
    #[inline(always)]
    pub const fn ahbbufregionend1(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionend1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 2."]
    #[inline(always)]
    pub const fn ahbbufregionstart2(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionstart2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Receive Buffer Region 2 End Address."]
    #[inline(always)]
    pub const fn ahbbufregionend2(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionend2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "Receive Buffer Start Address of Region 3."]
    #[inline(always)]
    pub const fn ahbbufregionstart3(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionstart3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "Receive Buffer Region 3 End Address."]
    #[inline(always)]
    pub const fn ahbbufregionend3(
        self,
    ) -> crate::pac::common::Reg<Ahbbufregionend3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "IPED context control 0."]
    #[inline(always)]
    pub const fn ipedctxctrl0(
        self,
    ) -> crate::pac::common::Reg<Ipedctxctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "IPED context control 1."]
    #[inline(always)]
    pub const fn ipedctxctrl1(
        self,
    ) -> crate::pac::common::Reg<Ipedctxctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "IPED Context0 IV0."]
    #[inline(always)]
    pub const fn ipedctx0iv0(self) -> crate::pac::common::Reg<Ipedctx0iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "IPED Context0 IV1."]
    #[inline(always)]
    pub const fn ipedctx0iv1(self) -> crate::pac::common::Reg<Ipedctx0iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx0start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx0start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx0end(self) -> crate::pac::common::Reg<Ipedctx0end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "IPED Context1 IV0."]
    #[inline(always)]
    pub const fn ipedctx1iv0(self) -> crate::pac::common::Reg<Ipedctx1iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "IPED Context1 IV1."]
    #[inline(always)]
    pub const fn ipedctx1iv1(self) -> crate::pac::common::Reg<Ipedctx1iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx1start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx1start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx1end(self) -> crate::pac::common::Reg<Ipedctx1end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "IPED Context2 IV0."]
    #[inline(always)]
    pub const fn ipedctx2iv0(self) -> crate::pac::common::Reg<Ipedctx2iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "IPED Context2 IV1."]
    #[inline(always)]
    pub const fn ipedctx2iv1(self) -> crate::pac::common::Reg<Ipedctx2iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx2start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx2start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx2end(self) -> crate::pac::common::Reg<Ipedctx2end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "IPED Context3 IV0."]
    #[inline(always)]
    pub const fn ipedctx3iv0(self) -> crate::pac::common::Reg<Ipedctx3iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "IPED Context3 IV1."]
    #[inline(always)]
    pub const fn ipedctx3iv1(self) -> crate::pac::common::Reg<Ipedctx3iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx3start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx3start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx3end(self) -> crate::pac::common::Reg<Ipedctx3end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "IPED Context4 IV0."]
    #[inline(always)]
    pub const fn ipedctx4iv0(self) -> crate::pac::common::Reg<Ipedctx4iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "IPED Context4 IV1."]
    #[inline(always)]
    pub const fn ipedctx4iv1(self) -> crate::pac::common::Reg<Ipedctx4iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx4start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx4start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx4end(self) -> crate::pac::common::Reg<Ipedctx4end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "IPED Context5 IV0."]
    #[inline(always)]
    pub const fn ipedctx5iv0(self) -> crate::pac::common::Reg<Ipedctx5iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "IPED Context5 IV1."]
    #[inline(always)]
    pub const fn ipedctx5iv1(self) -> crate::pac::common::Reg<Ipedctx5iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx5start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx5start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx5end(self) -> crate::pac::common::Reg<Ipedctx5end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "IPED Context6 IV0."]
    #[inline(always)]
    pub const fn ipedctx6iv0(self) -> crate::pac::common::Reg<Ipedctx6iv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "IPED Context6 IV1."]
    #[inline(always)]
    pub const fn ipedctx6iv1(self) -> crate::pac::common::Reg<Ipedctx6iv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctx6start(
        self,
    ) -> crate::pac::common::Reg<Ipedctx6start, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctx6end(self) -> crate::pac::common::Reg<Ipedctx6end, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
}
#[doc = "Receive Buffer Region 0 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend0(pub u32);
impl Ahbbufregionend0 {
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend0 {
    #[inline(always)]
    fn default() -> Ahbbufregionend0 {
        Ahbbufregionend0(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend0")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend0 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 1 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend1(pub u32);
impl Ahbbufregionend1 {
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend1 {
    #[inline(always)]
    fn default() -> Ahbbufregionend1 {
        Ahbbufregionend1(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend1")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend1 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 2 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend2(pub u32);
impl Ahbbufregionend2 {
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend2 {
    #[inline(always)]
    fn default() -> Ahbbufregionend2 {
        Ahbbufregionend2(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend2")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend2 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Region 3 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend3(pub u32);
impl Ahbbufregionend3 {
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionend3 {
    #[inline(always)]
    fn default() -> Ahbbufregionend3 {
        Ahbbufregionend3(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend3")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend3 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart0(pub u32);
impl Ahbbufregionstart0 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart0 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart0 {
        Ahbbufregionstart0(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart0")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart0 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart1(pub u32);
impl Ahbbufregionstart1 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart1 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart1 {
        Ahbbufregionstart1(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart1")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart1 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart2(pub u32);
impl Ahbbufregionstart2 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart2 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart2 {
        Ahbbufregionstart2(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart2")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart2 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart3(pub u32);
impl Ahbbufregionstart3 {
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Prefetch Sub-Buffer Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ahbbufregionstart3 {
    #[inline(always)]
    fn default() -> Ahbbufregionstart3 {
        Ahbbufregionstart3(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart3")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart3 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "AHB Bus Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbcr(pub u32);
impl Ahbcr {
    #[doc = "AHB Parallel Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn aparen(&self) -> Aparen {
        let val = (self.0 >> 0usize) & 0x01;
        Aparen::from_bits(val as u8)
    }
    #[doc = "AHB Parallel Mode Enable."]
    #[inline(always)]
    pub const fn set_aparen(&mut self, val: Aparen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear AHB Receive Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbrxbuf(&self) -> Clrahbrxbuf {
        let val = (self.0 >> 1usize) & 0x01;
        Clrahbrxbuf::from_bits(val as u8)
    }
    #[doc = "Clear AHB Receive Buffer."]
    #[inline(always)]
    pub const fn set_clrahbrxbuf(&mut self, val: Clrahbrxbuf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear AHB Transmit Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbtxbuf(&self) -> Clrahbtxbuf {
        let val = (self.0 >> 2usize) & 0x01;
        Clrahbtxbuf::from_bits(val as u8)
    }
    #[doc = "Clear AHB Transmit Buffer."]
    #[inline(always)]
    pub const fn set_clrahbtxbuf(&mut self, val: Clrahbtxbuf) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Cacheable Read Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cachableen(&self) -> Cachableen {
        let val = (self.0 >> 3usize) & 0x01;
        Cachableen::from_bits(val as u8)
    }
    #[doc = "Cacheable Read Access Enable."]
    #[inline(always)]
    pub const fn set_cachableen(&mut self, val: Cachableen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Bufferable Write Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bufferableen(&self) -> Bufferableen {
        let val = (self.0 >> 4usize) & 0x01;
        Bufferableen::from_bits(val as u8)
    }
    #[doc = "Bufferable Write Access Enable."]
    #[inline(always)]
    pub const fn set_bufferableen(&mut self, val: Bufferableen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> AhbcrPrefetchen {
        let val = (self.0 >> 5usize) & 0x01;
        AhbcrPrefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: AhbcrPrefetchen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "AHB Read Address Option."]
    #[must_use]
    #[inline(always)]
    pub const fn readaddropt(&self) -> Readaddropt {
        let val = (self.0 >> 6usize) & 0x01;
        Readaddropt::from_bits(val as u8)
    }
    #[doc = "AHB Read Address Option."]
    #[inline(always)]
    pub const fn set_readaddropt(&mut self, val: Readaddropt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Read Resume Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn resumedisable(&self) -> Resumedisable {
        let val = (self.0 >> 7usize) & 0x01;
        Resumedisable::from_bits(val as u8)
    }
    #[doc = "AHB Read Resume Disable."]
    #[inline(always)]
    pub const fn set_resumedisable(&mut self, val: Resumedisable) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AHB Read Size Alignment."]
    #[must_use]
    #[inline(always)]
    pub const fn readszalign(&self) -> Readszalign {
        let val = (self.0 >> 10usize) & 0x01;
        Readszalign::from_bits(val as u8)
    }
    #[doc = "AHB Read Size Alignment."]
    #[inline(always)]
    pub const fn set_readszalign(&mut self, val: Readszalign) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "AHB Boundary Alignment."]
    #[must_use]
    #[inline(always)]
    pub const fn alignment(&self) -> Alignment {
        let val = (self.0 >> 20usize) & 0x03;
        Alignment::from_bits(val as u8)
    }
    #[doc = "AHB Boundary Alignment."]
    #[inline(always)]
    pub const fn set_alignment(&mut self, val: Alignment) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "AHB Memory-Mapped Flash Base Address."]
    #[must_use]
    #[inline(always)]
    pub const fn aflashbase(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Memory-Mapped Flash Base Address."]
    #[inline(always)]
    pub const fn set_aflashbase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ahbcr {
    #[inline(always)]
    fn default() -> Ahbcr {
        Ahbcr(0)
    }
}
impl core::fmt::Debug for Ahbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbcr")
            .field("aparen", &self.aparen())
            .field("clrahbrxbuf", &self.clrahbrxbuf())
            .field("clrahbtxbuf", &self.clrahbtxbuf())
            .field("cachableen", &self.cachableen())
            .field("bufferableen", &self.bufferableen())
            .field("prefetchen", &self.prefetchen())
            .field("readaddropt", &self.readaddropt())
            .field("resumedisable", &self.resumedisable())
            .field("readszalign", &self.readszalign())
            .field("alignment", &self.alignment())
            .field("aflashbase", &self.aflashbase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbcr {{ aparen: {:?}, clrahbrxbuf: {:?}, clrahbtxbuf: {:?}, cachableen: {:?}, bufferableen: {:?}, prefetchen: {:?}, readaddropt: {:?}, resumedisable: {:?}, readszalign: {:?}, alignment: {:?}, aflashbase: {=u8:?} }}",
            self.aparen(),
            self.clrahbrxbuf(),
            self.clrahbtxbuf(),
            self.cachableen(),
            self.bufferableen(),
            self.prefetchen(),
            self.readaddropt(),
            self.resumedisable(),
            self.readszalign(),
            self.alignment(),
            self.aflashbase()
        )
    }
}
#[doc = "AHB Receive Buffer 0 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf0cr0(pub u32);
impl Ahbrxbuf0cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf0cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf0cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf0cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf0cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf0cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf0cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf0cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf0cr0 {
        Ahbrxbuf0cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf0cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf0cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf0cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf0cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 1 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf1cr0(pub u32);
impl Ahbrxbuf1cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf1cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf1cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf1cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf1cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf1cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf1cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf1cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf1cr0 {
        Ahbrxbuf1cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf1cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf1cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 2 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf2cr0(pub u32);
impl Ahbrxbuf2cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf2cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf2cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf2cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf2cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf2cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf2cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf2cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf2cr0 {
        Ahbrxbuf2cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf2cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf2cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 3 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf3cr0(pub u32);
impl Ahbrxbuf3cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf3cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf3cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf3cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf3cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf3cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf3cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf3cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf3cr0 {
        Ahbrxbuf3cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf3cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf3cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf3cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf3cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 4 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf4cr0(pub u32);
impl Ahbrxbuf4cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf4cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf4cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf4cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf4cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf4cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf4cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf4cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf4cr0 {
        Ahbrxbuf4cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf4cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf4cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf4cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf4cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 5 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf5cr0(pub u32);
impl Ahbrxbuf5cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf5cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf5cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf5cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf5cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf5cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf5cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf5cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf5cr0 {
        Ahbrxbuf5cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf5cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf5cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf5cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf5cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 6 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf6cr0(pub u32);
impl Ahbrxbuf6cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf6cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf6cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf6cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf6cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf6cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf6cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf6cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf6cr0 {
        Ahbrxbuf6cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf6cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf6cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf6cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf6cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Receive Buffer 7 Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbuf7cr0(pub u32);
impl Ahbrxbuf7cr0 {
    #[doc = "AHB Receive Buffer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn bufsz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Receive Buffer Size."]
    #[inline(always)]
    pub const fn set_bufsz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Controller ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mstrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "AHB Controller Read Priority."]
    #[must_use]
    #[inline(always)]
    pub const fn priority(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Controller Read Priority."]
    #[inline(always)]
    pub const fn set_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn regionen(&self) -> Ahbrxbuf7cr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbuf7cr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbuf7cr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbuf7cr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbuf7cr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbuf7cr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbuf7cr0 {
    #[inline(always)]
    fn default() -> Ahbrxbuf7cr0 {
        Ahbrxbuf7cr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbuf7cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbuf7cr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbuf7cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbuf7cr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
            self.bufsz(),
            self.mstrid(),
            self.priority(),
            self.regionen(),
            self.prefetchen()
        )
    }
}
#[doc = "AHB Suspend Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbspndsts(pub u32);
impl Ahbspndsts {
    #[doc = "Active AHB Read Prefetch Suspended."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 0usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "Active AHB Read Prefetch Suspended."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence."]
    #[must_use]
    #[inline(always)]
    pub const fn bufid(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Receive Buffer ID for Suspended Command Sequence."]
    #[inline(always)]
    pub const fn set_bufid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "Data Left."]
    #[must_use]
    #[inline(always)]
    pub const fn datlft(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Data Left."]
    #[inline(always)]
    pub const fn set_datlft(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ahbspndsts {
    #[inline(always)]
    fn default() -> Ahbspndsts {
        Ahbspndsts(0)
    }
}
impl core::fmt::Debug for Ahbspndsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbspndsts")
            .field("active", &self.active())
            .field("bufid", &self.bufid())
            .field("datlft", &self.datlft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbspndsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbspndsts {{ active: {:?}, bufid: {=u8:?}, datlft: {=u16:?} }}",
            self.active(),
            self.bufid(),
            self.datlft()
        )
    }
}
#[doc = "DLL Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dllcr(pub u32);
impl Dllcr {
    #[doc = "DLL Calibration Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dllen(&self) -> Dllen {
        let val = (self.0 >> 0usize) & 0x01;
        Dllen::from_bits(val as u8)
    }
    #[doc = "DLL Calibration Enable."]
    #[inline(always)]
    pub const fn set_dllen(&mut self, val: Dllen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dllreset(&self) -> Dllreset {
        let val = (self.0 >> 1usize) & 0x01;
        Dllreset::from_bits(val as u8)
    }
    #[doc = "DLL reset."]
    #[inline(always)]
    pub const fn set_dllreset(&mut self, val: Dllreset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Force update."]
    #[must_use]
    #[inline(always)]
    pub const fn forceupdate(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force update."]
    #[inline(always)]
    pub const fn set_forceupdate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Target Delay Line."]
    #[must_use]
    #[inline(always)]
    pub const fn slvdlytarget(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Target Delay Line."]
    #[inline(always)]
    pub const fn set_slvdlytarget(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Gate DLL Update."]
    #[must_use]
    #[inline(always)]
    pub const fn gateupdate(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Gate DLL Update."]
    #[inline(always)]
    pub const fn set_gateupdate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Target Clock Delay Line Override Value Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ovrden(&self) -> Ovrden {
        let val = (self.0 >> 8usize) & 0x01;
        Ovrden::from_bits(val as u8)
    }
    #[doc = "Target Clock Delay Line Override Value Enable."]
    #[inline(always)]
    pub const fn set_ovrden(&mut self, val: Ovrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Target Clock Delay Line Override Value."]
    #[must_use]
    #[inline(always)]
    pub const fn ovrdval(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Target Clock Delay Line Override Value."]
    #[inline(always)]
    pub const fn set_ovrdval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u32) & 0x3f) << 9usize);
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[must_use]
    #[inline(always)]
    pub const fn refphasegap(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "Reference Clock Delay Line Phase Adjust Gap. REFPHASEGAP setting of 2h is recommended if DLLEN is set."]
    #[inline(always)]
    pub const fn set_refphasegap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[doc = "Reference Clock Delay Line Start Phase."]
    #[must_use]
    #[inline(always)]
    pub const fn refphasestart(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Reference Clock Delay Line Start Phase."]
    #[inline(always)]
    pub const fn set_refphasestart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Target Delay Line Update Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn slvupdateint(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Target Delay Line Update Interval."]
    #[inline(always)]
    pub const fn set_slvupdateint(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "DLL Control Loop Update Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn refupdateint(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL Control Loop Update Interval."]
    #[inline(always)]
    pub const fn set_refupdateint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Dllcr {
    #[inline(always)]
    fn default() -> Dllcr {
        Dllcr(0)
    }
}
impl core::fmt::Debug for Dllcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dllcr")
            .field("dllen", &self.dllen())
            .field("dllreset", &self.dllreset())
            .field("forceupdate", &self.forceupdate())
            .field("slvdlytarget", &self.slvdlytarget())
            .field("gateupdate", &self.gateupdate())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .field("refphasegap", &self.refphasegap())
            .field("refphasestart", &self.refphasestart())
            .field("slvupdateint", &self.slvupdateint())
            .field("refupdateint", &self.refupdateint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dllcr {{ dllen: {:?}, dllreset: {:?}, forceupdate: {=bool:?}, slvdlytarget: {=u8:?}, gateupdate: {=bool:?}, ovrden: {:?}, ovrdval: {=u8:?}, refphasegap: {=u8:?}, refphasestart: {=u8:?}, slvupdateint: {=u8:?}, refupdateint: {=u8:?} }}",
            self.dllen(),
            self.dllreset(),
            self.forceupdate(),
            self.slvdlytarget(),
            self.gateupdate(),
            self.ovrden(),
            self.ovrdval(),
            self.refphasegap(),
            self.refphasestart(),
            self.slvupdateint(),
            self.refupdateint()
        )
    }
}
#[doc = "Data Learning Pattern."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlpr(pub u32);
impl Dlpr {
    #[doc = "Data Learning Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn dlp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Learning Pattern."]
    #[inline(always)]
    pub const fn set_dlp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dlpr {
    #[inline(always)]
    fn default() -> Dlpr {
        Dlpr(0)
    }
}
impl core::fmt::Debug for Dlpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dlpr").field("dlp", &self.dlp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlpr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dlpr {{ dlp: {=u32:?} }}", self.dlp())
    }
}
#[doc = "Flash Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha1cr0(pub u32);
impl Flsha1cr0 {
    #[doc = "Flash Size in KB."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control."]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control."]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flsha1cr0 {
    #[inline(always)]
    fn default() -> Flsha1cr0 {
        Flsha1cr0(0)
    }
}
impl core::fmt::Debug for Flsha1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha1cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?} }}",
            self.flshsz(),
            self.addrshift()
        )
    }
}
#[doc = "Flash Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flsha2cr0(pub u32);
impl Flsha2cr0 {
    #[doc = "Flash Size in KB."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control."]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control."]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flsha2cr0 {
    #[inline(always)]
    fn default() -> Flsha2cr0 {
        Flsha2cr0(0)
    }
}
impl core::fmt::Debug for Flsha2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flsha2cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?} }}",
            self.flshsz(),
            self.addrshift()
        )
    }
}
#[doc = "Flash Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb1cr0(pub u32);
impl Flshb1cr0 {
    #[doc = "Flash Size in KB."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control."]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control."]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flshb1cr0 {
    #[inline(always)]
    fn default() -> Flshb1cr0 {
        Flshb1cr0(0)
    }
}
impl core::fmt::Debug for Flshb1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb1cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?} }}",
            self.flshsz(),
            self.addrshift()
        )
    }
}
#[doc = "Flash Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshb2cr0(pub u32);
impl Flshb2cr0 {
    #[doc = "Flash Size in KB."]
    #[must_use]
    #[inline(always)]
    pub const fn flshsz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Flash Size in KB."]
    #[inline(always)]
    pub const fn set_flshsz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 0usize)) | (((val as u32) & 0x007f_ffff) << 0usize);
    }
    #[doc = "AHB Address Shift Function control."]
    #[must_use]
    #[inline(always)]
    pub const fn addrshift(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Address Shift Function control."]
    #[inline(always)]
    pub const fn set_addrshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Flshb2cr0 {
    #[inline(always)]
    fn default() -> Flshb2cr0 {
        Flshb2cr0(0)
    }
}
impl core::fmt::Debug for Flshb2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshb2cr0")
            .field("flshsz", &self.flshsz())
            .field("addrshift", &self.addrshift())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?} }}",
            self.flshsz(),
            self.addrshift()
        )
    }
}
#[doc = "Flash Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr1(pub u32);
impl Flshcr1 {
    #[doc = "Serial Flash CS Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn tcss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Setup Time."]
    #[inline(always)]
    pub const fn set_tcss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Serial Flash CS Hold Time."]
    #[must_use]
    #[inline(always)]
    pub const fn tcsh(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "Serial Flash CS Hold Time."]
    #[inline(always)]
    pub const fn set_tcsh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "Word-Addressable."]
    #[must_use]
    #[inline(always)]
    pub const fn wa(&self) -> Wa {
        let val = (self.0 >> 10usize) & 0x01;
        Wa::from_bits(val as u8)
    }
    #[doc = "Word-Addressable."]
    #[inline(always)]
    pub const fn set_wa(&mut self, val: Wa) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Column Address Size."]
    #[must_use]
    #[inline(always)]
    pub const fn cas(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Column Address Size."]
    #[inline(always)]
    pub const fn set_cas(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Chip Select Interval Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn csintervalunit(&self) -> Csintervalunit {
        let val = (self.0 >> 15usize) & 0x01;
        Csintervalunit::from_bits(val as u8)
    }
    #[doc = "Chip Select Interval Unit."]
    #[inline(always)]
    pub const fn set_csintervalunit(&mut self, val: Csintervalunit) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Chip Select Interval."]
    #[must_use]
    #[inline(always)]
    pub const fn csinterval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Chip Select Interval."]
    #[inline(always)]
    pub const fn set_csinterval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Flshcr1 {
    #[inline(always)]
    fn default() -> Flshcr1 {
        Flshcr1(0)
    }
}
impl core::fmt::Debug for Flshcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr1")
            .field("tcss", &self.tcss())
            .field("tcsh", &self.tcsh())
            .field("wa", &self.wa())
            .field("cas", &self.cas())
            .field("csintervalunit", &self.csintervalunit())
            .field("csinterval", &self.csinterval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr1 {{ tcss: {=u8:?}, tcsh: {=u8:?}, wa: {:?}, cas: {=u8:?}, csintervalunit: {:?}, csinterval: {=u16:?} }}",
            self.tcss(),
            self.tcsh(),
            self.wa(),
            self.cas(),
            self.csintervalunit(),
            self.csinterval()
        )
    }
}
#[doc = "Flash Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr2(pub u32);
impl Flshcr2 {
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Read-Triggered Command in LUT."]
    #[inline(always)]
    pub const fn set_ardseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command."]
    #[must_use]
    #[inline(always)]
    pub const fn ardseqnum(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Read-Triggered Command."]
    #[inline(always)]
    pub const fn set_ardseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command."]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index for AHB Write-Triggered Command."]
    #[inline(always)]
    pub const fn set_awrseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command."]
    #[must_use]
    #[inline(always)]
    pub const fn awrseqnum(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for AHB Write-Triggered Command."]
    #[inline(always)]
    pub const fn set_awrseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "AHB Write Wait."]
    #[must_use]
    #[inline(always)]
    pub const fn awrwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "AHB Write Wait."]
    #[inline(always)]
    pub const fn set_awrwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "AWRWAIT Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn awrwaitunit(&self) -> Awrwaitunit {
        let val = (self.0 >> 28usize) & 0x07;
        Awrwaitunit::from_bits(val as u8)
    }
    #[doc = "AWRWAIT Unit."]
    #[inline(always)]
    pub const fn set_awrwaitunit(&mut self, val: Awrwaitunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Clear Instruction Pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn clrinstrptr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Instruction Pointer."]
    #[inline(always)]
    pub const fn set_clrinstrptr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Flshcr2 {
    #[inline(always)]
    fn default() -> Flshcr2 {
        Flshcr2(0)
    }
}
impl core::fmt::Debug for Flshcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr2")
            .field("ardseqid", &self.ardseqid())
            .field("ardseqnum", &self.ardseqnum())
            .field("awrseqid", &self.awrseqid())
            .field("awrseqnum", &self.awrseqnum())
            .field("awrwait", &self.awrwait())
            .field("awrwaitunit", &self.awrwaitunit())
            .field("clrinstrptr", &self.clrinstrptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr2 {{ ardseqid: {=u8:?}, ardseqnum: {=u8:?}, awrseqid: {=u8:?}, awrseqnum: {=u8:?}, awrwait: {=u16:?}, awrwaitunit: {:?}, clrinstrptr: {=bool:?} }}",
            self.ardseqid(),
            self.ardseqnum(),
            self.awrseqid(),
            self.awrseqnum(),
            self.awrwait(),
            self.awrwaitunit(),
            self.clrinstrptr()
        )
    }
}
#[doc = "Flash Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr3(pub u32);
impl Flshcr3 {
    #[doc = "Data pins (SIO) default output level in IDLE state."]
    #[must_use]
    #[inline(always)]
    pub const fn siodoidle(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data pins (SIO) default output level in IDLE state."]
    #[inline(always)]
    pub const fn set_siodoidle(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data pins (SIO) default output level in NON-IDLE state (chip Select is asserted)."]
    #[must_use]
    #[inline(always)]
    pub const fn siodononidle(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data pins (SIO) default output level in NON-IDLE state (chip Select is asserted)."]
    #[inline(always)]
    pub const fn set_siodononidle(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data pins (SIO) default output enabled in IDLE state."]
    #[must_use]
    #[inline(always)]
    pub const fn siooeidle(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Data pins (SIO) default output enabled in IDLE state."]
    #[inline(always)]
    pub const fn set_siooeidle(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Data pins (SIO) default output enabled in NON-IDLE state (chip Select is asserted)."]
    #[must_use]
    #[inline(always)]
    pub const fn siooenonidle(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Data pins (SIO) default output enabled in NON-IDLE state (chip Select is asserted)."]
    #[inline(always)]
    pub const fn set_siooenonidle(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Flshcr3 {
    #[inline(always)]
    fn default() -> Flshcr3 {
        Flshcr3(0)
    }
}
impl core::fmt::Debug for Flshcr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr3")
            .field("siodoidle", &self.siodoidle())
            .field("siodononidle", &self.siodononidle())
            .field("siooeidle", &self.siooeidle())
            .field("siooenonidle", &self.siooenonidle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr3 {{ siodoidle: {=u8:?}, siodononidle: {=u8:?}, siooeidle: {=u8:?}, siooenonidle: {=u8:?} }}",
            self.siodoidle(),
            self.siodononidle(),
            self.siooeidle(),
            self.siooenonidle()
        )
    }
}
#[doc = "Flash Control 4."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flshcr4(pub u32);
impl Flshcr4 {
    #[doc = "Write Mask Option 1."]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Mask Option 1."]
    #[inline(always)]
    pub const fn set_wmopt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write Mask Enable for Port A."]
    #[must_use]
    #[inline(always)]
    pub const fn wmena(&self) -> Wmena {
        let val = (self.0 >> 2usize) & 0x01;
        Wmena::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port A."]
    #[inline(always)]
    pub const fn set_wmena(&mut self, val: Wmena) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Write Mask Enable for Port B."]
    #[must_use]
    #[inline(always)]
    pub const fn wmenb(&self) -> Wmenb {
        let val = (self.0 >> 3usize) & 0x01;
        Wmenb::from_bits(val as u8)
    }
    #[doc = "Write Mask Enable for Port B."]
    #[inline(always)]
    pub const fn set_wmenb(&mut self, val: Wmenb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SCLK Reset Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckrstdisabled(&self) -> Sckrstdisabled {
        let val = (self.0 >> 4usize) & 0x01;
        Sckrstdisabled::from_bits(val as u8)
    }
    #[doc = "SCLK Reset Disable."]
    #[inline(always)]
    pub const fn set_sckrstdisabled(&mut self, val: Sckrstdisabled) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DQS POS Capture."]
    #[must_use]
    #[inline(always)]
    pub const fn dqsposcap(&self) -> Dqsposcap {
        let val = (self.0 >> 5usize) & 0x01;
        Dqsposcap::from_bits(val as u8)
    }
    #[doc = "DQS POS Capture."]
    #[inline(always)]
    pub const fn set_dqsposcap(&mut self, val: Dqsposcap) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Write Mask Option 1B."]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt1b(&self) -> Wmopt1b {
        let val = (self.0 >> 6usize) & 0x01;
        Wmopt1b::from_bits(val as u8)
    }
    #[doc = "Write Mask Option 1B."]
    #[inline(always)]
    pub const fn set_wmopt1b(&mut self, val: Wmopt1b) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Write Mask Option 2B."]
    #[must_use]
    #[inline(always)]
    pub const fn wmopt2b(&self) -> Wmopt2b {
        let val = (self.0 >> 7usize) & 0x01;
        Wmopt2b::from_bits(val as u8)
    }
    #[doc = "Write Mask Option 2B."]
    #[inline(always)]
    pub const fn set_wmopt2b(&mut self, val: Wmopt2b) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Write Mask Option Differential."]
    #[must_use]
    #[inline(always)]
    pub const fn wmoptdiff(&self) -> Wmoptdiff {
        let val = (self.0 >> 8usize) & 0x01;
        Wmoptdiff::from_bits(val as u8)
    }
    #[doc = "Write Mask Option Differential."]
    #[inline(always)]
    pub const fn set_wmoptdiff(&mut self, val: Wmoptdiff) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Flshcr4 {
    #[inline(always)]
    fn default() -> Flshcr4 {
        Flshcr4(0)
    }
}
impl core::fmt::Debug for Flshcr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flshcr4")
            .field("wmopt1", &self.wmopt1())
            .field("wmena", &self.wmena())
            .field("wmenb", &self.wmenb())
            .field("sckrstdisabled", &self.sckrstdisabled())
            .field("dqsposcap", &self.dqsposcap())
            .field("wmopt1b", &self.wmopt1b())
            .field("wmopt2b", &self.wmopt2b())
            .field("wmoptdiff", &self.wmoptdiff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr4 {{ wmopt1: {=bool:?}, wmena: {:?}, wmenb: {:?}, sckrstdisabled: {:?}, dqsposcap: {:?}, wmopt1b: {:?}, wmopt2b: {:?}, wmoptdiff: {:?} }}",
            self.wmopt1(),
            self.wmena(),
            self.wmenb(),
            self.sckrstdisabled(),
            self.dqsposcap(),
            self.wmopt1b(),
            self.wmopt2b(),
            self.wmoptdiff()
        )
    }
}
#[doc = "HADDR REMAP END ADDR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddrend(pub u32);
impl Haddrend {
    #[doc = "End Address of HADDR Remap Range."]
    #[must_use]
    #[inline(always)]
    pub const fn endstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of HADDR Remap Range."]
    #[inline(always)]
    pub const fn set_endstart(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddrend {
    #[inline(always)]
    fn default() -> Haddrend {
        Haddrend(0)
    }
}
impl core::fmt::Debug for Haddrend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddrend")
            .field("endstart", &self.endstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddrend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Haddrend {{ endstart: {=u32:?} }}", self.endstart())
    }
}
#[doc = "HADDR Remap Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddroffset(pub u32);
impl Haddroffset {
    #[doc = "HADDR Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn addroffset(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR Offset."]
    #[inline(always)]
    pub const fn set_addroffset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddroffset {
    #[inline(always)]
    fn default() -> Haddroffset {
        Haddroffset(0)
    }
}
impl core::fmt::Debug for Haddroffset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddroffset")
            .field("addroffset", &self.addroffset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddroffset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Haddroffset {{ addroffset: {=u32:?} }}",
            self.addroffset()
        )
    }
}
#[doc = "HADDR REMAP Start Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Haddrstart(pub u32);
impl Haddrstart {
    #[doc = "AHB Bus Address Remap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn remapen(&self) -> Remapen {
        let val = (self.0 >> 0usize) & 0x01;
        Remapen::from_bits(val as u8)
    }
    #[doc = "AHB Bus Address Remap Enable."]
    #[inline(always)]
    pub const fn set_remapen(&mut self, val: Remapen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB or IP Bus Address Swap Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn swapen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB or IP Bus Address Swap Enable."]
    #[inline(always)]
    pub const fn set_swapen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "HADDR Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn addrstart(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "HADDR Start Address."]
    #[inline(always)]
    pub const fn set_addrstart(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Haddrstart {
    #[inline(always)]
    fn default() -> Haddrstart {
        Haddrstart(0)
    }
}
impl core::fmt::Debug for Haddrstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Haddrstart")
            .field("remapen", &self.remapen())
            .field("swapen", &self.swapen())
            .field("addrstart", &self.addrstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddrstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Haddrstart {{ remapen: {:?}, swapen: {=bool:?}, addrstart: {=u32:?} }}",
            self.remapen(),
            self.swapen(),
            self.addrstart()
        )
    }
}
#[doc = "Interrupt Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddoneen(&self) -> Ipcmddoneen {
        let val = (self.0 >> 0usize) & 0x01;
        Ipcmddoneen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ipcmddoneen(&mut self, val: Ipcmddoneen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdgeen(&self) -> Ipcmdgeen {
        let val = (self.0 >> 1usize) & 0x01;
        Ipcmdgeen::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ipcmdgeen(&mut self, val: Ipcmdgeen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdgeen(&self) -> Ahbcmdgeen {
        let val = (self.0 >> 2usize) & 0x01;
        Ahbcmdgeen::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbcmdgeen(&mut self, val: Ahbcmdgeen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderren(&self) -> Ipcmderren {
        let val = (self.0 >> 3usize) & 0x01;
        Ipcmderren::from_bits(val as u8)
    }
    #[doc = "IP-Triggered Command Sequences Error Detected Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ipcmderren(&mut self, val: Ipcmderren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderren(&self) -> Ahbcmderren {
        let val = (self.0 >> 4usize) & 0x01;
        Ahbcmderren::from_bits(val as u8)
    }
    #[doc = "AHB-Triggered Command Sequences Error Detected Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbcmderren(&mut self, val: Ahbcmderren) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwaen(&self) -> Iprxwaen {
        let val = (self.0 >> 5usize) & 0x01;
        Iprxwaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Watermark Available Interrupt Enable."]
    #[inline(always)]
    pub const fn set_iprxwaen(&mut self, val: Iprxwaen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iptxween(&self) -> Iptxween {
        let val = (self.0 >> 6usize) & 0x01;
        Iptxween::from_bits(val as u8)
    }
    #[doc = "IP Transmit FIFO Watermark Empty Interrupt Enable."]
    #[inline(always)]
    pub const fn set_iptxween(&mut self, val: Iptxween) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning Failed Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfailen(&self) -> Datalearnfailen {
        let val = (self.0 >> 7usize) & 0x01;
        Datalearnfailen::from_bits(val as u8)
    }
    #[doc = "Data Learning Failed Interrupt Enable."]
    #[inline(always)]
    pub const fn set_datalearnfailen(&mut self, val: Datalearnfailen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrden(&self) -> Sckstopbyrden {
        let val = (self.0 >> 8usize) & 0x01;
        Sckstopbyrden::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Read Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sckstopbyrden(&mut self, val: Sckstopbyrden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywren(&self) -> Sckstopbywren {
        let val = (self.0 >> 9usize) & 0x01;
        Sckstopbywren::from_bits(val as u8)
    }
    #[doc = "SCLK Stopped By Write Interrupt Enable."]
    #[inline(always)]
    pub const fn set_sckstopbywren(&mut self, val: Sckstopbywren) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeouten(&self) -> Ahbbustimeouten {
        let val = (self.0 >> 10usize) & 0x01;
        Ahbbustimeouten::from_bits(val as u8)
    }
    #[doc = "AHB Bus Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbbustimeouten(&mut self, val: Ahbbustimeouten) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence execution Timeout Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeouten(&self) -> Seqtimeouten {
        let val = (self.0 >> 11usize) & 0x01;
        Seqtimeouten::from_bits(val as u8)
    }
    #[doc = "Sequence execution Timeout Interrupt Enable."]
    #[inline(always)]
    pub const fn set_seqtimeouten(&mut self, val: Seqtimeouten) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "IP Command Security Violation Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdsecurevioen(&self) -> Ipcmdsecurevioen {
        let val = (self.0 >> 16usize) & 0x01;
        Ipcmdsecurevioen::from_bits(val as u8)
    }
    #[doc = "IP Command Security Violation Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ipcmdsecurevioen(&mut self, val: Ipcmdsecurevioen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("ipcmddoneen", &self.ipcmddoneen())
            .field("ipcmdgeen", &self.ipcmdgeen())
            .field("ahbcmdgeen", &self.ahbcmdgeen())
            .field("ipcmderren", &self.ipcmderren())
            .field("ahbcmderren", &self.ahbcmderren())
            .field("iprxwaen", &self.iprxwaen())
            .field("iptxween", &self.iptxween())
            .field("datalearnfailen", &self.datalearnfailen())
            .field("sckstopbyrden", &self.sckstopbyrden())
            .field("sckstopbywren", &self.sckstopbywren())
            .field("ahbbustimeouten", &self.ahbbustimeouten())
            .field("seqtimeouten", &self.seqtimeouten())
            .field("ipcmdsecurevioen", &self.ipcmdsecurevioen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ipcmddoneen: {:?}, ipcmdgeen: {:?}, ahbcmdgeen: {:?}, ipcmderren: {:?}, ahbcmderren: {:?}, iprxwaen: {:?}, iptxween: {:?}, datalearnfailen: {:?}, sckstopbyrden: {:?}, sckstopbywren: {:?}, ahbbustimeouten: {:?}, seqtimeouten: {:?}, ipcmdsecurevioen: {:?} }}",
            self.ipcmddoneen(),
            self.ipcmdgeen(),
            self.ahbcmdgeen(),
            self.ipcmderren(),
            self.ahbcmderren(),
            self.iprxwaen(),
            self.iptxween(),
            self.datalearnfailen(),
            self.sckstopbyrden(),
            self.sckstopbywren(),
            self.ahbbustimeouten(),
            self.seqtimeouten(),
            self.ipcmdsecurevioen()
        )
    }
}
#[doc = "Interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "IP-Triggered Command Sequences Execution Finished."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmddone(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Execution Finished."]
    #[inline(always)]
    pub const fn set_ipcmddone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdge(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Grant Timeout."]
    #[inline(always)]
    pub const fn set_ipcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmdge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Grant Timeout."]
    #[inline(always)]
    pub const fn set_ahbcmdge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "IP-Triggered Command Sequences Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "IP-Triggered Command Sequences Error."]
    #[inline(always)]
    pub const fn set_ipcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB-Triggered Command Sequences Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "AHB-Triggered Command Sequences Error."]
    #[inline(always)]
    pub const fn set_ahbcmderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IP Receive FIFO Watermark Available."]
    #[must_use]
    #[inline(always)]
    pub const fn iprxwa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IP Receive FIFO Watermark Available."]
    #[inline(always)]
    pub const fn set_iprxwa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "IP Transmit FIFO Watermark Empty."]
    #[must_use]
    #[inline(always)]
    pub const fn iptxwe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "IP Transmit FIFO Watermark Empty."]
    #[inline(always)]
    pub const fn set_iptxwe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data Learning Failed."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnfail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning Failed."]
    #[inline(always)]
    pub const fn set_datalearnfail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbyrd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Full Receive FIFO."]
    #[inline(always)]
    pub const fn set_sckstopbyrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn sckstopbywr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Stopped Due To Empty Transmit FIFO."]
    #[inline(always)]
    pub const fn set_sckstopbywr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "AHB Bus Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbustimeout(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus Timeout."]
    #[inline(always)]
    pub const fn set_ahbbustimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Sequence Execution Timeout."]
    #[must_use]
    #[inline(always)]
    pub const fn seqtimeout(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Execution Timeout."]
    #[inline(always)]
    pub const fn set_seqtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "IP Command Security Violation."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmdsecurevio(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "IP Command Security Violation."]
    #[inline(always)]
    pub const fn set_ipcmdsecurevio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("ipcmddone", &self.ipcmddone())
            .field("ipcmdge", &self.ipcmdge())
            .field("ahbcmdge", &self.ahbcmdge())
            .field("ipcmderr", &self.ipcmderr())
            .field("ahbcmderr", &self.ahbcmderr())
            .field("iprxwa", &self.iprxwa())
            .field("iptxwe", &self.iptxwe())
            .field("datalearnfail", &self.datalearnfail())
            .field("sckstopbyrd", &self.sckstopbyrd())
            .field("sckstopbywr", &self.sckstopbywr())
            .field("ahbbustimeout", &self.ahbbustimeout())
            .field("seqtimeout", &self.seqtimeout())
            .field("ipcmdsecurevio", &self.ipcmdsecurevio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intr {{ ipcmddone: {=bool:?}, ipcmdge: {=bool:?}, ahbcmdge: {=bool:?}, ipcmderr: {=bool:?}, ahbcmderr: {=bool:?}, iprxwa: {=bool:?}, iptxwe: {=bool:?}, datalearnfail: {=bool:?}, sckstopbyrd: {=bool:?}, sckstopbywr: {=bool:?}, ahbbustimeout: {=bool:?}, seqtimeout: {=bool:?}, ipcmdsecurevio: {=bool:?} }}",
            self.ipcmddone(),
            self.ipcmdge(),
            self.ahbcmdge(),
            self.ipcmderr(),
            self.ahbcmderr(),
            self.iprxwa(),
            self.iptxwe(),
            self.datalearnfail(),
            self.sckstopbyrd(),
            self.sckstopbywr(),
            self.ahbbustimeout(),
            self.seqtimeout(),
            self.ipcmdsecurevio()
        )
    }
}
#[doc = "IP Command."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcmd(pub u32);
impl Ipcmd {
    #[doc = "Command Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn trg(&self) -> Trg {
        let val = (self.0 >> 0usize) & 0x01;
        Trg::from_bits(val as u8)
    }
    #[doc = "Command Trigger."]
    #[inline(always)]
    pub const fn set_trg(&mut self, val: Trg) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ipcmd {
    #[inline(always)]
    fn default() -> Ipcmd {
        Ipcmd(0)
    }
}
impl core::fmt::Debug for Ipcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcmd").field("trg", &self.trg()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipcmd {{ trg: {:?} }}", self.trg())
    }
}
#[doc = "IP Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr0(pub u32);
impl Ipcr0 {
    #[doc = "Serial Flash Address."]
    #[must_use]
    #[inline(always)]
    pub const fn sfar(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Serial Flash Address."]
    #[inline(always)]
    pub const fn set_sfar(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipcr0 {
    #[inline(always)]
    fn default() -> Ipcr0 {
        Ipcr0(0)
    }
}
impl core::fmt::Debug for Ipcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr0").field("sfar", &self.sfar()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipcr0 {{ sfar: {=u32:?} }}", self.sfar())
    }
}
#[doc = "IP Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr1(pub u32);
impl Ipcr1 {
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn idatsz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Flash Read/Program Data Size (in bytes) for IP command."]
    #[inline(always)]
    pub const fn set_idatsz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Sequence Index in LUT for IP command."]
    #[inline(always)]
    pub const fn set_iseqid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[must_use]
    #[inline(always)]
    pub const fn iseqnum(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Sequence Number for IP command: ISEQNUM+1."]
    #[inline(always)]
    pub const fn set_iseqnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Parallel Mode Enable for IP Commands."]
    #[must_use]
    #[inline(always)]
    pub const fn iparen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Parallel Mode Enable for IP Commands."]
    #[inline(always)]
    pub const fn set_iparen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ipcr1 {
    #[inline(always)]
    fn default() -> Ipcr1 {
        Ipcr1(0)
    }
}
impl core::fmt::Debug for Ipcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr1")
            .field("idatsz", &self.idatsz())
            .field("iseqid", &self.iseqid())
            .field("iseqnum", &self.iseqnum())
            .field("iparen", &self.iparen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr1 {{ idatsz: {=u16:?}, iseqid: {=u8:?}, iseqnum: {=u8:?}, iparen: {=bool:?} }}",
            self.idatsz(),
            self.iseqid(),
            self.iseqnum(),
            self.iparen()
        )
    }
}
#[doc = "IP Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcr2(pub u32);
impl Ipcr2 {
    #[doc = "IP Command Blocking AHB Command Request Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkahbreq(&self) -> Ipblkahbreq {
        let val = (self.0 >> 0usize) & 0x01;
        Ipblkahbreq::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking AHB Command Request Enable."]
    #[inline(always)]
    pub const fn set_ipblkahbreq(&mut self, val: Ipblkahbreq) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP Command Blocking AHB Command Acknowledgment Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkahback(&self) -> Ipblkahback {
        let val = (self.0 >> 1usize) & 0x01;
        Ipblkahback::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking AHB Command Acknowledgment Enable."]
    #[inline(always)]
    pub const fn set_ipblkahback(&mut self, val: Ipblkahback) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Command Blocking All AHB Command Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipblkallahb(&self) -> Ipblkallahb {
        let val = (self.0 >> 2usize) & 0x01;
        Ipblkallahb::from_bits(val as u8)
    }
    #[doc = "IP Command Blocking All AHB Command Enable."]
    #[inline(always)]
    pub const fn set_ipblkallahb(&mut self, val: Ipblkallahb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ipcr2 {
    #[inline(always)]
    fn default() -> Ipcr2 {
        Ipcr2(0)
    }
}
impl core::fmt::Debug for Ipcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcr2")
            .field("ipblkahbreq", &self.ipblkahbreq())
            .field("ipblkahback", &self.ipblkahback())
            .field("ipblkallahb", &self.ipblkallahb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipcr2 {{ ipblkahbreq: {:?}, ipblkahback: {:?}, ipblkallahb: {:?} }}",
            self.ipblkahbreq(),
            self.ipblkahback(),
            self.ipblkallahb()
        )
    }
}
#[doc = "IPED Function Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctrl(pub u32);
impl Ipedctrl {
    #[doc = "IPED Mode Select."]
    #[must_use]
    #[inline(always)]
    pub const fn config(&self) -> Config {
        let val = (self.0 >> 0usize) & 0x01;
        Config::from_bits(val as u8)
    }
    #[doc = "IPED Mode Select."]
    #[inline(always)]
    pub const fn set_config(&mut self, val: Config) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IPED Encryption and Decryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn iped_en(&self) -> IpedEn {
        let val = (self.0 >> 1usize) & 0x01;
        IpedEn::from_bits(val as u8)
    }
    #[doc = "IPED Encryption and Decryption Enable."]
    #[inline(always)]
    pub const fn set_iped_en(&mut self, val: IpedEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Write IPED CTR Mode Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipwr_en(&self) -> IpwrEn {
        let val = (self.0 >> 2usize) & 0x01;
        IpwrEn::from_bits(val as u8)
    }
    #[doc = "IP Write IPED CTR Mode Encryption Enable."]
    #[inline(always)]
    pub const fn set_ipwr_en(&mut self, val: IpwrEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "AHB Write IPED CTR Mode Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbwr_en(&self) -> AhbwrEn {
        let val = (self.0 >> 3usize) & 0x01;
        AhbwrEn::from_bits(val as u8)
    }
    #[doc = "AHB Write IPED CTR Mode Encryption Enable."]
    #[inline(always)]
    pub const fn set_ahbwr_en(&mut self, val: AhbwrEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "AHB Read IPED CTR Mode Decryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbrd_en(&self) -> AhbrdEn {
        let val = (self.0 >> 4usize) & 0x01;
        AhbrdEn::from_bits(val as u8)
    }
    #[doc = "AHB Read IPED CTR Mode Decryption Enable."]
    #[inline(always)]
    pub const fn set_ahbrd_en(&mut self, val: AhbrdEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IPED Protection."]
    #[must_use]
    #[inline(always)]
    pub const fn iped_protect(&self) -> IpedProtect {
        let val = (self.0 >> 9usize) & 0x01;
        IpedProtect::from_bits(val as u8)
    }
    #[doc = "IPED Protection."]
    #[inline(always)]
    pub const fn set_iped_protect(&mut self, val: IpedProtect) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Abort Current Decryption or Encryption."]
    #[must_use]
    #[inline(always)]
    pub const fn iped_swreset(&self) -> IpedSwreset {
        let val = (self.0 >> 10usize) & 0x01;
        IpedSwreset::from_bits(val as u8)
    }
    #[doc = "Abort Current Decryption or Encryption."]
    #[inline(always)]
    pub const fn set_iped_swreset(&mut self, val: IpedSwreset) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Ipedctrl {
    #[inline(always)]
    fn default() -> Ipedctrl {
        Ipedctrl(0)
    }
}
impl core::fmt::Debug for Ipedctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctrl")
            .field("config", &self.config())
            .field("iped_en", &self.iped_en())
            .field("ipwr_en", &self.ipwr_en())
            .field("ahbwr_en", &self.ahbwr_en())
            .field("ahbrd_en", &self.ahbrd_en())
            .field("iped_protect", &self.iped_protect())
            .field("iped_swreset", &self.iped_swreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctrl {{ config: {:?}, iped_en: {:?}, ipwr_en: {:?}, ahbwr_en: {:?}, ahbrd_en: {:?}, iped_protect: {:?}, iped_swreset: {:?} }}",
            self.config(),
            self.iped_en(),
            self.ipwr_en(),
            self.ahbwr_en(),
            self.ahbrd_en(),
            self.iped_protect(),
            self.iped_swreset()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0end(pub u32);
impl Ipedctx0end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx0end {
    #[inline(always)]
    fn default() -> Ipedctx0end {
        Ipedctx0end(0)
    }
}
impl core::fmt::Debug for Ipedctx0end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context0 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0iv0(pub u32);
impl Ipedctx0iv0 {
    #[doc = "Lowest 32 bits of IV for region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 0."]
    #[inline(always)]
    pub const fn set_ctx0_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0iv0 {
    #[inline(always)]
    fn default() -> Ipedctx0iv0 {
        Ipedctx0iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx0iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0iv0")
            .field("ctx0_iv0", &self.ctx0_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx0iv0 {{ ctx0_iv0: {=u32:?} }}", self.ctx0_iv0())
    }
}
#[doc = "IPED Context0 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0iv1(pub u32);
impl Ipedctx0iv1 {
    #[doc = "Highest 32 bits of IV for region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 0."]
    #[inline(always)]
    pub const fn set_ctx0_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx0iv1 {
    #[inline(always)]
    fn default() -> Ipedctx0iv1 {
        Ipedctx0iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx0iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0iv1")
            .field("ctx0_iv1", &self.ctx0_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx0iv1 {{ ctx0_iv1: {=u32:?} }}", self.ctx0_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx0start(pub u32);
impl Ipedctx0start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx0startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx0startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx0startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx0start {
    #[inline(always)]
    fn default() -> Ipedctx0start {
        Ipedctx0start(0)
    }
}
impl core::fmt::Debug for Ipedctx0start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx0start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx0start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx0start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1end(pub u32);
impl Ipedctx1end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx1end {
    #[inline(always)]
    fn default() -> Ipedctx1end {
        Ipedctx1end(0)
    }
}
impl core::fmt::Debug for Ipedctx1end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context1 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1iv0(pub u32);
impl Ipedctx1iv0 {
    #[doc = "Lowest 32 bits of IV for region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 1."]
    #[inline(always)]
    pub const fn set_ctx1_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1iv0 {
    #[inline(always)]
    fn default() -> Ipedctx1iv0 {
        Ipedctx1iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx1iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1iv0")
            .field("ctx1_iv0", &self.ctx1_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx1iv0 {{ ctx1_iv0: {=u32:?} }}", self.ctx1_iv0())
    }
}
#[doc = "IPED Context1 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1iv1(pub u32);
impl Ipedctx1iv1 {
    #[doc = "Highest 32 bits of IV for region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 1."]
    #[inline(always)]
    pub const fn set_ctx1_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx1iv1 {
    #[inline(always)]
    fn default() -> Ipedctx1iv1 {
        Ipedctx1iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx1iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1iv1")
            .field("ctx1_iv1", &self.ctx1_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx1iv1 {{ ctx1_iv1: {=u32:?} }}", self.ctx1_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx1start(pub u32);
impl Ipedctx1start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx1startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx1startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx1startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx1start {
    #[inline(always)]
    fn default() -> Ipedctx1start {
        Ipedctx1start(0)
    }
}
impl core::fmt::Debug for Ipedctx1start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx1start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx1start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx1start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2end(pub u32);
impl Ipedctx2end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx2end {
    #[inline(always)]
    fn default() -> Ipedctx2end {
        Ipedctx2end(0)
    }
}
impl core::fmt::Debug for Ipedctx2end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context2 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2iv0(pub u32);
impl Ipedctx2iv0 {
    #[doc = "Lowest 32 bits of IV for region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 2."]
    #[inline(always)]
    pub const fn set_ctx2_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2iv0 {
    #[inline(always)]
    fn default() -> Ipedctx2iv0 {
        Ipedctx2iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx2iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2iv0")
            .field("ctx2_iv0", &self.ctx2_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx2iv0 {{ ctx2_iv0: {=u32:?} }}", self.ctx2_iv0())
    }
}
#[doc = "IPED Context2 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2iv1(pub u32);
impl Ipedctx2iv1 {
    #[doc = "Highest 32 bits of IV for region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 2."]
    #[inline(always)]
    pub const fn set_ctx2_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx2iv1 {
    #[inline(always)]
    fn default() -> Ipedctx2iv1 {
        Ipedctx2iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx2iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2iv1")
            .field("ctx2_iv1", &self.ctx2_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx2iv1 {{ ctx2_iv1: {=u32:?} }}", self.ctx2_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx2start(pub u32);
impl Ipedctx2start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx2startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx2startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx2startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx2start {
    #[inline(always)]
    fn default() -> Ipedctx2start {
        Ipedctx2start(0)
    }
}
impl core::fmt::Debug for Ipedctx2start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx2start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx2start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx2start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3end(pub u32);
impl Ipedctx3end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx3end {
    #[inline(always)]
    fn default() -> Ipedctx3end {
        Ipedctx3end(0)
    }
}
impl core::fmt::Debug for Ipedctx3end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context3 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3iv0(pub u32);
impl Ipedctx3iv0 {
    #[doc = "Lowest 32 bits of IV for region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 3."]
    #[inline(always)]
    pub const fn set_ctx3_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3iv0 {
    #[inline(always)]
    fn default() -> Ipedctx3iv0 {
        Ipedctx3iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx3iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3iv0")
            .field("ctx3_iv0", &self.ctx3_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx3iv0 {{ ctx3_iv0: {=u32:?} }}", self.ctx3_iv0())
    }
}
#[doc = "IPED Context3 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3iv1(pub u32);
impl Ipedctx3iv1 {
    #[doc = "Highest 32 bits of IV for region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 3."]
    #[inline(always)]
    pub const fn set_ctx3_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx3iv1 {
    #[inline(always)]
    fn default() -> Ipedctx3iv1 {
        Ipedctx3iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx3iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3iv1")
            .field("ctx3_iv1", &self.ctx3_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx3iv1 {{ ctx3_iv1: {=u32:?} }}", self.ctx3_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx3start(pub u32);
impl Ipedctx3start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx3startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx3startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx3startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx3start {
    #[inline(always)]
    fn default() -> Ipedctx3start {
        Ipedctx3start(0)
    }
}
impl core::fmt::Debug for Ipedctx3start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx3start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx3start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx3start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4end(pub u32);
impl Ipedctx4end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx4end {
    #[inline(always)]
    fn default() -> Ipedctx4end {
        Ipedctx4end(0)
    }
}
impl core::fmt::Debug for Ipedctx4end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context4 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4iv0(pub u32);
impl Ipedctx4iv0 {
    #[doc = "Lowest 32 bits of IV for region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 4."]
    #[inline(always)]
    pub const fn set_ctx4_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4iv0 {
    #[inline(always)]
    fn default() -> Ipedctx4iv0 {
        Ipedctx4iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx4iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4iv0")
            .field("ctx4_iv0", &self.ctx4_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx4iv0 {{ ctx4_iv0: {=u32:?} }}", self.ctx4_iv0())
    }
}
#[doc = "IPED Context4 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4iv1(pub u32);
impl Ipedctx4iv1 {
    #[doc = "Highest 32 bits of IV for region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 4."]
    #[inline(always)]
    pub const fn set_ctx4_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx4iv1 {
    #[inline(always)]
    fn default() -> Ipedctx4iv1 {
        Ipedctx4iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx4iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4iv1")
            .field("ctx4_iv1", &self.ctx4_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx4iv1 {{ ctx4_iv1: {=u32:?} }}", self.ctx4_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx4start(pub u32);
impl Ipedctx4start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx4startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx4startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx4startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx4start {
    #[inline(always)]
    fn default() -> Ipedctx4start {
        Ipedctx4start(0)
    }
}
impl core::fmt::Debug for Ipedctx4start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx4start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx4start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx4start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5end(pub u32);
impl Ipedctx5end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx5end {
    #[inline(always)]
    fn default() -> Ipedctx5end {
        Ipedctx5end(0)
    }
}
impl core::fmt::Debug for Ipedctx5end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context5 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5iv0(pub u32);
impl Ipedctx5iv0 {
    #[doc = "Lowest 32 bits of IV for region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 5."]
    #[inline(always)]
    pub const fn set_ctx5_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5iv0 {
    #[inline(always)]
    fn default() -> Ipedctx5iv0 {
        Ipedctx5iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx5iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5iv0")
            .field("ctx5_iv0", &self.ctx5_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx5iv0 {{ ctx5_iv0: {=u32:?} }}", self.ctx5_iv0())
    }
}
#[doc = "IPED Context5 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5iv1(pub u32);
impl Ipedctx5iv1 {
    #[doc = "Highest 32 bits of IV for region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 5."]
    #[inline(always)]
    pub const fn set_ctx5_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx5iv1 {
    #[inline(always)]
    fn default() -> Ipedctx5iv1 {
        Ipedctx5iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx5iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5iv1")
            .field("ctx5_iv1", &self.ctx5_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx5iv1 {{ ctx5_iv1: {=u32:?} }}", self.ctx5_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx5start(pub u32);
impl Ipedctx5start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx5startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx5startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx5startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx5start {
    #[inline(always)]
    fn default() -> Ipedctx5start {
        Ipedctx5start(0)
    }
}
impl core::fmt::Debug for Ipedctx5start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx5start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx5start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx5start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6end(pub u32);
impl Ipedctx6end {
    #[doc = "End Address of IPED Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "End Address of IPED Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx6end {
    #[inline(always)]
    fn default() -> Ipedctx6end {
        Ipedctx6end(0)
    }
}
impl core::fmt::Debug for Ipedctx6end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6end")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6end {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context6 IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6iv0(pub u32);
impl Ipedctx6iv0 {
    #[doc = "Lowest 32 bits of IV for region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region 6."]
    #[inline(always)]
    pub const fn set_ctx6_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6iv0 {
    #[inline(always)]
    fn default() -> Ipedctx6iv0 {
        Ipedctx6iv0(0)
    }
}
impl core::fmt::Debug for Ipedctx6iv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6iv0")
            .field("ctx6_iv0", &self.ctx6_iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6iv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx6iv0 {{ ctx6_iv0: {=u32:?} }}", self.ctx6_iv0())
    }
}
#[doc = "IPED Context6 IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6iv1(pub u32);
impl Ipedctx6iv1 {
    #[doc = "Highest 32 bits of IV for region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region 6."]
    #[inline(always)]
    pub const fn set_ctx6_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctx6iv1 {
    #[inline(always)]
    fn default() -> Ipedctx6iv1 {
        Ipedctx6iv1(0)
    }
}
impl core::fmt::Debug for Ipedctx6iv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6iv1")
            .field("ctx6_iv1", &self.ctx6_iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6iv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctx6iv1 {{ ctx6_iv1: {=u32:?} }}", self.ctx6_iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctx6start(pub u32);
impl Ipedctx6start {
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> Ipedctx6startAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        Ipedctx6startAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: Ipedctx6startAhbbuserrorDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Start Address."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Start Address."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Ipedctx6start {
    #[inline(always)]
    fn default() -> Ipedctx6start {
        Ipedctx6start(0)
    }
}
impl core::fmt::Debug for Ipedctx6start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctx6start")
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctx6start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctx6start {{ ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.ahbbuserror_dis(),
            self.start_address()
        )
    }
}
#[doc = "IPED context control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl0(pub u32);
impl Ipedctxctrl0 {
    #[doc = "Context Register Freeze for Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_freeze0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 0."]
    #[inline(always)]
    pub const fn set_ctx0_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Context Register Freeze for Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_freeze0(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 1."]
    #[inline(always)]
    pub const fn set_ctx1_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Context Register Freeze for Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_freeze0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 2."]
    #[inline(always)]
    pub const fn set_ctx2_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Context Register Freeze for Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_freeze0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 3."]
    #[inline(always)]
    pub const fn set_ctx3_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Context Register Freeze for Region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_freeze0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 4."]
    #[inline(always)]
    pub const fn set_ctx4_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Context Register Freeze for Region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_freeze0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 5."]
    #[inline(always)]
    pub const fn set_ctx5_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Context Register Freeze for Region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_freeze0(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 6."]
    #[inline(always)]
    pub const fn set_ctx6_freeze0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Ipedctxctrl0 {
    #[inline(always)]
    fn default() -> Ipedctxctrl0 {
        Ipedctxctrl0(0)
    }
}
impl core::fmt::Debug for Ipedctxctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxctrl0")
            .field("ctx0_freeze0", &self.ctx0_freeze0())
            .field("ctx1_freeze0", &self.ctx1_freeze0())
            .field("ctx2_freeze0", &self.ctx2_freeze0())
            .field("ctx3_freeze0", &self.ctx3_freeze0())
            .field("ctx4_freeze0", &self.ctx4_freeze0())
            .field("ctx5_freeze0", &self.ctx5_freeze0())
            .field("ctx6_freeze0", &self.ctx6_freeze0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl0 {{ ctx0_freeze0: {=u8:?}, ctx1_freeze0: {=u8:?}, ctx2_freeze0: {=u8:?}, ctx3_freeze0: {=u8:?}, ctx4_freeze0: {=u8:?}, ctx5_freeze0: {=u8:?}, ctx6_freeze0: {=u8:?} }}",
            self.ctx0_freeze0(),
            self.ctx1_freeze0(),
            self.ctx2_freeze0(),
            self.ctx3_freeze0(),
            self.ctx4_freeze0(),
            self.ctx5_freeze0(),
            self.ctx6_freeze0()
        )
    }
}
#[doc = "IPED context control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl1(pub u32);
impl Ipedctxctrl1 {
    #[doc = "Context Register Freeze for Region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx0_freeze1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 0."]
    #[inline(always)]
    pub const fn set_ctx0_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Context Register Freeze for Region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx1_freeze1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 1."]
    #[inline(always)]
    pub const fn set_ctx1_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Context Register Freeze for Region 2."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx2_freeze1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 2."]
    #[inline(always)]
    pub const fn set_ctx2_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Context Register Freeze for Region 3."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx3_freeze1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 3."]
    #[inline(always)]
    pub const fn set_ctx3_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Context Register Freeze for Region 4."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx4_freeze1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 4."]
    #[inline(always)]
    pub const fn set_ctx4_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Context Register Freeze for Region 5."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx5_freeze1(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 5."]
    #[inline(always)]
    pub const fn set_ctx5_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Context Register Freeze for Region 6."]
    #[must_use]
    #[inline(always)]
    pub const fn ctx6_freeze1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region 6."]
    #[inline(always)]
    pub const fn set_ctx6_freeze1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
}
impl Default for Ipedctxctrl1 {
    #[inline(always)]
    fn default() -> Ipedctxctrl1 {
        Ipedctxctrl1(0)
    }
}
impl core::fmt::Debug for Ipedctxctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxctrl1")
            .field("ctx0_freeze1", &self.ctx0_freeze1())
            .field("ctx1_freeze1", &self.ctx1_freeze1())
            .field("ctx2_freeze1", &self.ctx2_freeze1())
            .field("ctx3_freeze1", &self.ctx3_freeze1())
            .field("ctx4_freeze1", &self.ctx4_freeze1())
            .field("ctx5_freeze1", &self.ctx5_freeze1())
            .field("ctx6_freeze1", &self.ctx6_freeze1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl1 {{ ctx0_freeze1: {=u8:?}, ctx1_freeze1: {=u8:?}, ctx2_freeze1: {=u8:?}, ctx3_freeze1: {=u8:?}, ctx4_freeze1: {=u8:?}, ctx5_freeze1: {=u8:?}, ctx6_freeze1: {=u8:?} }}",
            self.ctx0_freeze1(),
            self.ctx1_freeze1(),
            self.ctx2_freeze1(),
            self.ctx3_freeze1(),
            self.ctx4_freeze1(),
            self.ctx5_freeze1(),
            self.ctx6_freeze1()
        )
    }
}
#[doc = "IP Receive FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfcr(pub u32);
impl Iprxfcr {
    #[doc = "Clear IP Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn clriprxf(&self) -> Clriprxf {
        let val = (self.0 >> 0usize) & 0x01;
        Clriprxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Receive FIFO."]
    #[inline(always)]
    pub const fn set_clriprxf(&mut self, val: Clriprxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdmaen(&self) -> Rxdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        Rxdmaen::from_bits(val as u8)
    }
    #[doc = "IP Receive FIFO Reading by DMA Enable."]
    #[inline(always)]
    pub const fn set_rxdmaen(&mut self, val: Rxdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IP Receive FIFO Watermark Level."]
    #[must_use]
    #[inline(always)]
    pub const fn rxwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "IP Receive FIFO Watermark Level."]
    #[inline(always)]
    pub const fn set_rxwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for Iprxfcr {
    #[inline(always)]
    fn default() -> Iprxfcr {
        Iprxfcr(0)
    }
}
impl core::fmt::Debug for Iprxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfcr")
            .field("clriprxf", &self.clriprxf())
            .field("rxdmaen", &self.rxdmaen())
            .field("rxwmrk", &self.rxwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iprxfcr {{ clriprxf: {:?}, rxdmaen: {:?}, rxwmrk: {=u8:?} }}",
            self.clriprxf(),
            self.rxdmaen(),
            self.rxwmrk()
        )
    }
}
#[doc = "IP Receive FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iprxfsts(pub u32);
impl Iprxfsts {
    #[doc = "Fill Level of IP Receive FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Receive FIFO."]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read Data Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn rdcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Read Data Counter."]
    #[inline(always)]
    pub const fn set_rdcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iprxfsts {
    #[inline(always)]
    fn default() -> Iprxfsts {
        Iprxfsts(0)
    }
}
impl core::fmt::Debug for Iprxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iprxfsts")
            .field("fill", &self.fill())
            .field("rdcntr", &self.rdcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iprxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iprxfsts {{ fill: {=u8:?}, rdcntr: {=u16:?} }}",
            self.fill(),
            self.rdcntr()
        )
    }
}
#[doc = "IPS Nonsecure Region 0 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszend0(pub u32);
impl Ipsnszend0 {
    #[doc = "End Address of Nonsecure Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Nonsecure Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszend0 {
    #[inline(always)]
    fn default() -> Ipsnszend0 {
        Ipsnszend0(0)
    }
}
impl core::fmt::Debug for Ipsnszend0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszend0")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszend0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszend0 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 1 End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszend1(pub u32);
impl Ipsnszend1 {
    #[doc = "End Address of Nonsecure Region."]
    #[must_use]
    #[inline(always)]
    pub const fn end_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "End Address of Nonsecure Region."]
    #[inline(always)]
    pub const fn set_end_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszend1 {
    #[inline(always)]
    fn default() -> Ipsnszend1 {
        Ipsnszend1(0)
    }
}
impl core::fmt::Debug for Ipsnszend1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszend1")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszend1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszend1 {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 0 Start Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszstart0(pub u32);
impl Ipsnszstart0 {
    #[doc = "Start Address of Nonsecure Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Nonsecure Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszstart0 {
    #[inline(always)]
    fn default() -> Ipsnszstart0 {
        Ipsnszstart0(0)
    }
}
impl core::fmt::Debug for Ipsnszstart0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszstart0")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszstart0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszstart0 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "IPS Nonsecure Region 1 Start Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipsnszstart1(pub u32);
impl Ipsnszstart1 {
    #[doc = "Start Address of Nonsecure Region."]
    #[must_use]
    #[inline(always)]
    pub const fn start_address(&self) -> u32 {
        let val = (self.0 >> 12usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Start Address of Nonsecure Region."]
    #[inline(always)]
    pub const fn set_start_address(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
    }
}
impl Default for Ipsnszstart1 {
    #[inline(always)]
    fn default() -> Ipsnszstart1 {
        Ipsnszstart1(0)
    }
}
impl core::fmt::Debug for Ipsnszstart1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipsnszstart1")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipsnszstart1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipsnszstart1 {{ start_address: {=u32:?} }}",
            self.start_address()
        )
    }
}
#[doc = "IP Transmit FIFO Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfcr(pub u32);
impl Iptxfcr {
    #[doc = "Clear IP Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn clriptxf(&self) -> Clriptxf {
        let val = (self.0 >> 0usize) & 0x01;
        Clriptxf::from_bits(val as u8)
    }
    #[doc = "Clear IP Transmit FIFO."]
    #[inline(always)]
    pub const fn set_clriptxf(&mut self, val: Clriptxf) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn txdmaen(&self) -> Txdmaen {
        let val = (self.0 >> 1usize) & 0x01;
        Txdmaen::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO DMA Enable."]
    #[inline(always)]
    pub const fn set_txdmaen(&mut self, val: Txdmaen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit Watermark Level."]
    #[must_use]
    #[inline(always)]
    pub const fn txwmrk(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit Watermark Level."]
    #[inline(always)]
    pub const fn set_txwmrk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
}
impl Default for Iptxfcr {
    #[inline(always)]
    fn default() -> Iptxfcr {
        Iptxfcr(0)
    }
}
impl core::fmt::Debug for Iptxfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfcr")
            .field("clriptxf", &self.clriptxf())
            .field("txdmaen", &self.txdmaen())
            .field("txwmrk", &self.txwmrk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iptxfcr {{ clriptxf: {:?}, txdmaen: {:?}, txwmrk: {=u8:?} }}",
            self.clriptxf(),
            self.txdmaen(),
            self.txwmrk()
        )
    }
}
#[doc = "IP Transmit FIFO Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iptxfsts(pub u32);
impl Iptxfsts {
    #[doc = "Fill Level of IP Transmit FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fill(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fill Level of IP Transmit FIFO."]
    #[inline(always)]
    pub const fn set_fill(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Write Data Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn wrcntr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Write Data Counter."]
    #[inline(always)]
    pub const fn set_wrcntr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Iptxfsts {
    #[inline(always)]
    fn default() -> Iptxfsts {
        Iptxfsts(0)
    }
}
impl core::fmt::Debug for Iptxfsts {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iptxfsts")
            .field("fill", &self.fill())
            .field("wrcntr", &self.wrcntr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iptxfsts {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Iptxfsts {{ fill: {=u8:?}, wrcntr: {=u16:?} }}",
            self.fill(),
            self.wrcntr()
        )
    }
}
#[doc = "Lookup Table x."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lut(pub u32);
impl Lut {
    #[doc = "OPERAND0."]
    #[must_use]
    #[inline(always)]
    pub const fn operand0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND0."]
    #[inline(always)]
    pub const fn set_operand0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "NUM_PADS0."]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS0."]
    #[inline(always)]
    pub const fn set_num_pads0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "OPCODE."]
    #[must_use]
    #[inline(always)]
    pub const fn opcode0(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE."]
    #[inline(always)]
    pub const fn set_opcode0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "OPERAND1."]
    #[must_use]
    #[inline(always)]
    pub const fn operand1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "OPERAND1."]
    #[inline(always)]
    pub const fn set_operand1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "NUM_PADS1."]
    #[must_use]
    #[inline(always)]
    pub const fn num_pads1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "NUM_PADS1."]
    #[inline(always)]
    pub const fn set_num_pads1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "OPCODE1."]
    #[must_use]
    #[inline(always)]
    pub const fn opcode1(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[doc = "OPCODE1."]
    #[inline(always)]
    pub const fn set_opcode1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for Lut {
    #[inline(always)]
    fn default() -> Lut {
        Lut(0)
    }
}
impl core::fmt::Debug for Lut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lut")
            .field("operand0", &self.operand0())
            .field("num_pads0", &self.num_pads0())
            .field("opcode0", &self.opcode0())
            .field("operand1", &self.operand1())
            .field("num_pads1", &self.num_pads1())
            .field("opcode1", &self.opcode1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lut {{ operand0: {=u8:?}, num_pads0: {=u8:?}, opcode0: {=u8:?}, operand1: {=u8:?}, num_pads1: {=u8:?}, opcode1: {=u8:?} }}",
            self.operand0(),
            self.num_pads0(),
            self.opcode0(),
            self.operand1(),
            self.num_pads1(),
            self.opcode1()
        )
    }
}
#[doc = "LUT Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutcr(pub u32);
impl Lutcr {
    #[doc = "Lock LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> Lock {
        let val = (self.0 >> 0usize) & 0x01;
        Lock::from_bits(val as u8)
    }
    #[doc = "Lock LUT."]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: Lock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Unlock LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> Unlock {
        let val = (self.0 >> 1usize) & 0x01;
        Unlock::from_bits(val as u8)
    }
    #[doc = "Unlock LUT."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: Unlock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LUT Protection."]
    #[must_use]
    #[inline(always)]
    pub const fn protect(&self) -> Protect {
        let val = (self.0 >> 2usize) & 0x01;
        Protect::from_bits(val as u8)
    }
    #[doc = "LUT Protection."]
    #[inline(always)]
    pub const fn set_protect(&mut self, val: Protect) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Lutcr {
    #[inline(always)]
    fn default() -> Lutcr {
        Lutcr(0)
    }
}
impl core::fmt::Debug for Lutcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lutcr")
            .field("lock", &self.lock())
            .field("unlock", &self.unlock())
            .field("protect", &self.protect())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Lutcr {{ lock: {:?}, unlock: {:?}, protect: {:?} }}",
            self.lock(),
            self.unlock(),
            self.protect()
        )
    }
}
#[doc = "LUT Key."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lutkey(pub u32);
impl Lutkey {
    #[doc = "LUT Key."]
    #[must_use]
    #[inline(always)]
    pub const fn key(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LUT Key."]
    #[inline(always)]
    pub const fn set_key(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Lutkey {
    #[inline(always)]
    fn default() -> Lutkey {
        Lutkey(0)
    }
}
impl core::fmt::Debug for Lutkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lutkey").field("key", &self.key()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lutkey {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Lutkey {{ key: {=u32:?} }}", self.key())
    }
}
#[doc = "Module Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr0(pub u32);
impl Mcr0 {
    #[doc = "Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn swreset(&self) -> Swreset {
        let val = (self.0 >> 0usize) & 0x01;
        Swreset::from_bits(val as u8)
    }
    #[doc = "Software Reset."]
    #[inline(always)]
    pub const fn set_swreset(&mut self, val: Swreset) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Module Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mdis(&self) -> Mdis {
        let val = (self.0 >> 1usize) & 0x01;
        Mdis::from_bits(val as u8)
    }
    #[doc = "Module Disable."]
    #[inline(always)]
    pub const fn set_mdis(&mut self, val: Mdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AHB and IPS Bus Endian Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn endcfg(&self) -> Endcfg {
        let val = (self.0 >> 2usize) & 0x03;
        Endcfg::from_bits(val as u8)
    }
    #[doc = "AHB and IPS Bus Endian Configuration."]
    #[inline(always)]
    pub const fn set_endcfg(&mut self, val: Endcfg) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Sample Clock Source for Flash Reading."]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc(&self) -> Rxclksrc {
        let val = (self.0 >> 4usize) & 0x03;
        Rxclksrc::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source for Flash Reading."]
    #[inline(always)]
    pub const fn set_rxclksrc(&mut self, val: Rxclksrc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ardfen(&self) -> Ardfen {
        let val = (self.0 >> 6usize) & 0x01;
        Ardfen::from_bits(val as u8)
    }
    #[doc = "AHB Read Access to IP Receive FIFO Enable."]
    #[inline(always)]
    pub const fn set_ardfen(&mut self, val: Ardfen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn atdfen(&self) -> Atdfen {
        let val = (self.0 >> 7usize) & 0x01;
        Atdfen::from_bits(val as u8)
    }
    #[doc = "AHB Write Access to IP Transmit FIFO Enable."]
    #[inline(always)]
    pub const fn set_atdfen(&mut self, val: Atdfen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial Root Clock Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn serclkdiv(&self) -> Serclkdiv {
        let val = (self.0 >> 8usize) & 0x07;
        Serclkdiv::from_bits(val as u8)
    }
    #[doc = "Serial Root Clock Divider."]
    #[inline(always)]
    pub const fn set_serclkdiv(&mut self, val: Serclkdiv) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn hsen(&self) -> Hsen {
        let val = (self.0 >> 11usize) & 0x01;
        Hsen::from_bits(val as u8)
    }
    #[doc = "Half Speed Serial Flash Memory Access Enable."]
    #[inline(always)]
    pub const fn set_hsen(&mut self, val: Hsen) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Doze Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> Dozeen {
        let val = (self.0 >> 12usize) & 0x01;
        Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Mode Enable."]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: Dozeen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Combination Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn combinationen(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Combination Mode Enable."]
    #[inline(always)]
    pub const fn set_combinationen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SCLK Free-running Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn sckfreerunen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SCLK Free-running Enable."]
    #[inline(always)]
    pub const fn set_sckfreerunen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Data Learning Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn learnen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Data Learning Enable."]
    #[inline(always)]
    pub const fn set_learnen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgrantwait(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Wait Cycle for IP Command Grant."]
    #[inline(always)]
    pub const fn set_ipgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgrantwait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Timeouts Wait Cycle for AHB command Grant."]
    #[inline(always)]
    pub const fn set_ahbgrantwait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr0 {
    #[inline(always)]
    fn default() -> Mcr0 {
        Mcr0(0)
    }
}
impl core::fmt::Debug for Mcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr0")
            .field("swreset", &self.swreset())
            .field("mdis", &self.mdis())
            .field("endcfg", &self.endcfg())
            .field("rxclksrc", &self.rxclksrc())
            .field("ardfen", &self.ardfen())
            .field("atdfen", &self.atdfen())
            .field("serclkdiv", &self.serclkdiv())
            .field("hsen", &self.hsen())
            .field("dozeen", &self.dozeen())
            .field("combinationen", &self.combinationen())
            .field("sckfreerunen", &self.sckfreerunen())
            .field("learnen", &self.learnen())
            .field("ipgrantwait", &self.ipgrantwait())
            .field("ahbgrantwait", &self.ahbgrantwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr0 {{ swreset: {:?}, mdis: {:?}, endcfg: {:?}, rxclksrc: {:?}, ardfen: {:?}, atdfen: {:?}, serclkdiv: {:?}, hsen: {:?}, dozeen: {:?}, combinationen: {=bool:?}, sckfreerunen: {=bool:?}, learnen: {=bool:?}, ipgrantwait: {=u8:?}, ahbgrantwait: {=u8:?} }}",
            self.swreset(),
            self.mdis(),
            self.endcfg(),
            self.rxclksrc(),
            self.ardfen(),
            self.atdfen(),
            self.serclkdiv(),
            self.hsen(),
            self.dozeen(),
            self.combinationen(),
            self.sckfreerunen(),
            self.learnen(),
            self.ipgrantwait(),
            self.ahbgrantwait()
        )
    }
}
#[doc = "Module Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr1(pub u32);
impl Mcr1 {
    #[doc = "AHB Bus Wait."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuswait(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "AHB Bus Wait."]
    #[inline(always)]
    pub const fn set_ahbbuswait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Command Sequence Wait."]
    #[must_use]
    #[inline(always)]
    pub const fn seqwait(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Command Sequence Wait."]
    #[inline(always)]
    pub const fn set_seqwait(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Mcr1 {
    #[inline(always)]
    fn default() -> Mcr1 {
        Mcr1(0)
    }
}
impl core::fmt::Debug for Mcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr1")
            .field("ahbbuswait", &self.ahbbuswait())
            .field("seqwait", &self.seqwait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr1 {{ ahbbuswait: {=u16:?}, seqwait: {=u16:?} }}",
            self.ahbbuswait(),
            self.seqwait()
        )
    }
}
#[doc = "Module Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr2(pub u32);
impl Mcr2 {
    #[doc = "Abort on Command Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn abortoncmden(&self) -> Abortoncmden {
        let val = (self.0 >> 0usize) & 0x01;
        Abortoncmden::from_bits(val as u8)
    }
    #[doc = "Abort on Command Enable."]
    #[inline(always)]
    pub const fn set_abortoncmden(&mut self, val: Abortoncmden) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow Command Sequence Abort during RADDR."]
    #[must_use]
    #[inline(always)]
    pub const fn abortonraddren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during RADDR."]
    #[inline(always)]
    pub const fn set_abortonraddren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Command Sequence Abort during CADDR."]
    #[must_use]
    #[inline(always)]
    pub const fn abortoncaddren(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during CADDR."]
    #[inline(always)]
    pub const fn set_abortoncaddren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allow Command Sequence Abort during MODE."]
    #[must_use]
    #[inline(always)]
    pub const fn abortonmodeen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during MODE."]
    #[inline(always)]
    pub const fn set_abortonmodeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Allow Command Sequence Abort during DUMMY."]
    #[must_use]
    #[inline(always)]
    pub const fn abortondummyen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during DUMMY."]
    #[inline(always)]
    pub const fn set_abortondummyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Allow Command Sequence Abort during WRITE."]
    #[must_use]
    #[inline(always)]
    pub const fn abortonwriteen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during WRITE."]
    #[inline(always)]
    pub const fn set_abortonwriteen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Allow Command Sequence Abort during READ."]
    #[must_use]
    #[inline(always)]
    pub const fn abortonreaden(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during READ."]
    #[inline(always)]
    pub const fn set_abortonreaden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Allow Command Sequence Abort during LEARN."]
    #[must_use]
    #[inline(always)]
    pub const fn abortonlearnen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during LEARN."]
    #[inline(always)]
    pub const fn set_abortonlearnen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Allow Command Sequence Abort during DATSZ."]
    #[must_use]
    #[inline(always)]
    pub const fn abortondatszen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Command Sequence Abort during DATSZ."]
    #[inline(always)]
    pub const fn set_abortondatszen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clear AHB Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn clrahbbufopt(&self) -> Clrahbbufopt {
        let val = (self.0 >> 11usize) & 0x01;
        Clrahbbufopt::from_bits(val as u8)
    }
    #[doc = "Clear AHB Buffer."]
    #[inline(always)]
    pub const fn set_clrahbbufopt(&mut self, val: Clrahbbufopt) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SCLK2 Toggle Output."]
    #[must_use]
    #[inline(always)]
    pub const fn sck2opt(&self) -> Sck2opt {
        let val = (self.0 >> 12usize) & 0x01;
        Sck2opt::from_bits(val as u8)
    }
    #[doc = "SCLK2 Toggle Output."]
    #[inline(always)]
    pub const fn set_sck2opt(&mut self, val: Sck2opt) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Test Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tstmd(&self) -> Tstmd {
        let val = (self.0 >> 13usize) & 0x01;
        Tstmd::from_bits(val as u8)
    }
    #[doc = "Test Mode Enable."]
    #[inline(always)]
    pub const fn set_tstmd(&mut self, val: Tstmd) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Clear Learn Phase Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn clrlearnphase(&self) -> Clrlearnphase {
        let val = (self.0 >> 14usize) & 0x01;
        Clrlearnphase::from_bits(val as u8)
    }
    #[doc = "Clear Learn Phase Selection."]
    #[inline(always)]
    pub const fn set_clrlearnphase(&mut self, val: Clrlearnphase) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Same Device Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn samedeviceen(&self) -> Samedeviceen {
        let val = (self.0 >> 15usize) & 0x01;
        Samedeviceen::from_bits(val as u8)
    }
    #[doc = "Same Device Enable."]
    #[inline(always)]
    pub const fn set_samedeviceen(&mut self, val: Samedeviceen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Flash DQS Option."]
    #[must_use]
    #[inline(always)]
    pub const fn flashdqsopt(&self) -> Flashdqsopt {
        let val = (self.0 >> 16usize) & 0x01;
        Flashdqsopt::from_bits(val as u8)
    }
    #[doc = "Flash DQS Option."]
    #[inline(always)]
    pub const fn set_flashdqsopt(&mut self, val: Flashdqsopt) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Option bit for RX data sampling (when MCR0\\[RXCLKSRC\\] is not set to 0x3), for internal use only."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdelayopt(&self) -> Rxdelayopt {
        let val = (self.0 >> 17usize) & 0x03;
        Rxdelayopt::from_bits(val as u8)
    }
    #[doc = "Option bit for RX data sampling (when MCR0\\[RXCLKSRC\\] is not set to 0x3), for internal use only."]
    #[inline(always)]
    pub const fn set_rxdelayopt(&mut self, val: Rxdelayopt) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "SCLK Port B Differential Output."]
    #[must_use]
    #[inline(always)]
    pub const fn sckbdiffopt(&self) -> Sckbdiffopt {
        let val = (self.0 >> 19usize) & 0x01;
        Sckbdiffopt::from_bits(val as u8)
    }
    #[doc = "SCLK Port B Differential Output."]
    #[inline(always)]
    pub const fn set_sckbdiffopt(&mut self, val: Sckbdiffopt) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Clock Phase Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn clkphaserst(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Phase Reset."]
    #[inline(always)]
    pub const fn set_clkphaserst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Port B Receiver Clock Source."]
    #[must_use]
    #[inline(always)]
    pub const fn rxclksrc_b(&self) -> RxclksrcB {
        let val = (self.0 >> 21usize) & 0x03;
        RxclksrcB::from_bits(val as u8)
    }
    #[doc = "Port B Receiver Clock Source."]
    #[inline(always)]
    pub const fn set_rxclksrc_b(&mut self, val: RxclksrcB) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Sample Clock Source Different."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_clk_src_diff(&self) -> RxClkSrcDiff {
        let val = (self.0 >> 23usize) & 0x01;
        RxClkSrcDiff::from_bits(val as u8)
    }
    #[doc = "Sample Clock Source Different."]
    #[inline(always)]
    pub const fn set_rx_clk_src_diff(&mut self, val: RxClkSrcDiff) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Resume Wait Duration."]
    #[must_use]
    #[inline(always)]
    pub const fn resumewait(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Resume Wait Duration."]
    #[inline(always)]
    pub const fn set_resumewait(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Mcr2 {
    #[inline(always)]
    fn default() -> Mcr2 {
        Mcr2(0)
    }
}
impl core::fmt::Debug for Mcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr2")
            .field("abortoncmden", &self.abortoncmden())
            .field("abortonraddren", &self.abortonraddren())
            .field("abortoncaddren", &self.abortoncaddren())
            .field("abortonmodeen", &self.abortonmodeen())
            .field("abortondummyen", &self.abortondummyen())
            .field("abortonwriteen", &self.abortonwriteen())
            .field("abortonreaden", &self.abortonreaden())
            .field("abortonlearnen", &self.abortonlearnen())
            .field("abortondatszen", &self.abortondatszen())
            .field("clrahbbufopt", &self.clrahbbufopt())
            .field("sck2opt", &self.sck2opt())
            .field("tstmd", &self.tstmd())
            .field("clrlearnphase", &self.clrlearnphase())
            .field("samedeviceen", &self.samedeviceen())
            .field("flashdqsopt", &self.flashdqsopt())
            .field("rxdelayopt", &self.rxdelayopt())
            .field("sckbdiffopt", &self.sckbdiffopt())
            .field("clkphaserst", &self.clkphaserst())
            .field("rxclksrc_b", &self.rxclksrc_b())
            .field("rx_clk_src_diff", &self.rx_clk_src_diff())
            .field("resumewait", &self.resumewait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mcr2 {{ abortoncmden: {:?}, abortonraddren: {=bool:?}, abortoncaddren: {=bool:?}, abortonmodeen: {=bool:?}, abortondummyen: {=bool:?}, abortonwriteen: {=bool:?}, abortonreaden: {=bool:?}, abortonlearnen: {=bool:?}, abortondatszen: {=bool:?}, clrahbbufopt: {:?}, sck2opt: {:?}, tstmd: {:?}, clrlearnphase: {:?}, samedeviceen: {:?}, flashdqsopt: {:?}, rxdelayopt: {:?}, sckbdiffopt: {:?}, clkphaserst: {=bool:?}, rxclksrc_b: {:?}, rx_clk_src_diff: {:?}, resumewait: {=u8:?} }}",
            self.abortoncmden(),
            self.abortonraddren(),
            self.abortoncaddren(),
            self.abortonmodeen(),
            self.abortondummyen(),
            self.abortonwriteen(),
            self.abortonreaden(),
            self.abortonlearnen(),
            self.abortondatszen(),
            self.clrahbbufopt(),
            self.sck2opt(),
            self.tstmd(),
            self.clrlearnphase(),
            self.samedeviceen(),
            self.flashdqsopt(),
            self.rxdelayopt(),
            self.sckbdiffopt(),
            self.clkphaserst(),
            self.rxclksrc_b(),
            self.rx_clk_src_diff(),
            self.resumewait()
        )
    }
}
#[doc = "Misc Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misccr2(pub u32);
impl Misccr2 {
    #[doc = "Data Learning Clock Phase Gap."]
    #[must_use]
    #[inline(always)]
    pub const fn learnphasegap(&self) -> Learnphasegap {
        let val = (self.0 >> 0usize) & 0x03;
        Learnphasegap::from_bits(val as u8)
    }
    #[doc = "Data Learning Clock Phase Gap."]
    #[inline(always)]
    pub const fn set_learnphasegap(&mut self, val: Learnphasegap) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Clock Phase Reset Output."]
    #[must_use]
    #[inline(always)]
    pub const fn phaserstopt(&self) -> Phaserstopt {
        let val = (self.0 >> 2usize) & 0x03;
        Phaserstopt::from_bits(val as u8)
    }
    #[doc = "Clock Phase Reset Output."]
    #[inline(always)]
    pub const fn set_phaserstopt(&mut self, val: Phaserstopt) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Data Line Output Delay Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn doeopt(&self) -> Doeopt {
        let val = (self.0 >> 4usize) & 0x01;
        Doeopt::from_bits(val as u8)
    }
    #[doc = "Data Line Output Delay Enable."]
    #[inline(always)]
    pub const fn set_doeopt(&mut self, val: Doeopt) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "RWDS Output Enable De-assertion Delay."]
    #[must_use]
    #[inline(always)]
    pub const fn rwdsoeopt(&self) -> Rwdsoeopt {
        let val = (self.0 >> 5usize) & 0x01;
        Rwdsoeopt::from_bits(val as u8)
    }
    #[doc = "RWDS Output Enable De-assertion Delay."]
    #[inline(always)]
    pub const fn set_rwdsoeopt(&mut self, val: Rwdsoeopt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "OTFAD Software Reset."]
    #[must_use]
    #[inline(always)]
    pub const fn otfad_swreset(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "OTFAD Software Reset."]
    #[inline(always)]
    pub const fn set_otfad_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Extend."]
    #[must_use]
    #[inline(always)]
    pub const fn timerextend(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Extend."]
    #[inline(always)]
    pub const fn set_timerextend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Software Reset for IPED MUX."]
    #[must_use]
    #[inline(always)]
    pub const fn ipedmux_swreset(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset for IPED MUX."]
    #[inline(always)]
    pub const fn set_ipedmux_swreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Misccr2 {
    #[inline(always)]
    fn default() -> Misccr2 {
        Misccr2(0)
    }
}
impl core::fmt::Debug for Misccr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misccr2")
            .field("learnphasegap", &self.learnphasegap())
            .field("phaserstopt", &self.phaserstopt())
            .field("doeopt", &self.doeopt())
            .field("rwdsoeopt", &self.rwdsoeopt())
            .field("otfad_swreset", &self.otfad_swreset())
            .field("timerextend", &self.timerextend())
            .field("ipedmux_swreset", &self.ipedmux_swreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misccr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misccr2 {{ learnphasegap: {:?}, phaserstopt: {:?}, doeopt: {:?}, rwdsoeopt: {:?}, otfad_swreset: {=bool:?}, timerextend: {=bool:?}, ipedmux_swreset: {=bool:?} }}",
            self.learnphasegap(),
            self.phaserstopt(),
            self.doeopt(),
            self.rwdsoeopt(),
            self.otfad_swreset(),
            self.timerextend(),
            self.ipedmux_swreset()
        )
    }
}
#[doc = "Misc Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misccr3(pub u32);
impl Misccr3 {
    #[doc = "Number of AHB clk delays between internal async TX FIFO push."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_wr_delay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of AHB clk delays between internal async TX FIFO push."]
    #[inline(always)]
    pub const fn set_tx_fifo_wr_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Misccr3 {
    #[inline(always)]
    fn default() -> Misccr3 {
        Misccr3(0)
    }
}
impl core::fmt::Debug for Misccr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Misccr3")
            .field("tx_fifo_wr_delay", &self.tx_fifo_wr_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Misccr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Misccr3 {{ tx_fifo_wr_delay: {=u8:?} }}",
            self.tx_fifo_wr_delay()
        )
    }
}
#[doc = "IP Receive FIFO Data x."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfdr(pub u32);
impl Rfdr {
    #[doc = "Receive Data."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Receive Data."]
    #[inline(always)]
    pub const fn set_rxdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rfdr {
    #[inline(always)]
    fn default() -> Rfdr {
        Rfdr(0)
    }
}
impl core::fmt::Debug for Rfdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfdr")
            .field("rxdata", &self.rxdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rfdr {{ rxdata: {=u32:?} }}", self.rxdata())
    }
}
#[doc = "Status 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts0(pub u32);
impl Sts0 {
    #[doc = "SEQ_CTL State Machine Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn seqidle(&self) -> Seqidle {
        let val = (self.0 >> 0usize) & 0x01;
        Seqidle::from_bits(val as u8)
    }
    #[doc = "SEQ_CTL State Machine Idle."]
    #[inline(always)]
    pub const fn set_seqidle(&mut self, val: Seqidle) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "ARB_CTL State Machine Idle."]
    #[must_use]
    #[inline(always)]
    pub const fn arbidle(&self) -> Arbidle {
        let val = (self.0 >> 1usize) & 0x01;
        Arbidle::from_bits(val as u8)
    }
    #[doc = "ARB_CTL State Machine Idle."]
    #[inline(always)]
    pub const fn set_arbidle(&mut self, val: Arbidle) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "ARB Command Source."]
    #[must_use]
    #[inline(always)]
    pub const fn arbcmdsrc(&self) -> Arbcmdsrc {
        let val = (self.0 >> 2usize) & 0x03;
        Arbcmdsrc::from_bits(val as u8)
    }
    #[doc = "ARB Command Source."]
    #[inline(always)]
    pub const fn set_arbcmdsrc(&mut self, val: Arbcmdsrc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Data Learning Phase Selection on Port A."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphasea(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Learning Phase Selection on Port A."]
    #[inline(always)]
    pub const fn set_datalearnphasea(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Data Learning Phase Selection on Port B."]
    #[must_use]
    #[inline(always)]
    pub const fn datalearnphaseb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Learning Phase Selection on Port B."]
    #[inline(always)]
    pub const fn set_datalearnphaseb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Sts0 {
    #[inline(always)]
    fn default() -> Sts0 {
        Sts0(0)
    }
}
impl core::fmt::Debug for Sts0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts0")
            .field("seqidle", &self.seqidle())
            .field("arbidle", &self.arbidle())
            .field("arbcmdsrc", &self.arbcmdsrc())
            .field("datalearnphasea", &self.datalearnphasea())
            .field("datalearnphaseb", &self.datalearnphaseb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts0 {{ seqidle: {:?}, arbidle: {:?}, arbcmdsrc: {:?}, datalearnphasea: {=u8:?}, datalearnphaseb: {=u8:?} }}",
            self.seqidle(),
            self.arbidle(),
            self.arbcmdsrc(),
            self.datalearnphasea(),
            self.datalearnphaseb()
        )
    }
}
#[doc = "Status 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts1(pub u32);
impl Sts1 {
    #[doc = "AHB Command Error ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "AHB Command Error ID."]
    #[inline(always)]
    pub const fn set_ahbcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "AHB Command Error Code."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbcmderrcode(&self) -> Ahbcmderrcode {
        let val = (self.0 >> 8usize) & 0x0f;
        Ahbcmderrcode::from_bits(val as u8)
    }
    #[doc = "AHB Command Error Code."]
    #[inline(always)]
    pub const fn set_ahbcmderrcode(&mut self, val: Ahbcmderrcode) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "IP Command Error ID."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrid(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IP Command Error ID."]
    #[inline(always)]
    pub const fn set_ipcmderrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "IP Command Error Code."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcmderrcode(&self) -> Ipcmderrcode {
        let val = (self.0 >> 24usize) & 0x0f;
        Ipcmderrcode::from_bits(val as u8)
    }
    #[doc = "IP Command Error Code."]
    #[inline(always)]
    pub const fn set_ipcmderrcode(&mut self, val: Ipcmderrcode) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Sts1 {
    #[inline(always)]
    fn default() -> Sts1 {
        Sts1(0)
    }
}
impl core::fmt::Debug for Sts1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts1")
            .field("ahbcmderrid", &self.ahbcmderrid())
            .field("ahbcmderrcode", &self.ahbcmderrcode())
            .field("ipcmderrid", &self.ipcmderrid())
            .field("ipcmderrcode", &self.ipcmderrcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts1 {{ ahbcmderrid: {=u8:?}, ahbcmderrcode: {:?}, ipcmderrid: {=u8:?}, ipcmderrcode: {:?} }}",
            self.ahbcmderrid(),
            self.ahbcmderrcode(),
            self.ipcmderrid(),
            self.ipcmderrcode()
        )
    }
}
#[doc = "Status 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts2(pub u32);
impl Sts2 {
    #[doc = "Flash A Sample Target Delay Line Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn aslvlock(&self) -> Aslvlock {
        let val = (self.0 >> 0usize) & 0x01;
        Aslvlock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Target Delay Line Locked."]
    #[inline(always)]
    pub const fn set_aslvlock(&mut self, val: Aslvlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn areflock(&self) -> Areflock {
        let val = (self.0 >> 1usize) & 0x01;
        Areflock::from_bits(val as u8)
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Locked."]
    #[inline(always)]
    pub const fn set_areflock(&mut self, val: Areflock) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number."]
    #[must_use]
    #[inline(always)]
    pub const fn aslvsel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Target Delay Line Delay Cell Number."]
    #[inline(always)]
    pub const fn set_aslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number."]
    #[must_use]
    #[inline(always)]
    pub const fn arefsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash A Sample Clock Reference Delay Line Delay Cell Number."]
    #[inline(always)]
    pub const fn set_arefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn bslvlock(&self) -> Bslvlock {
        let val = (self.0 >> 16usize) & 0x01;
        Bslvlock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Target Reference Delay Line Locked."]
    #[inline(always)]
    pub const fn set_bslvlock(&mut self, val: Bslvlock) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked."]
    #[must_use]
    #[inline(always)]
    pub const fn breflock(&self) -> Breflock {
        let val = (self.0 >> 17usize) & 0x01;
        Breflock::from_bits(val as u8)
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Locked."]
    #[inline(always)]
    pub const fn set_breflock(&mut self, val: Breflock) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number."]
    #[must_use]
    #[inline(always)]
    pub const fn bslvsel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Target Delay Line Delay Cell Number."]
    #[inline(always)]
    pub const fn set_bslvsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number."]
    #[must_use]
    #[inline(always)]
    pub const fn brefsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Flash B Sample Clock Reference Delay Line Delay Cell Number."]
    #[inline(always)]
    pub const fn set_brefsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Sts2 {
    #[inline(always)]
    fn default() -> Sts2 {
        Sts2(0)
    }
}
impl core::fmt::Debug for Sts2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sts2")
            .field("aslvlock", &self.aslvlock())
            .field("areflock", &self.areflock())
            .field("aslvsel", &self.aslvsel())
            .field("arefsel", &self.arefsel())
            .field("bslvlock", &self.bslvlock())
            .field("breflock", &self.breflock())
            .field("bslvsel", &self.bslvsel())
            .field("brefsel", &self.brefsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sts2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sts2 {{ aslvlock: {:?}, areflock: {:?}, aslvsel: {=u8:?}, arefsel: {=u8:?}, bslvlock: {:?}, breflock: {:?}, bslvsel: {=u8:?}, brefsel: {=u8:?} }}",
            self.aslvlock(),
            self.areflock(),
            self.aslvsel(),
            self.arefsel(),
            self.bslvlock(),
            self.breflock(),
            self.bslvsel(),
            self.brefsel()
        )
    }
}
#[doc = "IP TX FIFO Data x."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tfdr(pub u32);
impl Tfdr {
    #[doc = "Transmit Data."]
    #[must_use]
    #[inline(always)]
    pub const fn txdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Transmit Data."]
    #[inline(always)]
    pub const fn set_txdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tfdr {
    #[inline(always)]
    fn default() -> Tfdr {
        Tfdr(0)
    }
}
impl core::fmt::Debug for Tfdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tfdr")
            .field("txdata", &self.txdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tfdr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tfdr {{ txdata: {=u32:?} }}", self.txdata())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abortoncmden {
    #[doc = "When command abort request is received during CMD instruction, command sequence waits for the CMD instruction to complete instead of aborting immediately."]
    Val0 = 0x0,
    #[doc = "When command abort request is received during CMD instruction, command sequence is aborted immediately."]
    Val1 = 0x01,
}
impl Abortoncmden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abortoncmden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abortoncmden {
    #[inline(always)]
    fn from(val: u8) -> Abortoncmden {
        Abortoncmden::from_bits(val)
    }
}
impl From<Abortoncmden> for u8 {
    #[inline(always)]
    fn from(val: Abortoncmden) -> u8 {
        Abortoncmden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "No suspended AHB read prefetch command."]
    Val0 = 0x0,
    #[doc = "An AHB read prefetch command sequence has been suspended."]
    Val1 = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbustimeouten {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ahbbustimeouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbustimeouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbustimeouten {
    #[inline(always)]
    fn from(val: u8) -> Ahbbustimeouten {
        Ahbbustimeouten::from_bits(val)
    }
}
impl From<Ahbbustimeouten> for u8 {
    #[inline(always)]
    fn from(val: Ahbbustimeouten) -> u8 {
        Ahbbustimeouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderrcode {
    #[doc = "No error."]
    Val0 = 0x0,
    #[doc = "Write or read data size is not a multiple of two bytes while parallel mode is enabled (AHBCR\\[APAREN\\] = 1). This error only occurs when FLSHCR4\\[WMOPT\\] = 1."]
    Val1 = 0x01,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence."]
    Val2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence."]
    Val3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence."]
    Val4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence."]
    Val5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    Val6 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ahbcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderrcode {
        Ahbcmderrcode::from_bits(val)
    }
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderrcode) -> u8 {
        Ahbcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderren {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ahbcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderren {
        Ahbcmderren::from_bits(val)
    }
}
impl From<Ahbcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderren) -> u8 {
        Ahbcmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmdgeen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ahbcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmdgeen {
        Ahbcmdgeen::from_bits(val)
    }
}
impl From<Ahbcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmdgeen) -> u8 {
        Ahbcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbcrPrefetchen {
    #[doc = "Disable."]
    Value0 = 0x0,
    #[doc = "Enable."]
    Value1 = 0x01,
}
impl AhbcrPrefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbcrPrefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbcrPrefetchen {
    #[inline(always)]
    fn from(val: u8) -> AhbcrPrefetchen {
        AhbcrPrefetchen::from_bits(val)
    }
}
impl From<AhbcrPrefetchen> for u8 {
    #[inline(always)]
    fn from(val: AhbcrPrefetchen) -> u8 {
        AhbcrPrefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbrdEn {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl AhbrdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbrdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbrdEn {
    #[inline(always)]
    fn from(val: u8) -> AhbrdEn {
        AhbrdEn::from_bits(val)
    }
}
impl From<AhbrdEn> for u8 {
    #[inline(always)]
    fn from(val: AhbrdEn) -> u8 {
        AhbrdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        Ahbrxbuf0cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Prefetchen) -> u8 {
        Ahbrxbuf0cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Regionen {
        Ahbrxbuf0cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Regionen) -> u8 {
        Ahbrxbuf0cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        Ahbrxbuf1cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Prefetchen) -> u8 {
        Ahbrxbuf1cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Regionen {
        Ahbrxbuf1cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Regionen) -> u8 {
        Ahbrxbuf1cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        Ahbrxbuf2cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Prefetchen) -> u8 {
        Ahbrxbuf2cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Regionen {
        Ahbrxbuf2cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Regionen) -> u8 {
        Ahbrxbuf2cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        Ahbrxbuf3cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Prefetchen) -> u8 {
        Ahbrxbuf3cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Regionen {
        Ahbrxbuf3cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Regionen) -> u8 {
        Ahbrxbuf3cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        Ahbrxbuf4cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Prefetchen) -> u8 {
        Ahbrxbuf4cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Regionen {
        Ahbrxbuf4cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Regionen) -> u8 {
        Ahbrxbuf4cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        Ahbrxbuf5cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Prefetchen) -> u8 {
        Ahbrxbuf5cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Regionen {
        Ahbrxbuf5cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Regionen) -> u8 {
        Ahbrxbuf5cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        Ahbrxbuf6cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Prefetchen) -> u8 {
        Ahbrxbuf6cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Regionen {
        Ahbrxbuf6cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Regionen) -> u8 {
        Ahbrxbuf6cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        Ahbrxbuf7cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Prefetchen) -> u8 {
        Ahbrxbuf7cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Regionen {
        Ahbrxbuf7cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Regionen) -> u8 {
        Ahbrxbuf7cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbwrEn {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl AhbwrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbwrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbwrEn {
    #[inline(always)]
    fn from(val: u8) -> AhbwrEn {
        AhbwrEn::from_bits(val)
    }
}
impl From<AhbwrEn> for u8 {
    #[inline(always)]
    fn from(val: AhbwrEn) -> u8 {
        AhbwrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alignment {
    #[doc = "No limit."]
    Bit0 = 0x0,
    #[doc = "1 KB."]
    Bit1 = 0x01,
    #[doc = "512 bytes."]
    Bit2 = 0x02,
    #[doc = "256 bytes."]
    Bit3 = 0x03,
}
impl Alignment {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alignment {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alignment {
    #[inline(always)]
    fn from(val: u8) -> Alignment {
        Alignment::from_bits(val)
    }
}
impl From<Alignment> for u8 {
    #[inline(always)]
    fn from(val: Alignment) -> u8 {
        Alignment::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aparen {
    #[doc = "Flash is accessed in Individual mode."]
    Individual = 0x0,
    #[doc = "Flash is accessed in Parallel mode."]
    Enable = 0x01,
}
impl Aparen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aparen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aparen {
    #[inline(always)]
    fn from(val: u8) -> Aparen {
        Aparen::from_bits(val)
    }
}
impl From<Aparen> for u8 {
    #[inline(always)]
    fn from(val: Aparen) -> u8 {
        Aparen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbcmdsrc {
    #[doc = "Trigger source is AHB read command."]
    Val0 = 0x0,
    #[doc = "Trigger source is AHB write command."]
    Val1 = 0x01,
    #[doc = "Trigger source is IP command (by writing 1 to IPCMD\\[TRG\\])."]
    Val2 = 0x02,
    #[doc = "Trigger source is a suspended command that has resumed."]
    Val3 = 0x03,
}
impl Arbcmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbcmdsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbcmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Arbcmdsrc {
        Arbcmdsrc::from_bits(val)
    }
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Arbcmdsrc) -> u8 {
        Arbcmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbidle {
    #[doc = "Not idle."]
    Value0 = 0x0,
    #[doc = "Idle."]
    Value1 = 0x01,
}
impl Arbidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbidle {
    #[inline(always)]
    fn from(val: u8) -> Arbidle {
        Arbidle::from_bits(val)
    }
}
impl From<Arbidle> for u8 {
    #[inline(always)]
    fn from(val: Arbidle) -> u8 {
        Arbidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ardfen {
    #[doc = "AHB read access disabled. IP bus reads IP receive FIFO. AHB Bus read access to IP receive FIFO memory space produces bus error."]
    Val0 = 0x0,
    #[doc = "AHB read access enabled. AHB bus reads IP receive FIFO. IP Bus read access to IP receive FIFO memory space returns data zero and causes no bus error."]
    Val1 = 0x01,
}
impl Ardfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ardfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ardfen {
    #[inline(always)]
    fn from(val: u8) -> Ardfen {
        Ardfen::from_bits(val)
    }
}
impl From<Ardfen> for u8 {
    #[inline(always)]
    fn from(val: Ardfen) -> u8 {
        Ardfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Areflock {
    #[doc = "Not locked."]
    Val0 = 0x0,
    #[doc = "Locked."]
    Val1 = 0x01,
}
impl Areflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Areflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Areflock {
    #[inline(always)]
    fn from(val: u8) -> Areflock {
        Areflock::from_bits(val)
    }
}
impl From<Areflock> for u8 {
    #[inline(always)]
    fn from(val: Areflock) -> u8 {
        Areflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aslvlock {
    #[doc = "Not locked."]
    Val0 = 0x0,
    #[doc = "Locked."]
    Val1 = 0x01,
}
impl Aslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aslvlock {
    #[inline(always)]
    fn from(val: u8) -> Aslvlock {
        Aslvlock::from_bits(val)
    }
}
impl From<Aslvlock> for u8 {
    #[inline(always)]
    fn from(val: Aslvlock) -> u8 {
        Aslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atdfen {
    #[doc = "AHB write access disabled. IP bus writes to IP transmit FIFO. AHB bus write access to IP transmit FIFO memory space produces bus error."]
    Val0 = 0x0,
    #[doc = "AHB write access enabled. AHB bus writes to IP transmit FIFO. IP Bus write access to IP transmit FIFO memory space is ignored and causes no bus error."]
    Val1 = 0x01,
}
impl Atdfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atdfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atdfen {
    #[inline(always)]
    fn from(val: u8) -> Atdfen {
        Atdfen::from_bits(val)
    }
}
impl From<Atdfen> for u8 {
    #[inline(always)]
    fn from(val: Atdfen) -> u8 {
        Atdfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Awrwaitunit {
    #[doc = "2."]
    Val0 = 0x0,
    #[doc = "8."]
    Val1 = 0x01,
    #[doc = "32."]
    Val2 = 0x02,
    #[doc = "128."]
    Val3 = 0x03,
    #[doc = "512."]
    Val4 = 0x04,
    #[doc = "2048."]
    Val5 = 0x05,
    #[doc = "8192."]
    Val6 = 0x06,
    #[doc = "32768."]
    Val7 = 0x07,
}
impl Awrwaitunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awrwaitunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awrwaitunit {
    #[inline(always)]
    fn from(val: u8) -> Awrwaitunit {
        Awrwaitunit::from_bits(val)
    }
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(val: Awrwaitunit) -> u8 {
        Awrwaitunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Breflock {
    #[doc = "Not locked."]
    Val0 = 0x0,
    #[doc = "Locked."]
    Val1 = 0x01,
}
impl Breflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Breflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Breflock {
    #[inline(always)]
    fn from(val: u8) -> Breflock {
        Breflock::from_bits(val)
    }
}
impl From<Breflock> for u8 {
    #[inline(always)]
    fn from(val: Breflock) -> u8 {
        Breflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bslvlock {
    #[doc = "Not locked."]
    Val0 = 0x0,
    #[doc = "Locked."]
    Val1 = 0x01,
}
impl Bslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bslvlock {
    #[inline(always)]
    fn from(val: u8) -> Bslvlock {
        Bslvlock::from_bits(val)
    }
}
impl From<Bslvlock> for u8 {
    #[inline(always)]
    fn from(val: Bslvlock) -> u8 {
        Bslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufferableen {
    #[doc = "Disabled. For all AHB write accesses (bufferable or nonbufferable hprot\\[2\\]=0x1 or 0x0), FlexSPI returns AHB Bus Ready after transmitting all data and finishing command."]
    Val0 = 0x0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI returns AHB Bus Ready when the arbitrator grants the AHB command. FlexSPI does not wait for the AHB command to finish."]
    Val1 = 0x01,
}
impl Bufferableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufferableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufferableen {
    #[inline(always)]
    fn from(val: u8) -> Bufferableen {
        Bufferableen::from_bits(val)
    }
}
impl From<Bufferableen> for u8 {
    #[inline(always)]
    fn from(val: Bufferableen) -> u8 {
        Bufferableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cachableen {
    #[doc = "Disabled. When an AHB bus cacheable read access occurs, FlexSPI does not check whether it hit the AHB transmit buffer."]
    Val0 = 0x0,
    #[doc = "Enabled. When an AHB bus cacheable read access occurs, FlexSPI first checks whether the access hit the AHB transmit buffer."]
    Val1 = 0x01,
}
impl Cachableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cachableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cachableen {
    #[inline(always)]
    fn from(val: u8) -> Cachableen {
        Cachableen::from_bits(val)
    }
}
impl From<Cachableen> for u8 {
    #[inline(always)]
    fn from(val: Cachableen) -> u8 {
        Cachableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbbufopt {
    #[doc = "Not cleared automatically."]
    Val0 = 0x0,
    #[doc = "Cleared automatically."]
    Val1 = 0x01,
}
impl Clrahbbufopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbbufopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbbufopt {
    #[inline(always)]
    fn from(val: u8) -> Clrahbbufopt {
        Clrahbbufopt::from_bits(val)
    }
}
impl From<Clrahbbufopt> for u8 {
    #[inline(always)]
    fn from(val: Clrahbbufopt) -> u8 {
        Clrahbbufopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbrxbuf {
    #[doc = "No impact."]
    Val0 = 0x0,
    #[doc = "Enable clear operation."]
    Val1 = 0x01,
}
impl Clrahbrxbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbrxbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbrxbuf {
    #[inline(always)]
    fn from(val: u8) -> Clrahbrxbuf {
        Clrahbrxbuf::from_bits(val)
    }
}
impl From<Clrahbrxbuf> for u8 {
    #[inline(always)]
    fn from(val: Clrahbrxbuf) -> u8 {
        Clrahbrxbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbtxbuf {
    #[doc = "No impact."]
    Val0 = 0x0,
    #[doc = "Enable clear operation."]
    Val1 = 0x01,
}
impl Clrahbtxbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbtxbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbtxbuf {
    #[inline(always)]
    fn from(val: u8) -> Clrahbtxbuf {
        Clrahbtxbuf::from_bits(val)
    }
}
impl From<Clrahbtxbuf> for u8 {
    #[inline(always)]
    fn from(val: Clrahbtxbuf) -> u8 {
        Clrahbtxbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriprxf {
    #[doc = "No function."]
    Value0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in IP receive FIFO."]
    Value1 = 0x01,
}
impl Clriprxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriprxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriprxf {
    #[inline(always)]
    fn from(val: u8) -> Clriprxf {
        Clriprxf::from_bits(val)
    }
}
impl From<Clriprxf> for u8 {
    #[inline(always)]
    fn from(val: Clriprxf) -> u8 {
        Clriprxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriptxf {
    #[doc = "No function."]
    Value0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in the IP transmit FIFO."]
    Value1 = 0x01,
}
impl Clriptxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriptxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriptxf {
    #[inline(always)]
    fn from(val: u8) -> Clriptxf {
        Clriptxf::from_bits(val)
    }
}
impl From<Clriptxf> for u8 {
    #[inline(always)]
    fn from(val: Clriptxf) -> u8 {
        Clriptxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrlearnphase {
    #[doc = "No impact."]
    Val0 = 0x0,
    #[doc = "Reset sample clock phase selection to 0."]
    Val1 = 0x01,
}
impl Clrlearnphase {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrlearnphase {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrlearnphase {
    #[inline(always)]
    fn from(val: u8) -> Clrlearnphase {
        Clrlearnphase::from_bits(val)
    }
}
impl From<Clrlearnphase> for u8 {
    #[inline(always)]
    fn from(val: Clrlearnphase) -> u8 {
        Clrlearnphase::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Config {
    #[doc = "Fully pipelined."]
    Val0 = 0x0,
    #[doc = "Not fully pipelined."]
    Val1 = 0x01,
}
impl Config {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Config {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Config {
    #[inline(always)]
    fn from(val: u8) -> Config {
        Config::from_bits(val)
    }
}
impl From<Config> for u8 {
    #[inline(always)]
    fn from(val: Config) -> u8 {
        Config::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csintervalunit {
    #[doc = "1 serial clock cycle."]
    Val0 = 0x0,
    #[doc = "256 serial clock cycles."]
    Val1 = 0x01,
}
impl Csintervalunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csintervalunit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csintervalunit {
    #[inline(always)]
    fn from(val: u8) -> Csintervalunit {
        Csintervalunit::from_bits(val)
    }
}
impl From<Csintervalunit> for u8 {
    #[inline(always)]
    fn from(val: Csintervalunit) -> u8 {
        Csintervalunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datalearnfailen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Datalearnfailen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datalearnfailen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datalearnfailen {
    #[inline(always)]
    fn from(val: u8) -> Datalearnfailen {
        Datalearnfailen::from_bits(val)
    }
}
impl From<Datalearnfailen> for u8 {
    #[inline(always)]
    fn from(val: Datalearnfailen) -> u8 {
        Datalearnfailen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllen {
    #[doc = "Disable."]
    Value0 = 0x0,
    #[doc = "Enable."]
    Value1 = 0x01,
}
impl Dllen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllen {
    #[inline(always)]
    fn from(val: u8) -> Dllen {
        Dllen::from_bits(val)
    }
}
impl From<Dllen> for u8 {
    #[inline(always)]
    fn from(val: Dllen) -> u8 {
        Dllen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllreset {
    #[doc = "No function."]
    Value0 = 0x0,
    #[doc = "Force DLL reset."]
    Value1 = 0x01,
}
impl Dllreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllreset {
    #[inline(always)]
    fn from(val: u8) -> Dllreset {
        Dllreset::from_bits(val)
    }
}
impl From<Dllreset> for u8 {
    #[inline(always)]
    fn from(val: Dllreset) -> u8 {
        Dllreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doeopt {
    #[doc = "Data line output enable de-assertion is delayed for one cycle."]
    Val0 = 0x0,
    #[doc = "Data line output enable de-assertion is not delayed for one cycle."]
    Val1 = 0x01,
}
impl Doeopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doeopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doeopt {
    #[inline(always)]
    fn from(val: u8) -> Doeopt {
        Doeopt::from_bits(val)
    }
}
impl From<Doeopt> for u8 {
    #[inline(always)]
    fn from(val: Doeopt) -> u8 {
        Doeopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dqsposcap {
    #[doc = "DQS falling edge is used to capture the RX data."]
    Val0 = 0x0,
    #[doc = "DQS rising edge is used to capture the RX data."]
    Val1 = 0x01,
}
impl Dqsposcap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dqsposcap {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dqsposcap {
    #[inline(always)]
    fn from(val: u8) -> Dqsposcap {
        Dqsposcap::from_bits(val)
    }
}
impl From<Dqsposcap> for u8 {
    #[inline(always)]
    fn from(val: Dqsposcap) -> u8 {
        Dqsposcap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endcfg {
    #[doc = "64-bit Little Endian."]
    Val0 = 0x0,
    #[doc = "64-bit Big Endian."]
    Val1 = 0x01,
    #[doc = "32-bit Little Endian."]
    Val2 = 0x02,
    #[doc = "32-bit Big Endian."]
    Val3 = 0x03,
}
impl Endcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endcfg {
    #[inline(always)]
    fn from(val: u8) -> Endcfg {
        Endcfg::from_bits(val)
    }
}
impl From<Endcfg> for u8 {
    #[inline(always)]
    fn from(val: Endcfg) -> u8 {
        Endcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flashdqsopt {
    #[doc = "FlexSPI drives toggling SCLK output until it receives all flash read data bits. There will be an extra SCLK cycle after all read data bits are returned on SPI bus."]
    Val0 = 0x0,
    #[doc = "FlexSPI drives SCLK output according to read data size and pad number (no extra SCLK toggling). If there is DQS latency, FlexSPI receives less data than required. Never set this bit if there is DQS latency."]
    Val1 = 0x01,
}
impl Flashdqsopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flashdqsopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flashdqsopt {
    #[inline(always)]
    fn from(val: u8) -> Flashdqsopt {
        Flashdqsopt::from_bits(val)
    }
}
impl From<Flashdqsopt> for u8 {
    #[inline(always)]
    fn from(val: Flashdqsopt) -> u8 {
        Flashdqsopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsen {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl Hsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsen {
    #[inline(always)]
    fn from(val: u8) -> Hsen {
        Hsen::from_bits(val)
    }
}
impl From<Hsen> for u8 {
    #[inline(always)]
    fn from(val: Hsen) -> u8 {
        Hsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkahback {
    #[doc = "IP commands do not block AHB command acknowledgment."]
    Value0 = 0x0,
    #[doc = "IP commands block AHB command acknowledgment."]
    Value1 = 0x01,
}
impl Ipblkahback {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkahback {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkahback {
    #[inline(always)]
    fn from(val: u8) -> Ipblkahback {
        Ipblkahback::from_bits(val)
    }
}
impl From<Ipblkahback> for u8 {
    #[inline(always)]
    fn from(val: Ipblkahback) -> u8 {
        Ipblkahback::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkahbreq {
    #[doc = "IP commands do not block AHB command requests."]
    Value0 = 0x0,
    #[doc = "IP commands block AHB command requests."]
    Value1 = 0x01,
}
impl Ipblkahbreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkahbreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkahbreq {
    #[inline(always)]
    fn from(val: u8) -> Ipblkahbreq {
        Ipblkahbreq::from_bits(val)
    }
}
impl From<Ipblkahbreq> for u8 {
    #[inline(always)]
    fn from(val: Ipblkahbreq) -> u8 {
        Ipblkahbreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipblkallahb {
    #[doc = "IP commands only block AHB commands that affect the IPED region."]
    Value0 = 0x0,
    #[doc = "IP commands block all AHB commands."]
    Value1 = 0x01,
}
impl Ipblkallahb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipblkallahb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipblkallahb {
    #[inline(always)]
    fn from(val: u8) -> Ipblkallahb {
        Ipblkallahb::from_bits(val)
    }
}
impl From<Ipblkallahb> for u8 {
    #[inline(always)]
    fn from(val: Ipblkallahb) -> u8 {
        Ipblkallahb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddoneen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ipcmddoneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmddoneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmddoneen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmddoneen {
        Ipcmddoneen::from_bits(val)
    }
}
impl From<Ipcmddoneen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmddoneen) -> u8 {
        Ipcmddoneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderrcode {
    #[doc = "No error."]
    Val0 = 0x0,
    #[doc = "Write or read data size is not a multiple of two bytes while parallel mode is enabled (IPCR0\\[IPAREN\\] = 1). This error only occurs when FLSHCR4\\[WMOPT\\] = 1."]
    Val1 = 0x01,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence."]
    Val2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence."]
    Val3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence."]
    Val4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence."]
    Val5 = 0x05,
    #[doc = "Flash memory access start address exceeds entire flash address range (A1, A2, B1, and B2)."]
    Val6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout."]
    Val7 = 0x0e,
    #[doc = "Flash boundary crossed."]
    Val8 = 0x0f,
}
impl Ipcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderrcode {
        Ipcmderrcode::from_bits(val)
    }
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderrcode) -> u8 {
        Ipcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderren {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ipcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderren {
        Ipcmderren::from_bits(val)
    }
}
impl From<Ipcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderren) -> u8 {
        Ipcmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmdgeen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ipcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmdgeen {
        Ipcmdgeen::from_bits(val)
    }
}
impl From<Ipcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmdgeen) -> u8 {
        Ipcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmdsecurevioen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ipcmdsecurevioen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmdsecurevioen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmdsecurevioen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmdsecurevioen {
        Ipcmdsecurevioen::from_bits(val)
    }
}
impl From<Ipcmdsecurevioen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmdsecurevioen) -> u8 {
        Ipcmdsecurevioen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedEn {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl IpedEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedEn {
    #[inline(always)]
    fn from(val: u8) -> IpedEn {
        IpedEn::from_bits(val)
    }
}
impl From<IpedEn> for u8 {
    #[inline(always)]
    fn from(val: IpedEn) -> u8 {
        IpedEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedProtect {
    #[doc = "No restrictions."]
    Val0 = 0x0,
    #[doc = "Only privileged controllers can write IPED registers."]
    Val1 = 0x01,
}
impl IpedProtect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedProtect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedProtect {
    #[inline(always)]
    fn from(val: u8) -> IpedProtect {
        IpedProtect::from_bits(val)
    }
}
impl From<IpedProtect> for u8 {
    #[inline(always)]
    fn from(val: IpedProtect) -> u8 {
        IpedProtect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedSwreset {
    #[doc = "No function."]
    Val0 = 0x0,
    #[doc = "Aborts current decryption or encryption and waits for the next start operation."]
    Val1 = 0x01,
}
impl IpedSwreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedSwreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedSwreset {
    #[inline(always)]
    fn from(val: u8) -> IpedSwreset {
        IpedSwreset::from_bits(val)
    }
}
impl From<IpedSwreset> for u8 {
    #[inline(always)]
    fn from(val: IpedSwreset) -> u8 {
        IpedSwreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx0startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx0startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx0startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx0startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx0startAhbbuserrorDis {
        Ipedctx0startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx0startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx0startAhbbuserrorDis) -> u8 {
        Ipedctx0startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx1startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx1startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx1startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx1startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx1startAhbbuserrorDis {
        Ipedctx1startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx1startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx1startAhbbuserrorDis) -> u8 {
        Ipedctx1startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx2startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx2startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx2startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx2startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx2startAhbbuserrorDis {
        Ipedctx2startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx2startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx2startAhbbuserrorDis) -> u8 {
        Ipedctx2startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx3startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx3startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx3startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx3startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx3startAhbbuserrorDis {
        Ipedctx3startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx3startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx3startAhbbuserrorDis) -> u8 {
        Ipedctx3startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx4startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx4startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx4startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx4startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx4startAhbbuserrorDis {
        Ipedctx4startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx4startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx4startAhbbuserrorDis) -> u8 {
        Ipedctx4startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx5startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx5startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx5startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx5startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx5startAhbbuserrorDis {
        Ipedctx5startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx5startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx5startAhbbuserrorDis) -> u8 {
        Ipedctx5startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipedctx6startAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl Ipedctx6startAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipedctx6startAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipedctx6startAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> Ipedctx6startAhbbuserrorDis {
        Ipedctx6startAhbbuserrorDis::from_bits(val)
    }
}
impl From<Ipedctx6startAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: Ipedctx6startAhbbuserrorDis) -> u8 {
        Ipedctx6startAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iprxwaen {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Iprxwaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iprxwaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iprxwaen {
    #[inline(always)]
    fn from(val: u8) -> Iprxwaen {
        Iprxwaen::from_bits(val)
    }
}
impl From<Iprxwaen> for u8 {
    #[inline(always)]
    fn from(val: Iprxwaen) -> u8 {
        Iprxwaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iptxween {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Iptxween {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iptxween {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iptxween {
    #[inline(always)]
    fn from(val: u8) -> Iptxween {
        Iptxween::from_bits(val)
    }
}
impl From<Iptxween> for u8 {
    #[inline(always)]
    fn from(val: Iptxween) -> u8 {
        Iptxween::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpwrEn {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl IpwrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpwrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpwrEn {
    #[inline(always)]
    fn from(val: u8) -> IpwrEn {
        IpwrEn::from_bits(val)
    }
}
impl From<IpwrEn> for u8 {
    #[inline(always)]
    fn from(val: IpwrEn) -> u8 {
        IpwrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Learnphasegap {
    #[doc = "N = 2."]
    Val0 = 0x0,
    #[doc = "N = 3."]
    Val1 = 0x01,
    #[doc = "N = 4."]
    Val2 = 0x02,
    #[doc = "N = 5."]
    Val3 = 0x03,
}
impl Learnphasegap {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Learnphasegap {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Learnphasegap {
    #[inline(always)]
    fn from(val: u8) -> Learnphasegap {
        Learnphasegap::from_bits(val)
    }
}
impl From<Learnphasegap> for u8 {
    #[inline(always)]
    fn from(val: Learnphasegap) -> u8 {
        Learnphasegap::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "LUT is unlocked (LUTCR\\[UNLOCK\\] must be 1)."]
    Value0 = 0x0,
    #[doc = "LUT is locked and cannot be written."]
    Value1 = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "No impact."]
    Val0 = 0x0,
    #[doc = "Module disable."]
    Val1 = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ovrden {
    #[doc = "Disable."]
    Value0 = 0x0,
    #[doc = "Enable."]
    Value1 = 0x01,
}
impl Ovrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrden {
    #[inline(always)]
    fn from(val: u8) -> Ovrden {
        Ovrden::from_bits(val)
    }
}
impl From<Ovrden> for u8 {
    #[inline(always)]
    fn from(val: Ovrden) -> u8 {
        Ovrden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Phaserstopt {
    #[doc = "0.5 cycles."]
    Val0 = 0x0,
    #[doc = "1.5 cycles."]
    Val1 = 0x01,
    #[doc = "2.5 cycles."]
    Val2 = 0x02,
    #[doc = "3.5 cycles."]
    Val3 = 0x03,
}
impl Phaserstopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Phaserstopt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Phaserstopt {
    #[inline(always)]
    fn from(val: u8) -> Phaserstopt {
        Phaserstopt::from_bits(val)
    }
}
impl From<Phaserstopt> for u8 {
    #[inline(always)]
    fn from(val: Phaserstopt) -> u8 {
        Phaserstopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Protect {
    #[doc = "Not protected. All IPS controllers can access LUTCR and LUT memory."]
    Value0 = 0x0,
    #[doc = "Protected. Only secure IPS controller can change the value of LUTCR and write to LUT memory."]
    Value1 = 0x01,
}
impl Protect {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Protect {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Protect {
    #[inline(always)]
    fn from(val: u8) -> Protect {
        Protect::from_bits(val)
    }
}
impl From<Protect> for u8 {
    #[inline(always)]
    fn from(val: Protect) -> u8 {
        Protect::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readaddropt {
    #[doc = "AHB read burst start address alignment is limited when flash memory is accessed in parallel mode or flash is word-addressable."]
    Val0 = 0x0,
    #[doc = "AHB read burst start address alignment is not limited. FlexSPI fetches more data than the AHB burst requires for address alignment."]
    Val1 = 0x01,
}
impl Readaddropt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readaddropt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readaddropt {
    #[inline(always)]
    fn from(val: u8) -> Readaddropt {
        Readaddropt::from_bits(val)
    }
}
impl From<Readaddropt> for u8 {
    #[inline(always)]
    fn from(val: Readaddropt) -> u8 {
        Readaddropt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readszalign {
    #[doc = "Register settings such as PREFETCH_EN determine AHB read size."]
    Val0 = 0x0,
    #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching."]
    Val1 = 0x01,
}
impl Readszalign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readszalign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readszalign {
    #[inline(always)]
    fn from(val: u8) -> Readszalign {
        Readszalign::from_bits(val)
    }
}
impl From<Readszalign> for u8 {
    #[inline(always)]
    fn from(val: Readszalign) -> u8 {
        Readszalign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Remapen {
    #[doc = "HADDR REMAP Disabled."]
    Val0 = 0x0,
    #[doc = "HADDR REMAP Enabled."]
    Val1 = 0x01,
}
impl Remapen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Remapen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Remapen {
    #[inline(always)]
    fn from(val: u8) -> Remapen {
        Remapen::from_bits(val)
    }
}
impl From<Remapen> for u8 {
    #[inline(always)]
    fn from(val: Remapen) -> u8 {
        Remapen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resumedisable {
    #[doc = "Suspended AHB read prefetch resumes when AHB is IDLE."]
    Val0 = 0x0,
    #[doc = "Suspended AHB read prefetch does not resume once aborted."]
    Val1 = 0x01,
}
impl Resumedisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resumedisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resumedisable {
    #[inline(always)]
    fn from(val: u8) -> Resumedisable {
        Resumedisable::from_bits(val)
    }
}
impl From<Resumedisable> for u8 {
    #[inline(always)]
    fn from(val: Resumedisable) -> u8 {
        Resumedisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwdsoeopt {
    #[doc = "RWDS output enable de-assertion is delayed for one cycle."]
    Val0 = 0x0,
    #[doc = "RWDS output enable de-assertion is not delayed for one cycle."]
    Val1 = 0x01,
}
impl Rwdsoeopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwdsoeopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwdsoeopt {
    #[inline(always)]
    fn from(val: u8) -> Rwdsoeopt {
        Rwdsoeopt::from_bits(val)
    }
}
impl From<Rwdsoeopt> for u8 {
    #[inline(always)]
    fn from(val: Rwdsoeopt) -> u8 {
        Rwdsoeopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClkSrcDiff {
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A and Port B. MCR2\\[RXCLKSRC_B\\] is ignored and MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of both ports A and B."]
    Value0 = 0x0,
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A, and MCR2\\[RXCLKSRC_B\\] for Port B. MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of port A (A_SCLK) and MCR2\\[RXCLKSRC_B\\] selects the Sample Clock source for Flash Reading of port B (B_SCLK)."]
    Value1 = 0x01,
}
impl RxClkSrcDiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClkSrcDiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClkSrcDiff {
    #[inline(always)]
    fn from(val: u8) -> RxClkSrcDiff {
        RxClkSrcDiff::from_bits(val)
    }
}
impl From<RxClkSrcDiff> for u8 {
    #[inline(always)]
    fn from(val: RxClkSrcDiff) -> u8 {
        RxClkSrcDiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxclksrc {
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back internally."]
    Val0 = 0x0,
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back from DQS pad."]
    Val1 = 0x01,
    #[doc = "SCLK output clock and looped back from SCLK pad."]
    Val2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad."]
    Val3 = 0x03,
}
impl Rxclksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxclksrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxclksrc {
    #[inline(always)]
    fn from(val: u8) -> Rxclksrc {
        Rxclksrc::from_bits(val)
    }
}
impl From<Rxclksrc> for u8 {
    #[inline(always)]
    fn from(val: Rxclksrc) -> u8 {
        Rxclksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxclksrcB {
    #[doc = "Dummy read strobe that FlexSPI generates, looped back internally."]
    Val0 = 0x0,
    #[doc = "Dummy read strobe that FlexSPI generates, looped back from DQS pad."]
    Val1 = 0x01,
    #[doc = "SCLK output clock and looped back from SCLK pad."]
    Val2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad."]
    Val3 = 0x03,
}
impl RxclksrcB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxclksrcB {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxclksrcB {
    #[inline(always)]
    fn from(val: u8) -> RxclksrcB {
        RxclksrcB::from_bits(val)
    }
}
impl From<RxclksrcB> for u8 {
    #[inline(always)]
    fn from(val: RxclksrcB) -> u8 {
        RxclksrcB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdelayopt {
    #[doc = "FlexSPI will sample RX data lines 4 cycles after SCLK edge."]
    Val0 = 0x0,
    #[doc = "FlexSPI will sample RX data lines 5 cycles after SCLK edge."]
    Val1 = 0x01,
    #[doc = "FlexSPI will sample RX data lines 6 cycles after SCLK edge."]
    Val2 = 0x02,
    #[doc = "FlexSPI will sample RX data lines 7 cycles after SCLK edge."]
    Val3 = 0x03,
}
impl Rxdelayopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdelayopt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdelayopt {
    #[inline(always)]
    fn from(val: u8) -> Rxdelayopt {
        Rxdelayopt::from_bits(val)
    }
}
impl From<Rxdelayopt> for u8 {
    #[inline(always)]
    fn from(val: Rxdelayopt) -> u8 {
        Rxdelayopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdmaen {
    #[doc = "Disabled. The processor reads the FIFO."]
    Val0 = 0x0,
    #[doc = "Enabled. DMA reads the FIFO."]
    Val1 = 0x01,
}
impl Rxdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdmaen {
    #[inline(always)]
    fn from(val: u8) -> Rxdmaen {
        Rxdmaen::from_bits(val)
    }
}
impl From<Rxdmaen> for u8 {
    #[inline(always)]
    fn from(val: Rxdmaen) -> u8 {
        Rxdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Samedeviceen {
    #[doc = "In Individual mode, FLSHA1CRx and FLSHA2CRx, FLSHB1CRx and FLSHB2CRx settings are applied to Flash A1, A2, B1, B2 separately. In Parallel mode, FLSHA1CRx register setting is applied to Flash A1 and B1, FLSHA2CRx register setting is applied to Flash A2 and B2. FLSHB1CRx and FLSHB2CRx register settings are ignored."]
    IndividualParallel = 0x0,
    #[doc = "FLSHA1CR0, FLSHA1CR1, and FLSHA1CR2 register settings are applied to Flash A1, A2, B1, B2. FLSHA2CRx, FLSHB1CRx, and FLSHB2CRx settings are ignored."]
    Enable = 0x01,
}
impl Samedeviceen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Samedeviceen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Samedeviceen {
    #[inline(always)]
    fn from(val: u8) -> Samedeviceen {
        Samedeviceen::from_bits(val)
    }
}
impl From<Samedeviceen> for u8 {
    #[inline(always)]
    fn from(val: Samedeviceen) -> u8 {
        Samedeviceen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sck2opt {
    #[doc = "SCLK2 output clock toggles for READ and LEARN instructions only."]
    Val0 = 0x0,
    #[doc = "SCLK2 output clock toggles for entire sequence."]
    Val1 = 0x01,
}
impl Sck2opt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sck2opt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sck2opt {
    #[inline(always)]
    fn from(val: u8) -> Sck2opt {
        Sck2opt::from_bits(val)
    }
}
impl From<Sck2opt> for u8 {
    #[inline(always)]
    fn from(val: Sck2opt) -> u8 {
        Sck2opt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckbdiffopt {
    #[doc = "Use B_SCLK pad as port B SCLK clock output. Port B flash memory access is available."]
    Val1 = 0x0,
    #[doc = "Use B_SCLK pad as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash memory access is not available."]
    Val0 = 0x01,
}
impl Sckbdiffopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckbdiffopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckbdiffopt {
    #[inline(always)]
    fn from(val: u8) -> Sckbdiffopt {
        Sckbdiffopt::from_bits(val)
    }
}
impl From<Sckbdiffopt> for u8 {
    #[inline(always)]
    fn from(val: Sckbdiffopt) -> u8 {
        Sckbdiffopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckrstdisabled {
    #[doc = "SCLK is reset to default level before PCS signal is deasserted."]
    Val0 = 0x0,
    #[doc = "SCLK is reset to default level after PCS signal is deasserted."]
    Val1 = 0x01,
}
impl Sckrstdisabled {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckrstdisabled {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckrstdisabled {
    #[inline(always)]
    fn from(val: u8) -> Sckrstdisabled {
        Sckrstdisabled::from_bits(val)
    }
}
impl From<Sckrstdisabled> for u8 {
    #[inline(always)]
    fn from(val: Sckrstdisabled) -> u8 {
        Sckrstdisabled::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbyrden {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Sckstopbyrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbyrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbyrden {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbyrden {
        Sckstopbyrden::from_bits(val)
    }
}
impl From<Sckstopbyrden> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbyrden) -> u8 {
        Sckstopbyrden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbywren {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Sckstopbywren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbywren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbywren {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbywren {
        Sckstopbywren::from_bits(val)
    }
}
impl From<Sckstopbywren> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbywren) -> u8 {
        Sckstopbywren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqidle {
    #[doc = "Not idle."]
    Value0 = 0x0,
    #[doc = "Idle."]
    Value1 = 0x01,
}
impl Seqidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqidle {
    #[inline(always)]
    fn from(val: u8) -> Seqidle {
        Seqidle::from_bits(val)
    }
}
impl From<Seqidle> for u8 {
    #[inline(always)]
    fn from(val: Seqidle) -> u8 {
        Seqidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqtimeouten {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Seqtimeouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqtimeouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqtimeouten {
    #[inline(always)]
    fn from(val: u8) -> Seqtimeouten {
        Seqtimeouten::from_bits(val)
    }
}
impl From<Seqtimeouten> for u8 {
    #[inline(always)]
    fn from(val: Seqtimeouten) -> u8 {
        Seqtimeouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Serclkdiv {
    #[doc = "Divided by 1."]
    Val0 = 0x0,
    #[doc = "Divided by 2."]
    Val1 = 0x01,
    #[doc = "Divided by 3."]
    Val2 = 0x02,
    #[doc = "Divided by 4."]
    Val3 = 0x03,
    #[doc = "Divided by 5."]
    Val4 = 0x04,
    #[doc = "Divided by 6."]
    Val5 = 0x05,
    #[doc = "Divided by 7."]
    Val6 = 0x06,
    #[doc = "Divided by 8."]
    Val7 = 0x07,
}
impl Serclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Serclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Serclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Serclkdiv {
        Serclkdiv::from_bits(val)
    }
}
impl From<Serclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Serclkdiv) -> u8 {
        Serclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swreset {
    #[doc = "No impact."]
    Val0 = 0x0,
    #[doc = "Software reset."]
    Val1 = 0x01,
}
impl Swreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swreset {
    #[inline(always)]
    fn from(val: u8) -> Swreset {
        Swreset::from_bits(val)
    }
}
impl From<Swreset> for u8 {
    #[inline(always)]
    fn from(val: Swreset) -> u8 {
        Swreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg {
    #[doc = "No action."]
    Value0 = 0x0,
    #[doc = "Start the IP command that the IPCR0 and IPCR1 registers define."]
    Value1 = 0x01,
}
impl Trg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg {
    #[inline(always)]
    fn from(val: u8) -> Trg {
        Trg::from_bits(val)
    }
}
impl From<Trg> for u8 {
    #[inline(always)]
    fn from(val: Trg) -> u8 {
        Trg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstmd {
    #[doc = "Disable. Sequence execution is started when IP or AHB command triggers it as normal."]
    Val0 = 0x0,
    #[doc = "Enable. Sequence execution is not started after IP or AHB command triggers it. It remains blocked until ipt_tester_trigger input (another pin for test) is high. A high pulse of serial clock domain is required."]
    Val1 = 0x01,
}
impl Tstmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstmd {
    #[inline(always)]
    fn from(val: u8) -> Tstmd {
        Tstmd::from_bits(val)
    }
}
impl From<Tstmd> for u8 {
    #[inline(always)]
    fn from(val: Tstmd) -> u8 {
        Tstmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdmaen {
    #[doc = "Processor."]
    Val0 = 0x0,
    #[doc = "DMA."]
    Val1 = 0x01,
}
impl Txdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdmaen {
    #[inline(always)]
    fn from(val: u8) -> Txdmaen {
        Txdmaen::from_bits(val)
    }
}
impl From<Txdmaen> for u8 {
    #[inline(always)]
    fn from(val: Txdmaen) -> u8 {
        Txdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "LUT is locked (LUTCR\\[LOCK\\] must be 1)."]
    Value0 = 0x0,
    #[doc = "LUT is unlocked and can be written."]
    Value1 = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wa {
    #[doc = "Byte-addressable."]
    Value0 = 0x0,
    #[doc = "Word-addressable."]
    Value1 = 0x01,
}
impl Wa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wa {
    #[inline(always)]
    fn from(val: u8) -> Wa {
        Wa::from_bits(val)
    }
}
impl From<Wa> for u8 {
    #[inline(always)]
    fn from(val: Wa) -> u8 {
        Wa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmena {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    Val0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    Val1 = 0x01,
}
impl Wmena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmena {
    #[inline(always)]
    fn from(val: u8) -> Wmena {
        Wmena::from_bits(val)
    }
}
impl From<Wmena> for u8 {
    #[inline(always)]
    fn from(val: Wmena) -> u8 {
        Wmena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmenb {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    Val0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    Val1 = 0x01,
}
impl Wmenb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmenb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmenb {
    #[inline(always)]
    fn from(val: u8) -> Wmenb {
        Wmenb::from_bits(val)
    }
}
impl From<Wmenb> for u8 {
    #[inline(always)]
    fn from(val: Wmenb) -> u8 {
        Wmenb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmopt1b {
    #[doc = "DQSB used as Write Mask when writing to external device."]
    Val0 = 0x0,
    #[doc = "DQSB not used as Write Mask when writing to external device."]
    Val1 = 0x01,
}
impl Wmopt1b {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmopt1b {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmopt1b {
    #[inline(always)]
    fn from(val: u8) -> Wmopt1b {
        Wmopt1b::from_bits(val)
    }
}
impl From<Wmopt1b> for u8 {
    #[inline(always)]
    fn from(val: Wmopt1b) -> u8 {
        Wmopt1b::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmopt2b {
    #[doc = "DQSB is used as Write Mask when writing to external device."]
    Val0 = 0x0,
    #[doc = "DQSB is not used as Write Mask when writing to external device."]
    Val1 = 0x01,
}
impl Wmopt2b {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmopt2b {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmopt2b {
    #[inline(always)]
    fn from(val: u8) -> Wmopt2b {
        Wmopt2b::from_bits(val)
    }
}
impl From<Wmopt2b> for u8 {
    #[inline(always)]
    fn from(val: Wmopt2b) -> u8 {
        Wmopt2b::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmoptdiff {
    #[doc = "Write Mask behaves same on PORTA and PORTB, controlled by WMOPT1 and WMOPT2."]
    Val0 = 0x0,
    #[doc = "Write Mask behaves differently. WMOPT1 and WMOPT2 control PORTA. WMOPT1B and WMOPT2B control PORTB."]
    Val1 = 0x01,
}
impl Wmoptdiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmoptdiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmoptdiff {
    #[inline(always)]
    fn from(val: u8) -> Wmoptdiff {
        Wmoptdiff::from_bits(val)
    }
}
impl From<Wmoptdiff> for u8 {
    #[inline(always)]
    fn from(val: Wmoptdiff) -> u8 {
        Wmoptdiff::to_bits(val)
    }
}
