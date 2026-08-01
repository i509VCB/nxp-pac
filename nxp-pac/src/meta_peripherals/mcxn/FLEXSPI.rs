#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "FlexSPI."]
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
    #[doc = "AHB Receive Buffer Control 0."]
    #[inline(always)]
    pub const fn ahbrxbufcr0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ahbrxbufcr0, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _)
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
    #[doc = "Flash Control 4."]
    #[inline(always)]
    pub const fn flshcr4(self) -> crate::pac::common::Reg<Flshcr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
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
    #[doc = "Receive Buffer Start Address of Region."]
    #[inline(always)]
    pub const fn ahbbufregionstart(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ahbbufregionstart, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize + n * 8usize) as _)
        }
    }
    #[doc = "Receive Buffer Region End Address."]
    #[inline(always)]
    pub const fn ahbbufregionend(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ahbbufregionend, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize + n * 8usize) as _)
        }
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
    #[doc = "IPED Context IV0."]
    #[inline(always)]
    pub const fn ipedctxiv0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxiv0, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize + n * 32usize) as _)
        }
    }
    #[doc = "IPED Context IV1."]
    #[inline(always)]
    pub const fn ipedctxiv1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxiv1, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize + n * 32usize) as _)
        }
    }
    #[doc = "Start Address of Region."]
    #[inline(always)]
    pub const fn ipedctxstart(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxstart, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize + n * 32usize) as _)
        }
    }
    #[doc = "End Address of Region."]
    #[inline(always)]
    pub const fn ipedctxend(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxend, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize + n * 32usize) as _)
        }
    }
    #[doc = "IPED Context Additional Authenticated Data0."]
    #[inline(always)]
    pub const fn ipedctxaad0(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxaad0, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize + n * 32usize) as _)
        }
    }
    #[doc = "IPED Context Additional Authenticated Data1."]
    #[inline(always)]
    pub const fn ipedctxaad1(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ipedctxaad1, crate::pac::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize + n * 32usize) as _)
        }
    }
}
#[doc = "Receive Buffer Region End Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionend(pub u32);
impl Ahbbufregionend {
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
impl Default for Ahbbufregionend {
    #[inline(always)]
    fn default() -> Ahbbufregionend {
        Ahbbufregionend(0)
    }
}
impl core::fmt::Debug for Ahbbufregionend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionend")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionend {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "Receive Buffer Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbbufregionstart(pub u32);
impl Ahbbufregionstart {
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
impl Default for Ahbbufregionstart {
    #[inline(always)]
    fn default() -> Ahbbufregionstart {
        Ahbbufregionstart(0)
    }
}
impl core::fmt::Debug for Ahbbufregionstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbbufregionstart")
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbbufregionstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbbufregionstart {{ start_address: {=u32:?} }}",
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
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "AHB Memory-Mapped Flash Base Address."]
    #[inline(always)]
    pub const fn set_aflashbase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
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
#[doc = "AHB Receive Buffer Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbrxbufcr0(pub u32);
impl Ahbrxbufcr0 {
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
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "AHB Controller ID."]
    #[inline(always)]
    pub const fn set_mstrid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
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
    pub const fn regionen(&self) -> Ahbrxbufcr0Regionen {
        let val = (self.0 >> 30usize) & 0x01;
        Ahbrxbufcr0Regionen::from_bits(val as u8)
    }
    #[doc = "AHB Receive Buffer Address Region Enable."]
    #[inline(always)]
    pub const fn set_regionen(&mut self, val: Ahbrxbufcr0Regionen) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn prefetchen(&self) -> Ahbrxbufcr0Prefetchen {
        let val = (self.0 >> 31usize) & 0x01;
        Ahbrxbufcr0Prefetchen::from_bits(val as u8)
    }
    #[doc = "AHB Read Prefetch Enable."]
    #[inline(always)]
    pub const fn set_prefetchen(&mut self, val: Ahbrxbufcr0Prefetchen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbrxbufcr0 {
    #[inline(always)]
    fn default() -> Ahbrxbufcr0 {
        Ahbrxbufcr0(0)
    }
}
impl core::fmt::Debug for Ahbrxbufcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbrxbufcr0")
            .field("bufsz", &self.bufsz())
            .field("mstrid", &self.mstrid())
            .field("priority", &self.priority())
            .field("regionen", &self.regionen())
            .field("prefetchen", &self.prefetchen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbrxbufcr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbrxbufcr0 {{ bufsz: {=u8:?}, mstrid: {=u8:?}, priority: {=u8:?}, regionen: {:?}, prefetchen: {:?} }}",
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
            .field("slvdlytarget", &self.slvdlytarget())
            .field("ovrden", &self.ovrden())
            .field("ovrdval", &self.ovrdval())
            .field("refphasegap", &self.refphasegap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dllcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dllcr {{ dllen: {:?}, dllreset: {:?}, slvdlytarget: {=u8:?}, ovrden: {:?}, ovrdval: {=u8:?}, refphasegap: {=u8:?} }}",
            self.dllen(),
            self.dllreset(),
            self.slvdlytarget(),
            self.ovrden(),
            self.ovrdval(),
            self.refphasegap()
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
    #[doc = "AHB Write Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
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
    #[doc = "AHB Write Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flsha2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flsha2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
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
    #[doc = "AHB Write Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb1cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb1cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
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
    #[doc = "AHB Write Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitwren(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Write Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitwren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn splitrden(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read Access Split Function Enable."]
    #[inline(always)]
    pub const fn set_splitrden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("splitwren", &self.splitwren())
            .field("splitrden", &self.splitrden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshb2cr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshb2cr0 {{ flshsz: {=u32:?}, addrshift: {=bool:?}, splitwren: {=bool:?}, splitrden: {=bool:?} }}",
            self.flshsz(),
            self.addrshift(),
            self.splitwren(),
            self.splitrden()
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flshcr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flshcr4 {{ wmopt1: {=bool:?}, wmena: {:?}, wmenb: {:?} }}",
            self.wmopt1(),
            self.wmena(),
            self.wmenb()
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
            .field("addrstart", &self.addrstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Haddrstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Haddrstart {{ remapen: {:?}, addrstart: {=u32:?} }}",
            self.remapen(),
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
    #[doc = "AHB Read GCM Error Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmerren(&self) -> Ahbgcmerren {
        let val = (self.0 >> 17usize) & 0x01;
        Ahbgcmerren::from_bits(val as u8)
    }
    #[doc = "AHB Read GCM Error Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ahbgcmerren(&mut self, val: Ahbgcmerren) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
            .field("ahbgcmerren", &self.ahbgcmerren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Inten {{ ipcmddoneen: {:?}, ipcmdgeen: {:?}, ahbcmdgeen: {:?}, ipcmderren: {:?}, ahbcmderren: {:?}, iprxwaen: {:?}, iptxween: {:?}, datalearnfailen: {:?}, sckstopbyrden: {:?}, sckstopbywren: {:?}, ahbbustimeouten: {:?}, seqtimeouten: {:?}, ipcmdsecurevioen: {:?}, ahbgcmerren: {:?} }}",
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
            self.ipcmdsecurevioen(),
            self.ahbgcmerren()
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
    #[doc = "AHB Read GCM Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmerr(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Read GCM Error."]
    #[inline(always)]
    pub const fn set_ahbgcmerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
            .field("ahbgcmerr", &self.ahbgcmerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Intr {{ ipcmddone: {=bool:?}, ipcmdge: {=bool:?}, ahbcmdge: {=bool:?}, ipcmderr: {=bool:?}, ahbcmderr: {=bool:?}, iprxwa: {=bool:?}, iptxwe: {=bool:?}, datalearnfail: {=bool:?}, sckstopbyrd: {=bool:?}, sckstopbywr: {=bool:?}, ahbbustimeout: {=bool:?}, seqtimeout: {=bool:?}, ipcmdsecurevio: {=bool:?}, ahbgcmerr: {=bool:?} }}",
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
            self.ipcmdsecurevio(),
            self.ahbgcmerr()
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
    #[doc = "IP Write GCM Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgcmwr(&self) -> Ipgcmwr {
        let val = (self.0 >> 6usize) & 0x01;
        Ipgcmwr::from_bits(val as u8)
    }
    #[doc = "IP Write GCM Mode Enable."]
    #[inline(always)]
    pub const fn set_ipgcmwr(&mut self, val: Ipgcmwr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AHB Write IPED GCM Mode Encryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahgcmwr(&self) -> Ahgcmwr {
        let val = (self.0 >> 7usize) & 0x01;
        Ahgcmwr::from_bits(val as u8)
    }
    #[doc = "AHB Write IPED GCM Mode Encryption Enable."]
    #[inline(always)]
    pub const fn set_ahgcmwr(&mut self, val: Ahgcmwr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AHB Read IPED GCM Mode Decryption Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbgcmrd(&self) -> Ahbgcmrd {
        let val = (self.0 >> 8usize) & 0x01;
        Ahbgcmrd::from_bits(val as u8)
    }
    #[doc = "AHB Read IPED GCM Mode Decryption Enable."]
    #[inline(always)]
    pub const fn set_ahbgcmrd(&mut self, val: Ahbgcmrd) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("ipgcmwr", &self.ipgcmwr())
            .field("ahgcmwr", &self.ahgcmwr())
            .field("ahbgcmrd", &self.ahbgcmrd())
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
            "Ipedctrl {{ config: {:?}, iped_en: {:?}, ipwr_en: {:?}, ahbwr_en: {:?}, ahbrd_en: {:?}, ipgcmwr: {:?}, ahgcmwr: {:?}, ahbgcmrd: {:?}, iped_protect: {:?}, iped_swreset: {:?} }}",
            self.config(),
            self.iped_en(),
            self.ipwr_en(),
            self.ahbwr_en(),
            self.ahbrd_en(),
            self.ipgcmwr(),
            self.ahgcmwr(),
            self.ahbgcmrd(),
            self.iped_protect(),
            self.iped_swreset()
        )
    }
}
#[doc = "IPED Context Additional Authenticated Data0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxaad0(pub u32);
impl Ipedctxaad0 {
    #[doc = "CTX AAD."]
    #[must_use]
    #[inline(always)]
    pub const fn aad0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD."]
    #[inline(always)]
    pub const fn set_aad0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctxaad0 {
    #[inline(always)]
    fn default() -> Ipedctxaad0 {
        Ipedctxaad0(0)
    }
}
impl core::fmt::Debug for Ipedctxaad0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxaad0")
            .field("aad0", &self.aad0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxaad0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctxaad0 {{ aad0: {=u32:?} }}", self.aad0())
    }
}
#[doc = "IPED Context Additional Authenticated Data1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxaad1(pub u32);
impl Ipedctxaad1 {
    #[doc = "CTX AAD."]
    #[must_use]
    #[inline(always)]
    pub const fn aad1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "CTX AAD."]
    #[inline(always)]
    pub const fn set_aad1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctxaad1 {
    #[inline(always)]
    fn default() -> Ipedctxaad1 {
        Ipedctxaad1(0)
    }
}
impl core::fmt::Debug for Ipedctxaad1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxaad1")
            .field("aad1", &self.aad1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxaad1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctxaad1 {{ aad1: {=u32:?} }}", self.aad1())
    }
}
#[doc = "IPED context control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl0(pub u32);
impl Ipedctxctrl0 {
    #[doc = "Context Register Freeze for Region."]
    #[must_use]
    #[inline(always)]
    pub const fn freeze0(&self, n: usize) -> u8 {
        assert!(n < 7usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region."]
    #[inline(always)]
    pub const fn set_freeze0(&mut self, n: usize, val: u8) {
        assert!(n < 7usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
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
            .field("freeze0[0]", &self.freeze0(0usize))
            .field("freeze0[1]", &self.freeze0(1usize))
            .field("freeze0[2]", &self.freeze0(2usize))
            .field("freeze0[3]", &self.freeze0(3usize))
            .field("freeze0[4]", &self.freeze0(4usize))
            .field("freeze0[5]", &self.freeze0(5usize))
            .field("freeze0[6]", &self.freeze0(6usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl0 {{ freeze0[0]: {=u8:?}, freeze0[1]: {=u8:?}, freeze0[2]: {=u8:?}, freeze0[3]: {=u8:?}, freeze0[4]: {=u8:?}, freeze0[5]: {=u8:?}, freeze0[6]: {=u8:?} }}",
            self.freeze0(0usize),
            self.freeze0(1usize),
            self.freeze0(2usize),
            self.freeze0(3usize),
            self.freeze0(4usize),
            self.freeze0(5usize),
            self.freeze0(6usize)
        )
    }
}
#[doc = "IPED context control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxctrl1(pub u32);
impl Ipedctxctrl1 {
    #[doc = "Context Register Freeze for Region."]
    #[must_use]
    #[inline(always)]
    pub const fn freeze1(&self, n: usize) -> u8 {
        assert!(n < 7usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x03;
        val as u8
    }
    #[doc = "Context Register Freeze for Region."]
    #[inline(always)]
    pub const fn set_freeze1(&mut self, n: usize, val: u8) {
        assert!(n < 7usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
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
            .field("freeze1[0]", &self.freeze1(0usize))
            .field("freeze1[1]", &self.freeze1(1usize))
            .field("freeze1[2]", &self.freeze1(2usize))
            .field("freeze1[3]", &self.freeze1(3usize))
            .field("freeze1[4]", &self.freeze1(4usize))
            .field("freeze1[5]", &self.freeze1(5usize))
            .field("freeze1[6]", &self.freeze1(6usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxctrl1 {{ freeze1[0]: {=u8:?}, freeze1[1]: {=u8:?}, freeze1[2]: {=u8:?}, freeze1[3]: {=u8:?}, freeze1[4]: {=u8:?}, freeze1[5]: {=u8:?}, freeze1[6]: {=u8:?} }}",
            self.freeze1(0usize),
            self.freeze1(1usize),
            self.freeze1(2usize),
            self.freeze1(3usize),
            self.freeze1(4usize),
            self.freeze1(5usize),
            self.freeze1(6usize)
        )
    }
}
#[doc = "End Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxend(pub u32);
impl Ipedctxend {
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
impl Default for Ipedctxend {
    #[inline(always)]
    fn default() -> Ipedctxend {
        Ipedctxend(0)
    }
}
impl core::fmt::Debug for Ipedctxend {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxend")
            .field("end_address", &self.end_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxend {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxend {{ end_address: {=u32:?} }}",
            self.end_address()
        )
    }
}
#[doc = "IPED Context IV0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxiv0(pub u32);
impl Ipedctxiv0 {
    #[doc = "Lowest 32 bits of IV for region."]
    #[must_use]
    #[inline(always)]
    pub const fn iv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Lowest 32 bits of IV for region."]
    #[inline(always)]
    pub const fn set_iv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctxiv0 {
    #[inline(always)]
    fn default() -> Ipedctxiv0 {
        Ipedctxiv0(0)
    }
}
impl core::fmt::Debug for Ipedctxiv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxiv0")
            .field("iv0", &self.iv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxiv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctxiv0 {{ iv0: {=u32:?} }}", self.iv0())
    }
}
#[doc = "IPED Context IV1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxiv1(pub u32);
impl Ipedctxiv1 {
    #[doc = "Highest 32 bits of IV for region."]
    #[must_use]
    #[inline(always)]
    pub const fn iv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Highest 32 bits of IV for region."]
    #[inline(always)]
    pub const fn set_iv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ipedctxiv1 {
    #[inline(always)]
    fn default() -> Ipedctxiv1 {
        Ipedctxiv1(0)
    }
}
impl core::fmt::Debug for Ipedctxiv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxiv1")
            .field("iv1", &self.iv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxiv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ipedctxiv1 {{ iv1: {=u32:?} }}", self.iv1())
    }
}
#[doc = "Start Address of Region."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipedctxstart(pub u32);
impl Ipedctxstart {
    #[doc = "GCM Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn gcm(&self) -> IpedctxstartGcm {
        let val = (self.0 >> 0usize) & 0x01;
        IpedctxstartGcm::from_bits(val as u8)
    }
    #[doc = "GCM Mode Enable."]
    #[inline(always)]
    pub const fn set_gcm(&mut self, val: IpedctxstartGcm) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "AHB Bus Error Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn ahbbuserror_dis(&self) -> IpedctxstartAhbbuserrorDis {
        let val = (self.0 >> 1usize) & 0x01;
        IpedctxstartAhbbuserrorDis::from_bits(val as u8)
    }
    #[doc = "AHB Bus Error Disable."]
    #[inline(always)]
    pub const fn set_ahbbuserror_dis(&mut self, val: IpedctxstartAhbbuserrorDis) {
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
impl Default for Ipedctxstart {
    #[inline(always)]
    fn default() -> Ipedctxstart {
        Ipedctxstart(0)
    }
}
impl core::fmt::Debug for Ipedctxstart {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipedctxstart")
            .field("gcm", &self.gcm())
            .field("ahbbuserror_dis", &self.ahbbuserror_dis())
            .field("start_address", &self.start_address())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipedctxstart {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ipedctxstart {{ gcm: {:?}, ahbbuserror_dis: {:?}, start_address: {=u32:?} }}",
            self.gcm(),
            self.ahbbuserror_dis(),
            self.start_address()
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
            "Mcr0 {{ swreset: {:?}, mdis: {:?}, rxclksrc: {:?}, ardfen: {:?}, atdfen: {:?}, serclkdiv: {:?}, hsen: {:?}, dozeen: {:?}, combinationen: {=bool:?}, sckfreerunen: {=bool:?}, learnen: {=bool:?}, ipgrantwait: {=u8:?}, ahbgrantwait: {=u8:?} }}",
            self.swreset(),
            self.mdis(),
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
            .field("clrahbbufopt", &self.clrahbbufopt())
            .field("clrlearnphase", &self.clrlearnphase())
            .field("samedeviceen", &self.samedeviceen())
            .field("sckbdiffopt", &self.sckbdiffopt())
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
            "Mcr2 {{ clrahbbufopt: {:?}, clrlearnphase: {:?}, samedeviceen: {:?}, sckbdiffopt: {:?}, rxclksrc_b: {:?}, rx_clk_src_diff: {:?}, resumewait: {=u8:?} }}",
            self.clrahbbufopt(),
            self.clrlearnphase(),
            self.samedeviceen(),
            self.sckbdiffopt(),
            self.rxclksrc_b(),
            self.rx_clk_src_diff(),
            self.resumewait()
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
    _RESERVED_1 = 0x01,
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
pub enum Ahbgcmerren {
    #[doc = "Disable interrupt or no impact."]
    Value0 = 0x0,
    #[doc = "Enable interrupt."]
    Value1 = 0x01,
}
impl Ahbgcmerren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbgcmerren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbgcmerren {
    #[inline(always)]
    fn from(val: u8) -> Ahbgcmerren {
        Ahbgcmerren::from_bits(val)
    }
}
impl From<Ahbgcmerren> for u8 {
    #[inline(always)]
    fn from(val: Ahbgcmerren) -> u8 {
        Ahbgcmerren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbgcmrd {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl Ahbgcmrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbgcmrd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbgcmrd {
    #[inline(always)]
    fn from(val: u8) -> Ahbgcmrd {
        Ahbgcmrd::from_bits(val)
    }
}
impl From<Ahbgcmrd> for u8 {
    #[inline(always)]
    fn from(val: Ahbgcmrd) -> u8 {
        Ahbgcmrd::to_bits(val)
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
pub enum Ahbrxbufcr0Prefetchen {
    #[doc = "Disabled."]
    Value0 = 0x0,
    #[doc = "Enabled when is enabled."]
    Value1 = 0x01,
}
impl Ahbrxbufcr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbufcr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbufcr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbufcr0Prefetchen {
        Ahbrxbufcr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbufcr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbufcr0Prefetchen) -> u8 {
        Ahbrxbufcr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbufcr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    Value0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    Value1 = 0x01,
}
impl Ahbrxbufcr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbufcr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbufcr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbufcr0Regionen {
        Ahbrxbufcr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbufcr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbufcr0Regionen) -> u8 {
        Ahbrxbufcr0Regionen::to_bits(val)
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
pub enum Ahgcmwr {
    #[doc = "Disable."]
    Val0 = 0x0,
    #[doc = "Enable."]
    Val1 = 0x01,
}
impl Ahgcmwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahgcmwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahgcmwr {
    #[inline(always)]
    fn from(val: u8) -> Ahgcmwr {
        Ahgcmwr::from_bits(val)
    }
}
impl From<Ahgcmwr> for u8 {
    #[inline(always)]
    fn from(val: Ahgcmwr) -> u8 {
        Ahgcmwr::to_bits(val)
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
    #[doc = "Disabled. For all AHB write accesses (bufferable or nonbufferable), FlexSPI returns AHB Bus Ready after transmitting all data and finishing command."]
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
    _RESERVED_1 = 0x01,
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
pub enum IpedctxstartAhbbuserrorDis {
    #[doc = "AHB bus errors enabled."]
    Value0 = 0x0,
    #[doc = "AHB bus errors disabled."]
    Value1 = 0x01,
}
impl IpedctxstartAhbbuserrorDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedctxstartAhbbuserrorDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedctxstartAhbbuserrorDis {
    #[inline(always)]
    fn from(val: u8) -> IpedctxstartAhbbuserrorDis {
        IpedctxstartAhbbuserrorDis::from_bits(val)
    }
}
impl From<IpedctxstartAhbbuserrorDis> for u8 {
    #[inline(always)]
    fn from(val: IpedctxstartAhbbuserrorDis) -> u8 {
        IpedctxstartAhbbuserrorDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IpedctxstartGcm {
    #[doc = "Disabled. CTR mode is used."]
    Value0 = 0x0,
    #[doc = "Enabled. GCM mode is used."]
    Value1 = 0x01,
}
impl IpedctxstartGcm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IpedctxstartGcm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IpedctxstartGcm {
    #[inline(always)]
    fn from(val: u8) -> IpedctxstartGcm {
        IpedctxstartGcm::from_bits(val)
    }
}
impl From<IpedctxstartGcm> for u8 {
    #[inline(always)]
    fn from(val: IpedctxstartGcm) -> u8 {
        IpedctxstartGcm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipgcmwr {
    #[doc = "Disabled."]
    Val0 = 0x0,
    #[doc = "Enabled."]
    Val1 = 0x01,
}
impl Ipgcmwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipgcmwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipgcmwr {
    #[inline(always)]
    fn from(val: u8) -> Ipgcmwr {
        Ipgcmwr::from_bits(val)
    }
}
impl From<Ipgcmwr> for u8 {
    #[inline(always)]
    fn from(val: Ipgcmwr) -> u8 {
        Ipgcmwr::to_bits(val)
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
    #[doc = "Suspended AHB read prefetch does not resume once aborted,."]
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
