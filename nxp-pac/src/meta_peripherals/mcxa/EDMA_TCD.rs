#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_PRI, CH_SBR, TCD_ATTR, TCD_BITER_ELINKNO, TCD_BITER_ELINKYES, TCD_CITER_ELINKNO, TCD_CITER_ELINKYES, TCD_CSR, TCD_DADDR, TCD_DLAST_SGA, TCD_DOFF, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SADDR, TCD_SLAST_SDA, TCD_SOFF."]
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
    #[doc = "Channel Control and Status."]
    #[inline(always)]
    pub const fn ch_csr(self) -> crate::pac::common::Reg<ChCsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel Error Status."]
    #[inline(always)]
    pub const fn ch_es(self) -> crate::pac::common::Reg<ChEs, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel Interrupt Status."]
    #[inline(always)]
    pub const fn ch_int(self) -> crate::pac::common::Reg<ChInt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel System Bus."]
    #[inline(always)]
    pub const fn ch_sbr(self) -> crate::pac::common::Reg<ChSbr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Priority."]
    #[inline(always)]
    pub const fn ch_pri(self) -> crate::pac::common::Reg<ChPri, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel Multiplexor Configuration."]
    #[inline(always)]
    pub const fn ch_mux(self) -> crate::pac::common::Reg<ChMux, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TCD Source Address."]
    #[inline(always)]
    pub const fn tcd_saddr(self) -> crate::pac::common::Reg<TcdSaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TCD Signed Source Address Offset."]
    #[inline(always)]
    pub const fn tcd_soff(self) -> crate::pac::common::Reg<TcdSoff, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "TCD Transfer Attributes."]
    #[inline(always)]
    pub const fn tcd_attr(self) -> crate::pac::common::Reg<TcdAttr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "TCD Transfer Size Without Minor Loop Offsets."]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffno(
        self,
    ) -> crate::pac::common::Reg<TcdNbytesMloffno, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TCD Transfer Size with Minor Loop Offsets."]
    #[inline(always)]
    pub const fn tcd_nbytes_mloffyes(
        self,
    ) -> crate::pac::common::Reg<TcdNbytesMloffyes, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "TCD Last Source Address Adjustment / Store DADDR Address."]
    #[inline(always)]
    pub const fn tcd_slast_sda(
        self,
    ) -> crate::pac::common::Reg<TcdSlastSda, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "TCD Destination Address."]
    #[inline(always)]
    pub const fn tcd_daddr(self) -> crate::pac::common::Reg<TcdDaddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TCD Signed Destination Address Offset."]
    #[inline(always)]
    pub const fn tcd_doff(self) -> crate::pac::common::Reg<TcdDoff, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)."]
    #[inline(always)]
    pub const fn tcd_citer_elinkno(
        self,
    ) -> crate::pac::common::Reg<TcdCiterElinkno, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)."]
    #[inline(always)]
    pub const fn tcd_citer_elinkyes(
        self,
    ) -> crate::pac::common::Reg<TcdCiterElinkyes, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address."]
    #[inline(always)]
    pub const fn tcd_dlast_sga(
        self,
    ) -> crate::pac::common::Reg<TcdDlastSga, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "TCD Control and Status."]
    #[inline(always)]
    pub const fn tcd_csr(self) -> crate::pac::common::Reg<TcdCsr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)."]
    #[inline(always)]
    pub const fn tcd_biter_elinkno(
        self,
    ) -> crate::pac::common::Reg<TcdBiterElinkno, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)."]
    #[inline(always)]
    pub const fn tcd_biter_elinkyes(
        self,
    ) -> crate::pac::common::Reg<TcdBiterElinkyes, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
}
#[doc = "DMA TCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd4 {
    ptr: *mut u8,
}
unsafe impl Send for Tcd4 {}
unsafe impl Sync for Tcd4 {}
impl Tcd4 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_MUX, CH_PRI, CH_SBR, TCD_ATTR, TCD_BITER_ELINKNO, TCD_BITER_ELINKYES, TCD_CITER_ELINKNO, TCD_CITER_ELINKYES, TCD_CSR, TCD_DADDR, TCD_DLAST_SGA, TCD_DOFF, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SADDR, TCD_SLAST_SDA, TCD_SOFF."]
    #[inline(always)]
    pub const fn tcd(self, n: usize) -> Tcd {
        assert!(n < 4usize);
        unsafe { Tcd::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4096usize) as _) }
    }
}
#[doc = "DMA TCD."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcd8 {
    ptr: *mut u8,
}
unsafe impl Send for Tcd8 {}
unsafe impl Sync for Tcd8 {}
impl Tcd8 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: CH_CSR, CH_ES, CH_INT, CH_MUX, CH_PRI, CH_SBR, TCD_ATTR, TCD_BITER_ELINKNO, TCD_BITER_ELINKYES, TCD_CITER_ELINKNO, TCD_CITER_ELINKYES, TCD_CSR, TCD_DADDR, TCD_DLAST_SGA, TCD_DOFF, TCD_NBYTES_MLOFFNO, TCD_NBYTES_MLOFFYES, TCD_SADDR, TCD_SLAST_SDA, TCD_SOFF."]
    #[inline(always)]
    pub const fn tcd(self, n: usize) -> Tcd {
        assert!(n < 8usize);
        unsafe { Tcd::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4096usize) as _) }
    }
}
#[doc = "Channel Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCsr(pub u32);
impl ChCsr {
    #[doc = "Enable DMA Request."]
    #[must_use]
    #[inline(always)]
    pub const fn erq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request."]
    #[inline(always)]
    pub const fn set_erq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Asynchronous DMA Request."]
    #[must_use]
    #[inline(always)]
    pub const fn earq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Asynchronous DMA Request."]
    #[inline(always)]
    pub const fn set_earq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn eei(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt."]
    #[inline(always)]
    pub const fn set_eei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Buffered Writes."]
    #[must_use]
    #[inline(always)]
    pub const fn ebw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Buffered Writes."]
    #[inline(always)]
    pub const fn set_ebw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Done."]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done."]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel Active."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChCsr {
    #[inline(always)]
    fn default() -> ChCsr {
        ChCsr(0)
    }
}
impl core::fmt::Debug for ChCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCsr")
            .field("erq", &self.erq())
            .field("earq", &self.earq())
            .field("eei", &self.eei())
            .field("ebw", &self.ebw())
            .field("done", &self.done())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ChCsr {{ erq: {=bool:?}, earq: {=bool:?}, eei: {=bool:?}, ebw: {=bool:?}, done: {=bool:?}, active: {=bool:?} }}",
            self.erq(),
            self.earq(),
            self.eei(),
            self.ebw(),
            self.done(),
            self.active()
        )
    }
}
#[doc = "Channel Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChEs(pub u32);
impl ChEs {
    #[doc = "Destination Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error."]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error."]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error."]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error."]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error."]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Error In Channel."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error In Channel."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChEs {
    #[inline(always)]
    fn default() -> ChEs {
        ChEs(0)
    }
}
impl core::fmt::Debug for ChEs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChEs")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChEs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ChEs {{ dbe: {=bool:?}, sbe: {=bool:?}, sge: {=bool:?}, nce: {=bool:?}, doe: {=bool:?}, dae: {=bool:?}, soe: {=bool:?}, sae: {=bool:?}, err: {=bool:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.err()
        )
    }
}
#[doc = "Channel Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChInt(pub u32);
impl ChInt {
    #[doc = "Interrupt Request."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ChInt {
    #[inline(always)]
    fn default() -> ChInt {
        ChInt(0)
    }
}
impl core::fmt::Debug for ChInt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChInt").field("int", &self.int()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChInt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChInt {{ int: {=bool:?} }}", self.int())
    }
}
#[doc = "Channel Multiplexor Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChMux(pub u32);
impl ChMux {
    #[doc = "Service Request Source."]
    #[must_use]
    #[inline(always)]
    pub const fn src(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Service Request Source."]
    #[inline(always)]
    pub const fn set_src(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for ChMux {
    #[inline(always)]
    fn default() -> ChMux {
        ChMux(0)
    }
}
impl core::fmt::Debug for ChMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChMux").field("src", &self.src()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChMux {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChMux {{ src: {=u8:?} }}", self.src())
    }
}
#[doc = "Channel Priority."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChPri(pub u32);
impl ChPri {
    #[doc = "Arbitration Priority Level."]
    #[must_use]
    #[inline(always)]
    pub const fn apl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Arbitration Priority Level."]
    #[inline(always)]
    pub const fn set_apl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Disable Preempt Ability."]
    #[must_use]
    #[inline(always)]
    pub const fn dpa(&self) -> Dpa {
        let val = (self.0 >> 30usize) & 0x01;
        Dpa::from_bits(val as u8)
    }
    #[doc = "Disable Preempt Ability."]
    #[inline(always)]
    pub const fn set_dpa(&mut self, val: Dpa) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable Channel Preemption."]
    #[must_use]
    #[inline(always)]
    pub const fn ecp(&self) -> Ecp {
        let val = (self.0 >> 31usize) & 0x01;
        Ecp::from_bits(val as u8)
    }
    #[doc = "Enable Channel Preemption."]
    #[inline(always)]
    pub const fn set_ecp(&mut self, val: Ecp) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for ChPri {
    #[inline(always)]
    fn default() -> ChPri {
        ChPri(0)
    }
}
impl core::fmt::Debug for ChPri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChPri")
            .field("apl", &self.apl())
            .field("dpa", &self.dpa())
            .field("ecp", &self.ecp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChPri {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ChPri {{ apl: {=u8:?}, dpa: {:?}, ecp: {:?} }}",
            self.apl(),
            self.dpa(),
            self.ecp()
        )
    }
}
#[doc = "Channel System Bus."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChSbr(pub u32);
impl ChSbr {
    #[doc = "Master ID."]
    #[must_use]
    #[inline(always)]
    pub const fn mid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Master ID."]
    #[inline(always)]
    pub const fn set_mid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Security Level."]
    #[must_use]
    #[inline(always)]
    pub const fn sec(&self) -> Sec {
        let val = (self.0 >> 14usize) & 0x01;
        Sec::from_bits(val as u8)
    }
    #[doc = "Security Level."]
    #[inline(always)]
    pub const fn set_sec(&mut self, val: Sec) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Privileged Access Level."]
    #[must_use]
    #[inline(always)]
    pub const fn pal(&self) -> Pal {
        let val = (self.0 >> 15usize) & 0x01;
        Pal::from_bits(val as u8)
    }
    #[doc = "Privileged Access Level."]
    #[inline(always)]
    pub const fn set_pal(&mut self, val: Pal) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable Master ID Replication."]
    #[must_use]
    #[inline(always)]
    pub const fn emi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Master ID Replication."]
    #[inline(always)]
    pub const fn set_emi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Attribute Output."]
    #[must_use]
    #[inline(always)]
    pub const fn attr(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[doc = "Attribute Output."]
    #[inline(always)]
    pub const fn set_attr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
}
impl Default for ChSbr {
    #[inline(always)]
    fn default() -> ChSbr {
        ChSbr(0)
    }
}
impl core::fmt::Debug for ChSbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChSbr")
            .field("mid", &self.mid())
            .field("sec", &self.sec())
            .field("pal", &self.pal())
            .field("emi", &self.emi())
            .field("attr", &self.attr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChSbr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ChSbr {{ mid: {=u8:?}, sec: {:?}, pal: {:?}, emi: {=bool:?}, attr: {=u8:?} }}",
            self.mid(),
            self.sec(),
            self.pal(),
            self.emi(),
            self.attr()
        )
    }
}
#[doc = "TCD Transfer Attributes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdAttr(pub u16);
impl TcdAttr {
    #[doc = "Destination Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn dsize(&self) -> Size {
        let val = (self.0 >> 0usize) & 0x07;
        Size::from_bits(val as u8)
    }
    #[doc = "Destination Data Transfer Size."]
    #[inline(always)]
    pub const fn set_dsize(&mut self, val: Size) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u16) & 0x07) << 0usize);
    }
    #[doc = "Destination Address Modulo."]
    #[must_use]
    #[inline(always)]
    pub const fn dmod(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x1f;
        val as u8
    }
    #[doc = "Destination Address Modulo."]
    #[inline(always)]
    pub const fn set_dmod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u16) & 0x1f) << 3usize);
    }
    #[doc = "Source Data Transfer Size."]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> Size {
        let val = (self.0 >> 8usize) & 0x07;
        Size::from_bits(val as u8)
    }
    #[doc = "Source Data Transfer Size."]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: Size) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u16) & 0x07) << 8usize);
    }
    #[doc = "Source Address Modulo."]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Source Address Modulo."]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for TcdAttr {
    #[inline(always)]
    fn default() -> TcdAttr {
        TcdAttr(0)
    }
}
impl core::fmt::Debug for TcdAttr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdAttr")
            .field("dsize", &self.dsize())
            .field("dmod", &self.dmod())
            .field("ssize", &self.ssize())
            .field("smod", &self.smod())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdAttr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdAttr {{ dsize: {:?}, dmod: {=u8:?}, ssize: {:?}, smod: {=u8:?} }}",
            self.dsize(),
            self.dmod(),
            self.ssize(),
            self.smod()
        )
    }
}
#[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkno(pub u16);
impl TcdBiterElinkno {
    #[doc = "Starting Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count."]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enables Link."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Link."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkno {
    #[inline(always)]
    fn default() -> TcdBiterElinkno {
        TcdBiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkno")
            .field("biter", &self.biter())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkno {{ biter: {=u16:?}, elink: {=bool:?} }}",
            self.biter(),
            self.elink()
        )
    }
}
#[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count."]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Link Channel Number."]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable Link."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkyes {
    #[inline(always)]
    fn default() -> TcdBiterElinkyes {
        TcdBiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdBiterElinkyes {{ biter: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.biter(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc = "Current Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count."]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable Link."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkno {
    #[inline(always)]
    fn default() -> TcdCiterElinkno {
        TcdCiterElinkno(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkno {{ citer: {=u16:?}, elink: {=bool:?} }}",
            self.citer(),
            self.elink()
        )
    }
}
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc = "Current Major Iteration Count."]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count."]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number."]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u16) & 0x0f) << 9usize);
    }
    #[doc = "Enable Link."]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link."]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkyes {
    #[inline(always)]
    fn default() -> TcdCiterElinkyes {
        TcdCiterElinkyes(0)
    }
}
impl core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCiterElinkyes {{ citer: {=u16:?}, linkch: {=u8:?}, elink: {=bool:?} }}",
            self.citer(),
            self.linkch(),
            self.elink()
        )
    }
}
#[doc = "TCD Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc = "Channel Start."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> Start {
        let val = (self.0 >> 0usize) & 0x01;
        Start::from_bits(val as u8)
    }
    #[doc = "Channel Start."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: Start) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable Interrupt If Major count complete."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt If Major count complete."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt If Major Counter Half-complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt If Major Counter Half-complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request."]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> Dreq {
        let val = (self.0 >> 3usize) & 0x01;
        Dreq::from_bits(val as u8)
    }
    #[doc = "Disable Request."]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: Dreq) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing."]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> Esg {
        let val = (self.0 >> 4usize) & 0x01;
        Esg::from_bits(val as u8)
    }
    #[doc = "Enable Scatter/Gather Processing."]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: Esg) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable Link When Major Loop Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link When Major Loop Complete."]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Enable End-Of-Packet Processing."]
    #[must_use]
    #[inline(always)]
    pub const fn eeop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End-Of-Packet Processing."]
    #[inline(always)]
    pub const fn set_eeop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Enable Store Destination Address."]
    #[must_use]
    #[inline(always)]
    pub const fn esda(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Store Destination Address."]
    #[inline(always)]
    pub const fn set_esda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number."]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u16) & 0x0f) << 8usize);
    }
    #[doc = "Bandwidth Control."]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> Bwc {
        let val = (self.0 >> 14usize) & 0x03;
        Bwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control."]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: Bwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for TcdCsr {
    #[inline(always)]
    fn default() -> TcdCsr {
        TcdCsr(0)
    }
}
impl core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCsr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("eeop", &self.eeop())
            .field("esda", &self.esda())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdCsr {{ start: {:?}, intmajor: {=bool:?}, inthalf: {=bool:?}, dreq: {:?}, esg: {:?}, majorelink: {=bool:?}, eeop: {=bool:?}, esda: {=bool:?}, majorlinkch: {=u8:?}, bwc: {:?} }}",
            self.start(),
            self.intmajor(),
            self.inthalf(),
            self.dreq(),
            self.esg(),
            self.majorelink(),
            self.eeop(),
            self.esda(),
            self.majorlinkch(),
            self.bwc()
        )
    }
}
#[doc = "TCD Destination Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDaddr(pub u32);
impl TcdDaddr {
    #[doc = "Destination Address."]
    #[must_use]
    #[inline(always)]
    pub const fn daddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Destination Address."]
    #[inline(always)]
    pub const fn set_daddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDaddr {
    #[inline(always)]
    fn default() -> TcdDaddr {
        TcdDaddr(0)
    }
}
impl core::fmt::Debug for TcdDaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDaddr")
            .field("daddr", &self.daddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDaddr {{ daddr: {=u32:?} }}", self.daddr())
    }
}
#[doc = "TCD Last Destination Address Adjustment / Scatter Gather Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDlastSga(pub u32);
impl TcdDlastSga {
    #[doc = "Last Destination Address Adjustment / Scatter Gather Address."]
    #[must_use]
    #[inline(always)]
    pub const fn dlast_sga(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Destination Address Adjustment / Scatter Gather Address."]
    #[inline(always)]
    pub const fn set_dlast_sga(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdDlastSga {
    #[inline(always)]
    fn default() -> TcdDlastSga {
        TcdDlastSga(0)
    }
}
impl core::fmt::Debug for TcdDlastSga {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDlastSga")
            .field("dlast_sga", &self.dlast_sga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDlastSga {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDlastSga {{ dlast_sga: {=u32:?} }}", self.dlast_sga())
    }
}
#[doc = "TCD Signed Destination Address Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdDoff(pub u16);
impl TcdDoff {
    #[doc = "Destination Address Signed Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn doff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Destination Address Signed Offset."]
    #[inline(always)]
    pub const fn set_doff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdDoff {
    #[inline(always)]
    fn default() -> TcdDoff {
        TcdDoff(0)
    }
}
impl core::fmt::Debug for TcdDoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdDoff")
            .field("doff", &self.doff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdDoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdDoff {{ doff: {=u16:?} }}", self.doff())
    }
}
#[doc = "TCD Transfer Size Without Minor Loop Offsets."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffno(pub u32);
impl TcdNbytesMloffno {
    #[doc = "Number of Bytes To Transfer Per Service Request."]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Number of Bytes To Transfer Per Service Request."]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> TcdNbytesMloffnoDmloe {
        let val = (self.0 >> 30usize) & 0x01;
        TcdNbytesMloffnoDmloe::from_bits(val as u8)
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: TcdNbytesMloffnoDmloe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> TcdNbytesMloffnoSmloe {
        let val = (self.0 >> 31usize) & 0x01;
        TcdNbytesMloffnoSmloe::from_bits(val as u8)
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: TcdNbytesMloffnoSmloe) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffno {
    #[inline(always)]
    fn default() -> TcdNbytesMloffno {
        TcdNbytesMloffno(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffno")
            .field("nbytes", &self.nbytes())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffno {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffno {{ nbytes: {=u32:?}, dmloe: {:?}, smloe: {:?} }}",
            self.nbytes(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Transfer Size with Minor Loop Offsets."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdNbytesMloffyes(pub u32);
impl TcdNbytesMloffyes {
    #[doc = "Number of Bytes To Transfer Per Service Request."]
    #[must_use]
    #[inline(always)]
    pub const fn nbytes(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of Bytes To Transfer Per Service Request."]
    #[inline(always)]
    pub const fn set_nbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Minor Loop Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn mloff(&self) -> u32 {
        let val = (self.0 >> 10usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Minor Loop Offset."]
    #[inline(always)]
    pub const fn set_mloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 10usize)) | (((val as u32) & 0x000f_ffff) << 10usize);
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmloe(&self) -> TcdNbytesMloffyesDmloe {
        let val = (self.0 >> 30usize) & 0x01;
        TcdNbytesMloffyesDmloe::from_bits(val as u8)
    }
    #[doc = "Destination Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_dmloe(&mut self, val: TcdNbytesMloffyesDmloe) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn smloe(&self) -> TcdNbytesMloffyesSmloe {
        let val = (self.0 >> 31usize) & 0x01;
        TcdNbytesMloffyesSmloe::from_bits(val as u8)
    }
    #[doc = "Source Minor Loop Offset Enable."]
    #[inline(always)]
    pub const fn set_smloe(&mut self, val: TcdNbytesMloffyesSmloe) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for TcdNbytesMloffyes {
    #[inline(always)]
    fn default() -> TcdNbytesMloffyes {
        TcdNbytesMloffyes(0)
    }
}
impl core::fmt::Debug for TcdNbytesMloffyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdNbytesMloffyes")
            .field("nbytes", &self.nbytes())
            .field("mloff", &self.mloff())
            .field("dmloe", &self.dmloe())
            .field("smloe", &self.smloe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdNbytesMloffyes {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TcdNbytesMloffyes {{ nbytes: {=u16:?}, mloff: {=u32:?}, dmloe: {:?}, smloe: {:?} }}",
            self.nbytes(),
            self.mloff(),
            self.dmloe(),
            self.smloe()
        )
    }
}
#[doc = "TCD Source Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSaddr(pub u32);
impl TcdSaddr {
    #[doc = "Source Address."]
    #[must_use]
    #[inline(always)]
    pub const fn saddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Source Address."]
    #[inline(always)]
    pub const fn set_saddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSaddr {
    #[inline(always)]
    fn default() -> TcdSaddr {
        TcdSaddr(0)
    }
}
impl core::fmt::Debug for TcdSaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSaddr")
            .field("saddr", &self.saddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSaddr {{ saddr: {=u32:?} }}", self.saddr())
    }
}
#[doc = "TCD Last Source Address Adjustment / Store DADDR Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSlastSda(pub u32);
impl TcdSlastSda {
    #[doc = "Last Source Address Adjustment / Store DADDR Address."]
    #[must_use]
    #[inline(always)]
    pub const fn slast_sda(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Last Source Address Adjustment / Store DADDR Address."]
    #[inline(always)]
    pub const fn set_slast_sda(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TcdSlastSda {
    #[inline(always)]
    fn default() -> TcdSlastSda {
        TcdSlastSda(0)
    }
}
impl core::fmt::Debug for TcdSlastSda {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSlastSda")
            .field("slast_sda", &self.slast_sda())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSlastSda {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSlastSda {{ slast_sda: {=u32:?} }}", self.slast_sda())
    }
}
#[doc = "TCD Signed Source Address Offset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdSoff(pub u16);
impl TcdSoff {
    #[doc = "Source Address Signed Offset."]
    #[must_use]
    #[inline(always)]
    pub const fn soff(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Source Address Signed Offset."]
    #[inline(always)]
    pub const fn set_soff(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
    }
}
impl Default for TcdSoff {
    #[inline(always)]
    fn default() -> TcdSoff {
        TcdSoff(0)
    }
}
impl core::fmt::Debug for TcdSoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdSoff")
            .field("soff", &self.soff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdSoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TcdSoff {{ soff: {=u16:?} }}", self.soff())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls."]
    NoStall = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    EngineStallsFour = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    EngineStallsEight = 0x03,
}
impl Bwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwc {
    #[inline(always)]
    fn from(val: u8) -> Bwc {
        Bwc::from_bits(val)
    }
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(val: Bwc) -> u8 {
        Bwc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpa {
    #[doc = "Channel can suspend a lower-priority channel."]
    Suspend = 0x0,
    #[doc = "Channel cannot suspend any other channel, regardless of channel priority."]
    CannotSuspend = 0x01,
}
impl Dpa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpa {
    #[inline(always)]
    fn from(val: u8) -> Dpa {
        Dpa::from_bits(val)
    }
}
impl From<Dpa> for u8 {
    #[inline(always)]
    fn from(val: Dpa) -> u8 {
        Dpa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dreq {
    #[doc = "No operation."]
    ChannelNotAffected = 0x0,
    #[doc = "Clear the ERQ field to 0 upon major loop completion, thus disabling hardware service requests."]
    ErqFieldClear = 0x01,
}
impl Dreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dreq {
    #[inline(always)]
    fn from(val: u8) -> Dreq {
        Dreq::from_bits(val)
    }
}
impl From<Dreq> for u8 {
    #[inline(always)]
    fn from(val: Dreq) -> u8 {
        Dreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ecp {
    #[doc = "Channel cannot be suspended by a higher-priority channel's service request."]
    CannotSuspend = 0x0,
    #[doc = "Channel can be temporarily suspended by a higher-priority channel's service request."]
    Suspend = 0x01,
}
impl Ecp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ecp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ecp {
    #[inline(always)]
    fn from(val: u8) -> Ecp {
        Ecp::from_bits(val)
    }
}
impl From<Ecp> for u8 {
    #[inline(always)]
    fn from(val: Ecp) -> u8 {
        Ecp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Esg {
    #[doc = "Current channel's TCD is normal format."]
    NormalFormat = 0x0,
    #[doc = "Current channel's TCD specifies scatter/gather format."]
    ScatterGatherFormat = 0x01,
}
impl Esg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Esg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Esg {
    #[inline(always)]
    fn from(val: u8) -> Esg {
        Esg::from_bits(val)
    }
}
impl From<Esg> for u8 {
    #[inline(always)]
    fn from(val: Esg) -> u8 {
        Esg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pal {
    #[doc = "User protection level for DMA transfers."]
    UserProtection = 0x0,
    #[doc = "Privileged protection level for DMA transfers."]
    PrivilegedProtection = 0x01,
}
impl Pal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pal {
    #[inline(always)]
    fn from(val: u8) -> Pal {
        Pal::from_bits(val)
    }
}
impl From<Pal> for u8 {
    #[inline(always)]
    fn from(val: Pal) -> u8 {
        Pal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sec {
    #[doc = "Nonsecure protection level for DMA transfers."]
    NonsecureProtection = 0x0,
    #[doc = "Secure protection level for DMA transfers."]
    SecureProtection = 0x01,
}
impl Sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sec {
    #[inline(always)]
    fn from(val: u8) -> Sec {
        Sec::from_bits(val)
    }
}
impl From<Sec> for u8 {
    #[inline(always)]
    fn from(val: Sec) -> u8 {
        Sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Size {
    #[doc = "8-bit."]
    EightBit = 0x0,
    #[doc = "16-bit."]
    SixteenBit = 0x01,
    #[doc = "32-bit."]
    ThirtytwoBit = 0x02,
    #[doc = "64-bit."]
    SixtyfourBit = 0x03,
    #[doc = "16-byte."]
    SixteenByte = 0x04,
    #[doc = "32-byte."]
    ThirtytwoByte = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Size {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Size {
    #[inline(always)]
    fn from(val: u8) -> Size {
        Size::from_bits(val)
    }
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(val: Size) -> u8 {
        Size::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Start {
    #[doc = "Channel not explicitly started."]
    ChannelNotStarted = 0x0,
    #[doc = "Channel explicitly started via a software-initiated service request."]
    ChannelStarted = 0x01,
}
impl Start {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Start {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Start {
    #[inline(always)]
    fn from(val: u8) -> Start {
        Start::from_bits(val)
    }
}
impl From<Start> for u8 {
    #[inline(always)]
    fn from(val: Start) -> u8 {
        Start::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoDmloe {
    #[doc = "Minor loop offset not applied to DADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to DADDR."]
    OffsetApplied = 0x01,
}
impl TcdNbytesMloffnoDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoDmloe {
        TcdNbytesMloffnoDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoDmloe) -> u8 {
        TcdNbytesMloffnoDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffnoSmloe {
    #[doc = "Minor loop offset not applied to SADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to SADDR."]
    OffsetApplied = 0x01,
}
impl TcdNbytesMloffnoSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffnoSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffnoSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffnoSmloe {
        TcdNbytesMloffnoSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffnoSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffnoSmloe) -> u8 {
        TcdNbytesMloffnoSmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesDmloe {
    #[doc = "Minor loop offset not applied to DADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to DADDR."]
    OffsetApplied = 0x01,
}
impl TcdNbytesMloffyesDmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesDmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesDmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesDmloe {
        TcdNbytesMloffyesDmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesDmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesDmloe) -> u8 {
        TcdNbytesMloffyesDmloe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TcdNbytesMloffyesSmloe {
    #[doc = "Minor loop offset not applied to SADDR."]
    OffsetNotApplied = 0x0,
    #[doc = "Minor loop offset applied to SADDR."]
    OffsetApplied = 0x01,
}
impl TcdNbytesMloffyesSmloe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TcdNbytesMloffyesSmloe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TcdNbytesMloffyesSmloe {
    #[inline(always)]
    fn from(val: u8) -> TcdNbytesMloffyesSmloe {
        TcdNbytesMloffyesSmloe::from_bits(val)
    }
}
impl From<TcdNbytesMloffyesSmloe> for u8 {
    #[inline(always)]
    fn from(val: TcdNbytesMloffyesSmloe) -> u8 {
        TcdNbytesMloffyesSmloe::to_bits(val)
    }
}
