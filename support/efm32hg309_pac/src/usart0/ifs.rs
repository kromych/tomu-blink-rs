#[doc = "Register `IFS` writer"]
pub type W = crate::W<IFS_SPEC>;
#[doc = "Field `TXC` writer - Set TX Complete Interrupt Flag"]
pub type TXC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXFULL` writer - Set RX Buffer Full Interrupt Flag"]
pub type RXFULL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXOF` writer - Set RX Overflow Interrupt Flag"]
pub type RXOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RXUF` writer - Set RX Underflow Interrupt Flag"]
pub type RXUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXOF` writer - Set TX Overflow Interrupt Flag"]
pub type TXOF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXUF` writer - Set TX Underflow Interrupt Flag"]
pub type TXUF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PERR` writer - Set Parity Error Interrupt Flag"]
pub type PERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FERR` writer - Set Framing Error Interrupt Flag"]
pub type FERR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MPAF` writer - Set Multi-Processor Address Frame Interrupt Flag"]
pub type MPAF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSM` writer - Set Slave-Select in Master mode Interrupt Flag"]
pub type SSM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCF` writer - Set Collision Check Fail Interrupt Flag"]
pub type CCF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Set TX Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<IFS_SPEC, 0> {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Set RX Buffer Full Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<IFS_SPEC, 3> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - Set RX Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<IFS_SPEC, 4> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - Set RX Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<IFS_SPEC, 5> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - Set TX Overflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<IFS_SPEC, 6> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - Set TX Underflow Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<IFS_SPEC, 7> {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - Set Parity Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<IFS_SPEC, 8> {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - Set Framing Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<IFS_SPEC, 9> {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Set Multi-Processor Address Frame Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<IFS_SPEC, 10> {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - Set Slave-Select in Master mode Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<IFS_SPEC, 11> {
        SSM_W::new(self)
    }
    #[doc = "Bit 12 - Set Collision Check Fail Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<IFS_SPEC, 12> {
        CCF_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifs::W`](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
