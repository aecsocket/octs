use {
    crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write},
    core::{
        convert::Infallible,
        marker::{PhantomData, PhantomPinned},
    },
};

macro_rules! impl_zero_sized {
    (for $ty:tt $( <$ty_param:ident: ?Sized> )?) => {
        impl$( <$ty_param: ?Sized> )? FixedEncodeLen for $ty $( <$ty_param> )? {
            const ENCODE_LEN: usize = 0;
        }

        impl$( <$ty_param: ?Sized> )? Decode for $ty $( <$ty_param> )? {
            type Error = Infallible;

            #[inline]
            fn decode(_: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                Ok($ty)
            }
        }

        impl$( <$ty_param: ?Sized> )? Encode for $ty $( <$ty_param> )? {
            type Error = Infallible;

            #[inline]
            fn encode(&self, _: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                Ok(())
            }
        }
    };
}

impl_zero_sized!(for ());
impl_zero_sized!(for PhantomPinned);
impl_zero_sized!(for PhantomData<T: ?Sized>);
// don't implement for [T; 0] because this conflicts with [T; N]

#[cfg(test)]
mod tests {
    use {
        crate::test::*,
        core::marker::{PhantomData, PhantomPinned},
    };

    #[test]
    fn round_trip_unit() {
        hint_round_trip(&());
    }

    #[test]
    fn round_trip_phantom_pinned() {
        hint_round_trip(&PhantomPinned);
    }

    #[test]
    fn round_trip_phantom_data() {
        hint_round_trip(&PhantomData::<i32>);
    }
}
