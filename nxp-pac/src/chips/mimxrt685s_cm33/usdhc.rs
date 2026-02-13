#[doc = "uSDHC"]
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
    #[doc = "DMA System Address"]
    #[inline(always)]
    pub const fn ds_addr(self) -> crate::common::Reg<regs::DsAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Block Attributes"]
    #[inline(always)]
    pub const fn blk_att(self) -> crate::common::Reg<regs::BlkAtt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Argument"]
    #[inline(always)]
    pub const fn cmd_arg(self) -> crate::common::Reg<regs::CmdArg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Command Transfer Type"]
    #[inline(always)]
    pub const fn cmd_xfr_typ(self) -> crate::common::Reg<regs::CmdXfrTyp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Command Response0"]
    #[inline(always)]
    pub const fn cmd_rsp0(self) -> crate::common::Reg<regs::CmdRsp0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Command Response1"]
    #[inline(always)]
    pub const fn cmd_rsp1(self) -> crate::common::Reg<regs::CmdRsp1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Command Response2"]
    #[inline(always)]
    pub const fn cmd_rsp2(self) -> crate::common::Reg<regs::CmdRsp2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Command Response3"]
    #[inline(always)]
    pub const fn cmd_rsp3(self) -> crate::common::Reg<regs::CmdRsp3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Data Buffer Access Port"]
    #[inline(always)]
    pub const fn data_buff_acc_port(
        self,
    ) -> crate::common::Reg<regs::DataBuffAccPort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Present State"]
    #[inline(always)]
    pub const fn pres_state(self) -> crate::common::Reg<regs::PresState, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Protocol Control"]
    #[inline(always)]
    pub const fn prot_ctrl(self) -> crate::common::Reg<regs::ProtCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "System Control"]
    #[inline(always)]
    pub const fn sys_ctrl(self) -> crate::common::Reg<regs::SysCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Interrupt Status Enable"]
    #[inline(always)]
    pub const fn int_status_en(self) -> crate::common::Reg<regs::IntStatusEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Interrupt Signal Enable"]
    #[inline(always)]
    pub const fn int_signal_en(self) -> crate::common::Reg<regs::IntSignalEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Auto CMD12 Error Status"]
    #[inline(always)]
    pub const fn autocmd12_err_status(
        self,
    ) -> crate::common::Reg<regs::Autocmd12ErrStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Host Controller Capabilities"]
    #[inline(always)]
    pub const fn host_ctrl_cap(self) -> crate::common::Reg<regs::HostCtrlCap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Watermark Level"]
    #[inline(always)]
    pub const fn wtmk_lvl(self) -> crate::common::Reg<regs::WtmkLvl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Mixer Control"]
    #[inline(always)]
    pub const fn mix_ctrl(self) -> crate::common::Reg<regs::MixCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Force Event"]
    #[inline(always)]
    pub const fn force_event(self) -> crate::common::Reg<regs::ForceEvent, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "ADMA Error Status Register"]
    #[inline(always)]
    pub const fn adma_err_status(
        self,
    ) -> crate::common::Reg<regs::AdmaErrStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "ADMA System Address"]
    #[inline(always)]
    pub const fn adma_sys_addr(self) -> crate::common::Reg<regs::AdmaSysAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DLL (Delay Line) Control"]
    #[inline(always)]
    pub const fn dll_ctrl(self) -> crate::common::Reg<regs::DllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "DLL Status"]
    #[inline(always)]
    pub const fn dll_status(self) -> crate::common::Reg<regs::DllStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "CLK Tuning Control and Status"]
    #[inline(always)]
    pub const fn clk_tune_ctrl_status(
        self,
    ) -> crate::common::Reg<regs::ClkTuneCtrlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Strobe DLL Control"]
    #[inline(always)]
    pub const fn strobe_dll_ctrl(
        self,
    ) -> crate::common::Reg<regs::StrobeDllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Strobe DLL Status"]
    #[inline(always)]
    pub const fn strobe_dll_status(
        self,
    ) -> crate::common::Reg<regs::StrobeDllStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Vendor Specific Register"]
    #[inline(always)]
    pub const fn vend_spec(self) -> crate::common::Reg<regs::VendSpec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "MMC Boot Register"]
    #[inline(always)]
    pub const fn mmc_boot(self) -> crate::common::Reg<regs::MmcBoot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Vendor Specific 2 Register"]
    #[inline(always)]
    pub const fn vend_spec2(self) -> crate::common::Reg<regs::VendSpec2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Tuning Control Register"]
    #[inline(always)]
    pub const fn tuning_ctrl(self) -> crate::common::Reg<regs::TuningCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
}
pub mod regs;
pub mod vals;
