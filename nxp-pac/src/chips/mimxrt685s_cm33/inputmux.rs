#[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ct32bitCapSel {
    ptr: *mut u8,
}
unsafe impl Send for Ct32bitCapSel {}
unsafe impl Sync for Ct32bitCapSel {}
impl Ct32bitCapSel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
    #[inline(always)]
    pub const fn ct32bit_cap_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ct32bitCapSel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
#[doc = "LPC_Next0 Peripheral Input Multiplexers Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inputmux {
    ptr: *mut u8,
}
unsafe impl Send for Inputmux {}
unsafe impl Sync for Inputmux {}
impl Inputmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SCT Peripheral Input Multiplexers N"]
    #[inline(always)]
    pub const fn sct0_in_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Sct0InSel, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "GPIO Pin Input Multiplexer N"]
    #[inline(always)]
    pub const fn pint_sel(self, n: usize) -> crate::common::Reg<regs::PintSel, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _)
        }
    }
    #[doc = "DSP Interrupt Input Multiplexers N"]
    #[inline(always)]
    pub const fn dsp_int_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::DspIntSel, crate::common::RW> {
        assert!(n < 27usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "DMAC0 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac0_itrig_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dmac0ItrigSel, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "DMAC0 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac0_otrig_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dmac0OtrigSel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "DMAC1 Input Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac1_itrig_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dmac1ItrigSel, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "DMAC1 Output Trigger Multiplexers N"]
    #[inline(always)]
    pub const fn dmac1_otrig_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Dmac1OtrigSel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "CT32BITn Counter Timer Capture Trigger Multiplexers"]
    #[inline(always)]
    pub const fn ct32bit_cap_sel(self, n: usize) -> Ct32bitCapSel {
        assert!(n < 5usize);
        unsafe { Ct32bitCapSel::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 16usize) as _) }
    }
    #[doc = "Frequency Measurement Input Channel Multiplexers"]
    #[inline(always)]
    pub const fn fmeasure_ch_sel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::FmeasureChSel, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize + n * 4usize) as _)
        }
    }
    #[doc = "DMAC0 request enable 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0(self) -> crate::common::Reg<regs::Dmac0ReqEna0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "DMAC0 request enable 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1(self) -> crate::common::Reg<regs::Dmac0ReqEna1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0744usize) as _) }
    }
    #[doc = "DMAC0 request enable set 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0_set(
        self,
    ) -> crate::common::Reg<regs::Dmac0ReqEna0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0748usize) as _) }
    }
    #[doc = "DMAC0 request enable set 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1_set(
        self,
    ) -> crate::common::Reg<regs::Dmac0ReqEna1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[doc = "DMAC0 request enable clear 0"]
    #[inline(always)]
    pub const fn dmac0_req_ena0_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac0ReqEna0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "DMAC0 request enable clear 1"]
    #[inline(always)]
    pub const fn dmac0_req_ena1_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac0ReqEna1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0754usize) as _) }
    }
    #[doc = "DMAC1 request enable 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0(self) -> crate::common::Reg<regs::Dmac1ReqEna0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "DMAC1 request enable 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1(self) -> crate::common::Reg<regs::Dmac1ReqEna1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "DMAC1 request enable set 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0_set(
        self,
    ) -> crate::common::Reg<regs::Dmac1ReqEna0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "DMAC1 request enable set 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1_set(
        self,
    ) -> crate::common::Reg<regs::Dmac1ReqEna1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x076cusize) as _) }
    }
    #[doc = "DMAC1 request enable clear 0"]
    #[inline(always)]
    pub const fn dmac1_req_ena0_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac1ReqEna0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "DMAC1 request enable clear 1"]
    #[inline(always)]
    pub const fn dmac1_req_ena1_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac1ReqEna1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0774usize) as _) }
    }
    #[doc = "DMAC0 input trigger enable 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0(
        self,
    ) -> crate::common::Reg<regs::Dmac0ItrigEna0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "DMAC0 input trigger enable set 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0_set(
        self,
    ) -> crate::common::Reg<regs::Dmac0ItrigEna0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "DMAC0 input trigger enable clear 0"]
    #[inline(always)]
    pub const fn dmac0_itrig_ena0_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac0ItrigEna0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "DMAC1 input trigger enable 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0(
        self,
    ) -> crate::common::Reg<regs::Dmac1ItrigEna0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "DMAC1 input trigger enable set 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0_set(
        self,
    ) -> crate::common::Reg<regs::Dmac1ItrigEna0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "DMAC1 input trigger enable clear 0"]
    #[inline(always)]
    pub const fn dmac1_itrig_ena0_clr(
        self,
    ) -> crate::common::Reg<regs::Dmac1ItrigEna0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
