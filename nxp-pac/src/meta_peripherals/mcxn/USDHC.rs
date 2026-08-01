#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "uSDHC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usdhc {
    ptr: *mut u8,
}
unsafe impl Send for Usdhc {}
unsafe impl Sync for Usdhc {}
impl Usdhc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA System Address."]
    #[inline(always)]
    pub const fn ds_addr(self) -> crate::pac::common::Reg<DsAddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Block Attributes."]
    #[inline(always)]
    pub const fn blk_att(self) -> crate::pac::common::Reg<BlkAtt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Argument."]
    #[inline(always)]
    pub const fn cmd_arg(self) -> crate::pac::common::Reg<CmdArg, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Command Transfer Type."]
    #[inline(always)]
    pub const fn cmd_xfr_typ(self) -> crate::pac::common::Reg<CmdXfrTyp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Command Response."]
    #[inline(always)]
    pub const fn cmd_rsp(self, n: usize) -> crate::pac::common::Reg<CmdRsp, crate::pac::common::R> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _)
        }
    }
    #[doc = "Data Buffer Access Port."]
    #[inline(always)]
    pub const fn data_buff_acc_port(
        self,
    ) -> crate::pac::common::Reg<DataBuffAccPort, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Present State."]
    #[inline(always)]
    pub const fn pres_state(self) -> crate::pac::common::Reg<PresState, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Protocol Control."]
    #[inline(always)]
    pub const fn prot_ctrl(self) -> crate::pac::common::Reg<ProtCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "System Control."]
    #[inline(always)]
    pub const fn sys_ctrl(self) -> crate::pac::common::Reg<SysCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Status."]
    #[inline(always)]
    pub const fn int_status(self) -> crate::pac::common::Reg<IntStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt Status Enable."]
    #[inline(always)]
    pub const fn int_status_en(
        self,
    ) -> crate::pac::common::Reg<IntStatusEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Signal Enable."]
    #[inline(always)]
    pub const fn int_signal_en(
        self,
    ) -> crate::pac::common::Reg<IntSignalEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Auto CMD12 Error Status."]
    #[inline(always)]
    pub const fn autocmd12_err_status(
        self,
    ) -> crate::pac::common::Reg<Autocmd12ErrStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Host Controller Capabilities."]
    #[inline(always)]
    pub const fn host_ctrl_cap(
        self,
    ) -> crate::pac::common::Reg<HostCtrlCap, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Watermark Level."]
    #[inline(always)]
    pub const fn wtmk_lvl(self) -> crate::pac::common::Reg<WtmkLvl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Mixer Control."]
    #[inline(always)]
    pub const fn mix_ctrl(self) -> crate::pac::common::Reg<MixCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Force Event."]
    #[inline(always)]
    pub const fn force_event(self) -> crate::pac::common::Reg<ForceEvent, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ADMA Error Status."]
    #[inline(always)]
    pub const fn adma_err_status(
        self,
    ) -> crate::pac::common::Reg<AdmaErrStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ADMA System Address."]
    #[inline(always)]
    pub const fn adma_sys_addr(
        self,
    ) -> crate::pac::common::Reg<AdmaSysAddr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DLL (Delay Line) Control."]
    #[inline(always)]
    pub const fn dll_ctrl(self) -> crate::pac::common::Reg<DllCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DLL Status."]
    #[inline(always)]
    pub const fn dll_status(self) -> crate::pac::common::Reg<DllStatus, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "CLK Tuning Control and Status."]
    #[inline(always)]
    pub const fn clk_tune_ctrl_status(
        self,
    ) -> crate::pac::common::Reg<ClkTuneCtrlStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Vendor Specific Register."]
    #[inline(always)]
    pub const fn vend_spec(self) -> crate::pac::common::Reg<VendSpec, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "eMMC Boot."]
    #[inline(always)]
    pub const fn mmc_boot(self) -> crate::pac::common::Reg<MmcBoot, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Vendor Specific 2 Register."]
    #[inline(always)]
    pub const fn vend_spec2(self) -> crate::pac::common::Reg<VendSpec2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Tuning Control."]
    #[inline(always)]
    pub const fn tuning_ctrl(self) -> crate::pac::common::Reg<TuningCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
}
#[doc = "ADMA Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaErrStatus(pub u32);
impl AdmaErrStatus {
    #[doc = "ADMA error state (when ADMA error is occurred)."]
    #[must_use]
    #[inline(always)]
    pub const fn admaes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ADMA error state (when ADMA error is occurred)."]
    #[inline(always)]
    pub const fn set_admaes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "ADMA length mismatch error."]
    #[must_use]
    #[inline(always)]
    pub const fn admalme(&self) -> Admalme {
        let val = (self.0 >> 2usize) & 0x01;
        Admalme::from_bits(val as u8)
    }
    #[doc = "ADMA length mismatch error."]
    #[inline(always)]
    pub const fn set_admalme(&mut self, val: Admalme) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ADMA descriptor error."]
    #[must_use]
    #[inline(always)]
    pub const fn admadce(&self) -> Admadce {
        let val = (self.0 >> 3usize) & 0x01;
        Admadce::from_bits(val as u8)
    }
    #[doc = "ADMA descriptor error."]
    #[inline(always)]
    pub const fn set_admadce(&mut self, val: Admadce) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for AdmaErrStatus {
    #[inline(always)]
    fn default() -> AdmaErrStatus {
        AdmaErrStatus(0)
    }
}
impl core::fmt::Debug for AdmaErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdmaErrStatus")
            .field("admaes", &self.admaes())
            .field("admalme", &self.admalme())
            .field("admadce", &self.admadce())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdmaErrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AdmaErrStatus {{ admaes: {=u8:?}, admalme: {:?}, admadce: {:?} }}",
            self.admaes(),
            self.admalme(),
            self.admadce()
        )
    }
}
#[doc = "ADMA System Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaSysAddr(pub u32);
impl AdmaSysAddr {
    #[doc = "ADMA system address."]
    #[must_use]
    #[inline(always)]
    pub const fn ads_addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "ADMA system address."]
    #[inline(always)]
    pub const fn set_ads_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for AdmaSysAddr {
    #[inline(always)]
    fn default() -> AdmaSysAddr {
        AdmaSysAddr(0)
    }
}
impl core::fmt::Debug for AdmaSysAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdmaSysAddr")
            .field("ads_addr", &self.ads_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdmaSysAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdmaSysAddr {{ ads_addr: {=u32:?} }}", self.ads_addr())
    }
}
#[doc = "Auto CMD12 Error Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autocmd12ErrStatus(pub u32);
impl Autocmd12ErrStatus {
    #[doc = "Auto CMD12 not executed."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ne(&self) -> Ac12ne {
        let val = (self.0 >> 0usize) & 0x01;
        Ac12ne::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 not executed."]
    #[inline(always)]
    pub const fn set_ac12ne(&mut self, val: Ac12ne) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Auto CMD12 / 23 timeout error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12toe(&self) -> Ac12toe {
        let val = (self.0 >> 1usize) & 0x01;
        Ac12toe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 timeout error."]
    #[inline(always)]
    pub const fn set_ac12toe(&mut self, val: Ac12toe) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 / 23 CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ce(&self) -> Ac12ce {
        let val = (self.0 >> 2usize) & 0x01;
        Ac12ce::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 CRC error."]
    #[inline(always)]
    pub const fn set_ac12ce(&mut self, val: Ac12ce) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Auto CMD12 / 23 end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ebe(&self) -> Ac12ebe {
        let val = (self.0 >> 3usize) & 0x01;
        Ac12ebe::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 end bit error."]
    #[inline(always)]
    pub const fn set_ac12ebe(&mut self, val: Ac12ebe) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Auto CMD12 / 23 index error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12ie(&self) -> Ac12ie {
        let val = (self.0 >> 4usize) & 0x01;
        Ac12ie::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 / 23 index error."]
    #[inline(always)]
    pub const fn set_ac12ie(&mut self, val: Ac12ie) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Command not issued by Auto CMD12 error."]
    #[must_use]
    #[inline(always)]
    pub const fn cnibac12e(&self) -> Cnibac12e {
        let val = (self.0 >> 7usize) & 0x01;
        Cnibac12e::from_bits(val as u8)
    }
    #[doc = "Command not issued by Auto CMD12 error."]
    #[inline(always)]
    pub const fn set_cnibac12e(&mut self, val: Cnibac12e) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn execute_tuning(&self) -> ExecuteTuning {
        let val = (self.0 >> 22usize) & 0x01;
        ExecuteTuning::from_bits(val as u8)
    }
    #[doc = "Execute tuning."]
    #[inline(always)]
    pub const fn set_execute_tuning(&mut self, val: ExecuteTuning) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Sample clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> Autocmd12ErrStatusSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        Autocmd12ErrStatusSmpClkSel::from_bits(val as u8)
    }
    #[doc = "Sample clock select."]
    #[inline(always)]
    pub const fn set_smp_clk_sel(&mut self, val: Autocmd12ErrStatusSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
}
impl Default for Autocmd12ErrStatus {
    #[inline(always)]
    fn default() -> Autocmd12ErrStatus {
        Autocmd12ErrStatus(0)
    }
}
impl core::fmt::Debug for Autocmd12ErrStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autocmd12ErrStatus")
            .field("ac12ne", &self.ac12ne())
            .field("ac12toe", &self.ac12toe())
            .field("ac12ce", &self.ac12ce())
            .field("ac12ebe", &self.ac12ebe())
            .field("ac12ie", &self.ac12ie())
            .field("cnibac12e", &self.cnibac12e())
            .field("execute_tuning", &self.execute_tuning())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autocmd12ErrStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autocmd12ErrStatus {{ ac12ne: {:?}, ac12toe: {:?}, ac12ce: {:?}, ac12ebe: {:?}, ac12ie: {:?}, cnibac12e: {:?}, execute_tuning: {:?}, smp_clk_sel: {:?} }}",
            self.ac12ne(),
            self.ac12toe(),
            self.ac12ce(),
            self.ac12ebe(),
            self.ac12ie(),
            self.cnibac12e(),
            self.execute_tuning(),
            self.smp_clk_sel()
        )
    }
}
#[doc = "Block Attributes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkAtt(pub u32);
impl BlkAtt {
    #[doc = "Transfer block size."]
    #[must_use]
    #[inline(always)]
    pub const fn blksize(&self) -> Blksize {
        let val = (self.0 >> 0usize) & 0x1fff;
        Blksize::from_bits(val as u16)
    }
    #[doc = "Transfer block size."]
    #[inline(always)]
    pub const fn set_blksize(&mut self, val: Blksize) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val.to_bits() as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Blocks count for current transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn blkcnt(&self) -> Blkcnt {
        let val = (self.0 >> 16usize) & 0xffff;
        Blkcnt::from_bits(val as u16)
    }
    #[doc = "Blocks count for current transfer."]
    #[inline(always)]
    pub const fn set_blkcnt(&mut self, val: Blkcnt) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for BlkAtt {
    #[inline(always)]
    fn default() -> BlkAtt {
        BlkAtt(0)
    }
}
impl core::fmt::Debug for BlkAtt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BlkAtt")
            .field("blksize", &self.blksize())
            .field("blkcnt", &self.blkcnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BlkAtt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BlkAtt {{ blksize: {:?}, blkcnt: {:?} }}",
            self.blksize(),
            self.blkcnt()
        )
    }
}
#[doc = "CLK Tuning Control and Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkTuneCtrlStatus(pub u32);
impl ClkTuneCtrlStatus {
    #[doc = "Delay cells on the feedback clock between CLK_OUT and CLK_POST."]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_post(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells on the feedback clock between CLK_OUT and CLK_POST."]
    #[inline(always)]
    pub const fn set_dly_cell_set_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Delay cells on the feedback clock between CLK_PRE and CLK_OUT."]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_out(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells on the feedback clock between CLK_PRE and CLK_OUT."]
    #[inline(always)]
    pub const fn set_dly_cell_set_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "delay cells on the feedback clock between the feedback clock and CLK_PRE."]
    #[must_use]
    #[inline(always)]
    pub const fn dly_cell_set_pre(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "delay cells on the feedback clock between the feedback clock and CLK_PRE."]
    #[inline(always)]
    pub const fn set_dly_cell_set_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "NXT error."]
    #[must_use]
    #[inline(always)]
    pub const fn nxt_err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "NXT error."]
    #[inline(always)]
    pub const fn set_nxt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Delay cells added on the feedback clock between CLK_OUT and CLK_POST."]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_post(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells added on the feedback clock between CLK_OUT and CLK_POST."]
    #[inline(always)]
    pub const fn set_tap_sel_post(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Delay cells added on the feedback clock between CLK_PRE and CLK_OUT."]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_out(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay cells added on the feedback clock between CLK_PRE and CLK_OUT."]
    #[inline(always)]
    pub const fn set_tap_sel_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "TAP_SEL_PRE."]
    #[must_use]
    #[inline(always)]
    pub const fn tap_sel_pre(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "TAP_SEL_PRE."]
    #[inline(always)]
    pub const fn set_tap_sel_pre(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
    #[doc = "PRE error."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PRE error."]
    #[inline(always)]
    pub const fn set_pre_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ClkTuneCtrlStatus {
    #[inline(always)]
    fn default() -> ClkTuneCtrlStatus {
        ClkTuneCtrlStatus(0)
    }
}
impl core::fmt::Debug for ClkTuneCtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkTuneCtrlStatus")
            .field("dly_cell_set_post", &self.dly_cell_set_post())
            .field("dly_cell_set_out", &self.dly_cell_set_out())
            .field("dly_cell_set_pre", &self.dly_cell_set_pre())
            .field("nxt_err", &self.nxt_err())
            .field("tap_sel_post", &self.tap_sel_post())
            .field("tap_sel_out", &self.tap_sel_out())
            .field("tap_sel_pre", &self.tap_sel_pre())
            .field("pre_err", &self.pre_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkTuneCtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkTuneCtrlStatus {{ dly_cell_set_post: {=u8:?}, dly_cell_set_out: {=u8:?}, dly_cell_set_pre: {=u8:?}, nxt_err: {=bool:?}, tap_sel_post: {=u8:?}, tap_sel_out: {=u8:?}, tap_sel_pre: {=u8:?}, pre_err: {=bool:?} }}",
            self.dly_cell_set_post(),
            self.dly_cell_set_out(),
            self.dly_cell_set_pre(),
            self.nxt_err(),
            self.tap_sel_post(),
            self.tap_sel_out(),
            self.tap_sel_pre(),
            self.pre_err()
        )
    }
}
#[doc = "Command Argument."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdArg(pub u32);
impl CmdArg {
    #[doc = "Command argument."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdarg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command argument."]
    #[inline(always)]
    pub const fn set_cmdarg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdArg {
    #[inline(always)]
    fn default() -> CmdArg {
        CmdArg(0)
    }
}
impl core::fmt::Debug for CmdArg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdArg")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdArg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdArg {{ cmdarg: {=u32:?} }}", self.cmdarg())
    }
}
#[doc = "Command Response."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdRsp(pub u32);
impl CmdRsp {
    #[doc = "Command response."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrsp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command response."]
    #[inline(always)]
    pub const fn set_cmdrsp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CmdRsp {
    #[inline(always)]
    fn default() -> CmdRsp {
        CmdRsp(0)
    }
}
impl core::fmt::Debug for CmdRsp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdRsp")
            .field("cmdrsp", &self.cmdrsp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdRsp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CmdRsp {{ cmdrsp: {=u32:?} }}", self.cmdrsp())
    }
}
#[doc = "Command Transfer Type."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdXfrTyp(pub u32);
impl CmdXfrTyp {
    #[doc = "DMAEN."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> CmdXfrTypDmaen {
        let val = (self.0 >> 0usize) & 0x01;
        CmdXfrTypDmaen::from_bits(val as u8)
    }
    #[doc = "DMAEN."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: CmdXfrTypDmaen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "BCEN."]
    #[must_use]
    #[inline(always)]
    pub const fn bcen(&self) -> CmdXfrTypBcen {
        let val = (self.0 >> 1usize) & 0x01;
        CmdXfrTypBcen::from_bits(val as u8)
    }
    #[doc = "BCEN."]
    #[inline(always)]
    pub const fn set_bcen(&mut self, val: CmdXfrTypBcen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "AC12EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12en(&self) -> CmdXfrTypAc12en {
        let val = (self.0 >> 2usize) & 0x01;
        CmdXfrTypAc12en::from_bits(val as u8)
    }
    #[doc = "AC12EN."]
    #[inline(always)]
    pub const fn set_ac12en(&mut self, val: CmdXfrTypAc12en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DDR_EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr_en(&self) -> DdrEn {
        let val = (self.0 >> 3usize) & 0x01;
        DdrEn::from_bits(val as u8)
    }
    #[doc = "DDR_EN."]
    #[inline(always)]
    pub const fn set_ddr_en(&mut self, val: DdrEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DTDSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdsel(&self) -> CmdXfrTypDtdsel {
        let val = (self.0 >> 4usize) & 0x01;
        CmdXfrTypDtdsel::from_bits(val as u8)
    }
    #[doc = "DTDSEL."]
    #[inline(always)]
    pub const fn set_dtdsel(&mut self, val: CmdXfrTypDtdsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "MSBSEL."]
    #[must_use]
    #[inline(always)]
    pub const fn msbsel(&self) -> CmdXfrTypMsbsel {
        let val = (self.0 >> 5usize) & 0x01;
        CmdXfrTypMsbsel::from_bits(val as u8)
    }
    #[doc = "MSBSEL."]
    #[inline(always)]
    pub const fn set_msbsel(&mut self, val: CmdXfrTypMsbsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "NIBBLE_POS."]
    #[must_use]
    #[inline(always)]
    pub const fn nibble_pos(&self) -> NibblePos {
        let val = (self.0 >> 6usize) & 0x01;
        NibblePos::from_bits(val as u8)
    }
    #[doc = "NIBBLE_POS."]
    #[inline(always)]
    pub const fn set_nibble_pos(&mut self, val: NibblePos) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "AC23EN."]
    #[must_use]
    #[inline(always)]
    pub const fn ac23en(&self) -> Ac23en {
        let val = (self.0 >> 7usize) & 0x01;
        Ac23en::from_bits(val as u8)
    }
    #[doc = "AC23EN."]
    #[inline(always)]
    pub const fn set_ac23en(&mut self, val: Ac23en) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Response type select."]
    #[must_use]
    #[inline(always)]
    pub const fn rsptyp(&self) -> Rsptyp {
        let val = (self.0 >> 16usize) & 0x03;
        Rsptyp::from_bits(val as u8)
    }
    #[doc = "Response type select."]
    #[inline(always)]
    pub const fn set_rsptyp(&mut self, val: Rsptyp) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Command CRC check enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cccen(&self) -> Cccen {
        let val = (self.0 >> 19usize) & 0x01;
        Cccen::from_bits(val as u8)
    }
    #[doc = "Command CRC check enable."]
    #[inline(always)]
    pub const fn set_cccen(&mut self, val: Cccen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Command index check enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cicen(&self) -> Cicen {
        let val = (self.0 >> 20usize) & 0x01;
        Cicen::from_bits(val as u8)
    }
    #[doc = "Command index check enable."]
    #[inline(always)]
    pub const fn set_cicen(&mut self, val: Cicen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data present select."]
    #[must_use]
    #[inline(always)]
    pub const fn dpsel(&self) -> Dpsel {
        let val = (self.0 >> 21usize) & 0x01;
        Dpsel::from_bits(val as u8)
    }
    #[doc = "Data present select."]
    #[inline(always)]
    pub const fn set_dpsel(&mut self, val: Dpsel) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Command type."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdtyp(&self) -> Cmdtyp {
        let val = (self.0 >> 22usize) & 0x03;
        Cmdtyp::from_bits(val as u8)
    }
    #[doc = "Command type."]
    #[inline(always)]
    pub const fn set_cmdtyp(&mut self, val: Cmdtyp) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "Command index."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdinx(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index."]
    #[inline(always)]
    pub const fn set_cmdinx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for CmdXfrTyp {
    #[inline(always)]
    fn default() -> CmdXfrTyp {
        CmdXfrTyp(0)
    }
}
impl core::fmt::Debug for CmdXfrTyp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CmdXfrTyp")
            .field("dmaen", &self.dmaen())
            .field("bcen", &self.bcen())
            .field("ac12en", &self.ac12en())
            .field("ddr_en", &self.ddr_en())
            .field("dtdsel", &self.dtdsel())
            .field("msbsel", &self.msbsel())
            .field("nibble_pos", &self.nibble_pos())
            .field("ac23en", &self.ac23en())
            .field("rsptyp", &self.rsptyp())
            .field("cccen", &self.cccen())
            .field("cicen", &self.cicen())
            .field("dpsel", &self.dpsel())
            .field("cmdtyp", &self.cmdtyp())
            .field("cmdinx", &self.cmdinx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CmdXfrTyp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CmdXfrTyp {{ dmaen: {:?}, bcen: {:?}, ac12en: {:?}, ddr_en: {:?}, dtdsel: {:?}, msbsel: {:?}, nibble_pos: {:?}, ac23en: {:?}, rsptyp: {:?}, cccen: {:?}, cicen: {:?}, dpsel: {:?}, cmdtyp: {:?}, cmdinx: {=u8:?} }}",
            self.dmaen(),
            self.bcen(),
            self.ac12en(),
            self.ddr_en(),
            self.dtdsel(),
            self.msbsel(),
            self.nibble_pos(),
            self.ac23en(),
            self.rsptyp(),
            self.cccen(),
            self.cicen(),
            self.dpsel(),
            self.cmdtyp(),
            self.cmdinx()
        )
    }
}
#[doc = "Data Buffer Access Port."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataBuffAccPort(pub u32);
impl DataBuffAccPort {
    #[doc = "Data content."]
    #[must_use]
    #[inline(always)]
    pub const fn datcont(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data content."]
    #[inline(always)]
    pub const fn set_datcont(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DataBuffAccPort {
    #[inline(always)]
    fn default() -> DataBuffAccPort {
        DataBuffAccPort(0)
    }
}
impl core::fmt::Debug for DataBuffAccPort {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DataBuffAccPort")
            .field("datcont", &self.datcont())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DataBuffAccPort {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DataBuffAccPort {{ datcont: {=u32:?} }}", self.datcont())
    }
}
#[doc = "DLL (Delay Line) Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllCtrl(pub u32);
impl DllCtrl {
    #[doc = "DLL and delay chain."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DLL and delay chain."]
    #[inline(always)]
    pub const fn set_dll_ctrl_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DLL reset."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DLL reset."]
    #[inline(always)]
    pub const fn set_dll_ctrl_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DLL slave delay line."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_force_upd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DLL slave delay line."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_force_upd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DLL slave delay target0."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target0(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL slave delay target0."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_dly_target0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "DLL gate update."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_gate_update(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DLL gate update."]
    #[inline(always)]
    pub const fn set_dll_ctrl_gate_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DLL slave override."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DLL slave override."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DLL slave override val."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_override_val(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "DLL slave override val."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_override_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "DLL slave delay target1."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_dly_target1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "DLL slave delay target1."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_dly_target1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Slave delay line update interval."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_slv_update_int(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Slave delay line update interval."]
    #[inline(always)]
    pub const fn set_dll_ctrl_slv_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "DLL control loop update interval."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_ctrl_ref_update_int(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "DLL control loop update interval."]
    #[inline(always)]
    pub const fn set_dll_ctrl_ref_update_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for DllCtrl {
    #[inline(always)]
    fn default() -> DllCtrl {
        DllCtrl(0)
    }
}
impl core::fmt::Debug for DllCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DllCtrl")
            .field("dll_ctrl_enable", &self.dll_ctrl_enable())
            .field("dll_ctrl_reset", &self.dll_ctrl_reset())
            .field("dll_ctrl_slv_force_upd", &self.dll_ctrl_slv_force_upd())
            .field("dll_ctrl_slv_dly_target0", &self.dll_ctrl_slv_dly_target0())
            .field("dll_ctrl_gate_update", &self.dll_ctrl_gate_update())
            .field("dll_ctrl_slv_override", &self.dll_ctrl_slv_override())
            .field(
                "dll_ctrl_slv_override_val",
                &self.dll_ctrl_slv_override_val(),
            )
            .field("dll_ctrl_slv_dly_target1", &self.dll_ctrl_slv_dly_target1())
            .field("dll_ctrl_slv_update_int", &self.dll_ctrl_slv_update_int())
            .field("dll_ctrl_ref_update_int", &self.dll_ctrl_ref_update_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DllCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DllCtrl {{ dll_ctrl_enable: {=bool:?}, dll_ctrl_reset: {=bool:?}, dll_ctrl_slv_force_upd: {=bool:?}, dll_ctrl_slv_dly_target0: {=u8:?}, dll_ctrl_gate_update: {=bool:?}, dll_ctrl_slv_override: {=bool:?}, dll_ctrl_slv_override_val: {=u8:?}, dll_ctrl_slv_dly_target1: {=u8:?}, dll_ctrl_slv_update_int: {=u8:?}, dll_ctrl_ref_update_int: {=u8:?} }}",
            self.dll_ctrl_enable(),
            self.dll_ctrl_reset(),
            self.dll_ctrl_slv_force_upd(),
            self.dll_ctrl_slv_dly_target0(),
            self.dll_ctrl_gate_update(),
            self.dll_ctrl_slv_override(),
            self.dll_ctrl_slv_override_val(),
            self.dll_ctrl_slv_dly_target1(),
            self.dll_ctrl_slv_update_int(),
            self.dll_ctrl_ref_update_int()
        )
    }
}
#[doc = "DLL Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DllStatus(pub u32);
impl DllStatus {
    #[doc = "Slave delay-line lock status."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_slv_lock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay-line lock status."]
    #[inline(always)]
    pub const fn set_dll_sts_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reference DLL lock status."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_ref_lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reference DLL lock status."]
    #[inline(always)]
    pub const fn set_dll_sts_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Slave delay line select status."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_slv_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave delay line select status."]
    #[inline(always)]
    pub const fn set_dll_sts_slv_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 2usize)) | (((val as u32) & 0x7f) << 2usize);
    }
    #[doc = "Reference delay line select taps."]
    #[must_use]
    #[inline(always)]
    pub const fn dll_sts_ref_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "Reference delay line select taps."]
    #[inline(always)]
    pub const fn set_dll_sts_ref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
}
impl Default for DllStatus {
    #[inline(always)]
    fn default() -> DllStatus {
        DllStatus(0)
    }
}
impl core::fmt::Debug for DllStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DllStatus")
            .field("dll_sts_slv_lock", &self.dll_sts_slv_lock())
            .field("dll_sts_ref_lock", &self.dll_sts_ref_lock())
            .field("dll_sts_slv_sel", &self.dll_sts_slv_sel())
            .field("dll_sts_ref_sel", &self.dll_sts_ref_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DllStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DllStatus {{ dll_sts_slv_lock: {=bool:?}, dll_sts_ref_lock: {=bool:?}, dll_sts_slv_sel: {=u8:?}, dll_sts_ref_sel: {=u8:?} }}",
            self.dll_sts_slv_lock(),
            self.dll_sts_ref_lock(),
            self.dll_sts_slv_sel(),
            self.dll_sts_ref_sel()
        )
    }
}
#[doc = "DMA System Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DsAddr(pub u32);
impl DsAddr {
    #[doc = "System address."]
    #[must_use]
    #[inline(always)]
    pub const fn ds_addr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "System address."]
    #[inline(always)]
    pub const fn set_ds_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DsAddr {
    #[inline(always)]
    fn default() -> DsAddr {
        DsAddr(0)
    }
}
impl core::fmt::Debug for DsAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DsAddr")
            .field("ds_addr", &self.ds_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DsAddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DsAddr {{ ds_addr: {=u32:?} }}", self.ds_addr())
    }
}
#[doc = "Force Event."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceEvent(pub u32);
impl ForceEvent {
    #[doc = "Force event auto command 12 not executed."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ne(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 not executed."]
    #[inline(always)]
    pub const fn set_fevtac12ne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Force event auto command 12 time out error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12toe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 time out error."]
    #[inline(always)]
    pub const fn set_fevtac12toe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Force event auto command 12 CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ce(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Force event auto command 12 CRC error."]
    #[inline(always)]
    pub const fn set_fevtac12ce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Force event Auto Command 12 end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ebe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 end bit error."]
    #[inline(always)]
    pub const fn set_fevtac12ebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Force event Auto Command 12 index error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12ie(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 index error."]
    #[inline(always)]
    pub const fn set_fevtac12ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Force event command not executed by Auto Command 12 error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcnibac12e(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command not executed by Auto Command 12 error."]
    #[inline(always)]
    pub const fn set_fevtcnibac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Force event command time out error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtctoe(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command time out error."]
    #[inline(always)]
    pub const fn set_fevtctoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Force event command CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcce(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command CRC error."]
    #[inline(always)]
    pub const fn set_fevtcce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Force event command end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcebe(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command end bit error."]
    #[inline(always)]
    pub const fn set_fevtcebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Force event command index error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcie(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Force event command index error."]
    #[inline(always)]
    pub const fn set_fevtcie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Force event data time out error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdtoe(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data time out error."]
    #[inline(always)]
    pub const fn set_fevtdtoe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Force event data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdce(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data CRC error."]
    #[inline(always)]
    pub const fn set_fevtdce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Force event data end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdebe(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Force event data end bit error."]
    #[inline(always)]
    pub const fn set_fevtdebe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Force event Auto Command 12 error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtac12e(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Force event Auto Command 12 error."]
    #[inline(always)]
    pub const fn set_fevtac12e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Force tuning error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevttne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Force tuning error."]
    #[inline(always)]
    pub const fn set_fevttne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Force event DMA error."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtdmae(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Force event DMA error."]
    #[inline(always)]
    pub const fn set_fevtdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Force event card interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fevtcint(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Force event card interrupt."]
    #[inline(always)]
    pub const fn set_fevtcint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ForceEvent {
    #[inline(always)]
    fn default() -> ForceEvent {
        ForceEvent(0)
    }
}
impl core::fmt::Debug for ForceEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ForceEvent")
            .field("fevtac12ne", &self.fevtac12ne())
            .field("fevtac12toe", &self.fevtac12toe())
            .field("fevtac12ce", &self.fevtac12ce())
            .field("fevtac12ebe", &self.fevtac12ebe())
            .field("fevtac12ie", &self.fevtac12ie())
            .field("fevtcnibac12e", &self.fevtcnibac12e())
            .field("fevtctoe", &self.fevtctoe())
            .field("fevtcce", &self.fevtcce())
            .field("fevtcebe", &self.fevtcebe())
            .field("fevtcie", &self.fevtcie())
            .field("fevtdtoe", &self.fevtdtoe())
            .field("fevtdce", &self.fevtdce())
            .field("fevtdebe", &self.fevtdebe())
            .field("fevtac12e", &self.fevtac12e())
            .field("fevttne", &self.fevttne())
            .field("fevtdmae", &self.fevtdmae())
            .field("fevtcint", &self.fevtcint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ForceEvent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ForceEvent {{ fevtac12ne: {=bool:?}, fevtac12toe: {=bool:?}, fevtac12ce: {=bool:?}, fevtac12ebe: {=bool:?}, fevtac12ie: {=bool:?}, fevtcnibac12e: {=bool:?}, fevtctoe: {=bool:?}, fevtcce: {=bool:?}, fevtcebe: {=bool:?}, fevtcie: {=bool:?}, fevtdtoe: {=bool:?}, fevtdce: {=bool:?}, fevtdebe: {=bool:?}, fevtac12e: {=bool:?}, fevttne: {=bool:?}, fevtdmae: {=bool:?}, fevtcint: {=bool:?} }}",
            self.fevtac12ne(),
            self.fevtac12toe(),
            self.fevtac12ce(),
            self.fevtac12ebe(),
            self.fevtac12ie(),
            self.fevtcnibac12e(),
            self.fevtctoe(),
            self.fevtcce(),
            self.fevtcebe(),
            self.fevtcie(),
            self.fevtdtoe(),
            self.fevtdce(),
            self.fevtdebe(),
            self.fevtac12e(),
            self.fevttne(),
            self.fevtdmae(),
            self.fevtcint()
        )
    }
}
#[doc = "Host Controller Capabilities."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtrlCap(pub u32);
impl HostCtrlCap {
    #[doc = "SDR50 support."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr50_support(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SDR50 support."]
    #[inline(always)]
    pub const fn set_sdr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SDR104 support."]
    #[must_use]
    #[inline(always)]
    pub const fn sdr104_support(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SDR104 support."]
    #[inline(always)]
    pub const fn set_sdr104_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DDR50 support."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr50_support(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DDR50 support."]
    #[inline(always)]
    pub const fn set_ddr50_support(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Use Tuning for SDR50."]
    #[must_use]
    #[inline(always)]
    pub const fn use_tuning_sdr50(&self) -> UseTuningSdr50 {
        let val = (self.0 >> 13usize) & 0x01;
        UseTuningSdr50::from_bits(val as u8)
    }
    #[doc = "Use Tuning for SDR50."]
    #[inline(always)]
    pub const fn set_use_tuning_sdr50(&mut self, val: UseTuningSdr50) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Max block length."]
    #[must_use]
    #[inline(always)]
    pub const fn mbl(&self) -> Mbl {
        let val = (self.0 >> 16usize) & 0x07;
        Mbl::from_bits(val as u8)
    }
    #[doc = "Max block length."]
    #[inline(always)]
    pub const fn set_mbl(&mut self, val: Mbl) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "ADMA support."]
    #[must_use]
    #[inline(always)]
    pub const fn admas(&self) -> Admas {
        let val = (self.0 >> 20usize) & 0x01;
        Admas::from_bits(val as u8)
    }
    #[doc = "ADMA support."]
    #[inline(always)]
    pub const fn set_admas(&mut self, val: Admas) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High speed support."]
    #[must_use]
    #[inline(always)]
    pub const fn hss(&self) -> Hss {
        let val = (self.0 >> 21usize) & 0x01;
        Hss::from_bits(val as u8)
    }
    #[doc = "High speed support."]
    #[inline(always)]
    pub const fn set_hss(&mut self, val: Hss) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMA support."]
    #[must_use]
    #[inline(always)]
    pub const fn dmas(&self) -> Dmas {
        let val = (self.0 >> 22usize) & 0x01;
        Dmas::from_bits(val as u8)
    }
    #[doc = "DMA support."]
    #[inline(always)]
    pub const fn set_dmas(&mut self, val: Dmas) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Suspend / resume support."]
    #[must_use]
    #[inline(always)]
    pub const fn srs(&self) -> Srs {
        let val = (self.0 >> 23usize) & 0x01;
        Srs::from_bits(val as u8)
    }
    #[doc = "Suspend / resume support."]
    #[inline(always)]
    pub const fn set_srs(&mut self, val: Srs) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Voltage support 3.3 V."]
    #[must_use]
    #[inline(always)]
    pub const fn vs33(&self) -> Vs33 {
        let val = (self.0 >> 24usize) & 0x01;
        Vs33::from_bits(val as u8)
    }
    #[doc = "Voltage support 3.3 V."]
    #[inline(always)]
    pub const fn set_vs33(&mut self, val: Vs33) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Voltage support 3.0 V."]
    #[must_use]
    #[inline(always)]
    pub const fn vs30(&self) -> Vs30 {
        let val = (self.0 >> 25usize) & 0x01;
        Vs30::from_bits(val as u8)
    }
    #[doc = "Voltage support 3.0 V."]
    #[inline(always)]
    pub const fn set_vs30(&mut self, val: Vs30) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Voltage support 1.8 V."]
    #[must_use]
    #[inline(always)]
    pub const fn vs18(&self) -> Vs18 {
        let val = (self.0 >> 26usize) & 0x01;
        Vs18::from_bits(val as u8)
    }
    #[doc = "Voltage support 1.8 V."]
    #[inline(always)]
    pub const fn set_vs18(&mut self, val: Vs18) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for HostCtrlCap {
    #[inline(always)]
    fn default() -> HostCtrlCap {
        HostCtrlCap(0)
    }
}
impl core::fmt::Debug for HostCtrlCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HostCtrlCap")
            .field("sdr50_support", &self.sdr50_support())
            .field("sdr104_support", &self.sdr104_support())
            .field("ddr50_support", &self.ddr50_support())
            .field("use_tuning_sdr50", &self.use_tuning_sdr50())
            .field("mbl", &self.mbl())
            .field("admas", &self.admas())
            .field("hss", &self.hss())
            .field("dmas", &self.dmas())
            .field("srs", &self.srs())
            .field("vs33", &self.vs33())
            .field("vs30", &self.vs30())
            .field("vs18", &self.vs18())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HostCtrlCap {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HostCtrlCap {{ sdr50_support: {=bool:?}, sdr104_support: {=bool:?}, ddr50_support: {=bool:?}, use_tuning_sdr50: {:?}, mbl: {:?}, admas: {:?}, hss: {:?}, dmas: {:?}, srs: {:?}, vs33: {:?}, vs30: {:?}, vs18: {:?} }}",
            self.sdr50_support(),
            self.sdr104_support(),
            self.ddr50_support(),
            self.use_tuning_sdr50(),
            self.mbl(),
            self.admas(),
            self.hss(),
            self.dmas(),
            self.srs(),
            self.vs33(),
            self.vs30(),
            self.vs18()
        )
    }
}
#[doc = "Interrupt Signal Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSignalEn(pub u32);
impl IntSignalEn {
    #[doc = "Command complete interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccien(&self) -> Ccien {
        let val = (self.0 >> 0usize) & 0x01;
        Ccien::from_bits(val as u8)
    }
    #[doc = "Command complete interrupt enable."]
    #[inline(always)]
    pub const fn set_ccien(&mut self, val: Ccien) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcien(&self) -> Tcien {
        let val = (self.0 >> 1usize) & 0x01;
        Tcien::from_bits(val as u8)
    }
    #[doc = "Transfer complete interrupt enable."]
    #[inline(always)]
    pub const fn set_tcien(&mut self, val: Tcien) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bgeien(&self) -> Bgeien {
        let val = (self.0 >> 2usize) & 0x01;
        Bgeien::from_bits(val as u8)
    }
    #[doc = "Block gap event interrupt enable."]
    #[inline(always)]
    pub const fn set_bgeien(&mut self, val: Bgeien) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dintien(&self) -> Dintien {
        let val = (self.0 >> 3usize) & 0x01;
        Dintien::from_bits(val as u8)
    }
    #[doc = "DMA interrupt enable."]
    #[inline(always)]
    pub const fn set_dintien(&mut self, val: Dintien) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bwrien(&self) -> Bwrien {
        let val = (self.0 >> 4usize) & 0x01;
        Bwrien::from_bits(val as u8)
    }
    #[doc = "Buffer write ready interrupt enable."]
    #[inline(always)]
    pub const fn set_bwrien(&mut self, val: Bwrien) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn brrien(&self) -> Brrien {
        let val = (self.0 >> 5usize) & 0x01;
        Brrien::from_bits(val as u8)
    }
    #[doc = "Buffer read ready interrupt enable."]
    #[inline(always)]
    pub const fn set_brrien(&mut self, val: Brrien) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cinsien(&self) -> Cinsien {
        let val = (self.0 >> 6usize) & 0x01;
        Cinsien::from_bits(val as u8)
    }
    #[doc = "Card insertion interrupt enable."]
    #[inline(always)]
    pub const fn set_cinsien(&mut self, val: Cinsien) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crmien(&self) -> Crmien {
        let val = (self.0 >> 7usize) & 0x01;
        Crmien::from_bits(val as u8)
    }
    #[doc = "Card removal interrupt enable."]
    #[inline(always)]
    pub const fn set_crmien(&mut self, val: Crmien) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cintien(&self) -> Cintien {
        let val = (self.0 >> 8usize) & 0x01;
        Cintien::from_bits(val as u8)
    }
    #[doc = "Card interrupt enable."]
    #[inline(always)]
    pub const fn set_cintien(&mut self, val: Cintien) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rteien(&self) -> Rteien {
        let val = (self.0 >> 12usize) & 0x01;
        Rteien::from_bits(val as u8)
    }
    #[doc = "Re-tuning event interrupt enable."]
    #[inline(always)]
    pub const fn set_rteien(&mut self, val: Rteien) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning Pass interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpien(&self) -> Tpien {
        let val = (self.0 >> 14usize) & 0x01;
        Tpien::from_bits(val as u8)
    }
    #[doc = "Tuning Pass interrupt enable."]
    #[inline(always)]
    pub const fn set_tpien(&mut self, val: Tpien) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command timeout error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ctoeien(&self) -> Ctoeien {
        let val = (self.0 >> 16usize) & 0x01;
        Ctoeien::from_bits(val as u8)
    }
    #[doc = "Command timeout error interrupt enable."]
    #[inline(always)]
    pub const fn set_ctoeien(&mut self, val: Ctoeien) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cceien(&self) -> Cceien {
        let val = (self.0 >> 17usize) & 0x01;
        Cceien::from_bits(val as u8)
    }
    #[doc = "Command CRC error interrupt enable."]
    #[inline(always)]
    pub const fn set_cceien(&mut self, val: Cceien) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cebeien(&self) -> Cebeien {
        let val = (self.0 >> 18usize) & 0x01;
        Cebeien::from_bits(val as u8)
    }
    #[doc = "Command end bit error interrupt enable."]
    #[inline(always)]
    pub const fn set_cebeien(&mut self, val: Cebeien) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cieien(&self) -> Cieien {
        let val = (self.0 >> 19usize) & 0x01;
        Cieien::from_bits(val as u8)
    }
    #[doc = "Command index error interrupt enable."]
    #[inline(always)]
    pub const fn set_cieien(&mut self, val: Cieien) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtoeien(&self) -> Dtoeien {
        let val = (self.0 >> 20usize) & 0x01;
        Dtoeien::from_bits(val as u8)
    }
    #[doc = "Data timeout error interrupt enable."]
    #[inline(always)]
    pub const fn set_dtoeien(&mut self, val: Dtoeien) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dceien(&self) -> Dceien {
        let val = (self.0 >> 21usize) & 0x01;
        Dceien::from_bits(val as u8)
    }
    #[doc = "Data CRC error interrupt enable."]
    #[inline(always)]
    pub const fn set_dceien(&mut self, val: Dceien) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debeien(&self) -> Debeien {
        let val = (self.0 >> 22usize) & 0x01;
        Debeien::from_bits(val as u8)
    }
    #[doc = "Data end bit error interrupt enable."]
    #[inline(always)]
    pub const fn set_debeien(&mut self, val: Debeien) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12eien(&self) -> Ac12eien {
        let val = (self.0 >> 24usize) & 0x01;
        Ac12eien::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error interrupt enable."]
    #[inline(always)]
    pub const fn set_ac12eien(&mut self, val: Ac12eien) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tneien(&self) -> Tneien {
        let val = (self.0 >> 26usize) & 0x01;
        Tneien::from_bits(val as u8)
    }
    #[doc = "Tuning error interrupt enable."]
    #[inline(always)]
    pub const fn set_tneien(&mut self, val: Tneien) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error interrupt enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaeien(&self) -> Dmaeien {
        let val = (self.0 >> 28usize) & 0x01;
        Dmaeien::from_bits(val as u8)
    }
    #[doc = "DMA error interrupt enable."]
    #[inline(always)]
    pub const fn set_dmaeien(&mut self, val: Dmaeien) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntSignalEn {
    #[inline(always)]
    fn default() -> IntSignalEn {
        IntSignalEn(0)
    }
}
impl core::fmt::Debug for IntSignalEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSignalEn")
            .field("ccien", &self.ccien())
            .field("tcien", &self.tcien())
            .field("bgeien", &self.bgeien())
            .field("dintien", &self.dintien())
            .field("bwrien", &self.bwrien())
            .field("brrien", &self.brrien())
            .field("cinsien", &self.cinsien())
            .field("crmien", &self.crmien())
            .field("cintien", &self.cintien())
            .field("rteien", &self.rteien())
            .field("tpien", &self.tpien())
            .field("ctoeien", &self.ctoeien())
            .field("cceien", &self.cceien())
            .field("cebeien", &self.cebeien())
            .field("cieien", &self.cieien())
            .field("dtoeien", &self.dtoeien())
            .field("dceien", &self.dceien())
            .field("debeien", &self.debeien())
            .field("ac12eien", &self.ac12eien())
            .field("tneien", &self.tneien())
            .field("dmaeien", &self.dmaeien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSignalEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSignalEn {{ ccien: {:?}, tcien: {:?}, bgeien: {:?}, dintien: {:?}, bwrien: {:?}, brrien: {:?}, cinsien: {:?}, crmien: {:?}, cintien: {:?}, rteien: {:?}, tpien: {:?}, ctoeien: {:?}, cceien: {:?}, cebeien: {:?}, cieien: {:?}, dtoeien: {:?}, dceien: {:?}, debeien: {:?}, ac12eien: {:?}, tneien: {:?}, dmaeien: {:?} }}",
            self.ccien(),
            self.tcien(),
            self.bgeien(),
            self.dintien(),
            self.bwrien(),
            self.brrien(),
            self.cinsien(),
            self.crmien(),
            self.cintien(),
            self.rteien(),
            self.tpien(),
            self.ctoeien(),
            self.cceien(),
            self.cebeien(),
            self.cieien(),
            self.dtoeien(),
            self.dceien(),
            self.debeien(),
            self.ac12eien(),
            self.tneien(),
            self.dmaeien()
        )
    }
}
#[doc = "Interrupt Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "Command complete."]
    #[must_use]
    #[inline(always)]
    pub const fn cc(&self) -> Cc {
        let val = (self.0 >> 0usize) & 0x01;
        Cc::from_bits(val as u8)
    }
    #[doc = "Command complete."]
    #[inline(always)]
    pub const fn set_cc(&mut self, val: Cc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete."]
    #[must_use]
    #[inline(always)]
    pub const fn tc(&self) -> Tc {
        let val = (self.0 >> 1usize) & 0x01;
        Tc::from_bits(val as u8)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub const fn set_tc(&mut self, val: Tc) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event."]
    #[must_use]
    #[inline(always)]
    pub const fn bge(&self) -> Bge {
        let val = (self.0 >> 2usize) & 0x01;
        Bge::from_bits(val as u8)
    }
    #[doc = "Block gap event."]
    #[inline(always)]
    pub const fn set_bge(&mut self, val: Bge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dint(&self) -> Dint {
        let val = (self.0 >> 3usize) & 0x01;
        Dint::from_bits(val as u8)
    }
    #[doc = "DMA interrupt."]
    #[inline(always)]
    pub const fn set_dint(&mut self, val: Dint) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready."]
    #[must_use]
    #[inline(always)]
    pub const fn bwr(&self) -> Bwr {
        let val = (self.0 >> 4usize) & 0x01;
        Bwr::from_bits(val as u8)
    }
    #[doc = "Buffer write ready."]
    #[inline(always)]
    pub const fn set_bwr(&mut self, val: Bwr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready."]
    #[must_use]
    #[inline(always)]
    pub const fn brr(&self) -> Brr {
        let val = (self.0 >> 5usize) & 0x01;
        Brr::from_bits(val as u8)
    }
    #[doc = "Buffer read ready."]
    #[inline(always)]
    pub const fn set_brr(&mut self, val: Brr) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion."]
    #[must_use]
    #[inline(always)]
    pub const fn cins(&self) -> Cins {
        let val = (self.0 >> 6usize) & 0x01;
        Cins::from_bits(val as u8)
    }
    #[doc = "Card insertion."]
    #[inline(always)]
    pub const fn set_cins(&mut self, val: Cins) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal."]
    #[must_use]
    #[inline(always)]
    pub const fn crm(&self) -> Crm {
        let val = (self.0 >> 7usize) & 0x01;
        Crm::from_bits(val as u8)
    }
    #[doc = "Card removal."]
    #[inline(always)]
    pub const fn set_crm(&mut self, val: Crm) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cint(&self) -> Cint {
        let val = (self.0 >> 8usize) & 0x01;
        Cint::from_bits(val as u8)
    }
    #[doc = "Card interrupt."]
    #[inline(always)]
    pub const fn set_cint(&mut self, val: Cint) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn rte(&self) -> Rte {
        let val = (self.0 >> 12usize) & 0x01;
        Rte::from_bits(val as u8)
    }
    #[doc = "Re-tuning event: (only for SD3.0 SDR104 mode)."]
    #[inline(always)]
    pub const fn set_rte(&mut self, val: Rte) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn tp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning pass:(only for SD3.0 SDR104 mode)."]
    #[inline(always)]
    pub const fn set_tp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error Interrupt Status."]
    #[must_use]
    #[inline(always)]
    pub const fn err_int_status(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error Interrupt Status."]
    #[inline(always)]
    pub const fn set_err_int_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Command timeout error."]
    #[must_use]
    #[inline(always)]
    pub const fn ctoe(&self) -> Ctoe {
        let val = (self.0 >> 16usize) & 0x01;
        Ctoe::from_bits(val as u8)
    }
    #[doc = "Command timeout error."]
    #[inline(always)]
    pub const fn set_ctoe(&mut self, val: Ctoe) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn cce(&self) -> Cce {
        let val = (self.0 >> 17usize) & 0x01;
        Cce::from_bits(val as u8)
    }
    #[doc = "Command CRC error."]
    #[inline(always)]
    pub const fn set_cce(&mut self, val: Cce) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn cebe(&self) -> Cebe {
        let val = (self.0 >> 18usize) & 0x01;
        Cebe::from_bits(val as u8)
    }
    #[doc = "Command end bit error."]
    #[inline(always)]
    pub const fn set_cebe(&mut self, val: Cebe) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error."]
    #[must_use]
    #[inline(always)]
    pub const fn cie(&self) -> Cie {
        let val = (self.0 >> 19usize) & 0x01;
        Cie::from_bits(val as u8)
    }
    #[doc = "Command index error."]
    #[inline(always)]
    pub const fn set_cie(&mut self, val: Cie) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error."]
    #[must_use]
    #[inline(always)]
    pub const fn dtoe(&self) -> Dtoe {
        let val = (self.0 >> 20usize) & 0x01;
        Dtoe::from_bits(val as u8)
    }
    #[doc = "Data timeout error."]
    #[inline(always)]
    pub const fn set_dtoe(&mut self, val: Dtoe) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error."]
    #[must_use]
    #[inline(always)]
    pub const fn dce(&self) -> Dce {
        let val = (self.0 >> 21usize) & 0x01;
        Dce::from_bits(val as u8)
    }
    #[doc = "Data CRC error."]
    #[inline(always)]
    pub const fn set_dce(&mut self, val: Dce) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error."]
    #[must_use]
    #[inline(always)]
    pub const fn debe(&self) -> Debe {
        let val = (self.0 >> 22usize) & 0x01;
        Debe::from_bits(val as u8)
    }
    #[doc = "Data end bit error."]
    #[inline(always)]
    pub const fn set_debe(&mut self, val: Debe) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12e(&self) -> Ac12e {
        let val = (self.0 >> 24usize) & 0x01;
        Ac12e::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error."]
    #[inline(always)]
    pub const fn set_ac12e(&mut self, val: Ac12e) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn tne(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Tuning error: (only for SD3.0 SDR104 mode)."]
    #[inline(always)]
    pub const fn set_tne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error."]
    #[must_use]
    #[inline(always)]
    pub const fn dmae(&self) -> Dmae {
        let val = (self.0 >> 28usize) & 0x01;
        Dmae::from_bits(val as u8)
    }
    #[doc = "DMA error."]
    #[inline(always)]
    pub const fn set_dmae(&mut self, val: Dmae) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("cc", &self.cc())
            .field("tc", &self.tc())
            .field("bge", &self.bge())
            .field("dint", &self.dint())
            .field("bwr", &self.bwr())
            .field("brr", &self.brr())
            .field("cins", &self.cins())
            .field("crm", &self.crm())
            .field("cint", &self.cint())
            .field("rte", &self.rte())
            .field("tp", &self.tp())
            .field("err_int_status", &self.err_int_status())
            .field("ctoe", &self.ctoe())
            .field("cce", &self.cce())
            .field("cebe", &self.cebe())
            .field("cie", &self.cie())
            .field("dtoe", &self.dtoe())
            .field("dce", &self.dce())
            .field("debe", &self.debe())
            .field("ac12e", &self.ac12e())
            .field("tne", &self.tne())
            .field("dmae", &self.dmae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ cc: {:?}, tc: {:?}, bge: {:?}, dint: {:?}, bwr: {:?}, brr: {:?}, cins: {:?}, crm: {:?}, cint: {:?}, rte: {:?}, tp: {=bool:?}, err_int_status: {=bool:?}, ctoe: {:?}, cce: {:?}, cebe: {:?}, cie: {:?}, dtoe: {:?}, dce: {:?}, debe: {:?}, ac12e: {:?}, tne: {=bool:?}, dmae: {:?} }}",
            self.cc(),
            self.tc(),
            self.bge(),
            self.dint(),
            self.bwr(),
            self.brr(),
            self.cins(),
            self.crm(),
            self.cint(),
            self.rte(),
            self.tp(),
            self.err_int_status(),
            self.ctoe(),
            self.cce(),
            self.cebe(),
            self.cie(),
            self.dtoe(),
            self.dce(),
            self.debe(),
            self.ac12e(),
            self.tne(),
            self.dmae()
        )
    }
}
#[doc = "Interrupt Status Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatusEn(pub u32);
impl IntStatusEn {
    #[doc = "Command complete status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccsen(&self) -> Ccsen {
        let val = (self.0 >> 0usize) & 0x01;
        Ccsen::from_bits(val as u8)
    }
    #[doc = "Command complete status enable."]
    #[inline(always)]
    pub const fn set_ccsen(&mut self, val: Ccsen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer complete status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tcsen(&self) -> Tcsen {
        let val = (self.0 >> 1usize) & 0x01;
        Tcsen::from_bits(val as u8)
    }
    #[doc = "Transfer complete status enable."]
    #[inline(always)]
    pub const fn set_tcsen(&mut self, val: Tcsen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Block gap event status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bgesen(&self) -> Bgesen {
        let val = (self.0 >> 2usize) & 0x01;
        Bgesen::from_bits(val as u8)
    }
    #[doc = "Block gap event status enable."]
    #[inline(always)]
    pub const fn set_bgesen(&mut self, val: Bgesen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA interrupt status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dintsen(&self) -> Dintsen {
        let val = (self.0 >> 3usize) & 0x01;
        Dintsen::from_bits(val as u8)
    }
    #[doc = "DMA interrupt status enable."]
    #[inline(always)]
    pub const fn set_dintsen(&mut self, val: Dintsen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Buffer write ready status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bwrsen(&self) -> Bwrsen {
        let val = (self.0 >> 4usize) & 0x01;
        Bwrsen::from_bits(val as u8)
    }
    #[doc = "Buffer write ready status enable."]
    #[inline(always)]
    pub const fn set_bwrsen(&mut self, val: Bwrsen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Buffer read ready status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn brrsen(&self) -> Brrsen {
        let val = (self.0 >> 5usize) & 0x01;
        Brrsen::from_bits(val as u8)
    }
    #[doc = "Buffer read ready status enable."]
    #[inline(always)]
    pub const fn set_brrsen(&mut self, val: Brrsen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Card insertion status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cinssen(&self) -> Cinssen {
        let val = (self.0 >> 6usize) & 0x01;
        Cinssen::from_bits(val as u8)
    }
    #[doc = "Card insertion status enable."]
    #[inline(always)]
    pub const fn set_cinssen(&mut self, val: Cinssen) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Card removal status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn crmsen(&self) -> Crmsen {
        let val = (self.0 >> 7usize) & 0x01;
        Crmsen::from_bits(val as u8)
    }
    #[doc = "Card removal status enable."]
    #[inline(always)]
    pub const fn set_crmsen(&mut self, val: Crmsen) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Card interrupt status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cintsen(&self) -> Cintsen {
        let val = (self.0 >> 8usize) & 0x01;
        Cintsen::from_bits(val as u8)
    }
    #[doc = "Card interrupt status enable."]
    #[inline(always)]
    pub const fn set_cintsen(&mut self, val: Cintsen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Re-tuning event status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rtesen(&self) -> Rtesen {
        let val = (self.0 >> 12usize) & 0x01;
        Rtesen::from_bits(val as u8)
    }
    #[doc = "Re-tuning event status enable."]
    #[inline(always)]
    pub const fn set_rtesen(&mut self, val: Rtesen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tuning pass status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tpsen(&self) -> Tpsen {
        let val = (self.0 >> 14usize) & 0x01;
        Tpsen::from_bits(val as u8)
    }
    #[doc = "Tuning pass status enable."]
    #[inline(always)]
    pub const fn set_tpsen(&mut self, val: Tpsen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Command timeout error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ctoesen(&self) -> Ctoesen {
        let val = (self.0 >> 16usize) & 0x01;
        Ctoesen::from_bits(val as u8)
    }
    #[doc = "Command timeout error status enable."]
    #[inline(always)]
    pub const fn set_ctoesen(&mut self, val: Ctoesen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Command CRC error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ccesen(&self) -> Ccesen {
        let val = (self.0 >> 17usize) & 0x01;
        Ccesen::from_bits(val as u8)
    }
    #[doc = "Command CRC error status enable."]
    #[inline(always)]
    pub const fn set_ccesen(&mut self, val: Ccesen) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Command end bit error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn cebesen(&self) -> Cebesen {
        let val = (self.0 >> 18usize) & 0x01;
        Cebesen::from_bits(val as u8)
    }
    #[doc = "Command end bit error status enable."]
    #[inline(always)]
    pub const fn set_cebesen(&mut self, val: Cebesen) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Command index error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ciesen(&self) -> Ciesen {
        let val = (self.0 >> 19usize) & 0x01;
        Ciesen::from_bits(val as u8)
    }
    #[doc = "Command index error status enable."]
    #[inline(always)]
    pub const fn set_ciesen(&mut self, val: Ciesen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Data timeout error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dtoesen(&self) -> Dtoesen {
        let val = (self.0 >> 20usize) & 0x01;
        Dtoesen::from_bits(val as u8)
    }
    #[doc = "Data timeout error status enable."]
    #[inline(always)]
    pub const fn set_dtoesen(&mut self, val: Dtoesen) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Data CRC error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dcesen(&self) -> Dcesen {
        let val = (self.0 >> 21usize) & 0x01;
        Dcesen::from_bits(val as u8)
    }
    #[doc = "Data CRC error status enable."]
    #[inline(always)]
    pub const fn set_dcesen(&mut self, val: Dcesen) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Data end bit error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn debesen(&self) -> Debesen {
        let val = (self.0 >> 22usize) & 0x01;
        Debesen::from_bits(val as u8)
    }
    #[doc = "Data end bit error status enable."]
    #[inline(always)]
    pub const fn set_debesen(&mut self, val: Debesen) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Auto CMD12 error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12esen(&self) -> Ac12esen {
        let val = (self.0 >> 24usize) & 0x01;
        Ac12esen::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 error status enable."]
    #[inline(always)]
    pub const fn set_ac12esen(&mut self, val: Ac12esen) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Tuning error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tnesen(&self) -> Tnesen {
        let val = (self.0 >> 26usize) & 0x01;
        Tnesen::from_bits(val as u8)
    }
    #[doc = "Tuning error status enable."]
    #[inline(always)]
    pub const fn set_tnesen(&mut self, val: Tnesen) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMA error status enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaesen(&self) -> Dmaesen {
        let val = (self.0 >> 28usize) & 0x01;
        Dmaesen::from_bits(val as u8)
    }
    #[doc = "DMA error status enable."]
    #[inline(always)]
    pub const fn set_dmaesen(&mut self, val: Dmaesen) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for IntStatusEn {
    #[inline(always)]
    fn default() -> IntStatusEn {
        IntStatusEn(0)
    }
}
impl core::fmt::Debug for IntStatusEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatusEn")
            .field("ccsen", &self.ccsen())
            .field("tcsen", &self.tcsen())
            .field("bgesen", &self.bgesen())
            .field("dintsen", &self.dintsen())
            .field("bwrsen", &self.bwrsen())
            .field("brrsen", &self.brrsen())
            .field("cinssen", &self.cinssen())
            .field("crmsen", &self.crmsen())
            .field("cintsen", &self.cintsen())
            .field("rtesen", &self.rtesen())
            .field("tpsen", &self.tpsen())
            .field("ctoesen", &self.ctoesen())
            .field("ccesen", &self.ccesen())
            .field("cebesen", &self.cebesen())
            .field("ciesen", &self.ciesen())
            .field("dtoesen", &self.dtoesen())
            .field("dcesen", &self.dcesen())
            .field("debesen", &self.debesen())
            .field("ac12esen", &self.ac12esen())
            .field("tnesen", &self.tnesen())
            .field("dmaesen", &self.dmaesen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatusEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatusEn {{ ccsen: {:?}, tcsen: {:?}, bgesen: {:?}, dintsen: {:?}, bwrsen: {:?}, brrsen: {:?}, cinssen: {:?}, crmsen: {:?}, cintsen: {:?}, rtesen: {:?}, tpsen: {:?}, ctoesen: {:?}, ccesen: {:?}, cebesen: {:?}, ciesen: {:?}, dtoesen: {:?}, dcesen: {:?}, debesen: {:?}, ac12esen: {:?}, tnesen: {:?}, dmaesen: {:?} }}",
            self.ccsen(),
            self.tcsen(),
            self.bgesen(),
            self.dintsen(),
            self.bwrsen(),
            self.brrsen(),
            self.cinssen(),
            self.crmsen(),
            self.cintsen(),
            self.rtesen(),
            self.tpsen(),
            self.ctoesen(),
            self.ccesen(),
            self.cebesen(),
            self.ciesen(),
            self.dtoesen(),
            self.dcesen(),
            self.debesen(),
            self.ac12esen(),
            self.tnesen(),
            self.dmaesen()
        )
    }
}
#[doc = "Mixer Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MixCtrl(pub u32);
impl MixCtrl {
    #[doc = "DMA enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> MixCtrlDmaen {
        let val = (self.0 >> 0usize) & 0x01;
        MixCtrlDmaen::from_bits(val as u8)
    }
    #[doc = "DMA enable."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: MixCtrlDmaen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Block count enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bcen(&self) -> MixCtrlBcen {
        let val = (self.0 >> 1usize) & 0x01;
        MixCtrlBcen::from_bits(val as u8)
    }
    #[doc = "Block count enable."]
    #[inline(always)]
    pub const fn set_bcen(&mut self, val: MixCtrlBcen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Auto CMD12 enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12en(&self) -> MixCtrlAc12en {
        let val = (self.0 >> 2usize) & 0x01;
        MixCtrlAc12en::from_bits(val as u8)
    }
    #[doc = "Auto CMD12 enable."]
    #[inline(always)]
    pub const fn set_ac12en(&mut self, val: MixCtrlAc12en) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Dual data rate mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Dual data rate mode selection."]
    #[inline(always)]
    pub const fn set_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer direction select."]
    #[must_use]
    #[inline(always)]
    pub const fn dtdsel(&self) -> MixCtrlDtdsel {
        let val = (self.0 >> 4usize) & 0x01;
        MixCtrlDtdsel::from_bits(val as u8)
    }
    #[doc = "Data transfer direction select."]
    #[inline(always)]
    pub const fn set_dtdsel(&mut self, val: MixCtrlDtdsel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi / Single block select."]
    #[must_use]
    #[inline(always)]
    pub const fn msbsel(&self) -> MixCtrlMsbsel {
        let val = (self.0 >> 5usize) & 0x01;
        MixCtrlMsbsel::from_bits(val as u8)
    }
    #[doc = "Multi / Single block select."]
    #[inline(always)]
    pub const fn set_msbsel(&mut self, val: MixCtrlMsbsel) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Nibble position indication."]
    #[must_use]
    #[inline(always)]
    pub const fn nibble_pos(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Nibble position indication."]
    #[inline(always)]
    pub const fn set_nibble_pos(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Auto CMD23 enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ac23en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto CMD23 enable."]
    #[inline(always)]
    pub const fn set_ac23en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn exe_tune(&self) -> ExeTune {
        let val = (self.0 >> 22usize) & 0x01;
        ExeTune::from_bits(val as u8)
    }
    #[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode)."]
    #[inline(always)]
    pub const fn set_exe_tune(&mut self, val: ExeTune) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Clock selection."]
    #[must_use]
    #[inline(always)]
    pub const fn smp_clk_sel(&self) -> MixCtrlSmpClkSel {
        let val = (self.0 >> 23usize) & 0x01;
        MixCtrlSmpClkSel::from_bits(val as u8)
    }
    #[doc = "Clock selection."]
    #[inline(always)]
    pub const fn set_smp_clk_sel(&mut self, val: MixCtrlSmpClkSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn auto_tune_en(&self) -> AutoTuneEn {
        let val = (self.0 >> 24usize) & 0x01;
        AutoTuneEn::from_bits(val as u8)
    }
    #[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode)."]
    #[inline(always)]
    pub const fn set_auto_tune_en(&mut self, val: AutoTuneEn) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn fbclk_sel(&self) -> FbclkSel {
        let val = (self.0 >> 25usize) & 0x01;
        FbclkSel::from_bits(val as u8)
    }
    #[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode)."]
    #[inline(always)]
    pub const fn set_fbclk_sel(&mut self, val: FbclkSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for MixCtrl {
    #[inline(always)]
    fn default() -> MixCtrl {
        MixCtrl(0)
    }
}
impl core::fmt::Debug for MixCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MixCtrl")
            .field("dmaen", &self.dmaen())
            .field("bcen", &self.bcen())
            .field("ac12en", &self.ac12en())
            .field("ddr_en", &self.ddr_en())
            .field("dtdsel", &self.dtdsel())
            .field("msbsel", &self.msbsel())
            .field("nibble_pos", &self.nibble_pos())
            .field("ac23en", &self.ac23en())
            .field("exe_tune", &self.exe_tune())
            .field("smp_clk_sel", &self.smp_clk_sel())
            .field("auto_tune_en", &self.auto_tune_en())
            .field("fbclk_sel", &self.fbclk_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MixCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MixCtrl {{ dmaen: {:?}, bcen: {:?}, ac12en: {:?}, ddr_en: {=bool:?}, dtdsel: {:?}, msbsel: {:?}, nibble_pos: {=bool:?}, ac23en: {=bool:?}, exe_tune: {:?}, smp_clk_sel: {:?}, auto_tune_en: {:?}, fbclk_sel: {:?} }}",
            self.dmaen(),
            self.bcen(),
            self.ac12en(),
            self.ddr_en(),
            self.dtdsel(),
            self.msbsel(),
            self.nibble_pos(),
            self.ac23en(),
            self.exe_tune(),
            self.smp_clk_sel(),
            self.auto_tune_en(),
            self.fbclk_sel()
        )
    }
}
#[doc = "eMMC Boot."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MmcBoot(pub u32);
impl MmcBoot {
    #[doc = "Boot ACK time out."]
    #[must_use]
    #[inline(always)]
    pub const fn dtocv_ack(&self) -> DtocvAck {
        let val = (self.0 >> 0usize) & 0x0f;
        DtocvAck::from_bits(val as u8)
    }
    #[doc = "Boot ACK time out."]
    #[inline(always)]
    pub const fn set_dtocv_ack(&mut self, val: DtocvAck) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "BOOT ACK."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_ack(&self) -> BootAck {
        let val = (self.0 >> 4usize) & 0x01;
        BootAck::from_bits(val as u8)
    }
    #[doc = "BOOT ACK."]
    #[inline(always)]
    pub const fn set_boot_ack(&mut self, val: BootAck) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Boot mode."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_mode(&self) -> BootMode {
        let val = (self.0 >> 5usize) & 0x01;
        BootMode::from_bits(val as u8)
    }
    #[doc = "Boot mode."]
    #[inline(always)]
    pub const fn set_boot_mode(&mut self, val: BootMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Boot enable."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_en(&self) -> BootEn {
        let val = (self.0 >> 6usize) & 0x01;
        BootEn::from_bits(val as u8)
    }
    #[doc = "Boot enable."]
    #[inline(always)]
    pub const fn set_boot_en(&mut self, val: BootEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Auto stop at block gap."]
    #[must_use]
    #[inline(always)]
    pub const fn auto_sabg_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Auto stop at block gap."]
    #[inline(always)]
    pub const fn set_auto_sabg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Time out."]
    #[must_use]
    #[inline(always)]
    pub const fn disable_time_out(&self) -> DisableTimeOut {
        let val = (self.0 >> 8usize) & 0x01;
        DisableTimeOut::from_bits(val as u8)
    }
    #[doc = "Time out."]
    #[inline(always)]
    pub const fn set_disable_time_out(&mut self, val: DisableTimeOut) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Stop At Block Gap value of automatic mode."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_blk_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop At Block Gap value of automatic mode."]
    #[inline(always)]
    pub const fn set_boot_blk_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for MmcBoot {
    #[inline(always)]
    fn default() -> MmcBoot {
        MmcBoot(0)
    }
}
impl core::fmt::Debug for MmcBoot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MmcBoot")
            .field("dtocv_ack", &self.dtocv_ack())
            .field("boot_ack", &self.boot_ack())
            .field("boot_mode", &self.boot_mode())
            .field("boot_en", &self.boot_en())
            .field("auto_sabg_en", &self.auto_sabg_en())
            .field("disable_time_out", &self.disable_time_out())
            .field("boot_blk_cnt", &self.boot_blk_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MmcBoot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "MmcBoot {{ dtocv_ack: {:?}, boot_ack: {:?}, boot_mode: {:?}, boot_en: {:?}, auto_sabg_en: {=bool:?}, disable_time_out: {:?}, boot_blk_cnt: {=u16:?} }}",
            self.dtocv_ack(),
            self.boot_ack(),
            self.boot_mode(),
            self.boot_en(),
            self.auto_sabg_en(),
            self.disable_time_out(),
            self.boot_blk_cnt()
        )
    }
}
#[doc = "Present State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresState(pub u32);
impl PresState {
    #[doc = "Command inhibit (CMD)."]
    #[must_use]
    #[inline(always)]
    pub const fn cihb(&self) -> Cihb {
        let val = (self.0 >> 0usize) & 0x01;
        Cihb::from_bits(val as u8)
    }
    #[doc = "Command inhibit (CMD)."]
    #[inline(always)]
    pub const fn set_cihb(&mut self, val: Cihb) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Command Inhibit Data (DATA)."]
    #[must_use]
    #[inline(always)]
    pub const fn cdihb(&self) -> Cdihb {
        let val = (self.0 >> 1usize) & 0x01;
        Cdihb::from_bits(val as u8)
    }
    #[doc = "Command Inhibit Data (DATA)."]
    #[inline(always)]
    pub const fn set_cdihb(&mut self, val: Cdihb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Data line active."]
    #[must_use]
    #[inline(always)]
    pub const fn dla(&self) -> Dla {
        let val = (self.0 >> 2usize) & 0x01;
        Dla::from_bits(val as u8)
    }
    #[doc = "Data line active."]
    #[inline(always)]
    pub const fn set_dla(&mut self, val: Dla) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SD clock stable."]
    #[must_use]
    #[inline(always)]
    pub const fn sdstb(&self) -> Sdstb {
        let val = (self.0 >> 3usize) & 0x01;
        Sdstb::from_bits(val as u8)
    }
    #[doc = "SD clock stable."]
    #[inline(always)]
    pub const fn set_sdstb(&mut self, val: Sdstb) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Write transfer active."]
    #[must_use]
    #[inline(always)]
    pub const fn wta(&self) -> Wta {
        let val = (self.0 >> 8usize) & 0x01;
        Wta::from_bits(val as u8)
    }
    #[doc = "Write transfer active."]
    #[inline(always)]
    pub const fn set_wta(&mut self, val: Wta) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Read transfer active."]
    #[must_use]
    #[inline(always)]
    pub const fn rta(&self) -> Rta {
        let val = (self.0 >> 9usize) & 0x01;
        Rta::from_bits(val as u8)
    }
    #[doc = "Read transfer active."]
    #[inline(always)]
    pub const fn set_rta(&mut self, val: Rta) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Buffer write enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bwen(&self) -> Bwen {
        let val = (self.0 >> 10usize) & 0x01;
        Bwen::from_bits(val as u8)
    }
    #[doc = "Buffer write enable."]
    #[inline(always)]
    pub const fn set_bwen(&mut self, val: Bwen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Buffer read enable."]
    #[must_use]
    #[inline(always)]
    pub const fn bren(&self) -> Bren {
        let val = (self.0 >> 11usize) & 0x01;
        Bren::from_bits(val as u8)
    }
    #[doc = "Buffer read enable."]
    #[inline(always)]
    pub const fn set_bren(&mut self, val: Bren) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode)."]
    #[must_use]
    #[inline(always)]
    pub const fn rtr(&self) -> Rtr {
        let val = (self.0 >> 12usize) & 0x01;
        Rtr::from_bits(val as u8)
    }
    #[doc = "Re-Tuning Request (only for SD3.0 SDR104 mode)."]
    #[inline(always)]
    pub const fn set_rtr(&mut self, val: Rtr) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Tap select change done."]
    #[must_use]
    #[inline(always)]
    pub const fn tscd(&self) -> Tscd {
        let val = (self.0 >> 15usize) & 0x01;
        Tscd::from_bits(val as u8)
    }
    #[doc = "Tap select change done."]
    #[inline(always)]
    pub const fn set_tscd(&mut self, val: Tscd) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Card inserted."]
    #[must_use]
    #[inline(always)]
    pub const fn cinst(&self) -> Cinst {
        let val = (self.0 >> 16usize) & 0x01;
        Cinst::from_bits(val as u8)
    }
    #[doc = "Card inserted."]
    #[inline(always)]
    pub const fn set_cinst(&mut self, val: Cinst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "CMD line signal level."]
    #[must_use]
    #[inline(always)]
    pub const fn clsl(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "CMD line signal level."]
    #[inline(always)]
    pub const fn set_clsl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "DATA\\[7:0\\] line signal level."]
    #[must_use]
    #[inline(always)]
    pub const fn dlsl(&self) -> Dlsl {
        let val = (self.0 >> 24usize) & 0xff;
        Dlsl::from_bits(val as u8)
    }
    #[doc = "DATA\\[7:0\\] line signal level."]
    #[inline(always)]
    pub const fn set_dlsl(&mut self, val: Dlsl) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for PresState {
    #[inline(always)]
    fn default() -> PresState {
        PresState(0)
    }
}
impl core::fmt::Debug for PresState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PresState")
            .field("cihb", &self.cihb())
            .field("cdihb", &self.cdihb())
            .field("dla", &self.dla())
            .field("sdstb", &self.sdstb())
            .field("wta", &self.wta())
            .field("rta", &self.rta())
            .field("bwen", &self.bwen())
            .field("bren", &self.bren())
            .field("rtr", &self.rtr())
            .field("tscd", &self.tscd())
            .field("cinst", &self.cinst())
            .field("clsl", &self.clsl())
            .field("dlsl", &self.dlsl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PresState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PresState {{ cihb: {:?}, cdihb: {:?}, dla: {:?}, sdstb: {:?}, wta: {:?}, rta: {:?}, bwen: {:?}, bren: {:?}, rtr: {:?}, tscd: {:?}, cinst: {:?}, clsl: {=bool:?}, dlsl: {:?} }}",
            self.cihb(),
            self.cdihb(),
            self.dla(),
            self.sdstb(),
            self.wta(),
            self.rta(),
            self.bwen(),
            self.bren(),
            self.rtr(),
            self.tscd(),
            self.cinst(),
            self.clsl(),
            self.dlsl()
        )
    }
}
#[doc = "Protocol Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProtCtrl(pub u32);
impl ProtCtrl {
    #[doc = "Data transfer width."]
    #[must_use]
    #[inline(always)]
    pub const fn dtw(&self) -> Dtw {
        let val = (self.0 >> 1usize) & 0x03;
        Dtw::from_bits(val as u8)
    }
    #[doc = "Data transfer width."]
    #[inline(always)]
    pub const fn set_dtw(&mut self, val: Dtw) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "DATA3 as card detection pin."]
    #[must_use]
    #[inline(always)]
    pub const fn d3cd(&self) -> D3cd {
        let val = (self.0 >> 3usize) & 0x01;
        D3cd::from_bits(val as u8)
    }
    #[doc = "DATA3 as card detection pin."]
    #[inline(always)]
    pub const fn set_d3cd(&mut self, val: D3cd) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Endian mode."]
    #[must_use]
    #[inline(always)]
    pub const fn emode(&self) -> Emode {
        let val = (self.0 >> 4usize) & 0x03;
        Emode::from_bits(val as u8)
    }
    #[doc = "Endian mode."]
    #[inline(always)]
    pub const fn set_emode(&mut self, val: Emode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA select."]
    #[must_use]
    #[inline(always)]
    pub const fn dmasel(&self) -> Dmasel {
        let val = (self.0 >> 8usize) & 0x03;
        Dmasel::from_bits(val as u8)
    }
    #[doc = "DMA select."]
    #[inline(always)]
    pub const fn set_dmasel(&mut self, val: Dmasel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Stop at block gap request."]
    #[must_use]
    #[inline(always)]
    pub const fn sabgreq(&self) -> Sabgreq {
        let val = (self.0 >> 16usize) & 0x01;
        Sabgreq::from_bits(val as u8)
    }
    #[doc = "Stop at block gap request."]
    #[inline(always)]
    pub const fn set_sabgreq(&mut self, val: Sabgreq) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Continue request."]
    #[must_use]
    #[inline(always)]
    pub const fn creq(&self) -> Creq {
        let val = (self.0 >> 17usize) & 0x01;
        Creq::from_bits(val as u8)
    }
    #[doc = "Continue request."]
    #[inline(always)]
    pub const fn set_creq(&mut self, val: Creq) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Read wait control."]
    #[must_use]
    #[inline(always)]
    pub const fn rwctl(&self) -> Rwctl {
        let val = (self.0 >> 18usize) & 0x01;
        Rwctl::from_bits(val as u8)
    }
    #[doc = "Read wait control."]
    #[inline(always)]
    pub const fn set_rwctl(&mut self, val: Rwctl) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Interrupt at block gap."]
    #[must_use]
    #[inline(always)]
    pub const fn iabg(&self) -> Iabg {
        let val = (self.0 >> 19usize) & 0x01;
        Iabg::from_bits(val as u8)
    }
    #[doc = "Interrupt at block gap."]
    #[inline(always)]
    pub const fn set_iabg(&mut self, val: Iabg) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Read performed number 8 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_done_no_8clk(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Read performed number 8 clock."]
    #[inline(always)]
    pub const fn set_rd_done_no_8clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Wakeup event enable on card interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wecint(&self) -> Wecint {
        let val = (self.0 >> 24usize) & 0x01;
        Wecint::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on card interrupt."]
    #[inline(always)]
    pub const fn set_wecint(&mut self, val: Wecint) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Wakeup event enable on SD card insertion."]
    #[must_use]
    #[inline(always)]
    pub const fn wecins(&self) -> Wecins {
        let val = (self.0 >> 25usize) & 0x01;
        Wecins::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on SD card insertion."]
    #[inline(always)]
    pub const fn set_wecins(&mut self, val: Wecins) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Wakeup event enable on SD card removal."]
    #[must_use]
    #[inline(always)]
    pub const fn wecrm(&self) -> Wecrm {
        let val = (self.0 >> 26usize) & 0x01;
        Wecrm::from_bits(val as u8)
    }
    #[doc = "Wakeup event enable on SD card removal."]
    #[inline(always)]
    pub const fn set_wecrm(&mut self, val: Wecrm) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP."]
    #[must_use]
    #[inline(always)]
    pub const fn burst_len_en(&self) -> BurstLenEn {
        let val = (self.0 >> 27usize) & 0x07;
        BurstLenEn::from_bits(val as u8)
    }
    #[doc = "BURST length enable for INCR, INCR4 / INCR8 / INCR16, INCR4-WRAP / INCR8-WRAP / INCR16-WRAP."]
    #[inline(always)]
    pub const fn set_burst_len_en(&mut self, val: BurstLenEn) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Non-exact block read."]
    #[must_use]
    #[inline(always)]
    pub const fn non_exact_blk_rd(&self) -> NonExactBlkRd {
        let val = (self.0 >> 30usize) & 0x01;
        NonExactBlkRd::from_bits(val as u8)
    }
    #[doc = "Non-exact block read."]
    #[inline(always)]
    pub const fn set_non_exact_blk_rd(&mut self, val: NonExactBlkRd) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for ProtCtrl {
    #[inline(always)]
    fn default() -> ProtCtrl {
        ProtCtrl(0)
    }
}
impl core::fmt::Debug for ProtCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ProtCtrl")
            .field("dtw", &self.dtw())
            .field("d3cd", &self.d3cd())
            .field("emode", &self.emode())
            .field("dmasel", &self.dmasel())
            .field("sabgreq", &self.sabgreq())
            .field("creq", &self.creq())
            .field("rwctl", &self.rwctl())
            .field("iabg", &self.iabg())
            .field("rd_done_no_8clk", &self.rd_done_no_8clk())
            .field("wecint", &self.wecint())
            .field("wecins", &self.wecins())
            .field("wecrm", &self.wecrm())
            .field("burst_len_en", &self.burst_len_en())
            .field("non_exact_blk_rd", &self.non_exact_blk_rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProtCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ProtCtrl {{ dtw: {:?}, d3cd: {:?}, emode: {:?}, dmasel: {:?}, sabgreq: {:?}, creq: {:?}, rwctl: {:?}, iabg: {:?}, rd_done_no_8clk: {=bool:?}, wecint: {:?}, wecins: {:?}, wecrm: {:?}, burst_len_en: {:?}, non_exact_blk_rd: {:?} }}",
            self.dtw(),
            self.d3cd(),
            self.emode(),
            self.dmasel(),
            self.sabgreq(),
            self.creq(),
            self.rwctl(),
            self.iabg(),
            self.rd_done_no_8clk(),
            self.wecint(),
            self.wecins(),
            self.wecrm(),
            self.burst_len_en(),
            self.non_exact_blk_rd()
        )
    }
}
#[doc = "System Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrl(pub u32);
impl SysCtrl {
    #[doc = "Divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn dvs(&self) -> Dvs {
        let val = (self.0 >> 4usize) & 0x0f;
        Dvs::from_bits(val as u8)
    }
    #[doc = "Divisor."]
    #[inline(always)]
    pub const fn set_dvs(&mut self, val: Dvs) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "SDCLK frequency select."]
    #[must_use]
    #[inline(always)]
    pub const fn sdclkfs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "SDCLK frequency select."]
    #[inline(always)]
    pub const fn set_sdclkfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Data timeout counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn dtocv(&self) -> Dtocv {
        let val = (self.0 >> 16usize) & 0x0f;
        Dtocv::from_bits(val as u8)
    }
    #[doc = "Data timeout counter value."]
    #[inline(always)]
    pub const fn set_dtocv(&mut self, val: Dtocv) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "Reset the async FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rst_fifo(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Reset the async FIFO."]
    #[inline(always)]
    pub const fn set_rst_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Hardware reset."]
    #[must_use]
    #[inline(always)]
    pub const fn ipp_rst_n(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware reset."]
    #[inline(always)]
    pub const fn set_ipp_rst_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Software reset for all."]
    #[must_use]
    #[inline(always)]
    pub const fn rsta(&self) -> Rsta {
        let val = (self.0 >> 24usize) & 0x01;
        Rsta::from_bits(val as u8)
    }
    #[doc = "Software reset for all."]
    #[inline(always)]
    pub const fn set_rsta(&mut self, val: Rsta) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Software reset for CMD line."]
    #[must_use]
    #[inline(always)]
    pub const fn rstc(&self) -> Rstc {
        let val = (self.0 >> 25usize) & 0x01;
        Rstc::from_bits(val as u8)
    }
    #[doc = "Software reset for CMD line."]
    #[inline(always)]
    pub const fn set_rstc(&mut self, val: Rstc) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Software reset for data line."]
    #[must_use]
    #[inline(always)]
    pub const fn rstd(&self) -> Rstd {
        let val = (self.0 >> 26usize) & 0x01;
        Rstd::from_bits(val as u8)
    }
    #[doc = "Software reset for data line."]
    #[inline(always)]
    pub const fn set_rstd(&mut self, val: Rstd) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Initialization active."]
    #[must_use]
    #[inline(always)]
    pub const fn inita(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Initialization active."]
    #[inline(always)]
    pub const fn set_inita(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Reset tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn rstt(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Reset tuning."]
    #[inline(always)]
    pub const fn set_rstt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for SysCtrl {
    #[inline(always)]
    fn default() -> SysCtrl {
        SysCtrl(0)
    }
}
impl core::fmt::Debug for SysCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SysCtrl")
            .field("dvs", &self.dvs())
            .field("sdclkfs", &self.sdclkfs())
            .field("dtocv", &self.dtocv())
            .field("rst_fifo", &self.rst_fifo())
            .field("ipp_rst_n", &self.ipp_rst_n())
            .field("rsta", &self.rsta())
            .field("rstc", &self.rstc())
            .field("rstd", &self.rstd())
            .field("inita", &self.inita())
            .field("rstt", &self.rstt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SysCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SysCtrl {{ dvs: {:?}, sdclkfs: {=u8:?}, dtocv: {:?}, rst_fifo: {=bool:?}, ipp_rst_n: {=bool:?}, rsta: {:?}, rstc: {:?}, rstd: {:?}, inita: {=bool:?}, rstt: {=bool:?} }}",
            self.dvs(),
            self.sdclkfs(),
            self.dtocv(),
            self.rst_fifo(),
            self.ipp_rst_n(),
            self.rsta(),
            self.rstc(),
            self.rstd(),
            self.inita(),
            self.rstt()
        )
    }
}
#[doc = "Tuning Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TuningCtrl(pub u32);
impl TuningCtrl {
    #[doc = "Tuning start."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_start_tap(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Tuning start."]
    #[inline(always)]
    pub const fn set_tuning_start_tap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Disable command check for standard tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_cmd_chk_for_std_tuning(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Disable command check for standard tuning."]
    #[inline(always)]
    pub const fn set_dis_cmd_chk_for_std_tuning(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Tuning counter."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_counter(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Tuning counter."]
    #[inline(always)]
    pub const fn set_tuning_counter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "TUNING_STEP."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_step(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "TUNING_STEP."]
    #[inline(always)]
    pub const fn set_tuning_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Data window."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_window(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Data window."]
    #[inline(always)]
    pub const fn set_tuning_window(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Standard tuning circuit and procedure enable."]
    #[must_use]
    #[inline(always)]
    pub const fn std_tuning_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Standard tuning circuit and procedure enable."]
    #[inline(always)]
    pub const fn set_std_tuning_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for TuningCtrl {
    #[inline(always)]
    fn default() -> TuningCtrl {
        TuningCtrl(0)
    }
}
impl core::fmt::Debug for TuningCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TuningCtrl")
            .field("tuning_start_tap", &self.tuning_start_tap())
            .field(
                "dis_cmd_chk_for_std_tuning",
                &self.dis_cmd_chk_for_std_tuning(),
            )
            .field("tuning_counter", &self.tuning_counter())
            .field("tuning_step", &self.tuning_step())
            .field("tuning_window", &self.tuning_window())
            .field("std_tuning_en", &self.std_tuning_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TuningCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TuningCtrl {{ tuning_start_tap: {=u8:?}, dis_cmd_chk_for_std_tuning: {=bool:?}, tuning_counter: {=u8:?}, tuning_step: {=u8:?}, tuning_window: {=u8:?}, std_tuning_en: {=bool:?} }}",
            self.tuning_start_tap(),
            self.dis_cmd_chk_for_std_tuning(),
            self.tuning_counter(),
            self.tuning_step(),
            self.tuning_window(),
            self.std_tuning_en()
        )
    }
}
#[doc = "Vendor Specific Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec(pub u32);
impl VendSpec {
    #[doc = "Check busy enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ac12_wr_chkbusy_en(&self) -> Ac12WrChkbusyEn {
        let val = (self.0 >> 3usize) & 0x01;
        Ac12WrChkbusyEn::from_bits(val as u8)
    }
    #[doc = "Check busy enable."]
    #[inline(always)]
    pub const fn set_ac12_wr_chkbusy_en(&mut self, val: Ac12WrChkbusyEn) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Force CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_sdclk_on(&self) -> FrcSdclkOn {
        let val = (self.0 >> 8usize) & 0x01;
        FrcSdclkOn::from_bits(val as u8)
    }
    #[doc = "Force CLK."]
    #[inline(always)]
    pub const fn set_frc_sdclk_on(&mut self, val: FrcSdclkOn) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CRC Check Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_chk_dis(&self) -> CrcChkDis {
        let val = (self.0 >> 15usize) & 0x01;
        CrcChkDis::from_bits(val as u8)
    }
    #[doc = "CRC Check Disable."]
    #[inline(always)]
    pub const fn set_crc_chk_dis(&mut self, val: CrcChkDis) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Register byte access for CMD_XFR_TYP."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_byte_en(&self) -> CmdByteEn {
        let val = (self.0 >> 31usize) & 0x01;
        CmdByteEn::from_bits(val as u8)
    }
    #[doc = "Register byte access for CMD_XFR_TYP."]
    #[inline(always)]
    pub const fn set_cmd_byte_en(&mut self, val: CmdByteEn) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for VendSpec {
    #[inline(always)]
    fn default() -> VendSpec {
        VendSpec(0)
    }
}
impl core::fmt::Debug for VendSpec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendSpec")
            .field("ac12_wr_chkbusy_en", &self.ac12_wr_chkbusy_en())
            .field("frc_sdclk_on", &self.frc_sdclk_on())
            .field("crc_chk_dis", &self.crc_chk_dis())
            .field("cmd_byte_en", &self.cmd_byte_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendSpec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendSpec {{ ac12_wr_chkbusy_en: {:?}, frc_sdclk_on: {:?}, crc_chk_dis: {:?}, cmd_byte_en: {:?} }}",
            self.ac12_wr_chkbusy_en(),
            self.frc_sdclk_on(),
            self.crc_chk_dis(),
            self.cmd_byte_en()
        )
    }
}
#[doc = "Vendor Specific 2 Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VendSpec2(pub u32);
impl VendSpec2 {
    #[doc = "Card interrupt detection test."]
    #[must_use]
    #[inline(always)]
    pub const fn card_int_d3_test(&self) -> CardIntD3Test {
        let val = (self.0 >> 3usize) & 0x01;
        CardIntD3Test::from_bits(val as u8)
    }
    #[doc = "Card interrupt detection test."]
    #[inline(always)]
    pub const fn set_card_int_d3_test(&mut self, val: CardIntD3Test) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Tuning bit enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_bit_en(&self) -> TuningBitEn {
        let val = (self.0 >> 4usize) & 0x03;
        TuningBitEn::from_bits(val as u8)
    }
    #[doc = "Tuning bit enable."]
    #[inline(always)]
    pub const fn set_tuning_bit_en(&mut self, val: TuningBitEn) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Tuning command enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tuning_cmd_en(&self) -> TuningCmdEn {
        let val = (self.0 >> 6usize) & 0x01;
        TuningCmdEn::from_bits(val as u8)
    }
    #[doc = "Tuning command enable."]
    #[inline(always)]
    pub const fn set_tuning_cmd_en(&mut self, val: TuningCmdEn) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Argument2 register enable for ACMD23."]
    #[must_use]
    #[inline(always)]
    pub const fn acmd23_argu2_en(&self) -> Acmd23Argu2En {
        let val = (self.0 >> 12usize) & 0x01;
        Acmd23Argu2En::from_bits(val as u8)
    }
    #[doc = "Argument2 register enable for ACMD23."]
    #[inline(always)]
    pub const fn set_acmd23_argu2_en(&mut self, val: Acmd23Argu2En) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Select the clock source for host card detection."]
    #[must_use]
    #[inline(always)]
    pub const fn en_32k_clk(&self) -> En32kClk {
        let val = (self.0 >> 15usize) & 0x01;
        En32kClk::from_bits(val as u8)
    }
    #[doc = "Select the clock source for host card detection."]
    #[inline(always)]
    pub const fn set_en_32k_clk(&mut self, val: En32kClk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for VendSpec2 {
    #[inline(always)]
    fn default() -> VendSpec2 {
        VendSpec2(0)
    }
}
impl core::fmt::Debug for VendSpec2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VendSpec2")
            .field("card_int_d3_test", &self.card_int_d3_test())
            .field("tuning_bit_en", &self.tuning_bit_en())
            .field("tuning_cmd_en", &self.tuning_cmd_en())
            .field("acmd23_argu2_en", &self.acmd23_argu2_en())
            .field("en_32k_clk", &self.en_32k_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VendSpec2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VendSpec2 {{ card_int_d3_test: {:?}, tuning_bit_en: {:?}, tuning_cmd_en: {:?}, acmd23_argu2_en: {:?}, en_32k_clk: {:?} }}",
            self.card_int_d3_test(),
            self.tuning_bit_en(),
            self.tuning_cmd_en(),
            self.acmd23_argu2_en(),
            self.en_32k_clk()
        )
    }
}
#[doc = "Watermark Level."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WtmkLvl(pub u32);
impl WtmkLvl {
    #[doc = "Read watermark level."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_wml(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Read watermark level."]
    #[inline(always)]
    pub const fn set_rd_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Read burst length due to system restriction, the actual burst length might not exceed 16."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_brst_len(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Read burst length due to system restriction, the actual burst length might not exceed 16."]
    #[inline(always)]
    pub const fn set_rd_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Write watermark level."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_wml(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write watermark level."]
    #[inline(always)]
    pub const fn set_wr_wml(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Write burst length due to system restriction, the actual burst length might not exceed 16."]
    #[must_use]
    #[inline(always)]
    pub const fn wr_brst_len(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Write burst length due to system restriction, the actual burst length might not exceed 16."]
    #[inline(always)]
    pub const fn set_wr_brst_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for WtmkLvl {
    #[inline(always)]
    fn default() -> WtmkLvl {
        WtmkLvl(0)
    }
}
impl core::fmt::Debug for WtmkLvl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WtmkLvl")
            .field("rd_wml", &self.rd_wml())
            .field("rd_brst_len", &self.rd_brst_len())
            .field("wr_wml", &self.wr_wml())
            .field("wr_brst_len", &self.wr_brst_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WtmkLvl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WtmkLvl {{ rd_wml: {=u8:?}, rd_brst_len: {=u8:?}, wr_wml: {=u8:?}, wr_brst_len: {=u8:?} }}",
            self.rd_wml(),
            self.rd_brst_len(),
            self.wr_wml(),
            self.wr_brst_len()
        )
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12WrChkbusyEn {
    #[doc = "Do not check busy after auto CMD12 for write data packet."]
    Ac12WrChkbusyEnA = 0x0,
    #[doc = "Check busy after auto CMD12 for write data packet."]
    Ac12WrChkbusyEnB = 0x01,
}
impl Ac12WrChkbusyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12WrChkbusyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12WrChkbusyEn {
    #[inline(always)]
    fn from(val: u8) -> Ac12WrChkbusyEn {
        Ac12WrChkbusyEn::from_bits(val)
    }
}
impl From<Ac12WrChkbusyEn> for u8 {
    #[inline(always)]
    fn from(val: Ac12WrChkbusyEn) -> u8 {
        Ac12WrChkbusyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ce {
    #[doc = "No CRC error."]
    Ac12ceB = 0x0,
    #[doc = "CRC error met in Auto CMD12/23 response."]
    Ac12ceA = 0x01,
}
impl Ac12ce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ce {
    #[inline(always)]
    fn from(val: u8) -> Ac12ce {
        Ac12ce::from_bits(val)
    }
}
impl From<Ac12ce> for u8 {
    #[inline(always)]
    fn from(val: Ac12ce) -> u8 {
        Ac12ce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12e {
    #[doc = "No error."]
    Ac12eA = 0x0,
    #[doc = "Error."]
    Ac12eB = 0x01,
}
impl Ac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12e {
    #[inline(always)]
    fn from(val: u8) -> Ac12e {
        Ac12e::from_bits(val)
    }
}
impl From<Ac12e> for u8 {
    #[inline(always)]
    fn from(val: Ac12e) -> u8 {
        Ac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ebe {
    #[doc = "No error."]
    Ac12ebeB = 0x0,
    #[doc = "End bit error generated."]
    Ac12ebeA = 0x01,
}
impl Ac12ebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ebe {
    #[inline(always)]
    fn from(val: u8) -> Ac12ebe {
        Ac12ebe::from_bits(val)
    }
}
impl From<Ac12ebe> for u8 {
    #[inline(always)]
    fn from(val: Ac12ebe) -> u8 {
        Ac12ebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12eien {
    #[doc = "Masked."]
    Ac12eienB = 0x0,
    #[doc = "Enabled."]
    Ac12eienA = 0x01,
}
impl Ac12eien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12eien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12eien {
    #[inline(always)]
    fn from(val: u8) -> Ac12eien {
        Ac12eien::from_bits(val)
    }
}
impl From<Ac12eien> for u8 {
    #[inline(always)]
    fn from(val: Ac12eien) -> u8 {
        Ac12eien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12esen {
    #[doc = "Masked."]
    Ac12esenA = 0x0,
    #[doc = "Enabled."]
    Ac12esenB = 0x01,
}
impl Ac12esen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12esen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12esen {
    #[inline(always)]
    fn from(val: u8) -> Ac12esen {
        Ac12esen::from_bits(val)
    }
}
impl From<Ac12esen> for u8 {
    #[inline(always)]
    fn from(val: Ac12esen) -> u8 {
        Ac12esen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ie {
    #[doc = "No error."]
    Ac12ieB = 0x0,
    #[doc = "Error, the CMD index in response is not CMD12/23."]
    Ac12ieA = 0x01,
}
impl Ac12ie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ie {
    #[inline(always)]
    fn from(val: u8) -> Ac12ie {
        Ac12ie::from_bits(val)
    }
}
impl From<Ac12ie> for u8 {
    #[inline(always)]
    fn from(val: Ac12ie) -> u8 {
        Ac12ie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12ne {
    #[doc = "Executed."]
    Ac12neB = 0x0,
    #[doc = "Not executed."]
    Ac12neA = 0x01,
}
impl Ac12ne {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12ne {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12ne {
    #[inline(always)]
    fn from(val: u8) -> Ac12ne {
        Ac12ne::from_bits(val)
    }
}
impl From<Ac12ne> for u8 {
    #[inline(always)]
    fn from(val: Ac12ne) -> u8 {
        Ac12ne::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac12toe {
    #[doc = "No error."]
    Ac12toeB = 0x0,
    #[doc = "Time out."]
    Ac12toeA = 0x01,
}
impl Ac12toe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac12toe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac12toe {
    #[inline(always)]
    fn from(val: u8) -> Ac12toe {
        Ac12toe::from_bits(val)
    }
}
impl From<Ac12toe> for u8 {
    #[inline(always)]
    fn from(val: Ac12toe) -> u8 {
        Ac12toe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ac23en {
    #[doc = "Disable."]
    CmdXfrTyp7B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp7A = 0x01,
}
impl Ac23en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ac23en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ac23en {
    #[inline(always)]
    fn from(val: u8) -> Ac23en {
        Ac23en::from_bits(val)
    }
}
impl From<Ac23en> for u8 {
    #[inline(always)]
    fn from(val: Ac23en) -> u8 {
        Ac23en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Acmd23Argu2En {
    #[doc = "Disable."]
    Acmd23Argu2EnB = 0x0,
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enabled."]
    Acmd23Argu2EnA = 0x01,
}
impl Acmd23Argu2En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Acmd23Argu2En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Acmd23Argu2En {
    #[inline(always)]
    fn from(val: u8) -> Acmd23Argu2En {
        Acmd23Argu2En::from_bits(val)
    }
}
impl From<Acmd23Argu2En> for u8 {
    #[inline(always)]
    fn from(val: Acmd23Argu2En) -> u8 {
        Acmd23Argu2En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admadce {
    #[doc = "No error."]
    AdmadceB = 0x0,
    #[doc = "Error."]
    AdmadceA = 0x01,
}
impl Admadce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admadce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admadce {
    #[inline(always)]
    fn from(val: u8) -> Admadce {
        Admadce::from_bits(val)
    }
}
impl From<Admadce> for u8 {
    #[inline(always)]
    fn from(val: Admadce) -> u8 {
        Admadce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admalme {
    #[doc = "No error."]
    AdmaesB = 0x0,
    #[doc = "Error."]
    AdmaesA = 0x01,
}
impl Admalme {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admalme {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admalme {
    #[inline(always)]
    fn from(val: u8) -> Admalme {
        Admalme::from_bits(val)
    }
}
impl From<Admalme> for u8 {
    #[inline(always)]
    fn from(val: Admalme) -> u8 {
        Admalme::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Admas {
    #[doc = "Advanced DMA not supported."]
    AdmasB = 0x0,
    #[doc = "Advanced DMA supported."]
    AdmasA = 0x01,
}
impl Admas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Admas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Admas {
    #[inline(always)]
    fn from(val: u8) -> Admas {
        Admas::from_bits(val)
    }
}
impl From<Admas> for u8 {
    #[inline(always)]
    fn from(val: Admas) -> u8 {
        Admas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AutoTuneEn {
    #[doc = "Disable auto tuning."]
    AutotuneB = 0x0,
    #[doc = "Enable auto tuning."]
    AutoTuneA = 0x01,
}
impl AutoTuneEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AutoTuneEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AutoTuneEn {
    #[inline(always)]
    fn from(val: u8) -> AutoTuneEn {
        AutoTuneEn::from_bits(val)
    }
}
impl From<AutoTuneEn> for u8 {
    #[inline(always)]
    fn from(val: AutoTuneEn) -> u8 {
        AutoTuneEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Autocmd12ErrStatusSmpClkSel {
    #[doc = "Fixed clock is used to sample data."]
    SmpClkB = 0x0,
    #[doc = "Tuned clock is used to sample data."]
    SmpClkA = 0x01,
}
impl Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Autocmd12ErrStatusSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> Autocmd12ErrStatusSmpClkSel {
        Autocmd12ErrStatusSmpClkSel::from_bits(val)
    }
}
impl From<Autocmd12ErrStatusSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: Autocmd12ErrStatusSmpClkSel) -> u8 {
        Autocmd12ErrStatusSmpClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bge {
    #[doc = "No block gap event."]
    BgeB = 0x0,
    #[doc = "Transaction stopped at block gap."]
    BgeA = 0x01,
}
impl Bge {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bge {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bge {
    #[inline(always)]
    fn from(val: u8) -> Bge {
        Bge::from_bits(val)
    }
}
impl From<Bge> for u8 {
    #[inline(always)]
    fn from(val: Bge) -> u8 {
        Bge::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bgeien {
    #[doc = "Masked."]
    BgienA = 0x0,
    #[doc = "Enabled."]
    BgienB = 0x01,
}
impl Bgeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgeien {
    #[inline(always)]
    fn from(val: u8) -> Bgeien {
        Bgeien::from_bits(val)
    }
}
impl From<Bgeien> for u8 {
    #[inline(always)]
    fn from(val: Bgeien) -> u8 {
        Bgeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bgesen {
    #[doc = "Masked."]
    BgesenA = 0x0,
    #[doc = "Enabled."]
    BgesenB = 0x01,
}
impl Bgesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bgesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bgesen {
    #[inline(always)]
    fn from(val: u8) -> Bgesen {
        Bgesen::from_bits(val)
    }
}
impl From<Bgesen> for u8 {
    #[inline(always)]
    fn from(val: Bgesen) -> u8 {
        Bgesen::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blkcnt(u16);
impl Blkcnt {
    #[doc = "Stop count."]
    pub const BlkcntD: Self = Self(0x0);
    #[doc = "1 block."]
    pub const BlkcntC: Self = Self(0x01);
    #[doc = "2 blocks."]
    pub const BlkcntB: Self = Self(0x02);
    #[doc = "65535 blocks."]
    pub const BlkcntA: Self = Self(0xffff);
}
impl Blkcnt {
    pub const fn from_bits(val: u16) -> Blkcnt {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Blkcnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BlkcntD"),
            0x01 => f.write_str("BlkcntC"),
            0x02 => f.write_str("BlkcntB"),
            0xffff => f.write_str("BlkcntA"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blkcnt {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BlkcntD"),
            0x01 => defmt::write!(f, "BlkcntC"),
            0x02 => defmt::write!(f, "BlkcntB"),
            0xffff => defmt::write!(f, "BlkcntA"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Blkcnt {
    #[inline(always)]
    fn from(val: u16) -> Blkcnt {
        Blkcnt::from_bits(val)
    }
}
impl From<Blkcnt> for u16 {
    #[inline(always)]
    fn from(val: Blkcnt) -> u16 {
        Blkcnt::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Blksize(u16);
impl Blksize {
    #[doc = "No data transfer."]
    pub const BlkAttI: Self = Self(0x0);
    #[doc = "1 byte."]
    pub const BlkAttH: Self = Self(0x01);
    #[doc = "2 bytes."]
    pub const BlkAttG: Self = Self(0x02);
    #[doc = "3 bytes."]
    pub const BlkAttF: Self = Self(0x03);
    #[doc = "4 bytes."]
    pub const BlkAttE: Self = Self(0x04);
    #[doc = "511 bytes."]
    pub const BlkAttD: Self = Self(0x01ff);
    #[doc = "512 bytes."]
    pub const BlkAttC: Self = Self(0x0200);
    #[doc = "2048 bytes."]
    pub const BlkAttB: Self = Self(0x0800);
    #[doc = "4096 bytes."]
    pub const BlkAttA: Self = Self(0x1000);
}
impl Blksize {
    pub const fn from_bits(val: u16) -> Blksize {
        Self(val & 0x1fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Blksize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BlkAttI"),
            0x01 => f.write_str("BlkAttH"),
            0x02 => f.write_str("BlkAttG"),
            0x03 => f.write_str("BlkAttF"),
            0x04 => f.write_str("BlkAttE"),
            0x01ff => f.write_str("BlkAttD"),
            0x0200 => f.write_str("BlkAttC"),
            0x0800 => f.write_str("BlkAttB"),
            0x1000 => f.write_str("BlkAttA"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blksize {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BlkAttI"),
            0x01 => defmt::write!(f, "BlkAttH"),
            0x02 => defmt::write!(f, "BlkAttG"),
            0x03 => defmt::write!(f, "BlkAttF"),
            0x04 => defmt::write!(f, "BlkAttE"),
            0x01ff => defmt::write!(f, "BlkAttD"),
            0x0200 => defmt::write!(f, "BlkAttC"),
            0x0800 => defmt::write!(f, "BlkAttB"),
            0x1000 => defmt::write!(f, "BlkAttA"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Blksize {
    #[inline(always)]
    fn from(val: u16) -> Blksize {
        Blksize::from_bits(val)
    }
}
impl From<Blksize> for u16 {
    #[inline(always)]
    fn from(val: Blksize) -> u16 {
        Blksize::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootAck {
    #[doc = "No ack."]
    BootAckA = 0x0,
    #[doc = "Ack."]
    BootAckB = 0x01,
}
impl BootAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootAck {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootAck {
    #[inline(always)]
    fn from(val: u8) -> BootAck {
        BootAck::from_bits(val)
    }
}
impl From<BootAck> for u8 {
    #[inline(always)]
    fn from(val: BootAck) -> u8 {
        BootAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootEn {
    #[doc = "Fast boot disable."]
    BootEnA = 0x0,
    #[doc = "Fast boot enable."]
    BootEnB = 0x01,
}
impl BootEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootEn {
    #[inline(always)]
    fn from(val: u8) -> BootEn {
        BootEn::from_bits(val)
    }
}
impl From<BootEn> for u8 {
    #[inline(always)]
    fn from(val: BootEn) -> u8 {
        BootEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootMode {
    #[doc = "Normal boot."]
    BootModeA = 0x0,
    #[doc = "Alternative boot."]
    BootModeB = 0x01,
}
impl BootMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootMode {
    #[inline(always)]
    fn from(val: u8) -> BootMode {
        BootMode::from_bits(val)
    }
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(val: BootMode) -> u8 {
        BootMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bren {
    #[doc = "Read disable."]
    BrenB = 0x0,
    #[doc = "Read enable."]
    BrenA = 0x01,
}
impl Bren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bren {
    #[inline(always)]
    fn from(val: u8) -> Bren {
        Bren::from_bits(val)
    }
}
impl From<Bren> for u8 {
    #[inline(always)]
    fn from(val: Bren) -> u8 {
        Bren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brr {
    #[doc = "Not ready to read buffer."]
    BrrB = 0x0,
    #[doc = "Ready to read buffer."]
    BrrA = 0x01,
}
impl Brr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brr {
    #[inline(always)]
    fn from(val: u8) -> Brr {
        Brr::from_bits(val)
    }
}
impl From<Brr> for u8 {
    #[inline(always)]
    fn from(val: Brr) -> u8 {
        Brr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brrien {
    #[doc = "Masked."]
    BrrienB = 0x0,
    #[doc = "Enabled."]
    BrrienA = 0x01,
}
impl Brrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrien {
    #[inline(always)]
    fn from(val: u8) -> Brrien {
        Brrien::from_bits(val)
    }
}
impl From<Brrien> for u8 {
    #[inline(always)]
    fn from(val: Brrien) -> u8 {
        Brrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Brrsen {
    #[doc = "Masked."]
    BrrsenA = 0x0,
    #[doc = "Enabled."]
    BrrenB = 0x01,
}
impl Brrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Brrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Brrsen {
    #[inline(always)]
    fn from(val: u8) -> Brrsen {
        Brrsen::from_bits(val)
    }
}
impl From<Brrsen> for u8 {
    #[inline(always)]
    fn from(val: Brrsen) -> u8 {
        Brrsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BurstLenEn {
    _RESERVED_0 = 0x0,
    #[doc = "Burst length is enabled for INCR."]
    BurstA = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl BurstLenEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BurstLenEn {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BurstLenEn {
    #[inline(always)]
    fn from(val: u8) -> BurstLenEn {
        BurstLenEn::from_bits(val)
    }
}
impl From<BurstLenEn> for u8 {
    #[inline(always)]
    fn from(val: BurstLenEn) -> u8 {
        BurstLenEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwen {
    #[doc = "Write disable."]
    BwenB = 0x0,
    #[doc = "Write enable."]
    BwenA = 0x01,
}
impl Bwen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwen {
    #[inline(always)]
    fn from(val: u8) -> Bwen {
        Bwen::from_bits(val)
    }
}
impl From<Bwen> for u8 {
    #[inline(always)]
    fn from(val: Bwen) -> u8 {
        Bwen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwr {
    #[doc = "Not ready to write buffer."]
    BwrB = 0x0,
    #[doc = "Ready to write buffer."]
    BwrA = 0x01,
}
impl Bwr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwr {
    #[inline(always)]
    fn from(val: u8) -> Bwr {
        Bwr::from_bits(val)
    }
}
impl From<Bwr> for u8 {
    #[inline(always)]
    fn from(val: Bwr) -> u8 {
        Bwr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwrien {
    #[doc = "Masked."]
    BwrienA = 0x0,
    #[doc = "Enabled."]
    BwrienB = 0x01,
}
impl Bwrien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrien {
    #[inline(always)]
    fn from(val: u8) -> Bwrien {
        Bwrien::from_bits(val)
    }
}
impl From<Bwrien> for u8 {
    #[inline(always)]
    fn from(val: Bwrien) -> u8 {
        Bwrien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwrsen {
    #[doc = "Masked."]
    BwrsenA = 0x0,
    #[doc = "Enabled."]
    BwrsenB = 0x01,
}
impl Bwrsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwrsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwrsen {
    #[inline(always)]
    fn from(val: u8) -> Bwrsen {
        Bwrsen::from_bits(val)
    }
}
impl From<Bwrsen> for u8 {
    #[inline(always)]
    fn from(val: Bwrsen) -> u8 {
        Bwrsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CardIntD3Test {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    CardIntD3A = 0x0,
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    CardIntD3B = 0x01,
}
impl CardIntD3Test {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CardIntD3Test {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CardIntD3Test {
    #[inline(always)]
    fn from(val: u8) -> CardIntD3Test {
        CardIntD3Test::from_bits(val)
    }
}
impl From<CardIntD3Test> for u8 {
    #[inline(always)]
    fn from(val: CardIntD3Test) -> u8 {
        CardIntD3Test::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cc {
    #[doc = "Command not complete."]
    CcB = 0x0,
    #[doc = "Command complete."]
    CcA = 0x01,
}
impl Cc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cc {
    #[inline(always)]
    fn from(val: u8) -> Cc {
        Cc::from_bits(val)
    }
}
impl From<Cc> for u8 {
    #[inline(always)]
    fn from(val: Cc) -> u8 {
        Cc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cccen {
    #[doc = "Disables command CRC check."]
    CccenB = 0x0,
    #[doc = "Enables command CRC check."]
    CccenA = 0x01,
}
impl Cccen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cccen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cccen {
    #[inline(always)]
    fn from(val: u8) -> Cccen {
        Cccen::from_bits(val)
    }
}
impl From<Cccen> for u8 {
    #[inline(always)]
    fn from(val: Cccen) -> u8 {
        Cccen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cce {
    #[doc = "No error."]
    CceA = 0x0,
    #[doc = "CRC error generated."]
    CceB = 0x01,
}
impl Cce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cce {
    #[inline(always)]
    fn from(val: u8) -> Cce {
        Cce::from_bits(val)
    }
}
impl From<Cce> for u8 {
    #[inline(always)]
    fn from(val: Cce) -> u8 {
        Cce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cceien {
    #[doc = "Masked."]
    CceienB = 0x0,
    #[doc = "Enabled."]
    CceienA = 0x01,
}
impl Cceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cceien {
    #[inline(always)]
    fn from(val: u8) -> Cceien {
        Cceien::from_bits(val)
    }
}
impl From<Cceien> for u8 {
    #[inline(always)]
    fn from(val: Cceien) -> u8 {
        Cceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccesen {
    #[doc = "Masked."]
    CcesenA = 0x0,
    #[doc = "Enabled."]
    CcesenB = 0x01,
}
impl Ccesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccesen {
    #[inline(always)]
    fn from(val: u8) -> Ccesen {
        Ccesen::from_bits(val)
    }
}
impl From<Ccesen> for u8 {
    #[inline(always)]
    fn from(val: Ccesen) -> u8 {
        Ccesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccien {
    #[doc = "Masked."]
    CcienA = 0x0,
    #[doc = "Enabled."]
    CcienB = 0x01,
}
impl Ccien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccien {
    #[inline(always)]
    fn from(val: u8) -> Ccien {
        Ccien::from_bits(val)
    }
}
impl From<Ccien> for u8 {
    #[inline(always)]
    fn from(val: Ccien) -> u8 {
        Ccien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ccsen {
    #[doc = "Masked."]
    CcsenA = 0x0,
    #[doc = "Enabled."]
    CcsenB = 0x01,
}
impl Ccsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ccsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ccsen {
    #[inline(always)]
    fn from(val: u8) -> Ccsen {
        Ccsen::from_bits(val)
    }
}
impl From<Ccsen> for u8 {
    #[inline(always)]
    fn from(val: Ccsen) -> u8 {
        Ccsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cdihb {
    #[doc = "Can issue command that uses the DATA line."]
    CdihbB = 0x0,
    #[doc = "Cannot issue command that uses the DATA line."]
    CdihbA = 0x01,
}
impl Cdihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdihb {
    #[inline(always)]
    fn from(val: u8) -> Cdihb {
        Cdihb::from_bits(val)
    }
}
impl From<Cdihb> for u8 {
    #[inline(always)]
    fn from(val: Cdihb) -> u8 {
        Cdihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebe {
    #[doc = "No error."]
    CebeA = 0x0,
    #[doc = "End bit error generated."]
    CebeB = 0x01,
}
impl Cebe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebe {
    #[inline(always)]
    fn from(val: u8) -> Cebe {
        Cebe::from_bits(val)
    }
}
impl From<Cebe> for u8 {
    #[inline(always)]
    fn from(val: Cebe) -> u8 {
        Cebe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebeien {
    #[doc = "Masked."]
    CebeienB = 0x0,
    #[doc = "Enabled."]
    CebeienA = 0x01,
}
impl Cebeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebeien {
    #[inline(always)]
    fn from(val: u8) -> Cebeien {
        Cebeien::from_bits(val)
    }
}
impl From<Cebeien> for u8 {
    #[inline(always)]
    fn from(val: Cebeien) -> u8 {
        Cebeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cebesen {
    #[doc = "Masked."]
    CebesenA = 0x0,
    #[doc = "Enabled."]
    CebesenB = 0x01,
}
impl Cebesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cebesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cebesen {
    #[inline(always)]
    fn from(val: u8) -> Cebesen {
        Cebesen::from_bits(val)
    }
}
impl From<Cebesen> for u8 {
    #[inline(always)]
    fn from(val: Cebesen) -> u8 {
        Cebesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cicen {
    #[doc = "Disable command index check."]
    CicenB = 0x0,
    #[doc = "Enables command index check."]
    CicenA = 0x01,
}
impl Cicen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cicen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cicen {
    #[inline(always)]
    fn from(val: u8) -> Cicen {
        Cicen::from_bits(val)
    }
}
impl From<Cicen> for u8 {
    #[inline(always)]
    fn from(val: Cicen) -> u8 {
        Cicen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cie {
    #[doc = "No error."]
    CieA = 0x0,
    #[doc = "Error."]
    CieB = 0x01,
}
impl Cie {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cie {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cie {
    #[inline(always)]
    fn from(val: u8) -> Cie {
        Cie::from_bits(val)
    }
}
impl From<Cie> for u8 {
    #[inline(always)]
    fn from(val: Cie) -> u8 {
        Cie::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cieien {
    #[doc = "Masked."]
    CieienB = 0x0,
    #[doc = "Enabled."]
    CieienA = 0x01,
}
impl Cieien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cieien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cieien {
    #[inline(always)]
    fn from(val: u8) -> Cieien {
        Cieien::from_bits(val)
    }
}
impl From<Cieien> for u8 {
    #[inline(always)]
    fn from(val: Cieien) -> u8 {
        Cieien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ciesen {
    #[doc = "Masked."]
    CiesenA = 0x0,
    #[doc = "Enabled."]
    CiesenB = 0x01,
}
impl Ciesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ciesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ciesen {
    #[inline(always)]
    fn from(val: u8) -> Ciesen {
        Ciesen::from_bits(val)
    }
}
impl From<Ciesen> for u8 {
    #[inline(always)]
    fn from(val: Ciesen) -> u8 {
        Ciesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cihb {
    #[doc = "Can issue command using only CMD line."]
    CihbA = 0x0,
    #[doc = "Cannot issue command."]
    CihbB = 0x01,
}
impl Cihb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cihb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cihb {
    #[inline(always)]
    fn from(val: u8) -> Cihb {
        Cihb::from_bits(val)
    }
}
impl From<Cihb> for u8 {
    #[inline(always)]
    fn from(val: Cihb) -> u8 {
        Cihb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cins {
    #[doc = "Card state unstable or removed."]
    BwrB = 0x0,
    #[doc = "Card inserted."]
    BwrA = 0x01,
}
impl Cins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cins {
    #[inline(always)]
    fn from(val: u8) -> Cins {
        Cins::from_bits(val)
    }
}
impl From<Cins> for u8 {
    #[inline(always)]
    fn from(val: Cins) -> u8 {
        Cins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinsien {
    #[doc = "Masked."]
    CinsienA = 0x0,
    #[doc = "Enabled."]
    CinsienB = 0x01,
}
impl Cinsien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinsien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinsien {
    #[inline(always)]
    fn from(val: u8) -> Cinsien {
        Cinsien::from_bits(val)
    }
}
impl From<Cinsien> for u8 {
    #[inline(always)]
    fn from(val: Cinsien) -> u8 {
        Cinsien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinssen {
    #[doc = "Masked."]
    CinsenA = 0x0,
    #[doc = "Enabled."]
    CinsenB = 0x01,
}
impl Cinssen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinssen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinssen {
    #[inline(always)]
    fn from(val: u8) -> Cinssen {
        Cinssen::from_bits(val)
    }
}
impl From<Cinssen> for u8 {
    #[inline(always)]
    fn from(val: Cinssen) -> u8 {
        Cinssen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cinst {
    #[doc = "Power on reset or no card."]
    CinstA = 0x0,
    #[doc = "Card inserted."]
    CinstB = 0x01,
}
impl Cinst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cinst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cinst {
    #[inline(always)]
    fn from(val: u8) -> Cinst {
        Cinst::from_bits(val)
    }
}
impl From<Cinst> for u8 {
    #[inline(always)]
    fn from(val: Cinst) -> u8 {
        Cinst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cint {
    #[doc = "No card interrupt."]
    CintA = 0x0,
    #[doc = "Generate card interrupt."]
    CintB = 0x01,
}
impl Cint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cint {
    #[inline(always)]
    fn from(val: u8) -> Cint {
        Cint::from_bits(val)
    }
}
impl From<Cint> for u8 {
    #[inline(always)]
    fn from(val: Cint) -> u8 {
        Cint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cintien {
    #[doc = "Masked."]
    CintienB = 0x0,
    #[doc = "Enabled."]
    CintienA = 0x01,
}
impl Cintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintien {
    #[inline(always)]
    fn from(val: u8) -> Cintien {
        Cintien::from_bits(val)
    }
}
impl From<Cintien> for u8 {
    #[inline(always)]
    fn from(val: Cintien) -> u8 {
        Cintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cintsen {
    #[doc = "Masked."]
    CintsenA = 0x0,
    #[doc = "Enabled."]
    CintsenB = 0x01,
}
impl Cintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cintsen {
    #[inline(always)]
    fn from(val: u8) -> Cintsen {
        Cintsen::from_bits(val)
    }
}
impl From<Cintsen> for u8 {
    #[inline(always)]
    fn from(val: Cintsen) -> u8 {
        Cintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdByteEn {
    #[doc = "Disable. MIX_CTRL\\[7:0\\] is read/write and CMD_XFR_TYP\\[7:0\\] is read-only."]
    CmdByteEnA = 0x0,
    #[doc = "Enable. MIX_CTRL\\[7:0\\] is read-only and CMD_XFR_TYP\\[7:0\\] is read/write."]
    CmdByteEnB = 0x01,
}
impl CmdByteEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdByteEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdByteEn {
    #[inline(always)]
    fn from(val: u8) -> CmdByteEn {
        CmdByteEn::from_bits(val)
    }
}
impl From<CmdByteEn> for u8 {
    #[inline(always)]
    fn from(val: CmdByteEn) -> u8 {
        CmdByteEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypAc12en {
    #[doc = "Disable."]
    CmdXfrTyp2B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp2A = 0x01,
}
impl CmdXfrTypAc12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypAc12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypAc12en {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypAc12en {
        CmdXfrTypAc12en::from_bits(val)
    }
}
impl From<CmdXfrTypAc12en> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypAc12en) -> u8 {
        CmdXfrTypAc12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypBcen {
    #[doc = "Disable."]
    CmdXfrTyp1B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp1A = 0x01,
}
impl CmdXfrTypBcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypBcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypBcen {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypBcen {
        CmdXfrTypBcen::from_bits(val)
    }
}
impl From<CmdXfrTypBcen> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypBcen) -> u8 {
        CmdXfrTypBcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypDmaen {
    #[doc = "Disable."]
    CmdXfrTyp0b = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp0a = 0x01,
}
impl CmdXfrTypDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypDmaen {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypDmaen {
        CmdXfrTypDmaen::from_bits(val)
    }
}
impl From<CmdXfrTypDmaen> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypDmaen) -> u8 {
        CmdXfrTypDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypDtdsel {
    #[doc = "Disable."]
    CmdXfrTyp4B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp4A = 0x01,
}
impl CmdXfrTypDtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypDtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypDtdsel {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypDtdsel {
        CmdXfrTypDtdsel::from_bits(val)
    }
}
impl From<CmdXfrTypDtdsel> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypDtdsel) -> u8 {
        CmdXfrTypDtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmdXfrTypMsbsel {
    #[doc = "Disable."]
    CmdXfrTyp5B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp5A = 0x01,
}
impl CmdXfrTypMsbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmdXfrTypMsbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmdXfrTypMsbsel {
    #[inline(always)]
    fn from(val: u8) -> CmdXfrTypMsbsel {
        CmdXfrTypMsbsel::from_bits(val)
    }
}
impl From<CmdXfrTypMsbsel> for u8 {
    #[inline(always)]
    fn from(val: CmdXfrTypMsbsel) -> u8 {
        CmdXfrTypMsbsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmdtyp {
    #[doc = "Normal other commands."]
    CmdtypD = 0x0,
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    CmdtypC = 0x01,
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    CmdtypB = 0x02,
    #[doc = "Abort CMD12, CMD52 for writing I/O Abort in CCCR."]
    CmdtypA = 0x03,
}
impl Cmdtyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmdtyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmdtyp {
    #[inline(always)]
    fn from(val: u8) -> Cmdtyp {
        Cmdtyp::from_bits(val)
    }
}
impl From<Cmdtyp> for u8 {
    #[inline(always)]
    fn from(val: Cmdtyp) -> u8 {
        Cmdtyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cnibac12e {
    #[doc = "No error."]
    Cnibac12eB = 0x0,
    #[doc = "Not issued."]
    Cnibac12eA = 0x01,
}
impl Cnibac12e {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cnibac12e {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cnibac12e {
    #[inline(always)]
    fn from(val: u8) -> Cnibac12e {
        Cnibac12e::from_bits(val)
    }
}
impl From<Cnibac12e> for u8 {
    #[inline(always)]
    fn from(val: Cnibac12e) -> u8 {
        Cnibac12e::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcChkDis {
    #[doc = "Check CRC16 for every read data packet and check CRC fields for every write data packet."]
    CrcChkDisA = 0x0,
    #[doc = "Ignore CRC16 check for every read data packet and ignore CRC fields check for every write data packet."]
    CrcChkDisB = 0x01,
}
impl CrcChkDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcChkDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcChkDis {
    #[inline(always)]
    fn from(val: u8) -> CrcChkDis {
        CrcChkDis::from_bits(val)
    }
}
impl From<CrcChkDis> for u8 {
    #[inline(always)]
    fn from(val: CrcChkDis) -> u8 {
        CrcChkDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Creq {
    #[doc = "No effect."]
    CreqB = 0x0,
    #[doc = "Restart."]
    CreqA = 0x01,
}
impl Creq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Creq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Creq {
    #[inline(always)]
    fn from(val: u8) -> Creq {
        Creq::from_bits(val)
    }
}
impl From<Creq> for u8 {
    #[inline(always)]
    fn from(val: Creq) -> u8 {
        Creq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crm {
    #[doc = "Card state unstable or inserted."]
    CrmA = 0x0,
    #[doc = "Card removed."]
    CrmB = 0x01,
}
impl Crm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crm {
    #[inline(always)]
    fn from(val: u8) -> Crm {
        Crm::from_bits(val)
    }
}
impl From<Crm> for u8 {
    #[inline(always)]
    fn from(val: Crm) -> u8 {
        Crm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crmien {
    #[doc = "Masked."]
    CrmienA = 0x0,
    #[doc = "Enabled."]
    CrmienB = 0x01,
}
impl Crmien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmien {
    #[inline(always)]
    fn from(val: u8) -> Crmien {
        Crmien::from_bits(val)
    }
}
impl From<Crmien> for u8 {
    #[inline(always)]
    fn from(val: Crmien) -> u8 {
        Crmien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crmsen {
    #[doc = "Masked."]
    CrmsenA = 0x0,
    #[doc = "Enabled."]
    CrmsenB = 0x01,
}
impl Crmsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crmsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crmsen {
    #[inline(always)]
    fn from(val: u8) -> Crmsen {
        Crmsen::from_bits(val)
    }
}
impl From<Crmsen> for u8 {
    #[inline(always)]
    fn from(val: Crmsen) -> u8 {
        Crmsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoe {
    #[doc = "No error."]
    CtoeA = 0x0,
    #[doc = "Time out."]
    CtoeB = 0x01,
}
impl Ctoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoe {
    #[inline(always)]
    fn from(val: u8) -> Ctoe {
        Ctoe::from_bits(val)
    }
}
impl From<Ctoe> for u8 {
    #[inline(always)]
    fn from(val: Ctoe) -> u8 {
        Ctoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoeien {
    #[doc = "Masked."]
    CtoeienB = 0x0,
    #[doc = "Enabled."]
    CtoeienA = 0x01,
}
impl Ctoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoeien {
    #[inline(always)]
    fn from(val: u8) -> Ctoeien {
        Ctoeien::from_bits(val)
    }
}
impl From<Ctoeien> for u8 {
    #[inline(always)]
    fn from(val: Ctoeien) -> u8 {
        Ctoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctoesen {
    #[doc = "Masked."]
    CtosenA = 0x0,
    #[doc = "Enabled."]
    CtosenB = 0x01,
}
impl Ctoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ctoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ctoesen {
    #[inline(always)]
    fn from(val: u8) -> Ctoesen {
        Ctoesen::from_bits(val)
    }
}
impl From<Ctoesen> for u8 {
    #[inline(always)]
    fn from(val: Ctoesen) -> u8 {
        Ctoesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum D3cd {
    #[doc = "DATA3 does not monitor card insertion."]
    D3cdB = 0x0,
    #[doc = "DATA3 as card detection pin."]
    D3cdA = 0x01,
}
impl D3cd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> D3cd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for D3cd {
    #[inline(always)]
    fn from(val: u8) -> D3cd {
        D3cd::from_bits(val)
    }
}
impl From<D3cd> for u8 {
    #[inline(always)]
    fn from(val: D3cd) -> u8 {
        D3cd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dce {
    #[doc = "No error."]
    DceA = 0x0,
    #[doc = "Error."]
    DceB = 0x01,
}
impl Dce {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dce {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dce {
    #[inline(always)]
    fn from(val: u8) -> Dce {
        Dce::from_bits(val)
    }
}
impl From<Dce> for u8 {
    #[inline(always)]
    fn from(val: Dce) -> u8 {
        Dce::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dceien {
    #[doc = "Masked."]
    DceienB = 0x0,
    #[doc = "Enabled."]
    DceienA = 0x01,
}
impl Dceien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dceien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dceien {
    #[inline(always)]
    fn from(val: u8) -> Dceien {
        Dceien::from_bits(val)
    }
}
impl From<Dceien> for u8 {
    #[inline(always)]
    fn from(val: Dceien) -> u8 {
        Dceien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dcesen {
    #[doc = "Masked."]
    DcesenA = 0x0,
    #[doc = "Enabled."]
    DcesenB = 0x01,
}
impl Dcesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dcesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dcesen {
    #[inline(always)]
    fn from(val: u8) -> Dcesen {
        Dcesen::from_bits(val)
    }
}
impl From<Dcesen> for u8 {
    #[inline(always)]
    fn from(val: Dcesen) -> u8 {
        Dcesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DdrEn {
    #[doc = "Disable."]
    CmdXfrTyp3B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp3A = 0x01,
}
impl DdrEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DdrEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DdrEn {
    #[inline(always)]
    fn from(val: u8) -> DdrEn {
        DdrEn::from_bits(val)
    }
}
impl From<DdrEn> for u8 {
    #[inline(always)]
    fn from(val: DdrEn) -> u8 {
        DdrEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debe {
    #[doc = "No error."]
    DebeA = 0x0,
    #[doc = "Error."]
    DebeB = 0x01,
}
impl Debe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debe {
    #[inline(always)]
    fn from(val: u8) -> Debe {
        Debe::from_bits(val)
    }
}
impl From<Debe> for u8 {
    #[inline(always)]
    fn from(val: Debe) -> u8 {
        Debe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debeien {
    #[doc = "Masked."]
    DebeienB = 0x0,
    #[doc = "Enabled."]
    DebeienA = 0x01,
}
impl Debeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debeien {
    #[inline(always)]
    fn from(val: u8) -> Debeien {
        Debeien::from_bits(val)
    }
}
impl From<Debeien> for u8 {
    #[inline(always)]
    fn from(val: Debeien) -> u8 {
        Debeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Debesen {
    #[doc = "Masked."]
    DbesenA = 0x0,
    #[doc = "Enabled."]
    DbesenB = 0x01,
}
impl Debesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Debesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Debesen {
    #[inline(always)]
    fn from(val: u8) -> Debesen {
        Debesen::from_bits(val)
    }
}
impl From<Debesen> for u8 {
    #[inline(always)]
    fn from(val: Debesen) -> u8 {
        Debesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dint {
    #[doc = "No DMA interrupt."]
    DintB = 0x0,
    #[doc = "DMA interrupt is generated."]
    DintA = 0x01,
}
impl Dint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dint {
    #[inline(always)]
    fn from(val: u8) -> Dint {
        Dint::from_bits(val)
    }
}
impl From<Dint> for u8 {
    #[inline(always)]
    fn from(val: Dint) -> u8 {
        Dint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dintien {
    #[doc = "Masked."]
    DintienB = 0x0,
    #[doc = "Enabled."]
    DintienA = 0x01,
}
impl Dintien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintien {
    #[inline(always)]
    fn from(val: u8) -> Dintien {
        Dintien::from_bits(val)
    }
}
impl From<Dintien> for u8 {
    #[inline(always)]
    fn from(val: Dintien) -> u8 {
        Dintien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dintsen {
    #[doc = "Masked."]
    DintsenA = 0x0,
    #[doc = "Enabled."]
    DintsenB = 0x01,
}
impl Dintsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dintsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dintsen {
    #[inline(always)]
    fn from(val: u8) -> Dintsen {
        Dintsen::from_bits(val)
    }
}
impl From<Dintsen> for u8 {
    #[inline(always)]
    fn from(val: Dintsen) -> u8 {
        Dintsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableTimeOut {
    #[doc = "Enable time out."]
    DisableTimeoutA = 0x0,
    #[doc = "Disable time out."]
    DisableTimeoutB = 0x01,
}
impl DisableTimeOut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableTimeOut {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableTimeOut {
    #[inline(always)]
    fn from(val: u8) -> DisableTimeOut {
        DisableTimeOut::from_bits(val)
    }
}
impl From<DisableTimeOut> for u8 {
    #[inline(always)]
    fn from(val: DisableTimeOut) -> u8 {
        DisableTimeOut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dla {
    #[doc = "DATA line inactive."]
    DlaA = 0x0,
    #[doc = "DATA line active."]
    DlaB = 0x01,
}
impl Dla {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dla {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dla {
    #[inline(always)]
    fn from(val: u8) -> Dla {
        Dla::from_bits(val)
    }
}
impl From<Dla> for u8 {
    #[inline(always)]
    fn from(val: Dla) -> u8 {
        Dla::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Dlsl(u8);
impl Dlsl {
    #[doc = "Data 0 line signal level."]
    pub const Data0: Self = Self(0x01);
    #[doc = "Data 1 line signal level."]
    pub const Data1: Self = Self(0x02);
    #[doc = "Data 2 line signal level."]
    pub const Data2: Self = Self(0x04);
    #[doc = "Data 3 line signal level."]
    pub const Data3: Self = Self(0x08);
    #[doc = "Data 4 line signal level."]
    pub const Data4: Self = Self(0x10);
    #[doc = "Data 5 line signal level."]
    pub const Data5: Self = Self(0x20);
    #[doc = "Data 6 line signal level."]
    pub const Data6: Self = Self(0x40);
    #[doc = "Data 7 line signal level."]
    pub const Data7: Self = Self(0x80);
}
impl Dlsl {
    pub const fn from_bits(val: u8) -> Dlsl {
        Self(val & 0xff)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Dlsl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("Data0"),
            0x02 => f.write_str("Data1"),
            0x04 => f.write_str("Data2"),
            0x08 => f.write_str("Data3"),
            0x10 => f.write_str("Data4"),
            0x20 => f.write_str("Data5"),
            0x40 => f.write_str("Data6"),
            0x80 => f.write_str("Data7"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlsl {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "Data0"),
            0x02 => defmt::write!(f, "Data1"),
            0x04 => defmt::write!(f, "Data2"),
            0x08 => defmt::write!(f, "Data3"),
            0x10 => defmt::write!(f, "Data4"),
            0x20 => defmt::write!(f, "Data5"),
            0x40 => defmt::write!(f, "Data6"),
            0x80 => defmt::write!(f, "Data7"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Dlsl {
    #[inline(always)]
    fn from(val: u8) -> Dlsl {
        Dlsl::from_bits(val)
    }
}
impl From<Dlsl> for u8 {
    #[inline(always)]
    fn from(val: Dlsl) -> u8 {
        Dlsl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmae {
    #[doc = "No error."]
    DmaeA = 0x0,
    #[doc = "Error."]
    DmaeB = 0x01,
}
impl Dmae {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmae {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmae {
    #[inline(always)]
    fn from(val: u8) -> Dmae {
        Dmae::from_bits(val)
    }
}
impl From<Dmae> for u8 {
    #[inline(always)]
    fn from(val: Dmae) -> u8 {
        Dmae::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaeien {
    #[doc = "Masked."]
    DmaeienB = 0x0,
    #[doc = "Enable."]
    DmaeienA = 0x01,
}
impl Dmaeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaeien {
    #[inline(always)]
    fn from(val: u8) -> Dmaeien {
        Dmaeien::from_bits(val)
    }
}
impl From<Dmaeien> for u8 {
    #[inline(always)]
    fn from(val: Dmaeien) -> u8 {
        Dmaeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmaesen {
    #[doc = "Masked."]
    DmasenB = 0x0,
    #[doc = "Enabled."]
    DmasenA = 0x01,
}
impl Dmaesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmaesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmaesen {
    #[inline(always)]
    fn from(val: u8) -> Dmaesen {
        Dmaesen::from_bits(val)
    }
}
impl From<Dmaesen> for u8 {
    #[inline(always)]
    fn from(val: Dmaesen) -> u8 {
        Dmaesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmas {
    #[doc = "DMA not supported."]
    DmasB = 0x0,
    #[doc = "DMA supported."]
    DmasA = 0x01,
}
impl Dmas {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmas {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmas {
    #[inline(always)]
    fn from(val: u8) -> Dmas {
        Dmas::from_bits(val)
    }
}
impl From<Dmas> for u8 {
    #[inline(always)]
    fn from(val: Dmas) -> u8 {
        Dmas::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dmasel {
    #[doc = "No DMA or simple DMA is selected."]
    DmaselA = 0x0,
    #[doc = "ADMA1 is selected."]
    DmaselB = 0x01,
    #[doc = "ADMA2 is selected."]
    DmaselC = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dmasel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dmasel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dmasel {
    #[inline(always)]
    fn from(val: u8) -> Dmasel {
        Dmasel::from_bits(val)
    }
}
impl From<Dmasel> for u8 {
    #[inline(always)]
    fn from(val: Dmasel) -> u8 {
        Dmasel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dpsel {
    #[doc = "No data present."]
    DpselB = 0x0,
    #[doc = "Data present."]
    DpselA = 0x01,
}
impl Dpsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dpsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dpsel {
    #[inline(always)]
    fn from(val: u8) -> Dpsel {
        Dpsel::from_bits(val)
    }
}
impl From<Dpsel> for u8 {
    #[inline(always)]
    fn from(val: Dpsel) -> u8 {
        Dpsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtocv {
    #[doc = "SDCLK x 2 32."]
    DtocvX = 0x0,
    #[doc = "SDCLK x 2 33."]
    DtocvW = 0x01,
    #[doc = "SDCLK x 2 18."]
    DtocvV = 0x02,
    #[doc = "SDCLK x 2 19."]
    DtocvU = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    #[doc = "SDCLK x 2 29, recommend to use for supported speed modes except SDR104 mode."]
    DtocvT = 0x0d,
    #[doc = "SDCLK x 2 30, recommend to use for SDR104 mode."]
    DtocvS = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Dtocv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtocv {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtocv {
    #[inline(always)]
    fn from(val: u8) -> Dtocv {
        Dtocv::from_bits(val)
    }
}
impl From<Dtocv> for u8 {
    #[inline(always)]
    fn from(val: Dtocv) -> u8 {
        Dtocv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DtocvAck {
    #[doc = "SDCLK x 2^14."]
    DtocvAckA = 0x0,
    #[doc = "SDCLK x 2^15."]
    DtocvAckB = 0x01,
    #[doc = "SDCLK x 2^16."]
    DtocvAckC = 0x02,
    #[doc = "SDCLK x 2^17."]
    DtocvAckD = 0x03,
    #[doc = "SDCLK x 2^18."]
    DtocvAckE = 0x04,
    #[doc = "SDCLK x 2^19."]
    DtocvAckF = 0x05,
    #[doc = "SDCLK x 2^20."]
    DtocvAckG = 0x06,
    #[doc = "SDCLK x 2^21."]
    DtocvAckH = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "SDCLK x 2^28."]
    DtocvAckI = 0x0e,
    #[doc = "SDCLK x 2^29."]
    DtocvAckJ = 0x0f,
}
impl DtocvAck {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DtocvAck {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DtocvAck {
    #[inline(always)]
    fn from(val: u8) -> DtocvAck {
        DtocvAck::from_bits(val)
    }
}
impl From<DtocvAck> for u8 {
    #[inline(always)]
    fn from(val: DtocvAck) -> u8 {
        DtocvAck::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtoe {
    #[doc = "No error."]
    DtoeA = 0x0,
    #[doc = "Time out."]
    DtoeB = 0x01,
}
impl Dtoe {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoe {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoe {
    #[inline(always)]
    fn from(val: u8) -> Dtoe {
        Dtoe::from_bits(val)
    }
}
impl From<Dtoe> for u8 {
    #[inline(always)]
    fn from(val: Dtoe) -> u8 {
        Dtoe::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtoeien {
    #[doc = "Masked."]
    DtoeienB = 0x0,
    #[doc = "Enabled."]
    DtoeienA = 0x01,
}
impl Dtoeien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoeien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoeien {
    #[inline(always)]
    fn from(val: u8) -> Dtoeien {
        Dtoeien::from_bits(val)
    }
}
impl From<Dtoeien> for u8 {
    #[inline(always)]
    fn from(val: Dtoeien) -> u8 {
        Dtoeien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtoesen {
    #[doc = "Masked."]
    DtoesenA = 0x0,
    #[doc = "Enabled."]
    DtoesenB = 0x01,
}
impl Dtoesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtoesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtoesen {
    #[inline(always)]
    fn from(val: u8) -> Dtoesen {
        Dtoesen::from_bits(val)
    }
}
impl From<Dtoesen> for u8 {
    #[inline(always)]
    fn from(val: Dtoesen) -> u8 {
        Dtoesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dtw {
    #[doc = "1-bit mode."]
    DtwC = 0x0,
    #[doc = "4-bit mode."]
    DtwB = 0x01,
    #[doc = "8-bit mode."]
    DtwA = 0x02,
    _RESERVED_3 = 0x03,
}
impl Dtw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dtw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dtw {
    #[inline(always)]
    fn from(val: u8) -> Dtw {
        Dtw::from_bits(val)
    }
}
impl From<Dtw> for u8 {
    #[inline(always)]
    fn from(val: Dtw) -> u8 {
        Dtw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dvs {
    #[doc = "Divide-by-1."]
    DvsA = 0x0,
    #[doc = "Divide-by-2."]
    DvsB = 0x01,
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
    #[doc = "Divide-by-15."]
    DvsC = 0x0e,
    #[doc = "Divide-by-16."]
    DvsD = 0x0f,
}
impl Dvs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dvs {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dvs {
    #[inline(always)]
    fn from(val: u8) -> Dvs {
        Dvs::from_bits(val)
    }
}
impl From<Dvs> for u8 {
    #[inline(always)]
    fn from(val: Dvs) -> u8 {
        Dvs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emode {
    #[doc = "Big endian mode."]
    EmodeA = 0x0,
    #[doc = "Half word big endian mode."]
    EmodeB = 0x01,
    #[doc = "Little endian mode."]
    EmodeC = 0x02,
    _RESERVED_3 = 0x03,
}
impl Emode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emode {
    #[inline(always)]
    fn from(val: u8) -> Emode {
        Emode::from_bits(val)
    }
}
impl From<Emode> for u8 {
    #[inline(always)]
    fn from(val: Emode) -> u8 {
        Emode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En32kClk {
    #[doc = "Use the peripheral clock (ipg_clk) for card detection."]
    CdClkSelA = 0x0,
    #[doc = "Use the low power clock (ipg_clk_lp) for card detection."]
    CdClkSelB = 0x01,
}
impl En32kClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En32kClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En32kClk {
    #[inline(always)]
    fn from(val: u8) -> En32kClk {
        En32kClk::from_bits(val)
    }
}
impl From<En32kClk> for u8 {
    #[inline(always)]
    fn from(val: En32kClk) -> u8 {
        En32kClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExeTune {
    #[doc = "Not tuned or tuning completed."]
    ExeTuneD = 0x0,
    #[doc = "Execute tuning."]
    ExeTuneC = 0x01,
}
impl ExeTune {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExeTune {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExeTune {
    #[inline(always)]
    fn from(val: u8) -> ExeTune {
        ExeTune::from_bits(val)
    }
}
impl From<ExeTune> for u8 {
    #[inline(always)]
    fn from(val: ExeTune) -> u8 {
        ExeTune::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ExecuteTuning {
    #[doc = "Tuning procedure is aborted."]
    ExTunB = 0x0,
    #[doc = "Start tuning procedure."]
    ExTunA = 0x01,
}
impl ExecuteTuning {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ExecuteTuning {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ExecuteTuning {
    #[inline(always)]
    fn from(val: u8) -> ExecuteTuning {
        ExecuteTuning::from_bits(val)
    }
}
impl From<ExecuteTuning> for u8 {
    #[inline(always)]
    fn from(val: ExecuteTuning) -> u8 {
        ExecuteTuning::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FbclkSel {
    #[doc = "Feedback clock comes from the loopback CLK."]
    FbclkB = 0x0,
    #[doc = "Feedback clock comes from the ipp_card_clk_out."]
    FbclkA = 0x01,
}
impl FbclkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FbclkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FbclkSel {
    #[inline(always)]
    fn from(val: u8) -> FbclkSel {
        FbclkSel::from_bits(val)
    }
}
impl From<FbclkSel> for u8 {
    #[inline(always)]
    fn from(val: FbclkSel) -> u8 {
        FbclkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcSdclkOn {
    #[doc = "CLK active or inactive is fully controlled by the hardware."]
    FrcSdclkOnA = 0x0,
    #[doc = "Force CLK active."]
    FrcSdclkOnB = 0x01,
}
impl FrcSdclkOn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcSdclkOn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcSdclkOn {
    #[inline(always)]
    fn from(val: u8) -> FrcSdclkOn {
        FrcSdclkOn::from_bits(val)
    }
}
impl From<FrcSdclkOn> for u8 {
    #[inline(always)]
    fn from(val: FrcSdclkOn) -> u8 {
        FrcSdclkOn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hss {
    #[doc = "High speed not supported."]
    HssB = 0x0,
    #[doc = "High speed supported."]
    HssA = 0x01,
}
impl Hss {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hss {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hss {
    #[inline(always)]
    fn from(val: u8) -> Hss {
        Hss::from_bits(val)
    }
}
impl From<Hss> for u8 {
    #[inline(always)]
    fn from(val: Hss) -> u8 {
        Hss::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iabg {
    #[doc = "Disables interrupt at block gap."]
    IabgB = 0x0,
    #[doc = "Enables interrupt at block gap."]
    IabgA = 0x01,
}
impl Iabg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iabg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iabg {
    #[inline(always)]
    fn from(val: u8) -> Iabg {
        Iabg::from_bits(val)
    }
}
impl From<Iabg> for u8 {
    #[inline(always)]
    fn from(val: Iabg) -> u8 {
        Iabg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mbl {
    #[doc = "512 bytes."]
    MblA = 0x0,
    #[doc = "1024 bytes."]
    MblB = 0x01,
    #[doc = "2048 bytes."]
    MblC = 0x02,
    #[doc = "4096 bytes."]
    MblD = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Mbl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mbl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mbl {
    #[inline(always)]
    fn from(val: u8) -> Mbl {
        Mbl::from_bits(val)
    }
}
impl From<Mbl> for u8 {
    #[inline(always)]
    fn from(val: Mbl) -> u8 {
        Mbl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlAc12en {
    #[doc = "Disable."]
    Ac12enB = 0x0,
    #[doc = "Enable."]
    Ac12enA = 0x01,
}
impl MixCtrlAc12en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlAc12en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlAc12en {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlAc12en {
        MixCtrlAc12en::from_bits(val)
    }
}
impl From<MixCtrlAc12en> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlAc12en) -> u8 {
        MixCtrlAc12en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlBcen {
    #[doc = "Disable."]
    BcenB = 0x0,
    #[doc = "Enable."]
    BcenA = 0x01,
}
impl MixCtrlBcen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlBcen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlBcen {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlBcen {
        MixCtrlBcen::from_bits(val)
    }
}
impl From<MixCtrlBcen> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlBcen) -> u8 {
        MixCtrlBcen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlDmaen {
    #[doc = "Disable."]
    DmainB = 0x0,
    #[doc = "Enable."]
    DmainA = 0x01,
}
impl MixCtrlDmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlDmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlDmaen {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlDmaen {
        MixCtrlDmaen::from_bits(val)
    }
}
impl From<MixCtrlDmaen> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlDmaen) -> u8 {
        MixCtrlDmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlDtdsel {
    #[doc = "Write (Host to card)."]
    DtdselB = 0x0,
    #[doc = "Read (Card to host)."]
    DtdselA = 0x01,
}
impl MixCtrlDtdsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlDtdsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlDtdsel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlDtdsel {
        MixCtrlDtdsel::from_bits(val)
    }
}
impl From<MixCtrlDtdsel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlDtdsel) -> u8 {
        MixCtrlDtdsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlMsbsel {
    #[doc = "Single block."]
    MsbselB = 0x0,
    #[doc = "Multiple blocks."]
    MsbselA = 0x01,
}
impl MixCtrlMsbsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlMsbsel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlMsbsel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlMsbsel {
        MixCtrlMsbsel::from_bits(val)
    }
}
impl From<MixCtrlMsbsel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlMsbsel) -> u8 {
        MixCtrlMsbsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MixCtrlSmpClkSel {
    #[doc = "Fixed clock is used to sample data / cmd."]
    SmpselB = 0x0,
    #[doc = "Tuned clock is used to sample data / cmd."]
    SmpselA = 0x01,
}
impl MixCtrlSmpClkSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MixCtrlSmpClkSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MixCtrlSmpClkSel {
    #[inline(always)]
    fn from(val: u8) -> MixCtrlSmpClkSel {
        MixCtrlSmpClkSel::from_bits(val)
    }
}
impl From<MixCtrlSmpClkSel> for u8 {
    #[inline(always)]
    fn from(val: MixCtrlSmpClkSel) -> u8 {
        MixCtrlSmpClkSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NibblePos {
    #[doc = "Disable."]
    CmdXfrTyp6B = 0x0,
    #[doc = "Enable."]
    CmdXfrTyp6A = 0x01,
}
impl NibblePos {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NibblePos {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NibblePos {
    #[inline(always)]
    fn from(val: u8) -> NibblePos {
        NibblePos::from_bits(val)
    }
}
impl From<NibblePos> for u8 {
    #[inline(always)]
    fn from(val: NibblePos) -> u8 {
        NibblePos::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NonExactBlkRd {
    #[doc = "The block read is exact block read. Host driver does not need to issue abort command to terminate this multi-block read."]
    ExactB = 0x0,
    #[doc = "The block read is non-exact block read. Host driver needs to issue abort command to terminate this multi-block read."]
    ExactA = 0x01,
}
impl NonExactBlkRd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NonExactBlkRd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NonExactBlkRd {
    #[inline(always)]
    fn from(val: u8) -> NonExactBlkRd {
        NonExactBlkRd::from_bits(val)
    }
}
impl From<NonExactBlkRd> for u8 {
    #[inline(always)]
    fn from(val: NonExactBlkRd) -> u8 {
        NonExactBlkRd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsptyp {
    #[doc = "No response."]
    RsptypA = 0x0,
    #[doc = "Response length 136."]
    RsptypB = 0x01,
    #[doc = "Response length 48."]
    RsptypC = 0x02,
    #[doc = "Response length 48, check busy after response."]
    RsptypD = 0x03,
}
impl Rsptyp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsptyp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsptyp {
    #[inline(always)]
    fn from(val: u8) -> Rsptyp {
        Rsptyp::from_bits(val)
    }
}
impl From<Rsptyp> for u8 {
    #[inline(always)]
    fn from(val: Rsptyp) -> u8 {
        Rsptyp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rsta {
    #[doc = "No reset."]
    RstaB = 0x0,
    #[doc = "Reset."]
    RstaA = 0x01,
}
impl Rsta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rsta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rsta {
    #[inline(always)]
    fn from(val: u8) -> Rsta {
        Rsta::from_bits(val)
    }
}
impl From<Rsta> for u8 {
    #[inline(always)]
    fn from(val: Rsta) -> u8 {
        Rsta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstc {
    #[doc = "No reset."]
    RstcB = 0x0,
    #[doc = "Reset."]
    RstcA = 0x01,
}
impl Rstc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstc {
    #[inline(always)]
    fn from(val: u8) -> Rstc {
        Rstc::from_bits(val)
    }
}
impl From<Rstc> for u8 {
    #[inline(always)]
    fn from(val: Rstc) -> u8 {
        Rstc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstd {
    #[doc = "No reset."]
    RstdB = 0x0,
    #[doc = "Reset."]
    RstdA = 0x01,
}
impl Rstd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstd {
    #[inline(always)]
    fn from(val: u8) -> Rstd {
        Rstd::from_bits(val)
    }
}
impl From<Rstd> for u8 {
    #[inline(always)]
    fn from(val: Rstd) -> u8 {
        Rstd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rta {
    #[doc = "No valid data."]
    RtaB = 0x0,
    #[doc = "Transferring data."]
    RtaA = 0x01,
}
impl Rta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rta {
    #[inline(always)]
    fn from(val: u8) -> Rta {
        Rta::from_bits(val)
    }
}
impl From<Rta> for u8 {
    #[inline(always)]
    fn from(val: Rta) -> u8 {
        Rta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rte {
    #[doc = "Re-tuning is not required."]
    RteA = 0x0,
    #[doc = "Re-tuning should be performed."]
    RteB = 0x01,
}
impl Rte {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rte {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rte {
    #[inline(always)]
    fn from(val: u8) -> Rte {
        Rte::from_bits(val)
    }
}
impl From<Rte> for u8 {
    #[inline(always)]
    fn from(val: Rte) -> u8 {
        Rte::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rteien {
    #[doc = "Masked."]
    RteienO = 0x0,
    #[doc = "Enabled."]
    RteienN = 0x01,
}
impl Rteien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rteien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rteien {
    #[inline(always)]
    fn from(val: u8) -> Rteien {
        Rteien::from_bits(val)
    }
}
impl From<Rteien> for u8 {
    #[inline(always)]
    fn from(val: Rteien) -> u8 {
        Rteien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtesen {
    #[doc = "Masked."]
    RtesenA = 0x0,
    #[doc = "Enabled."]
    RtesenB = 0x01,
}
impl Rtesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtesen {
    #[inline(always)]
    fn from(val: u8) -> Rtesen {
        Rtesen::from_bits(val)
    }
}
impl From<Rtesen> for u8 {
    #[inline(always)]
    fn from(val: Rtesen) -> u8 {
        Rtesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rtr {
    #[doc = "Fixed or well tuned sampling clock."]
    RtrB = 0x0,
    #[doc = "Sampling clock needs re-tuning."]
    RtrA = 0x01,
}
impl Rtr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rtr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rtr {
    #[inline(always)]
    fn from(val: u8) -> Rtr {
        Rtr::from_bits(val)
    }
}
impl From<Rtr> for u8 {
    #[inline(always)]
    fn from(val: Rtr) -> u8 {
        Rtr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rwctl {
    #[doc = "Disables read wait control and stop SD clock at block gap when SABGREQ field is set."]
    RwctlB = 0x0,
    #[doc = "Enables read wait control and assert read wait without stopping SD clock at block gap when SABGREQ field is set."]
    RwctlA = 0x01,
}
impl Rwctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rwctl {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rwctl {
    #[inline(always)]
    fn from(val: u8) -> Rwctl {
        Rwctl::from_bits(val)
    }
}
impl From<Rwctl> for u8 {
    #[inline(always)]
    fn from(val: Rwctl) -> u8 {
        Rwctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sabgreq {
    #[doc = "Transfer."]
    SabgreqB = 0x0,
    #[doc = "Stop."]
    SabgreqA = 0x01,
}
impl Sabgreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sabgreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sabgreq {
    #[inline(always)]
    fn from(val: u8) -> Sabgreq {
        Sabgreq::from_bits(val)
    }
}
impl From<Sabgreq> for u8 {
    #[inline(always)]
    fn from(val: Sabgreq) -> u8 {
        Sabgreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdstb {
    #[doc = "Clock is changing frequency and not stable."]
    SdstbB = 0x0,
    #[doc = "Clock is stable."]
    SdstbA = 0x01,
}
impl Sdstb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdstb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdstb {
    #[inline(always)]
    fn from(val: u8) -> Sdstb {
        Sdstb::from_bits(val)
    }
}
impl From<Sdstb> for u8 {
    #[inline(always)]
    fn from(val: Sdstb) -> u8 {
        Sdstb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Srs {
    #[doc = "Not supported."]
    SrsB = 0x0,
    #[doc = "Supported."]
    SrsA = 0x01,
}
impl Srs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Srs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Srs {
    #[inline(always)]
    fn from(val: u8) -> Srs {
        Srs::from_bits(val)
    }
}
impl From<Srs> for u8 {
    #[inline(always)]
    fn from(val: Srs) -> u8 {
        Srs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tc {
    #[doc = "Transfer does not complete."]
    TcB = 0x0,
    #[doc = "Transfer complete."]
    TcA = 0x01,
}
impl Tc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tc {
    #[inline(always)]
    fn from(val: u8) -> Tc {
        Tc::from_bits(val)
    }
}
impl From<Tc> for u8 {
    #[inline(always)]
    fn from(val: Tc) -> u8 {
        Tc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcien {
    #[doc = "Masked."]
    TcienA = 0x0,
    #[doc = "Enabled."]
    TcienB = 0x01,
}
impl Tcien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcien {
    #[inline(always)]
    fn from(val: u8) -> Tcien {
        Tcien::from_bits(val)
    }
}
impl From<Tcien> for u8 {
    #[inline(always)]
    fn from(val: Tcien) -> u8 {
        Tcien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tcsen {
    #[doc = "Masked."]
    TcsenB = 0x0,
    #[doc = "Enabled."]
    TcsenA = 0x01,
}
impl Tcsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tcsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tcsen {
    #[inline(always)]
    fn from(val: u8) -> Tcsen {
        Tcsen::from_bits(val)
    }
}
impl From<Tcsen> for u8 {
    #[inline(always)]
    fn from(val: Tcsen) -> u8 {
        Tcsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tneien {
    #[doc = "Masked."]
    TneienB = 0x0,
    #[doc = "Enabled."]
    TneienA = 0x01,
}
impl Tneien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tneien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tneien {
    #[inline(always)]
    fn from(val: u8) -> Tneien {
        Tneien::from_bits(val)
    }
}
impl From<Tneien> for u8 {
    #[inline(always)]
    fn from(val: Tneien) -> u8 {
        Tneien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tnesen {
    #[doc = "Masked."]
    TnesenA = 0x0,
    #[doc = "Enabled."]
    TnesenB = 0x01,
}
impl Tnesen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tnesen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tnesen {
    #[inline(always)]
    fn from(val: u8) -> Tnesen {
        Tnesen::from_bits(val)
    }
}
impl From<Tnesen> for u8 {
    #[inline(always)]
    fn from(val: Tnesen) -> u8 {
        Tnesen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpien {
    #[doc = "Masked."]
    TpienR = 0x0,
    #[doc = "Enabled."]
    TpienQ = 0x01,
}
impl Tpien {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpien {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpien {
    #[inline(always)]
    fn from(val: u8) -> Tpien {
        Tpien::from_bits(val)
    }
}
impl From<Tpien> for u8 {
    #[inline(always)]
    fn from(val: Tpien) -> u8 {
        Tpien::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tpsen {
    #[doc = "Masked."]
    TpsenA = 0x0,
    #[doc = "Enabled."]
    TpsenB = 0x01,
}
impl Tpsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tpsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tpsen {
    #[inline(always)]
    fn from(val: u8) -> Tpsen {
        Tpsen::from_bits(val)
    }
}
impl From<Tpsen> for u8 {
    #[inline(always)]
    fn from(val: Tpsen) -> u8 {
        Tpsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tscd {
    #[doc = "Delay cell select change is not finished."]
    TscdB = 0x0,
    #[doc = "Delay cell select change is finished."]
    TscdA = 0x01,
}
impl Tscd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tscd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tscd {
    #[inline(always)]
    fn from(val: u8) -> Tscd {
        Tscd::from_bits(val)
    }
}
impl From<Tscd> for u8 {
    #[inline(always)]
    fn from(val: Tscd) -> u8 {
        Tscd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TuningBitEn {
    #[doc = "Enable Tuning circuit for DATA\\[3:0\\]."]
    TuningBitEn0 = 0x0,
    #[doc = "Enable Tuning circuit for DATA\\[7:0\\]."]
    TuningBitEn1 = 0x01,
    #[doc = "Enable Tuning circuit for DATA\\[0\\]."]
    TuningBitEn2 = 0x02,
    #[doc = "Invalid."]
    TuningBitEn3 = 0x03,
}
impl TuningBitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TuningBitEn {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TuningBitEn {
    #[inline(always)]
    fn from(val: u8) -> TuningBitEn {
        TuningBitEn::from_bits(val)
    }
}
impl From<TuningBitEn> for u8 {
    #[inline(always)]
    fn from(val: TuningBitEn) -> u8 {
        TuningBitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TuningCmdEn {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    TuningCmdEnA = 0x0,
    #[doc = "Auto tuning circuit checks the CMD line."]
    TuningCmdEnB = 0x01,
}
impl TuningCmdEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TuningCmdEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TuningCmdEn {
    #[inline(always)]
    fn from(val: u8) -> TuningCmdEn {
        TuningCmdEn::from_bits(val)
    }
}
impl From<TuningCmdEn> for u8 {
    #[inline(always)]
    fn from(val: TuningCmdEn) -> u8 {
        TuningCmdEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UseTuningSdr50 {
    #[doc = "SDR50 does not support tuning."]
    UseTuningB = 0x0,
    #[doc = "SDR50 supports tuning."]
    UseTuningA = 0x01,
}
impl UseTuningSdr50 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UseTuningSdr50 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UseTuningSdr50 {
    #[inline(always)]
    fn from(val: u8) -> UseTuningSdr50 {
        UseTuningSdr50::from_bits(val)
    }
}
impl From<UseTuningSdr50> for u8 {
    #[inline(always)]
    fn from(val: UseTuningSdr50) -> u8 {
        UseTuningSdr50::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs18 {
    #[doc = "1.8 V not supported."]
    Vs18B = 0x0,
    #[doc = "1.8 V supported."]
    Vs18A = 0x01,
}
impl Vs18 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs18 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs18 {
    #[inline(always)]
    fn from(val: u8) -> Vs18 {
        Vs18::from_bits(val)
    }
}
impl From<Vs18> for u8 {
    #[inline(always)]
    fn from(val: Vs18) -> u8 {
        Vs18::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs30 {
    #[doc = "3.0 V not supported."]
    Vs30B = 0x0,
    #[doc = "3.0 V supported."]
    Vs30A = 0x01,
}
impl Vs30 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs30 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs30 {
    #[inline(always)]
    fn from(val: u8) -> Vs30 {
        Vs30::from_bits(val)
    }
}
impl From<Vs30> for u8 {
    #[inline(always)]
    fn from(val: Vs30) -> u8 {
        Vs30::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs33 {
    #[doc = "3.3 V not supported."]
    Vs33B = 0x0,
    #[doc = "3.3 V supported."]
    Vs33A = 0x01,
}
impl Vs33 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs33 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs33 {
    #[inline(always)]
    fn from(val: u8) -> Vs33 {
        Vs33::from_bits(val)
    }
}
impl From<Vs33> for u8 {
    #[inline(always)]
    fn from(val: Vs33) -> u8 {
        Vs33::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecins {
    #[doc = "Disable wakeup event enable on SD card insertion."]
    WecinsB = 0x0,
    #[doc = "Enable wakeup event enable on SD card insertion."]
    WecinsA = 0x01,
}
impl Wecins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecins {
    #[inline(always)]
    fn from(val: u8) -> Wecins {
        Wecins::from_bits(val)
    }
}
impl From<Wecins> for u8 {
    #[inline(always)]
    fn from(val: Wecins) -> u8 {
        Wecins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecint {
    #[doc = "Disables wakeup event enable on card interrupt."]
    WecintA = 0x0,
    #[doc = "Enables wakeup event enable on card interrupt."]
    WecintB = 0x01,
}
impl Wecint {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecint {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecint {
    #[inline(always)]
    fn from(val: u8) -> Wecint {
        Wecint::from_bits(val)
    }
}
impl From<Wecint> for u8 {
    #[inline(always)]
    fn from(val: Wecint) -> u8 {
        Wecint::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wecrm {
    #[doc = "Disables wakeup event enable on SD card removal."]
    WecrmB = 0x0,
    #[doc = "Enables wakeup event enable on SD card removal."]
    WecrmA = 0x01,
}
impl Wecrm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wecrm {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wecrm {
    #[inline(always)]
    fn from(val: u8) -> Wecrm {
        Wecrm::from_bits(val)
    }
}
impl From<Wecrm> for u8 {
    #[inline(always)]
    fn from(val: Wecrm) -> u8 {
        Wecrm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wta {
    #[doc = "No valid data."]
    WtaB = 0x0,
    #[doc = "Transferring data."]
    WtaA = 0x01,
}
impl Wta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wta {
    #[inline(always)]
    fn from(val: u8) -> Wta {
        Wta::from_bits(val)
    }
}
impl From<Wta> for u8 {
    #[inline(always)]
    fn from(val: Wta) -> u8 {
        Wta::to_bits(val)
    }
}
