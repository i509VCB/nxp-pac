#[doc = "DMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Error Status"]
    #[inline(always)]
    pub const fn es(self) -> crate::common::Reg<regs::Es, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Enable Request"]
    #[inline(always)]
    pub const fn erq(self) -> crate::common::Reg<regs::Erq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Enable Error Interrupt"]
    #[inline(always)]
    pub const fn eei(self) -> crate::common::Reg<regs::Eei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Clear Enable Error Interrupt"]
    #[inline(always)]
    pub const fn ceei(self) -> crate::common::Reg<regs::Ceei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Set Enable Error Interrupt"]
    #[inline(always)]
    pub const fn seei(self) -> crate::common::Reg<regs::Seei, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "Clear Enable Request"]
    #[inline(always)]
    pub const fn cerq(self) -> crate::common::Reg<regs::Cerq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Set Enable Request"]
    #[inline(always)]
    pub const fn serq(self) -> crate::common::Reg<regs::Serq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "Clear DONE Status Bit"]
    #[inline(always)]
    pub const fn cdne(self) -> crate::common::Reg<regs::Cdne, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Set START Bit"]
    #[inline(always)]
    pub const fn ssrt(self) -> crate::common::Reg<regs::Ssrt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1dusize) as _) }
    }
    #[doc = "Clear Error"]
    #[inline(always)]
    pub const fn cerr(self) -> crate::common::Reg<regs::Cerr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Clear Interrupt Request"]
    #[inline(always)]
    pub const fn cint(self) -> crate::common::Reg<regs::Cint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1fusize) as _) }
    }
    #[doc = "Interrupt Request"]
    #[inline(always)]
    pub const fn int(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn err(self) -> crate::common::Reg<regs::Err, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Hardware Request Status"]
    #[inline(always)]
    pub const fn hrs(self) -> crate::common::Reg<regs::Hrs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Enable Asynchronous Request in Stop"]
    #[inline(always)]
    pub const fn ears(self) -> crate::common::Reg<regs::Ears, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri3(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri2(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0101usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri1(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri0(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri7(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri6(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0105usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri5(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri4(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0107usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri11(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri10(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri9(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri8(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010busize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri15(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri14(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010dusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri13(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010eusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri12(self) -> crate::common::Reg<regs::Dchpri, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010fusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd0_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd0_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd0_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1006usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd0_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd0_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd0_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd0_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1016usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd0_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1016usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd0_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd0_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd0_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd0_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd1_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd1_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd1_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1026usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd1_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd1_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd1_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd1_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1036usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd1_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1036usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd1_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd1_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd1_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd1_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd2_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd2_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd2_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1046usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd2_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd2_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd2_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd2_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1056usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd2_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1056usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd2_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd2_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd2_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd2_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd3_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd3_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd3_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1066usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd3_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd3_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd3_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd3_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1076usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd3_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1076usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd3_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd3_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd3_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd3_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd4_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd4_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd4_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1086usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd4_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x108cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd4_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd4_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd4_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1096usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd4_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1096usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd4_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd4_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd4_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd4_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd5_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd5_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd5_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd5_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd5_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd5_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd5_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd5_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd5_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd5_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd5_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd5_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd6_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd6_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd6_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd6_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd6_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd6_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd6_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd6_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd6_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd6_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd6_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd6_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd7_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd7_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd7_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd7_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd7_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd7_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd7_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd7_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd7_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd7_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd7_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd7_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10feusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd8_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd8_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd8_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1106usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd8_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x110cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd8_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd8_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd8_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1116usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd8_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1116usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd8_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd8_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd8_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd8_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd9_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd9_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd9_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1126usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd9_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd9_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd9_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd9_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1136usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd9_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1136usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd9_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd9_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd9_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd9_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd10_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd10_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd10_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1146usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd10_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd10_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd10_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd10_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1156usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd10_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1156usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd10_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd10_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd10_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd10_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd11_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd11_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd11_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1166usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd11_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd11_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd11_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1174usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd11_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1176usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd11_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1176usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd11_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1178usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd11_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd11_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd11_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd12_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd12_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd12_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1186usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd12_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x118cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd12_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1190usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd12_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd12_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1196usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd12_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1196usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd12_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1198usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd12_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd12_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd12_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd13_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd13_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd13_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd13_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd13_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd13_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd13_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd13_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd13_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd13_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd13_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd13_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd14_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd14_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd14_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd14_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd14_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd14_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd14_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd14_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd14_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd14_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd14_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd14_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd15_saddr(self) -> crate::common::Reg<regs::TcdSaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd15_soff(self) -> crate::common::Reg<regs::TcdSoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd15_attr(self) -> crate::common::Reg<regs::TcdAttr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::TcdNbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd15_slast(self) -> crate::common::Reg<regs::TcdSlast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd15_daddr(self) -> crate::common::Reg<regs::TcdDaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd15_doff(self) -> crate::common::Reg<regs::TcdDoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd15_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd15_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdCiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd15_dlastsga(self) -> crate::common::Reg<regs::TcdDlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd15_csr(self) -> crate::common::Reg<regs::TcdCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd15_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd15_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::TcdBiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11feusize) as _) }
    }
}
pub mod regs;
pub mod vals;
