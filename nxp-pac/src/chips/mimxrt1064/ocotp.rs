#[doc = "OCOTP"]
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
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn ctrl_set(self) -> crate::common::Reg<regs::CtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn ctrl_clr(self) -> crate::common::Reg<regs::CtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "OTP Controller Control and Status Register"]
    #[inline(always)]
    pub const fn ctrl_tog(self) -> crate::common::Reg<regs::CtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "OTP Controller Timing Register"]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "OTP Controller Write Data Register"]
    #[inline(always)]
    pub const fn read_ctrl(self) -> crate::common::Reg<regs::ReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "OTP Controller Read Data Register"]
    #[inline(always)]
    pub const fn read_fuse_data(self) -> crate::common::Reg<regs::ReadFuseData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Sticky bit Register"]
    #[inline(always)]
    pub const fn sw_sticky(self) -> crate::common::Reg<regs::SwSticky, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn scs(self) -> crate::common::Reg<regs::Scs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn scs_set(self) -> crate::common::Reg<regs::ScsSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn scs_clr(self) -> crate::common::Reg<regs::ScsClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Software Controllable Signals Register"]
    #[inline(always)]
    pub const fn scs_tog(self) -> crate::common::Reg<regs::ScsTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "OTP Controller Version Register"]
    #[inline(always)]
    pub const fn version(self) -> crate::common::Reg<regs::Version, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "OTP Controller Timing Register 2"]
    #[inline(always)]
    pub const fn timing2(self) -> crate::common::Reg<regs::Timing2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word0 (Lock controls)"]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg0(self) -> crate::common::Reg<regs::Cfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word2 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg1(self) -> crate::common::Reg<regs::Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word3 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word4 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg3(self) -> crate::common::Reg<regs::Cfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word5 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg4(self) -> crate::common::Reg<regs::Cfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word6 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg5(self) -> crate::common::Reg<regs::Cfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Value of OTP Bank0 Word7 (Configuration and Manufacturing Info.)"]
    #[inline(always)]
    pub const fn cfg6(self) -> crate::common::Reg<regs::Cfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word0 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn mem0(self) -> crate::common::Reg<regs::Mem0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word1 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn mem1(self) -> crate::common::Reg<regs::Mem1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word2 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn mem2(self) -> crate::common::Reg<regs::Mem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word3 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn mem3(self) -> crate::common::Reg<regs::Mem3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word4 (Memory Related Info.)"]
    #[inline(always)]
    pub const fn mem4(self) -> crate::common::Reg<regs::Mem4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word5 (Analog Info.)"]
    #[inline(always)]
    pub const fn ana0(self) -> crate::common::Reg<regs::Ana0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word6 (Analog Info.)"]
    #[inline(always)]
    pub const fn ana1(self) -> crate::common::Reg<regs::Ana1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "Value of OTP Bank1 Word7 (Analog Info.)"]
    #[inline(always)]
    pub const fn ana2(self) -> crate::common::Reg<regs::Ana2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word0 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk0(self) -> crate::common::Reg<regs::Srk0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word1 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk1(self) -> crate::common::Reg<regs::Srk1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word2 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk2(self) -> crate::common::Reg<regs::Srk2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word3 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk3(self) -> crate::common::Reg<regs::Srk3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word4 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk4(self) -> crate::common::Reg<regs::Srk4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word5 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk5(self) -> crate::common::Reg<regs::Srk5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word6 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk6(self) -> crate::common::Reg<regs::Srk6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "Shadow Register for OTP Bank3 Word7 (SRK Hash)"]
    #[inline(always)]
    pub const fn srk7(self) -> crate::common::Reg<regs::Srk7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word0 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn sjc_resp0(self) -> crate::common::Reg<regs::SjcResp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word1 (Secure JTAG Response Field)"]
    #[inline(always)]
    pub const fn sjc_resp1(self) -> crate::common::Reg<regs::SjcResp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word2 (MAC Address)"]
    #[inline(always)]
    pub const fn mac0(self) -> crate::common::Reg<regs::Mac0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word3 (MAC Address)"]
    #[inline(always)]
    pub const fn mac1(self) -> crate::common::Reg<regs::Mac1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word4 (MAC2 Address)"]
    #[inline(always)]
    pub const fn mac2(self) -> crate::common::Reg<regs::Mac2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word6 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn gp1(self) -> crate::common::Reg<regs::Gp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "Value of OTP Bank4 Word7 (General Purpose Customer Defined Info)"]
    #[inline(always)]
    pub const fn gp2(self) -> crate::common::Reg<regs::Gp2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word0 (SW GP1)"]
    #[inline(always)]
    pub const fn sw_gp1(self) -> crate::common::Reg<regs::SwGp1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word1 (SW GP2)"]
    #[inline(always)]
    pub const fn sw_gp20(self) -> crate::common::Reg<regs::SwGp20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word2 (SW GP2)"]
    #[inline(always)]
    pub const fn sw_gp21(self) -> crate::common::Reg<regs::SwGp21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word3 (SW GP2)"]
    #[inline(always)]
    pub const fn sw_gp22(self) -> crate::common::Reg<regs::SwGp22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word4 (SW GP2)"]
    #[inline(always)]
    pub const fn sw_gp23(self) -> crate::common::Reg<regs::SwGp23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word5 (Misc Conf)"]
    #[inline(always)]
    pub const fn misc_conf0(self) -> crate::common::Reg<regs::MiscConf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word6 (Misc Conf)"]
    #[inline(always)]
    pub const fn misc_conf1(self) -> crate::common::Reg<regs::MiscConf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "Value of OTP Bank5 Word7 (SRK Revoke)"]
    #[inline(always)]
    pub const fn srk_revoke(self) -> crate::common::Reg<regs::SrkRevoke, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word0 (GP3)"]
    #[inline(always)]
    pub const fn gp30(self) -> crate::common::Reg<regs::Gp30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word1 (GP3)"]
    #[inline(always)]
    pub const fn gp31(self) -> crate::common::Reg<regs::Gp31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word2 (GP3)"]
    #[inline(always)]
    pub const fn gp32(self) -> crate::common::Reg<regs::Gp32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word3 (GP3)"]
    #[inline(always)]
    pub const fn gp33(self) -> crate::common::Reg<regs::Gp33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08b0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word4 (GP4)"]
    #[inline(always)]
    pub const fn gp40(self) -> crate::common::Reg<regs::Gp40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word5 (GP4)"]
    #[inline(always)]
    pub const fn gp41(self) -> crate::common::Reg<regs::Gp41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08d0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word6 (GP4)"]
    #[inline(always)]
    pub const fn gp42(self) -> crate::common::Reg<regs::Gp42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08e0usize) as _) }
    }
    #[doc = "Value of OTP Bank7 Word7 (GP4)"]
    #[inline(always)]
    pub const fn gp43(self) -> crate::common::Reg<regs::Gp43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08f0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
