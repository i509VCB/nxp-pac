#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "DMA MP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma12 {
    ptr: *mut u8,
}
unsafe impl Send for Dma12 {}
unsafe impl Sync for Dma12 {}
impl Dma12 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Management Page Control."]
    #[inline(always)]
    pub const fn mp_csr(self) -> crate::pac::common::Reg<MpCsr12, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Management Page Error Status."]
    #[inline(always)]
    pub const fn mp_es(self) -> crate::pac::common::Reg<MpEs12, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status."]
    #[inline(always)]
    pub const fn mp_int(self) -> crate::pac::common::Reg<MpInt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status."]
    #[inline(always)]
    pub const fn mp_hrs(self) -> crate::pac::common::Reg<MpHrs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Arbitration Group."]
    #[inline(always)]
    pub const fn ch_grpri(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ChGrpri, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
}
#[doc = "DMA MP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4 {
    ptr: *mut u8,
}
unsafe impl Send for Dma4 {}
unsafe impl Sync for Dma4 {}
impl Dma4 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Management Page Control."]
    #[inline(always)]
    pub const fn mp_csr(self) -> crate::pac::common::Reg<MpCsr4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Management Page Error Status."]
    #[inline(always)]
    pub const fn mp_es(self) -> crate::pac::common::Reg<MpEs4, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status."]
    #[inline(always)]
    pub const fn mp_int(self) -> crate::pac::common::Reg<MpInt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status."]
    #[inline(always)]
    pub const fn mp_hrs(self) -> crate::pac::common::Reg<MpHrs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Arbitration Group."]
    #[inline(always)]
    pub const fn ch_grpri(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ChGrpri, crate::pac::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
}
#[doc = "DMA MP."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma8 {
    ptr: *mut u8,
}
unsafe impl Send for Dma8 {}
unsafe impl Sync for Dma8 {}
impl Dma8 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Management Page Control."]
    #[inline(always)]
    pub const fn mp_csr(self) -> crate::pac::common::Reg<MpCsr8, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Management Page Error Status."]
    #[inline(always)]
    pub const fn mp_es(self) -> crate::pac::common::Reg<MpEs8, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status."]
    #[inline(always)]
    pub const fn mp_int(self) -> crate::pac::common::Reg<MpInt, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status."]
    #[inline(always)]
    pub const fn mp_hrs(self) -> crate::pac::common::Reg<MpHrs, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel Arbitration Group."]
    #[inline(always)]
    pub const fn ch_grpri(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<ChGrpri, crate::pac::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
}
#[doc = "Channel Arbitration Group."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChGrpri(pub u32);
impl ChGrpri {
    #[doc = "Arbitration Group For Channel n."]
    #[must_use]
    #[inline(always)]
    pub const fn grpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Arbitration Group For Channel n."]
    #[inline(always)]
    pub const fn set_grpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for ChGrpri {
    #[inline(always)]
    fn default() -> ChGrpri {
        ChGrpri(0)
    }
}
impl core::fmt::Debug for ChGrpri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChGrpri")
            .field("grpri", &self.grpri())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChGrpri {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChGrpri {{ grpri: {=u8:?} }}", self.grpri())
    }
}
#[doc = "Management Page Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpCsr12(pub u32);
impl MpCsr12 {
    #[doc = "Enable Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug."]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Halt After Error."]
    #[must_use]
    #[inline(always)]
    pub const fn hae(&self) -> Hae {
        let val = (self.0 >> 4usize) & 0x01;
        Hae::from_bits(val as u8)
    }
    #[doc = "Halt After Error."]
    #[inline(always)]
    pub const fn set_hae(&mut self, val: Hae) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt DMA Operations."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Halt {
        let val = (self.0 >> 5usize) & 0x01;
        Halt::from_bits(val as u8)
    }
    #[doc = "Halt DMA Operations."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Channel Linking Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gclc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Channel Linking Control."]
    #[inline(always)]
    pub const fn set_gclc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Master ID Replication Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gmrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Master ID Replication Control."]
    #[inline(always)]
    pub const fn set_gmrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Cancel Transfer With Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpCsrEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpCsrEcx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer With Error."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpCsrEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Cancel Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> Cx {
        let val = (self.0 >> 9usize) & 0x01;
        Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer."]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: Cx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Active Channel ID."]
    #[must_use]
    #[inline(always)]
    pub const fn active_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Active Channel ID."]
    #[inline(always)]
    pub const fn set_active_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "DMA Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 31usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "DMA Active Status."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpCsr12 {
    #[inline(always)]
    fn default() -> MpCsr12 {
        MpCsr12(0)
    }
}
impl core::fmt::Debug for MpCsr12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpCsr12")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hae", &self.hae())
            .field("halt", &self.halt())
            .field("gclc", &self.gclc())
            .field("gmrc", &self.gmrc())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active_id", &self.active_id())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpCsr12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpCsr12 {{ edbg: {=bool:?}, erca: {=bool:?}, hae: {:?}, halt: {:?}, gclc: {=bool:?}, gmrc: {=bool:?}, ecx: {:?}, cx: {:?}, active_id: {=u8:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.hae(),
            self.halt(),
            self.gclc(),
            self.gmrc(),
            self.ecx(),
            self.cx(),
            self.active_id(),
            self.active()
        )
    }
}
#[doc = "Management Page Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpCsr4(pub u32);
impl MpCsr4 {
    #[doc = "Enable Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug."]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Halt After Error."]
    #[must_use]
    #[inline(always)]
    pub const fn hae(&self) -> Hae {
        let val = (self.0 >> 4usize) & 0x01;
        Hae::from_bits(val as u8)
    }
    #[doc = "Halt After Error."]
    #[inline(always)]
    pub const fn set_hae(&mut self, val: Hae) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt DMA Operations."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Halt {
        let val = (self.0 >> 5usize) & 0x01;
        Halt::from_bits(val as u8)
    }
    #[doc = "Halt DMA Operations."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Channel Linking Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gclc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Channel Linking Control."]
    #[inline(always)]
    pub const fn set_gclc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Master ID Replication Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gmrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Master ID Replication Control."]
    #[inline(always)]
    pub const fn set_gmrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Cancel Transfer With Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpCsrEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpCsrEcx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer With Error."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpCsrEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Cancel Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> Cx {
        let val = (self.0 >> 9usize) & 0x01;
        Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer."]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: Cx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Active Channel ID."]
    #[must_use]
    #[inline(always)]
    pub const fn active_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Active Channel ID."]
    #[inline(always)]
    pub const fn set_active_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "DMA Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 31usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "DMA Active Status."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpCsr4 {
    #[inline(always)]
    fn default() -> MpCsr4 {
        MpCsr4(0)
    }
}
impl core::fmt::Debug for MpCsr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpCsr4")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hae", &self.hae())
            .field("halt", &self.halt())
            .field("gclc", &self.gclc())
            .field("gmrc", &self.gmrc())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active_id", &self.active_id())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpCsr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpCsr4 {{ edbg: {=bool:?}, erca: {=bool:?}, hae: {:?}, halt: {:?}, gclc: {=bool:?}, gmrc: {=bool:?}, ecx: {:?}, cx: {:?}, active_id: {=u8:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.hae(),
            self.halt(),
            self.gclc(),
            self.gmrc(),
            self.ecx(),
            self.cx(),
            self.active_id(),
            self.active()
        )
    }
}
#[doc = "Management Page Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpCsr8(pub u32);
impl MpCsr8 {
    #[doc = "Enable Debug."]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug."]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration."]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Halt After Error."]
    #[must_use]
    #[inline(always)]
    pub const fn hae(&self) -> Hae {
        let val = (self.0 >> 4usize) & 0x01;
        Hae::from_bits(val as u8)
    }
    #[doc = "Halt After Error."]
    #[inline(always)]
    pub const fn set_hae(&mut self, val: Hae) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt DMA Operations."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Halt {
        let val = (self.0 >> 5usize) & 0x01;
        Halt::from_bits(val as u8)
    }
    #[doc = "Halt DMA Operations."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Halt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Channel Linking Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gclc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Channel Linking Control."]
    #[inline(always)]
    pub const fn set_gclc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Master ID Replication Control."]
    #[must_use]
    #[inline(always)]
    pub const fn gmrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Master ID Replication Control."]
    #[inline(always)]
    pub const fn set_gmrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Cancel Transfer With Error."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpCsrEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpCsrEcx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer With Error."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpCsrEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Cancel Transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn cx(&self) -> Cx {
        let val = (self.0 >> 9usize) & 0x01;
        Cx::from_bits(val as u8)
    }
    #[doc = "Cancel Transfer."]
    #[inline(always)]
    pub const fn set_cx(&mut self, val: Cx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Active Channel ID."]
    #[must_use]
    #[inline(always)]
    pub const fn active_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Active Channel ID."]
    #[inline(always)]
    pub const fn set_active_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "DMA Active Status."]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> Active {
        let val = (self.0 >> 31usize) & 0x01;
        Active::from_bits(val as u8)
    }
    #[doc = "DMA Active Status."]
    #[inline(always)]
    pub const fn set_active(&mut self, val: Active) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpCsr8 {
    #[inline(always)]
    fn default() -> MpCsr8 {
        MpCsr8(0)
    }
}
impl core::fmt::Debug for MpCsr8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpCsr8")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hae", &self.hae())
            .field("halt", &self.halt())
            .field("gclc", &self.gclc())
            .field("gmrc", &self.gmrc())
            .field("ecx", &self.ecx())
            .field("cx", &self.cx())
            .field("active_id", &self.active_id())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpCsr8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpCsr8 {{ edbg: {=bool:?}, erca: {=bool:?}, hae: {:?}, halt: {:?}, gclc: {=bool:?}, gmrc: {=bool:?}, ecx: {:?}, cx: {:?}, active_id: {=u8:?}, active: {:?} }}",
            self.edbg(),
            self.erca(),
            self.hae(),
            self.halt(),
            self.gclc(),
            self.gmrc(),
            self.ecx(),
            self.cx(),
            self.active_id(),
            self.active()
        )
    }
}
#[doc = "Management Page Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpEs12(pub u32);
impl MpEs12 {
    #[doc = "Destination Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> Dbe {
        let val = (self.0 >> 0usize) & 0x01;
        Dbe::from_bits(val as u8)
    }
    #[doc = "Destination Bus Error."]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: Dbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> Sbe {
        let val = (self.0 >> 1usize) & 0x01;
        Sbe::from_bits(val as u8)
    }
    #[doc = "Source Bus Error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: Sbe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> Sge {
        let val = (self.0 >> 2usize) & 0x01;
        Sge::from_bits(val as u8)
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: Sge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> Nce {
        let val = (self.0 >> 3usize) & 0x01;
        Nce::from_bits(val as u8)
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: Nce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> Doe {
        let val = (self.0 >> 4usize) & 0x01;
        Doe::from_bits(val as u8)
    }
    #[doc = "Destination Offset Error."]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: Doe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> Dae {
        let val = (self.0 >> 5usize) & 0x01;
        Dae::from_bits(val as u8)
    }
    #[doc = "Destination Address Error."]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: Dae) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> Soe {
        let val = (self.0 >> 6usize) & 0x01;
        Soe::from_bits(val as u8)
    }
    #[doc = "Source Offset Error."]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: Soe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> Sae {
        let val = (self.0 >> 7usize) & 0x01;
        Sae::from_bits(val as u8)
    }
    #[doc = "Source Address Error."]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: Sae) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer Canceled."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpEsEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpEsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpEsEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> Vld {
        let val = (self.0 >> 31usize) & 0x01;
        Vld::from_bits(val as u8)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: Vld) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpEs12 {
    #[inline(always)]
    fn default() -> MpEs12 {
        MpEs12(0)
    }
}
impl core::fmt::Debug for MpEs12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpEs12")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("ecx", &self.ecx())
            .field("errchn", &self.errchn())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpEs12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpEs12 {{ dbe: {:?}, sbe: {:?}, sge: {:?}, nce: {:?}, doe: {:?}, dae: {:?}, soe: {:?}, sae: {:?}, ecx: {:?}, errchn: {=u8:?}, vld: {:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.ecx(),
            self.errchn(),
            self.vld()
        )
    }
}
#[doc = "Management Page Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpEs4(pub u32);
impl MpEs4 {
    #[doc = "Destination Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> Dbe {
        let val = (self.0 >> 0usize) & 0x01;
        Dbe::from_bits(val as u8)
    }
    #[doc = "Destination Bus Error."]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: Dbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> Sbe {
        let val = (self.0 >> 1usize) & 0x01;
        Sbe::from_bits(val as u8)
    }
    #[doc = "Source Bus Error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: Sbe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> Sge {
        let val = (self.0 >> 2usize) & 0x01;
        Sge::from_bits(val as u8)
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: Sge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> Nce {
        let val = (self.0 >> 3usize) & 0x01;
        Nce::from_bits(val as u8)
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: Nce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> Doe {
        let val = (self.0 >> 4usize) & 0x01;
        Doe::from_bits(val as u8)
    }
    #[doc = "Destination Offset Error."]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: Doe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> Dae {
        let val = (self.0 >> 5usize) & 0x01;
        Dae::from_bits(val as u8)
    }
    #[doc = "Destination Address Error."]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: Dae) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> Soe {
        let val = (self.0 >> 6usize) & 0x01;
        Soe::from_bits(val as u8)
    }
    #[doc = "Source Offset Error."]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: Soe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> Sae {
        let val = (self.0 >> 7usize) & 0x01;
        Sae::from_bits(val as u8)
    }
    #[doc = "Source Address Error."]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: Sae) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer Canceled."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpEsEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpEsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpEsEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> Vld {
        let val = (self.0 >> 31usize) & 0x01;
        Vld::from_bits(val as u8)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: Vld) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpEs4 {
    #[inline(always)]
    fn default() -> MpEs4 {
        MpEs4(0)
    }
}
impl core::fmt::Debug for MpEs4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpEs4")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("ecx", &self.ecx())
            .field("errchn", &self.errchn())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpEs4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpEs4 {{ dbe: {:?}, sbe: {:?}, sge: {:?}, nce: {:?}, doe: {:?}, dae: {:?}, soe: {:?}, sae: {:?}, ecx: {:?}, errchn: {=u8:?}, vld: {:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.ecx(),
            self.errchn(),
            self.vld()
        )
    }
}
#[doc = "Management Page Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpEs8(pub u32);
impl MpEs8 {
    #[doc = "Destination Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> Dbe {
        let val = (self.0 >> 0usize) & 0x01;
        Dbe::from_bits(val as u8)
    }
    #[doc = "Destination Bus Error."]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: Dbe) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> Sbe {
        let val = (self.0 >> 1usize) & 0x01;
        Sbe::from_bits(val as u8)
    }
    #[doc = "Source Bus Error."]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: Sbe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> Sge {
        let val = (self.0 >> 2usize) & 0x01;
        Sge::from_bits(val as u8)
    }
    #[doc = "Scatter/Gather Configuration Error."]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: Sge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> Nce {
        let val = (self.0 >> 3usize) & 0x01;
        Nce::from_bits(val as u8)
    }
    #[doc = "NBYTES/CITER Configuration Error."]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: Nce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> Doe {
        let val = (self.0 >> 4usize) & 0x01;
        Doe::from_bits(val as u8)
    }
    #[doc = "Destination Offset Error."]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: Doe) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> Dae {
        let val = (self.0 >> 5usize) & 0x01;
        Dae::from_bits(val as u8)
    }
    #[doc = "Destination Address Error."]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: Dae) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error."]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> Soe {
        let val = (self.0 >> 6usize) & 0x01;
        Soe::from_bits(val as u8)
    }
    #[doc = "Source Offset Error."]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: Soe) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> Sae {
        let val = (self.0 >> 7usize) & 0x01;
        Sae::from_bits(val as u8)
    }
    #[doc = "Source Address Error."]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: Sae) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer Canceled."]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> MpEsEcx {
        let val = (self.0 >> 8usize) & 0x01;
        MpEsEcx::from_bits(val as u8)
    }
    #[doc = "Transfer Canceled."]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: MpEsEcx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number."]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Valid."]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> Vld {
        let val = (self.0 >> 31usize) & 0x01;
        Vld::from_bits(val as u8)
    }
    #[doc = "Valid."]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: Vld) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for MpEs8 {
    #[inline(always)]
    fn default() -> MpEs8 {
        MpEs8(0)
    }
}
impl core::fmt::Debug for MpEs8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpEs8")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("ecx", &self.ecx())
            .field("errchn", &self.errchn())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpEs8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MpEs8 {{ dbe: {:?}, sbe: {:?}, sge: {:?}, nce: {:?}, doe: {:?}, dae: {:?}, soe: {:?}, sae: {:?}, ecx: {:?}, errchn: {=u8:?}, vld: {:?} }}",
            self.dbe(),
            self.sbe(),
            self.sge(),
            self.nce(),
            self.doe(),
            self.dae(),
            self.soe(),
            self.sae(),
            self.ecx(),
            self.errchn(),
            self.vld()
        )
    }
}
#[doc = "Management Page Hardware Request Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpHrs(pub u32);
impl MpHrs {
    #[doc = "Hardware Request Status."]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Hardware Request Status."]
    #[inline(always)]
    pub const fn set_hrs(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MpHrs {
    #[inline(always)]
    fn default() -> MpHrs {
        MpHrs(0)
    }
}
impl core::fmt::Debug for MpHrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpHrs").field("hrs", &self.hrs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrs {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MpHrs {{ hrs: {=u32:?} }}", self.hrs())
    }
}
#[doc = "Management Page Interrupt Request Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpInt(pub u32);
impl MpInt {
    #[doc = "Interrupt Request Status."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Interrupt Request Status."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for MpInt {
    #[inline(always)]
    fn default() -> MpInt {
        MpInt(0)
    }
}
impl core::fmt::Debug for MpInt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpInt").field("int", &self.int()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpInt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MpInt {{ int: {=u16:?} }}", self.int())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "eDMA is idle."]
    Idle = 0x0,
    #[doc = "eDMA is executing a channel."]
    Execution = 0x01,
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
pub enum Cx {
    #[doc = "Normal operation."]
    NormalOperation = 0x0,
    #[doc = "Cancel the remaining data transfer."]
    DataTransferCancel = 0x01,
}
impl Cx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cx {
    #[inline(always)]
    fn from(val: u8) -> Cx {
        Cx::from_bits(val)
    }
}
impl From<Cx> for u8 {
    #[inline(always)]
    fn from(val: Cx) -> u8 {
        Cx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dae {
    #[doc = "No destination address configuration error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DADDR field."]
    ConfigurationError = 0x01,
}
impl Dae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dae {
    #[inline(always)]
    fn from(val: u8) -> Dae {
        Dae::from_bits(val)
    }
}
impl From<Dae> for u8 {
    #[inline(always)]
    fn from(val: Dae) -> u8 {
        Dae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbe {
    #[doc = "No destination bus error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a bus error on a destination write."]
    BusError = 0x01,
}
impl Dbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbe {
    #[inline(always)]
    fn from(val: u8) -> Dbe {
        Dbe::from_bits(val)
    }
}
impl From<Dbe> for u8 {
    #[inline(always)]
    fn from(val: Dbe) -> u8 {
        Dbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Doe {
    #[doc = "No destination offset configuration error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DOFF field."]
    ConfigurationError = 0x01,
}
impl Doe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Doe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Doe {
    #[inline(always)]
    fn from(val: u8) -> Doe {
        Doe::from_bits(val)
    }
}
impl From<Doe> for u8 {
    #[inline(always)]
    fn from(val: Doe) -> u8 {
        Doe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hae {
    #[doc = "Normal operation."]
    NormalOperation = 0x0,
    #[doc = "Any error causes the HALT field to be set to 1."]
    Halt = 0x01,
}
impl Hae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hae {
    #[inline(always)]
    fn from(val: u8) -> Hae {
        Hae::from_bits(val)
    }
}
impl From<Hae> for u8 {
    #[inline(always)]
    fn from(val: Hae) -> u8 {
        Hae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Halt {
    #[doc = "Normal operation."]
    NormalOperation = 0x0,
    #[doc = "Stall the start of any new channels."]
    Stall = 0x01,
}
impl Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Halt {
    #[inline(always)]
    fn from(val: u8) -> Halt {
        Halt::from_bits(val)
    }
}
impl From<Halt> for u8 {
    #[inline(always)]
    fn from(val: Halt) -> u8 {
        Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MpCsrEcx {
    #[doc = "Normal operation."]
    NORMAL_OPERATION = 0x0,
    #[doc = "Cancel the remaining data transfer."]
    CANCEL = 0x01,
}
impl MpCsrEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MpCsrEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MpCsrEcx {
    #[inline(always)]
    fn from(val: u8) -> MpCsrEcx {
        MpCsrEcx::from_bits(val)
    }
}
impl From<MpCsrEcx> for u8 {
    #[inline(always)]
    fn from(val: MpCsrEcx) -> u8 {
        MpCsrEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MpEsEcx {
    #[doc = "No canceled transfers."]
    NO_CANCELED_TRANSFERS = 0x0,
    #[doc = "Last recorded entry was a canceled transfer by the error cancel transfer input."]
    CANCELED_TRANSFER = 0x01,
}
impl MpEsEcx {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MpEsEcx {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MpEsEcx {
    #[inline(always)]
    fn from(val: u8) -> MpEsEcx {
        MpEsEcx::from_bits(val)
    }
}
impl From<MpEsEcx> for u8 {
    #[inline(always)]
    fn from(val: MpEsEcx) -> u8 {
        MpEsEcx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nce {
    #[doc = "No NBYTES/CITER configuration error."]
    NoError = 0x0,
    #[doc = "The last recorded error was NBYTES equal to zero or a CITER not equal to BITER error."]
    ConfigurationError = 0x01,
}
impl Nce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nce {
    #[inline(always)]
    fn from(val: u8) -> Nce {
        Nce::from_bits(val)
    }
}
impl From<Nce> for u8 {
    #[inline(always)]
    fn from(val: Nce) -> u8 {
        Nce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sae {
    #[doc = "No source address configuration error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SADDR field."]
    ConfigurationError = 0x01,
}
impl Sae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sae {
    #[inline(always)]
    fn from(val: u8) -> Sae {
        Sae::from_bits(val)
    }
}
impl From<Sae> for u8 {
    #[inline(always)]
    fn from(val: Sae) -> u8 {
        Sae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sbe {
    #[doc = "No source bus error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a bus error on a source read."]
    BusError = 0x01,
}
impl Sbe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sbe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sbe {
    #[inline(always)]
    fn from(val: u8) -> Sbe {
        Sbe::from_bits(val)
    }
}
impl From<Sbe> for u8 {
    #[inline(always)]
    fn from(val: Sbe) -> u8 {
        Sbe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sge {
    #[doc = "No scatter/gather configuration error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_DLAST_SGA field."]
    ConfigurationError = 0x01,
}
impl Sge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sge {
    #[inline(always)]
    fn from(val: u8) -> Sge {
        Sge::from_bits(val)
    }
}
impl From<Sge> for u8 {
    #[inline(always)]
    fn from(val: Sge) -> u8 {
        Sge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Soe {
    #[doc = "No source offset configuration error."]
    NoError = 0x0,
    #[doc = "Last recorded error was a configuration error detected in the TCDn_SOFF field."]
    ConfigurationError = 0x01,
}
impl Soe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Soe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Soe {
    #[inline(always)]
    fn from(val: u8) -> Soe {
        Soe::from_bits(val)
    }
}
impl From<Soe> for u8 {
    #[inline(always)]
    fn from(val: Soe) -> u8 {
        Soe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vld {
    #[doc = "No CHn_ES\\[ERR\\] fields are set to 1."]
    NoFieldSetOne = 0x0,
    #[doc = "At least one CHn_ES\\[ERR\\] field is set to 1, indicating a valid error exists that software has not cleared."]
    AtleastOneField = 0x01,
}
impl Vld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vld {
    #[inline(always)]
    fn from(val: u8) -> Vld {
        Vld::from_bits(val)
    }
}
impl From<Vld> for u8 {
    #[inline(always)]
    fn from(val: Vld) -> u8 {
        Vld::to_bits(val)
    }
}
