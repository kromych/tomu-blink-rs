#[doc = "Register `XORDATA` reader"]
pub type R = crate::R<XORDATA_SPEC>;
#[doc = "Register `XORDATA` writer"]
pub type W = crate::W<XORDATA_SPEC>;
#[doc = "Field `XORDATA` reader - XOR Data Access"]
pub type XORDATA_R = crate::FieldReader<u32>;
#[doc = "Field `XORDATA` writer - XOR Data Access"]
pub type XORDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    pub fn xordata(&self) -> XORDATA_R {
        XORDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    #[must_use]
    pub fn xordata(&mut self) -> XORDATA_W<XORDATA_SPEC, 0> {
        XORDATA_W::new(self)
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
#[doc = "XORDATA Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xordata::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xordata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XORDATA_SPEC;
impl crate::RegisterSpec for XORDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xordata::R`](R) reader structure"]
impl crate::Readable for XORDATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xordata::W`](W) writer structure"]
impl crate::Writable for XORDATA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XORDATA to value 0"]
impl crate::Resettable for XORDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
