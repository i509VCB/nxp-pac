#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "FlashTest."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmu0test {
    ptr: *mut u8,
}
unsafe impl Send for Fmu0test {}
unsafe impl Sync for Fmu0test {}
impl Fmu0test {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Status Register."]
    #[inline(always)]
    pub const fn fstat(self) -> crate::pac::common::Reg<Fstat, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash Configuration Register."]
    #[inline(always)]
    pub const fn fcnfg(self) -> crate::pac::common::Reg<Fcnfg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash Control Register."]
    #[inline(always)]
    pub const fn fctrl(self) -> crate::pac::common::Reg<Fctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Test Register."]
    #[inline(always)]
    pub const fn ftest(self) -> crate::pac::common::Reg<Ftest, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Flash Command Control 0 Register."]
    #[inline(always)]
    pub const fn fccob0(self) -> crate::pac::common::Reg<Fccob0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Flash Command Control 1 Register."]
    #[inline(always)]
    pub const fn fccob1(self) -> crate::pac::common::Reg<Fccob1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Flash Command Control 2 Register."]
    #[inline(always)]
    pub const fn fccob2(self) -> crate::pac::common::Reg<Fccob2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Flash Command Control 3 Register."]
    #[inline(always)]
    pub const fn fccob3(self) -> crate::pac::common::Reg<Fccob3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Flash Command Control 4 Register."]
    #[inline(always)]
    pub const fn fccob4(self) -> crate::pac::common::Reg<Fccob4, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Flash Command Control 5 Register."]
    #[inline(always)]
    pub const fn fccob5(self) -> crate::pac::common::Reg<Fccob5, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Flash Command Control 6 Register."]
    #[inline(always)]
    pub const fn fccob6(self) -> crate::pac::common::Reg<Fccob6, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Flash Command Control 7 Register."]
    #[inline(always)]
    pub const fn fccob7(self) -> crate::pac::common::Reg<Fccob7, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "FMU Initialization Tracking Register."]
    #[inline(always)]
    pub const fn reset_status(
        self,
    ) -> crate::pac::common::Reg<ResetStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FMU Control Register."]
    #[inline(always)]
    pub const fn mctl(self) -> crate::pac::common::Reg<Mctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "FMU Block Select Generation Register."]
    #[inline(always)]
    pub const fn bsel_gen(self) -> crate::pac::common::Reg<BselGen, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Power Mode Options Register."]
    #[inline(always)]
    pub const fn pwr_opt(self) -> crate::pac::common::Reg<PwrOpt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "FMU Command Check Register."]
    #[inline(always)]
    pub const fn cmd_check(self) -> crate::pac::common::Reg<CmdCheck, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "FMU Block Select Register."]
    #[inline(always)]
    pub const fn bsel(self) -> crate::pac::common::Reg<Bsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FMU Memory Size Register."]
    #[inline(always)]
    pub const fn msize(self) -> crate::pac::common::Reg<Msize, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Flash Read Address Register."]
    #[inline(always)]
    pub const fn flash_rd_add(self) -> crate::pac::common::Reg<FlashRdAdd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Flash Stop Address Register."]
    #[inline(always)]
    pub const fn flash_stop_add(
        self,
    ) -> crate::pac::common::Reg<FlashStopAdd, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Flash Read Control Register."]
    #[inline(always)]
    pub const fn flash_rd_ctrl(
        self,
    ) -> crate::pac::common::Reg<FlashRdCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Memory Map Address Register."]
    #[inline(always)]
    pub const fn mm_addr(self) -> crate::pac::common::Reg<MmAddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Memory Map Write Data Register."]
    #[inline(always)]
    pub const fn mm_wdata(self) -> crate::pac::common::Reg<MmWdata, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Memory Map Control Register."]
    #[inline(always)]
    pub const fn mm_ctl(self) -> crate::pac::common::Reg<MmCtl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "User Interface Control Register."]
    #[inline(always)]
    pub const fn uint_ctl(self) -> crate::pac::common::Reg<UintCtl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Read Data 0 Register."]
    #[inline(always)]
    pub const fn rd_data0(self) -> crate::pac::common::Reg<RdData0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Read Data 1 Register."]
    #[inline(always)]
    pub const fn rd_data1(self) -> crate::pac::common::Reg<RdData1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Read Data 2 Register."]
    #[inline(always)]
    pub const fn rd_data2(self) -> crate::pac::common::Reg<RdData2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Read Data 3 Register."]
    #[inline(always)]
    pub const fn rd_data3(self) -> crate::pac::common::Reg<RdData3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Parity Register."]
    #[inline(always)]
    pub const fn parity(self) -> crate::pac::common::Reg<Parity, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Read Path Control and Status Register."]
    #[inline(always)]
    pub const fn rd_path_ctrl_status(
        self,
    ) -> crate::pac::common::Reg<RdPathCtrlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "SMW DIN 0 Register."]
    #[inline(always)]
    pub const fn smw_din0(self) -> crate::pac::common::Reg<SmwDin0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "SMW DIN 1 Register."]
    #[inline(always)]
    pub const fn smw_din1(self) -> crate::pac::common::Reg<SmwDin1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "SMW DIN 2 Register."]
    #[inline(always)]
    pub const fn smw_din2(self) -> crate::pac::common::Reg<SmwDin2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "SMW DIN 3 Register."]
    #[inline(always)]
    pub const fn smw_din3(self) -> crate::pac::common::Reg<SmwDin3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "SMW Address Register."]
    #[inline(always)]
    pub const fn smw_addr(self) -> crate::pac::common::Reg<SmwAddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "SMW Command and Wait Register."]
    #[inline(always)]
    pub const fn smw_cmd_wait(self) -> crate::pac::common::Reg<SmwCmdWait, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "SMW Status Register."]
    #[inline(always)]
    pub const fn smw_status(self) -> crate::pac::common::Reg<SmwStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim0_0(self) -> crate::pac::common::Reg<Soctrim00, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim0_1(self) -> crate::pac::common::Reg<Soctrim01, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim0_2(self) -> crate::pac::common::Reg<Soctrim02, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim0_3(self) -> crate::pac::common::Reg<Soctrim03, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim1_0(self) -> crate::pac::common::Reg<Soctrim10, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim1_1(self) -> crate::pac::common::Reg<Soctrim11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim1_2(self) -> crate::pac::common::Reg<Soctrim12, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim1_3(self) -> crate::pac::common::Reg<Soctrim13, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim2_0(self) -> crate::pac::common::Reg<Soctrim20, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim2_1(self) -> crate::pac::common::Reg<Soctrim21, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim2_2(self) -> crate::pac::common::Reg<Soctrim22, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim2_3(self) -> crate::pac::common::Reg<Soctrim23, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim3_0(self) -> crate::pac::common::Reg<Soctrim30, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim3_1(self) -> crate::pac::common::Reg<Soctrim31, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim3_2(self) -> crate::pac::common::Reg<Soctrim32, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim3_3(self) -> crate::pac::common::Reg<Soctrim33, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim4_0(self) -> crate::pac::common::Reg<Soctrim40, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim4_1(self) -> crate::pac::common::Reg<Soctrim41, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim4_2(self) -> crate::pac::common::Reg<Soctrim42, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim4_3(self) -> crate::pac::common::Reg<Soctrim43, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim5_0(self) -> crate::pac::common::Reg<Soctrim50, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim5_1(self) -> crate::pac::common::Reg<Soctrim51, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim5_2(self) -> crate::pac::common::Reg<Soctrim52, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim5_3(self) -> crate::pac::common::Reg<Soctrim53, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim6_0(self) -> crate::pac::common::Reg<Soctrim60, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim6_1(self) -> crate::pac::common::Reg<Soctrim61, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim6_2(self) -> crate::pac::common::Reg<Soctrim62, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim6_3(self) -> crate::pac::common::Reg<Soctrim63, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 0 Register."]
    #[inline(always)]
    pub const fn soctrim7_0(self) -> crate::pac::common::Reg<Soctrim70, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 1 Register."]
    #[inline(always)]
    pub const fn soctrim7_1(self) -> crate::pac::common::Reg<Soctrim71, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 2 Register."]
    #[inline(always)]
    pub const fn soctrim7_2(self) -> crate::pac::common::Reg<Soctrim72, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 3 Register."]
    #[inline(always)]
    pub const fn soctrim7_3(self) -> crate::pac::common::Reg<Soctrim73, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "BIST Configuration Register."]
    #[inline(always)]
    pub const fn r_ip_config(self) -> crate::pac::common::Reg<RIpConfig, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "BIST Test Code Register."]
    #[inline(always)]
    pub const fn r_testcode(self) -> crate::pac::common::Reg<RTestcode, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "BIST DFT Control Register."]
    #[inline(always)]
    pub const fn r_dft_ctrl(self) -> crate::pac::common::Reg<RDftCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "BIST Address Control Register."]
    #[inline(always)]
    pub const fn r_adr_ctrl(self) -> crate::pac::common::Reg<RAdrCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "BIST Data Control 0 Register."]
    #[inline(always)]
    pub const fn r_data_ctrl0(self) -> crate::pac::common::Reg<RDataCtrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "BIST Pin Control Register."]
    #[inline(always)]
    pub const fn r_pin_ctrl(self) -> crate::pac::common::Reg<RPinCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "BIST Loop Count Control Register."]
    #[inline(always)]
    pub const fn r_cnt_loop_ctrl(
        self,
    ) -> crate::pac::common::Reg<RCntLoopCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "BIST Timer Control Register."]
    #[inline(always)]
    pub const fn r_timer_ctrl(self) -> crate::pac::common::Reg<RTimerCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "BIST Test Control Register."]
    #[inline(always)]
    pub const fn r_test_ctrl(self) -> crate::pac::common::Reg<RTestCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "BIST Abort Loop Register."]
    #[inline(always)]
    pub const fn r_abort_loop(self) -> crate::pac::common::Reg<RAbortLoop, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "BIST Address Query Register."]
    #[inline(always)]
    pub const fn r_adr_query(self) -> crate::pac::common::Reg<RAdrQuery, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "BIST DOUT Query 0 Register."]
    #[inline(always)]
    pub const fn r_dout_query0(
        self,
    ) -> crate::pac::common::Reg<RDoutQuery0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "BIST SMW Query Register."]
    #[inline(always)]
    pub const fn r_smw_query(self) -> crate::pac::common::Reg<RSmwQuery, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "BIST SMW Setting 0 Register."]
    #[inline(always)]
    pub const fn r_smw_setting0(
        self,
    ) -> crate::pac::common::Reg<RSmwSetting0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "BIST SMW Setting 1 Register."]
    #[inline(always)]
    pub const fn r_smw_setting1(
        self,
    ) -> crate::pac::common::Reg<RSmwSetting1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "BIST SMP WHV Setting 0 Register."]
    #[inline(always)]
    pub const fn r_smp_whv0(self) -> crate::pac::common::Reg<RSmpWhv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "BIST SMP WHV Setting 1 Register."]
    #[inline(always)]
    pub const fn r_smp_whv1(self) -> crate::pac::common::Reg<RSmpWhv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "BIST SME WHV Setting 0 Register."]
    #[inline(always)]
    pub const fn r_sme_whv0(self) -> crate::pac::common::Reg<RSmeWhv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "BIST SME WHV Setting 1 Register."]
    #[inline(always)]
    pub const fn r_sme_whv1(self) -> crate::pac::common::Reg<RSmeWhv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "BIST SMW Setting 2 Register."]
    #[inline(always)]
    pub const fn r_smw_setting2(
        self,
    ) -> crate::pac::common::Reg<RSmwSetting2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "BIST DIN MISR 0 Register."]
    #[inline(always)]
    pub const fn r_d_misr0(self) -> crate::pac::common::Reg<RDMisr0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "BIST Address MISR 0 Register."]
    #[inline(always)]
    pub const fn r_a_misr0(self) -> crate::pac::common::Reg<RAMisr0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "BIST Control MISR 0 Register."]
    #[inline(always)]
    pub const fn r_c_misr0(self) -> crate::pac::common::Reg<RCMisr0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "BIST SMW Setting 3 Register."]
    #[inline(always)]
    pub const fn r_smw_setting3(
        self,
    ) -> crate::pac::common::Reg<RSmwSetting3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "BIST Data Control 1 Register."]
    #[inline(always)]
    pub const fn r_data_ctrl1(self) -> crate::pac::common::Reg<RDataCtrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "BIST Data Control 2 Register."]
    #[inline(always)]
    pub const fn r_data_ctrl2(self) -> crate::pac::common::Reg<RDataCtrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "BIST Data Control 3 Register."]
    #[inline(always)]
    pub const fn r_data_ctrl3(self) -> crate::pac::common::Reg<RDataCtrl3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "BIST Repair 0 for Block 0 Register."]
    #[inline(always)]
    pub const fn r_repair0_0(self) -> crate::pac::common::Reg<RRepair00, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "BIST Repair 1 Block 0 Register."]
    #[inline(always)]
    pub const fn r_repair0_1(self) -> crate::pac::common::Reg<RRepair01, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "BIST Repair 0 Block 1 Register."]
    #[inline(always)]
    pub const fn r_repair1_0(self) -> crate::pac::common::Reg<RRepair10, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "BIST Repair 1 Block 1 Register."]
    #[inline(always)]
    pub const fn r_repair1_1(self) -> crate::pac::common::Reg<RRepair11, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "BIST Data Control 0 Extension Register."]
    #[inline(always)]
    pub const fn r_data_ctrl0_ex(
        self,
    ) -> crate::pac::common::Reg<RDataCtrl0Ex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "BIST Timer Control Extension Register."]
    #[inline(always)]
    pub const fn r_timer_ctrl_ex(
        self,
    ) -> crate::pac::common::Reg<RTimerCtrlEx, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "BIST DOUT Query 1 Register."]
    #[inline(always)]
    pub const fn r_dout_query1(
        self,
    ) -> crate::pac::common::Reg<RDoutQuery1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "BIST DIN MISR 1 Register."]
    #[inline(always)]
    pub const fn r_d_misr1(self) -> crate::pac::common::Reg<RDMisr1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "BIST Address MISR 1 Register."]
    #[inline(always)]
    pub const fn r_a_misr1(self) -> crate::pac::common::Reg<RAMisr1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "BIST Control MISR 1 Register."]
    #[inline(always)]
    pub const fn r_c_misr1(self) -> crate::pac::common::Reg<RCMisr1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "BIST Data Control 1 Extension Register."]
    #[inline(always)]
    pub const fn r_data_ctrl1_ex(
        self,
    ) -> crate::pac::common::Reg<RDataCtrl1Ex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "BIST Data Control 2 Extension Register."]
    #[inline(always)]
    pub const fn r_data_ctrl2_ex(
        self,
    ) -> crate::pac::common::Reg<RDataCtrl2Ex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "BIST Data Control 3 Extension Register."]
    #[inline(always)]
    pub const fn r_data_ctrl3_ex(
        self,
    ) -> crate::pac::common::Reg<RDataCtrl3Ex, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "SMW Timer Option Register."]
    #[inline(always)]
    pub const fn smw_timer_option(
        self,
    ) -> crate::pac::common::Reg<SmwTimerOption, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SMW Setting Option 0 Register."]
    #[inline(always)]
    pub const fn smw_setting_option0(
        self,
    ) -> crate::pac::common::Reg<SmwSettingOption0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SMW Setting Option 2 Register."]
    #[inline(always)]
    pub const fn smw_setting_option2(
        self,
    ) -> crate::pac::common::Reg<SmwSettingOption2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "SMW Setting Option 3 Register."]
    #[inline(always)]
    pub const fn smw_setting_option3(
        self,
    ) -> crate::pac::common::Reg<SmwSettingOption3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "SMW SMP WHV Option 0 Register."]
    #[inline(always)]
    pub const fn smw_smp_whv_option0(
        self,
    ) -> crate::pac::common::Reg<SmwSmpWhvOption0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "SMW SME WHV Option 0 Register."]
    #[inline(always)]
    pub const fn smw_sme_whv_option0(
        self,
    ) -> crate::pac::common::Reg<SmwSmeWhvOption0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "SMW Setting Option 1 Register."]
    #[inline(always)]
    pub const fn smw_setting_option1(
        self,
    ) -> crate::pac::common::Reg<SmwSettingOption1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "SMW SMP WHV Option 1 Register."]
    #[inline(always)]
    pub const fn smw_smp_whv_option1(
        self,
    ) -> crate::pac::common::Reg<SmwSmpWhvOption1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "SMW SME WHV Option 1 Register."]
    #[inline(always)]
    pub const fn smw_sme_whv_option1(
        self,
    ) -> crate::pac::common::Reg<SmwSmeWhvOption1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "FMU Repair 0 Block 0 Register."]
    #[inline(always)]
    pub const fn repair0_0(self) -> crate::pac::common::Reg<Repair00, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "FMU Repair 1 Block 0 Register."]
    #[inline(always)]
    pub const fn repair0_1(self) -> crate::pac::common::Reg<Repair01, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "FMU Repair 0 Block 1 Register."]
    #[inline(always)]
    pub const fn repair1_0(self) -> crate::pac::common::Reg<Repair10, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "FMU Repair 1 Block 1 Register."]
    #[inline(always)]
    pub const fn repair1_1(self) -> crate::pac::common::Reg<Repair11, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "SMW HB Signals Register."]
    #[inline(always)]
    pub const fn smw_hb_signals(
        self,
    ) -> crate::pac::common::Reg<SmwHbSignals, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "BIST Datadump Control Register."]
    #[inline(always)]
    pub const fn bist_dump_ctrl(
        self,
    ) -> crate::pac::common::Reg<BistDumpCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "ATX Pin Control Register."]
    #[inline(always)]
    pub const fn atx_pin_ctrl(self) -> crate::pac::common::Reg<AtxPinCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "Fail Count Register."]
    #[inline(always)]
    pub const fn failcnt(self) -> crate::pac::common::Reg<Failcnt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Block 0 Program Pulse Count Register."]
    #[inline(always)]
    pub const fn pgm_pulse_cnt0(
        self,
    ) -> crate::pac::common::Reg<PgmPulseCnt0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "Block 1 Program Pulse Count Register."]
    #[inline(always)]
    pub const fn pgm_pulse_cnt1(
        self,
    ) -> crate::pac::common::Reg<PgmPulseCnt1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Erase Pulse Count Register."]
    #[inline(always)]
    pub const fn ers_pulse_cnt(
        self,
    ) -> crate::pac::common::Reg<ErsPulseCnt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Maximum Pulse Count Register."]
    #[inline(always)]
    pub const fn max_pulse_cnt(
        self,
    ) -> crate::pac::common::Reg<MaxPulseCnt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Port Control Register."]
    #[inline(always)]
    pub const fn port_ctrl(self) -> crate::pac::common::Reg<PortCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
}
#[doc = "ATX Pin Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AtxPinCtrl(pub u32);
impl AtxPinCtrl {
    #[doc = "TM to ATX."]
    #[must_use]
    #[inline(always)]
    pub const fn tm_to_atx(&self) -> TmToAtx {
        let val = (self.0 >> 0usize) & 0xff;
        TmToAtx::from_bits(val as u8)
    }
    #[doc = "TM to ATX."]
    #[inline(always)]
    pub const fn set_tm_to_atx(&mut self, val: TmToAtx) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for AtxPinCtrl {
    #[inline(always)]
    fn default() -> AtxPinCtrl {
        AtxPinCtrl(0)
    }
}
impl core::fmt::Debug for AtxPinCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AtxPinCtrl")
            .field("tm_to_atx", &self.tm_to_atx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AtxPinCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtxPinCtrl {{ tm_to_atx: {:?} }}", self.tm_to_atx())
    }
}
#[doc = "BIST Datadump Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BistDumpCtrl(pub u32);
impl BistDumpCtrl {
    #[doc = "BIST Done."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_done(&self) -> BistDone {
        let val = (self.0 >> 16usize) & 0x01;
        BistDone::from_bits(val as u8)
    }
    #[doc = "BIST Done."]
    #[inline(always)]
    pub const fn set_bist_done(&mut self, val: BistDone) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "BIST Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_fail(&self) -> BistFail {
        let val = (self.0 >> 17usize) & 0x01;
        BistFail::from_bits(val as u8)
    }
    #[doc = "BIST Fail."]
    #[inline(always)]
    pub const fn set_bist_fail(&mut self, val: BistFail) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Data Dump Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datadump(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Data Dump Enable."]
    #[inline(always)]
    pub const fn set_datadump(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Data Dump Trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_trig(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Data Dump Trigger."]
    #[inline(always)]
    pub const fn set_datadump_trig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Dump Pattern Select."]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_patt(&self) -> DatadumpPatt {
        let val = (self.0 >> 20usize) & 0x03;
        DatadumpPatt::from_bits(val as u8)
    }
    #[doc = "Data Dump Pattern Select."]
    #[inline(always)]
    pub const fn set_datadump_patt(&mut self, val: DatadumpPatt) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Data Dump Margin Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_mrgen(&self) -> DatadumpMrgen {
        let val = (self.0 >> 22usize) & 0x01;
        DatadumpMrgen::from_bits(val as u8)
    }
    #[doc = "Data Dump Margin Enable."]
    #[inline(always)]
    pub const fn set_datadump_mrgen(&mut self, val: DatadumpMrgen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Data Dump Margin Type."]
    #[must_use]
    #[inline(always)]
    pub const fn datadump_mrgtype(&self) -> DatadumpMrgtype {
        let val = (self.0 >> 23usize) & 0x01;
        DatadumpMrgtype::from_bits(val as u8)
    }
    #[doc = "Data Dump Margin Type."]
    #[inline(always)]
    pub const fn set_datadump_mrgtype(&mut self, val: DatadumpMrgtype) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for BistDumpCtrl {
    #[inline(always)]
    fn default() -> BistDumpCtrl {
        BistDumpCtrl(0)
    }
}
impl core::fmt::Debug for BistDumpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BistDumpCtrl")
            .field("bist_done", &self.bist_done())
            .field("bist_fail", &self.bist_fail())
            .field("datadump", &self.datadump())
            .field("datadump_trig", &self.datadump_trig())
            .field("datadump_patt", &self.datadump_patt())
            .field("datadump_mrgen", &self.datadump_mrgen())
            .field("datadump_mrgtype", &self.datadump_mrgtype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BistDumpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BistDumpCtrl {{ bist_done: {:?}, bist_fail: {:?}, datadump: {=bool:?}, datadump_trig: {=bool:?}, datadump_patt: {:?}, datadump_mrgen: {:?}, datadump_mrgtype: {:?} }}",
            self.bist_done(),
            self.bist_fail(),
            self.datadump(),
            self.datadump_trig(),
            self.datadump_patt(),
            self.datadump_mrgen(),
            self.datadump_mrgtype()
        )
    }
}
#[doc = "FMU Block Select Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bsel(pub u32);
impl Bsel {
    #[doc = "Slave Block Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sbsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Slave Block Select."]
    #[inline(always)]
    pub const fn set_sbsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Master Block Select."]
    #[must_use]
    #[inline(always)]
    pub const fn mbsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Master Block Select."]
    #[inline(always)]
    pub const fn set_mbsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Bsel {
    #[inline(always)]
    fn default() -> Bsel {
        Bsel(0)
    }
}
impl core::fmt::Debug for Bsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bsel")
            .field("sbsel", &self.sbsel())
            .field("mbsel", &self.mbsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Bsel {{ sbsel: {=u8:?}, mbsel: {=u8:?} }}",
            self.sbsel(),
            self.mbsel()
        )
    }
}
#[doc = "FMU Block Select Generation Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BselGen(pub u32);
impl BselGen {
    #[doc = "Generated SBSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn sbsel_gen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Generated SBSEL."]
    #[inline(always)]
    pub const fn set_sbsel_gen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Generated MBSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn mbsel_gen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Generated MBSEL."]
    #[inline(always)]
    pub const fn set_mbsel_gen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for BselGen {
    #[inline(always)]
    fn default() -> BselGen {
        BselGen(0)
    }
}
impl core::fmt::Debug for BselGen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BselGen")
            .field("sbsel_gen", &self.sbsel_gen())
            .field("mbsel_gen", &self.mbsel_gen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BselGen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BselGen {{ sbsel_gen: {=u8:?}, mbsel_gen: {=u8:?} }}",
            self.sbsel_gen(),
            self.mbsel_gen()
        )
    }
}
#[doc = "FMU Command Check Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdCheck(pub u32);
impl CmdCheck {
    #[doc = "Phrase Alignment Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_phr(&self) -> AlignfailPhr {
        let val = (self.0 >> 0usize) & 0x01;
        AlignfailPhr::from_bits(val as u8)
    }
    #[doc = "Phrase Alignment Fail."]
    #[inline(always)]
    pub const fn set_alignfail_phr(&mut self, val: AlignfailPhr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Page Alignment Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_pg(&self) -> AlignfailPg {
        let val = (self.0 >> 1usize) & 0x01;
        AlignfailPg::from_bits(val as u8)
    }
    #[doc = "Page Alignment Fail."]
    #[inline(always)]
    pub const fn set_alignfail_pg(&mut self, val: AlignfailPg) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Sector Alignment Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_scr(&self) -> AlignfailScr {
        let val = (self.0 >> 2usize) & 0x01;
        AlignfailScr::from_bits(val as u8)
    }
    #[doc = "Sector Alignment Fail."]
    #[inline(always)]
    pub const fn set_alignfail_scr(&mut self, val: AlignfailScr) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Block Alignment Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn alignfail_blk(&self) -> AlignfailBlk {
        let val = (self.0 >> 3usize) & 0x01;
        AlignfailBlk::from_bits(val as u8)
    }
    #[doc = "Block Alignment Fail."]
    #[inline(always)]
    pub const fn set_alignfail_blk(&mut self, val: AlignfailBlk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Address Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn addr_fail(&self) -> AddrFail {
        let val = (self.0 >> 4usize) & 0x01;
        AddrFail::from_bits(val as u8)
    }
    #[doc = "Address Fail."]
    #[inline(always)]
    pub const fn set_addr_fail(&mut self, val: AddrFail) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IFR Command."]
    #[must_use]
    #[inline(always)]
    pub const fn ifr_cmd(&self) -> IfrCmd {
        let val = (self.0 >> 5usize) & 0x01;
        IfrCmd::from_bits(val as u8)
    }
    #[doc = "IFR Command."]
    #[inline(always)]
    pub const fn set_ifr_cmd(&mut self, val: IfrCmd) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "All Blocks Command."]
    #[must_use]
    #[inline(always)]
    pub const fn all_cmd(&self) -> AllCmd {
        let val = (self.0 >> 6usize) & 0x01;
        AllCmd::from_bits(val as u8)
    }
    #[doc = "All Blocks Command."]
    #[inline(always)]
    pub const fn set_all_cmd(&mut self, val: AllCmd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Address Range Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn range_fail(&self) -> RangeFail {
        let val = (self.0 >> 7usize) & 0x01;
        RangeFail::from_bits(val as u8)
    }
    #[doc = "Address Range Fail."]
    #[inline(always)]
    pub const fn set_range_fail(&mut self, val: RangeFail) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Sector Alignment Check."]
    #[must_use]
    #[inline(always)]
    pub const fn scr_align_chk(&self) -> ScrAlignChk {
        let val = (self.0 >> 8usize) & 0x01;
        ScrAlignChk::from_bits(val as u8)
    }
    #[doc = "Sector Alignment Check."]
    #[inline(always)]
    pub const fn set_scr_align_chk(&mut self, val: ScrAlignChk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Option Check Fail."]
    #[must_use]
    #[inline(always)]
    pub const fn option_fail(&self) -> OptionFail {
        let val = (self.0 >> 9usize) & 0x01;
        OptionFail::from_bits(val as u8)
    }
    #[doc = "Option Check Fail."]
    #[inline(always)]
    pub const fn set_option_fail(&mut self, val: OptionFail) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Illegal Command."]
    #[must_use]
    #[inline(always)]
    pub const fn illegal_cmd(&self) -> IllegalCmd {
        let val = (self.0 >> 10usize) & 0x01;
        IllegalCmd::from_bits(val as u8)
    }
    #[doc = "Illegal Command."]
    #[inline(always)]
    pub const fn set_illegal_cmd(&mut self, val: IllegalCmd) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for CmdCheck {
    #[inline(always)]
    fn default() -> CmdCheck {
        CmdCheck(0)
    }
}
impl core::fmt::Debug for CmdCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdCheck")
            .field("alignfail_phr", &self.alignfail_phr())
            .field("alignfail_pg", &self.alignfail_pg())
            .field("alignfail_scr", &self.alignfail_scr())
            .field("alignfail_blk", &self.alignfail_blk())
            .field("addr_fail", &self.addr_fail())
            .field("ifr_cmd", &self.ifr_cmd())
            .field("all_cmd", &self.all_cmd())
            .field("range_fail", &self.range_fail())
            .field("scr_align_chk", &self.scr_align_chk())
            .field("option_fail", &self.option_fail())
            .field("illegal_cmd", &self.illegal_cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdCheck {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmdCheck {{ alignfail_phr: {:?}, alignfail_pg: {:?}, alignfail_scr: {:?}, alignfail_blk: {:?}, addr_fail: {:?}, ifr_cmd: {:?}, all_cmd: {:?}, range_fail: {:?}, scr_align_chk: {:?}, option_fail: {:?}, illegal_cmd: {:?} }}",
            self.alignfail_phr(),
            self.alignfail_pg(),
            self.alignfail_scr(),
            self.alignfail_blk(),
            self.addr_fail(),
            self.ifr_cmd(),
            self.all_cmd(),
            self.range_fail(),
            self.scr_align_chk(),
            self.option_fail(),
            self.illegal_cmd()
        )
    }
}
#[doc = "Erase Pulse Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErsPulseCnt(pub u32);
impl ErsPulseCnt {
    #[doc = "Block 0 Erase Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn ers_cnt0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Block 0 Erase Pulse Count."]
    #[inline(always)]
    pub const fn set_ers_cnt0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Block 1 Erase Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn ers_cnt1(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Block 1 Erase Pulse Count."]
    #[inline(always)]
    pub const fn set_ers_cnt1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ErsPulseCnt {
    #[inline(always)]
    fn default() -> ErsPulseCnt {
        ErsPulseCnt(0)
    }
}
impl core::fmt::Debug for ErsPulseCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ErsPulseCnt")
            .field("ers_cnt0", &self.ers_cnt0())
            .field("ers_cnt1", &self.ers_cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ErsPulseCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ErsPulseCnt {{ ers_cnt0: {=u16:?}, ers_cnt1: {=u16:?} }}",
            self.ers_cnt0(),
            self.ers_cnt1()
        )
    }
}
#[doc = "Fail Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Failcnt(pub u32);
impl Failcnt {
    #[doc = "Fail Count."]
    #[must_use]
    #[inline(always)]
    pub const fn failcnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Fail Count."]
    #[inline(always)]
    pub const fn set_failcnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Failcnt {
    #[inline(always)]
    fn default() -> Failcnt {
        Failcnt(0)
    }
}
impl core::fmt::Debug for Failcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Failcnt")
            .field("failcnt", &self.failcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Failcnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Failcnt {{ failcnt: {=u32:?} }}", self.failcnt())
    }
}
#[doc = "Flash Command Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob0(pub u32);
impl Fccob0 {
    #[doc = "Command code."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command code."]
    #[inline(always)]
    pub const fn set_cmdcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fccob0 {
    #[inline(always)]
    fn default() -> Fccob0 {
        Fccob0(0)
    }
}
impl core::fmt::Debug for Fccob0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob0")
            .field("cmdcode", &self.cmdcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob0 {{ cmdcode: {=u8:?} }}", self.cmdcode())
    }
}
#[doc = "Flash Command Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob1(pub u32);
impl Fccob1 {
    #[doc = "Command options."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdopt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Command options."]
    #[inline(always)]
    pub const fn set_cmdopt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fccob1 {
    #[inline(always)]
    fn default() -> Fccob1 {
        Fccob1(0)
    }
}
impl core::fmt::Debug for Fccob1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob1")
            .field("cmdopt", &self.cmdopt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob1 {{ cmdopt: {=u8:?} }}", self.cmdopt())
    }
}
#[doc = "Flash Command Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob2(pub u32);
impl Fccob2 {
    #[doc = "Command starting address."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command starting address."]
    #[inline(always)]
    pub const fn set_cmdaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob2 {
    #[inline(always)]
    fn default() -> Fccob2 {
        Fccob2(0)
    }
}
impl core::fmt::Debug for Fccob2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob2")
            .field("cmdaddr", &self.cmdaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob2 {{ cmdaddr: {=u32:?} }}", self.cmdaddr())
    }
}
#[doc = "Flash Command Control 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob3(pub u32);
impl Fccob3 {
    #[doc = "Command ending address."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdaddre(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command ending address."]
    #[inline(always)]
    pub const fn set_cmdaddre(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob3 {
    #[inline(always)]
    fn default() -> Fccob3 {
        Fccob3(0)
    }
}
impl core::fmt::Debug for Fccob3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob3")
            .field("cmdaddre", &self.cmdaddre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob3 {{ cmdaddre: {=u32:?} }}", self.cmdaddre())
    }
}
#[doc = "Flash Command Control 4 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob4(pub u32);
impl Fccob4 {
    #[doc = "Command data word 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 0."]
    #[inline(always)]
    pub const fn set_cmddata0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob4 {
    #[inline(always)]
    fn default() -> Fccob4 {
        Fccob4(0)
    }
}
impl core::fmt::Debug for Fccob4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob4")
            .field("cmddata0", &self.cmddata0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob4 {{ cmddata0: {=u32:?} }}", self.cmddata0())
    }
}
#[doc = "Flash Command Control 5 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob5(pub u32);
impl Fccob5 {
    #[doc = "Command data word 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 1."]
    #[inline(always)]
    pub const fn set_cmddata1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob5 {
    #[inline(always)]
    fn default() -> Fccob5 {
        Fccob5(0)
    }
}
impl core::fmt::Debug for Fccob5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob5")
            .field("cmddata1", &self.cmddata1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob5 {{ cmddata1: {=u32:?} }}", self.cmddata1())
    }
}
#[doc = "Flash Command Control 6 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob6(pub u32);
impl Fccob6 {
    #[doc = "Command data word 2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 2."]
    #[inline(always)]
    pub const fn set_cmddata2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob6 {
    #[inline(always)]
    fn default() -> Fccob6 {
        Fccob6(0)
    }
}
impl core::fmt::Debug for Fccob6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob6")
            .field("cmddata2", &self.cmddata2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob6 {{ cmddata2: {=u32:?} }}", self.cmddata2())
    }
}
#[doc = "Flash Command Control 7 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fccob7(pub u32);
impl Fccob7 {
    #[doc = "Command data word 3."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddata3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command data word 3."]
    #[inline(always)]
    pub const fn set_cmddata3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fccob7 {
    #[inline(always)]
    fn default() -> Fccob7 {
        Fccob7(0)
    }
}
impl core::fmt::Debug for Fccob7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fccob7")
            .field("cmddata3", &self.cmddata3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fccob7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fccob7 {{ cmddata3: {=u32:?} }}", self.cmddata3())
    }
}
#[doc = "Flash Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcnfg(pub u32);
impl Fcnfg {
    #[doc = "Command Complete Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccie(&self) -> Ccie {
        let val = (self.0 >> 7usize) & 0x01;
        Ccie::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Enable."]
    #[inline(always)]
    pub const fn set_ccie(&mut self, val: Ccie) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Mass Erase (Erase All) Request."]
    #[must_use]
    #[inline(always)]
    pub const fn ersreq(&self) -> Ersreq {
        let val = (self.0 >> 8usize) & 0x01;
        Ersreq::from_bits(val as u8)
    }
    #[doc = "Mass Erase (Erase All) Request."]
    #[inline(always)]
    pub const fn set_ersreq(&mut self, val: Ersreq) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dfdie(&self) -> Dfdie {
        let val = (self.0 >> 16usize) & 0x01;
        Dfdie::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Enable."]
    #[inline(always)]
    pub const fn set_dfdie(&mut self, val: Dfdie) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn ersien0(&self) -> Ersien0 {
        let val = (self.0 >> 24usize) & 0x0f;
        Ersien0::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 0."]
    #[inline(always)]
    pub const fn set_ersien0(&mut self, val: Ersien0) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)."]
    #[must_use]
    #[inline(always)]
    pub const fn ersien1(&self) -> Ersien1 {
        let val = (self.0 >> 28usize) & 0x0f;
        Ersien1::from_bits(val as u8)
    }
    #[doc = "Erase IFR Sector Enable - Block 1 (for dual block configs)."]
    #[inline(always)]
    pub const fn set_ersien1(&mut self, val: Ersien1) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Fcnfg {
    #[inline(always)]
    fn default() -> Fcnfg {
        Fcnfg(0)
    }
}
impl core::fmt::Debug for Fcnfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcnfg")
            .field("ccie", &self.ccie())
            .field("ersreq", &self.ersreq())
            .field("dfdie", &self.dfdie())
            .field("ersien0", &self.ersien0())
            .field("ersien1", &self.ersien1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcnfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fcnfg {{ ccie: {:?}, ersreq: {:?}, dfdie: {:?}, ersien0: {:?}, ersien1: {:?} }}",
            self.ccie(),
            self.ersreq(),
            self.dfdie(),
            self.ersien0(),
            self.ersien1()
        )
    }
}
#[doc = "Flash Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl(pub u32);
impl Fctrl {
    #[doc = "Read Wait-State Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rwsc(&self) -> Rwsc {
        let val = (self.0 >> 0usize) & 0x0f;
        Rwsc::from_bits(val as u8)
    }
    #[doc = "Read Wait-State Control."]
    #[inline(always)]
    pub const fn set_rwsc(&mut self, val: Rwsc) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Low Speed Active Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn lsactive(&self) -> Lsactive {
        let val = (self.0 >> 8usize) & 0x01;
        Lsactive::from_bits(val as u8)
    }
    #[doc = "Low Speed Active Mode."]
    #[inline(always)]
    pub const fn set_lsactive(&mut self, val: Lsactive) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Force Double Bit Fault Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn fdfd(&self) -> Fdfd {
        let val = (self.0 >> 16usize) & 0x01;
        Fdfd::from_bits(val as u8)
    }
    #[doc = "Force Double Bit Fault Detect."]
    #[inline(always)]
    pub const fn set_fdfd(&mut self, val: Fdfd) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Abort Request."]
    #[must_use]
    #[inline(always)]
    pub const fn abtreq(&self) -> Abtreq {
        let val = (self.0 >> 24usize) & 0x01;
        Abtreq::from_bits(val as u8)
    }
    #[doc = "Abort Request."]
    #[inline(always)]
    pub const fn set_abtreq(&mut self, val: Abtreq) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        Fctrl(0)
    }
}
impl core::fmt::Debug for Fctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fctrl")
            .field("rwsc", &self.rwsc())
            .field("lsactive", &self.lsactive())
            .field("fdfd", &self.fdfd())
            .field("abtreq", &self.abtreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fctrl {{ rwsc: {:?}, lsactive: {:?}, fdfd: {:?}, abtreq: {:?} }}",
            self.rwsc(),
            self.lsactive(),
            self.fdfd(),
            self.abtreq()
        )
    }
}
#[doc = "Flash Read Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashRdAdd(pub u32);
impl FlashRdAdd {
    #[doc = "Flash Read Address."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rd_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Flash Read Address."]
    #[inline(always)]
    pub const fn set_flash_rd_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlashRdAdd {
    #[inline(always)]
    fn default() -> FlashRdAdd {
        FlashRdAdd(0)
    }
}
impl core::fmt::Debug for FlashRdAdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashRdAdd")
            .field("flash_rd_add", &self.flash_rd_add())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashRdAdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashRdAdd {{ flash_rd_add: {=u32:?} }}",
            self.flash_rd_add()
        )
    }
}
#[doc = "Flash Read Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashRdCtrl(pub u32);
impl FlashRdCtrl {
    #[doc = "Flash Read Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_rd(&self) -> FlashRd {
        let val = (self.0 >> 0usize) & 0x01;
        FlashRd::from_bits(val as u8)
    }
    #[doc = "Flash Read Enable."]
    #[inline(always)]
    pub const fn set_flash_rd(&mut self, val: FlashRd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Wide Load Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wide_load(&self) -> WideLoad {
        let val = (self.0 >> 1usize) & 0x01;
        WideLoad::from_bits(val as u8)
    }
    #[doc = "Wide Load Enable."]
    #[inline(always)]
    pub const fn set_wide_load(&mut self, val: WideLoad) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Single Flash Read."]
    #[must_use]
    #[inline(always)]
    pub const fn single_rd(&self) -> SingleRd {
        let val = (self.0 >> 2usize) & 0x01;
        SingleRd::from_bits(val as u8)
    }
    #[doc = "Single Flash Read."]
    #[inline(always)]
    pub const fn set_single_rd(&mut self, val: SingleRd) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for FlashRdCtrl {
    #[inline(always)]
    fn default() -> FlashRdCtrl {
        FlashRdCtrl(0)
    }
}
impl core::fmt::Debug for FlashRdCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashRdCtrl")
            .field("flash_rd", &self.flash_rd())
            .field("wide_load", &self.wide_load())
            .field("single_rd", &self.single_rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashRdCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashRdCtrl {{ flash_rd: {:?}, wide_load: {:?}, single_rd: {:?} }}",
            self.flash_rd(),
            self.wide_load(),
            self.single_rd()
        )
    }
}
#[doc = "Flash Stop Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashStopAdd(pub u32);
impl FlashStopAdd {
    #[doc = "Flash Stop Address."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stop_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Flash Stop Address."]
    #[inline(always)]
    pub const fn set_flash_stop_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for FlashStopAdd {
    #[inline(always)]
    fn default() -> FlashStopAdd {
        FlashStopAdd(0)
    }
}
impl core::fmt::Debug for FlashStopAdd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashStopAdd")
            .field("flash_stop_add", &self.flash_stop_add())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashStopAdd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlashStopAdd {{ flash_stop_add: {=u32:?} }}",
            self.flash_stop_add()
        )
    }
}
#[doc = "Flash Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstat(pub u32);
impl Fstat {
    #[doc = "Command Fail Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn fail(&self) -> Fail {
        let val = (self.0 >> 0usize) & 0x01;
        Fail::from_bits(val as u8)
    }
    #[doc = "Command Fail Flag."]
    #[inline(always)]
    pub const fn set_fail(&mut self, val: Fail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Abort Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdabt(&self) -> Cmdabt {
        let val = (self.0 >> 2usize) & 0x01;
        Cmdabt::from_bits(val as u8)
    }
    #[doc = "Command Abort Flag."]
    #[inline(always)]
    pub const fn set_cmdabt(&mut self, val: Cmdabt) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Command Protection Violation Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pviol(&self) -> Pviol {
        let val = (self.0 >> 4usize) & 0x01;
        Pviol::from_bits(val as u8)
    }
    #[doc = "Command Protection Violation Flag."]
    #[inline(always)]
    pub const fn set_pviol(&mut self, val: Pviol) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command Access Error Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn accerr(&self) -> Accerr {
        let val = (self.0 >> 5usize) & 0x01;
        Accerr::from_bits(val as u8)
    }
    #[doc = "Command Access Error Flag."]
    #[inline(always)]
    pub const fn set_accerr(&mut self, val: Accerr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Command Write Sequence Abort Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cwsabt(&self) -> Cwsabt {
        let val = (self.0 >> 6usize) & 0x01;
        Cwsabt::from_bits(val as u8)
    }
    #[doc = "Command Write Sequence Abort Flag."]
    #[inline(always)]
    pub const fn set_cwsabt(&mut self, val: Cwsabt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Command Complete Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn ccif(&self) -> Ccif {
        let val = (self.0 >> 7usize) & 0x01;
        Ccif::from_bits(val as u8)
    }
    #[doc = "Command Complete Interrupt Flag."]
    #[inline(always)]
    pub const fn set_ccif(&mut self, val: Ccif) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Protection Level."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdprt(&self) -> Cmdprt {
        let val = (self.0 >> 8usize) & 0x03;
        Cmdprt::from_bits(val as u8)
    }
    #[doc = "Command Protection Level."]
    #[inline(always)]
    pub const fn set_cmdprt(&mut self, val: Cmdprt) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Command Protection Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdp(&self) -> Cmdp {
        let val = (self.0 >> 11usize) & 0x01;
        Cmdp::from_bits(val as u8)
    }
    #[doc = "Command Protection Status Flag."]
    #[inline(always)]
    pub const fn set_cmdp(&mut self, val: Cmdp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Command Domain ID."]
    #[must_use]
    #[inline(always)]
    pub const fn cmddid(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Command Domain ID."]
    #[inline(always)]
    pub const fn set_cmddid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn dfdif(&self) -> Dfdif {
        let val = (self.0 >> 16usize) & 0x01;
        Dfdif::from_bits(val as u8)
    }
    #[doc = "Double Bit Fault Detect Interrupt Flag."]
    #[inline(always)]
    pub const fn set_dfdif(&mut self, val: Dfdif) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Salvage Used for Erase operation."]
    #[must_use]
    #[inline(always)]
    pub const fn salv_used(&self) -> SalvUsed {
        let val = (self.0 >> 17usize) & 0x01;
        SalvUsed::from_bits(val as u8)
    }
    #[doc = "Salvage Used for Erase operation."]
    #[inline(always)]
    pub const fn set_salv_used(&mut self, val: SalvUsed) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Program-Erase Write Enable Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pewen(&self) -> Pewen {
        let val = (self.0 >> 24usize) & 0x03;
        Pewen::from_bits(val as u8)
    }
    #[doc = "Program-Erase Write Enable Control."]
    #[inline(always)]
    pub const fn set_pewen(&mut self, val: Pewen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Program/Erase Ready Control/Status Flag."]
    #[must_use]
    #[inline(always)]
    pub const fn perdy(&self) -> Perdy {
        let val = (self.0 >> 31usize) & 0x01;
        Perdy::from_bits(val as u8)
    }
    #[doc = "Program/Erase Ready Control/Status Flag."]
    #[inline(always)]
    pub const fn set_perdy(&mut self, val: Perdy) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Fstat {
    #[inline(always)]
    fn default() -> Fstat {
        Fstat(0)
    }
}
impl core::fmt::Debug for Fstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fstat")
            .field("fail", &self.fail())
            .field("cmdabt", &self.cmdabt())
            .field("pviol", &self.pviol())
            .field("accerr", &self.accerr())
            .field("cwsabt", &self.cwsabt())
            .field("ccif", &self.ccif())
            .field("cmdprt", &self.cmdprt())
            .field("cmdp", &self.cmdp())
            .field("cmddid", &self.cmddid())
            .field("dfdif", &self.dfdif())
            .field("salv_used", &self.salv_used())
            .field("pewen", &self.pewen())
            .field("perdy", &self.perdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fstat {{ fail: {:?}, cmdabt: {:?}, pviol: {:?}, accerr: {:?}, cwsabt: {:?}, ccif: {:?}, cmdprt: {:?}, cmdp: {:?}, cmddid: {=u8:?}, dfdif: {:?}, salv_used: {:?}, pewen: {:?}, perdy: {:?} }}",
            self.fail(),
            self.cmdabt(),
            self.pviol(),
            self.accerr(),
            self.cwsabt(),
            self.ccif(),
            self.cmdprt(),
            self.cmdp(),
            self.cmddid(),
            self.dfdif(),
            self.salv_used(),
            self.pewen(),
            self.perdy()
        )
    }
}
#[doc = "Flash Test Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ftest(pub u32);
impl Ftest {
    #[doc = "Test Mode Entry Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tmectl(&self) -> Tmectl {
        let val = (self.0 >> 0usize) & 0x01;
        Tmectl::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Control."]
    #[inline(always)]
    pub const fn set_tmectl(&mut self, val: Tmectl) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Test Mode Entry Writable."]
    #[must_use]
    #[inline(always)]
    pub const fn tmewr(&self) -> Tmewr {
        let val = (self.0 >> 1usize) & 0x01;
        Tmewr::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Writable."]
    #[inline(always)]
    pub const fn set_tmewr(&mut self, val: Tmewr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Test Mode Entry."]
    #[must_use]
    #[inline(always)]
    pub const fn tme(&self) -> Tme {
        let val = (self.0 >> 2usize) & 0x01;
        Tme::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry."]
    #[inline(always)]
    pub const fn set_tme(&mut self, val: Tme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Test Mode Status."]
    #[must_use]
    #[inline(always)]
    pub const fn tmode(&self) -> Tmode {
        let val = (self.0 >> 3usize) & 0x01;
        Tmode::from_bits(val as u8)
    }
    #[doc = "Test Mode Status."]
    #[inline(always)]
    pub const fn set_tmode(&mut self, val: Tmode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Test Mode Entry Lock."]
    #[must_use]
    #[inline(always)]
    pub const fn tmelock(&self) -> Tmelock {
        let val = (self.0 >> 4usize) & 0x01;
        Tmelock::from_bits(val as u8)
    }
    #[doc = "Test Mode Entry Lock."]
    #[inline(always)]
    pub const fn set_tmelock(&mut self, val: Tmelock) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for Ftest {
    #[inline(always)]
    fn default() -> Ftest {
        Ftest(0)
    }
}
impl core::fmt::Debug for Ftest {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ftest")
            .field("tmectl", &self.tmectl())
            .field("tmewr", &self.tmewr())
            .field("tme", &self.tme())
            .field("tmode", &self.tmode())
            .field("tmelock", &self.tmelock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ftest {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ftest {{ tmectl: {:?}, tmewr: {:?}, tme: {:?}, tmode: {:?}, tmelock: {:?} }}",
            self.tmectl(),
            self.tmewr(),
            self.tme(),
            self.tmode(),
            self.tmelock()
        )
    }
}
#[doc = "Maximum Pulse Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MaxPulseCnt(pub u32);
impl MaxPulseCnt {
    #[doc = "Last SMW Operation's Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn last_pcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Last SMW Operation's Pulse Count."]
    #[inline(always)]
    pub const fn set_last_pcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Maximum Erase Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn max_ers_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Maximum Erase Pulse Count."]
    #[inline(always)]
    pub const fn set_max_ers_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
    #[doc = "Maximum Program Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn max_pgm_cnt(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x1f;
        val as u8
    }
    #[doc = "Maximum Program Pulse Count."]
    #[inline(always)]
    pub const fn set_max_pgm_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 27usize)) | (((val as u32) & 0x1f) << 27usize);
    }
}
impl Default for MaxPulseCnt {
    #[inline(always)]
    fn default() -> MaxPulseCnt {
        MaxPulseCnt(0)
    }
}
impl core::fmt::Debug for MaxPulseCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MaxPulseCnt")
            .field("last_pcnt", &self.last_pcnt())
            .field("max_ers_cnt", &self.max_ers_cnt())
            .field("max_pgm_cnt", &self.max_pgm_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MaxPulseCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MaxPulseCnt {{ last_pcnt: {=u16:?}, max_ers_cnt: {=u16:?}, max_pgm_cnt: {=u8:?} }}",
            self.last_pcnt(),
            self.max_ers_cnt(),
            self.max_pgm_cnt()
        )
    }
}
#[doc = "FMU Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctl(pub u32);
impl Mctl {
    #[doc = "Core Hold."]
    #[must_use]
    #[inline(always)]
    pub const fn corehld(&self) -> Corehld {
        let val = (self.0 >> 0usize) & 0x01;
        Corehld::from_bits(val as u8)
    }
    #[doc = "Core Hold."]
    #[inline(always)]
    pub const fn set_corehld(&mut self, val: Corehld) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LSACTIVE Feature Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lsact_en(&self) -> LsactEn {
        let val = (self.0 >> 2usize) & 0x01;
        LsactEn::from_bits(val as u8)
    }
    #[doc = "LSACTIVE Feature Enable."]
    #[inline(always)]
    pub const fn set_lsact_en(&mut self, val: LsactEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSACTIVE Write Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lsactwren(&self) -> Lsactwren {
        let val = (self.0 >> 3usize) & 0x01;
        Lsactwren::from_bits(val as u8)
    }
    #[doc = "LSACTIVE Write Enable."]
    #[inline(always)]
    pub const fn set_lsactwren(&mut self, val: Lsactwren) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Master Repair Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn master_repair_en(&self) -> MasterRepairEn {
        let val = (self.0 >> 4usize) & 0x01;
        MasterRepairEn::from_bits(val as u8)
    }
    #[doc = "Master Repair Enable."]
    #[inline(always)]
    pub const fn set_master_repair_en(&mut self, val: MasterRepairEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "RF Active Command Enable Control."]
    #[must_use]
    #[inline(always)]
    pub const fn rfcmden(&self) -> Rfcmden {
        let val = (self.0 >> 5usize) & 0x01;
        Rfcmden::from_bits(val as u8)
    }
    #[doc = "RF Active Command Enable Control."]
    #[inline(always)]
    pub const fn set_rfcmden(&mut self, val: Rfcmden) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Command Write Sequence Abort Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cwsabten(&self) -> Cwsabten {
        let val = (self.0 >> 6usize) & 0x01;
        Cwsabten::from_bits(val as u8)
    }
    #[doc = "Command Write Sequence Abort Enable."]
    #[inline(always)]
    pub const fn set_cwsabten(&mut self, val: Cwsabten) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Margin Read Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrddis(&self) -> Mrgrddis {
        let val = (self.0 >> 7usize) & 0x01;
        Mrgrddis::from_bits(val as u8)
    }
    #[doc = "Margin Read Disable."]
    #[inline(always)]
    pub const fn set_mrgrddis(&mut self, val: Mrgrddis) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Margin Read Setting for Program."]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrd0(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Margin Read Setting for Program."]
    #[inline(always)]
    pub const fn set_mrgrd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Margin Read Setting for Erase."]
    #[must_use]
    #[inline(always)]
    pub const fn mrgrd1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Margin Read Setting for Erase."]
    #[inline(always)]
    pub const fn set_mrgrd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Mass Erase (Erase All) Acknowledge."]
    #[must_use]
    #[inline(always)]
    pub const fn ersaack(&self) -> Ersaack {
        let val = (self.0 >> 16usize) & 0x01;
        Ersaack::from_bits(val as u8)
    }
    #[doc = "Mass Erase (Erase All) Acknowledge."]
    #[inline(always)]
    pub const fn set_ersaack(&mut self, val: Ersaack) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Scan Observability Control."]
    #[must_use]
    #[inline(always)]
    pub const fn scan_obs(&self) -> ScanObs {
        let val = (self.0 >> 19usize) & 0x01;
        ScanObs::from_bits(val as u8)
    }
    #[doc = "Scan Observability Control."]
    #[inline(always)]
    pub const fn set_scan_obs(&mut self, val: ScanObs) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "BIST IP Control."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_ctl(&self) -> BistCtl {
        let val = (self.0 >> 20usize) & 0x01;
        BistCtl::from_bits(val as u8)
    }
    #[doc = "BIST IP Control."]
    #[inline(always)]
    pub const fn set_bist_ctl(&mut self, val: BistCtl) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "SMWR IP Control."]
    #[must_use]
    #[inline(always)]
    pub const fn smwr_ctl(&self) -> SmwrCtl {
        let val = (self.0 >> 21usize) & 0x01;
        SmwrCtl::from_bits(val as u8)
    }
    #[doc = "SMWR IP Control."]
    #[inline(always)]
    pub const fn set_smwr_ctl(&mut self, val: SmwrCtl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Salvage Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn salv_dis(&self) -> SalvDis {
        let val = (self.0 >> 24usize) & 0x01;
        SalvDis::from_bits(val as u8)
    }
    #[doc = "Salvage Disable."]
    #[inline(always)]
    pub const fn set_salv_dis(&mut self, val: SalvDis) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SOC ECC Control."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_ecc_ctl(&self) -> SocEccCtl {
        let val = (self.0 >> 25usize) & 0x01;
        SocEccCtl::from_bits(val as u8)
    }
    #[doc = "SOC ECC Control."]
    #[inline(always)]
    pub const fn set_soc_ecc_ctl(&mut self, val: SocEccCtl) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FMU ECC Control."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_ecc_ctl(&self) -> FmuEccCtl {
        let val = (self.0 >> 26usize) & 0x01;
        FmuEccCtl::from_bits(val as u8)
    }
    #[doc = "FMU ECC Control."]
    #[inline(always)]
    pub const fn set_fmu_ecc_ctl(&mut self, val: FmuEccCtl) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BIST Power Mode Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_pwr_dis(&self) -> BistPwrDis {
        let val = (self.0 >> 29usize) & 0x01;
        BistPwrDis::from_bits(val as u8)
    }
    #[doc = "BIST Power Mode Disable."]
    #[inline(always)]
    pub const fn set_bist_pwr_dis(&mut self, val: BistPwrDis) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Oscillator control."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_h(&self) -> OscH {
        let val = (self.0 >> 31usize) & 0x01;
        OscH::from_bits(val as u8)
    }
    #[doc = "Oscillator control."]
    #[inline(always)]
    pub const fn set_osc_h(&mut self, val: OscH) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Mctl {
    #[inline(always)]
    fn default() -> Mctl {
        Mctl(0)
    }
}
impl core::fmt::Debug for Mctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mctl")
            .field("corehld", &self.corehld())
            .field("lsact_en", &self.lsact_en())
            .field("lsactwren", &self.lsactwren())
            .field("master_repair_en", &self.master_repair_en())
            .field("rfcmden", &self.rfcmden())
            .field("cwsabten", &self.cwsabten())
            .field("mrgrddis", &self.mrgrddis())
            .field("mrgrd0", &self.mrgrd0())
            .field("mrgrd1", &self.mrgrd1())
            .field("ersaack", &self.ersaack())
            .field("scan_obs", &self.scan_obs())
            .field("bist_ctl", &self.bist_ctl())
            .field("smwr_ctl", &self.smwr_ctl())
            .field("salv_dis", &self.salv_dis())
            .field("soc_ecc_ctl", &self.soc_ecc_ctl())
            .field("fmu_ecc_ctl", &self.fmu_ecc_ctl())
            .field("bist_pwr_dis", &self.bist_pwr_dis())
            .field("osc_h", &self.osc_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Mctl {{ corehld: {:?}, lsact_en: {:?}, lsactwren: {:?}, master_repair_en: {:?}, rfcmden: {:?}, cwsabten: {:?}, mrgrddis: {:?}, mrgrd0: {=u8:?}, mrgrd1: {=u8:?}, ersaack: {:?}, scan_obs: {:?}, bist_ctl: {:?}, smwr_ctl: {:?}, salv_dis: {:?}, soc_ecc_ctl: {:?}, fmu_ecc_ctl: {:?}, bist_pwr_dis: {:?}, osc_h: {:?} }}",
            self.corehld(),
            self.lsact_en(),
            self.lsactwren(),
            self.master_repair_en(),
            self.rfcmden(),
            self.cwsabten(),
            self.mrgrddis(),
            self.mrgrd0(),
            self.mrgrd1(),
            self.ersaack(),
            self.scan_obs(),
            self.bist_ctl(),
            self.smwr_ctl(),
            self.salv_dis(),
            self.soc_ecc_ctl(),
            self.fmu_ecc_ctl(),
            self.bist_pwr_dis(),
            self.osc_h()
        )
    }
}
#[doc = "Memory Map Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmAddr(pub u32);
impl MmAddr {
    #[doc = "Memory Map Address."]
    #[must_use]
    #[inline(always)]
    pub const fn mm_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory Map Address."]
    #[inline(always)]
    pub const fn set_mm_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MmAddr {
    #[inline(always)]
    fn default() -> MmAddr {
        MmAddr(0)
    }
}
impl core::fmt::Debug for MmAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmAddr")
            .field("mm_addr", &self.mm_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MmAddr {{ mm_addr: {=u32:?} }}", self.mm_addr())
    }
}
#[doc = "Memory Map Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmCtl(pub u32);
impl MmCtl {
    #[doc = "Register Access Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mm_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Register Access Enable."]
    #[inline(always)]
    pub const fn set_mm_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Register R/W Control."]
    #[must_use]
    #[inline(always)]
    pub const fn mm_rd(&self) -> MmRd {
        let val = (self.0 >> 1usize) & 0x01;
        MmRd::from_bits(val as u8)
    }
    #[doc = "Register R/W Control."]
    #[inline(always)]
    pub const fn set_mm_rd(&mut self, val: MmRd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST on."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_on(&self) -> BistOn {
        let val = (self.0 >> 2usize) & 0x01;
        BistOn::from_bits(val as u8)
    }
    #[doc = "BIST on."]
    #[inline(always)]
    pub const fn set_bist_on(&mut self, val: BistOn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Force Switch Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn force_sw_clk(&self) -> ForceSwClk {
        let val = (self.0 >> 3usize) & 0x01;
        ForceSwClk::from_bits(val as u8)
    }
    #[doc = "Force Switch Clock."]
    #[inline(always)]
    pub const fn set_force_sw_clk(&mut self, val: ForceSwClk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for MmCtl {
    #[inline(always)]
    fn default() -> MmCtl {
        MmCtl(0)
    }
}
impl core::fmt::Debug for MmCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmCtl")
            .field("mm_sel", &self.mm_sel())
            .field("mm_rd", &self.mm_rd())
            .field("bist_on", &self.bist_on())
            .field("force_sw_clk", &self.force_sw_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MmCtl {{ mm_sel: {=bool:?}, mm_rd: {:?}, bist_on: {:?}, force_sw_clk: {:?} }}",
            self.mm_sel(),
            self.mm_rd(),
            self.bist_on(),
            self.force_sw_clk()
        )
    }
}
#[doc = "Memory Map Write Data Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmWdata(pub u32);
impl MmWdata {
    #[doc = "Memory Map Write Data."]
    #[must_use]
    #[inline(always)]
    pub const fn mm_wdata(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Memory Map Write Data."]
    #[inline(always)]
    pub const fn set_mm_wdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MmWdata {
    #[inline(always)]
    fn default() -> MmWdata {
        MmWdata(0)
    }
}
impl core::fmt::Debug for MmWdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmWdata")
            .field("mm_wdata", &self.mm_wdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmWdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MmWdata {{ mm_wdata: {=u32:?} }}", self.mm_wdata())
    }
}
#[doc = "FMU Memory Size Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msize(pub u32);
impl Msize {
    #[doc = "Size of Flash Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn maxaddr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Size of Flash Block 0."]
    #[inline(always)]
    pub const fn set_maxaddr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Msize {
    #[inline(always)]
    fn default() -> Msize {
        Msize(0)
    }
}
impl core::fmt::Debug for Msize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msize")
            .field("maxaddr0", &self.maxaddr0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msize {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Msize {{ maxaddr0: {=u8:?} }}", self.maxaddr0())
    }
}
#[doc = "Parity Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parity(pub u32);
impl Parity {
    #[doc = "Read data \\[136:128\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn parity(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Read data \\[136:128\\]."]
    #[inline(always)]
    pub const fn set_parity(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Parity {
    #[inline(always)]
    fn default() -> Parity {
        Parity(0)
    }
}
impl core::fmt::Debug for Parity {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parity")
            .field("parity", &self.parity())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Parity {{ parity: {=u16:?} }}", self.parity())
    }
}
#[doc = "Block 0 Program Pulse Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmPulseCnt0(pub u32);
impl PgmPulseCnt0 {
    #[doc = "Program Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pgm_cnt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Program Pulse Count."]
    #[inline(always)]
    pub const fn set_pgm_cnt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmPulseCnt0 {
    #[inline(always)]
    fn default() -> PgmPulseCnt0 {
        PgmPulseCnt0(0)
    }
}
impl core::fmt::Debug for PgmPulseCnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmPulseCnt0")
            .field("pgm_cnt0", &self.pgm_cnt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmPulseCnt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmPulseCnt0 {{ pgm_cnt0: {=u32:?} }}", self.pgm_cnt0())
    }
}
#[doc = "Block 1 Program Pulse Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgmPulseCnt1(pub u32);
impl PgmPulseCnt1 {
    #[doc = "Program Pulse Count."]
    #[must_use]
    #[inline(always)]
    pub const fn pgm_cnt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Program Pulse Count."]
    #[inline(always)]
    pub const fn set_pgm_cnt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for PgmPulseCnt1 {
    #[inline(always)]
    fn default() -> PgmPulseCnt1 {
        PgmPulseCnt1(0)
    }
}
impl core::fmt::Debug for PgmPulseCnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgmPulseCnt1")
            .field("pgm_cnt1", &self.pgm_cnt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgmPulseCnt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PgmPulseCnt1 {{ pgm_cnt1: {=u32:?} }}", self.pgm_cnt1())
    }
}
#[doc = "Port Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PortCtrl(pub u32);
impl PortCtrl {
    #[doc = "BIST Done Select."]
    #[must_use]
    #[inline(always)]
    pub const fn bdone_sel(&self) -> BdoneSel {
        let val = (self.0 >> 0usize) & 0x03;
        BdoneSel::from_bits(val as u8)
    }
    #[doc = "BIST Done Select."]
    #[inline(always)]
    pub const fn set_bdone_sel(&mut self, val: BdoneSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "BIST Serial Data Output Select."]
    #[must_use]
    #[inline(always)]
    pub const fn bsdo_sel(&self) -> BsdoSel {
        let val = (self.0 >> 2usize) & 0x03;
        BsdoSel::from_bits(val as u8)
    }
    #[doc = "BIST Serial Data Output Select."]
    #[inline(always)]
    pub const fn set_bsdo_sel(&mut self, val: BsdoSel) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for PortCtrl {
    #[inline(always)]
    fn default() -> PortCtrl {
        PortCtrl(0)
    }
}
impl core::fmt::Debug for PortCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PortCtrl")
            .field("bdone_sel", &self.bdone_sel())
            .field("bsdo_sel", &self.bsdo_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PortCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PortCtrl {{ bdone_sel: {:?}, bsdo_sel: {:?} }}",
            self.bdone_sel(),
            self.bsdo_sel()
        )
    }
}
#[doc = "Power Mode Options Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrOpt(pub u32);
impl PwrOpt {
    #[doc = "Power Down Clock Divider Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn pd_cdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Power Down Clock Divider Setting."]
    #[inline(always)]
    pub const fn set_pd_cdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Sleep Recovery Timer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn slm_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sleep Recovery Timer Count."]
    #[inline(always)]
    pub const fn set_slm_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Power Down BIST Timer Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pd_timer_en(&self) -> PdTimerEn {
        let val = (self.0 >> 31usize) & 0x01;
        PdTimerEn::from_bits(val as u8)
    }
    #[doc = "Power Down BIST Timer Enable."]
    #[inline(always)]
    pub const fn set_pd_timer_en(&mut self, val: PdTimerEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for PwrOpt {
    #[inline(always)]
    fn default() -> PwrOpt {
        PwrOpt(0)
    }
}
impl core::fmt::Debug for PwrOpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwrOpt")
            .field("pd_cdiv", &self.pd_cdiv())
            .field("slm_count", &self.slm_count())
            .field("pd_timer_en", &self.pd_timer_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwrOpt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PwrOpt {{ pd_cdiv: {=u8:?}, slm_count: {=u16:?}, pd_timer_en: {:?} }}",
            self.pd_cdiv(),
            self.slm_count(),
            self.pd_timer_en()
        )
    }
}
#[doc = "BIST Address MISR 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMisr0(pub u32);
impl RAMisr0 {
    #[doc = "Address Signature."]
    #[must_use]
    #[inline(always)]
    pub const fn adrsig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Address Signature."]
    #[inline(always)]
    pub const fn set_adrsig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RAMisr0 {
    #[inline(always)]
    fn default() -> RAMisr0 {
        RAMisr0(0)
    }
}
impl core::fmt::Debug for RAMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMisr0")
            .field("adrsig0", &self.adrsig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAMisr0 {{ adrsig0: {=u32:?} }}", self.adrsig0())
    }
}
#[doc = "BIST Address MISR 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAMisr1(pub u32);
impl RAMisr1 {
    #[doc = "MISR Address Signature High."]
    #[must_use]
    #[inline(always)]
    pub const fn adrsig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Address Signature High."]
    #[inline(always)]
    pub const fn set_adrsig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RAMisr1 {
    #[inline(always)]
    fn default() -> RAMisr1 {
        RAMisr1(0)
    }
}
impl core::fmt::Debug for RAMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAMisr1")
            .field("adrsig1", &self.adrsig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAMisr1 {{ adrsig1: {=u8:?} }}", self.adrsig1())
    }
}
#[doc = "BIST Abort Loop Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAbortLoop(pub u32);
impl RAbortLoop {
    #[doc = "Abort Loop."]
    #[must_use]
    #[inline(always)]
    pub const fn abort_loop(&self) -> AbortLoop {
        let val = (self.0 >> 0usize) & 0x01;
        AbortLoop::from_bits(val as u8)
    }
    #[doc = "Abort Loop."]
    #[inline(always)]
    pub const fn set_abort_loop(&mut self, val: AbortLoop) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RAbortLoop {
    #[inline(always)]
    fn default() -> RAbortLoop {
        RAbortLoop(0)
    }
}
impl core::fmt::Debug for RAbortLoop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAbortLoop")
            .field("abort_loop", &self.abort_loop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAbortLoop {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RAbortLoop {{ abort_loop: {:?} }}", self.abort_loop())
    }
}
#[doc = "BIST Address Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAdrCtrl(pub u32);
impl RAdrCtrl {
    #[doc = "Data Group Select."]
    #[must_use]
    #[inline(always)]
    pub const fn grpsel(&self) -> Grpsel {
        let val = (self.0 >> 0usize) & 0x0f;
        Grpsel::from_bits(val as u8)
    }
    #[doc = "Data Group Select."]
    #[inline(always)]
    pub const fn set_grpsel(&mut self, val: Grpsel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "BIST XADR."]
    #[must_use]
    #[inline(always)]
    pub const fn xadr(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "BIST XADR."]
    #[inline(always)]
    pub const fn set_xadr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
    #[doc = "BIST YADR."]
    #[must_use]
    #[inline(always)]
    pub const fn yadr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "BIST YADR."]
    #[inline(always)]
    pub const fn set_yadr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Program Attribute."]
    #[must_use]
    #[inline(always)]
    pub const fn prog_attr(&self) -> ProgAttr {
        let val = (self.0 >> 21usize) & 0x07;
        ProgAttr::from_bits(val as u8)
    }
    #[doc = "Program Attribute."]
    #[inline(always)]
    pub const fn set_prog_attr(&mut self, val: ProgAttr) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
}
impl Default for RAdrCtrl {
    #[inline(always)]
    fn default() -> RAdrCtrl {
        RAdrCtrl(0)
    }
}
impl core::fmt::Debug for RAdrCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAdrCtrl")
            .field("grpsel", &self.grpsel())
            .field("xadr", &self.xadr())
            .field("yadr", &self.yadr())
            .field("prog_attr", &self.prog_attr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAdrCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAdrCtrl {{ grpsel: {:?}, xadr: {=u16:?}, yadr: {=u8:?}, prog_attr: {:?} }}",
            self.grpsel(),
            self.xadr(),
            self.yadr(),
            self.prog_attr()
        )
    }
}
#[doc = "BIST Address Query Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RAdrQuery(pub u32);
impl RAdrQuery {
    #[doc = "Failing YADR."]
    #[must_use]
    #[inline(always)]
    pub const fn yadrfail(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Failing YADR."]
    #[inline(always)]
    pub const fn set_yadrfail(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Failing XADR."]
    #[must_use]
    #[inline(always)]
    pub const fn xadrfail(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x0fff;
        val as u16
    }
    #[doc = "Failing XADR."]
    #[inline(always)]
    pub const fn set_xadrfail(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 5usize)) | (((val as u32) & 0x0fff) << 5usize);
    }
}
impl Default for RAdrQuery {
    #[inline(always)]
    fn default() -> RAdrQuery {
        RAdrQuery(0)
    }
}
impl core::fmt::Debug for RAdrQuery {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAdrQuery")
            .field("yadrfail", &self.yadrfail())
            .field("xadrfail", &self.xadrfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RAdrQuery {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RAdrQuery {{ yadrfail: {=u8:?}, xadrfail: {=u16:?} }}",
            self.yadrfail(),
            self.xadrfail()
        )
    }
}
#[doc = "BIST Control MISR 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCMisr0(pub u32);
impl RCMisr0 {
    #[doc = "Control Signature."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrlsig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Control Signature."]
    #[inline(always)]
    pub const fn set_ctrlsig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RCMisr0 {
    #[inline(always)]
    fn default() -> RCMisr0 {
        RCMisr0(0)
    }
}
impl core::fmt::Debug for RCMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCMisr0")
            .field("ctrlsig0", &self.ctrlsig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCMisr0 {{ ctrlsig0: {=u32:?} }}", self.ctrlsig0())
    }
}
#[doc = "BIST Control MISR 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCMisr1(pub u32);
impl RCMisr1 {
    #[doc = "MISR Control Signature High."]
    #[must_use]
    #[inline(always)]
    pub const fn ctrlsig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Control Signature High."]
    #[inline(always)]
    pub const fn set_ctrlsig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RCMisr1 {
    #[inline(always)]
    fn default() -> RCMisr1 {
        RCMisr1(0)
    }
}
impl core::fmt::Debug for RCMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCMisr1")
            .field("ctrlsig1", &self.ctrlsig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RCMisr1 {{ ctrlsig1: {=u8:?} }}", self.ctrlsig1())
    }
}
#[doc = "BIST Loop Count Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RCntLoopCtrl(pub u32);
impl RCntLoopCtrl {
    #[doc = "Loop Count Control."]
    #[must_use]
    #[inline(always)]
    pub const fn loopcnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Loop Count Control."]
    #[inline(always)]
    pub const fn set_loopcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Loop Option."]
    #[must_use]
    #[inline(always)]
    pub const fn loopopt(&self) -> Loopopt {
        let val = (self.0 >> 12usize) & 0x07;
        Loopopt::from_bits(val as u8)
    }
    #[doc = "Loop Option."]
    #[inline(always)]
    pub const fn set_loopopt(&mut self, val: Loopopt) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Loop Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn loopunit(&self) -> Loopunit {
        let val = (self.0 >> 15usize) & 0x07;
        Loopunit::from_bits(val as u8)
    }
    #[doc = "Loop Time Unit."]
    #[inline(always)]
    pub const fn set_loopunit(&mut self, val: Loopunit) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
    }
    #[doc = "Loop Time Delay Scalar."]
    #[must_use]
    #[inline(always)]
    pub const fn loopdly(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x7f;
        val as u8
    }
    #[doc = "Loop Time Delay Scalar."]
    #[inline(always)]
    pub const fn set_loopdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
    }
}
impl Default for RCntLoopCtrl {
    #[inline(always)]
    fn default() -> RCntLoopCtrl {
        RCntLoopCtrl(0)
    }
}
impl core::fmt::Debug for RCntLoopCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCntLoopCtrl")
            .field("loopcnt", &self.loopcnt())
            .field("loopopt", &self.loopopt())
            .field("loopunit", &self.loopunit())
            .field("loopdly", &self.loopdly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RCntLoopCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RCntLoopCtrl {{ loopcnt: {=u16:?}, loopopt: {:?}, loopunit: {:?}, loopdly: {=u8:?} }}",
            self.loopcnt(),
            self.loopopt(),
            self.loopunit(),
            self.loopdly()
        )
    }
}
#[doc = "BIST DIN MISR 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDMisr0(pub u32);
impl RDMisr0 {
    #[doc = "Data Signature."]
    #[must_use]
    #[inline(always)]
    pub const fn datasig0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data Signature."]
    #[inline(always)]
    pub const fn set_datasig0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDMisr0 {
    #[inline(always)]
    fn default() -> RDMisr0 {
        RDMisr0(0)
    }
}
impl core::fmt::Debug for RDMisr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDMisr0")
            .field("datasig0", &self.datasig0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDMisr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDMisr0 {{ datasig0: {=u32:?} }}", self.datasig0())
    }
}
#[doc = "BIST DIN MISR 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDMisr1(pub u32);
impl RDMisr1 {
    #[doc = "MISR Data Signature High."]
    #[must_use]
    #[inline(always)]
    pub const fn datasig1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "MISR Data Signature High."]
    #[inline(always)]
    pub const fn set_datasig1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RDMisr1 {
    #[inline(always)]
    fn default() -> RDMisr1 {
        RDMisr1(0)
    }
}
impl core::fmt::Debug for RDMisr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDMisr1")
            .field("datasig1", &self.datasig1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDMisr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDMisr1 {{ datasig1: {=u8:?} }}", self.datasig1())
    }
}
#[doc = "BIST Data Control 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl0(pub u32);
impl RDataCtrl0 {
    #[doc = "BIST Data 0 Low."]
    #[must_use]
    #[inline(always)]
    pub const fn data0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 0 Low."]
    #[inline(always)]
    pub const fn set_data0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl0 {
    #[inline(always)]
    fn default() -> RDataCtrl0 {
        RDataCtrl0(0)
    }
}
impl core::fmt::Debug for RDataCtrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl0")
            .field("data0", &self.data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl0 {{ data0: {=u32:?} }}", self.data0())
    }
}
#[doc = "BIST Data Control 0 Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl0Ex(pub u32);
impl RDataCtrl0Ex {
    #[doc = "BIST Data 0 High."]
    #[must_use]
    #[inline(always)]
    pub const fn data0x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 0 High."]
    #[inline(always)]
    pub const fn set_data0x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl0Ex {
    #[inline(always)]
    fn default() -> RDataCtrl0Ex {
        RDataCtrl0Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl0Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl0Ex")
            .field("data0x", &self.data0x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl0Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl0Ex {{ data0x: {=u8:?} }}", self.data0x())
    }
}
#[doc = "BIST Data Control 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl1(pub u32);
impl RDataCtrl1 {
    #[doc = "BIST Data 1 Low."]
    #[must_use]
    #[inline(always)]
    pub const fn data1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 1 Low."]
    #[inline(always)]
    pub const fn set_data1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl1 {
    #[inline(always)]
    fn default() -> RDataCtrl1 {
        RDataCtrl1(0)
    }
}
impl core::fmt::Debug for RDataCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl1")
            .field("data1", &self.data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl1 {{ data1: {=u32:?} }}", self.data1())
    }
}
#[doc = "BIST Data Control 1 Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl1Ex(pub u32);
impl RDataCtrl1Ex {
    #[doc = "BIST Data 1 High."]
    #[must_use]
    #[inline(always)]
    pub const fn data1x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 1 High."]
    #[inline(always)]
    pub const fn set_data1x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl1Ex {
    #[inline(always)]
    fn default() -> RDataCtrl1Ex {
        RDataCtrl1Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl1Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl1Ex")
            .field("data1x", &self.data1x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl1Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl1Ex {{ data1x: {=u8:?} }}", self.data1x())
    }
}
#[doc = "BIST Data Control 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl2(pub u32);
impl RDataCtrl2 {
    #[doc = "BIST Data 2 Low."]
    #[must_use]
    #[inline(always)]
    pub const fn data2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 2 Low."]
    #[inline(always)]
    pub const fn set_data2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl2 {
    #[inline(always)]
    fn default() -> RDataCtrl2 {
        RDataCtrl2(0)
    }
}
impl core::fmt::Debug for RDataCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl2")
            .field("data2", &self.data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl2 {{ data2: {=u32:?} }}", self.data2())
    }
}
#[doc = "BIST Data Control 2 Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl2Ex(pub u32);
impl RDataCtrl2Ex {
    #[doc = "BIST Data 2 High."]
    #[must_use]
    #[inline(always)]
    pub const fn data2x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 2 High."]
    #[inline(always)]
    pub const fn set_data2x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl2Ex {
    #[inline(always)]
    fn default() -> RDataCtrl2Ex {
        RDataCtrl2Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl2Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl2Ex")
            .field("data2x", &self.data2x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl2Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl2Ex {{ data2x: {=u8:?} }}", self.data2x())
    }
}
#[doc = "BIST Data Control 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl3(pub u32);
impl RDataCtrl3 {
    #[doc = "BIST Data 3 Low."]
    #[must_use]
    #[inline(always)]
    pub const fn data3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "BIST Data 3 Low."]
    #[inline(always)]
    pub const fn set_data3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDataCtrl3 {
    #[inline(always)]
    fn default() -> RDataCtrl3 {
        RDataCtrl3(0)
    }
}
impl core::fmt::Debug for RDataCtrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl3")
            .field("data3", &self.data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl3 {{ data3: {=u32:?} }}", self.data3())
    }
}
#[doc = "BIST Data Control 3 Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDataCtrl3Ex(pub u32);
impl RDataCtrl3Ex {
    #[doc = "BIST Data 3 High."]
    #[must_use]
    #[inline(always)]
    pub const fn data3x(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BIST Data 3 High."]
    #[inline(always)]
    pub const fn set_data3x(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDataCtrl3Ex {
    #[inline(always)]
    fn default() -> RDataCtrl3Ex {
        RDataCtrl3Ex(0)
    }
}
impl core::fmt::Debug for RDataCtrl3Ex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDataCtrl3Ex")
            .field("data3x", &self.data3x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDataCtrl3Ex {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDataCtrl3Ex {{ data3x: {=u8:?} }}", self.data3x())
    }
}
#[doc = "BIST DFT Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDftCtrl(pub u32);
impl RDftCtrl {
    #[doc = "DFT XADR Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn dft_xadr(&self) -> DftXadr {
        let val = (self.0 >> 0usize) & 0x0f;
        DftXadr::from_bits(val as u8)
    }
    #[doc = "DFT XADR Pattern."]
    #[inline(always)]
    pub const fn set_dft_xadr(&mut self, val: DftXadr) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "DFT YADR Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn dft_yadr(&self) -> DftYadr {
        let val = (self.0 >> 4usize) & 0x0f;
        DftYadr::from_bits(val as u8)
    }
    #[doc = "DFT YADR Pattern."]
    #[inline(always)]
    pub const fn set_dft_yadr(&mut self, val: DftYadr) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "DFT Data Pattern."]
    #[must_use]
    #[inline(always)]
    pub const fn dft_data(&self) -> DftData {
        let val = (self.0 >> 8usize) & 0x0f;
        DftData::from_bits(val as u8)
    }
    #[doc = "DFT Data Pattern."]
    #[inline(always)]
    pub const fn set_dft_data(&mut self, val: DftData) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Data Compare Mask."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp_mask(&self) -> CmpMask {
        let val = (self.0 >> 12usize) & 0x03;
        CmpMask::from_bits(val as u8)
    }
    #[doc = "Data Compare Mask."]
    #[inline(always)]
    pub const fn set_cmp_mask(&mut self, val: CmpMask) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "DFT Data Source."]
    #[must_use]
    #[inline(always)]
    pub const fn dft_data_src(&self) -> DftDataSrc {
        let val = (self.0 >> 14usize) & 0x01;
        DftDataSrc::from_bits(val as u8)
    }
    #[doc = "DFT Data Source."]
    #[inline(always)]
    pub const fn set_dft_data_src(&mut self, val: DftDataSrc) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
}
impl Default for RDftCtrl {
    #[inline(always)]
    fn default() -> RDftCtrl {
        RDftCtrl(0)
    }
}
impl core::fmt::Debug for RDftCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDftCtrl")
            .field("dft_xadr", &self.dft_xadr())
            .field("dft_yadr", &self.dft_yadr())
            .field("dft_data", &self.dft_data())
            .field("cmp_mask", &self.cmp_mask())
            .field("dft_data_src", &self.dft_data_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDftCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RDftCtrl {{ dft_xadr: {:?}, dft_yadr: {:?}, dft_data: {:?}, cmp_mask: {:?}, dft_data_src: {:?} }}",
            self.dft_xadr(),
            self.dft_yadr(),
            self.dft_data(),
            self.cmp_mask(),
            self.dft_data_src()
        )
    }
}
#[doc = "BIST DOUT Query 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDoutQuery0(pub u32);
impl RDoutQuery0 {
    #[doc = "Failing DOUT Low."]
    #[must_use]
    #[inline(always)]
    pub const fn doutfail(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Failing DOUT Low."]
    #[inline(always)]
    pub const fn set_doutfail(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RDoutQuery0 {
    #[inline(always)]
    fn default() -> RDoutQuery0 {
        RDoutQuery0(0)
    }
}
impl core::fmt::Debug for RDoutQuery0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDoutQuery0")
            .field("doutfail", &self.doutfail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDoutQuery0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDoutQuery0 {{ doutfail: {=u32:?} }}", self.doutfail())
    }
}
#[doc = "BIST DOUT Query 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RDoutQuery1(pub u32);
impl RDoutQuery1 {
    #[doc = "Failing DOUT High."]
    #[must_use]
    #[inline(always)]
    pub const fn dout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Failing DOUT High."]
    #[inline(always)]
    pub const fn set_dout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RDoutQuery1 {
    #[inline(always)]
    fn default() -> RDoutQuery1 {
        RDoutQuery1(0)
    }
}
impl core::fmt::Debug for RDoutQuery1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDoutQuery1")
            .field("dout", &self.dout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RDoutQuery1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RDoutQuery1 {{ dout: {=u8:?} }}", self.dout())
    }
}
#[doc = "BIST Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RIpConfig(pub u32);
impl RIpConfig {
    #[doc = "Block 0 Select Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ipsel0(&self) -> Ipsel0 {
        let val = (self.0 >> 0usize) & 0x03;
        Ipsel0::from_bits(val as u8)
    }
    #[doc = "Block 0 Select Control."]
    #[inline(always)]
    pub const fn set_ipsel0(&mut self, val: Ipsel0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Block 1 Select Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ipsel1(&self) -> Ipsel1 {
        let val = (self.0 >> 2usize) & 0x03;
        Ipsel1::from_bits(val as u8)
    }
    #[doc = "Block 1 Select Control."]
    #[inline(always)]
    pub const fn set_ipsel1(&mut self, val: Ipsel1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Clock Divide Scalar for Long Pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_cdivl(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divide Scalar for Long Pulse."]
    #[inline(always)]
    pub const fn set_bist_cdivl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
    #[doc = "Number of clock cycles to generate short pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn cdivs(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of clock cycles to generate short pulse."]
    #[inline(always)]
    pub const fn set_cdivs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer adjust for verify."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_tvfy(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer adjust for verify."]
    #[inline(always)]
    pub const fn set_bist_tvfy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "BIST self-test control."]
    #[must_use]
    #[inline(always)]
    pub const fn tstctl(&self) -> Tstctl {
        let val = (self.0 >> 20usize) & 0x03;
        Tstctl::from_bits(val as u8)
    }
    #[doc = "BIST self-test control."]
    #[inline(always)]
    pub const fn set_tstctl(&mut self, val: Tstctl) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Debug feature control."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgctl(&self) -> Dbgctl {
        let val = (self.0 >> 22usize) & 0x01;
        Dbgctl::from_bits(val as u8)
    }
    #[doc = "Debug feature control."]
    #[inline(always)]
    pub const fn set_dbgctl(&mut self, val: Dbgctl) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "BIST Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_clk_sel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Clock Select."]
    #[inline(always)]
    pub const fn set_bist_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "SMWR DOUT Function Control."]
    #[must_use]
    #[inline(always)]
    pub const fn smwtst(&self) -> Smwtst {
        let val = (self.0 >> 24usize) & 0x03;
        Smwtst::from_bits(val as u8)
    }
    #[doc = "SMWR DOUT Function Control."]
    #[inline(always)]
    pub const fn set_smwtst(&mut self, val: Smwtst) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "BIST ECC Control."]
    #[must_use]
    #[inline(always)]
    pub const fn eccen(&self) -> Eccen {
        let val = (self.0 >> 26usize) & 0x01;
        Eccen::from_bits(val as u8)
    }
    #[doc = "BIST ECC Control."]
    #[inline(always)]
    pub const fn set_eccen(&mut self, val: Eccen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for RIpConfig {
    #[inline(always)]
    fn default() -> RIpConfig {
        RIpConfig(0)
    }
}
impl core::fmt::Debug for RIpConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIpConfig")
            .field("ipsel0", &self.ipsel0())
            .field("ipsel1", &self.ipsel1())
            .field("bist_cdivl", &self.bist_cdivl())
            .field("cdivs", &self.cdivs())
            .field("bist_tvfy", &self.bist_tvfy())
            .field("tstctl", &self.tstctl())
            .field("dbgctl", &self.dbgctl())
            .field("bist_clk_sel", &self.bist_clk_sel())
            .field("smwtst", &self.smwtst())
            .field("eccen", &self.eccen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RIpConfig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RIpConfig {{ ipsel0: {:?}, ipsel1: {:?}, bist_cdivl: {=u8:?}, cdivs: {=u8:?}, bist_tvfy: {=u8:?}, tstctl: {:?}, dbgctl: {:?}, bist_clk_sel: {=bool:?}, smwtst: {:?}, eccen: {:?} }}",
            self.ipsel0(),
            self.ipsel1(),
            self.bist_cdivl(),
            self.cdivs(),
            self.bist_tvfy(),
            self.tstctl(),
            self.dbgctl(),
            self.bist_clk_sel(),
            self.smwtst(),
            self.eccen()
        )
    }
}
#[doc = "BIST Pin Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RPinCtrl(pub u32);
impl RPinCtrl {
    #[doc = "Mass Erase."]
    #[must_use]
    #[inline(always)]
    pub const fn mas1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mass Erase."]
    #[inline(always)]
    pub const fn set_mas1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IFR Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ifren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IFR Enable."]
    #[inline(always)]
    pub const fn set_ifren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IFR1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ifren1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "IFR1 Enable."]
    #[inline(always)]
    pub const fn set_ifren1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Redundancy Block Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn reden(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Redundancy Block Enable."]
    #[inline(always)]
    pub const fn set_reden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Low Voltage Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn lve(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Low Voltage Enable."]
    #[inline(always)]
    pub const fn set_lve(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Program Verify Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Program Verify Enable."]
    #[inline(always)]
    pub const fn set_pv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Erase Verify Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ev(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Verify Enable."]
    #[inline(always)]
    pub const fn set_ev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Program Current."]
    #[must_use]
    #[inline(always)]
    pub const fn wipgm(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current."]
    #[inline(always)]
    pub const fn set_wipgm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "High Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn whv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "High Voltage Level."]
    #[inline(always)]
    pub const fn set_whv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "Medium Voltage Level."]
    #[must_use]
    #[inline(always)]
    pub const fn wmv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level."]
    #[inline(always)]
    pub const fn set_wmv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "X Address Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn xe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "X Address Enable."]
    #[inline(always)]
    pub const fn set_xe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Y Address Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ye(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Y Address Enable."]
    #[inline(always)]
    pub const fn set_ye(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Sense Amp Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn se(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Sense Amp Enable."]
    #[inline(always)]
    pub const fn set_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Erase Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn erase(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Erase Mode."]
    #[inline(always)]
    pub const fn set_erase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Program Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn prog(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Program Mode."]
    #[inline(always)]
    pub const fn set_prog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "NVM Store."]
    #[must_use]
    #[inline(always)]
    pub const fn nvstr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "NVM Store."]
    #[inline(always)]
    pub const fn set_nvstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Sleep Mode Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn slm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Sleep Mode Enable."]
    #[inline(always)]
    pub const fn set_slm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Recall Trim Code."]
    #[must_use]
    #[inline(always)]
    pub const fn recall(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Recall Trim Code."]
    #[inline(always)]
    pub const fn set_recall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "HEM Control."]
    #[must_use]
    #[inline(always)]
    pub const fn hem(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "HEM Control."]
    #[inline(always)]
    pub const fn set_hem(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for RPinCtrl {
    #[inline(always)]
    fn default() -> RPinCtrl {
        RPinCtrl(0)
    }
}
impl core::fmt::Debug for RPinCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPinCtrl")
            .field("mas1", &self.mas1())
            .field("ifren", &self.ifren())
            .field("ifren1", &self.ifren1())
            .field("reden", &self.reden())
            .field("lve", &self.lve())
            .field("pv", &self.pv())
            .field("ev", &self.ev())
            .field("wipgm", &self.wipgm())
            .field("whv", &self.whv())
            .field("wmv", &self.wmv())
            .field("xe", &self.xe())
            .field("ye", &self.ye())
            .field("se", &self.se())
            .field("erase", &self.erase())
            .field("prog", &self.prog())
            .field("nvstr", &self.nvstr())
            .field("slm", &self.slm())
            .field("recall", &self.recall())
            .field("hem", &self.hem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RPinCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RPinCtrl {{ mas1: {=bool:?}, ifren: {=bool:?}, ifren1: {=bool:?}, reden: {=bool:?}, lve: {=bool:?}, pv: {=bool:?}, ev: {=bool:?}, wipgm: {=u8:?}, whv: {=u8:?}, wmv: {=u8:?}, xe: {=bool:?}, ye: {=bool:?}, se: {=bool:?}, erase: {=bool:?}, prog: {=bool:?}, nvstr: {=bool:?}, slm: {=bool:?}, recall: {=bool:?}, hem: {=bool:?} }}",
            self.mas1(),
            self.ifren(),
            self.ifren1(),
            self.reden(),
            self.lve(),
            self.pv(),
            self.ev(),
            self.wipgm(),
            self.whv(),
            self.wmv(),
            self.xe(),
            self.ye(),
            self.se(),
            self.erase(),
            self.prog(),
            self.nvstr(),
            self.slm(),
            self.recall(),
            self.hem()
        )
    }
}
#[doc = "BIST Repair 0 for Block 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair00(pub u32);
impl RRepair00 {
    #[doc = "Control Repair 0 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_0(&self) -> RRepair00Rdis00 {
        let val = (self.0 >> 0usize) & 0x01;
        RRepair00Rdis00::from_bits(val as u8)
    }
    #[doc = "Control Repair 0 in Block 0."]
    #[inline(always)]
    pub const fn set_rdis0_0(&mut self, val: RRepair00Rdis00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 0 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 0 in Block 0."]
    #[inline(always)]
    pub const fn set_radr0_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair00 {
    #[inline(always)]
    fn default() -> RRepair00 {
        RRepair00(0)
    }
}
impl core::fmt::Debug for RRepair00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair00")
            .field("rdis0_0", &self.rdis0_0())
            .field("radr0_0", &self.radr0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair00 {{ rdis0_0: {:?}, radr0_0: {=u8:?} }}",
            self.rdis0_0(),
            self.radr0_0()
        )
    }
}
#[doc = "BIST Repair 1 Block 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair01(pub u32);
impl RRepair01 {
    #[doc = "Control Repair 1 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_1(&self) -> RRepair01Rdis01 {
        let val = (self.0 >> 0usize) & 0x01;
        RRepair01Rdis01::from_bits(val as u8)
    }
    #[doc = "Control Repair 1 in Block 0."]
    #[inline(always)]
    pub const fn set_rdis0_1(&mut self, val: RRepair01Rdis01) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 1 in Block 0."]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 1 in Block 0."]
    #[inline(always)]
    pub const fn set_radr0_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair01 {
    #[inline(always)]
    fn default() -> RRepair01 {
        RRepair01(0)
    }
}
impl core::fmt::Debug for RRepair01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair01")
            .field("rdis0_1", &self.rdis0_1())
            .field("radr0_1", &self.radr0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair01 {{ rdis0_1: {:?}, radr0_1: {=u8:?} }}",
            self.rdis0_1(),
            self.radr0_1()
        )
    }
}
#[doc = "BIST Repair 0 Block 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair10(pub u32);
impl RRepair10 {
    #[doc = "Control Repair 0 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_0(&self) -> RRepair10Rdis10 {
        let val = (self.0 >> 0usize) & 0x01;
        RRepair10Rdis10::from_bits(val as u8)
    }
    #[doc = "Control Repair 0 in Block 1."]
    #[inline(always)]
    pub const fn set_rdis1_0(&mut self, val: RRepair10Rdis10) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 0 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 0 in Block 1."]
    #[inline(always)]
    pub const fn set_radr1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair10 {
    #[inline(always)]
    fn default() -> RRepair10 {
        RRepair10(0)
    }
}
impl core::fmt::Debug for RRepair10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair10")
            .field("rdis1_0", &self.rdis1_0())
            .field("radr1_0", &self.radr1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair10 {{ rdis1_0: {:?}, radr1_0: {=u8:?} }}",
            self.rdis1_0(),
            self.radr1_0()
        )
    }
}
#[doc = "BIST Repair 1 Block 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RRepair11(pub u32);
impl RRepair11 {
    #[doc = "Control Repair 1 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_1(&self) -> RRepair11Rdis11 {
        let val = (self.0 >> 0usize) & 0x01;
        RRepair11Rdis11::from_bits(val as u8)
    }
    #[doc = "Control Repair 1 in Block 1."]
    #[inline(always)]
    pub const fn set_rdis1_1(&mut self, val: RRepair11Rdis11) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "XADR for Repair 1 in Block 1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "XADR for Repair 1 in Block 1."]
    #[inline(always)]
    pub const fn set_radr1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for RRepair11 {
    #[inline(always)]
    fn default() -> RRepair11 {
        RRepair11(0)
    }
}
impl core::fmt::Debug for RRepair11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RRepair11")
            .field("rdis1_1", &self.rdis1_1())
            .field("radr1_1", &self.radr1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RRepair11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RRepair11 {{ rdis1_1: {:?}, radr1_1: {=u8:?} }}",
            self.rdis1_1(),
            self.radr1_1()
        )
    }
}
#[doc = "BIST SME WHV Setting 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmeWhv0(pub u32);
impl RSmeWhv0 {
    #[doc = "SME WHV Parameter Set 0."]
    #[must_use]
    #[inline(always)]
    pub const fn smewhv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SME WHV Parameter Set 0."]
    #[inline(always)]
    pub const fn set_smewhv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmeWhv0 {
    #[inline(always)]
    fn default() -> RSmeWhv0 {
        RSmeWhv0(0)
    }
}
impl core::fmt::Debug for RSmeWhv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmeWhv0")
            .field("smewhv0", &self.smewhv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmeWhv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmeWhv0 {{ smewhv0: {=u32:?} }}", self.smewhv0())
    }
}
#[doc = "BIST SME WHV Setting 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmeWhv1(pub u32);
impl RSmeWhv1 {
    #[doc = "SME WHV Parameter Set 1."]
    #[must_use]
    #[inline(always)]
    pub const fn smewhv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SME WHV Parameter Set 1."]
    #[inline(always)]
    pub const fn set_smewhv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmeWhv1 {
    #[inline(always)]
    fn default() -> RSmeWhv1 {
        RSmeWhv1(0)
    }
}
impl core::fmt::Debug for RSmeWhv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmeWhv1")
            .field("smewhv1", &self.smewhv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmeWhv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmeWhv1 {{ smewhv1: {=u32:?} }}", self.smewhv1())
    }
}
#[doc = "BIST SMP WHV Setting 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmpWhv0(pub u32);
impl RSmpWhv0 {
    #[doc = "SMP WHV Parameter Set 0."]
    #[must_use]
    #[inline(always)]
    pub const fn smpwhv0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMP WHV Parameter Set 0."]
    #[inline(always)]
    pub const fn set_smpwhv0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmpWhv0 {
    #[inline(always)]
    fn default() -> RSmpWhv0 {
        RSmpWhv0(0)
    }
}
impl core::fmt::Debug for RSmpWhv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmpWhv0")
            .field("smpwhv0", &self.smpwhv0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmpWhv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmpWhv0 {{ smpwhv0: {=u32:?} }}", self.smpwhv0())
    }
}
#[doc = "BIST SMP WHV Setting 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmpWhv1(pub u32);
impl RSmpWhv1 {
    #[doc = "SMP WHV Parameter Set 1."]
    #[must_use]
    #[inline(always)]
    pub const fn smpwhv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMP WHV Parameter Set 1."]
    #[inline(always)]
    pub const fn set_smpwhv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RSmpWhv1 {
    #[inline(always)]
    fn default() -> RSmpWhv1 {
        RSmpWhv1(0)
    }
}
impl core::fmt::Debug for RSmpWhv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmpWhv1")
            .field("smpwhv1", &self.smpwhv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmpWhv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmpWhv1 {{ smpwhv1: {=u32:?} }}", self.smpwhv1())
    }
}
#[doc = "BIST SMW Query Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwQuery(pub u32);
impl RSmwQuery {
    #[doc = "SMW Total Loop Count."]
    #[must_use]
    #[inline(always)]
    pub const fn smwloop(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "SMW Total Loop Count."]
    #[inline(always)]
    pub const fn set_smwloop(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SMW Last Voltage Setting."]
    #[must_use]
    #[inline(always)]
    pub const fn smwlast(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x01ff;
        val as u16
    }
    #[doc = "SMW Last Voltage Setting."]
    #[inline(always)]
    pub const fn set_smwlast(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 10usize)) | (((val as u32) & 0x01ff) << 10usize);
    }
}
impl Default for RSmwQuery {
    #[inline(always)]
    fn default() -> RSmwQuery {
        RSmwQuery(0)
    }
}
impl core::fmt::Debug for RSmwQuery {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwQuery")
            .field("smwloop", &self.smwloop())
            .field("smwlast", &self.smwlast())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwQuery {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RSmwQuery {{ smwloop: {=u16:?}, smwlast: {=u16:?} }}",
            self.smwloop(),
            self.smwlast()
        )
    }
}
#[doc = "BIST SMW Setting 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting0(pub u32);
impl RSmwSetting0 {
    #[doc = "SMW Parameter Set 0."]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 0."]
    #[inline(always)]
    pub const fn set_smwparm0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting0 {
    #[inline(always)]
    fn default() -> RSmwSetting0 {
        RSmwSetting0(0)
    }
}
impl core::fmt::Debug for RSmwSetting0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting0")
            .field("smwparm0", &self.smwparm0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting0 {{ smwparm0: {=u32:?} }}", self.smwparm0())
    }
}
#[doc = "BIST SMW Setting 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting1(pub u32);
impl RSmwSetting1 {
    #[doc = "SMW Parameter Set 1."]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 1."]
    #[inline(always)]
    pub const fn set_smwparm1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting1 {
    #[inline(always)]
    fn default() -> RSmwSetting1 {
        RSmwSetting1(0)
    }
}
impl core::fmt::Debug for RSmwSetting1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting1")
            .field("smwparm1", &self.smwparm1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting1 {{ smwparm1: {=u32:?} }}", self.smwparm1())
    }
}
#[doc = "BIST SMW Setting 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting2(pub u32);
impl RSmwSetting2 {
    #[doc = "SMW Parameter Set 2."]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 2."]
    #[inline(always)]
    pub const fn set_smwparm2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for RSmwSetting2 {
    #[inline(always)]
    fn default() -> RSmwSetting2 {
        RSmwSetting2(0)
    }
}
impl core::fmt::Debug for RSmwSetting2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting2")
            .field("smwparm2", &self.smwparm2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting2 {{ smwparm2: {=u32:?} }}", self.smwparm2())
    }
}
#[doc = "BIST SMW Setting 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RSmwSetting3(pub u32);
impl RSmwSetting3 {
    #[doc = "SMW Parameter Set 3."]
    #[must_use]
    #[inline(always)]
    pub const fn smwparm3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[doc = "SMW Parameter Set 3."]
    #[inline(always)]
    pub const fn set_smwparm3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for RSmwSetting3 {
    #[inline(always)]
    fn default() -> RSmwSetting3 {
        RSmwSetting3(0)
    }
}
impl core::fmt::Debug for RSmwSetting3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSmwSetting3")
            .field("smwparm3", &self.smwparm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RSmwSetting3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RSmwSetting3 {{ smwparm3: {=u32:?} }}", self.smwparm3())
    }
}
#[doc = "BIST Test Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTestCtrl(pub u32);
impl RTestCtrl {
    #[doc = "BIST Busy Status."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> Busy {
        let val = (self.0 >> 0usize) & 0x01;
        Busy::from_bits(val as u8)
    }
    #[doc = "BIST Busy Status."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: Busy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BIST Debug Status."]
    #[must_use]
    #[inline(always)]
    pub const fn debug(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Debug Status."]
    #[inline(always)]
    pub const fn set_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST Status 0."]
    #[must_use]
    #[inline(always)]
    pub const fn status0(&self) -> Status0 {
        let val = (self.0 >> 2usize) & 0x01;
        Status0::from_bits(val as u8)
    }
    #[doc = "BIST Status 0."]
    #[inline(always)]
    pub const fn set_status0(&mut self, val: Status0) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "BIST status 1."]
    #[must_use]
    #[inline(always)]
    pub const fn status1(&self) -> Status1 {
        let val = (self.0 >> 3usize) & 0x01;
        Status1::from_bits(val as u8)
    }
    #[doc = "BIST status 1."]
    #[inline(always)]
    pub const fn set_status1(&mut self, val: Status1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "BIST Continue Debug Run."]
    #[must_use]
    #[inline(always)]
    pub const fn debugrun(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Continue Debug Run."]
    #[inline(always)]
    pub const fn set_debugrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Run New BIST Operation."]
    #[must_use]
    #[inline(always)]
    pub const fn startrun(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Run New BIST Operation."]
    #[inline(always)]
    pub const fn set_startrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "BIST Command Index (code)."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdindex(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x03ff;
        val as u16
    }
    #[doc = "BIST Command Index (code)."]
    #[inline(always)]
    pub const fn set_cmdindex(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 6usize)) | (((val as u32) & 0x03ff) << 6usize);
    }
    #[doc = "BIST Disable IP1."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_ip1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BIST Disable IP1."]
    #[inline(always)]
    pub const fn set_disable_ip1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for RTestCtrl {
    #[inline(always)]
    fn default() -> RTestCtrl {
        RTestCtrl(0)
    }
}
impl core::fmt::Debug for RTestCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTestCtrl")
            .field("busy", &self.busy())
            .field("debug", &self.debug())
            .field("status0", &self.status0())
            .field("status1", &self.status1())
            .field("debugrun", &self.debugrun())
            .field("startrun", &self.startrun())
            .field("cmdindex", &self.cmdindex())
            .field("disable_ip1", &self.disable_ip1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTestCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTestCtrl {{ busy: {:?}, debug: {=bool:?}, status0: {:?}, status1: {:?}, debugrun: {=bool:?}, startrun: {=bool:?}, cmdindex: {=u16:?}, disable_ip1: {=bool:?} }}",
            self.busy(),
            self.debug(),
            self.status0(),
            self.status1(),
            self.debugrun(),
            self.startrun(),
            self.cmdindex(),
            self.disable_ip1()
        )
    }
}
#[doc = "BIST Test Code Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTestcode(pub u32);
impl RTestcode {
    #[doc = "Used to store test code information before running TMR-RST/TMRSET BIST command."]
    #[must_use]
    #[inline(always)]
    pub const fn testcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Used to store test code information before running TMR-RST/TMRSET BIST command."]
    #[inline(always)]
    pub const fn set_testcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RTestcode {
    #[inline(always)]
    fn default() -> RTestcode {
        RTestcode(0)
    }
}
impl core::fmt::Debug for RTestcode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTestcode")
            .field("testcode", &self.testcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTestcode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RTestcode {{ testcode: {=u8:?} }}", self.testcode())
    }
}
#[doc = "BIST Timer Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTimerCtrl(pub u32);
impl RTimerCtrl {
    #[doc = "Tnvs Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvsunit(&self) -> Tnvsunit {
        let val = (self.0 >> 0usize) & 0x07;
        Tnvsunit::from_bits(val as u8)
    }
    #[doc = "Tnvs Time Unit."]
    #[inline(always)]
    pub const fn set_tnvsunit(&mut self, val: Tnvsunit) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Tnvs Time Delay Scalar."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvsdly(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "Tnvs Time Delay Scalar."]
    #[inline(always)]
    pub const fn set_tnvsdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Tnvh Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvhunit(&self) -> Tnvhunit {
        let val = (self.0 >> 7usize) & 0x07;
        Tnvhunit::from_bits(val as u8)
    }
    #[doc = "Tnvh Time Unit."]
    #[inline(always)]
    pub const fn set_tnvhunit(&mut self, val: Tnvhunit) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val.to_bits() as u32) & 0x07) << 7usize);
    }
    #[doc = "Tnvh Time Delay Scalar."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvhdly(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "Tnvh Time Delay Scalar."]
    #[inline(always)]
    pub const fn set_tnvhdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Tpgs Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn tpgsunit(&self) -> Tpgsunit {
        let val = (self.0 >> 14usize) & 0x07;
        Tpgsunit::from_bits(val as u8)
    }
    #[doc = "Tpgs Time Unit."]
    #[inline(always)]
    pub const fn set_tpgsunit(&mut self, val: Tpgsunit) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val.to_bits() as u32) & 0x07) << 14usize);
    }
    #[doc = "Tpgs Time Delay Scalar."]
    #[must_use]
    #[inline(always)]
    pub const fn tpgsdly(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[doc = "Tpgs Time Delay Scalar."]
    #[inline(always)]
    pub const fn set_tpgsdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
    #[doc = "Trcv Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn trcvunit(&self) -> Trcvunit {
        let val = (self.0 >> 21usize) & 0x07;
        Trcvunit::from_bits(val as u8)
    }
    #[doc = "Trcv Time Unit."]
    #[inline(always)]
    pub const fn set_trcvunit(&mut self, val: Trcvunit) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Trcv Time Delay Scalar."]
    #[must_use]
    #[inline(always)]
    pub const fn trcvdly(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Trcv Time Delay Scalar."]
    #[inline(always)]
    pub const fn set_trcvdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Tlvs Time Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsunit(&self) -> Tlvsunit {
        let val = (self.0 >> 28usize) & 0x07;
        Tlvsunit::from_bits(val as u8)
    }
    #[doc = "Tlvs Time Unit."]
    #[inline(always)]
    pub const fn set_tlvsunit(&mut self, val: Tlvsunit) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
    }
    #[doc = "Tlvs Time Delay Scalar Low."]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsdly_l(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Tlvs Time Delay Scalar Low."]
    #[inline(always)]
    pub const fn set_tlvsdly_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RTimerCtrl {
    #[inline(always)]
    fn default() -> RTimerCtrl {
        RTimerCtrl(0)
    }
}
impl core::fmt::Debug for RTimerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTimerCtrl")
            .field("tnvsunit", &self.tnvsunit())
            .field("tnvsdly", &self.tnvsdly())
            .field("tnvhunit", &self.tnvhunit())
            .field("tnvhdly", &self.tnvhdly())
            .field("tpgsunit", &self.tpgsunit())
            .field("tpgsdly", &self.tpgsdly())
            .field("trcvunit", &self.trcvunit())
            .field("trcvdly", &self.trcvdly())
            .field("tlvsunit", &self.tlvsunit())
            .field("tlvsdly_l", &self.tlvsdly_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTimerCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RTimerCtrl {{ tnvsunit: {:?}, tnvsdly: {=u8:?}, tnvhunit: {:?}, tnvhdly: {=u8:?}, tpgsunit: {:?}, tpgsdly: {=u8:?}, trcvunit: {:?}, trcvdly: {=u8:?}, tlvsunit: {:?}, tlvsdly_l: {=bool:?} }}",
            self.tnvsunit(),
            self.tnvsdly(),
            self.tnvhunit(),
            self.tnvhdly(),
            self.tpgsunit(),
            self.tpgsdly(),
            self.trcvunit(),
            self.trcvdly(),
            self.tlvsunit(),
            self.tlvsdly_l()
        )
    }
}
#[doc = "BIST Timer Control Extension Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RTimerCtrlEx(pub u32);
impl RTimerCtrlEx {
    #[doc = "Tlvs Time Delay Scalar High."]
    #[must_use]
    #[inline(always)]
    pub const fn tlvsdly_h(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Tlvs Time Delay Scalar High."]
    #[inline(always)]
    pub const fn set_tlvsdly_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for RTimerCtrlEx {
    #[inline(always)]
    fn default() -> RTimerCtrlEx {
        RTimerCtrlEx(0)
    }
}
impl core::fmt::Debug for RTimerCtrlEx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTimerCtrlEx")
            .field("tlvsdly_h", &self.tlvsdly_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RTimerCtrlEx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RTimerCtrlEx {{ tlvsdly_h: {=u8:?} }}", self.tlvsdly_h())
    }
}
#[doc = "Read Data 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData0(pub u32);
impl RdData0 {
    #[doc = "Read Data 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 0."]
    #[inline(always)]
    pub const fn set_rd_data0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData0 {
    #[inline(always)]
    fn default() -> RdData0 {
        RdData0(0)
    }
}
impl core::fmt::Debug for RdData0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData0")
            .field("rd_data0", &self.rd_data0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData0 {{ rd_data0: {=u32:?} }}", self.rd_data0())
    }
}
#[doc = "Read Data 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData1(pub u32);
impl RdData1 {
    #[doc = "Read Data 1."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 1."]
    #[inline(always)]
    pub const fn set_rd_data1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData1 {
    #[inline(always)]
    fn default() -> RdData1 {
        RdData1(0)
    }
}
impl core::fmt::Debug for RdData1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData1")
            .field("rd_data1", &self.rd_data1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData1 {{ rd_data1: {=u32:?} }}", self.rd_data1())
    }
}
#[doc = "Read Data 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData2(pub u32);
impl RdData2 {
    #[doc = "Read Data 2."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 2."]
    #[inline(always)]
    pub const fn set_rd_data2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData2 {
    #[inline(always)]
    fn default() -> RdData2 {
        RdData2(0)
    }
}
impl core::fmt::Debug for RdData2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData2")
            .field("rd_data2", &self.rd_data2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData2 {{ rd_data2: {=u32:?} }}", self.rd_data2())
    }
}
#[doc = "Read Data 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdData3(pub u32);
impl RdData3 {
    #[doc = "Read Data 3."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_data3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read Data 3."]
    #[inline(always)]
    pub const fn set_rd_data3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RdData3 {
    #[inline(always)]
    fn default() -> RdData3 {
        RdData3(0)
    }
}
impl core::fmt::Debug for RdData3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdData3")
            .field("rd_data3", &self.rd_data3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdData3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RdData3 {{ rd_data3: {=u32:?} }}", self.rd_data3())
    }
}
#[doc = "Read Path Control and Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdPathCtrlStatus(pub u32);
impl RdPathCtrlStatus {
    #[doc = "Read Capture Clock Periods."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_capt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read Capture Clock Periods."]
    #[inline(always)]
    pub const fn set_rd_capt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "SE Clock Periods."]
    #[must_use]
    #[inline(always)]
    pub const fn se_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SE Clock Periods."]
    #[inline(always)]
    pub const fn set_se_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "ECC Decoder Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ecc_enableb(&self) -> EccEnableb {
        let val = (self.0 >> 16usize) & 0x01;
        EccEnableb::from_bits(val as u8)
    }
    #[doc = "ECC Decoder Control."]
    #[inline(always)]
    pub const fn set_ecc_enableb(&mut self, val: EccEnableb) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "MISR Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn misr_en(&self) -> MisrEn {
        let val = (self.0 >> 17usize) & 0x01;
        MisrEn::from_bits(val as u8)
    }
    #[doc = "MISR Enable."]
    #[inline(always)]
    pub const fn set_misr_en(&mut self, val: MisrEn) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Copy Parity Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cpy_par_en(&self) -> CpyParEn {
        let val = (self.0 >> 18usize) & 0x01;
        CpyParEn::from_bits(val as u8)
    }
    #[doc = "Copy Parity Enable."]
    #[inline(always)]
    pub const fn set_cpy_par_en(&mut self, val: CpyParEn) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "BIST Mux to SMW."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_mux_to_smw(&self) -> BistMuxToSmw {
        let val = (self.0 >> 19usize) & 0x01;
        BistMuxToSmw::from_bits(val as u8)
    }
    #[doc = "BIST Mux to SMW."]
    #[inline(always)]
    pub const fn set_bist_mux_to_smw(&mut self, val: BistMuxToSmw) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Multi-Cycle Address Setup Time."]
    #[must_use]
    #[inline(always)]
    pub const fn ad_set(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Multi-Cycle Address Setup Time."]
    #[inline(always)]
    pub const fn set_ad_set(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Write Path Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_path_en(&self) -> WrPathEn {
        let val = (self.0 >> 24usize) & 0x01;
        WrPathEn::from_bits(val as u8)
    }
    #[doc = "Write Path Enable."]
    #[inline(always)]
    pub const fn set_wr_path_en(&mut self, val: WrPathEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Write Path ECC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_path_ecc_en(&self) -> WrPathEccEn {
        let val = (self.0 >> 25usize) & 0x01;
        WrPathEccEn::from_bits(val as u8)
    }
    #[doc = "Write Path ECC Enable."]
    #[inline(always)]
    pub const fn set_wr_path_ecc_en(&mut self, val: WrPathEccEn) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Double-Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dberr_reg(&self) -> DberrReg {
        let val = (self.0 >> 26usize) & 0x01;
        DberrReg::from_bits(val as u8)
    }
    #[doc = "Double-Bit Error."]
    #[inline(always)]
    pub const fn set_dberr_reg(&mut self, val: DberrReg) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Single-Bit Error."]
    #[must_use]
    #[inline(always)]
    pub const fn sberr_reg(&self) -> SberrReg {
        let val = (self.0 >> 27usize) & 0x01;
        SberrReg::from_bits(val as u8)
    }
    #[doc = "Single-Bit Error."]
    #[inline(always)]
    pub const fn set_sberr_reg(&mut self, val: SberrReg) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Copy Phrase Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cpy_phrase_en(&self) -> CpyPhraseEn {
        let val = (self.0 >> 28usize) & 0x01;
        CpyPhraseEn::from_bits(val as u8)
    }
    #[doc = "Copy Phrase Enable."]
    #[inline(always)]
    pub const fn set_cpy_phrase_en(&mut self, val: CpyPhraseEn) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SMW_ARRAY1_SMW0_SEL."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_array1_smw0_sel(&self) -> SmwArray1Smw0Sel {
        let val = (self.0 >> 29usize) & 0x01;
        SmwArray1Smw0Sel::from_bits(val as u8)
    }
    #[doc = "SMW_ARRAY1_SMW0_SEL."]
    #[inline(always)]
    pub const fn set_smw_array1_smw0_sel(&mut self, val: SmwArray1Smw0Sel) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "BIST ECC Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_ecc_en(&self) -> BistEccEn {
        let val = (self.0 >> 30usize) & 0x01;
        BistEccEn::from_bits(val as u8)
    }
    #[doc = "BIST ECC Enable."]
    #[inline(always)]
    pub const fn set_bist_ecc_en(&mut self, val: BistEccEn) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Last Read."]
    #[must_use]
    #[inline(always)]
    pub const fn last_read(&self) -> LastRead {
        let val = (self.0 >> 31usize) & 0x01;
        LastRead::from_bits(val as u8)
    }
    #[doc = "Last Read."]
    #[inline(always)]
    pub const fn set_last_read(&mut self, val: LastRead) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for RdPathCtrlStatus {
    #[inline(always)]
    fn default() -> RdPathCtrlStatus {
        RdPathCtrlStatus(0)
    }
}
impl core::fmt::Debug for RdPathCtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RdPathCtrlStatus")
            .field("rd_capt", &self.rd_capt())
            .field("se_size", &self.se_size())
            .field("ecc_enableb", &self.ecc_enableb())
            .field("misr_en", &self.misr_en())
            .field("cpy_par_en", &self.cpy_par_en())
            .field("bist_mux_to_smw", &self.bist_mux_to_smw())
            .field("ad_set", &self.ad_set())
            .field("wr_path_en", &self.wr_path_en())
            .field("wr_path_ecc_en", &self.wr_path_ecc_en())
            .field("dberr_reg", &self.dberr_reg())
            .field("sberr_reg", &self.sberr_reg())
            .field("cpy_phrase_en", &self.cpy_phrase_en())
            .field("smw_array1_smw0_sel", &self.smw_array1_smw0_sel())
            .field("bist_ecc_en", &self.bist_ecc_en())
            .field("last_read", &self.last_read())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RdPathCtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RdPathCtrlStatus {{ rd_capt: {=u8:?}, se_size: {=u8:?}, ecc_enableb: {:?}, misr_en: {:?}, cpy_par_en: {:?}, bist_mux_to_smw: {:?}, ad_set: {=u8:?}, wr_path_en: {:?}, wr_path_ecc_en: {:?}, dberr_reg: {:?}, sberr_reg: {:?}, cpy_phrase_en: {:?}, smw_array1_smw0_sel: {:?}, bist_ecc_en: {:?}, last_read: {:?} }}",
            self.rd_capt(),
            self.se_size(),
            self.ecc_enableb(),
            self.misr_en(),
            self.cpy_par_en(),
            self.bist_mux_to_smw(),
            self.ad_set(),
            self.wr_path_en(),
            self.wr_path_ecc_en(),
            self.dberr_reg(),
            self.sberr_reg(),
            self.cpy_phrase_en(),
            self.smw_array1_smw0_sel(),
            self.bist_ecc_en(),
            self.last_read()
        )
    }
}
#[doc = "FMU Repair 0 Block 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair00(pub u32);
impl Repair00 {
    #[doc = "RDIS0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_0(&self) -> Repair00Rdis00 {
        let val = (self.0 >> 0usize) & 0x01;
        Repair00Rdis00::from_bits(val as u8)
    }
    #[doc = "RDIS0_0."]
    #[inline(always)]
    pub const fn set_rdis0_0(&mut self, val: Repair00Rdis00) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR0_0."]
    #[inline(always)]
    pub const fn set_radr0_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair00 {
    #[inline(always)]
    fn default() -> Repair00 {
        Repair00(0)
    }
}
impl core::fmt::Debug for Repair00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair00")
            .field("rdis0_0", &self.rdis0_0())
            .field("radr0_0", &self.radr0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair00 {{ rdis0_0: {:?}, radr0_0: {=u8:?} }}",
            self.rdis0_0(),
            self.radr0_0()
        )
    }
}
#[doc = "FMU Repair 1 Block 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair01(pub u32);
impl Repair01 {
    #[doc = "RDIS0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis0_1(&self) -> Repair01Rdis01 {
        let val = (self.0 >> 0usize) & 0x01;
        Repair01Rdis01::from_bits(val as u8)
    }
    #[doc = "RDIS0_1."]
    #[inline(always)]
    pub const fn set_rdis0_1(&mut self, val: Repair01Rdis01) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr0_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR0_1."]
    #[inline(always)]
    pub const fn set_radr0_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair01 {
    #[inline(always)]
    fn default() -> Repair01 {
        Repair01(0)
    }
}
impl core::fmt::Debug for Repair01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair01")
            .field("rdis0_1", &self.rdis0_1())
            .field("radr0_1", &self.radr0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair01 {{ rdis0_1: {:?}, radr0_1: {=u8:?} }}",
            self.rdis0_1(),
            self.radr0_1()
        )
    }
}
#[doc = "FMU Repair 0 Block 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair10(pub u32);
impl Repair10 {
    #[doc = "RDIS1_0."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_0(&self) -> Repair10Rdis10 {
        let val = (self.0 >> 0usize) & 0x01;
        Repair10Rdis10::from_bits(val as u8)
    }
    #[doc = "RDIS1_0."]
    #[inline(always)]
    pub const fn set_rdis1_0(&mut self, val: Repair10Rdis10) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR1_0."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR1_0."]
    #[inline(always)]
    pub const fn set_radr1_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair10 {
    #[inline(always)]
    fn default() -> Repair10 {
        Repair10(0)
    }
}
impl core::fmt::Debug for Repair10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair10")
            .field("rdis1_0", &self.rdis1_0())
            .field("radr1_0", &self.radr1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair10 {{ rdis1_0: {:?}, radr1_0: {=u8:?} }}",
            self.rdis1_0(),
            self.radr1_0()
        )
    }
}
#[doc = "FMU Repair 1 Block 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Repair11(pub u32);
impl Repair11 {
    #[doc = "RDIS1_1."]
    #[must_use]
    #[inline(always)]
    pub const fn rdis1_1(&self) -> Repair11Rdis11 {
        let val = (self.0 >> 0usize) & 0x01;
        Repair11Rdis11::from_bits(val as u8)
    }
    #[doc = "RDIS1_1."]
    #[inline(always)]
    pub const fn set_rdis1_1(&mut self, val: Repair11Rdis11) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RADR1_1."]
    #[must_use]
    #[inline(always)]
    pub const fn radr1_1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[doc = "RADR1_1."]
    #[inline(always)]
    pub const fn set_radr1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
}
impl Default for Repair11 {
    #[inline(always)]
    fn default() -> Repair11 {
        Repair11(0)
    }
}
impl core::fmt::Debug for Repair11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Repair11")
            .field("rdis1_1", &self.rdis1_1())
            .field("radr1_1", &self.radr1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Repair11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Repair11 {{ rdis1_1: {:?}, radr1_1: {=u8:?} }}",
            self.rdis1_1(),
            self.radr1_1()
        )
    }
}
#[doc = "FMU Initialization Tracking Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ResetStatus(pub u32);
impl ResetStatus {
    #[doc = "Array Trim Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn ary_trim_done(&self) -> AryTrimDone {
        let val = (self.0 >> 0usize) & 0x01;
        AryTrimDone::from_bits(val as u8)
    }
    #[doc = "Array Trim Complete."]
    #[inline(always)]
    pub const fn set_ary_trim_done(&mut self, val: AryTrimDone) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the FMU parameters."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_parm_en(&self) -> FmuParmEn {
        let val = (self.0 >> 1usize) & 0x01;
        FmuParmEn::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the FMU parameters."]
    #[inline(always)]
    pub const fn set_fmu_parm_en(&mut self, val: FmuParmEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FMU Register Load Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_parm_done(&self) -> FmuParmDone {
        let val = (self.0 >> 2usize) & 0x01;
        FmuParmDone::from_bits(val as u8)
    }
    #[doc = "FMU Register Load Complete."]
    #[inline(always)]
    pub const fn set_fmu_parm_done(&mut self, val: FmuParmDone) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the SoC trim settings."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_en(&self) -> SocTrimEn {
        let val = (self.0 >> 3usize) & 0x01;
        SocTrimEn::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check to enable loading of the SoC trim settings."]
    #[inline(always)]
    pub const fn set_soc_trim_en(&mut self, val: SocTrimEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_ecc(&self) -> SocTrimEcc {
        let val = (self.0 >> 4usize) & 0x01;
        SocTrimEcc::from_bits(val as u8)
    }
    #[doc = "Status of the C0DE_C0DEh check for enabling ECC decoder during reads of SoC trim settings."]
    #[inline(always)]
    pub const fn set_soc_trim_ecc(&mut self, val: SocTrimEcc) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SoC Trim Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_done(&self) -> SocTrimDone {
        let val = (self.0 >> 5usize) & 0x01;
        SocTrimDone::from_bits(val as u8)
    }
    #[doc = "SoC Trim Complete."]
    #[inline(always)]
    pub const fn set_soc_trim_done(&mut self, val: SocTrimDone) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Array Repair Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn rpr_done(&self) -> RprDone {
        let val = (self.0 >> 6usize) & 0x01;
        RprDone::from_bits(val as u8)
    }
    #[doc = "Array Repair Complete."]
    #[inline(always)]
    pub const fn set_rpr_done(&mut self, val: RprDone) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Initialization Done."]
    #[must_use]
    #[inline(always)]
    pub const fn init_done(&self) -> InitDone {
        let val = (self.0 >> 7usize) & 0x01;
        InitDone::from_bits(val as u8)
    }
    #[doc = "Initialization Done."]
    #[inline(always)]
    pub const fn set_init_done(&mut self, val: InitDone) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "ECC Single Fault during Reset Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_sf_err(&self) -> RstSfErr {
        let val = (self.0 >> 8usize) & 0x01;
        RstSfErr::from_bits(val as u8)
    }
    #[doc = "ECC Single Fault during Reset Recovery."]
    #[inline(always)]
    pub const fn set_rst_sf_err(&mut self, val: RstSfErr) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "ECC Double Fault during Reset Recovery."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_df_err(&self) -> RstDfErr {
        let val = (self.0 >> 9usize) & 0x01;
        RstDfErr::from_bits(val as u8)
    }
    #[doc = "ECC Double Fault during Reset Recovery."]
    #[inline(always)]
    pub const fn set_rst_df_err(&mut self, val: RstDfErr) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "ECC Double Fault during load of SoC Trim phrases."]
    #[must_use]
    #[inline(always)]
    pub const fn soc_trim_df_err(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0xff;
        val as u8
    }
    #[doc = "ECC Double Fault during load of SoC Trim phrases."]
    #[inline(always)]
    pub const fn set_soc_trim_df_err(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
    }
    #[doc = "Reset Patch Required."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_patch_ld(&self) -> RstPatchLd {
        let val = (self.0 >> 18usize) & 0x01;
        RstPatchLd::from_bits(val as u8)
    }
    #[doc = "Reset Patch Required."]
    #[inline(always)]
    pub const fn set_rst_patch_ld(&mut self, val: RstPatchLd) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Recall Data Mismatch."]
    #[must_use]
    #[inline(always)]
    pub const fn recall_data_mismatch(&self) -> RecallDataMismatch {
        let val = (self.0 >> 19usize) & 0x01;
        RecallDataMismatch::from_bits(val as u8)
    }
    #[doc = "Recall Data Mismatch."]
    #[inline(always)]
    pub const fn set_recall_data_mismatch(&mut self, val: RecallDataMismatch) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for ResetStatus {
    #[inline(always)]
    fn default() -> ResetStatus {
        ResetStatus(0)
    }
}
impl core::fmt::Debug for ResetStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ResetStatus")
            .field("ary_trim_done", &self.ary_trim_done())
            .field("fmu_parm_en", &self.fmu_parm_en())
            .field("fmu_parm_done", &self.fmu_parm_done())
            .field("soc_trim_en", &self.soc_trim_en())
            .field("soc_trim_ecc", &self.soc_trim_ecc())
            .field("soc_trim_done", &self.soc_trim_done())
            .field("rpr_done", &self.rpr_done())
            .field("init_done", &self.init_done())
            .field("rst_sf_err", &self.rst_sf_err())
            .field("rst_df_err", &self.rst_df_err())
            .field("soc_trim_df_err", &self.soc_trim_df_err())
            .field("rst_patch_ld", &self.rst_patch_ld())
            .field("recall_data_mismatch", &self.recall_data_mismatch())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ResetStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ResetStatus {{ ary_trim_done: {:?}, fmu_parm_en: {:?}, fmu_parm_done: {:?}, soc_trim_en: {:?}, soc_trim_ecc: {:?}, soc_trim_done: {:?}, rpr_done: {:?}, init_done: {:?}, rst_sf_err: {:?}, rst_df_err: {:?}, soc_trim_df_err: {=u8:?}, rst_patch_ld: {:?}, recall_data_mismatch: {:?} }}",
            self.ary_trim_done(),
            self.fmu_parm_en(),
            self.fmu_parm_done(),
            self.soc_trim_en(),
            self.soc_trim_ecc(),
            self.soc_trim_done(),
            self.rpr_done(),
            self.init_done(),
            self.rst_sf_err(),
            self.rst_df_err(),
            self.soc_trim_df_err(),
            self.rst_patch_ld(),
            self.recall_data_mismatch()
        )
    }
}
#[doc = "SMW Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwAddr(pub u32);
impl SmwAddr {
    #[doc = "SMW Address."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW Address."]
    #[inline(always)]
    pub const fn set_smw_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwAddr {
    #[inline(always)]
    fn default() -> SmwAddr {
        SmwAddr(0)
    }
}
impl core::fmt::Debug for SmwAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwAddr")
            .field("smw_addr", &self.smw_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwAddr {{ smw_addr: {=u32:?} }}", self.smw_addr())
    }
}
#[doc = "SMW Command and Wait Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwCmdWait(pub u32);
impl SmwCmdWait {
    #[doc = "SMW Command."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> Cmd {
        let val = (self.0 >> 0usize) & 0x07;
        Cmd::from_bits(val as u8)
    }
    #[doc = "SMW Command."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: Cmd) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SMW Wait Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_en(&self) -> WaitEn {
        let val = (self.0 >> 3usize) & 0x01;
        WaitEn::from_bits(val as u8)
    }
    #[doc = "SMW Wait Enable."]
    #[inline(always)]
    pub const fn set_wait_en(&mut self, val: WaitEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "SMW Wait Auto Set."]
    #[must_use]
    #[inline(always)]
    pub const fn wait_auto_set(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SMW Wait Auto Set."]
    #[inline(always)]
    pub const fn set_wait_auto_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for SmwCmdWait {
    #[inline(always)]
    fn default() -> SmwCmdWait {
        SmwCmdWait(0)
    }
}
impl core::fmt::Debug for SmwCmdWait {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwCmdWait")
            .field("cmd", &self.cmd())
            .field("wait_en", &self.wait_en())
            .field("wait_auto_set", &self.wait_auto_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwCmdWait {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwCmdWait {{ cmd: {:?}, wait_en: {:?}, wait_auto_set: {=bool:?} }}",
            self.cmd(),
            self.wait_en(),
            self.wait_auto_set()
        )
    }
}
#[doc = "SMW DIN 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin0(pub u32);
impl SmwDin0 {
    #[doc = "SMW DIN 0."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 0."]
    #[inline(always)]
    pub const fn set_smw_din0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin0 {
    #[inline(always)]
    fn default() -> SmwDin0 {
        SmwDin0(0)
    }
}
impl core::fmt::Debug for SmwDin0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin0")
            .field("smw_din0", &self.smw_din0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin0 {{ smw_din0: {=u32:?} }}", self.smw_din0())
    }
}
#[doc = "SMW DIN 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin1(pub u32);
impl SmwDin1 {
    #[doc = "SMW DIN 1."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 1."]
    #[inline(always)]
    pub const fn set_smw_din1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin1 {
    #[inline(always)]
    fn default() -> SmwDin1 {
        SmwDin1(0)
    }
}
impl core::fmt::Debug for SmwDin1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin1")
            .field("smw_din1", &self.smw_din1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin1 {{ smw_din1: {=u32:?} }}", self.smw_din1())
    }
}
#[doc = "SMW DIN 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin2(pub u32);
impl SmwDin2 {
    #[doc = "SMW DIN 2."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 2."]
    #[inline(always)]
    pub const fn set_smw_din2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin2 {
    #[inline(always)]
    fn default() -> SmwDin2 {
        SmwDin2(0)
    }
}
impl core::fmt::Debug for SmwDin2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin2")
            .field("smw_din2", &self.smw_din2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin2 {{ smw_din2: {=u32:?} }}", self.smw_din2())
    }
}
#[doc = "SMW DIN 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwDin3(pub u32);
impl SmwDin3 {
    #[doc = "SMW DIN 3."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_din3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "SMW DIN 3."]
    #[inline(always)]
    pub const fn set_smw_din3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwDin3 {
    #[inline(always)]
    fn default() -> SmwDin3 {
        SmwDin3(0)
    }
}
impl core::fmt::Debug for SmwDin3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwDin3")
            .field("smw_din3", &self.smw_din3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwDin3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SmwDin3 {{ smw_din3: {=u32:?} }}", self.smw_din3())
    }
}
#[doc = "SMW HB Signals Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwHbSignals(pub u32);
impl SmwHbSignals {
    #[doc = "SMW Region Select."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_array(&self) -> SmwArray {
        let val = (self.0 >> 0usize) & 0x07;
        SmwArray::from_bits(val as u8)
    }
    #[doc = "SMW Region Select."]
    #[inline(always)]
    pub const fn set_smw_array(&mut self, val: SmwArray) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "IFR1 Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn user_ifren1(&self) -> UserIfren1 {
        let val = (self.0 >> 3usize) & 0x01;
        UserIfren1::from_bits(val as u8)
    }
    #[doc = "IFR1 Enable."]
    #[inline(always)]
    pub const fn set_user_ifren1(&mut self, val: UserIfren1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Program Verify."]
    #[must_use]
    #[inline(always)]
    pub const fn user_pv(&self) -> UserPv {
        let val = (self.0 >> 4usize) & 0x01;
        UserPv::from_bits(val as u8)
    }
    #[doc = "Program Verify."]
    #[inline(always)]
    pub const fn set_user_pv(&mut self, val: UserPv) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Erase Verify."]
    #[must_use]
    #[inline(always)]
    pub const fn user_ev(&self) -> UserEv {
        let val = (self.0 >> 5usize) & 0x01;
        UserEv::from_bits(val as u8)
    }
    #[doc = "Erase Verify."]
    #[inline(always)]
    pub const fn set_user_ev(&mut self, val: UserEv) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IFR Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn user_ifren(&self) -> UserIfren {
        let val = (self.0 >> 6usize) & 0x01;
        UserIfren::from_bits(val as u8)
    }
    #[doc = "IFR Enable."]
    #[inline(always)]
    pub const fn set_user_ifren(&mut self, val: UserIfren) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Repair Read Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn user_reden(&self) -> UserReden {
        let val = (self.0 >> 7usize) & 0x01;
        UserReden::from_bits(val as u8)
    }
    #[doc = "Repair Read Enable."]
    #[inline(always)]
    pub const fn set_user_reden(&mut self, val: UserReden) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "High Endurance Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn user_hem(&self) -> UserHem {
        let val = (self.0 >> 8usize) & 0x01;
        UserHem::from_bits(val as u8)
    }
    #[doc = "High Endurance Enable."]
    #[inline(always)]
    pub const fn set_user_hem(&mut self, val: UserHem) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for SmwHbSignals {
    #[inline(always)]
    fn default() -> SmwHbSignals {
        SmwHbSignals(0)
    }
}
impl core::fmt::Debug for SmwHbSignals {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwHbSignals")
            .field("smw_array", &self.smw_array())
            .field("user_ifren1", &self.user_ifren1())
            .field("user_pv", &self.user_pv())
            .field("user_ev", &self.user_ev())
            .field("user_ifren", &self.user_ifren())
            .field("user_reden", &self.user_reden())
            .field("user_hem", &self.user_hem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwHbSignals {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwHbSignals {{ smw_array: {:?}, user_ifren1: {:?}, user_pv: {:?}, user_ev: {:?}, user_ifren: {:?}, user_reden: {:?}, user_hem: {:?} }}",
            self.smw_array(),
            self.user_ifren1(),
            self.user_pv(),
            self.user_ev(),
            self.user_ifren(),
            self.user_reden(),
            self.user_hem()
        )
    }
}
#[doc = "SMW Setting Option 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption0(pub u32);
impl SmwSettingOption0 {
    #[doc = "Medium Voltage Level Select Initial."]
    #[must_use]
    #[inline(always)]
    pub const fn mv_init(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level Select Initial."]
    #[inline(always)]
    pub const fn set_mv_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[doc = "Medium Voltage Level Select Final."]
    #[must_use]
    #[inline(always)]
    pub const fn mv_end(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[doc = "Medium Voltage Level Select Final."]
    #[inline(always)]
    pub const fn set_mv_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[doc = "Medium Voltage Control Misc."]
    #[must_use]
    #[inline(always)]
    pub const fn mv_misc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Medium Voltage Control Misc."]
    #[inline(always)]
    pub const fn set_mv_misc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Program Current Control Initial."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_init(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current Control Initial."]
    #[inline(always)]
    pub const fn set_ipgm_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Program Current Control Final."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_end(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Program Current Control Final."]
    #[inline(always)]
    pub const fn set_ipgm_end(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Program Current Control Misc."]
    #[must_use]
    #[inline(always)]
    pub const fn ipgm_misc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Program Current Control Misc."]
    #[inline(always)]
    pub const fn set_ipgm_misc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for SmwSettingOption0 {
    #[inline(always)]
    fn default() -> SmwSettingOption0 {
        SmwSettingOption0(0)
    }
}
impl core::fmt::Debug for SmwSettingOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption0")
            .field("mv_init", &self.mv_init())
            .field("mv_end", &self.mv_end())
            .field("mv_misc", &self.mv_misc())
            .field("ipgm_init", &self.ipgm_init())
            .field("ipgm_end", &self.ipgm_end())
            .field("ipgm_misc", &self.ipgm_misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption0 {{ mv_init: {=u8:?}, mv_end: {=u8:?}, mv_misc: {=u8:?}, ipgm_init: {=u8:?}, ipgm_end: {=u8:?}, ipgm_misc: {=u8:?} }}",
            self.mv_init(),
            self.mv_end(),
            self.mv_misc(),
            self.ipgm_init(),
            self.ipgm_end(),
            self.ipgm_misc()
        )
    }
}
#[doc = "SMW Setting Option 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption1(pub u32);
impl SmwSettingOption1 {
    #[doc = "Ters Control."]
    #[must_use]
    #[inline(always)]
    pub const fn ters_ctrl0(&self) -> TersCtrl0 {
        let val = (self.0 >> 0usize) & 0x07;
        TersCtrl0::from_bits(val as u8)
    }
    #[doc = "Ters Control."]
    #[inline(always)]
    pub const fn set_ters_ctrl0(&mut self, val: TersCtrl0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Tpgm Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tpgm_ctrl(&self) -> TpgmCtrl {
        let val = (self.0 >> 3usize) & 0x03;
        TpgmCtrl::from_bits(val as u8)
    }
    #[doc = "Tpgm Control."]
    #[inline(always)]
    pub const fn set_tpgm_ctrl(&mut self, val: TpgmCtrl) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Tnvs Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvs_ctrl(&self) -> TnvsCtrl {
        let val = (self.0 >> 5usize) & 0x07;
        TnvsCtrl::from_bits(val as u8)
    }
    #[doc = "Tnvs Control."]
    #[inline(always)]
    pub const fn set_tnvs_ctrl(&mut self, val: TnvsCtrl) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Tnvh Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tnvh_ctrl(&self) -> TnvhCtrl {
        let val = (self.0 >> 8usize) & 0x07;
        TnvhCtrl::from_bits(val as u8)
    }
    #[doc = "Tnvh Control."]
    #[inline(always)]
    pub const fn set_tnvh_ctrl(&mut self, val: TnvhCtrl) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Tpgs Control."]
    #[must_use]
    #[inline(always)]
    pub const fn tpgs_ctrl(&self) -> TpgsCtrl {
        let val = (self.0 >> 11usize) & 0x07;
        TpgsCtrl::from_bits(val as u8)
    }
    #[doc = "Tpgs Control."]
    #[inline(always)]
    pub const fn set_tpgs_ctrl(&mut self, val: TpgsCtrl) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
    }
    #[doc = "Number of Erase Shots."]
    #[must_use]
    #[inline(always)]
    pub const fn max_erase(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x01ff;
        val as u16
    }
    #[doc = "Number of Erase Shots."]
    #[inline(always)]
    pub const fn set_max_erase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 14usize)) | (((val as u32) & 0x01ff) << 14usize);
    }
    #[doc = "Number of Program Shots."]
    #[must_use]
    #[inline(always)]
    pub const fn max_prog(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of Program Shots."]
    #[inline(always)]
    pub const fn set_max_prog(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
}
impl Default for SmwSettingOption1 {
    #[inline(always)]
    fn default() -> SmwSettingOption1 {
        SmwSettingOption1(0)
    }
}
impl core::fmt::Debug for SmwSettingOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption1")
            .field("ters_ctrl0", &self.ters_ctrl0())
            .field("tpgm_ctrl", &self.tpgm_ctrl())
            .field("tnvs_ctrl", &self.tnvs_ctrl())
            .field("tnvh_ctrl", &self.tnvh_ctrl())
            .field("tpgs_ctrl", &self.tpgs_ctrl())
            .field("max_erase", &self.max_erase())
            .field("max_prog", &self.max_prog())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption1 {{ ters_ctrl0: {:?}, tpgm_ctrl: {:?}, tnvs_ctrl: {:?}, tnvh_ctrl: {:?}, tpgs_ctrl: {:?}, max_erase: {=u16:?}, max_prog: {=u8:?} }}",
            self.ters_ctrl0(),
            self.tpgm_ctrl(),
            self.tnvs_ctrl(),
            self.tnvh_ctrl(),
            self.tpgs_ctrl(),
            self.max_erase(),
            self.max_prog()
        )
    }
}
#[doc = "SMW Setting Option 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption2(pub u32);
impl SmwSettingOption2 {
    #[doc = "Thvs control."]
    #[must_use]
    #[inline(always)]
    pub const fn thvs_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Thvs control."]
    #[inline(always)]
    pub const fn set_thvs_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Trcv Control."]
    #[must_use]
    #[inline(always)]
    pub const fn trcv_ctrl(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Trcv Control."]
    #[inline(always)]
    pub const fn set_trcv_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "Number of Post Shots for SME."]
    #[must_use]
    #[inline(always)]
    pub const fn xtra_ers(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Post Shots for SME."]
    #[inline(always)]
    pub const fn set_xtra_ers(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Number of Post Shots for SMP."]
    #[must_use]
    #[inline(always)]
    pub const fn xtra_pgm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Post Shots for SMP."]
    #[inline(always)]
    pub const fn set_xtra_pgm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "WHV Counter."]
    #[must_use]
    #[inline(always)]
    pub const fn whv_cntr(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0xff;
        val as u8
    }
    #[doc = "WHV Counter."]
    #[inline(always)]
    pub const fn set_whv_cntr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 10usize)) | (((val as u32) & 0xff) << 10usize);
    }
    #[doc = "Post Ters Time."]
    #[must_use]
    #[inline(always)]
    pub const fn post_ters(&self) -> PostTers {
        let val = (self.0 >> 18usize) & 0x07;
        PostTers::from_bits(val as u8)
    }
    #[doc = "Post Ters Time."]
    #[inline(always)]
    pub const fn set_post_ters(&mut self, val: PostTers) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Post Tpgm Time."]
    #[must_use]
    #[inline(always)]
    pub const fn post_tpgm(&self) -> PostTpgm {
        let val = (self.0 >> 21usize) & 0x03;
        PostTpgm::from_bits(val as u8)
    }
    #[doc = "Post Tpgm Time."]
    #[inline(always)]
    pub const fn set_post_tpgm(&mut self, val: PostTpgm) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Verify Option."]
    #[must_use]
    #[inline(always)]
    pub const fn vfy_opt(&self) -> VfyOpt {
        let val = (self.0 >> 23usize) & 0x03;
        VfyOpt::from_bits(val as u8)
    }
    #[doc = "Verify Option."]
    #[inline(always)]
    pub const fn set_vfy_opt(&mut self, val: VfyOpt) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val.to_bits() as u32) & 0x03) << 23usize);
    }
    #[doc = "Tpgm Option."]
    #[must_use]
    #[inline(always)]
    pub const fn tpgm_opt(&self) -> TpgmOpt {
        let val = (self.0 >> 25usize) & 0x03;
        TpgmOpt::from_bits(val as u8)
    }
    #[doc = "Tpgm Option."]
    #[inline(always)]
    pub const fn set_tpgm_opt(&mut self, val: TpgmOpt) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "MASK0_OPT."]
    #[must_use]
    #[inline(always)]
    pub const fn mask0_opt(&self) -> Mask0Opt {
        let val = (self.0 >> 27usize) & 0x01;
        Mask0Opt::from_bits(val as u8)
    }
    #[doc = "MASK0_OPT."]
    #[inline(always)]
    pub const fn set_mask0_opt(&mut self, val: Mask0Opt) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Disable pre-PV Read before First Program Shot."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_prer(&self) -> DisPrer {
        let val = (self.0 >> 28usize) & 0x01;
        DisPrer::from_bits(val as u8)
    }
    #[doc = "Disable pre-PV Read before First Program Shot."]
    #[inline(always)]
    pub const fn set_dis_prer(&mut self, val: DisPrer) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for SmwSettingOption2 {
    #[inline(always)]
    fn default() -> SmwSettingOption2 {
        SmwSettingOption2(0)
    }
}
impl core::fmt::Debug for SmwSettingOption2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption2")
            .field("thvs_ctrl", &self.thvs_ctrl())
            .field("trcv_ctrl", &self.trcv_ctrl())
            .field("xtra_ers", &self.xtra_ers())
            .field("xtra_pgm", &self.xtra_pgm())
            .field("whv_cntr", &self.whv_cntr())
            .field("post_ters", &self.post_ters())
            .field("post_tpgm", &self.post_tpgm())
            .field("vfy_opt", &self.vfy_opt())
            .field("tpgm_opt", &self.tpgm_opt())
            .field("mask0_opt", &self.mask0_opt())
            .field("dis_prer", &self.dis_prer())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption2 {{ thvs_ctrl: {=u8:?}, trcv_ctrl: {=u8:?}, xtra_ers: {=u8:?}, xtra_pgm: {=u8:?}, whv_cntr: {=u8:?}, post_ters: {:?}, post_tpgm: {:?}, vfy_opt: {:?}, tpgm_opt: {:?}, mask0_opt: {:?}, dis_prer: {:?} }}",
            self.thvs_ctrl(),
            self.trcv_ctrl(),
            self.xtra_ers(),
            self.xtra_pgm(),
            self.whv_cntr(),
            self.post_ters(),
            self.post_tpgm(),
            self.vfy_opt(),
            self.tpgm_opt(),
            self.mask0_opt(),
            self.dis_prer()
        )
    }
}
#[doc = "SMW Setting Option 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSettingOption3(pub u32);
impl SmwSettingOption3 {
    #[doc = "WHV_COUNTER for HEM-erase Cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn hem_whv_cntr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "WHV_COUNTER for HEM-erase Cycle."]
    #[inline(always)]
    pub const fn set_hem_whv_cntr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "HEM Max Erase Shot Count."]
    #[must_use]
    #[inline(always)]
    pub const fn hem_max_ers(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "HEM Max Erase Shot Count."]
    #[inline(always)]
    pub const fn set_hem_max_ers(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
    }
}
impl Default for SmwSettingOption3 {
    #[inline(always)]
    fn default() -> SmwSettingOption3 {
        SmwSettingOption3(0)
    }
}
impl core::fmt::Debug for SmwSettingOption3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSettingOption3")
            .field("hem_whv_cntr", &self.hem_whv_cntr())
            .field("hem_max_ers", &self.hem_max_ers())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSettingOption3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSettingOption3 {{ hem_whv_cntr: {=u8:?}, hem_max_ers: {=u16:?} }}",
            self.hem_whv_cntr(),
            self.hem_max_ers()
        )
    }
}
#[doc = "SMW SME WHV Option 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmeWhvOption0(pub u32);
impl SmwSmeWhvOption0 {
    #[doc = "Smart Erase WHV Option Low."]
    #[must_use]
    #[inline(always)]
    pub const fn sme_whv_opt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Erase WHV Option Low."]
    #[inline(always)]
    pub const fn set_sme_whv_opt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmeWhvOption0 {
    #[inline(always)]
    fn default() -> SmwSmeWhvOption0 {
        SmwSmeWhvOption0(0)
    }
}
impl core::fmt::Debug for SmwSmeWhvOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmeWhvOption0")
            .field("sme_whv_opt0", &self.sme_whv_opt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmeWhvOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmeWhvOption0 {{ sme_whv_opt0: {=u32:?} }}",
            self.sme_whv_opt0()
        )
    }
}
#[doc = "SMW SME WHV Option 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmeWhvOption1(pub u32);
impl SmwSmeWhvOption1 {
    #[doc = "Smart Erase WHV Option High."]
    #[must_use]
    #[inline(always)]
    pub const fn sme_whv_opt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Erase WHV Option High."]
    #[inline(always)]
    pub const fn set_sme_whv_opt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmeWhvOption1 {
    #[inline(always)]
    fn default() -> SmwSmeWhvOption1 {
        SmwSmeWhvOption1(0)
    }
}
impl core::fmt::Debug for SmwSmeWhvOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmeWhvOption1")
            .field("sme_whv_opt1", &self.sme_whv_opt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmeWhvOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmeWhvOption1 {{ sme_whv_opt1: {=u32:?} }}",
            self.sme_whv_opt1()
        )
    }
}
#[doc = "SMW SMP WHV Option 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmpWhvOption0(pub u32);
impl SmwSmpWhvOption0 {
    #[doc = "Smart Program WHV Option Low."]
    #[must_use]
    #[inline(always)]
    pub const fn smp_whv_opt0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Program WHV Option Low."]
    #[inline(always)]
    pub const fn set_smp_whv_opt0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmpWhvOption0 {
    #[inline(always)]
    fn default() -> SmwSmpWhvOption0 {
        SmwSmpWhvOption0(0)
    }
}
impl core::fmt::Debug for SmwSmpWhvOption0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmpWhvOption0")
            .field("smp_whv_opt0", &self.smp_whv_opt0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmpWhvOption0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmpWhvOption0 {{ smp_whv_opt0: {=u32:?} }}",
            self.smp_whv_opt0()
        )
    }
}
#[doc = "SMW SMP WHV Option 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwSmpWhvOption1(pub u32);
impl SmwSmpWhvOption1 {
    #[doc = "Smart Program WHV Option High."]
    #[must_use]
    #[inline(always)]
    pub const fn smp_whv_opt1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Smart Program WHV Option High."]
    #[inline(always)]
    pub const fn set_smp_whv_opt1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SmwSmpWhvOption1 {
    #[inline(always)]
    fn default() -> SmwSmpWhvOption1 {
        SmwSmpWhvOption1(0)
    }
}
impl core::fmt::Debug for SmwSmpWhvOption1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwSmpWhvOption1")
            .field("smp_whv_opt1", &self.smp_whv_opt1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwSmpWhvOption1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwSmpWhvOption1 {{ smp_whv_opt1: {=u32:?} }}",
            self.smp_whv_opt1()
        )
    }
}
#[doc = "SMW Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwStatus(pub u32);
impl SmwStatus {
    #[doc = "SMW Error."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_err(&self) -> SmwErr {
        let val = (self.0 >> 0usize) & 0x01;
        SmwErr::from_bits(val as u8)
    }
    #[doc = "SMW Error."]
    #[inline(always)]
    pub const fn set_smw_err(&mut self, val: SmwErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SMW Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_busy(&self) -> SmwBusy {
        let val = (self.0 >> 1usize) & 0x01;
        SmwBusy::from_bits(val as u8)
    }
    #[doc = "SMW Busy."]
    #[inline(always)]
    pub const fn set_smw_busy(&mut self, val: SmwBusy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "BIST Busy."]
    #[must_use]
    #[inline(always)]
    pub const fn bist_busy(&self) -> BistBusy {
        let val = (self.0 >> 2usize) & 0x01;
        BistBusy::from_bits(val as u8)
    }
    #[doc = "BIST Busy."]
    #[inline(always)]
    pub const fn set_bist_busy(&mut self, val: BistBusy) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for SmwStatus {
    #[inline(always)]
    fn default() -> SmwStatus {
        SmwStatus(0)
    }
}
impl core::fmt::Debug for SmwStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwStatus")
            .field("smw_err", &self.smw_err())
            .field("smw_busy", &self.smw_busy())
            .field("bist_busy", &self.bist_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwStatus {{ smw_err: {:?}, smw_busy: {:?}, bist_busy: {:?} }}",
            self.smw_err(),
            self.smw_busy(),
            self.bist_busy()
        )
    }
}
#[doc = "SMW Timer Option Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmwTimerOption(pub u32);
impl SmwTimerOption {
    #[doc = "Clock Divide Scalar for Long Pulse."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_cdivl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divide Scalar for Long Pulse."]
    #[inline(always)]
    pub const fn set_smw_cdivl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Adjust for Verify."]
    #[must_use]
    #[inline(always)]
    pub const fn smw_tvfy(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Adjust for Verify."]
    #[inline(always)]
    pub const fn set_smw_tvfy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for SmwTimerOption {
    #[inline(always)]
    fn default() -> SmwTimerOption {
        SmwTimerOption(0)
    }
}
impl core::fmt::Debug for SmwTimerOption {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmwTimerOption")
            .field("smw_cdivl", &self.smw_cdivl())
            .field("smw_tvfy", &self.smw_tvfy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmwTimerOption {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmwTimerOption {{ smw_cdivl: {=u8:?}, smw_tvfy: {=u8:?} }}",
            self.smw_cdivl(),
            self.smw_tvfy()
        )
    }
}
#[doc = "SoC Trim Phrase 0 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim00(pub u32);
impl Soctrim00 {
    #[doc = "TRIM0_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_0."]
    #[inline(always)]
    pub const fn set_trim0_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim00 {
    #[inline(always)]
    fn default() -> Soctrim00 {
        Soctrim00(0)
    }
}
impl core::fmt::Debug for Soctrim00 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim00")
            .field("trim0_0", &self.trim0_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim00 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim00 {{ trim0_0: {=u32:?} }}", self.trim0_0())
    }
}
#[doc = "SoC Trim Phrase 0 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim01(pub u32);
impl Soctrim01 {
    #[doc = "TRIM0_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_1."]
    #[inline(always)]
    pub const fn set_trim0_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim01 {
    #[inline(always)]
    fn default() -> Soctrim01 {
        Soctrim01(0)
    }
}
impl core::fmt::Debug for Soctrim01 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim01")
            .field("trim0_1", &self.trim0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim01 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim01 {{ trim0_1: {=u32:?} }}", self.trim0_1())
    }
}
#[doc = "SoC Trim Phrase 0 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim02(pub u32);
impl Soctrim02 {
    #[doc = "TRIM0_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_2."]
    #[inline(always)]
    pub const fn set_trim0_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim02 {
    #[inline(always)]
    fn default() -> Soctrim02 {
        Soctrim02(0)
    }
}
impl core::fmt::Debug for Soctrim02 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim02")
            .field("trim0_2", &self.trim0_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim02 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim02 {{ trim0_2: {=u32:?} }}", self.trim0_2())
    }
}
#[doc = "SoC Trim Phrase 0 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim03(pub u32);
impl Soctrim03 {
    #[doc = "TRIM0_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim0_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM0_3."]
    #[inline(always)]
    pub const fn set_trim0_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim03 {
    #[inline(always)]
    fn default() -> Soctrim03 {
        Soctrim03(0)
    }
}
impl core::fmt::Debug for Soctrim03 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim03")
            .field("trim0_3", &self.trim0_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim03 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim03 {{ trim0_3: {=u32:?} }}", self.trim0_3())
    }
}
#[doc = "SoC Trim Phrase 1 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim10(pub u32);
impl Soctrim10 {
    #[doc = "TRIM1_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_0."]
    #[inline(always)]
    pub const fn set_trim1_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim10 {
    #[inline(always)]
    fn default() -> Soctrim10 {
        Soctrim10(0)
    }
}
impl core::fmt::Debug for Soctrim10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim10")
            .field("trim1_0", &self.trim1_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim10 {{ trim1_0: {=u32:?} }}", self.trim1_0())
    }
}
#[doc = "SoC Trim Phrase 1 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim11(pub u32);
impl Soctrim11 {
    #[doc = "TRIM1_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_1."]
    #[inline(always)]
    pub const fn set_trim1_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim11 {
    #[inline(always)]
    fn default() -> Soctrim11 {
        Soctrim11(0)
    }
}
impl core::fmt::Debug for Soctrim11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim11")
            .field("trim1_1", &self.trim1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim11 {{ trim1_1: {=u32:?} }}", self.trim1_1())
    }
}
#[doc = "SoC Trim Phrase 1 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim12(pub u32);
impl Soctrim12 {
    #[doc = "TRIM1_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_2."]
    #[inline(always)]
    pub const fn set_trim1_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim12 {
    #[inline(always)]
    fn default() -> Soctrim12 {
        Soctrim12(0)
    }
}
impl core::fmt::Debug for Soctrim12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim12")
            .field("trim1_2", &self.trim1_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim12 {{ trim1_2: {=u32:?} }}", self.trim1_2())
    }
}
#[doc = "SoC Trim Phrase 1 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim13(pub u32);
impl Soctrim13 {
    #[doc = "TRIM1_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim1_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM1_3."]
    #[inline(always)]
    pub const fn set_trim1_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim13 {
    #[inline(always)]
    fn default() -> Soctrim13 {
        Soctrim13(0)
    }
}
impl core::fmt::Debug for Soctrim13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim13")
            .field("trim1_3", &self.trim1_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim13 {{ trim1_3: {=u32:?} }}", self.trim1_3())
    }
}
#[doc = "SoC Trim Phrase 2 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim20(pub u32);
impl Soctrim20 {
    #[doc = "TRIM2_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_0."]
    #[inline(always)]
    pub const fn set_trim2_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim20 {
    #[inline(always)]
    fn default() -> Soctrim20 {
        Soctrim20(0)
    }
}
impl core::fmt::Debug for Soctrim20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim20")
            .field("trim2_0", &self.trim2_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim20 {{ trim2_0: {=u32:?} }}", self.trim2_0())
    }
}
#[doc = "SoC Trim Phrase 2 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim21(pub u32);
impl Soctrim21 {
    #[doc = "TRIM2_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_1."]
    #[inline(always)]
    pub const fn set_trim2_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim21 {
    #[inline(always)]
    fn default() -> Soctrim21 {
        Soctrim21(0)
    }
}
impl core::fmt::Debug for Soctrim21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim21")
            .field("trim2_1", &self.trim2_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim21 {{ trim2_1: {=u32:?} }}", self.trim2_1())
    }
}
#[doc = "SoC Trim Phrase 2 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim22(pub u32);
impl Soctrim22 {
    #[doc = "TRIM2_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_2."]
    #[inline(always)]
    pub const fn set_trim2_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim22 {
    #[inline(always)]
    fn default() -> Soctrim22 {
        Soctrim22(0)
    }
}
impl core::fmt::Debug for Soctrim22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim22")
            .field("trim2_2", &self.trim2_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim22 {{ trim2_2: {=u32:?} }}", self.trim2_2())
    }
}
#[doc = "SoC Trim Phrase 2 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim23(pub u32);
impl Soctrim23 {
    #[doc = "TRIM2_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim2_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM2_3."]
    #[inline(always)]
    pub const fn set_trim2_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim23 {
    #[inline(always)]
    fn default() -> Soctrim23 {
        Soctrim23(0)
    }
}
impl core::fmt::Debug for Soctrim23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim23")
            .field("trim2_3", &self.trim2_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim23 {{ trim2_3: {=u32:?} }}", self.trim2_3())
    }
}
#[doc = "SoC Trim Phrase 3 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim30(pub u32);
impl Soctrim30 {
    #[doc = "TRIM3_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_0."]
    #[inline(always)]
    pub const fn set_trim3_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim30 {
    #[inline(always)]
    fn default() -> Soctrim30 {
        Soctrim30(0)
    }
}
impl core::fmt::Debug for Soctrim30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim30")
            .field("trim3_0", &self.trim3_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim30 {{ trim3_0: {=u32:?} }}", self.trim3_0())
    }
}
#[doc = "SoC Trim Phrase 3 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim31(pub u32);
impl Soctrim31 {
    #[doc = "TRIM3_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_1."]
    #[inline(always)]
    pub const fn set_trim3_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim31 {
    #[inline(always)]
    fn default() -> Soctrim31 {
        Soctrim31(0)
    }
}
impl core::fmt::Debug for Soctrim31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim31")
            .field("trim3_1", &self.trim3_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim31 {{ trim3_1: {=u32:?} }}", self.trim3_1())
    }
}
#[doc = "SoC Trim Phrase 3 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim32(pub u32);
impl Soctrim32 {
    #[doc = "TRIM3_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_2."]
    #[inline(always)]
    pub const fn set_trim3_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim32 {
    #[inline(always)]
    fn default() -> Soctrim32 {
        Soctrim32(0)
    }
}
impl core::fmt::Debug for Soctrim32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim32")
            .field("trim3_2", &self.trim3_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim32 {{ trim3_2: {=u32:?} }}", self.trim3_2())
    }
}
#[doc = "SoC Trim Phrase 3 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim33(pub u32);
impl Soctrim33 {
    #[doc = "TRIM3_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim3_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM3_3."]
    #[inline(always)]
    pub const fn set_trim3_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim33 {
    #[inline(always)]
    fn default() -> Soctrim33 {
        Soctrim33(0)
    }
}
impl core::fmt::Debug for Soctrim33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim33")
            .field("trim3_3", &self.trim3_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim33 {{ trim3_3: {=u32:?} }}", self.trim3_3())
    }
}
#[doc = "SoC Trim Phrase 4 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim40(pub u32);
impl Soctrim40 {
    #[doc = "TRIM4_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_0."]
    #[inline(always)]
    pub const fn set_trim4_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim40 {
    #[inline(always)]
    fn default() -> Soctrim40 {
        Soctrim40(0)
    }
}
impl core::fmt::Debug for Soctrim40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim40")
            .field("trim4_0", &self.trim4_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim40 {{ trim4_0: {=u32:?} }}", self.trim4_0())
    }
}
#[doc = "SoC Trim Phrase 4 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim41(pub u32);
impl Soctrim41 {
    #[doc = "TRIM4_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_1."]
    #[inline(always)]
    pub const fn set_trim4_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim41 {
    #[inline(always)]
    fn default() -> Soctrim41 {
        Soctrim41(0)
    }
}
impl core::fmt::Debug for Soctrim41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim41")
            .field("trim4_1", &self.trim4_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim41 {{ trim4_1: {=u32:?} }}", self.trim4_1())
    }
}
#[doc = "SoC Trim Phrase 4 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim42(pub u32);
impl Soctrim42 {
    #[doc = "TRIM4_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_2."]
    #[inline(always)]
    pub const fn set_trim4_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim42 {
    #[inline(always)]
    fn default() -> Soctrim42 {
        Soctrim42(0)
    }
}
impl core::fmt::Debug for Soctrim42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim42")
            .field("trim4_2", &self.trim4_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim42 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim42 {{ trim4_2: {=u32:?} }}", self.trim4_2())
    }
}
#[doc = "SoC Trim Phrase 4 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim43(pub u32);
impl Soctrim43 {
    #[doc = "TRIM4_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim4_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM4_3."]
    #[inline(always)]
    pub const fn set_trim4_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim43 {
    #[inline(always)]
    fn default() -> Soctrim43 {
        Soctrim43(0)
    }
}
impl core::fmt::Debug for Soctrim43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim43")
            .field("trim4_3", &self.trim4_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim43 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim43 {{ trim4_3: {=u32:?} }}", self.trim4_3())
    }
}
#[doc = "SoC Trim Phrase 5 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim50(pub u32);
impl Soctrim50 {
    #[doc = "TRIM5_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_0."]
    #[inline(always)]
    pub const fn set_trim5_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim50 {
    #[inline(always)]
    fn default() -> Soctrim50 {
        Soctrim50(0)
    }
}
impl core::fmt::Debug for Soctrim50 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim50")
            .field("trim5_0", &self.trim5_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim50 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim50 {{ trim5_0: {=u32:?} }}", self.trim5_0())
    }
}
#[doc = "SoC Trim Phrase 5 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim51(pub u32);
impl Soctrim51 {
    #[doc = "TRIM5_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_1."]
    #[inline(always)]
    pub const fn set_trim5_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim51 {
    #[inline(always)]
    fn default() -> Soctrim51 {
        Soctrim51(0)
    }
}
impl core::fmt::Debug for Soctrim51 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim51")
            .field("trim5_1", &self.trim5_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim51 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim51 {{ trim5_1: {=u32:?} }}", self.trim5_1())
    }
}
#[doc = "SoC Trim Phrase 5 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim52(pub u32);
impl Soctrim52 {
    #[doc = "TRIM5_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_2."]
    #[inline(always)]
    pub const fn set_trim5_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim52 {
    #[inline(always)]
    fn default() -> Soctrim52 {
        Soctrim52(0)
    }
}
impl core::fmt::Debug for Soctrim52 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim52")
            .field("trim5_2", &self.trim5_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim52 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim52 {{ trim5_2: {=u32:?} }}", self.trim5_2())
    }
}
#[doc = "SoC Trim Phrase 5 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim53(pub u32);
impl Soctrim53 {
    #[doc = "TRIM5_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim5_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM5_3."]
    #[inline(always)]
    pub const fn set_trim5_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim53 {
    #[inline(always)]
    fn default() -> Soctrim53 {
        Soctrim53(0)
    }
}
impl core::fmt::Debug for Soctrim53 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim53")
            .field("trim5_3", &self.trim5_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim53 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim53 {{ trim5_3: {=u32:?} }}", self.trim5_3())
    }
}
#[doc = "SoC Trim Phrase 6 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim60(pub u32);
impl Soctrim60 {
    #[doc = "TRIM6_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_0."]
    #[inline(always)]
    pub const fn set_trim6_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim60 {
    #[inline(always)]
    fn default() -> Soctrim60 {
        Soctrim60(0)
    }
}
impl core::fmt::Debug for Soctrim60 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim60")
            .field("trim6_0", &self.trim6_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim60 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim60 {{ trim6_0: {=u32:?} }}", self.trim6_0())
    }
}
#[doc = "SoC Trim Phrase 6 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim61(pub u32);
impl Soctrim61 {
    #[doc = "TRIM6_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_1."]
    #[inline(always)]
    pub const fn set_trim6_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim61 {
    #[inline(always)]
    fn default() -> Soctrim61 {
        Soctrim61(0)
    }
}
impl core::fmt::Debug for Soctrim61 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim61")
            .field("trim6_1", &self.trim6_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim61 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim61 {{ trim6_1: {=u32:?} }}", self.trim6_1())
    }
}
#[doc = "SoC Trim Phrase 6 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim62(pub u32);
impl Soctrim62 {
    #[doc = "TRIM6_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_2."]
    #[inline(always)]
    pub const fn set_trim6_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim62 {
    #[inline(always)]
    fn default() -> Soctrim62 {
        Soctrim62(0)
    }
}
impl core::fmt::Debug for Soctrim62 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim62")
            .field("trim6_2", &self.trim6_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim62 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim62 {{ trim6_2: {=u32:?} }}", self.trim6_2())
    }
}
#[doc = "SoC Trim Phrase 6 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim63(pub u32);
impl Soctrim63 {
    #[doc = "TRIM6_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim6_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM6_3."]
    #[inline(always)]
    pub const fn set_trim6_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim63 {
    #[inline(always)]
    fn default() -> Soctrim63 {
        Soctrim63(0)
    }
}
impl core::fmt::Debug for Soctrim63 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim63")
            .field("trim6_3", &self.trim6_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim63 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim63 {{ trim6_3: {=u32:?} }}", self.trim6_3())
    }
}
#[doc = "SoC Trim Phrase 7 Word 0 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim70(pub u32);
impl Soctrim70 {
    #[doc = "TRIM7_0."]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_0."]
    #[inline(always)]
    pub const fn set_trim7_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim70 {
    #[inline(always)]
    fn default() -> Soctrim70 {
        Soctrim70(0)
    }
}
impl core::fmt::Debug for Soctrim70 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim70")
            .field("trim7_0", &self.trim7_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim70 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim70 {{ trim7_0: {=u32:?} }}", self.trim7_0())
    }
}
#[doc = "SoC Trim Phrase 7 Word 1 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim71(pub u32);
impl Soctrim71 {
    #[doc = "TRIM7_1."]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_1."]
    #[inline(always)]
    pub const fn set_trim7_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim71 {
    #[inline(always)]
    fn default() -> Soctrim71 {
        Soctrim71(0)
    }
}
impl core::fmt::Debug for Soctrim71 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim71")
            .field("trim7_1", &self.trim7_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim71 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim71 {{ trim7_1: {=u32:?} }}", self.trim7_1())
    }
}
#[doc = "SoC Trim Phrase 7 Word 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim72(pub u32);
impl Soctrim72 {
    #[doc = "TRIM7_2."]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_2."]
    #[inline(always)]
    pub const fn set_trim7_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim72 {
    #[inline(always)]
    fn default() -> Soctrim72 {
        Soctrim72(0)
    }
}
impl core::fmt::Debug for Soctrim72 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim72")
            .field("trim7_2", &self.trim7_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim72 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim72 {{ trim7_2: {=u32:?} }}", self.trim7_2())
    }
}
#[doc = "SoC Trim Phrase 7 Word 3 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Soctrim73(pub u32);
impl Soctrim73 {
    #[doc = "TRIM7_3."]
    #[must_use]
    #[inline(always)]
    pub const fn trim7_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TRIM7_3."]
    #[inline(always)]
    pub const fn set_trim7_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Soctrim73 {
    #[inline(always)]
    fn default() -> Soctrim73 {
        Soctrim73(0)
    }
}
impl core::fmt::Debug for Soctrim73 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Soctrim73")
            .field("trim7_3", &self.trim7_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Soctrim73 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Soctrim73 {{ trim7_3: {=u32:?} }}", self.trim7_3())
    }
}
#[doc = "User Interface Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UintCtl(pub u32);
impl UintCtl {
    #[doc = "Set Fail On Exit."]
    #[must_use]
    #[inline(always)]
    pub const fn set_fail(&self) -> SetFail {
        let val = (self.0 >> 0usize) & 0x01;
        SetFail::from_bits(val as u8)
    }
    #[doc = "Set Fail On Exit."]
    #[inline(always)]
    pub const fn set_set_fail(&mut self, val: SetFail) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Double-Bit ECC Fault Detect."]
    #[must_use]
    #[inline(always)]
    pub const fn dberr(&self) -> Dberr {
        let val = (self.0 >> 1usize) & 0x01;
        Dberr::from_bits(val as u8)
    }
    #[doc = "Double-Bit ECC Fault Detect."]
    #[inline(always)]
    pub const fn set_dberr(&mut self, val: Dberr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for UintCtl {
    #[inline(always)]
    fn default() -> UintCtl {
        UintCtl(0)
    }
}
impl core::fmt::Debug for UintCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UintCtl")
            .field("set_fail", &self.set_fail())
            .field("dberr", &self.dberr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UintCtl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "UintCtl {{ set_fail: {:?}, dberr: {:?} }}",
            self.set_fail(),
            self.dberr()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AbortLoop {
    #[doc = "No effect."]
    Zz335 = 0x0,
    #[doc = "Abort BIST loop commands and force the loop counter to return to 0x0."]
    Zz336 = 0x01,
}
impl AbortLoop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AbortLoop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AbortLoop {
    #[inline(always)]
    fn from(val: u8) -> AbortLoop {
        AbortLoop::from_bits(val)
    }
}
impl From<AbortLoop> for u8 {
    #[inline(always)]
    fn from(val: AbortLoop) -> u8 {
        AbortLoop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Abtreq {
    #[doc = "No request to abort a command write sequence."]
    Zz39 = 0x0,
    #[doc = "Request to abort a command write sequence."]
    Zz40 = 0x01,
}
impl Abtreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abtreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abtreq {
    #[inline(always)]
    fn from(val: u8) -> Abtreq {
        Abtreq::from_bits(val)
    }
}
impl From<Abtreq> for u8 {
    #[inline(always)]
    fn from(val: Abtreq) -> u8 {
        Abtreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Accerr {
    #[doc = "No access error detected."]
    Zz21 = 0x0,
    #[doc = "Access error detected."]
    Zz22 = 0x01,
}
impl Accerr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Accerr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Accerr {
    #[inline(always)]
    fn from(val: u8) -> Accerr {
        Accerr::from_bits(val)
    }
}
impl From<Accerr> for u8 {
    #[inline(always)]
    fn from(val: Accerr) -> u8 {
        Accerr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrFail {
    #[doc = "The address is within the flash or IFR address space."]
    Zz141 = 0x0,
    #[doc = "The address is outside the flash or IFR address space."]
    Zz142 = 0x01,
}
impl AddrFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrFail {
    #[inline(always)]
    fn from(val: u8) -> AddrFail {
        AddrFail::from_bits(val)
    }
}
impl From<AddrFail> for u8 {
    #[inline(always)]
    fn from(val: AddrFail) -> u8 {
        AddrFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailBlk {
    #[doc = "The address is block-aligned."]
    Zz143 = 0x0,
    #[doc = "The address is not block-aligned."]
    Zz144 = 0x01,
}
impl AlignfailBlk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailBlk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailBlk {
    #[inline(always)]
    fn from(val: u8) -> AlignfailBlk {
        AlignfailBlk::from_bits(val)
    }
}
impl From<AlignfailBlk> for u8 {
    #[inline(always)]
    fn from(val: AlignfailBlk) -> u8 {
        AlignfailBlk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailPg {
    #[doc = "The address is page-aligned."]
    Zz147 = 0x0,
    #[doc = "The address is not page-aligned."]
    Zz148 = 0x01,
}
impl AlignfailPg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailPg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailPg {
    #[inline(always)]
    fn from(val: u8) -> AlignfailPg {
        AlignfailPg::from_bits(val)
    }
}
impl From<AlignfailPg> for u8 {
    #[inline(always)]
    fn from(val: AlignfailPg) -> u8 {
        AlignfailPg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailPhr {
    #[doc = "The address is phrase-aligned."]
    Zz149 = 0x0,
    #[doc = "The address is not phrase-aligned."]
    Zz150 = 0x01,
}
impl AlignfailPhr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailPhr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailPhr {
    #[inline(always)]
    fn from(val: u8) -> AlignfailPhr {
        AlignfailPhr::from_bits(val)
    }
}
impl From<AlignfailPhr> for u8 {
    #[inline(always)]
    fn from(val: AlignfailPhr) -> u8 {
        AlignfailPhr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AlignfailScr {
    #[doc = "The address is sector-aligned."]
    Zz145 = 0x0,
    #[doc = "The address is not sector-aligned."]
    Zz146 = 0x01,
}
impl AlignfailScr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AlignfailScr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AlignfailScr {
    #[inline(always)]
    fn from(val: u8) -> AlignfailScr {
        AlignfailScr::from_bits(val)
    }
}
impl From<AlignfailScr> for u8 {
    #[inline(always)]
    fn from(val: AlignfailScr) -> u8 {
        AlignfailScr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AllCmd {
    #[doc = "The command operates on a single flash block."]
    Zz137 = 0x0,
    #[doc = "The command operates on all flash blocks."]
    Zz138 = 0x01,
}
impl AllCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AllCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AllCmd {
    #[inline(always)]
    fn from(val: u8) -> AllCmd {
        AllCmd::from_bits(val)
    }
}
impl From<AllCmd> for u8 {
    #[inline(always)]
    fn from(val: AllCmd) -> u8 {
        AllCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AryTrimDone {
    #[doc = "Recall register load operation has not been completed."]
    Zz93 = 0x0,
    #[doc = "Recall register load operation has completed."]
    Zz94 = 0x01,
}
impl AryTrimDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AryTrimDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AryTrimDone {
    #[inline(always)]
    fn from(val: u8) -> AryTrimDone {
        AryTrimDone::from_bits(val)
    }
}
impl From<AryTrimDone> for u8 {
    #[inline(always)]
    fn from(val: AryTrimDone) -> u8 {
        AryTrimDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BdoneSel {
    #[doc = "Select internal bist_done signal from current module instantiation."]
    Zz453 = 0x0,
    #[doc = "Select ipt_bist_fail signal from current module instantiation."]
    Zz454 = 0x01,
    #[doc = "Select ipt_bist_done signal from other module instantiation."]
    Zz455 = 0x02,
    #[doc = "Select AND of internal bist_done signal from current module instantiation with ipt_bist_done signal from other module instantiation."]
    Zz456 = 0x03,
}
impl BdoneSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BdoneSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BdoneSel {
    #[inline(always)]
    fn from(val: u8) -> BdoneSel {
        BdoneSel::from_bits(val)
    }
}
impl From<BdoneSel> for u8 {
    #[inline(always)]
    fn from(val: BdoneSel) -> u8 {
        BdoneSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistBusy {
    #[doc = "BIST Command not active."]
    Zz201 = 0x0,
    #[doc = "BIST Command is active."]
    Zz202 = 0x01,
}
impl BistBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistBusy {
    #[inline(always)]
    fn from(val: u8) -> BistBusy {
        BistBusy::from_bits(val)
    }
}
impl From<BistBusy> for u8 {
    #[inline(always)]
    fn from(val: BistBusy) -> u8 {
        BistBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistCtl {
    #[doc = "BIST IP disabled."]
    Zz107 = 0x0,
    #[doc = "BIST IP enabled."]
    Zz108 = 0x01,
}
impl BistCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistCtl {
    #[inline(always)]
    fn from(val: u8) -> BistCtl {
        BistCtl::from_bits(val)
    }
}
impl From<BistCtl> for u8 {
    #[inline(always)]
    fn from(val: BistCtl) -> u8 {
        BistCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistDone {
    #[doc = "The BIST (or data dump) is running."]
    Zz439 = 0x0,
    #[doc = "The BIST (or data dump) has completed."]
    Zz440 = 0x01,
}
impl BistDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistDone {
    #[inline(always)]
    fn from(val: u8) -> BistDone {
        BistDone::from_bits(val)
    }
}
impl From<BistDone> for u8 {
    #[inline(always)]
    fn from(val: BistDone) -> u8 {
        BistDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistEccEn {
    #[doc = "ECC correction disabled."]
    Zz169 = 0x0,
    #[doc = "ECC correction enabled."]
    Zz170 = 0x01,
}
impl BistEccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistEccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistEccEn {
    #[inline(always)]
    fn from(val: u8) -> BistEccEn {
        BistEccEn::from_bits(val)
    }
}
impl From<BistEccEn> for u8 {
    #[inline(always)]
    fn from(val: BistEccEn) -> u8 {
        BistEccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistFail {
    #[doc = "The last BIST operation completed successfully (or could not fail)."]
    Zz437 = 0x0,
    #[doc = "The last BIST operation failed."]
    Zz438 = 0x01,
}
impl BistFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistFail {
    #[inline(always)]
    fn from(val: u8) -> BistFail {
        BistFail::from_bits(val)
    }
}
impl From<BistFail> for u8 {
    #[inline(always)]
    fn from(val: BistFail) -> u8 {
        BistFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistMuxToSmw {
    #[doc = "BIST drives fields."]
    Zz183 = 0x0,
    #[doc = "SMW registers drive fields."]
    Zz184 = 0x01,
}
impl BistMuxToSmw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistMuxToSmw {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistMuxToSmw {
    #[inline(always)]
    fn from(val: u8) -> BistMuxToSmw {
        BistMuxToSmw::from_bits(val)
    }
}
impl From<BistMuxToSmw> for u8 {
    #[inline(always)]
    fn from(val: BistMuxToSmw) -> u8 {
        BistMuxToSmw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistOn {
    #[doc = "BIST enable not forced by user interface."]
    Zz159 = 0x0,
    #[doc = "BIST enable control by user interface."]
    Zz160 = 0x01,
}
impl BistOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistOn {
    #[inline(always)]
    fn from(val: u8) -> BistOn {
        BistOn::from_bits(val)
    }
}
impl From<BistOn> for u8 {
    #[inline(always)]
    fn from(val: BistOn) -> u8 {
        BistOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BistPwrDis {
    #[doc = "BIST DFT logic has full control of SLM and LVE when BIST is enabled (including during commands)."]
    Zz97 = 0x0,
    #[doc = "BIST DFT logic has no control of SLM and LVE; power mode RTL is in complete control of SLM and LVE values."]
    Zz98 = 0x01,
}
impl BistPwrDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BistPwrDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BistPwrDis {
    #[inline(always)]
    fn from(val: u8) -> BistPwrDis {
        BistPwrDis::from_bits(val)
    }
}
impl From<BistPwrDis> for u8 {
    #[inline(always)]
    fn from(val: BistPwrDis) -> u8 {
        BistPwrDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BsdoSel {
    #[doc = "Select internal bist_sdo signal from current module instantiation."]
    Zz449 = 0x0,
    #[doc = "Select ipt_bist_done signal from current module instantiation."]
    Zz450 = 0x01,
    #[doc = "Select ipt_bist_sdo signal from other module instantiation."]
    Zz451 = 0x02,
    #[doc = "Select ipt_bist_done signal from other module instantiation."]
    Zz452 = 0x03,
}
impl BsdoSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BsdoSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BsdoSel {
    #[inline(always)]
    fn from(val: u8) -> BsdoSel {
        BsdoSel::from_bits(val)
    }
}
impl From<BsdoSel> for u8 {
    #[inline(always)]
    fn from(val: BsdoSel) -> u8 {
        BsdoSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "BIST is idle."]
    Zz333 = 0x0,
    #[doc = "BIST is busy."]
    Zz334 = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccie {
    #[doc = "Command complete interrupt disabled."]
    Zz37 = 0x0,
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\] flag is set."]
    Zz38 = 0x01,
}
impl Ccie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccie {
    #[inline(always)]
    fn from(val: u8) -> Ccie {
        Ccie::from_bits(val)
    }
}
impl From<Ccie> for u8 {
    #[inline(always)]
    fn from(val: Ccie) -> u8 {
        Ccie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccif {
    #[doc = "Flash command or initialization in progress."]
    Zz17 = 0x0,
    #[doc = "Flash command or initialization has completed."]
    Zz18 = 0x01,
}
impl Ccif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccif {
    #[inline(always)]
    fn from(val: u8) -> Ccif {
        Ccif::from_bits(val)
    }
}
impl From<Ccif> for u8 {
    #[inline(always)]
    fn from(val: Ccif) -> u8 {
        Ccif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmd {
    #[doc = "IDLE."]
    Zz193 = 0x0,
    #[doc = "ABORT."]
    Zz194 = 0x01,
    #[doc = "SME2 to one-shot mass erase."]
    Zz195 = 0x02,
    #[doc = "SME3 to sector erase on selected array."]
    Zz196 = 0x03,
    #[doc = "SMP1 to program phrase or page on selected array with shot disabled on previously programmed bit."]
    Zz197 = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "SMP2 to program phrase or page on selected array to repair cells of weak program after power loss."]
    Zz199 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Cmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmd {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmd {
    #[inline(always)]
    fn from(val: u8) -> Cmd {
        Cmd::from_bits(val)
    }
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(val: Cmd) -> u8 {
        Cmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdabt {
    #[doc = "No command abort detected."]
    Zz25 = 0x0,
    #[doc = "Command abort detected."]
    Zz26 = 0x01,
}
impl Cmdabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdabt {
    #[inline(always)]
    fn from(val: u8) -> Cmdabt {
        Cmdabt::from_bits(val)
    }
}
impl From<Cmdabt> for u8 {
    #[inline(always)]
    fn from(val: Cmdabt) -> u8 {
        Cmdabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdp {
    #[doc = "Command protection level and domain ID are stale."]
    Zz11 = 0x0,
    #[doc = "Command protection level (CMDPRT) and domain ID (CMDDID) are set."]
    Zz12 = 0x01,
}
impl Cmdp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdp {
    #[inline(always)]
    fn from(val: u8) -> Cmdp {
        Cmdp::from_bits(val)
    }
}
impl From<Cmdp> for u8 {
    #[inline(always)]
    fn from(val: Cmdp) -> u8 {
        Cmdp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdprt {
    #[doc = "Secure, normal access."]
    Zz13 = 0x0,
    #[doc = "Secure, privileged access."]
    Zz14 = 0x01,
    #[doc = "Nonsecure, normal access."]
    Zz15 = 0x02,
    #[doc = "Nonsecure, privileged access."]
    Zz16 = 0x03,
}
impl Cmdprt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdprt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdprt {
    #[inline(always)]
    fn from(val: u8) -> Cmdprt {
        Cmdprt::from_bits(val)
    }
}
impl From<Cmdprt> for u8 {
    #[inline(always)]
    fn from(val: Cmdprt) -> u8 {
        Cmdprt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpMask {
    #[doc = "Expected data is compared to DOUT."]
    Zz229 = 0x0,
    #[doc = "Expected data (only 0s are considered) are compared to DOUT."]
    Zz230 = 0x01,
    #[doc = "Expected data (only 1s are considered) are compared to DOUT."]
    Zz231 = 0x02,
    _RESERVED_3 = 0x03,
}
impl CmpMask {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpMask {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpMask {
    #[inline(always)]
    fn from(val: u8) -> CmpMask {
        CmpMask::from_bits(val)
    }
}
impl From<CmpMask> for u8 {
    #[inline(always)]
    fn from(val: CmpMask) -> u8 {
        CmpMask::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Corehld {
    #[doc = "CPU access is allowed."]
    Zz125 = 0x0,
    #[doc = "CPU access must be blocked."]
    Zz126 = 0x01,
}
impl Corehld {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Corehld {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Corehld {
    #[inline(always)]
    fn from(val: u8) -> Corehld {
        Corehld::from_bits(val)
    }
}
impl From<Corehld> for u8 {
    #[inline(always)]
    fn from(val: Corehld) -> u8 {
        Corehld::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpyParEn {
    #[doc = "Copy parity disabled."]
    Zz185 = 0x0,
    #[doc = "Copy parity enabled."]
    Zz186 = 0x01,
}
impl CpyParEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpyParEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpyParEn {
    #[inline(always)]
    fn from(val: u8) -> CpyParEn {
        CpyParEn::from_bits(val)
    }
}
impl From<CpyParEn> for u8 {
    #[inline(always)]
    fn from(val: CpyParEn) -> u8 {
        CpyParEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpyPhraseEn {
    #[doc = "Copy Flash read data disabled."]
    Zz173 = 0x0,
    #[doc = "Copy Flash read data enabled."]
    Zz174 = 0x01,
}
impl CpyPhraseEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpyPhraseEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpyPhraseEn {
    #[inline(always)]
    fn from(val: u8) -> CpyPhraseEn {
        CpyPhraseEn::from_bits(val)
    }
}
impl From<CpyPhraseEn> for u8 {
    #[inline(always)]
    fn from(val: CpyPhraseEn) -> u8 {
        CpyPhraseEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsabt {
    #[doc = "Command write sequence not aborted."]
    Zz19 = 0x0,
    #[doc = "Command write sequence aborted."]
    Zz20 = 0x01,
}
impl Cwsabt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsabt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsabt {
    #[inline(always)]
    fn from(val: u8) -> Cwsabt {
        Cwsabt::from_bits(val)
    }
}
impl From<Cwsabt> for u8 {
    #[inline(always)]
    fn from(val: Cwsabt) -> u8 {
        Cwsabt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cwsabten {
    #[doc = "CWS abort feature is disabled."]
    Zz115 = 0x0,
    #[doc = "CWS abort feature is enabled."]
    Zz116 = 0x01,
}
impl Cwsabten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cwsabten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cwsabten {
    #[inline(always)]
    fn from(val: u8) -> Cwsabten {
        Cwsabten::from_bits(val)
    }
}
impl From<Cwsabten> for u8 {
    #[inline(always)]
    fn from(val: Cwsabten) -> u8 {
        Cwsabten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpMrgen {
    #[doc = "Normal read pulse shape."]
    Zz431 = 0x0,
    #[doc = "Margin read pulse shape."]
    Zz432 = 0x01,
}
impl DatadumpMrgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpMrgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpMrgen {
    #[inline(always)]
    fn from(val: u8) -> DatadumpMrgen {
        DatadumpMrgen::from_bits(val)
    }
}
impl From<DatadumpMrgen> for u8 {
    #[inline(always)]
    fn from(val: DatadumpMrgen) -> u8 {
        DatadumpMrgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpMrgtype {
    #[doc = "DIN method used."]
    Zz429 = 0x0,
    #[doc = "TM method used."]
    Zz430 = 0x01,
}
impl DatadumpMrgtype {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpMrgtype {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpMrgtype {
    #[inline(always)]
    fn from(val: u8) -> DatadumpMrgtype {
        DatadumpMrgtype::from_bits(val)
    }
}
impl From<DatadumpMrgtype> for u8 {
    #[inline(always)]
    fn from(val: DatadumpMrgtype) -> u8 {
        DatadumpMrgtype::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DatadumpPatt {
    #[doc = "All ones."]
    Zz433 = 0x0,
    #[doc = "All zeroes."]
    Zz434 = 0x01,
    #[doc = "Checkerboard."]
    Zz435 = 0x02,
    #[doc = "Inverse checkerboard."]
    Zz436 = 0x03,
}
impl DatadumpPatt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DatadumpPatt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DatadumpPatt {
    #[inline(always)]
    fn from(val: u8) -> DatadumpPatt {
        DatadumpPatt::from_bits(val)
    }
}
impl From<DatadumpPatt> for u8 {
    #[inline(always)]
    fn from(val: DatadumpPatt) -> u8 {
        DatadumpPatt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dberr {
    #[doc = "No double-bit fault detected during UINT-driven read sequence."]
    Zz163 = 0x0,
    #[doc = "Double-bit fault detected during UINT-driven read sequence."]
    Zz164 = 0x01,
}
impl Dberr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dberr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dberr {
    #[inline(always)]
    fn from(val: u8) -> Dberr {
        Dberr::from_bits(val)
    }
}
impl From<Dberr> for u8 {
    #[inline(always)]
    fn from(val: Dberr) -> u8 {
        Dberr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DberrReg {
    #[doc = "Double-bit fault not detected."]
    Zz177 = 0x0,
    #[doc = "Double-bit fault detected on previous UINT flash read."]
    Zz178 = 0x01,
}
impl DberrReg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DberrReg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DberrReg {
    #[inline(always)]
    fn from(val: u8) -> DberrReg {
        DberrReg::from_bits(val)
    }
}
impl From<DberrReg> for u8 {
    #[inline(always)]
    fn from(val: DberrReg) -> u8 {
        DberrReg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbgctl {
    #[doc = "Default."]
    Zz213 = 0x0,
    #[doc = "Enable debug feature to collect failure address and data."]
    Zz214 = 0x01,
}
impl Dbgctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbgctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbgctl {
    #[inline(always)]
    fn from(val: u8) -> Dbgctl {
        Dbgctl::from_bits(val)
    }
}
impl From<Dbgctl> for u8 {
    #[inline(always)]
    fn from(val: Dbgctl) -> u8 {
        Dbgctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdie {
    #[doc = "Double bit fault detect interrupt disabled."]
    Zz33 = 0x0,
    #[doc = "Double bit fault detect interrupt enabled; an interrupt request is generated whenever the FSTAT\\[DFDIF\\] flag is set."]
    Zz34 = 0x01,
}
impl Dfdie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdie {
    #[inline(always)]
    fn from(val: u8) -> Dfdie {
        Dfdie::from_bits(val)
    }
}
impl From<Dfdie> for u8 {
    #[inline(always)]
    fn from(val: Dfdie) -> u8 {
        Dfdie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dfdif {
    #[doc = "Double bit fault not detected during a valid flash read access from the FMC."]
    Zz9 = 0x0,
    #[doc = "Double bit fault detected (or FCTRL\\[FDFD\\] is set) during a valid flash read access from the FMC."]
    Zz10 = 0x01,
}
impl Dfdif {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dfdif {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dfdif {
    #[inline(always)]
    fn from(val: u8) -> Dfdif {
        Dfdif::from_bits(val)
    }
}
impl From<Dfdif> for u8 {
    #[inline(always)]
    fn from(val: Dfdif) -> u8 {
        Dfdif::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftData {
    #[doc = "CKBD pattern. For READ operations only, compare DOUT with checkerboard data pattern for each read cycle."]
    Zz232 = 0x0,
    #[doc = "ICKBD pattern. For READ operations only, compare DOUT with inverse checkerboard data pattern for each read cycle."]
    Zz233 = 0x01,
    #[doc = "Diagonal pattern. Used for READ operations only, compare DOUT to diagonal pattern."]
    Zz234 = 0x02,
    #[doc = "Fixed data pattern. For READ operations, comparison to DOUT for selected groups; refer to R_ADR_CTRL\\[GRPSEL\\] for modules with multiple groups."]
    Zz235 = 0x03,
    #[doc = "Random data pattern which will be generated based on the initial seed set in R_DATA; for READ operations, used for DOUT comparison of selected groups. For PROG operations, used to control DIN of selected groups."]
    Zz236 = 0x04,
    #[doc = "DOUT based pattern. For READ operations only, DOUT of selected group will be latched in R_DATA. If more than one group is selected in R_ADR_CTRL\\[GRPSEL\\], the group with the lower index will be latched."]
    Zz237 = 0x05,
    #[doc = "R_DATA based pattern. For READ operations, expected DOUT value of selected groups equals to R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]. For PROG operations, DIN of selected groups equals R_DATA when XADR\\[0\\]==YADR\\[0\\] or ~R_DATA when XADR\\[0\\]!=YADR\\[0\\]."]
    Zz238 = 0x06,
    #[doc = "SCAN-IO pattern. For READ operations, control expected DOUT value of selected groups to SCAN-IO data pattern. For PROG operations, control DIN of selected groups to SCAN-IO data pattern."]
    Zz239 = 0x07,
    #[doc = "REPAIR set. For PROG operation to IFR1(7,1) and IFR1(7,2), R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 and R_REPAIR1_1 will control DIN. For READ operation on IFR1(7,1) and IFR1(7,2), DOUT will be compared against R_REPAIR0_0 and R_REPAIR0_1 or R_REPAIR1_0 andR_REPAIR1_1. When this option is selected, only one flash block can be selected."]
    Zz240 = 0x08,
    #[doc = "REPAIR load. For READ operation only, DOUT from IFR1(7,1) and IFR1(7,2) is loaded to R_REPAIR0 and R_REPAIR1."]
    Zz241 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftData {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftData {
    #[inline(always)]
    fn from(val: u8) -> DftData {
        DftData::from_bits(val)
    }
}
impl From<DftData> for u8 {
    #[inline(always)]
    fn from(val: DftData) -> u8 {
        DftData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftDataSrc {
    #[doc = "{R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used."]
    Zz227 = 0x0,
    #[doc = "{R_DATA_CTRL3,R_DATA_CTRL2_EX\\[2:0\\],R_DATA_CTRL2,R_DATA_CTRL1_EX\\[2:0\\],R_DATA_CTRL1,R_DATA_CTRL_EX\\[2:0\\],R_DATA_CTRL0} is used."]
    Zz228 = 0x01,
}
impl DftDataSrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftDataSrc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftDataSrc {
    #[inline(always)]
    fn from(val: u8) -> DftDataSrc {
        DftDataSrc::from_bits(val)
    }
}
impl From<DftDataSrc> for u8 {
    #[inline(always)]
    fn from(val: DftDataSrc) -> u8 {
        DftDataSrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftXadr {
    #[doc = "XADR fixed, no change at all."]
    Zz252 = 0x0,
    #[doc = "XADR increased by 1 after row. For READ operation, XADR increases by 1 after reading the last word of row. For PROG operation, XADR increases by 1 after NVSTR falls."]
    Zz253 = 0x01,
    #[doc = "XADR increased for diagonal. For PROG-DIAGONAL operation, XADR is increased to create diagonal pattern."]
    Zz254 = 0x02,
    #[doc = "XADR increased by sector. During ERASE operation, XADR increased by number of rows in a sector when NVSTR falls."]
    Zz255 = 0x03,
    #[doc = "XADR inversed. XADR is inversed after reading one word or after programming one row when NVSTR falls."]
    Zz256 = 0x04,
    #[doc = "XADR increased by 2 after row. For READ operation, XADR is increased by 2 after reading the last word of a row. For PROG operation, XADR is increased by 2 when NVSTR falls."]
    Zz257 = 0x05,
    #[doc = "XADR\\[0\\] inversed. XADR\\[0\\] is inversed after reading one word or after programming one row when NVSTR falls."]
    Zz258 = 0x06,
    #[doc = "XADR increased by 1. For READ operations only, XADR increased by 1 after each read cycle."]
    Zz259 = 0x07,
    #[doc = "XADR decreased by 1 after row. For READ operations only, XADR is decreased by 1 after YADR decreases to 0."]
    Zz260 = 0x08,
    #[doc = "XADR decreased by 1. For READ operations only, XADR is decreased by 1 after each read cycle."]
    Zz261 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftXadr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftXadr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftXadr {
    #[inline(always)]
    fn from(val: u8) -> DftXadr {
        DftXadr::from_bits(val)
    }
}
impl From<DftXadr> for u8 {
    #[inline(always)]
    fn from(val: DftXadr) -> u8 {
        DftXadr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DftYadr {
    #[doc = "YADR fixed, no change at all."]
    Zz242 = 0x0,
    #[doc = "YADR for ICKBD. For PROG and READ operations, YADR changed to generate inverse checkerboard pattern."]
    Zz243 = 0x01,
    #[doc = "YADR for CKBD. For PROG and READ operations, YADR changed to generate checkerboard pattern."]
    Zz244 = 0x02,
    #[doc = "YADR increased by 1. For READ operations, YADR increased by 1 after each read cycle. For PROG operations, YADR increased by 1 after YE falls."]
    Zz245 = 0x03,
    #[doc = "YADR increased for diagonal. For PROG-DIAGONAL operation, YADR is increased to create diagonal pattern."]
    Zz246 = 0x04,
    #[doc = "YADR inversed. YADR is inversed after reading one word or after programming one word when YE falls."]
    Zz247 = 0x05,
    #[doc = "YADR\\[0\\] inversed. YADR\\[0\\] is inversed after reading one word or after programming one word when YE falls."]
    Zz248 = 0x06,
    #[doc = "YADR increased by 1 after last row. For READ operations only, YADR is increased by 1 after XADR reaches last row."]
    Zz249 = 0x07,
    #[doc = "YADR decreased by 1. For READ operations only, YADR is decreased by 1 after each read cycle."]
    Zz250 = 0x08,
    #[doc = "YADR decreased by 1 after first row. For READ operations only, YADR is decreased by 1 after XADR decreases to 0."]
    Zz251 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl DftYadr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DftYadr {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DftYadr {
    #[inline(always)]
    fn from(val: u8) -> DftYadr {
        DftYadr::from_bits(val)
    }
}
impl From<DftYadr> for u8 {
    #[inline(always)]
    fn from(val: DftYadr) -> u8 {
        DftYadr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisPrer {
    #[doc = "Enable pre-PV read before first program shot."]
    Zz345 = 0x0,
    #[doc = "Disable pre-PV read before first program shot."]
    Zz346 = 0x01,
}
impl DisPrer {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisPrer {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisPrer {
    #[inline(always)]
    fn from(val: u8) -> DisPrer {
        DisPrer::from_bits(val)
    }
}
impl From<DisPrer> for u8 {
    #[inline(always)]
    fn from(val: DisPrer) -> u8 {
        DisPrer::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccEnableb {
    #[doc = "ECC decoder enabled (default)."]
    Zz189 = 0x0,
    #[doc = "ECC decoder disabled."]
    Zz190 = 0x01,
}
impl EccEnableb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccEnableb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccEnableb {
    #[inline(always)]
    fn from(val: u8) -> EccEnableb {
        EccEnableb::from_bits(val)
    }
}
impl From<EccEnableb> for u8 {
    #[inline(always)]
    fn from(val: EccEnableb) -> u8 {
        EccEnableb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Eccen {
    #[doc = "Default mode (no ECC encode or decode)."]
    Zz207 = 0x0,
    #[doc = "Enable ECC encode/decode."]
    Zz208 = 0x01,
}
impl Eccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Eccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Eccen {
    #[inline(always)]
    fn from(val: u8) -> Eccen {
        Eccen::from_bits(val)
    }
}
impl From<Eccen> for u8 {
    #[inline(always)]
    fn from(val: Eccen) -> u8 {
        Eccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersaack {
    #[doc = "Mass Erase operation is not active (operation has completed or has not started)."]
    Zz111 = 0x0,
    #[doc = "Mass Erase operation is active (controller acknowledges that the soc_ersall_req input is asserted and will continue with the operation)."]
    Zz112 = 0x01,
}
impl Ersaack {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersaack {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersaack {
    #[inline(always)]
    fn from(val: u8) -> Ersaack {
        Ersaack::from_bits(val)
    }
}
impl From<Ersaack> for u8 {
    #[inline(always)]
    fn from(val: Ersaack) -> u8 {
        Ersaack::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien0 {
    #[doc = "Block 0 IFR Sector X is protected from erase by ERSSCR command."]
    Zz31 = 0x0,
    #[doc = "Block 0 IFR Sector X is not protected from erase by ERSSCR command."]
    Zz32 = 0x01,
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
impl Ersien0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien0 {
    #[inline(always)]
    fn from(val: u8) -> Ersien0 {
        Ersien0::from_bits(val)
    }
}
impl From<Ersien0> for u8 {
    #[inline(always)]
    fn from(val: Ersien0) -> u8 {
        Ersien0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersien1 {
    #[doc = "Block 1 IFR Sector X is protected from erase by ERSSCR command."]
    Zz29 = 0x0,
    #[doc = "Block 1 IFR Sector X is not protected from erase by ERSSCR command."]
    Zz30 = 0x01,
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
impl Ersien1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersien1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersien1 {
    #[inline(always)]
    fn from(val: u8) -> Ersien1 {
        Ersien1::from_bits(val)
    }
}
impl From<Ersien1> for u8 {
    #[inline(always)]
    fn from(val: Ersien1) -> u8 {
        Ersien1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ersreq {
    #[doc = "No request or request complete."]
    Zz35 = 0x0,
    #[doc = "Request to run the Mass Erase operation."]
    Zz36 = 0x01,
}
impl Ersreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ersreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ersreq {
    #[inline(always)]
    fn from(val: u8) -> Ersreq {
        Ersreq::from_bits(val)
    }
}
impl From<Ersreq> for u8 {
    #[inline(always)]
    fn from(val: Ersreq) -> u8 {
        Ersreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fail {
    #[doc = "Error not detected."]
    Zz27 = 0x0,
    #[doc = "Error detected."]
    Zz28 = 0x01,
}
impl Fail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fail {
    #[inline(always)]
    fn from(val: u8) -> Fail {
        Fail::from_bits(val)
    }
}
impl From<Fail> for u8 {
    #[inline(always)]
    fn from(val: Fail) -> u8 {
        Fail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Fdfd {
    #[doc = "FSTAT\\[DFDIF\\] sets only if a double bit fault is detected during a valid flash read access from the FMC."]
    Zz41 = 0x0,
    #[doc = "FSTAT\\[DFDIF\\] sets during any valid flash read access from the FMC; an interrupt request is generated if the DFDIE bit is set."]
    Zz42 = 0x01,
}
impl Fdfd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Fdfd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Fdfd {
    #[inline(always)]
    fn from(val: u8) -> Fdfd {
        Fdfd::from_bits(val)
    }
}
impl From<Fdfd> for u8 {
    #[inline(always)]
    fn from(val: Fdfd) -> u8 {
        Fdfd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashRd {
    #[doc = "Manual flash read not enabled.(default)."]
    Zz155 = 0x0,
    #[doc = "Manual flash read enabled."]
    Zz156 = 0x01,
}
impl FlashRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashRd {
    #[inline(always)]
    fn from(val: u8) -> FlashRd {
        FlashRd::from_bits(val)
    }
}
impl From<FlashRd> for u8 {
    #[inline(always)]
    fn from(val: FlashRd) -> u8 {
        FlashRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuEccCtl {
    #[doc = "ECC is enabled for FMU program operations."]
    Zz99 = 0x0,
    #[doc = "ECC is disabled for FMU program operations."]
    Zz100 = 0x01,
}
impl FmuEccCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuEccCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuEccCtl {
    #[inline(always)]
    fn from(val: u8) -> FmuEccCtl {
        FmuEccCtl::from_bits(val)
    }
}
impl From<FmuEccCtl> for u8 {
    #[inline(always)]
    fn from(val: FmuEccCtl) -> u8 {
        FmuEccCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuParmDone {
    #[doc = "FMU registers have not been loaded."]
    Zz89 = 0x0,
    #[doc = "FMU registers have been loaded."]
    Zz90 = 0x01,
}
impl FmuParmDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuParmDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuParmDone {
    #[inline(always)]
    fn from(val: u8) -> FmuParmDone {
        FmuParmDone::from_bits(val)
    }
}
impl From<FmuParmDone> for u8 {
    #[inline(always)]
    fn from(val: FmuParmDone) -> u8 {
        FmuParmDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuParmEn {
    #[doc = "C0DE_C0DEh check not attempted."]
    Zz91 = 0x0,
    #[doc = "C0DE_C0DEh check completed."]
    Zz92 = 0x01,
}
impl FmuParmEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuParmEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuParmEn {
    #[inline(always)]
    fn from(val: u8) -> FmuParmEn {
        FmuParmEn::from_bits(val)
    }
}
impl From<FmuParmEn> for u8 {
    #[inline(always)]
    fn from(val: FmuParmEn) -> u8 {
        FmuParmEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceSwClk {
    #[doc = "Switch clock not forced on (gated normally)."]
    Zz157 = 0x0,
    #[doc = "Switch clock forced on."]
    Zz158 = 0x01,
}
impl ForceSwClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceSwClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceSwClk {
    #[inline(always)]
    fn from(val: u8) -> ForceSwClk {
        ForceSwClk::from_bits(val)
    }
}
impl From<ForceSwClk> for u8 {
    #[inline(always)]
    fn from(val: ForceSwClk) -> u8 {
        ForceSwClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Grpsel {
    #[doc = "Select no data."]
    Zz270 = 0x0,
    #[doc = "Select data slice \\[34:0\\]."]
    Zz271 = 0x01,
    #[doc = "Select data slice \\[69:35\\]."]
    Zz272 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "Select data slice \\[104:70\\]."]
    Zz273 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    #[doc = "Select data slice \\[136:105\\]."]
    Zz274 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    #[doc = "Select data \\[136:0\\]."]
    Zz275 = 0x0f,
}
impl Grpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Grpsel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Grpsel {
    #[inline(always)]
    fn from(val: u8) -> Grpsel {
        Grpsel::from_bits(val)
    }
}
impl From<Grpsel> for u8 {
    #[inline(always)]
    fn from(val: Grpsel) -> u8 {
        Grpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IfrCmd {
    #[doc = "The command operates on a main flash address."]
    Zz139 = 0x0,
    #[doc = "The command operates on an IFR address."]
    Zz140 = 0x01,
}
impl IfrCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IfrCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IfrCmd {
    #[inline(always)]
    fn from(val: u8) -> IfrCmd {
        IfrCmd::from_bits(val)
    }
}
impl From<IfrCmd> for u8 {
    #[inline(always)]
    fn from(val: IfrCmd) -> u8 {
        IfrCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IllegalCmd {
    #[doc = "Command is legal."]
    Zz129 = 0x0,
    #[doc = "Command is illegal."]
    Zz130 = 0x01,
}
impl IllegalCmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IllegalCmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IllegalCmd {
    #[inline(always)]
    fn from(val: u8) -> IllegalCmd {
        IllegalCmd::from_bits(val)
    }
}
impl From<IllegalCmd> for u8 {
    #[inline(always)]
    fn from(val: IllegalCmd) -> u8 {
        IllegalCmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InitDone {
    #[doc = "All initialization steps did not complete."]
    Zz79 = 0x0,
    #[doc = "All initialization steps completed."]
    Zz80 = 0x01,
}
impl InitDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> InitDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for InitDone {
    #[inline(always)]
    fn from(val: u8) -> InitDone {
        InitDone::from_bits(val)
    }
}
impl From<InitDone> for u8 {
    #[inline(always)]
    fn from(val: InitDone) -> u8 {
        InitDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipsel0 {
    #[doc = "Unselect block 0."]
    Zz223 = 0x0,
    #[doc = "not used, reserved."]
    Zz224 = 0x01,
    #[doc = "Enable block 0 test, repair off (default)."]
    Zz225 = 0x02,
    #[doc = "Enable block 0 test, repair on."]
    Zz226 = 0x03,
}
impl Ipsel0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipsel0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipsel0 {
    #[inline(always)]
    fn from(val: u8) -> Ipsel0 {
        Ipsel0::from_bits(val)
    }
}
impl From<Ipsel0> for u8 {
    #[inline(always)]
    fn from(val: Ipsel0) -> u8 {
        Ipsel0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipsel1 {
    #[doc = "Unselect block 1."]
    Zz219 = 0x0,
    #[doc = "not used, reserved."]
    Zz220 = 0x01,
    #[doc = "Enable block 1 test, repair off (default)."]
    Zz221 = 0x02,
    #[doc = "Enable block 1 test, repair on."]
    Zz222 = 0x03,
}
impl Ipsel1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipsel1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipsel1 {
    #[inline(always)]
    fn from(val: u8) -> Ipsel1 {
        Ipsel1::from_bits(val)
    }
}
impl From<Ipsel1> for u8 {
    #[inline(always)]
    fn from(val: Ipsel1) -> u8 {
        Ipsel1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LastRead {
    #[doc = "Latest read not last in multi-address operation."]
    Zz167 = 0x0,
    #[doc = "Latest read last in multi-address operation."]
    Zz168 = 0x01,
}
impl LastRead {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LastRead {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LastRead {
    #[inline(always)]
    fn from(val: u8) -> LastRead {
        LastRead::from_bits(val)
    }
}
impl From<LastRead> for u8 {
    #[inline(always)]
    fn from(val: LastRead) -> u8 {
        LastRead::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopopt {
    #[doc = "Loop is disabled; selected BIST operation is run once."]
    Zz284 = 0x0,
    #[doc = "Loop is enabled; XADR increments by 1 XADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz285 = 0x01,
    #[doc = "Loop is enabled; YADR increments by 1 YADR increments by 1 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz286 = 0x02,
    #[doc = "Loop is enabled; XADR increments by 2 XADR increments by 2 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz287 = 0x03,
    #[doc = "Loop is enabled; XADR increments by sector XADR increments by 16 for each new loop. Stops when total loop count meets LOOPCNT+1."]
    Zz288 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Loopopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopopt {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopopt {
    #[inline(always)]
    fn from(val: u8) -> Loopopt {
        Loopopt::from_bits(val)
    }
}
impl From<Loopopt> for u8 {
    #[inline(always)]
    fn from(val: Loopopt) -> u8 {
        Loopopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Loopunit {
    #[doc = "Clock cycles."]
    Zz276 = 0x0,
    #[doc = "0.5 usec."]
    Zz277 = 0x01,
    #[doc = "1 usec."]
    Zz278 = 0x02,
    #[doc = "10 usec."]
    Zz279 = 0x03,
    #[doc = "100 usec."]
    Zz280 = 0x04,
    #[doc = "1 msec."]
    Zz281 = 0x05,
    #[doc = "10 msec."]
    Zz282 = 0x06,
    #[doc = "100 msec."]
    Zz283 = 0x07,
}
impl Loopunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Loopunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Loopunit {
    #[inline(always)]
    fn from(val: u8) -> Loopunit {
        Loopunit::from_bits(val)
    }
}
impl From<Loopunit> for u8 {
    #[inline(always)]
    fn from(val: Loopunit) -> u8 {
        Loopunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LsactEn {
    #[doc = "LSACTIVE feature disabled completely: FCTRL\\[LSACTIVE\\] is forced low and no longer writable, LVE cannot assert at the TSMC array interface."]
    Zz123 = 0x0,
    #[doc = "LSACTIVE feature fully enabled and controllable by SoC and internal UINT SM."]
    Zz124 = 0x01,
}
impl LsactEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LsactEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LsactEn {
    #[inline(always)]
    fn from(val: u8) -> LsactEn {
        LsactEn::from_bits(val)
    }
}
impl From<LsactEn> for u8 {
    #[inline(always)]
    fn from(val: LsactEn) -> u8 {
        LsactEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsactive {
    #[doc = "Full speed active mode requested."]
    Zz43 = 0x0,
    #[doc = "Low speed active mode requested."]
    Zz44 = 0x01,
}
impl Lsactive {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsactive {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsactive {
    #[inline(always)]
    fn from(val: u8) -> Lsactive {
        Lsactive::from_bits(val)
    }
}
impl From<Lsactive> for u8 {
    #[inline(always)]
    fn from(val: Lsactive) -> u8 {
        Lsactive::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lsactwren {
    #[doc = "Unrestricted write access allowed."]
    Zz121 = 0x0,
    #[doc = "Write access while CMP set must match CMDDID and CMDPRT."]
    Zz122 = 0x01,
}
impl Lsactwren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lsactwren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lsactwren {
    #[inline(always)]
    fn from(val: u8) -> Lsactwren {
        Lsactwren::from_bits(val)
    }
}
impl From<Lsactwren> for u8 {
    #[inline(always)]
    fn from(val: Lsactwren) -> u8 {
        Lsactwren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mask0Opt {
    #[doc = "Mask programmed bits passing PV until extra shot."]
    Zz347 = 0x0,
    #[doc = "Always program bits even if they pass PV."]
    Zz348 = 0x01,
}
impl Mask0Opt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mask0Opt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mask0Opt {
    #[inline(always)]
    fn from(val: u8) -> Mask0Opt {
        Mask0Opt::from_bits(val)
    }
}
impl From<Mask0Opt> for u8 {
    #[inline(always)]
    fn from(val: Mask0Opt) -> u8 {
        Mask0Opt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MasterRepairEn {
    #[doc = "Repair disabled."]
    Zz119 = 0x0,
    #[doc = "Repair enable determined by bit 0 of each REPAIR register."]
    Zz120 = 0x01,
}
impl MasterRepairEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MasterRepairEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MasterRepairEn {
    #[inline(always)]
    fn from(val: u8) -> MasterRepairEn {
        MasterRepairEn::from_bits(val)
    }
}
impl From<MasterRepairEn> for u8 {
    #[inline(always)]
    fn from(val: MasterRepairEn) -> u8 {
        MasterRepairEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MisrEn {
    #[doc = "MISR option disabled (default)."]
    Zz187 = 0x0,
    #[doc = "MISR option enabled."]
    Zz188 = 0x01,
}
impl MisrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MisrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MisrEn {
    #[inline(always)]
    fn from(val: u8) -> MisrEn {
        MisrEn::from_bits(val)
    }
}
impl From<MisrEn> for u8 {
    #[inline(always)]
    fn from(val: MisrEn) -> u8 {
        MisrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MmRd {
    #[doc = "Write to register."]
    Zz161 = 0x0,
    #[doc = "Read register."]
    Zz162 = 0x01,
}
impl MmRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MmRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MmRd {
    #[inline(always)]
    fn from(val: u8) -> MmRd {
        MmRd::from_bits(val)
    }
}
impl From<MmRd> for u8 {
    #[inline(always)]
    fn from(val: MmRd) -> u8 {
        MmRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mrgrddis {
    #[doc = "Margin Read Settings are enabled."]
    Zz113 = 0x0,
    #[doc = "Margin Read Settings are disabled."]
    Zz114 = 0x01,
}
impl Mrgrddis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mrgrddis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mrgrddis {
    #[inline(always)]
    fn from(val: u8) -> Mrgrddis {
        Mrgrddis::from_bits(val)
    }
}
impl From<Mrgrddis> for u8 {
    #[inline(always)]
    fn from(val: Mrgrddis) -> u8 {
        Mrgrddis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OptionFail {
    #[doc = "Option check passes for read command or command is not a read command."]
    Zz131 = 0x0,
    #[doc = "Option check fails for read command."]
    Zz132 = 0x01,
}
impl OptionFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OptionFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OptionFail {
    #[inline(always)]
    fn from(val: u8) -> OptionFail {
        OptionFail::from_bits(val)
    }
}
impl From<OptionFail> for u8 {
    #[inline(always)]
    fn from(val: OptionFail) -> u8 {
        OptionFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscH {
    #[doc = "Use APB clock."]
    Zz95 = 0x0,
    #[doc = "Use a known fixed-frequency clock, e.g. 12 MHz."]
    Zz96 = 0x01,
}
impl OscH {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscH {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscH {
    #[inline(always)]
    fn from(val: u8) -> OscH {
        OscH::from_bits(val)
    }
}
impl From<OscH> for u8 {
    #[inline(always)]
    fn from(val: OscH) -> u8 {
        OscH::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PdTimerEn {
    #[doc = "BIST timer is not triggered during Power Down recovery."]
    Zz127 = 0x0,
    #[doc = "BIST timer is triggered during Power Down recovery (default behavior)."]
    Zz128 = 0x01,
}
impl PdTimerEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PdTimerEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PdTimerEn {
    #[inline(always)]
    fn from(val: u8) -> PdTimerEn {
        PdTimerEn::from_bits(val)
    }
}
impl From<PdTimerEn> for u8 {
    #[inline(always)]
    fn from(val: PdTimerEn) -> u8 {
        PdTimerEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Perdy {
    #[doc = "Program or sector erase command operation is not stalled."]
    Zz1 = 0x0,
    #[doc = "Program or sector erase command operation is stalled."]
    Zz2 = 0x01,
}
impl Perdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Perdy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Perdy {
    #[inline(always)]
    fn from(val: u8) -> Perdy {
        Perdy::from_bits(val)
    }
}
impl From<Perdy> for u8 {
    #[inline(always)]
    fn from(val: Perdy) -> u8 {
        Perdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pewen {
    #[doc = "Writes are not enabled."]
    Zz3 = 0x0,
    #[doc = "Writes are enabled for one flash or IFR phrase (phrase programming, sector erase)."]
    Zz4 = 0x01,
    #[doc = "Writes are enabled for one flash or IFR page (page programming)."]
    Zz5 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pewen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pewen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pewen {
    #[inline(always)]
    fn from(val: u8) -> Pewen {
        Pewen::from_bits(val)
    }
}
impl From<Pewen> for u8 {
    #[inline(always)]
    fn from(val: Pewen) -> u8 {
        Pewen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostTers {
    #[doc = "50 usec."]
    Zz361 = 0x0,
    #[doc = "100 usec."]
    Zz362 = 0x01,
    #[doc = "200 usec."]
    Zz363 = 0x02,
    #[doc = "300 usec."]
    Zz364 = 0x03,
    #[doc = "500 usec."]
    Zz365 = 0x04,
    #[doc = "1 msec."]
    Zz366 = 0x05,
    #[doc = "1.5 msec."]
    Zz367 = 0x06,
    #[doc = "2 msec."]
    Zz368 = 0x07,
}
impl PostTers {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostTers {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostTers {
    #[inline(always)]
    fn from(val: u8) -> PostTers {
        PostTers::from_bits(val)
    }
}
impl From<PostTers> for u8 {
    #[inline(always)]
    fn from(val: PostTers) -> u8 {
        PostTers::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PostTpgm {
    #[doc = "1 usec."]
    Zz357 = 0x0,
    #[doc = "2 usec."]
    Zz358 = 0x01,
    #[doc = "4 usec."]
    Zz359 = 0x02,
    #[doc = "8 usec."]
    Zz360 = 0x03,
}
impl PostTpgm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PostTpgm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PostTpgm {
    #[inline(always)]
    fn from(val: u8) -> PostTpgm {
        PostTpgm::from_bits(val)
    }
}
impl From<PostTpgm> for u8 {
    #[inline(always)]
    fn from(val: PostTpgm) -> u8 {
        PostTpgm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgAttr {
    #[doc = "One YE pulse will program one data slice group."]
    Zz262 = 0x0,
    #[doc = "One YE pulse will program two data slice groups."]
    Zz263 = 0x01,
    #[doc = "One YE pulse will program three data slice groups (reserved)."]
    Zz264 = 0x02,
    #[doc = "One YE pulse will program four data slice groups."]
    Zz265 = 0x03,
    #[doc = "One YE pulse will program five data slice groups (reserved)."]
    Zz266 = 0x04,
    #[doc = "One YE pulse will program six data slice groups (reserved)."]
    Zz267 = 0x05,
    #[doc = "One YE pulse will program seven data slice groups (reserved)."]
    Zz268 = 0x06,
    #[doc = "One YE pulse will program eight data slice groups (reserved)."]
    Zz269 = 0x07,
}
impl ProgAttr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ProgAttr {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ProgAttr {
    #[inline(always)]
    fn from(val: u8) -> ProgAttr {
        ProgAttr::from_bits(val)
    }
}
impl From<ProgAttr> for u8 {
    #[inline(always)]
    fn from(val: ProgAttr) -> u8 {
        ProgAttr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pviol {
    #[doc = "No protection violation detected."]
    Zz23 = 0x0,
    #[doc = "Protection violation detected."]
    Zz24 = 0x01,
}
impl Pviol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pviol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pviol {
    #[inline(always)]
    fn from(val: u8) -> Pviol {
        Pviol::from_bits(val)
    }
}
impl From<Pviol> for u8 {
    #[inline(always)]
    fn from(val: Pviol) -> u8 {
        Pviol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair00Rdis00 {
    #[doc = "Repair address is valid."]
    Zz337 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz338 = 0x01,
}
impl RRepair00Rdis00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair00Rdis00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair00Rdis00 {
    #[inline(always)]
    fn from(val: u8) -> RRepair00Rdis00 {
        RRepair00Rdis00::from_bits(val)
    }
}
impl From<RRepair00Rdis00> for u8 {
    #[inline(always)]
    fn from(val: RRepair00Rdis00) -> u8 {
        RRepair00Rdis00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair01Rdis01 {
    #[doc = "Repair address is valid."]
    Zz339 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz340 = 0x01,
}
impl RRepair01Rdis01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair01Rdis01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair01Rdis01 {
    #[inline(always)]
    fn from(val: u8) -> RRepair01Rdis01 {
        RRepair01Rdis01::from_bits(val)
    }
}
impl From<RRepair01Rdis01> for u8 {
    #[inline(always)]
    fn from(val: RRepair01Rdis01) -> u8 {
        RRepair01Rdis01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair10Rdis10 {
    #[doc = "Repair address is valid."]
    Zz341 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz342 = 0x01,
}
impl RRepair10Rdis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair10Rdis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair10Rdis10 {
    #[inline(always)]
    fn from(val: u8) -> RRepair10Rdis10 {
        RRepair10Rdis10::from_bits(val)
    }
}
impl From<RRepair10Rdis10> for u8 {
    #[inline(always)]
    fn from(val: RRepair10Rdis10) -> u8 {
        RRepair10Rdis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RRepair11Rdis11 {
    #[doc = "Repair address is valid."]
    Zz343 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz344 = 0x01,
}
impl RRepair11Rdis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RRepair11Rdis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RRepair11Rdis11 {
    #[inline(always)]
    fn from(val: u8) -> RRepair11Rdis11 {
        RRepair11Rdis11::from_bits(val)
    }
}
impl From<RRepair11Rdis11> for u8 {
    #[inline(always)]
    fn from(val: RRepair11Rdis11) -> u8 {
        RRepair11Rdis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RangeFail {
    #[doc = "The address range is valid."]
    Zz135 = 0x0,
    #[doc = "The address range is invalid."]
    Zz136 = 0x01,
}
impl RangeFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RangeFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RangeFail {
    #[inline(always)]
    fn from(val: u8) -> RangeFail {
        RangeFail::from_bits(val)
    }
}
impl From<RangeFail> for u8 {
    #[inline(always)]
    fn from(val: RangeFail) -> u8 {
        RangeFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RecallDataMismatch {
    #[doc = "Data read towards end of reset matched data read for Recall."]
    Zz71 = 0x0,
    #[doc = "Data read towards end of reset did not match data read for recall."]
    Zz72 = 0x01,
}
impl RecallDataMismatch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RecallDataMismatch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RecallDataMismatch {
    #[inline(always)]
    fn from(val: u8) -> RecallDataMismatch {
        RecallDataMismatch::from_bits(val)
    }
}
impl From<RecallDataMismatch> for u8 {
    #[inline(always)]
    fn from(val: RecallDataMismatch) -> u8 {
        RecallDataMismatch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair00Rdis00 {
    #[doc = "Repair address is valid."]
    Zz405 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz406 = 0x01,
}
impl Repair00Rdis00 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair00Rdis00 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair00Rdis00 {
    #[inline(always)]
    fn from(val: u8) -> Repair00Rdis00 {
        Repair00Rdis00::from_bits(val)
    }
}
impl From<Repair00Rdis00> for u8 {
    #[inline(always)]
    fn from(val: Repair00Rdis00) -> u8 {
        Repair00Rdis00::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair01Rdis01 {
    #[doc = "Repair address is valid."]
    Zz407 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz408 = 0x01,
}
impl Repair01Rdis01 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair01Rdis01 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair01Rdis01 {
    #[inline(always)]
    fn from(val: u8) -> Repair01Rdis01 {
        Repair01Rdis01::from_bits(val)
    }
}
impl From<Repair01Rdis01> for u8 {
    #[inline(always)]
    fn from(val: Repair01Rdis01) -> u8 {
        Repair01Rdis01::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair10Rdis10 {
    #[doc = "Repair address is valid."]
    Zz409 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz410 = 0x01,
}
impl Repair10Rdis10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair10Rdis10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair10Rdis10 {
    #[inline(always)]
    fn from(val: u8) -> Repair10Rdis10 {
        Repair10Rdis10::from_bits(val)
    }
}
impl From<Repair10Rdis10> for u8 {
    #[inline(always)]
    fn from(val: Repair10Rdis10) -> u8 {
        Repair10Rdis10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Repair11Rdis11 {
    #[doc = "Repair address is valid."]
    Zz411 = 0x0,
    #[doc = "Repair address is not valid."]
    Zz412 = 0x01,
}
impl Repair11Rdis11 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Repair11Rdis11 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Repair11Rdis11 {
    #[inline(always)]
    fn from(val: u8) -> Repair11Rdis11 {
        Repair11Rdis11::from_bits(val)
    }
}
impl From<Repair11Rdis11> for u8 {
    #[inline(always)]
    fn from(val: Repair11Rdis11) -> u8 {
        Repair11Rdis11::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rfcmden {
    #[doc = "Flash commands blocked (CCIF not writable)."]
    Zz117 = 0x0,
    #[doc = "Flash commands allowed."]
    Zz118 = 0x01,
}
impl Rfcmden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rfcmden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rfcmden {
    #[inline(always)]
    fn from(val: u8) -> Rfcmden {
        Rfcmden::from_bits(val)
    }
}
impl From<Rfcmden> for u8 {
    #[inline(always)]
    fn from(val: Rfcmden) -> u8 {
        Rfcmden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RprDone {
    #[doc = "Repair registers have not been loaded."]
    Zz81 = 0x0,
    #[doc = "Repair registers have been loaded."]
    Zz82 = 0x01,
}
impl RprDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RprDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RprDone {
    #[inline(always)]
    fn from(val: u8) -> RprDone {
        RprDone::from_bits(val)
    }
}
impl From<RprDone> for u8 {
    #[inline(always)]
    fn from(val: RprDone) -> u8 {
        RprDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstDfErr {
    #[doc = "No double-bit faults detected during initialization."]
    Zz75 = 0x0,
    #[doc = "Double-bit ECC fault was detected during initialization."]
    Zz76 = 0x01,
}
impl RstDfErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstDfErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstDfErr {
    #[inline(always)]
    fn from(val: u8) -> RstDfErr {
        RstDfErr::from_bits(val)
    }
}
impl From<RstDfErr> for u8 {
    #[inline(always)]
    fn from(val: RstDfErr) -> u8 {
        RstDfErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstPatchLd {
    #[doc = "No patch required to be loaded during reset."]
    Zz73 = 0x0,
    #[doc = "Patch loaded during reset."]
    Zz74 = 0x01,
}
impl RstPatchLd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstPatchLd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstPatchLd {
    #[inline(always)]
    fn from(val: u8) -> RstPatchLd {
        RstPatchLd::from_bits(val)
    }
}
impl From<RstPatchLd> for u8 {
    #[inline(always)]
    fn from(val: RstPatchLd) -> u8 {
        RstPatchLd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RstSfErr {
    #[doc = "No single-bit faults detected during initialization."]
    Zz77 = 0x0,
    #[doc = "At least one single ECC fault was detected during initialization."]
    Zz78 = 0x01,
}
impl RstSfErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RstSfErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RstSfErr {
    #[inline(always)]
    fn from(val: u8) -> RstSfErr {
        RstSfErr::from_bits(val)
    }
}
impl From<RstSfErr> for u8 {
    #[inline(always)]
    fn from(val: RstSfErr) -> u8 {
        RstSfErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwsc {
    #[doc = "no additional wait-states are added (single cycle access)."]
    Zz45 = 0x0,
    #[doc = "1 additional wait-state is added."]
    Zz46 = 0x01,
    #[doc = "2 additional wait-states are added."]
    Zz47 = 0x02,
    #[doc = "3 additional wait-states are added."]
    Zz48 = 0x03,
    #[doc = "4 additional wait-states are added."]
    Zz49 = 0x04,
    #[doc = "5 additional wait-states are added."]
    Zz50 = 0x05,
    #[doc = "6 additional wait-states are added."]
    Zz51 = 0x06,
    #[doc = "7 additional wait-states are added."]
    Zz52 = 0x07,
    #[doc = "8 additional wait-states are added."]
    Zz53 = 0x08,
    #[doc = "9 additional wait-states are added."]
    Zz54 = 0x09,
    #[doc = "10 additional wait-states are added."]
    Zz55 = 0x0a,
    #[doc = "11 additional wait-states are added."]
    Zz56 = 0x0b,
    #[doc = "12 additional wait-states are added."]
    Zz57 = 0x0c,
    #[doc = "13 additional wait-states are added."]
    Zz58 = 0x0d,
    #[doc = "14 additional wait-states are added."]
    Zz59 = 0x0e,
    #[doc = "15 additional wait-states are added."]
    Zz60 = 0x0f,
}
impl Rwsc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwsc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwsc {
    #[inline(always)]
    fn from(val: u8) -> Rwsc {
        Rwsc::from_bits(val)
    }
}
impl From<Rwsc> for u8 {
    #[inline(always)]
    fn from(val: Rwsc) -> u8 {
        Rwsc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SalvDis {
    #[doc = "Salvage enabled (ECC used during erase verify)."]
    Zz103 = 0x0,
    #[doc = "Salvage disabled (ECC not used during erase verify)."]
    Zz104 = 0x01,
}
impl SalvDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SalvDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SalvDis {
    #[inline(always)]
    fn from(val: u8) -> SalvDis {
        SalvDis::from_bits(val)
    }
}
impl From<SalvDis> for u8 {
    #[inline(always)]
    fn from(val: SalvDis) -> u8 {
        SalvDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SalvUsed {
    #[doc = "Salvage not used during the last operation."]
    Zz7 = 0x0,
    #[doc = "Salvage used during the last erase operation."]
    Zz8 = 0x01,
}
impl SalvUsed {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SalvUsed {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SalvUsed {
    #[inline(always)]
    fn from(val: u8) -> SalvUsed {
        SalvUsed::from_bits(val)
    }
}
impl From<SalvUsed> for u8 {
    #[inline(always)]
    fn from(val: SalvUsed) -> u8 {
        SalvUsed::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SberrReg {
    #[doc = "Single-bit fault not detected."]
    Zz175 = 0x0,
    #[doc = "Single-bit fault detected on previous UINT flash read."]
    Zz176 = 0x01,
}
impl SberrReg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SberrReg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SberrReg {
    #[inline(always)]
    fn from(val: u8) -> SberrReg {
        SberrReg::from_bits(val)
    }
}
impl From<SberrReg> for u8 {
    #[inline(always)]
    fn from(val: SberrReg) -> u8 {
        SberrReg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScanObs {
    #[doc = "Normal functional behavior."]
    Zz109 = 0x0,
    #[doc = "Enables observation of signals that may otherwise be ATPG untestable."]
    Zz110 = 0x01,
}
impl ScanObs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScanObs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScanObs {
    #[inline(always)]
    fn from(val: u8) -> ScanObs {
        ScanObs::from_bits(val)
    }
}
impl From<ScanObs> for u8 {
    #[inline(always)]
    fn from(val: ScanObs) -> u8 {
        ScanObs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ScrAlignChk {
    #[doc = "No sector alignment check."]
    Zz133 = 0x0,
    #[doc = "Sector alignment check."]
    Zz134 = 0x01,
}
impl ScrAlignChk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ScrAlignChk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ScrAlignChk {
    #[inline(always)]
    fn from(val: u8) -> ScrAlignChk {
        ScrAlignChk::from_bits(val)
    }
}
impl From<ScrAlignChk> for u8 {
    #[inline(always)]
    fn from(val: ScrAlignChk) -> u8 {
        ScrAlignChk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SetFail {
    #[doc = "FAIL flag should not be set on command exit (no failure detected)."]
    Zz165 = 0x0,
    #[doc = "FAIL flag should be set on command exit."]
    Zz166 = 0x01,
}
impl SetFail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SetFail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SetFail {
    #[inline(always)]
    fn from(val: u8) -> SetFail {
        SetFail::from_bits(val)
    }
}
impl From<SetFail> for u8 {
    #[inline(always)]
    fn from(val: SetFail) -> u8 {
        SetFail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleRd {
    #[doc = "Normal UINT operation."]
    Zz151 = 0x0,
    #[doc = "UINT configured for single cycle reads."]
    Zz152 = 0x01,
}
impl SingleRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleRd {
    #[inline(always)]
    fn from(val: u8) -> SingleRd {
        SingleRd::from_bits(val)
    }
}
impl From<SingleRd> for u8 {
    #[inline(always)]
    fn from(val: SingleRd) -> u8 {
        SingleRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwArray {
    #[doc = "Main array."]
    Zz425 = 0x0,
    #[doc = "IFR space only or main (and REDEN space) with IFR space for mass erase."]
    Zz426 = 0x01,
    #[doc = "IFR1 space."]
    Zz427 = 0x02,
    _RESERVED_3 = 0x03,
    #[doc = "REDEN space."]
    Zz428 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl SmwArray {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwArray {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwArray {
    #[inline(always)]
    fn from(val: u8) -> SmwArray {
        SmwArray::from_bits(val)
    }
}
impl From<SmwArray> for u8 {
    #[inline(always)]
    fn from(val: SmwArray) -> u8 {
        SmwArray::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwArray1Smw0Sel {
    #[doc = "Select block 0."]
    Zz171 = 0x0,
    #[doc = "Select block 1."]
    Zz172 = 0x01,
}
impl SmwArray1Smw0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwArray1Smw0Sel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwArray1Smw0Sel {
    #[inline(always)]
    fn from(val: u8) -> SmwArray1Smw0Sel {
        SmwArray1Smw0Sel::from_bits(val)
    }
}
impl From<SmwArray1Smw0Sel> for u8 {
    #[inline(always)]
    fn from(val: SmwArray1Smw0Sel) -> u8 {
        SmwArray1Smw0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwBusy {
    #[doc = "SMW command not active."]
    Zz203 = 0x0,
    #[doc = "SMW command is active."]
    Zz204 = 0x01,
}
impl SmwBusy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwBusy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwBusy {
    #[inline(always)]
    fn from(val: u8) -> SmwBusy {
        SmwBusy::from_bits(val)
    }
}
impl From<SmwBusy> for u8 {
    #[inline(always)]
    fn from(val: SmwBusy) -> u8 {
        SmwBusy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwErr {
    #[doc = "Error not detected."]
    Zz205 = 0x0,
    #[doc = "Error detected."]
    Zz206 = 0x01,
}
impl SmwErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwErr {
    #[inline(always)]
    fn from(val: u8) -> SmwErr {
        SmwErr::from_bits(val)
    }
}
impl From<SmwErr> for u8 {
    #[inline(always)]
    fn from(val: SmwErr) -> u8 {
        SmwErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmwrCtl {
    #[doc = "SMWR IP disabled."]
    Zz105 = 0x0,
    #[doc = "SMWR IP enabled."]
    Zz106 = 0x01,
}
impl SmwrCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmwrCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmwrCtl {
    #[inline(always)]
    fn from(val: u8) -> SmwrCtl {
        SmwrCtl::from_bits(val)
    }
}
impl From<SmwrCtl> for u8 {
    #[inline(always)]
    fn from(val: SmwrCtl) -> u8 {
        SmwrCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Smwtst {
    #[doc = "Default."]
    Zz209 = 0x0,
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 0."]
    Zz210 = 0x01,
    #[doc = "Enable SMWR self-test mode, DOUT from macro will be forced to all 1."]
    Zz211 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Smwtst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Smwtst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Smwtst {
    #[inline(always)]
    fn from(val: u8) -> Smwtst {
        Smwtst::from_bits(val)
    }
}
impl From<Smwtst> for u8 {
    #[inline(always)]
    fn from(val: Smwtst) -> u8 {
        Smwtst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocEccCtl {
    #[doc = "ECC is enabled for SOC read access."]
    Zz101 = 0x0,
    #[doc = "ECC is disabled for SOC read access."]
    Zz102 = 0x01,
}
impl SocEccCtl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocEccCtl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocEccCtl {
    #[inline(always)]
    fn from(val: u8) -> SocEccCtl {
        SocEccCtl::from_bits(val)
    }
}
impl From<SocEccCtl> for u8 {
    #[inline(always)]
    fn from(val: SocEccCtl) -> u8 {
        SocEccCtl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimDone {
    #[doc = "SoC Trim registers have not been updated."]
    Zz83 = 0x0,
    #[doc = "All SoC Trim registers have been updated."]
    Zz84 = 0x01,
}
impl SocTrimDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimDone {
    #[inline(always)]
    fn from(val: u8) -> SocTrimDone {
        SocTrimDone::from_bits(val)
    }
}
impl From<SocTrimDone> for u8 {
    #[inline(always)]
    fn from(val: SocTrimDone) -> u8 {
        SocTrimDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimEcc {
    #[doc = "C0DE_C0DEh check failed."]
    Zz85 = 0x0,
    #[doc = "C0DE_C0DEh check passed."]
    Zz86 = 0x01,
}
impl SocTrimEcc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimEcc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimEcc {
    #[inline(always)]
    fn from(val: u8) -> SocTrimEcc {
        SocTrimEcc::from_bits(val)
    }
}
impl From<SocTrimEcc> for u8 {
    #[inline(always)]
    fn from(val: SocTrimEcc) -> u8 {
        SocTrimEcc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SocTrimEn {
    #[doc = "C0DE_C0DEh check not attempted."]
    Zz87 = 0x0,
    #[doc = "C0DE_C0DEh check completed."]
    Zz88 = 0x01,
}
impl SocTrimEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SocTrimEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SocTrimEn {
    #[inline(always)]
    fn from(val: u8) -> SocTrimEn {
        SocTrimEn::from_bits(val)
    }
}
impl From<SocTrimEn> for u8 {
    #[inline(always)]
    fn from(val: SocTrimEn) -> u8 {
        SocTrimEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status0 {
    #[doc = "BIST test passed on flash block 0."]
    Zz331 = 0x0,
    #[doc = "BIST test failed on flash block 0."]
    Zz332 = 0x01,
}
impl Status0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status0 {
    #[inline(always)]
    fn from(val: u8) -> Status0 {
        Status0::from_bits(val)
    }
}
impl From<Status0> for u8 {
    #[inline(always)]
    fn from(val: Status0) -> u8 {
        Status0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Status1 {
    #[doc = "BIST test passed on flash block 1."]
    Zz329 = 0x0,
    #[doc = "BIST test failed on flash block 1."]
    Zz330 = 0x01,
}
impl Status1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Status1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Status1 {
    #[inline(always)]
    fn from(val: u8) -> Status1 {
        Status1::from_bits(val)
    }
}
impl From<Status1> for u8 {
    #[inline(always)]
    fn from(val: Status1) -> u8 {
        Status1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TersCtrl0 {
    #[doc = "50 usec."]
    Zz397 = 0x0,
    #[doc = "100 usec."]
    Zz398 = 0x01,
    #[doc = "200 usec."]
    Zz399 = 0x02,
    #[doc = "300 usec."]
    Zz400 = 0x03,
    #[doc = "500 usec."]
    Zz401 = 0x04,
    #[doc = "1 msec."]
    Zz402 = 0x05,
    #[doc = "1.5 msec."]
    Zz403 = 0x06,
    #[doc = "2 msec."]
    Zz404 = 0x07,
}
impl TersCtrl0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TersCtrl0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TersCtrl0 {
    #[inline(always)]
    fn from(val: u8) -> TersCtrl0 {
        TersCtrl0::from_bits(val)
    }
}
impl From<TersCtrl0> for u8 {
    #[inline(always)]
    fn from(val: TersCtrl0) -> u8 {
        TersCtrl0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tlvsunit {
    #[doc = "Clock cycles."]
    Zz289 = 0x0,
    #[doc = "0.5 usec."]
    Zz290 = 0x01,
    #[doc = "1 usec."]
    Zz291 = 0x02,
    #[doc = "10 usec."]
    Zz292 = 0x03,
    #[doc = "100 usec."]
    Zz293 = 0x04,
    #[doc = "1 msec."]
    Zz294 = 0x05,
    #[doc = "10 msec."]
    Zz295 = 0x06,
    #[doc = "100 msec."]
    Zz296 = 0x07,
}
impl Tlvsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tlvsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tlvsunit {
    #[inline(always)]
    fn from(val: u8) -> Tlvsunit {
        Tlvsunit::from_bits(val)
    }
}
impl From<Tlvsunit> for u8 {
    #[inline(always)]
    fn from(val: Tlvsunit) -> u8 {
        Tlvsunit::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct TmToAtx(u8);
impl TmToAtx {
    #[doc = "TM\\[0\\] to ATX0."]
    pub const Zz441: Self = Self(0x01);
    #[doc = "TM\\[1\\] to ATX0."]
    pub const Zz442: Self = Self(0x02);
    #[doc = "TM\\[2\\] to ATX0."]
    pub const Zz443: Self = Self(0x04);
    #[doc = "TM\\[3\\] to ATX0."]
    pub const Zz444: Self = Self(0x08);
    #[doc = "TM\\[0\\] to ATX1."]
    pub const Zz445: Self = Self(0x10);
    #[doc = "TM\\[1\\] to ATX1."]
    pub const Zz446: Self = Self(0x20);
    #[doc = "TM\\[2\\] to ATX1."]
    pub const Zz447: Self = Self(0x40);
    #[doc = "TM\\[3\\] to ATX1."]
    pub const Zz448: Self = Self(0x80);
}
impl TmToAtx {
    pub const fn from_bits(val: u8) -> TmToAtx {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for TmToAtx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Zz441"),
            0x02 => f.write_str("Zz442"),
            0x04 => f.write_str("Zz443"),
            0x08 => f.write_str("Zz444"),
            0x10 => f.write_str("Zz445"),
            0x20 => f.write_str("Zz446"),
            0x40 => f.write_str("Zz447"),
            0x80 => f.write_str("Zz448"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TmToAtx {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Zz441"),
            0x02 => defmt::write!(f, "Zz442"),
            0x04 => defmt::write!(f, "Zz443"),
            0x08 => defmt::write!(f, "Zz444"),
            0x10 => defmt::write!(f, "Zz445"),
            0x20 => defmt::write!(f, "Zz446"),
            0x40 => defmt::write!(f, "Zz447"),
            0x80 => defmt::write!(f, "Zz448"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for TmToAtx {
    #[inline(always)]
    fn from(val: u8) -> TmToAtx {
        TmToAtx::from_bits(val)
    }
}
impl From<TmToAtx> for u8 {
    #[inline(always)]
    fn from(val: TmToAtx) -> u8 {
        TmToAtx::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tme {
    #[doc = "Test mode entry not requested."]
    Zz65 = 0x0,
    #[doc = "Test mode entry requested."]
    Zz66 = 0x01,
}
impl Tme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tme {
    #[inline(always)]
    fn from(val: u8) -> Tme {
        Tme::from_bits(val)
    }
}
impl From<Tme> for u8 {
    #[inline(always)]
    fn from(val: Tme) -> u8 {
        Tme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmectl {
    #[doc = "FTEST register always reads 0 and writes to FTEST are ignored."]
    Zz69 = 0x0,
    #[doc = "FTEST register is readable and can be written to enable writability of TME."]
    Zz70 = 0x01,
}
impl Tmectl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmectl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmectl {
    #[inline(always)]
    fn from(val: u8) -> Tmectl {
        Tmectl::from_bits(val)
    }
}
impl From<Tmectl> for u8 {
    #[inline(always)]
    fn from(val: Tmectl) -> u8 {
        Tmectl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmelock {
    #[doc = "FTEST register not locked from accepting writes."]
    Zz61 = 0x0,
    #[doc = "FTEST register locked from accepting writes."]
    Zz62 = 0x01,
}
impl Tmelock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmelock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmelock {
    #[inline(always)]
    fn from(val: u8) -> Tmelock {
        Tmelock::from_bits(val)
    }
}
impl From<Tmelock> for u8 {
    #[inline(always)]
    fn from(val: Tmelock) -> u8 {
        Tmelock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmewr {
    #[doc = "TME bit is not writable."]
    Zz67 = 0x0,
    #[doc = "TME bit is writable."]
    Zz68 = 0x01,
}
impl Tmewr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmewr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmewr {
    #[inline(always)]
    fn from(val: u8) -> Tmewr {
        Tmewr::from_bits(val)
    }
}
impl From<Tmewr> for u8 {
    #[inline(always)]
    fn from(val: Tmewr) -> u8 {
        Tmewr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tmode {
    #[doc = "Test mode not active."]
    Zz63 = 0x0,
    #[doc = "Test mode active."]
    Zz64 = 0x01,
}
impl Tmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tmode {
    #[inline(always)]
    fn from(val: u8) -> Tmode {
        Tmode::from_bits(val)
    }
}
impl From<Tmode> for u8 {
    #[inline(always)]
    fn from(val: Tmode) -> u8 {
        Tmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnvhCtrl {
    #[doc = "2 usec."]
    Zz377 = 0x0,
    #[doc = "2.5 usec."]
    Zz378 = 0x01,
    #[doc = "3 usec."]
    Zz379 = 0x02,
    #[doc = "3.5 usec."]
    Zz380 = 0x03,
    #[doc = "4 usec."]
    Zz381 = 0x04,
    #[doc = "4.5 usec."]
    Zz382 = 0x05,
    #[doc = "5 usec."]
    Zz383 = 0x06,
    #[doc = "5.5 usec."]
    Zz384 = 0x07,
}
impl TnvhCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnvhCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnvhCtrl {
    #[inline(always)]
    fn from(val: u8) -> TnvhCtrl {
        TnvhCtrl::from_bits(val)
    }
}
impl From<TnvhCtrl> for u8 {
    #[inline(always)]
    fn from(val: TnvhCtrl) -> u8 {
        TnvhCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnvhunit {
    #[doc = "Clock cycles."]
    Zz313 = 0x0,
    #[doc = "0.5 usec."]
    Zz314 = 0x01,
    #[doc = "1 usec."]
    Zz315 = 0x02,
    #[doc = "10 usec."]
    Zz316 = 0x03,
    #[doc = "100 usec."]
    Zz317 = 0x04,
    #[doc = "1 msec."]
    Zz318 = 0x05,
    #[doc = "10 msec."]
    Zz319 = 0x06,
    #[doc = "100 msec."]
    Zz320 = 0x07,
}
impl Tnvhunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnvhunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnvhunit {
    #[inline(always)]
    fn from(val: u8) -> Tnvhunit {
        Tnvhunit::from_bits(val)
    }
}
impl From<Tnvhunit> for u8 {
    #[inline(always)]
    fn from(val: Tnvhunit) -> u8 {
        Tnvhunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TnvsCtrl {
    #[doc = "5 usec."]
    Zz385 = 0x0,
    #[doc = "8 usec."]
    Zz386 = 0x01,
    #[doc = "11 usec."]
    Zz387 = 0x02,
    #[doc = "14 usec."]
    Zz388 = 0x03,
    #[doc = "17 usec."]
    Zz389 = 0x04,
    #[doc = "20 usec."]
    Zz390 = 0x05,
    #[doc = "23 usec."]
    Zz391 = 0x06,
    #[doc = "26 usec."]
    Zz392 = 0x07,
}
impl TnvsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TnvsCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TnvsCtrl {
    #[inline(always)]
    fn from(val: u8) -> TnvsCtrl {
        TnvsCtrl::from_bits(val)
    }
}
impl From<TnvsCtrl> for u8 {
    #[inline(always)]
    fn from(val: TnvsCtrl) -> u8 {
        TnvsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnvsunit {
    #[doc = "Clock cycles."]
    Zz321 = 0x0,
    #[doc = "0.5 usec."]
    Zz322 = 0x01,
    #[doc = "1 usec."]
    Zz323 = 0x02,
    #[doc = "10 usec."]
    Zz324 = 0x03,
    #[doc = "100 usec."]
    Zz325 = 0x04,
    #[doc = "1 msec."]
    Zz326 = 0x05,
    #[doc = "10 msec."]
    Zz327 = 0x06,
    #[doc = "100 msec."]
    Zz328 = 0x07,
}
impl Tnvsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnvsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnvsunit {
    #[inline(always)]
    fn from(val: u8) -> Tnvsunit {
        Tnvsunit::from_bits(val)
    }
}
impl From<Tnvsunit> for u8 {
    #[inline(always)]
    fn from(val: Tnvsunit) -> u8 {
        Tnvsunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgmCtrl {
    #[doc = "1 usec."]
    Zz393 = 0x0,
    #[doc = "2 usec."]
    Zz394 = 0x01,
    #[doc = "4 usec."]
    Zz395 = 0x02,
    #[doc = "8 usec."]
    Zz396 = 0x03,
}
impl TpgmCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgmCtrl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgmCtrl {
    #[inline(always)]
    fn from(val: u8) -> TpgmCtrl {
        TpgmCtrl::from_bits(val)
    }
}
impl From<TpgmCtrl> for u8 {
    #[inline(always)]
    fn from(val: TpgmCtrl) -> u8 {
        TpgmCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgmOpt {
    #[doc = "Fixed Tpgm for all shots, except post shot."]
    Zz349 = 0x0,
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 4 usec."]
    Zz350 = 0x01,
    #[doc = "Increase Tpgm option by 1 for each loop until Tpgm reaches 8 usec."]
    Zz351 = 0x02,
    #[doc = "Unused."]
    Zz352 = 0x03,
}
impl TpgmOpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgmOpt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgmOpt {
    #[inline(always)]
    fn from(val: u8) -> TpgmOpt {
        TpgmOpt::from_bits(val)
    }
}
impl From<TpgmOpt> for u8 {
    #[inline(always)]
    fn from(val: TpgmOpt) -> u8 {
        TpgmOpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TpgsCtrl {
    #[doc = "1 usec."]
    Zz369 = 0x0,
    #[doc = "2 usec."]
    Zz370 = 0x01,
    #[doc = "3 usec."]
    Zz371 = 0x02,
    #[doc = "4 usec."]
    Zz372 = 0x03,
    #[doc = "5 usec."]
    Zz373 = 0x04,
    #[doc = "6 usec."]
    Zz374 = 0x05,
    #[doc = "7 usec."]
    Zz375 = 0x06,
    #[doc = "8 usec."]
    Zz376 = 0x07,
}
impl TpgsCtrl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TpgsCtrl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TpgsCtrl {
    #[inline(always)]
    fn from(val: u8) -> TpgsCtrl {
        TpgsCtrl::from_bits(val)
    }
}
impl From<TpgsCtrl> for u8 {
    #[inline(always)]
    fn from(val: TpgsCtrl) -> u8 {
        TpgsCtrl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpgsunit {
    #[doc = "Clock cycles."]
    Zz305 = 0x0,
    #[doc = "0.5 usec."]
    Zz306 = 0x01,
    #[doc = "1 usec."]
    Zz307 = 0x02,
    #[doc = "10 usec."]
    Zz308 = 0x03,
    #[doc = "100 usec."]
    Zz309 = 0x04,
    #[doc = "1 msec."]
    Zz310 = 0x05,
    #[doc = "10 msec."]
    Zz311 = 0x06,
    #[doc = "100 msec."]
    Zz312 = 0x07,
}
impl Tpgsunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpgsunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpgsunit {
    #[inline(always)]
    fn from(val: u8) -> Tpgsunit {
        Tpgsunit::from_bits(val)
    }
}
impl From<Tpgsunit> for u8 {
    #[inline(always)]
    fn from(val: Tpgsunit) -> u8 {
        Tpgsunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trcvunit {
    #[doc = "Clock cycles."]
    Zz297 = 0x0,
    #[doc = "0.5 usec."]
    Zz298 = 0x01,
    #[doc = "1 usec."]
    Zz299 = 0x02,
    #[doc = "10 usec."]
    Zz300 = 0x03,
    #[doc = "100 usec."]
    Zz301 = 0x04,
    #[doc = "1 msec."]
    Zz302 = 0x05,
    #[doc = "10 msec."]
    Zz303 = 0x06,
    #[doc = "100 msec."]
    Zz304 = 0x07,
}
impl Trcvunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trcvunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trcvunit {
    #[inline(always)]
    fn from(val: u8) -> Trcvunit {
        Trcvunit::from_bits(val)
    }
}
impl From<Trcvunit> for u8 {
    #[inline(always)]
    fn from(val: Trcvunit) -> u8 {
        Trcvunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tstctl {
    #[doc = "Default, disable both BIST self-test and MISR."]
    Zz215 = 0x0,
    #[doc = "Enable BIST self-test mode DOUT from macro will be forced to '0', and disable MISR."]
    Zz216 = 0x01,
    #[doc = "Enable MISR."]
    Zz217 = 0x02,
    #[doc = "Enable both BIST self-test mode and MISR."]
    Zz218 = 0x03,
}
impl Tstctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tstctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tstctl {
    #[inline(always)]
    fn from(val: u8) -> Tstctl {
        Tstctl::from_bits(val)
    }
}
impl From<Tstctl> for u8 {
    #[inline(always)]
    fn from(val: Tstctl) -> u8 {
        Tstctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserEv {
    #[doc = "EV input to the flash array is driven LOW."]
    Zz419 = 0x0,
    #[doc = "EV input to the flash array is driven HIGH."]
    Zz420 = 0x01,
}
impl UserEv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserEv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserEv {
    #[inline(always)]
    fn from(val: u8) -> UserEv {
        UserEv::from_bits(val)
    }
}
impl From<UserEv> for u8 {
    #[inline(always)]
    fn from(val: UserEv) -> u8 {
        UserEv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserHem {
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven LOW."]
    Zz413 = 0x0,
    #[doc = "HEM input to SMW / BIST PIN_CTRL\\[24\\] is driven HIGH."]
    Zz414 = 0x01,
}
impl UserHem {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserHem {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserHem {
    #[inline(always)]
    fn from(val: u8) -> UserHem {
        UserHem::from_bits(val)
    }
}
impl From<UserHem> for u8 {
    #[inline(always)]
    fn from(val: UserHem) -> u8 {
        UserHem::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserIfren {
    #[doc = "IFREN input to the flash array is driven LOW."]
    Zz417 = 0x0,
    #[doc = "IFREN input to the flash array is driven HIGH."]
    Zz418 = 0x01,
}
impl UserIfren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserIfren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserIfren {
    #[inline(always)]
    fn from(val: u8) -> UserIfren {
        UserIfren::from_bits(val)
    }
}
impl From<UserIfren> for u8 {
    #[inline(always)]
    fn from(val: UserIfren) -> u8 {
        UserIfren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserIfren1 {
    #[doc = "IFREN1 input to the flash array is driven LOW."]
    Zz423 = 0x0,
    #[doc = "IFREN1 input to the flash array is driven HIGH."]
    Zz424 = 0x01,
}
impl UserIfren1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserIfren1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserIfren1 {
    #[inline(always)]
    fn from(val: u8) -> UserIfren1 {
        UserIfren1::from_bits(val)
    }
}
impl From<UserIfren1> for u8 {
    #[inline(always)]
    fn from(val: UserIfren1) -> u8 {
        UserIfren1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserPv {
    #[doc = "PV input to the flash array is driven LOW."]
    Zz421 = 0x0,
    #[doc = "PV input to the flash array is driven HIGH."]
    Zz422 = 0x01,
}
impl UserPv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserPv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserPv {
    #[inline(always)]
    fn from(val: u8) -> UserPv {
        UserPv::from_bits(val)
    }
}
impl From<UserPv> for u8 {
    #[inline(always)]
    fn from(val: UserPv) -> u8 {
        UserPv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UserReden {
    #[doc = "REDEN input to the flash array is driven LOW."]
    Zz415 = 0x0,
    #[doc = "REDEN input to the flash array is driven HIGH."]
    Zz416 = 0x01,
}
impl UserReden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UserReden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UserReden {
    #[inline(always)]
    fn from(val: u8) -> UserReden {
        UserReden::from_bits(val)
    }
}
impl From<UserReden> for u8 {
    #[inline(always)]
    fn from(val: UserReden) -> u8 {
        UserReden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VfyOpt {
    #[doc = "Skip verify for post shot only, verify for all other shots."]
    Zz353 = 0x0,
    #[doc = "Skip verify for the 1st and post shots."]
    Zz354 = 0x01,
    #[doc = "Skip the 1st, 2nd, and post shots."]
    Zz355 = 0x02,
    #[doc = "Skip verify for all shots."]
    Zz356 = 0x03,
}
impl VfyOpt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VfyOpt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VfyOpt {
    #[inline(always)]
    fn from(val: u8) -> VfyOpt {
        VfyOpt::from_bits(val)
    }
}
impl From<VfyOpt> for u8 {
    #[inline(always)]
    fn from(val: VfyOpt) -> u8 {
        VfyOpt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WaitEn {
    #[doc = "Wait feature disabled."]
    Zz191 = 0x0,
    #[doc = "Wait feature enabled."]
    Zz192 = 0x01,
}
impl WaitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WaitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WaitEn {
    #[inline(always)]
    fn from(val: u8) -> WaitEn {
        WaitEn::from_bits(val)
    }
}
impl From<WaitEn> for u8 {
    #[inline(always)]
    fn from(val: WaitEn) -> u8 {
        WaitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WideLoad {
    #[doc = "Wide load mode disabled (default)."]
    Zz153 = 0x0,
    #[doc = "Wide load mode enabled."]
    Zz154 = 0x01,
}
impl WideLoad {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WideLoad {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WideLoad {
    #[inline(always)]
    fn from(val: u8) -> WideLoad {
        WideLoad::from_bits(val)
    }
}
impl From<WideLoad> for u8 {
    #[inline(always)]
    fn from(val: WideLoad) -> u8 {
        WideLoad::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrPathEccEn {
    #[doc = "ECC encoding disabled."]
    Zz179 = 0x0,
    #[doc = "ECC encoding enabled."]
    Zz180 = 0x01,
}
impl WrPathEccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrPathEccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrPathEccEn {
    #[inline(always)]
    fn from(val: u8) -> WrPathEccEn {
        WrPathEccEn::from_bits(val)
    }
}
impl From<WrPathEccEn> for u8 {
    #[inline(always)]
    fn from(val: WrPathEccEn) -> u8 {
        WrPathEccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WrPathEn {
    #[doc = "Writes to BIST setting registers driven by MM_WDATA."]
    Zz181 = 0x0,
    #[doc = "Writes to BIST setting registers driven by SMW_DIN."]
    Zz182 = 0x01,
}
impl WrPathEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WrPathEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WrPathEn {
    #[inline(always)]
    fn from(val: u8) -> WrPathEn {
        WrPathEn::from_bits(val)
    }
}
impl From<WrPathEn> for u8 {
    #[inline(always)]
    fn from(val: WrPathEn) -> u8 {
        WrPathEn::to_bits(val)
    }
}
