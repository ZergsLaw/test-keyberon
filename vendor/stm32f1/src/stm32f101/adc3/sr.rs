#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_A {
    #[doc = "0: No regular channel conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Regular channel conversion has started"]
    STARTED = 1,
}
impl From<STRT_A> for bool {
    #[inline(always)]
    fn from(variant: STRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STRT`"]
pub type STRT_R = crate::R<bool, STRT_A>;
impl STRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRT_A {
        match self.bits {
            false => STRT_A::NOTSTARTED,
            true => STRT_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRT_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRT_A::STARTED
    }
}
#[doc = "Regular channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STRT_AW {
    #[doc = "0: Clear the Regular channel Start flag"]
    CLEAR = 0,
}
impl From<STRT_AW> for bool {
    #[inline(always)]
    fn from(variant: STRT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STRT`"]
pub struct STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> STRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STRT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the Regular channel Start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STRT_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT_A {
    #[doc = "0: No injected group conversion started"]
    NOTSTARTED = 0,
    #[doc = "1: Injected group conversion has started"]
    STARTED = 1,
}
impl From<JSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: JSTRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JSTRT`"]
pub type JSTRT_R = crate::R<bool, JSTRT_A>;
impl JSTRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JSTRT_A {
        match self.bits {
            false => JSTRT_A::NOTSTARTED,
            true => JSTRT_A::STARTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTSTARTED`"]
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRT_A::NOTSTARTED
    }
    #[doc = "Checks if the value of the field is `STARTED`"]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRT_A::STARTED
    }
}
#[doc = "Injected channel start flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSTRT_AW {
    #[doc = "0: Clear Injected channel Start flag"]
    CLEAR = 0,
}
impl From<JSTRT_AW> for bool {
    #[inline(always)]
    fn from(variant: JSTRT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `JSTRT`"]
pub struct JSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JSTRT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear Injected channel Start flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JSTRT_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<JEOC_A> for bool {
    #[inline(always)]
    fn from(variant: JEOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `JEOC`"]
pub type JEOC_R = crate::R<bool, JEOC_A>;
impl JEOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOC_A {
        match self.bits {
            false => JEOC_A::NOTCOMPLETE,
            true => JEOC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOC_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOC_A::COMPLETE
    }
}
#[doc = "Injected channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JEOC_AW {
    #[doc = "0: Clear Injected channel end of conversion flag"]
    CLEAR = 0,
}
impl From<JEOC_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `JEOC`"]
pub struct JEOC_W<'a> {
    w: &'a mut W,
}
impl<'a> JEOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: JEOC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear Injected channel end of conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_A {
    #[doc = "0: Conversion is not complete"]
    NOTCOMPLETE = 0,
    #[doc = "1: Conversion complete"]
    COMPLETE = 1,
}
impl From<EOC_A> for bool {
    #[inline(always)]
    fn from(variant: EOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOC`"]
pub type EOC_R = crate::R<bool, EOC_A>;
impl EOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOC_A {
        match self.bits {
            false => EOC_A::NOTCOMPLETE,
            true => EOC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOC_A::NOTCOMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOC_A::COMPLETE
    }
}
#[doc = "Regular channel end of conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOC_AW {
    #[doc = "0: Clear End of conversion flag"]
    CLEAR = 0,
}
impl From<EOC_AW> for bool {
    #[inline(always)]
    fn from(variant: EOC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EOC`"]
pub struct EOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOC_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear End of conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOC_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_A {
    #[doc = "0: No analog watchdog event occurred"]
    NOEVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<AWD_A> for bool {
    #[inline(always)]
    fn from(variant: AWD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AWD`"]
pub type AWD_R = crate::R<bool, AWD_A>;
impl AWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD_A {
        match self.bits {
            false => AWD_A::NOEVENT,
            true => AWD_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOEVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD_A::NOEVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD_A::EVENT
    }
}
#[doc = "Analog watchdog flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AWD_AW {
    #[doc = "0: Clear the analog watchdog event flag"]
    CLEAR = 0,
}
impl From<AWD_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `AWD`"]
pub struct AWD_W<'a> {
    w: &'a mut W,
}
impl<'a> AWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AWD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the analog watchdog event flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD_AW::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W {
        STRT_W { w: self }
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W {
        JSTRT_W { w: self }
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W {
        JEOC_W { w: self }
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W {
        EOC_W { w: self }
    }
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W {
        AWD_W { w: self }
    }
}
