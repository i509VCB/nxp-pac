#[doc = "LPC-Next0 RNG"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
unsafe impl Send for Trng {}
unsafe impl Sync for Trng {}
impl Trng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn mctl(self) -> crate::common::Reg<regs::Mctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Statistical Check Miscellaneous Register"]
    #[inline(always)]
    pub const fn scmisc(self) -> crate::common::Reg<regs::Scmisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Poker Range Register"]
    #[inline(always)]
    pub const fn pkrrng(self) -> crate::common::Reg<regs::Pkrrng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Poker Maximum Limit Register"]
    #[inline(always)]
    pub const fn pkrmax(self) -> crate::common::Reg<regs::Pkrmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Poker Square Calculation Result Register"]
    #[inline(always)]
    pub const fn pkrsq(self) -> crate::common::Reg<regs::Pkrsq, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Seed Control Register"]
    #[inline(always)]
    pub const fn sdctl(self) -> crate::common::Reg<regs::Sdctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Sparse Bit Limit Register"]
    #[inline(always)]
    pub const fn sblim(self) -> crate::common::Reg<regs::Sblim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Total Samples Register"]
    #[inline(always)]
    pub const fn totsam(self) -> crate::common::Reg<regs::Totsam, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Frequency Count Minimum Limit Register"]
    #[inline(always)]
    pub const fn frqmin(self) -> crate::common::Reg<regs::Frqmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Frequency Count Register"]
    #[inline(always)]
    pub const fn frqcnt(self) -> crate::common::Reg<regs::Frqcnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Frequency Count Maximum Limit Register"]
    #[inline(always)]
    pub const fn frqmax(self) -> crate::common::Reg<regs::Frqmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Statistical Check Monobit Count Register"]
    #[inline(always)]
    pub const fn scmc(self) -> crate::common::Reg<regs::Scmc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Statistical Check Monobit Limit Register"]
    #[inline(always)]
    pub const fn scml(self) -> crate::common::Reg<regs::Scml, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Statistical Check Run Length 1 Count Register"]
    #[inline(always)]
    pub const fn scr1c(self) -> crate::common::Reg<regs::Scr1c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Statistical Check Run Length 1 Limit Register"]
    #[inline(always)]
    pub const fn scr1l(self) -> crate::common::Reg<regs::Scr1l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Statistical Check Run Length 2 Count Register"]
    #[inline(always)]
    pub const fn scr2c(self) -> crate::common::Reg<regs::Scr2c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Statistical Check Run Length 2 Limit Register"]
    #[inline(always)]
    pub const fn scr2l(self) -> crate::common::Reg<regs::Scr2l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Statistical Check Run Length 3 Count Register"]
    #[inline(always)]
    pub const fn scr3c(self) -> crate::common::Reg<regs::Scr3c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Statistical Check Run Length 3 Limit Register"]
    #[inline(always)]
    pub const fn scr3l(self) -> crate::common::Reg<regs::Scr3l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Statistical Check Run Length 4 Count Register"]
    #[inline(always)]
    pub const fn scr4c(self) -> crate::common::Reg<regs::Scr4c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Statistical Check Run Length 4 Limit Register"]
    #[inline(always)]
    pub const fn scr4l(self) -> crate::common::Reg<regs::Scr4l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Statistical Check Run Length 5 Count Register"]
    #[inline(always)]
    pub const fn scr5c(self) -> crate::common::Reg<regs::Scr5c, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Statistical Check Run Length 5 Limit Register"]
    #[inline(always)]
    pub const fn scr5l(self) -> crate::common::Reg<regs::Scr5l, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Statistical Check Run Length 6+ Count Register"]
    #[inline(always)]
    pub const fn scr6pc(self) -> crate::common::Reg<regs::Scr6pc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Statistical Check Run Length 6+ Limit Register"]
    #[inline(always)]
    pub const fn scr6pl(self) -> crate::common::Reg<regs::Scr6pl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Entropy Read Register"]
    #[inline(always)]
    pub const fn ent(self, n: usize) -> crate::common::Reg<regs::Ent, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Statistical Check Poker Count 1 and 0 Register"]
    #[inline(always)]
    pub const fn pkrcnt10(self) -> crate::common::Reg<regs::Pkrcnt10, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Statistical Check Poker Count 3 and 2 Register"]
    #[inline(always)]
    pub const fn pkrcnt32(self) -> crate::common::Reg<regs::Pkrcnt32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Statistical Check Poker Count 5 and 4 Register"]
    #[inline(always)]
    pub const fn pkrcnt54(self) -> crate::common::Reg<regs::Pkrcnt54, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Statistical Check Poker Count 7 and 6 Register"]
    #[inline(always)]
    pub const fn pkrcnt76(self) -> crate::common::Reg<regs::Pkrcnt76, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Statistical Check Poker Count 9 and 8 Register"]
    #[inline(always)]
    pub const fn pkrcnt98(self) -> crate::common::Reg<regs::Pkrcnt98, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Statistical Check Poker Count B and A Register"]
    #[inline(always)]
    pub const fn pkrcntba(self) -> crate::common::Reg<regs::Pkrcntba, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Statistical Check Poker Count D and C Register"]
    #[inline(always)]
    pub const fn pkrcntdc(self) -> crate::common::Reg<regs::Pkrcntdc, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Statistical Check Poker Count F and E Register"]
    #[inline(always)]
    pub const fn pkrcntfe(self) -> crate::common::Reg<regs::Pkrcntfe, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Security Configuration Register"]
    #[inline(always)]
    pub const fn sec_cfg(self) -> crate::common::Reg<regs::SecCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Interrupt Control Register"]
    #[inline(always)]
    pub const fn int_ctrl(self) -> crate::common::Reg<regs::IntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Mask Register"]
    #[inline(always)]
    pub const fn int_mask(self) -> crate::common::Reg<regs::IntMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Version ID Register (MS)"]
    #[inline(always)]
    pub const fn vid1(self) -> crate::common::Reg<regs::Vid1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Version ID Register (LS)"]
    #[inline(always)]
    pub const fn vid2(self) -> crate::common::Reg<regs::Vid2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
