#[doc = "DMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma0 {
    ptr: *mut u8,
}
unsafe impl Send for Dma0 {}
unsafe impl Sync for Dma0 {}
impl Dma0 {
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
    pub const fn dchpri3(self) -> crate::common::Reg<regs::Dchpri3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri2(self) -> crate::common::Reg<regs::Dchpri2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0101usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri1(self) -> crate::common::Reg<regs::Dchpri1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri0(self) -> crate::common::Reg<regs::Dchpri0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri7(self) -> crate::common::Reg<regs::Dchpri7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri6(self) -> crate::common::Reg<regs::Dchpri6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0105usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri5(self) -> crate::common::Reg<regs::Dchpri5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0106usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri4(self) -> crate::common::Reg<regs::Dchpri4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0107usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri11(self) -> crate::common::Reg<regs::Dchpri11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri10(self) -> crate::common::Reg<regs::Dchpri10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri9(self) -> crate::common::Reg<regs::Dchpri9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri8(self) -> crate::common::Reg<regs::Dchpri8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010busize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri15(self) -> crate::common::Reg<regs::Dchpri15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri14(self) -> crate::common::Reg<regs::Dchpri14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010dusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri13(self) -> crate::common::Reg<regs::Dchpri13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010eusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri12(self) -> crate::common::Reg<regs::Dchpri12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010fusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri19(self) -> crate::common::Reg<regs::Dchpri19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri18(self) -> crate::common::Reg<regs::Dchpri18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0111usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri17(self) -> crate::common::Reg<regs::Dchpri17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0112usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri16(self) -> crate::common::Reg<regs::Dchpri16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0113usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri23(self) -> crate::common::Reg<regs::Dchpri23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri22(self) -> crate::common::Reg<regs::Dchpri22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0115usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri21(self) -> crate::common::Reg<regs::Dchpri21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0116usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri20(self) -> crate::common::Reg<regs::Dchpri20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0117usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri27(self) -> crate::common::Reg<regs::Dchpri27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri26(self) -> crate::common::Reg<regs::Dchpri26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0119usize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri25(self) -> crate::common::Reg<regs::Dchpri25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011ausize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri24(self) -> crate::common::Reg<regs::Dchpri24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011busize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri31(self) -> crate::common::Reg<regs::Dchpri31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri30(self) -> crate::common::Reg<regs::Dchpri30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011dusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri29(self) -> crate::common::Reg<regs::Dchpri29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011eusize) as _) }
    }
    #[doc = "Channel Priority"]
    #[inline(always)]
    pub const fn dchpri28(self) -> crate::common::Reg<regs::Dchpri28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011fusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd0_saddr(self) -> crate::common::Reg<regs::Tcd0Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd0_soff(self) -> crate::common::Reg<regs::Tcd0Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd0_attr(self) -> crate::common::Reg<regs::Tcd0Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1006usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd0NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd0NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd0_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd0NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd0_slast(self) -> crate::common::Reg<regs::Tcd0Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x100cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd0_daddr(self) -> crate::common::Reg<regs::Tcd0Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd0_doff(self) -> crate::common::Reg<regs::Tcd0Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd0_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd0CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1016usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd0_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd0CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1016usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd0_dlastsga(self) -> crate::common::Reg<regs::Tcd0Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd0_csr(self) -> crate::common::Reg<regs::Tcd0Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd0_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd0BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd0_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd0BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x101eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd1_saddr(self) -> crate::common::Reg<regs::Tcd1Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1020usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd1_soff(self) -> crate::common::Reg<regs::Tcd1Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1024usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd1_attr(self) -> crate::common::Reg<regs::Tcd1Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1026usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd1NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd1NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd1_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd1NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1028usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd1_slast(self) -> crate::common::Reg<regs::Tcd1Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x102cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd1_daddr(self) -> crate::common::Reg<regs::Tcd1Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1030usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd1_doff(self) -> crate::common::Reg<regs::Tcd1Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1034usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd1_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd1CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1036usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd1_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd1CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1036usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd1_dlastsga(self) -> crate::common::Reg<regs::Tcd1Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1038usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd1_csr(self) -> crate::common::Reg<regs::Tcd1Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd1_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd1BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd1_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd1BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x103eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd2_saddr(self) -> crate::common::Reg<regs::Tcd2Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1040usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd2_soff(self) -> crate::common::Reg<regs::Tcd2Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1044usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd2_attr(self) -> crate::common::Reg<regs::Tcd2Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1046usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd2NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd2NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd2_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd2NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1048usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd2_slast(self) -> crate::common::Reg<regs::Tcd2Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x104cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd2_daddr(self) -> crate::common::Reg<regs::Tcd2Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1050usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd2_doff(self) -> crate::common::Reg<regs::Tcd2Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1054usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd2_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd2CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1056usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd2_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd2CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1056usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd2_dlastsga(self) -> crate::common::Reg<regs::Tcd2Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1058usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd2_csr(self) -> crate::common::Reg<regs::Tcd2Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd2_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd2BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd2_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd2BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x105eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd3_saddr(self) -> crate::common::Reg<regs::Tcd3Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1060usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd3_soff(self) -> crate::common::Reg<regs::Tcd3Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1064usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd3_attr(self) -> crate::common::Reg<regs::Tcd3Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1066usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd3NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd3NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd3_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd3NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1068usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd3_slast(self) -> crate::common::Reg<regs::Tcd3Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x106cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd3_daddr(self) -> crate::common::Reg<regs::Tcd3Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1070usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd3_doff(self) -> crate::common::Reg<regs::Tcd3Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1074usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd3_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd3CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1076usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd3_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd3CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1076usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd3_dlastsga(self) -> crate::common::Reg<regs::Tcd3Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1078usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd3_csr(self) -> crate::common::Reg<regs::Tcd3Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd3_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd3BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd3_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd3BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x107eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd4_saddr(self) -> crate::common::Reg<regs::Tcd4Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1080usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd4_soff(self) -> crate::common::Reg<regs::Tcd4Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1084usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd4_attr(self) -> crate::common::Reg<regs::Tcd4Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1086usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd4NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd4NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd4_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd4NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1088usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd4_slast(self) -> crate::common::Reg<regs::Tcd4Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x108cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd4_daddr(self) -> crate::common::Reg<regs::Tcd4Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1090usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd4_doff(self) -> crate::common::Reg<regs::Tcd4Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1094usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd4_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd4CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1096usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd4_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd4CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1096usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd4_dlastsga(self) -> crate::common::Reg<regs::Tcd4Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1098usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd4_csr(self) -> crate::common::Reg<regs::Tcd4Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd4_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd4BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd4_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd4BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x109eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd5_saddr(self) -> crate::common::Reg<regs::Tcd5Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd5_soff(self) -> crate::common::Reg<regs::Tcd5Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd5_attr(self) -> crate::common::Reg<regs::Tcd5Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd5NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd5NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd5_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd5NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd5_slast(self) -> crate::common::Reg<regs::Tcd5Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd5_daddr(self) -> crate::common::Reg<regs::Tcd5Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd5_doff(self) -> crate::common::Reg<regs::Tcd5Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd5_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd5CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd5_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd5CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd5_dlastsga(self) -> crate::common::Reg<regs::Tcd5Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd5_csr(self) -> crate::common::Reg<regs::Tcd5Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd5_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd5BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd5_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd5BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd6_saddr(self) -> crate::common::Reg<regs::Tcd6Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd6_soff(self) -> crate::common::Reg<regs::Tcd6Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd6_attr(self) -> crate::common::Reg<regs::Tcd6Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd6NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd6NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd6_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd6NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd6_slast(self) -> crate::common::Reg<regs::Tcd6Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd6_daddr(self) -> crate::common::Reg<regs::Tcd6Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd6_doff(self) -> crate::common::Reg<regs::Tcd6Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd6_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd6CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd6_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd6CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd6_dlastsga(self) -> crate::common::Reg<regs::Tcd6Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd6_csr(self) -> crate::common::Reg<regs::Tcd6Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd6_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd6BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd6_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd6BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd7_saddr(self) -> crate::common::Reg<regs::Tcd7Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd7_soff(self) -> crate::common::Reg<regs::Tcd7Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd7_attr(self) -> crate::common::Reg<regs::Tcd7Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd7NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd7NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd7_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd7NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd7_slast(self) -> crate::common::Reg<regs::Tcd7Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd7_daddr(self) -> crate::common::Reg<regs::Tcd7Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd7_doff(self) -> crate::common::Reg<regs::Tcd7Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd7_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd7CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd7_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd7CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd7_dlastsga(self) -> crate::common::Reg<regs::Tcd7Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd7_csr(self) -> crate::common::Reg<regs::Tcd7Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd7_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd7BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd7_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd7BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10feusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd8_saddr(self) -> crate::common::Reg<regs::Tcd8Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1100usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd8_soff(self) -> crate::common::Reg<regs::Tcd8Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1104usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd8_attr(self) -> crate::common::Reg<regs::Tcd8Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1106usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd8NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd8NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd8_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd8NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1108usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd8_slast(self) -> crate::common::Reg<regs::Tcd8Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x110cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd8_daddr(self) -> crate::common::Reg<regs::Tcd8Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1110usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd8_doff(self) -> crate::common::Reg<regs::Tcd8Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1114usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd8_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd8CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1116usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd8_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd8CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1116usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd8_dlastsga(self) -> crate::common::Reg<regs::Tcd8Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1118usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd8_csr(self) -> crate::common::Reg<regs::Tcd8Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd8_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd8BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd8_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd8BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x111eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd9_saddr(self) -> crate::common::Reg<regs::Tcd9Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1120usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd9_soff(self) -> crate::common::Reg<regs::Tcd9Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1124usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd9_attr(self) -> crate::common::Reg<regs::Tcd9Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1126usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd9NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd9NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd9_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd9NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1128usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd9_slast(self) -> crate::common::Reg<regs::Tcd9Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x112cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd9_daddr(self) -> crate::common::Reg<regs::Tcd9Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1130usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd9_doff(self) -> crate::common::Reg<regs::Tcd9Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1134usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd9_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd9CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1136usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd9_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd9CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1136usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd9_dlastsga(self) -> crate::common::Reg<regs::Tcd9Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1138usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd9_csr(self) -> crate::common::Reg<regs::Tcd9Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd9_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd9BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd9_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd9BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x113eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd10_saddr(self) -> crate::common::Reg<regs::Tcd10Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1140usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd10_soff(self) -> crate::common::Reg<regs::Tcd10Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1144usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd10_attr(self) -> crate::common::Reg<regs::Tcd10Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1146usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd10NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd10NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd10_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd10NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1148usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd10_slast(self) -> crate::common::Reg<regs::Tcd10Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x114cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd10_daddr(self) -> crate::common::Reg<regs::Tcd10Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1150usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd10_doff(self) -> crate::common::Reg<regs::Tcd10Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1154usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd10_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd10CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1156usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd10_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd10CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1156usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd10_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd10Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1158usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd10_csr(self) -> crate::common::Reg<regs::Tcd10Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd10_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd10BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd10_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd10BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x115eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd11_saddr(self) -> crate::common::Reg<regs::Tcd11Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1160usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd11_soff(self) -> crate::common::Reg<regs::Tcd11Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1164usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd11_attr(self) -> crate::common::Reg<regs::Tcd11Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1166usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd11NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd11NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd11_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd11NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1168usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd11_slast(self) -> crate::common::Reg<regs::Tcd11Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x116cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd11_daddr(self) -> crate::common::Reg<regs::Tcd11Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1170usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd11_doff(self) -> crate::common::Reg<regs::Tcd11Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1174usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd11_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd11CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1176usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd11_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd11CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1176usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd11_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd11Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1178usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd11_csr(self) -> crate::common::Reg<regs::Tcd11Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd11_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd11BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd11_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd11BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x117eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd12_saddr(self) -> crate::common::Reg<regs::Tcd12Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1180usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd12_soff(self) -> crate::common::Reg<regs::Tcd12Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1184usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd12_attr(self) -> crate::common::Reg<regs::Tcd12Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1186usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd12NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd12NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd12_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd12NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1188usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd12_slast(self) -> crate::common::Reg<regs::Tcd12Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x118cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd12_daddr(self) -> crate::common::Reg<regs::Tcd12Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1190usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd12_doff(self) -> crate::common::Reg<regs::Tcd12Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1194usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd12_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd12CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1196usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd12_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd12CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1196usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd12_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd12Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1198usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd12_csr(self) -> crate::common::Reg<regs::Tcd12Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd12_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd12BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd12_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd12BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x119eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd13_saddr(self) -> crate::common::Reg<regs::Tcd13Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd13_soff(self) -> crate::common::Reg<regs::Tcd13Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd13_attr(self) -> crate::common::Reg<regs::Tcd13Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd13NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd13NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd13_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd13NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd13_slast(self) -> crate::common::Reg<regs::Tcd13Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd13_daddr(self) -> crate::common::Reg<regs::Tcd13Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd13_doff(self) -> crate::common::Reg<regs::Tcd13Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd13_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd13CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd13_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd13CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd13_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd13Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd13_csr(self) -> crate::common::Reg<regs::Tcd13Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd13_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd13BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd13_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd13BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd14_saddr(self) -> crate::common::Reg<regs::Tcd14Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd14_soff(self) -> crate::common::Reg<regs::Tcd14Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd14_attr(self) -> crate::common::Reg<regs::Tcd14Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd14NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd14NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd14_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd14NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd14_slast(self) -> crate::common::Reg<regs::Tcd14Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd14_daddr(self) -> crate::common::Reg<regs::Tcd14Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd14_doff(self) -> crate::common::Reg<regs::Tcd14Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd14_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd14CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd14_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd14CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd14_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd14Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd14_csr(self) -> crate::common::Reg<regs::Tcd14Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd14_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd14BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd14_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd14BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd15_saddr(self) -> crate::common::Reg<regs::Tcd15Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd15_soff(self) -> crate::common::Reg<regs::Tcd15Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd15_attr(self) -> crate::common::Reg<regs::Tcd15Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd15NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd15NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd15_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd15NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd15_slast(self) -> crate::common::Reg<regs::Tcd15Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd15_daddr(self) -> crate::common::Reg<regs::Tcd15Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd15_doff(self) -> crate::common::Reg<regs::Tcd15Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd15_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd15CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd15_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd15CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd15_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd15Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd15_csr(self) -> crate::common::Reg<regs::Tcd15Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd15_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd15BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd15_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd15BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11feusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd16_saddr(self) -> crate::common::Reg<regs::Tcd16Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1200usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd16_soff(self) -> crate::common::Reg<regs::Tcd16Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1204usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd16_attr(self) -> crate::common::Reg<regs::Tcd16Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1206usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd16_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd16NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd16_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd16NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd16_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd16NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1208usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd16_slast(self) -> crate::common::Reg<regs::Tcd16Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x120cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd16_daddr(self) -> crate::common::Reg<regs::Tcd16Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1210usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd16_doff(self) -> crate::common::Reg<regs::Tcd16Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1214usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd16_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd16CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1216usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd16_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd16CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1216usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd16_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd16Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1218usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd16_csr(self) -> crate::common::Reg<regs::Tcd16Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd16_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd16BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd16_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd16BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x121eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd17_saddr(self) -> crate::common::Reg<regs::Tcd17Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1220usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd17_soff(self) -> crate::common::Reg<regs::Tcd17Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1224usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd17_attr(self) -> crate::common::Reg<regs::Tcd17Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1226usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd17_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd17NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd17_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd17NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd17_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd17NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1228usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd17_slast(self) -> crate::common::Reg<regs::Tcd17Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x122cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd17_daddr(self) -> crate::common::Reg<regs::Tcd17Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1230usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd17_doff(self) -> crate::common::Reg<regs::Tcd17Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1234usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd17_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd17CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1236usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd17_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd17CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1236usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd17_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd17Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1238usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd17_csr(self) -> crate::common::Reg<regs::Tcd17Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x123cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd17_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd17BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x123eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd17_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd17BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x123eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd18_saddr(self) -> crate::common::Reg<regs::Tcd18Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1240usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd18_soff(self) -> crate::common::Reg<regs::Tcd18Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1244usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd18_attr(self) -> crate::common::Reg<regs::Tcd18Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1246usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd18_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd18NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1248usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd18_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd18NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1248usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd18_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd18NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1248usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd18_slast(self) -> crate::common::Reg<regs::Tcd18Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x124cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd18_daddr(self) -> crate::common::Reg<regs::Tcd18Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1250usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd18_doff(self) -> crate::common::Reg<regs::Tcd18Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1254usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd18_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd18CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1256usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd18_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd18CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1256usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd18_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd18Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1258usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd18_csr(self) -> crate::common::Reg<regs::Tcd18Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x125cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd18_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd18BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x125eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd18_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd18BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x125eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd19_saddr(self) -> crate::common::Reg<regs::Tcd19Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1260usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd19_soff(self) -> crate::common::Reg<regs::Tcd19Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1264usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd19_attr(self) -> crate::common::Reg<regs::Tcd19Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1266usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd19_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd19NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1268usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd19_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd19NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1268usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd19_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd19NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1268usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd19_slast(self) -> crate::common::Reg<regs::Tcd19Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x126cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd19_daddr(self) -> crate::common::Reg<regs::Tcd19Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1270usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd19_doff(self) -> crate::common::Reg<regs::Tcd19Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1274usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd19_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd19CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1276usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd19_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd19CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1276usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd19_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd19Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1278usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd19_csr(self) -> crate::common::Reg<regs::Tcd19Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x127cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd19_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd19BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x127eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd19_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd19BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x127eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd20_saddr(self) -> crate::common::Reg<regs::Tcd20Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1280usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd20_soff(self) -> crate::common::Reg<regs::Tcd20Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1284usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd20_attr(self) -> crate::common::Reg<regs::Tcd20Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1286usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd20_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd20NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1288usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd20_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd20NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1288usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd20_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd20NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1288usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd20_slast(self) -> crate::common::Reg<regs::Tcd20Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x128cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd20_daddr(self) -> crate::common::Reg<regs::Tcd20Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1290usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd20_doff(self) -> crate::common::Reg<regs::Tcd20Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1294usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd20_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd20CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1296usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd20_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd20CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1296usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd20_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd20Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1298usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd20_csr(self) -> crate::common::Reg<regs::Tcd20Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x129cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd20_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd20BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x129eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd20_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd20BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x129eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd21_saddr(self) -> crate::common::Reg<regs::Tcd21Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd21_soff(self) -> crate::common::Reg<regs::Tcd21Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd21_attr(self) -> crate::common::Reg<regs::Tcd21Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd21_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd21NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd21_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd21NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd21_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd21NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd21_slast(self) -> crate::common::Reg<regs::Tcd21Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd21_daddr(self) -> crate::common::Reg<regs::Tcd21Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd21_doff(self) -> crate::common::Reg<regs::Tcd21Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd21_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd21CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd21_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd21CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd21_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd21Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd21_csr(self) -> crate::common::Reg<regs::Tcd21Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd21_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd21BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd21_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd21BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd22_saddr(self) -> crate::common::Reg<regs::Tcd22Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd22_soff(self) -> crate::common::Reg<regs::Tcd22Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd22_attr(self) -> crate::common::Reg<regs::Tcd22Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd22_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd22NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd22_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd22NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd22_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd22NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd22_slast(self) -> crate::common::Reg<regs::Tcd22Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd22_daddr(self) -> crate::common::Reg<regs::Tcd22Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd22_doff(self) -> crate::common::Reg<regs::Tcd22Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd22_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd22CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd22_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd22CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd22_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd22Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd22_csr(self) -> crate::common::Reg<regs::Tcd22Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd22_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd22BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd22_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd22BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd23_saddr(self) -> crate::common::Reg<regs::Tcd23Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd23_soff(self) -> crate::common::Reg<regs::Tcd23Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd23_attr(self) -> crate::common::Reg<regs::Tcd23Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd23_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd23NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd23_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd23NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd23_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd23NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd23_slast(self) -> crate::common::Reg<regs::Tcd23Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd23_daddr(self) -> crate::common::Reg<regs::Tcd23Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd23_doff(self) -> crate::common::Reg<regs::Tcd23Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd23_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd23CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd23_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd23CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd23_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd23Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd23_csr(self) -> crate::common::Reg<regs::Tcd23Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd23_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd23BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd23_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd23BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12feusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd24_saddr(self) -> crate::common::Reg<regs::Tcd24Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1300usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd24_soff(self) -> crate::common::Reg<regs::Tcd24Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1304usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd24_attr(self) -> crate::common::Reg<regs::Tcd24Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1306usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd24_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd24NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1308usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd24_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd24NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1308usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd24_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd24NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1308usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd24_slast(self) -> crate::common::Reg<regs::Tcd24Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x130cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd24_daddr(self) -> crate::common::Reg<regs::Tcd24Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1310usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd24_doff(self) -> crate::common::Reg<regs::Tcd24Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1314usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd24_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd24CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1316usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd24_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd24CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1316usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd24_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd24Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1318usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd24_csr(self) -> crate::common::Reg<regs::Tcd24Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x131cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd24_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd24BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x131eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd24_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd24BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x131eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd25_saddr(self) -> crate::common::Reg<regs::Tcd25Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1320usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd25_soff(self) -> crate::common::Reg<regs::Tcd25Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1324usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd25_attr(self) -> crate::common::Reg<regs::Tcd25Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1326usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd25_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd25NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1328usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd25_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd25NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1328usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd25_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd25NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1328usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd25_slast(self) -> crate::common::Reg<regs::Tcd25Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x132cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd25_daddr(self) -> crate::common::Reg<regs::Tcd25Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1330usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd25_doff(self) -> crate::common::Reg<regs::Tcd25Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1334usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd25_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd25CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1336usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd25_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd25CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1336usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd25_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd25Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1338usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd25_csr(self) -> crate::common::Reg<regs::Tcd25Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x133cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd25_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd25BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x133eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd25_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd25BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x133eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd26_saddr(self) -> crate::common::Reg<regs::Tcd26Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1340usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd26_soff(self) -> crate::common::Reg<regs::Tcd26Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1344usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd26_attr(self) -> crate::common::Reg<regs::Tcd26Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1346usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd26_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd26NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd26_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd26NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd26_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd26NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1348usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd26_slast(self) -> crate::common::Reg<regs::Tcd26Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x134cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd26_daddr(self) -> crate::common::Reg<regs::Tcd26Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1350usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd26_doff(self) -> crate::common::Reg<regs::Tcd26Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1354usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd26_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd26CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1356usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd26_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd26CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1356usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd26_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd26Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1358usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd26_csr(self) -> crate::common::Reg<regs::Tcd26Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x135cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd26_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd26BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x135eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd26_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd26BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x135eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd27_saddr(self) -> crate::common::Reg<regs::Tcd27Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1360usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd27_soff(self) -> crate::common::Reg<regs::Tcd27Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1364usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd27_attr(self) -> crate::common::Reg<regs::Tcd27Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1366usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd27_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd27NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1368usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd27_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd27NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1368usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd27_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd27NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1368usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd27_slast(self) -> crate::common::Reg<regs::Tcd27Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x136cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd27_daddr(self) -> crate::common::Reg<regs::Tcd27Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1370usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd27_doff(self) -> crate::common::Reg<regs::Tcd27Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1374usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd27_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd27CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1376usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd27_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd27CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1376usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd27_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd27Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1378usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd27_csr(self) -> crate::common::Reg<regs::Tcd27Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x137cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd27_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd27BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x137eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd27_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd27BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x137eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd28_saddr(self) -> crate::common::Reg<regs::Tcd28Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1380usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd28_soff(self) -> crate::common::Reg<regs::Tcd28Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1384usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd28_attr(self) -> crate::common::Reg<regs::Tcd28Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1386usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd28_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd28NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1388usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd28_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd28NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1388usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd28_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd28NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1388usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd28_slast(self) -> crate::common::Reg<regs::Tcd28Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x138cusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd28_daddr(self) -> crate::common::Reg<regs::Tcd28Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1390usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd28_doff(self) -> crate::common::Reg<regs::Tcd28Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1394usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd28_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd28CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1396usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd28_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd28CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1396usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd28_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd28Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1398usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd28_csr(self) -> crate::common::Reg<regs::Tcd28Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x139cusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd28_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd28BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x139eusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd28_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd28BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x139eusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd29_saddr(self) -> crate::common::Reg<regs::Tcd29Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd29_soff(self) -> crate::common::Reg<regs::Tcd29Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd29_attr(self) -> crate::common::Reg<regs::Tcd29Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd29_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd29NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd29_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd29NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd29_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd29NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13a8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd29_slast(self) -> crate::common::Reg<regs::Tcd29Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13acusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd29_daddr(self) -> crate::common::Reg<regs::Tcd29Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd29_doff(self) -> crate::common::Reg<regs::Tcd29Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd29_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd29CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd29_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd29CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd29_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd29Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13b8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd29_csr(self) -> crate::common::Reg<regs::Tcd29Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13bcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd29_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd29BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13beusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd29_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd29BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13beusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd30_saddr(self) -> crate::common::Reg<regs::Tcd30Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd30_soff(self) -> crate::common::Reg<regs::Tcd30Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd30_attr(self) -> crate::common::Reg<regs::Tcd30Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd30_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd30NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd30_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd30NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd30_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd30NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13c8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd30_slast(self) -> crate::common::Reg<regs::Tcd30Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13ccusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd30_daddr(self) -> crate::common::Reg<regs::Tcd30Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd30_doff(self) -> crate::common::Reg<regs::Tcd30Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd30_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd30CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd30_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd30CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd30_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd30Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13d8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd30_csr(self) -> crate::common::Reg<regs::Tcd30Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13dcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd30_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd30BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13deusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd30_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd30BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13deusize) as _) }
    }
    #[doc = "TCD Source Address"]
    #[inline(always)]
    pub const fn tcd31_saddr(self) -> crate::common::Reg<regs::Tcd31Saddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e0usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset"]
    #[inline(always)]
    pub const fn tcd31_soff(self) -> crate::common::Reg<regs::Tcd31Soff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e4usize) as _) }
    }
    #[doc = "TCD Transfer Attributes"]
    #[inline(always)]
    pub const fn tcd31_attr(self) -> crate::common::Reg<regs::Tcd31Attr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e6usize) as _) }
    }
    #[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub const fn tcd31_nbytes_mlno(
        self,
    ) -> crate::common::Reg<regs::Tcd31NbytesMlno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn tcd31_nbytes_mloffno(
        self,
    ) -> crate::common::Reg<regs::Tcd31NbytesMloffno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e8usize) as _) }
    }
    #[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub const fn tcd31_nbytes_mloffyes(
        self,
    ) -> crate::common::Reg<regs::Tcd31NbytesMloffyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13e8usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment"]
    #[inline(always)]
    pub const fn tcd31_slast(self) -> crate::common::Reg<regs::Tcd31Slast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13ecusize) as _) }
    }
    #[doc = "TCD Destination Address"]
    #[inline(always)]
    pub const fn tcd31_daddr(self) -> crate::common::Reg<regs::Tcd31Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f0usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset"]
    #[inline(always)]
    pub const fn tcd31_doff(self) -> crate::common::Reg<regs::Tcd31Doff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f4usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd31_citer_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd31CiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f6usize) as _) }
    }
    #[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd31_citer_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd31CiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f6usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
    #[inline(always)]
    pub const fn tcd31_dlastsga(
        self,
    ) -> crate::common::Reg<regs::Tcd31Dlastsga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13f8usize) as _) }
    }
    #[doc = "TCD Control and Status"]
    #[inline(always)]
    pub const fn tcd31_csr(self) -> crate::common::Reg<regs::Tcd31Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13fcusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn tcd31_biter_elinkno(
        self,
    ) -> crate::common::Reg<regs::Tcd31BiterElinkno, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13feusize) as _) }
    }
    #[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn tcd31_biter_elinkyes(
        self,
    ) -> crate::common::Reg<regs::Tcd31BiterElinkyes, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13feusize) as _) }
    }
}
pub mod regs;
pub mod vals;
