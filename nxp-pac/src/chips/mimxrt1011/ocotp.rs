#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocotp {
    ptr: *mut u8,
}
unsafe impl Send for Ocotp {}
unsafe impl Sync for Ocotp {}
impl Ocotp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl(self) -> crate::common::Reg<regs::HwOcotpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_set(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_clr(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn hw_ocotp_ctrl_tog(
        self,
    ) -> crate::common::Reg<regs::HwOcotpCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "OTP Controller Timing Register"]
    #[inline(always)]
    pub const fn hw_ocotp_timing(
        self,
    ) -> crate::common::Reg<regs::HwOcotpTiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_data(self) -> crate::common::Reg<regs::HwOcotpData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_read_ctrl(
        self,
    ) -> crate::common::Reg<regs::HwOcotpReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "OTP Controller Read Data Register"]
    #[inline(always)]
    pub const fn hw_ocotp_read_fuse_data(
        self,
    ) -> crate::common::Reg<regs::HwOcotpReadFuseData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Sticky bit Register"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_sticky(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwSticky, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs(self) -> crate::common::Reg<regs::HwOcotpScs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_set(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_clr(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn hw_ocotp_scs_tog(
        self,
    ) -> crate::common::Reg<regs::HwOcotpScsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "OTP Controller Version Register"]
    #[inline(always)]
    pub const fn hw_ocotp_version(
        self,
    ) -> crate::common::Reg<regs::HwOcotpVersion, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "OTP Controller Timing Register 2"]
    #[inline(always)]
    pub const fn hw_ocotp_timing2(
        self,
    ) -> crate::common::Reg<regs::HwOcotpTiming2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
    #[inline(always)]
    pub const fn hw_ocotp_lock(self) -> crate::common::Reg<regs::HwOcotpLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg0(self) -> crate::common::Reg<regs::HwOcotpCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg1(self) -> crate::common::Reg<regs::HwOcotpCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg2(self) -> crate::common::Reg<regs::HwOcotpCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg3(self) -> crate::common::Reg<regs::HwOcotpCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg4(self) -> crate::common::Reg<regs::HwOcotpCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg5(self) -> crate::common::Reg<regs::HwOcotpCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_cfg6(self) -> crate::common::Reg<regs::HwOcotpCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem0(self) -> crate::common::Reg<regs::HwOcotpMem0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem1(self) -> crate::common::Reg<regs::HwOcotpMem1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem2(self) -> crate::common::Reg<regs::HwOcotpMem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem3(self) -> crate::common::Reg<regs::HwOcotpMem3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 4 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_mem4(self) -> crate::common::Reg<regs::HwOcotpMem4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 5 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana0(self) -> crate::common::Reg<regs::HwOcotpAna0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 6 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana1(self) -> crate::common::Reg<regs::HwOcotpAna1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "Value of OTP Bank 1 Word 7 (Analog Info.)"]
    #[inline(always)]
    pub const fn hw_ocotp_ana2(self) -> crate::common::Reg<regs::HwOcotpAna2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk0(self) -> crate::common::Reg<regs::HwOcotpSrk0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk1(self) -> crate::common::Reg<regs::HwOcotpSrk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk2(self) -> crate::common::Reg<regs::HwOcotpSrk2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk3(self) -> crate::common::Reg<regs::HwOcotpSrk3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk4(self) -> crate::common::Reg<regs::HwOcotpSrk4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk5(self) -> crate::common::Reg<regs::HwOcotpSrk5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk6(self) -> crate::common::Reg<regs::HwOcotpSrk6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk7(self) -> crate::common::Reg<regs::HwOcotpSrk7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn hw_ocotp_sjc_resp0(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSjcResp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn hw_ocotp_sjc_resp1(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSjcResp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word2 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_otfad_cfg0(
        self,
    ) -> crate::common::Reg<regs::HwOcotpOtfadCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word3 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_otfad_cfg1(
        self,
    ) -> crate::common::Reg<regs::HwOcotpOtfadCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word4 (OTFAD)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp3(self) -> crate::common::Reg<regs::HwOcotpGp3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp1(self) -> crate::common::Reg<regs::HwOcotpGp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn hw_ocotp_gp2(self) -> crate::common::Reg<regs::HwOcotpGp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp1(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwGp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp20(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwGp20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp21(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwGp21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp22(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwGp22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
    #[inline(always)]
    pub const fn hw_ocotp_sw_gp23(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSwGp23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
    #[inline(always)]
    pub const fn hw_ocotp_misc_conf0(
        self,
    ) -> crate::common::Reg<regs::HwOcotpMiscConf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
    #[inline(always)]
    pub const fn hw_ocotp_misc_conf1(
        self,
    ) -> crate::common::Reg<regs::HwOcotpMiscConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
    #[inline(always)]
    pub const fn hw_ocotp_srk_revoke(
        self,
    ) -> crate::common::Reg<regs::HwOcotpSrkRevoke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
