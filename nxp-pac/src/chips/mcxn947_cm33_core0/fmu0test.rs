#[doc = "FlashTest"]
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
    #[doc = "Flash Status Register"]
    #[inline(always)]
    pub const fn fstat(self) -> crate::common::Reg<regs::Fstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash Configuration Register"]
    #[inline(always)]
    pub const fn fcnfg(self) -> crate::common::Reg<regs::Fcnfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash Control Register"]
    #[inline(always)]
    pub const fn fctrl(self) -> crate::common::Reg<regs::Fctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Test Register"]
    #[inline(always)]
    pub const fn ftest(self) -> crate::common::Reg<regs::Ftest, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Flash Command Control 0 Register"]
    #[inline(always)]
    pub const fn fccob0(self) -> crate::common::Reg<regs::Fccob0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Flash Command Control 1 Register"]
    #[inline(always)]
    pub const fn fccob1(self) -> crate::common::Reg<regs::Fccob1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Flash Command Control 2 Register"]
    #[inline(always)]
    pub const fn fccob2(self) -> crate::common::Reg<regs::Fccob2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Flash Command Control 3 Register"]
    #[inline(always)]
    pub const fn fccob3(self) -> crate::common::Reg<regs::Fccob3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Flash Command Control 4 Register"]
    #[inline(always)]
    pub const fn fccob4(self) -> crate::common::Reg<regs::Fccob4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Flash Command Control 5 Register"]
    #[inline(always)]
    pub const fn fccob5(self) -> crate::common::Reg<regs::Fccob5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Flash Command Control 6 Register"]
    #[inline(always)]
    pub const fn fccob6(self) -> crate::common::Reg<regs::Fccob6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Flash Command Control 7 Register"]
    #[inline(always)]
    pub const fn fccob7(self) -> crate::common::Reg<regs::Fccob7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "FMU Initialization Tracking Register"]
    #[inline(always)]
    pub const fn reset_status(self) -> crate::common::Reg<regs::ResetStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FMU Control Register"]
    #[inline(always)]
    pub const fn mctl(self) -> crate::common::Reg<regs::Mctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "FMU Block Select Generation Register"]
    #[inline(always)]
    pub const fn bsel_gen(self) -> crate::common::Reg<regs::BselGen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Power Mode Options Register"]
    #[inline(always)]
    pub const fn pwr_opt(self) -> crate::common::Reg<regs::PwrOpt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "FMU Command Check Register"]
    #[inline(always)]
    pub const fn cmd_check(self) -> crate::common::Reg<regs::CmdCheck, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "FMU Block Select Register"]
    #[inline(always)]
    pub const fn bsel(self) -> crate::common::Reg<regs::Bsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "FMU Memory Size Register"]
    #[inline(always)]
    pub const fn msize(self) -> crate::common::Reg<regs::Msize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Flash Read Address Register"]
    #[inline(always)]
    pub const fn flash_rd_add(self) -> crate::common::Reg<regs::FlashRdAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Flash Stop Address Register"]
    #[inline(always)]
    pub const fn flash_stop_add(self) -> crate::common::Reg<regs::FlashStopAdd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Flash Read Control Register"]
    #[inline(always)]
    pub const fn flash_rd_ctrl(self) -> crate::common::Reg<regs::FlashRdCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Memory Map Address Register"]
    #[inline(always)]
    pub const fn mm_addr(self) -> crate::common::Reg<regs::MmAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Memory Map Write Data Register"]
    #[inline(always)]
    pub const fn mm_wdata(self) -> crate::common::Reg<regs::MmWdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "Memory Map Control Register"]
    #[inline(always)]
    pub const fn mm_ctl(self) -> crate::common::Reg<regs::MmCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "User Interface Control Register"]
    #[inline(always)]
    pub const fn uint_ctl(self) -> crate::common::Reg<regs::UintCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Read Data 0 Register"]
    #[inline(always)]
    pub const fn rd_data0(self) -> crate::common::Reg<regs::RdData0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "Read Data 1 Register"]
    #[inline(always)]
    pub const fn rd_data1(self) -> crate::common::Reg<regs::RdData1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Read Data 2 Register"]
    #[inline(always)]
    pub const fn rd_data2(self) -> crate::common::Reg<regs::RdData2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Read Data 3 Register"]
    #[inline(always)]
    pub const fn rd_data3(self) -> crate::common::Reg<regs::RdData3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Parity Register"]
    #[inline(always)]
    pub const fn parity(self) -> crate::common::Reg<regs::Parity, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Read Path Control and Status Register"]
    #[inline(always)]
    pub const fn rd_path_ctrl_status(
        self,
    ) -> crate::common::Reg<regs::RdPathCtrlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "SMW DIN 0 Register"]
    #[inline(always)]
    pub const fn smw_din0(self) -> crate::common::Reg<regs::SmwDin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "SMW DIN 1 Register"]
    #[inline(always)]
    pub const fn smw_din1(self) -> crate::common::Reg<regs::SmwDin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "SMW DIN 2 Register"]
    #[inline(always)]
    pub const fn smw_din2(self) -> crate::common::Reg<regs::SmwDin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "SMW DIN 3 Register"]
    #[inline(always)]
    pub const fn smw_din3(self) -> crate::common::Reg<regs::SmwDin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "SMW Address Register"]
    #[inline(always)]
    pub const fn smw_addr(self) -> crate::common::Reg<regs::SmwAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "SMW Command and Wait Register"]
    #[inline(always)]
    pub const fn smw_cmd_wait(self) -> crate::common::Reg<regs::SmwCmdWait, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "SMW Status Register"]
    #[inline(always)]
    pub const fn smw_status(self) -> crate::common::Reg<regs::SmwStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim0_0(self) -> crate::common::Reg<regs::Soctrim00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim0_1(self) -> crate::common::Reg<regs::Soctrim01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim0_2(self) -> crate::common::Reg<regs::Soctrim02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "SoC Trim Phrase 0 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim0_3(self) -> crate::common::Reg<regs::Soctrim03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim1_0(self) -> crate::common::Reg<regs::Soctrim10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim1_1(self) -> crate::common::Reg<regs::Soctrim11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim1_2(self) -> crate::common::Reg<regs::Soctrim12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "SoC Trim Phrase 1 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim1_3(self) -> crate::common::Reg<regs::Soctrim13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim2_0(self) -> crate::common::Reg<regs::Soctrim20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim2_1(self) -> crate::common::Reg<regs::Soctrim21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim2_2(self) -> crate::common::Reg<regs::Soctrim22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 2 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim2_3(self) -> crate::common::Reg<regs::Soctrim23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim3_0(self) -> crate::common::Reg<regs::Soctrim30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim3_1(self) -> crate::common::Reg<regs::Soctrim31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim3_2(self) -> crate::common::Reg<regs::Soctrim32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 3 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim3_3(self) -> crate::common::Reg<regs::Soctrim33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim4_0(self) -> crate::common::Reg<regs::Soctrim40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim4_1(self) -> crate::common::Reg<regs::Soctrim41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim4_2(self) -> crate::common::Reg<regs::Soctrim42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 4 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim4_3(self) -> crate::common::Reg<regs::Soctrim43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim5_0(self) -> crate::common::Reg<regs::Soctrim50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim5_1(self) -> crate::common::Reg<regs::Soctrim51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim5_2(self) -> crate::common::Reg<regs::Soctrim52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 5 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim5_3(self) -> crate::common::Reg<regs::Soctrim53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim6_0(self) -> crate::common::Reg<regs::Soctrim60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim6_1(self) -> crate::common::Reg<regs::Soctrim61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim6_2(self) -> crate::common::Reg<regs::Soctrim62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 6 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim6_3(self) -> crate::common::Reg<regs::Soctrim63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 0 Register"]
    #[inline(always)]
    pub const fn soctrim7_0(self) -> crate::common::Reg<regs::Soctrim70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 1 Register"]
    #[inline(always)]
    pub const fn soctrim7_1(self) -> crate::common::Reg<regs::Soctrim71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 2 Register"]
    #[inline(always)]
    pub const fn soctrim7_2(self) -> crate::common::Reg<regs::Soctrim72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "SoC Trim Phrase 7 Word 3 Register"]
    #[inline(always)]
    pub const fn soctrim7_3(self) -> crate::common::Reg<regs::Soctrim73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "BIST Configuration Register"]
    #[inline(always)]
    pub const fn r_ip_config(self) -> crate::common::Reg<regs::RIpConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "BIST Test Code Register"]
    #[inline(always)]
    pub const fn r_testcode(self) -> crate::common::Reg<regs::RTestcode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "BIST DFT Control Register"]
    #[inline(always)]
    pub const fn r_dft_ctrl(self) -> crate::common::Reg<regs::RDftCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "BIST Address Control Register"]
    #[inline(always)]
    pub const fn r_adr_ctrl(self) -> crate::common::Reg<regs::RAdrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "BIST Data Control 0 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl0(self) -> crate::common::Reg<regs::RDataCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "BIST Pin Control Register"]
    #[inline(always)]
    pub const fn r_pin_ctrl(self) -> crate::common::Reg<regs::RPinCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "BIST Loop Count Control Register"]
    #[inline(always)]
    pub const fn r_cnt_loop_ctrl(
        self,
    ) -> crate::common::Reg<regs::RCntLoopCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "BIST Timer Control Register"]
    #[inline(always)]
    pub const fn r_timer_ctrl(self) -> crate::common::Reg<regs::RTimerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "BIST Test Control Register"]
    #[inline(always)]
    pub const fn r_test_ctrl(self) -> crate::common::Reg<regs::RTestCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "BIST Abort Loop Register"]
    #[inline(always)]
    pub const fn r_abort_loop(self) -> crate::common::Reg<regs::RAbortLoop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "BIST Address Query Register"]
    #[inline(always)]
    pub const fn r_adr_query(self) -> crate::common::Reg<regs::RAdrQuery, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "BIST DOUT Query 0 Register"]
    #[inline(always)]
    pub const fn r_dout_query0(self) -> crate::common::Reg<regs::RDoutQuery0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "BIST SMW Query Register"]
    #[inline(always)]
    pub const fn r_smw_query(self) -> crate::common::Reg<regs::RSmwQuery, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "BIST SMW Setting 0 Register"]
    #[inline(always)]
    pub const fn r_smw_setting0(self) -> crate::common::Reg<regs::RSmwSetting0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "BIST SMW Setting 1 Register"]
    #[inline(always)]
    pub const fn r_smw_setting1(self) -> crate::common::Reg<regs::RSmwSetting1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "BIST SMP WHV Setting 0 Register"]
    #[inline(always)]
    pub const fn r_smp_whv0(self) -> crate::common::Reg<regs::RSmpWhv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "BIST SMP WHV Setting 1 Register"]
    #[inline(always)]
    pub const fn r_smp_whv1(self) -> crate::common::Reg<regs::RSmpWhv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "BIST SME WHV Setting 0 Register"]
    #[inline(always)]
    pub const fn r_sme_whv0(self) -> crate::common::Reg<regs::RSmeWhv0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "BIST SME WHV Setting 1 Register"]
    #[inline(always)]
    pub const fn r_sme_whv1(self) -> crate::common::Reg<regs::RSmeWhv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "BIST SMW Setting 2 Register"]
    #[inline(always)]
    pub const fn r_smw_setting2(self) -> crate::common::Reg<regs::RSmwSetting2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "BIST DIN MISR 0 Register"]
    #[inline(always)]
    pub const fn r_d_misr0(self) -> crate::common::Reg<regs::RDMisr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "BIST Address MISR 0 Register"]
    #[inline(always)]
    pub const fn r_a_misr0(self) -> crate::common::Reg<regs::RAMisr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "BIST Control MISR 0 Register"]
    #[inline(always)]
    pub const fn r_c_misr0(self) -> crate::common::Reg<regs::RCMisr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "BIST SMW Setting 3 Register"]
    #[inline(always)]
    pub const fn r_smw_setting3(self) -> crate::common::Reg<regs::RSmwSetting3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "BIST Data Control 1 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl1(self) -> crate::common::Reg<regs::RDataCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "BIST Data Control 2 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl2(self) -> crate::common::Reg<regs::RDataCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "BIST Data Control 3 Register"]
    #[inline(always)]
    pub const fn r_data_ctrl3(self) -> crate::common::Reg<regs::RDataCtrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "BIST Repair 0 for Block 0 Register"]
    #[inline(always)]
    pub const fn r_repair0_0(self) -> crate::common::Reg<regs::RRepair00, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "BIST Repair 1 Block 0 Register"]
    #[inline(always)]
    pub const fn r_repair0_1(self) -> crate::common::Reg<regs::RRepair01, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "BIST Repair 0 Block 1 Register"]
    #[inline(always)]
    pub const fn r_repair1_0(self) -> crate::common::Reg<regs::RRepair10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "BIST Repair 1 Block 1 Register"]
    #[inline(always)]
    pub const fn r_repair1_1(self) -> crate::common::Reg<regs::RRepair11, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "BIST Data Control 0 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl0_ex(
        self,
    ) -> crate::common::Reg<regs::RDataCtrl0Ex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "BIST Timer Control Extension Register"]
    #[inline(always)]
    pub const fn r_timer_ctrl_ex(
        self,
    ) -> crate::common::Reg<regs::RTimerCtrlEx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "BIST DOUT Query 1 Register"]
    #[inline(always)]
    pub const fn r_dout_query1(self) -> crate::common::Reg<regs::RDoutQuery1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "BIST DIN MISR 1 Register"]
    #[inline(always)]
    pub const fn r_d_misr1(self) -> crate::common::Reg<regs::RDMisr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "BIST Address MISR 1 Register"]
    #[inline(always)]
    pub const fn r_a_misr1(self) -> crate::common::Reg<regs::RAMisr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "BIST Control MISR 1 Register"]
    #[inline(always)]
    pub const fn r_c_misr1(self) -> crate::common::Reg<regs::RCMisr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "BIST Data Control 1 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl1_ex(
        self,
    ) -> crate::common::Reg<regs::RDataCtrl1Ex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "BIST Data Control 2 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl2_ex(
        self,
    ) -> crate::common::Reg<regs::RDataCtrl2Ex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "BIST Data Control 3 Extension Register"]
    #[inline(always)]
    pub const fn r_data_ctrl3_ex(
        self,
    ) -> crate::common::Reg<regs::RDataCtrl3Ex, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "SMW Timer Option Register"]
    #[inline(always)]
    pub const fn smw_timer_option(
        self,
    ) -> crate::common::Reg<regs::SmwTimerOption, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "SMW Setting Option 0 Register"]
    #[inline(always)]
    pub const fn smw_setting_option0(
        self,
    ) -> crate::common::Reg<regs::SmwSettingOption0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SMW Setting Option 2 Register"]
    #[inline(always)]
    pub const fn smw_setting_option2(
        self,
    ) -> crate::common::Reg<regs::SmwSettingOption2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "SMW Setting Option 3 Register"]
    #[inline(always)]
    pub const fn smw_setting_option3(
        self,
    ) -> crate::common::Reg<regs::SmwSettingOption3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "SMW SMP WHV Option 0 Register"]
    #[inline(always)]
    pub const fn smw_smp_whv_option0(
        self,
    ) -> crate::common::Reg<regs::SmwSmpWhvOption0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "SMW SME WHV Option 0 Register"]
    #[inline(always)]
    pub const fn smw_sme_whv_option0(
        self,
    ) -> crate::common::Reg<regs::SmwSmeWhvOption0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "SMW Setting Option 1 Register"]
    #[inline(always)]
    pub const fn smw_setting_option1(
        self,
    ) -> crate::common::Reg<regs::SmwSettingOption1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "SMW SMP WHV Option 1 Register"]
    #[inline(always)]
    pub const fn smw_smp_whv_option1(
        self,
    ) -> crate::common::Reg<regs::SmwSmpWhvOption1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "SMW SME WHV Option 1 Register"]
    #[inline(always)]
    pub const fn smw_sme_whv_option1(
        self,
    ) -> crate::common::Reg<regs::SmwSmeWhvOption1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "FMU Repair 0 Block 0 Register"]
    #[inline(always)]
    pub const fn repair0_0(self) -> crate::common::Reg<regs::Repair00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "FMU Repair 1 Block 0 Register"]
    #[inline(always)]
    pub const fn repair0_1(self) -> crate::common::Reg<regs::Repair01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "FMU Repair 0 Block 1 Register"]
    #[inline(always)]
    pub const fn repair1_0(self) -> crate::common::Reg<regs::Repair10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "FMU Repair 1 Block 1 Register"]
    #[inline(always)]
    pub const fn repair1_1(self) -> crate::common::Reg<regs::Repair11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "SMW HB Signals Register"]
    #[inline(always)]
    pub const fn smw_hb_signals(self) -> crate::common::Reg<regs::SmwHbSignals, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "BIST Datadump Control Register"]
    #[inline(always)]
    pub const fn bist_dump_ctrl(self) -> crate::common::Reg<regs::BistDumpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "ATX Pin Control Register"]
    #[inline(always)]
    pub const fn atx_pin_ctrl(self) -> crate::common::Reg<regs::AtxPinCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "Fail Count Register"]
    #[inline(always)]
    pub const fn failcnt(self) -> crate::common::Reg<regs::Failcnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Block 0 Program Pulse Count Register"]
    #[inline(always)]
    pub const fn pgm_pulse_cnt0(self) -> crate::common::Reg<regs::PgmPulseCnt0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "Block 1 Program Pulse Count Register"]
    #[inline(always)]
    pub const fn pgm_pulse_cnt1(self) -> crate::common::Reg<regs::PgmPulseCnt1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "Erase Pulse Count Register"]
    #[inline(always)]
    pub const fn ers_pulse_cnt(self) -> crate::common::Reg<regs::ErsPulseCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "Maximum Pulse Count Register"]
    #[inline(always)]
    pub const fn max_pulse_cnt(self) -> crate::common::Reg<regs::MaxPulseCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Port Control Register"]
    #[inline(always)]
    pub const fn port_ctrl(self) -> crate::common::Reg<regs::PortCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
}
pub mod regs;
pub mod vals;
