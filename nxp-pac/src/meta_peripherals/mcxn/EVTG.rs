#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "EVTG."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evtg {
    ptr: *mut u8,
}
unsafe impl Send for Evtg {}
unsafe impl Sync for Evtg {}
impl Evtg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: EVTG_AOI0_BFT01, EVTG_AOI0_BFT23, EVTG_AOI1_BFT01, EVTG_AOI1_BFT23, EVTG_CTRL, EVTG_AOI0_FILT, EVTG_AOI1_FILT."]
    #[inline(always)]
    pub const fn evtg_inst(self, n: usize) -> EvtgInst {
        assert!(n < 4usize);
        unsafe { EvtgInst::from_ptr(self.ptr.wrapping_add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "Array of registers: EVTG_AOI0_BFT01, EVTG_AOI0_BFT23, EVTG_AOI1_BFT01, EVTG_AOI1_BFT23, EVTG_CTRL, EVTG_AOI0_FILT, EVTG_AOI1_FILT."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgInst {
    ptr: *mut u8,
}
unsafe impl Send for EvtgInst {}
unsafe impl Sync for EvtgInst {}
impl EvtgInst {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "AOI0 Boolean Function Term 0 and 1 Configuration."]
    #[inline(always)]
    pub const fn evtg_aoi0_bft01(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi0Bft01, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "AOI0 Boolean Function Term 2 and 3 Configuration."]
    #[inline(always)]
    pub const fn evtg_aoi0_bft23(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi0Bft23, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "AOI1 Boolean Function Term 0 and 1 Configuration."]
    #[inline(always)]
    pub const fn evtg_aoi1_bft01(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi1Bft01, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "AOI1 Boolean Function Term 2 and 3 Configuration."]
    #[inline(always)]
    pub const fn evtg_aoi1_bft23(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi1Bft23, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Control and Status."]
    #[inline(always)]
    pub const fn evtg_ctrl(self) -> crate::pac::common::Reg<EvtgCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "AOI0 Output Filter."]
    #[inline(always)]
    pub const fn evtg_aoi0_filt(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi0Filt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AOI1 Output Filter."]
    #[inline(always)]
    pub const fn evtg_aoi1_filt(
        self,
    ) -> crate::pac::common::Reg<EvtgAoi1Filt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
}
#[doc = "AOI0 Boolean Function Term 0 and 1 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Bft01(pub u16);
impl EvtgAoi0Bft01 {
    #[doc = "Product Term 1, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> EvtgAoi0Bft01Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        EvtgAoi0Bft01Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: EvtgAoi0Bft01Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> EvtgAoi0Bft01Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        EvtgAoi0Bft01Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: EvtgAoi0Bft01Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> EvtgAoi0Bft01Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        EvtgAoi0Bft01Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: EvtgAoi0Bft01Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> EvtgAoi0Bft01Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        EvtgAoi0Bft01Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: EvtgAoi0Bft01Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> EvtgAoi0Bft01Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        EvtgAoi0Bft01Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: EvtgAoi0Bft01Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> EvtgAoi0Bft01Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        EvtgAoi0Bft01Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: EvtgAoi0Bft01Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> EvtgAoi0Bft01Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        EvtgAoi0Bft01Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: EvtgAoi0Bft01Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> EvtgAoi0Bft01Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        EvtgAoi0Bft01Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: EvtgAoi0Bft01Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi0Bft01 {
    #[inline(always)]
    fn default() -> EvtgAoi0Bft01 {
        EvtgAoi0Bft01(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Bft01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Bft01")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi0Bft01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Bft01 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "AOI0 Boolean Function Term 2 and 3 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Bft23(pub u16);
impl EvtgAoi0Bft23 {
    #[doc = "Product Term 3, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> EvtgAoi0Bft23Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        EvtgAoi0Bft23Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: EvtgAoi0Bft23Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> EvtgAoi0Bft23Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        EvtgAoi0Bft23Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: EvtgAoi0Bft23Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> EvtgAoi0Bft23Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        EvtgAoi0Bft23Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: EvtgAoi0Bft23Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> EvtgAoi0Bft23Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        EvtgAoi0Bft23Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: EvtgAoi0Bft23Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> EvtgAoi0Bft23Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        EvtgAoi0Bft23Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: EvtgAoi0Bft23Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> EvtgAoi0Bft23Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        EvtgAoi0Bft23Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: EvtgAoi0Bft23Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> EvtgAoi0Bft23Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        EvtgAoi0Bft23Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: EvtgAoi0Bft23Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> EvtgAoi0Bft23Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        EvtgAoi0Bft23Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: EvtgAoi0Bft23Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi0Bft23 {
    #[inline(always)]
    fn default() -> EvtgAoi0Bft23 {
        EvtgAoi0Bft23(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Bft23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Bft23")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi0Bft23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Bft23 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[doc = "AOI0 Output Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi0Filt(pub u16);
impl EvtgAoi0Filt {
    #[doc = "Output Filter Sample Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output Filter Sample Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Output Filter Sample Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Output Filter Sample Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for EvtgAoi0Filt {
    #[inline(always)]
    fn default() -> EvtgAoi0Filt {
        EvtgAoi0Filt(0)
    }
}
impl core::fmt::Debug for EvtgAoi0Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi0Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi0Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi0Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "AOI1 Boolean Function Term 0 and 1 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Bft01(pub u16);
impl EvtgAoi1Bft01 {
    #[doc = "Product Term 1, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_dc(&self) -> EvtgAoi1Bft01Pt1Dc {
        let val = (self.0 >> 0usize) & 0x03;
        EvtgAoi1Bft01Pt1Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_dc(&mut self, val: EvtgAoi1Bft01Pt1Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 1, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_cc(&self) -> EvtgAoi1Bft01Pt1Cc {
        let val = (self.0 >> 2usize) & 0x03;
        EvtgAoi1Bft01Pt1Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_cc(&mut self, val: EvtgAoi1Bft01Pt1Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 1, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_bc(&self) -> EvtgAoi1Bft01Pt1Bc {
        let val = (self.0 >> 4usize) & 0x03;
        EvtgAoi1Bft01Pt1Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 1, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_bc(&mut self, val: EvtgAoi1Bft01Pt1Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 1, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt1_ac(&self) -> EvtgAoi1Bft01Pt1Ac {
        let val = (self.0 >> 6usize) & 0x03;
        EvtgAoi1Bft01Pt1Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 1, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt1_ac(&mut self, val: EvtgAoi1Bft01Pt1Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 0, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_dc(&self) -> EvtgAoi1Bft01Pt0Dc {
        let val = (self.0 >> 8usize) & 0x03;
        EvtgAoi1Bft01Pt0Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_dc(&mut self, val: EvtgAoi1Bft01Pt0Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 0, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_cc(&self) -> EvtgAoi1Bft01Pt0Cc {
        let val = (self.0 >> 10usize) & 0x03;
        EvtgAoi1Bft01Pt0Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_cc(&mut self, val: EvtgAoi1Bft01Pt0Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 0, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_bc(&self) -> EvtgAoi1Bft01Pt0Bc {
        let val = (self.0 >> 12usize) & 0x03;
        EvtgAoi1Bft01Pt0Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 0, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_bc(&mut self, val: EvtgAoi1Bft01Pt0Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 0, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt0_ac(&self) -> EvtgAoi1Bft01Pt0Ac {
        let val = (self.0 >> 14usize) & 0x03;
        EvtgAoi1Bft01Pt0Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 0, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt0_ac(&mut self, val: EvtgAoi1Bft01Pt0Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi1Bft01 {
    #[inline(always)]
    fn default() -> EvtgAoi1Bft01 {
        EvtgAoi1Bft01(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Bft01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Bft01")
            .field("pt1_dc", &self.pt1_dc())
            .field("pt1_cc", &self.pt1_cc())
            .field("pt1_bc", &self.pt1_bc())
            .field("pt1_ac", &self.pt1_ac())
            .field("pt0_dc", &self.pt0_dc())
            .field("pt0_cc", &self.pt0_cc())
            .field("pt0_bc", &self.pt0_bc())
            .field("pt0_ac", &self.pt0_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi1Bft01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Bft01 {{ pt1_dc: {:?}, pt1_cc: {:?}, pt1_bc: {:?}, pt1_ac: {:?}, pt0_dc: {:?}, pt0_cc: {:?}, pt0_bc: {:?}, pt0_ac: {:?} }}",
            self.pt1_dc(),
            self.pt1_cc(),
            self.pt1_bc(),
            self.pt1_ac(),
            self.pt0_dc(),
            self.pt0_cc(),
            self.pt0_bc(),
            self.pt0_ac()
        )
    }
}
#[doc = "AOI1 Boolean Function Term 2 and 3 Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Bft23(pub u16);
impl EvtgAoi1Bft23 {
    #[doc = "Product Term 3, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_dc(&self) -> EvtgAoi1Bft23Pt3Dc {
        let val = (self.0 >> 0usize) & 0x03;
        EvtgAoi1Bft23Pt3Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_dc(&mut self, val: EvtgAoi1Bft23Pt3Dc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u16) & 0x03) << 0usize);
    }
    #[doc = "Product Term 3, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_cc(&self) -> EvtgAoi1Bft23Pt3Cc {
        let val = (self.0 >> 2usize) & 0x03;
        EvtgAoi1Bft23Pt3Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_cc(&mut self, val: EvtgAoi1Bft23Pt3Cc) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u16) & 0x03) << 2usize);
    }
    #[doc = "Product Term 3, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_bc(&self) -> EvtgAoi1Bft23Pt3Bc {
        let val = (self.0 >> 4usize) & 0x03;
        EvtgAoi1Bft23Pt3Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 3, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_bc(&mut self, val: EvtgAoi1Bft23Pt3Bc) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u16) & 0x03) << 4usize);
    }
    #[doc = "Product Term 3, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt3_ac(&self) -> EvtgAoi1Bft23Pt3Ac {
        let val = (self.0 >> 6usize) & 0x03;
        EvtgAoi1Bft23Pt3Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 3, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt3_ac(&mut self, val: EvtgAoi1Bft23Pt3Ac) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Product Term 2, D Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_dc(&self) -> EvtgAoi1Bft23Pt2Dc {
        let val = (self.0 >> 8usize) & 0x03;
        EvtgAoi1Bft23Pt2Dc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, D Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_dc(&mut self, val: EvtgAoi1Bft23Pt2Dc) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u16) & 0x03) << 8usize);
    }
    #[doc = "Product Term 2, C Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_cc(&self) -> EvtgAoi1Bft23Pt2Cc {
        let val = (self.0 >> 10usize) & 0x03;
        EvtgAoi1Bft23Pt2Cc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, C Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_cc(&mut self, val: EvtgAoi1Bft23Pt2Cc) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u16) & 0x03) << 10usize);
    }
    #[doc = "Product Term 2, B Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_bc(&self) -> EvtgAoi1Bft23Pt2Bc {
        let val = (self.0 >> 12usize) & 0x03;
        EvtgAoi1Bft23Pt2Bc::from_bits(val as u8)
    }
    #[doc = "Product Term 2, B Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_bc(&mut self, val: EvtgAoi1Bft23Pt2Bc) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
    #[doc = "Product Term 2, A Input Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn pt2_ac(&self) -> EvtgAoi1Bft23Pt2Ac {
        let val = (self.0 >> 14usize) & 0x03;
        EvtgAoi1Bft23Pt2Ac::from_bits(val as u8)
    }
    #[doc = "Product Term 2, A Input Configuration."]
    #[inline(always)]
    pub const fn set_pt2_ac(&mut self, val: EvtgAoi1Bft23Pt2Ac) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for EvtgAoi1Bft23 {
    #[inline(always)]
    fn default() -> EvtgAoi1Bft23 {
        EvtgAoi1Bft23(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Bft23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Bft23")
            .field("pt3_dc", &self.pt3_dc())
            .field("pt3_cc", &self.pt3_cc())
            .field("pt3_bc", &self.pt3_bc())
            .field("pt3_ac", &self.pt3_ac())
            .field("pt2_dc", &self.pt2_dc())
            .field("pt2_cc", &self.pt2_cc())
            .field("pt2_bc", &self.pt2_bc())
            .field("pt2_ac", &self.pt2_ac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi1Bft23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Bft23 {{ pt3_dc: {:?}, pt3_cc: {:?}, pt3_bc: {:?}, pt3_ac: {:?}, pt2_dc: {:?}, pt2_cc: {:?}, pt2_bc: {:?}, pt2_ac: {:?} }}",
            self.pt3_dc(),
            self.pt3_cc(),
            self.pt3_bc(),
            self.pt3_ac(),
            self.pt2_dc(),
            self.pt2_cc(),
            self.pt2_bc(),
            self.pt2_ac()
        )
    }
}
#[doc = "AOI1 Output Filter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgAoi1Filt(pub u16);
impl EvtgAoi1Filt {
    #[doc = "Output Filter Sample Period."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_per(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Output Filter Sample Period."]
    #[inline(always)]
    pub const fn set_filt_per(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "Output Filter Sample Count."]
    #[must_use]
    #[inline(always)]
    pub const fn filt_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Output Filter Sample Count."]
    #[inline(always)]
    pub const fn set_filt_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u16) & 0x07) << 8usize);
    }
}
impl Default for EvtgAoi1Filt {
    #[inline(always)]
    fn default() -> EvtgAoi1Filt {
        EvtgAoi1Filt(0)
    }
}
impl core::fmt::Debug for EvtgAoi1Filt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgAoi1Filt")
            .field("filt_per", &self.filt_per())
            .field("filt_cnt", &self.filt_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgAoi1Filt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgAoi1Filt {{ filt_per: {=u8:?}, filt_cnt: {=u8:?} }}",
            self.filt_per(),
            self.filt_cnt()
        )
    }
}
#[doc = "Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EvtgCtrl(pub u16);
impl EvtgCtrl {
    #[doc = "Flip flop Initial Value Configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn ff_init(&self) -> FfInit {
        let val = (self.0 >> 0usize) & 0x01;
        FfInit::from_bits(val as u8)
    }
    #[doc = "Flip flop Initial Value Configuration."]
    #[inline(always)]
    pub const fn set_ff_init(&mut self, val: FfInit) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "Flip-Flop Initial Output Enable Control."]
    #[must_use]
    #[inline(always)]
    pub const fn init_en(&self) -> InitEn {
        let val = (self.0 >> 1usize) & 0x01;
        InitEn::from_bits(val as u8)
    }
    #[doc = "Flip-Flop Initial Output Enable Control."]
    #[inline(always)]
    pub const fn set_init_en(&mut self, val: InitEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "Flip-Flop Mode Selection."]
    #[must_use]
    #[inline(always)]
    pub const fn mode_sel(&self) -> ModeSel {
        let val = (self.0 >> 2usize) & 0x07;
        ModeSel::from_bits(val as u8)
    }
    #[doc = "Flip-Flop Mode Selection."]
    #[inline(always)]
    pub const fn set_mode_sel(&mut self, val: ModeSel) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u16) & 0x07) << 2usize);
    }
    #[doc = "EVTG Output Feedback Override Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fb_ovrd(&self) -> FbOvrd {
        let val = (self.0 >> 6usize) & 0x03;
        FbOvrd::from_bits(val as u8)
    }
    #[doc = "EVTG Output Feedback Override Control."]
    #[inline(always)]
    pub const fn set_fb_ovrd(&mut self, val: FbOvrd) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u16) & 0x03) << 6usize);
    }
    #[doc = "Synchronize Control."]
    #[must_use]
    #[inline(always)]
    pub const fn sync_ctrl(&self) -> SyncCtrl {
        let val = (self.0 >> 8usize) & 0x0f;
        SyncCtrl::from_bits(val as u8)
    }
    #[doc = "Synchronize Control."]
    #[inline(always)]
    pub const fn set_sync_ctrl(&mut self, val: SyncCtrl) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "Force Bypass Control."]
    #[must_use]
    #[inline(always)]
    pub const fn force_bypass(&self) -> ForceBypass {
        let val = (self.0 >> 12usize) & 0x03;
        ForceBypass::from_bits(val as u8)
    }
    #[doc = "Force Bypass Control."]
    #[inline(always)]
    pub const fn set_force_bypass(&mut self, val: ForceBypass) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u16) & 0x03) << 12usize);
    }
}
impl Default for EvtgCtrl {
    #[inline(always)]
    fn default() -> EvtgCtrl {
        EvtgCtrl(0)
    }
}
impl core::fmt::Debug for EvtgCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EvtgCtrl")
            .field("ff_init", &self.ff_init())
            .field("init_en", &self.init_en())
            .field("mode_sel", &self.mode_sel())
            .field("fb_ovrd", &self.fb_ovrd())
            .field("sync_ctrl", &self.sync_ctrl())
            .field("force_bypass", &self.force_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EvtgCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EvtgCtrl {{ ff_init: {:?}, init_en: {:?}, mode_sel: {:?}, fb_ovrd: {:?}, sync_ctrl: {:?}, force_bypass: {:?} }}",
            self.ff_init(),
            self.init_en(),
            self.mode_sel(),
            self.fb_ovrd(),
            self.sync_ctrl(),
            self.force_bypass()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt0Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt0Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt0Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt0Ac3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Ac {
        EvtgAoi0Bft01Pt0Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Ac) -> u8 {
        EvtgAoi0Bft01Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt0Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt0Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt0Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt0Bc3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Bc {
        EvtgAoi0Bft01Pt0Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Bc) -> u8 {
        EvtgAoi0Bft01Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Cc00 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Cc01 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Cc02 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Cc03 = 0x03,
}
impl EvtgAoi0Bft01Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Cc {
        EvtgAoi0Bft01Pt0Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Cc) -> u8 {
        EvtgAoi0Bft01Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt0Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Dc3 = 0x03,
}
impl EvtgAoi0Bft01Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt0Dc {
        EvtgAoi0Bft01Pt0Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt0Dc) -> u8 {
        EvtgAoi0Bft01Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Ac3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Ac {
        EvtgAoi0Bft01Pt1Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Ac) -> u8 {
        EvtgAoi0Bft01Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Bc3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Bc {
        EvtgAoi0Bft01Pt1Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Bc) -> u8 {
        EvtgAoi0Bft01Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Cc3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Cc {
        EvtgAoi0Bft01Pt1Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Cc) -> u8 {
        EvtgAoi0Bft01Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft01Pt1Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt1Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt1Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt1Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt1Dc3 = 0x03,
}
impl EvtgAoi0Bft01Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft01Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft01Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft01Pt1Dc {
        EvtgAoi0Bft01Pt1Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft01Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft01Pt1Dc) -> u8 {
        EvtgAoi0Bft01Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt2Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt2Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt2Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt2Ac3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Ac {
        EvtgAoi0Bft23Pt2Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Ac) -> u8 {
        EvtgAoi0Bft23Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt2Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt2Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt2Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt2Bc3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Bc {
        EvtgAoi0Bft23Pt2Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Bc) -> u8 {
        EvtgAoi0Bft23Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt2Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt2Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt2Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt2Cc3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Cc {
        EvtgAoi0Bft23Pt2Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Cc) -> u8 {
        EvtgAoi0Bft23Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt2Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt2Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt2Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt2Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt2Dc3 = 0x03,
}
impl EvtgAoi0Bft23Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt2Dc {
        EvtgAoi0Bft23Pt2Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt2Dc) -> u8 {
        EvtgAoi0Bft23Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt3Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt3Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt3Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt3Ac3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Ac {
        EvtgAoi0Bft23Pt3Ac::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Ac) -> u8 {
        EvtgAoi0Bft23Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt3Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt3Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt3Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt3Bc3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Bc {
        EvtgAoi0Bft23Pt3Bc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Bc) -> u8 {
        EvtgAoi0Bft23Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt3Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt3Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt3Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt3Cc3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Cc {
        EvtgAoi0Bft23Pt3Cc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Cc) -> u8 {
        EvtgAoi0Bft23Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi0Bft23Pt3Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt3Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt3Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt3Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt3Dc3 = 0x03,
}
impl EvtgAoi0Bft23Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi0Bft23Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi0Bft23Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi0Bft23Pt3Dc {
        EvtgAoi0Bft23Pt3Dc::from_bits(val)
    }
}
impl From<EvtgAoi0Bft23Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi0Bft23Pt3Dc) -> u8 {
        EvtgAoi0Bft23Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt0Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt0Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt0Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt0Ac3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Ac {
        EvtgAoi1Bft01Pt0Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Ac) -> u8 {
        EvtgAoi1Bft01Pt0Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt0Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt0Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt0Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt0Bc3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Bc {
        EvtgAoi1Bft01Pt0Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Bc) -> u8 {
        EvtgAoi1Bft01Pt0Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt0Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt0Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt0Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt0Cc3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Cc {
        EvtgAoi1Bft01Pt0Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Cc) -> u8 {
        EvtgAoi1Bft01Pt0Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt0Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt0Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt0Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt0Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt0Dc3 = 0x03,
}
impl EvtgAoi1Bft01Pt0Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt0Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt0Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt0Dc {
        EvtgAoi1Bft01Pt0Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt0Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt0Dc) -> u8 {
        EvtgAoi1Bft01Pt0Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt1Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt1Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt1Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt1Ac3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Ac {
        EvtgAoi1Bft01Pt1Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Ac) -> u8 {
        EvtgAoi1Bft01Pt1Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt1Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt1Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt1Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt1Bc3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Bc {
        EvtgAoi1Bft01Pt1Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Bc) -> u8 {
        EvtgAoi1Bft01Pt1Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt1Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt1Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt1Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt1Cc3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Cc {
        EvtgAoi1Bft01Pt1Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Cc) -> u8 {
        EvtgAoi1Bft01Pt1Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft01Pt1Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt1Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt1Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt1Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt1Dc3 = 0x03,
}
impl EvtgAoi1Bft01Pt1Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft01Pt1Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft01Pt1Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft01Pt1Dc {
        EvtgAoi1Bft01Pt1Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft01Pt1Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft01Pt1Dc) -> u8 {
        EvtgAoi1Bft01Pt1Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt2Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt2Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt2Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt2Ac3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Ac {
        EvtgAoi1Bft23Pt2Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Ac) -> u8 {
        EvtgAoi1Bft23Pt2Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt2Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt2Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt2Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt2Bc3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Bc {
        EvtgAoi1Bft23Pt2Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Bc) -> u8 {
        EvtgAoi1Bft23Pt2Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt2Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt2Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt2Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt2Cc3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Cc {
        EvtgAoi1Bft23Pt2Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Cc) -> u8 {
        EvtgAoi1Bft23Pt2Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt2Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt2Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt2Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt2Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt2Dc3 = 0x03,
}
impl EvtgAoi1Bft23Pt2Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt2Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt2Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt2Dc {
        EvtgAoi1Bft23Pt2Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt2Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt2Dc) -> u8 {
        EvtgAoi1Bft23Pt2Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Ac {
    #[doc = "Force the A input in this product term to a logical zero."]
    Pt3Ac0 = 0x0,
    #[doc = "Pass the A input in this product term."]
    Pt3Ac1 = 0x01,
    #[doc = "Complement the A input in this product term."]
    Pt3Ac2 = 0x02,
    #[doc = "Force the A input in this product term to a logical one."]
    Pt3Ac3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Ac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Ac {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Ac {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Ac {
        EvtgAoi1Bft23Pt3Ac::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Ac> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Ac) -> u8 {
        EvtgAoi1Bft23Pt3Ac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Bc {
    #[doc = "Force the B input in this product term to a logical zero."]
    Pt3Bc0 = 0x0,
    #[doc = "Pass the B input in this product term."]
    Pt3Bc1 = 0x01,
    #[doc = "Complement the B input in this product term."]
    Pt3Bc2 = 0x02,
    #[doc = "Force the B input in this product term to a logical one."]
    Pt3Bc3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Bc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Bc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Bc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Bc {
        EvtgAoi1Bft23Pt3Bc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Bc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Bc) -> u8 {
        EvtgAoi1Bft23Pt3Bc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Cc {
    #[doc = "Force the C input in this product term to a logical zero."]
    Pt3Cc0 = 0x0,
    #[doc = "Pass the C input in this product term."]
    Pt3Cc1 = 0x01,
    #[doc = "Complement the C input in this product term."]
    Pt3Cc2 = 0x02,
    #[doc = "Force the C input in this product term to a logical one."]
    Pt3Cc3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Cc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Cc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Cc {
        EvtgAoi1Bft23Pt3Cc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Cc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Cc) -> u8 {
        EvtgAoi1Bft23Pt3Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EvtgAoi1Bft23Pt3Dc {
    #[doc = "Force the D input in this product term to a logical zero."]
    Pt3Dc0 = 0x0,
    #[doc = "Pass the D input in this product term."]
    Pt3Dc1 = 0x01,
    #[doc = "Complement the D input in this product term."]
    Pt3Dc2 = 0x02,
    #[doc = "Force the D input in this product term to a logical one."]
    Pt3Dc3 = 0x03,
}
impl EvtgAoi1Bft23Pt3Dc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EvtgAoi1Bft23Pt3Dc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EvtgAoi1Bft23Pt3Dc {
    #[inline(always)]
    fn from(val: u8) -> EvtgAoi1Bft23Pt3Dc {
        EvtgAoi1Bft23Pt3Dc::from_bits(val)
    }
}
impl From<EvtgAoi1Bft23Pt3Dc> for u8 {
    #[inline(always)]
    fn from(val: EvtgAoi1Bft23Pt3Dc) -> u8 {
        EvtgAoi1Bft23Pt3Dc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FbOvrd {
    #[doc = "Replace An."]
    An = 0x0,
    #[doc = "Replace Bn."]
    Bn = 0x01,
    #[doc = "Replace Cn."]
    Cn = 0x02,
    #[doc = "Replace Dn."]
    Dn = 0x03,
}
impl FbOvrd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FbOvrd {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FbOvrd {
    #[inline(always)]
    fn from(val: u8) -> FbOvrd {
        FbOvrd::from_bits(val)
    }
}
impl From<FbOvrd> for u8 {
    #[inline(always)]
    fn from(val: FbOvrd) -> u8 {
        FbOvrd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FfInit {
    #[doc = "0."]
    Ff0 = 0x0,
    #[doc = "1."]
    Ff1 = 0x01,
}
impl FfInit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FfInit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FfInit {
    #[inline(always)]
    fn from(val: u8) -> FfInit {
        FfInit::from_bits(val)
    }
}
impl From<FfInit> for u8 {
    #[inline(always)]
    fn from(val: FfInit) -> u8 {
        FfInit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceBypass {
    #[doc = "Will not force the bypass."]
    Nfb0 = 0x0,
    #[doc = "Whatever MODE_SEL is, will force bypass flip-flop and route the AOI_0(Filter_0) value directly to EVTG_OUTA."]
    FFfA = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl ForceBypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceBypass {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceBypass {
    #[inline(always)]
    fn from(val: u8) -> ForceBypass {
        ForceBypass::from_bits(val)
    }
}
impl From<ForceBypass> for u8 {
    #[inline(always)]
    fn from(val: ForceBypass) -> u8 {
        ForceBypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitEn {
    #[doc = "Write 0 does not generate enable pulse."]
    Pulse = 0x0,
    #[doc = "Write 1 generates enable pulse."]
    NoPulse = 0x01,
}
impl InitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitEn {
    #[inline(always)]
    fn from(val: u8) -> InitEn {
        InitEn::from_bits(val)
    }
}
impl From<InitEn> for u8 {
    #[inline(always)]
    fn from(val: InitEn) -> u8 {
        InitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ModeSel {
    #[doc = "Bypass mode."]
    Bypass = 0x0,
    #[doc = "RS Trigger mode."]
    Rs = 0x01,
    #[doc = "T-FF mode."]
    Tff = 0x02,
    #[doc = "D-FF mode."]
    Dff = 0x03,
    #[doc = "JK-FF mode."]
    Jkff = 0x04,
    #[doc = "Latch mode."]
    Latch = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl ModeSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ModeSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ModeSel {
    #[inline(always)]
    fn from(val: u8) -> ModeSel {
        ModeSel::from_bits(val)
    }
}
impl From<ModeSel> for u8 {
    #[inline(always)]
    fn from(val: ModeSel) -> u8 {
        ModeSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SyncCtrl {
    #[doc = "EVTG input \"An\" will not be synced."]
    ANotsync = 0x0,
    #[doc = "EVTG input \"An\" will be synced by two bus clk cycles."]
    ASync = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl SyncCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SyncCtrl {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SyncCtrl {
    #[inline(always)]
    fn from(val: u8) -> SyncCtrl {
        SyncCtrl::from_bits(val)
    }
}
impl From<SyncCtrl> for u8 {
    #[inline(always)]
    fn from(val: SyncCtrl) -> u8 {
        SyncCtrl::to_bits(val)
    }
}
