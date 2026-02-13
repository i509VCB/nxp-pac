#[doc = "Array of registers: EV_STATE, EV_CTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Event {
    ptr: *mut u8,
}
unsafe impl Send for Event {}
unsafe impl Sync for Event {}
impl Event {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Event n State"]
    #[inline(always)]
    pub const fn ev_state(self) -> crate::common::Reg<regs::EvState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Event n Control"]
    #[inline(always)]
    pub const fn ev_ctrl(self) -> crate::common::Reg<regs::EvCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "Array of registers: OUT_SET, OUT_CLR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out {
    ptr: *mut u8,
}
unsafe impl Send for Out {}
unsafe impl Sync for Out {}
impl Out {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Output n Set"]
    #[inline(always)]
    pub const fn out_set(self) -> crate::common::Reg<regs::OutSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Output n Clear"]
    #[inline(always)]
    pub const fn out_clr(self) -> crate::common::Reg<regs::OutClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "SCT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0 {
    ptr: *mut u8,
}
unsafe impl Send for Sct0 {}
unsafe impl Sync for Sct0 {}
impl Sct0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT Configuration"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SCT Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SCT Limit Event Select"]
    #[inline(always)]
    pub const fn limit(self) -> crate::common::Reg<regs::Limit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Halt Event Select"]
    #[inline(always)]
    pub const fn halt(self) -> crate::common::Reg<regs::Halt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Stop Event Select"]
    #[inline(always)]
    pub const fn stop(self) -> crate::common::Reg<regs::Stop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Start Event Select"]
    #[inline(always)]
    pub const fn start(self) -> crate::common::Reg<regs::Start, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Dither Condition"]
    #[inline(always)]
    pub const fn dither(self) -> crate::common::Reg<regs::Dither, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Counter Value"]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "State Variable"]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Input State"]
    #[inline(always)]
    pub const fn input(self) -> crate::common::Reg<regs::Input, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Match and Capture Register Mode"]
    #[inline(always)]
    pub const fn regmode(self) -> crate::common::Reg<regs::Regmode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Output State"]
    #[inline(always)]
    pub const fn output(self) -> crate::common::Reg<regs::Output, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Output Counter Direction Control"]
    #[inline(always)]
    pub const fn outputdirctrl(self) -> crate::common::Reg<regs::Outputdirctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Output Conflict Resolution"]
    #[inline(always)]
    pub const fn res(self) -> crate::common::Reg<regs::Res, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "DMA Request 0"]
    #[inline(always)]
    pub const fn dmareq0(self) -> crate::common::Reg<regs::Dmareq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "DMA Request 1"]
    #[inline(always)]
    pub const fn dmareq1(self) -> crate::common::Reg<regs::Dmareq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Event Interrupt Enable"]
    #[inline(always)]
    pub const fn even(self) -> crate::common::Reg<regs::Even, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Event Flag"]
    #[inline(always)]
    pub const fn evflag(self) -> crate::common::Reg<regs::Evflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Conflict Interrupt Enable"]
    #[inline(always)]
    pub const fn conen(self) -> crate::common::Reg<regs::Conen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Conflict Flag"]
    #[inline(always)]
    pub const fn conflag(self) -> crate::common::Reg<regs::Conflag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap0(self) -> crate::common::Reg<regs::Cap0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match0(self) -> crate::common::Reg<regs::Match0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap1(self) -> crate::common::Reg<regs::Cap1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match1(self) -> crate::common::Reg<regs::Match1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap2(self) -> crate::common::Reg<regs::Cap2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match2(self) -> crate::common::Reg<regs::Match2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap3(self) -> crate::common::Reg<regs::Cap3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match3(self) -> crate::common::Reg<regs::Match3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap4(self) -> crate::common::Reg<regs::Cap4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match4(self) -> crate::common::Reg<regs::Match4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap5(self) -> crate::common::Reg<regs::Cap5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match5(self) -> crate::common::Reg<regs::Match5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap6(self) -> crate::common::Reg<regs::Cap6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match6(self) -> crate::common::Reg<regs::Match6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap7(self) -> crate::common::Reg<regs::Cap7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match7(self) -> crate::common::Reg<regs::Match7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap8(self) -> crate::common::Reg<regs::Cap8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match8(self) -> crate::common::Reg<regs::Match8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap9(self) -> crate::common::Reg<regs::Cap9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match9(self) -> crate::common::Reg<regs::Match9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap10(self) -> crate::common::Reg<regs::Cap10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match10(self) -> crate::common::Reg<regs::Match10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap11(self) -> crate::common::Reg<regs::Cap11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match11(self) -> crate::common::Reg<regs::Match11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap12(self) -> crate::common::Reg<regs::Cap12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match12(self) -> crate::common::Reg<regs::Match12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap13(self) -> crate::common::Reg<regs::Cap13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match13(self) -> crate::common::Reg<regs::Match13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap14(self) -> crate::common::Reg<regs::Cap14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match14(self) -> crate::common::Reg<regs::Match14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "Capture Value"]
    #[inline(always)]
    pub const fn cap15(self) -> crate::common::Reg<regs::Cap15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Match Value"]
    #[inline(always)]
    pub const fn match15(self) -> crate::common::Reg<regs::Match15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "Fractional Match"]
    #[inline(always)]
    pub const fn fracmat(self, n: usize) -> crate::common::Reg<regs::Fracmat, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl0(self) -> crate::common::Reg<regs::Capctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel0(self) -> crate::common::Reg<regs::Matchrel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl1(self) -> crate::common::Reg<regs::Capctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel1(self) -> crate::common::Reg<regs::Matchrel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl2(self) -> crate::common::Reg<regs::Capctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel2(self) -> crate::common::Reg<regs::Matchrel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl3(self) -> crate::common::Reg<regs::Capctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel3(self) -> crate::common::Reg<regs::Matchrel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl4(self) -> crate::common::Reg<regs::Capctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel4(self) -> crate::common::Reg<regs::Matchrel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl5(self) -> crate::common::Reg<regs::Capctrl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel5(self) -> crate::common::Reg<regs::Matchrel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl6(self) -> crate::common::Reg<regs::Capctrl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel6(self) -> crate::common::Reg<regs::Matchrel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl7(self) -> crate::common::Reg<regs::Capctrl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel7(self) -> crate::common::Reg<regs::Matchrel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl8(self) -> crate::common::Reg<regs::Capctrl8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel8(self) -> crate::common::Reg<regs::Matchrel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl9(self) -> crate::common::Reg<regs::Capctrl9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel9(self) -> crate::common::Reg<regs::Matchrel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl10(self) -> crate::common::Reg<regs::Capctrl10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel10(self) -> crate::common::Reg<regs::Matchrel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl11(self) -> crate::common::Reg<regs::Capctrl11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel11(self) -> crate::common::Reg<regs::Matchrel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl12(self) -> crate::common::Reg<regs::Capctrl12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel12(self) -> crate::common::Reg<regs::Matchrel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl13(self) -> crate::common::Reg<regs::Capctrl13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel13(self) -> crate::common::Reg<regs::Matchrel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl14(self) -> crate::common::Reg<regs::Capctrl14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel14(self) -> crate::common::Reg<regs::Matchrel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "Capture Control"]
    #[inline(always)]
    pub const fn capctrl15(self) -> crate::common::Reg<regs::Capctrl15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Match Reload Value"]
    #[inline(always)]
    pub const fn matchrel15(self) -> crate::common::Reg<regs::Matchrel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "Fractional Match Reload"]
    #[inline(always)]
    pub const fn fracmatrel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Fracmatrel, crate::common::RW> {
        assert!(n < 6usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "Array of registers: EV_STATE, EV_CTRL"]
    #[inline(always)]
    pub const fn event(self, n: usize) -> Event {
        assert!(n < 16usize);
        unsafe { Event::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 8usize) as _) }
    }
    #[doc = "Array of registers: OUT_SET, OUT_CLR"]
    #[inline(always)]
    pub const fn out(self, n: usize) -> Out {
        assert!(n < 10usize);
        unsafe { Out::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
