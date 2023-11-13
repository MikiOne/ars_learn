use std::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BizCode(NonZeroU32);

impl BizCode {
    pub fn code(&self) -> u32 {
        self.0.get()
    }

    pub fn desc(&self) -> Option<&'static str> {
        canonical_reason(self.code())
    }
}

macro_rules! biz_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl BizCode {
        $(
            $(#[$docs])*
            pub const $konst: BizCode = BizCode(unsafe { NonZeroU32::new_unchecked($num) });
        )+

        }

        fn canonical_reason(num: u32) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

biz_codes! {
    (000001, SUCCESS, "Success");
    (000002, FAILURE, "Failure");
}
