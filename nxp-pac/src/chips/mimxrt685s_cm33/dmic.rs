#[doc = "Array of registers: DC_CTRL, DIVHFCLK, FIFO_CTRL, FIFO_DATA, FIFO_STATUS, GAINSHIFT, OSR, PHY_CTRL, PREAC2FSCOEF, PREAC4FSCOEF"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CIC Filter decimation rate"]
    #[inline(always)]
    pub const fn osr(self) -> crate::common::Reg<regs::Osr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Divider for generating PDM clock from DMIC clock input"]
    #[inline(always)]
    pub const fn divhfclk(self) -> crate::common::Reg<regs::Divhfclk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Compensation filter for 2FS"]
    #[inline(always)]
    pub const fn preac2fscoef(self) -> crate::common::Reg<regs::Preac2fscoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Compensation filter for 4FS"]
    #[inline(always)]
    pub const fn preac4fscoef(self) -> crate::common::Reg<regs::Preac4fscoef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Decimator output gain adjustment"]
    #[inline(always)]
    pub const fn gainshift(self) -> crate::common::Reg<regs::Gainshift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FIFO Control"]
    #[inline(always)]
    pub const fn fifo_ctrl(self) -> crate::common::Reg<regs::FifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "FIFO Status"]
    #[inline(always)]
    pub const fn fifo_status(self) -> crate::common::Reg<regs::FifoStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FIFO Data"]
    #[inline(always)]
    pub const fn fifo_data(self) -> crate::common::Reg<regs::FifoData, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Phy Ctrl"]
    #[inline(always)]
    pub const fn phy_ctrl(self) -> crate::common::Reg<regs::PhyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "DC Filter Control"]
    #[inline(always)]
    pub const fn dc_ctrl(self) -> crate::common::Reg<regs::DcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
}
#[doc = "LPC_Next0 DMIC Subsystem (DMIC))"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmic {
    ptr: *mut u8,
}
unsafe impl Send for Dmic {}
unsafe impl Sync for Dmic {}
impl Dmic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: DC_CTRL, DIVHFCLK, FIFO_CTRL, FIFO_DATA, FIFO_STATUS, GAINSHIFT, OSR, PHY_CTRL, PREAC2FSCOEF, PREAC4FSCOEF"]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 8usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x0usize + n * 256usize) as _) }
    }
    #[doc = "Channel Enable register"]
    #[inline(always)]
    pub const fn chanen(self) -> crate::common::Reg<regs::Chanen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f00usize) as _) }
    }
    #[doc = "Use 2FS register"]
    #[inline(always)]
    pub const fn use2fs(self) -> crate::common::Reg<regs::Use2fs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f10usize) as _) }
    }
    #[doc = "global sync enable"]
    #[inline(always)]
    pub const fn global_sync_en(self) -> crate::common::Reg<regs::GlobalSyncEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f14usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn global_count_val(
        self,
    ) -> crate::common::Reg<regs::GlobalCountVal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f18usize) as _) }
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn decreset(self) -> crate::common::Reg<regs::Decreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f1cusize) as _) }
    }
    #[doc = "HWVAD input gain register"]
    #[inline(always)]
    pub const fn hwvadgain(self) -> crate::common::Reg<regs::Hwvadgain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f80usize) as _) }
    }
    #[doc = "HWVAD filter control register"]
    #[inline(always)]
    pub const fn hwvadhpfs(self) -> crate::common::Reg<regs::Hwvadhpfs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f84usize) as _) }
    }
    #[doc = "HWVAD control register"]
    #[inline(always)]
    pub const fn hwvadst10(self) -> crate::common::Reg<regs::Hwvadst10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f88usize) as _) }
    }
    #[doc = "HWVAD filter reset register"]
    #[inline(always)]
    pub const fn hwvadrstt(self) -> crate::common::Reg<regs::Hwvadrstt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f8cusize) as _) }
    }
    #[doc = "HWVAD noise estimator gain register"]
    #[inline(always)]
    pub const fn hwvadthgn(self) -> crate::common::Reg<regs::Hwvadthgn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f90usize) as _) }
    }
    #[doc = "HWVAD signal estimator gain register"]
    #[inline(always)]
    pub const fn hwvadthgs(self) -> crate::common::Reg<regs::Hwvadthgs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f94usize) as _) }
    }
    #[doc = "HWVAD noise envelope estimator register"]
    #[inline(always)]
    pub const fn hwvadlowz(self) -> crate::common::Reg<regs::Hwvadlowz, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0f98usize) as _) }
    }
}
pub mod regs;
pub mod vals;
