use proc_macro2::TokenStream;
use quote::quote;

pub trait Inline {
    fn inline(&self) -> TokenStream;
}

macro_rules! inline_primitive {
    ($x:ty) => {
        impl Inline for $x {
            fn inline(&self) -> TokenStream {
                quote! { #self }
            }
        }
    };
}

// Primitive implementations
// -----------------------------------------------------------------------------
inline_primitive!(bool);
inline_primitive!(char);
inline_primitive!(f32);
inline_primitive!(f64);
inline_primitive!(i8);
inline_primitive!(i16);
inline_primitive!(i32);
inline_primitive!(i64);
inline_primitive!(i128);
inline_primitive!(isize);
inline_primitive!(str);
inline_primitive!(u8);
inline_primitive!(u16);
inline_primitive!(u32);
inline_primitive!(u64);
inline_primitive!(u128);
impl Inline for () {
    fn inline(&self) -> TokenStream {
        quote! {
            ()
        }
    }
}
inline_primitive!(usize);

// Collections implementations
// -----------------------------------------------------------------------------
impl<T: Inline> Inline for Vec<T> {
    fn inline(&self) -> TokenStream {
        let fields = self.iter().map(|x| x.inline());
        quote! {
            vec![#(#fields,)*]
        }
    }
}

// Misc implementations
// -----------------------------------------------------------------------------
impl<T> Inline for std::marker::PhantomData<T> {
    fn inline(&self) -> TokenStream {
        quote! {
            std::marker::PhantomData
        }
    }
}
impl Inline for String {
    fn inline(&self) -> TokenStream {
        quote! {
            String::from(#self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primitive_test() {
        let a = 2u8;
        assert_eq!(a.inline().to_string(), quote! { 2u8 }.to_string())
    }
    #[test]
    fn vec_test() {
        let a: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(
            a.inline().to_string(),
            quote! { vec![1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8,] }.to_string()
        )
    }
    #[test]
    fn str_test() {
        let a = "1 2 3 a b c 4 d";
        assert_eq!(format!("\"{}\"", a), a.inline().to_string());
    }
    #[test]
    fn string_test() {
        let a = String::from("1 2 3 a b c 4 d");
        assert_eq!(
            a.inline().to_string(),
            quote! { String::from("1 2 3 a b c 4 d") }.to_string()
        )
    }
}
