#[doc = "DMA TCD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edma0Tcd {
    ptr: *mut u8,
}
unsafe impl Send for Edma0Tcd {}
unsafe impl Sync for Edma0Tcd {}
impl Edma0Tcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
    #[inline(always)]
    pub const fn tcd(self, n: usize) -> Tcd {
        assert!(n < 16usize);
        unsafe { Tcd::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4096usize) as _) }
    }
}
#[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, CH_MUX, TCD_SADDR, TCD_SOFF, TCD_ATTR, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SLAST_SDA, TCD_DADDR, TCD_CITER_ELINKYES, TCD_CITER_ELINKNO, TCD_DOFF, TCD_DLAST_SGA, TCD_BITER_ELINKYES, TCD_BITER_ELINKNO, TCD_CSR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd {
    ptr: *mut u8,
}
unsafe impl Send for Tcd {}
unsafe impl Sync for Tcd {}
impl Tcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel Control and Status"]
    #[inline(always)]
    pub const fn ch_csr(self) -> crate::common::Reg<regs::ChCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel Error Status"]
    #[inline(always)]
    pub const fn ch_es(self) -> crate::common::Reg<regs::ChEs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel Interrupt Status"]
    #[inline(always)]
    pub const fn ch_int(self) -> crate::common::Reg<regs::ChInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel System Bus"]
    #[inline(always)]
    pub const fn ch_sbr(self) -> crate::common::Reg<regs::ChSbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn ch_pri(self) -> crate::common::Reg<regs::ChPri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel Multiplexor Configuration"]
    #[inline(always)]
    pub const fn ch_mux(self) -> crate::common::Reg<regs::ChMux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets"]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TCD Transfer Size with Minor Loop Offsets"]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address"]
    #[inline(always)]
    pub const fn tcd_slast_sda(self) -> crate::common::Reg<regs::TcdSlastSda, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd_dlast_sga(self) -> crate::common::Reg<regs::TcdDlastSga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
}
pub mod regs;
pub mod vals;
