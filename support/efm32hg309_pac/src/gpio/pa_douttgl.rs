#[doc = "Register `PA_DOUTTGL` writer"]
pub type W = crate::W<PA_DOUTTGL_SPEC>;
#[doc = "Field `DOUTTGL` writer - Data Out Toggle"]
pub type DOUTTGL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn douttgl(&mut self) -> DOUTTGL_W<PA_DOUTTGL_SPEC, 0> {
        DOUTTGL_W::new(self)
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
#[doc = "Port Data Out Toggle Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_douttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_DOUTTGL_SPEC;
impl crate::RegisterSpec for PA_DOUTTGL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pa_douttgl::W`](W) writer structure"]
impl crate::Writable for PA_DOUTTGL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA_DOUTTGL to value 0"]
impl crate::Resettable for PA_DOUTTGL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
