#[doc = "Register `PE_DOUTCLR` writer"]
pub type W = crate::W<PE_DOUTCLR_SPEC>;
#[doc = "Field `DOUTCLR` writer - Data Out Clear"]
pub type DOUTCLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Clear"]
    #[inline(always)]
    #[must_use]
    pub fn doutclr(&mut self) -> DOUTCLR_W<PE_DOUTCLR_SPEC, 0> {
        DOUTCLR_W::new(self)
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
#[doc = "Port Data Out Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_doutclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_DOUTCLR_SPEC;
impl crate::RegisterSpec for PE_DOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pe_doutclr::W`](W) writer structure"]
impl crate::Writable for PE_DOUTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PE_DOUTCLR to value 0"]
impl crate::Resettable for PE_DOUTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
