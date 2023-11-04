#[doc = "Register `PE_DOUTSET` writer"]
pub type W = crate::W<PE_DOUTSET_SPEC>;
#[doc = "Field `DOUTSET` writer - Data Out Set"]
pub type DOUTSET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl W {
    #[doc = "Bits 0:15 - Data Out Set"]
    #[inline(always)]
    #[must_use]
    pub fn doutset(&mut self) -> DOUTSET_W<PE_DOUTSET_SPEC, 0> {
        DOUTSET_W::new(self)
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
#[doc = "Port Data Out Set Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_doutset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_DOUTSET_SPEC;
impl crate::RegisterSpec for PE_DOUTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pe_doutset::W`](W) writer structure"]
impl crate::Writable for PE_DOUTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PE_DOUTSET to value 0"]
impl crate::Resettable for PE_DOUTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
