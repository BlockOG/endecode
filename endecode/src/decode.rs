use paste::paste;
use std::{collections::HashMap, hash::Hash, marker::PhantomData, mem::size_of, rc::Rc};

pub trait Decode {
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self;
}

macro_rules! impl_num {
    ($($type:ty),+) => {
        $(
            impl Decode for $type {
                fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
                    let mut bytes = [0; size_of::<Self>()];
                    for byte in bytes.iter_mut() {
                        *byte = iter.next().unwrap();
                    }
                    Self::from_be_bytes(bytes)
                }
            }
        )+
    };
}

impl_num!(u8, u16, u32, u64, u128, usize);
impl_num!(i8, i16, i32, i64, i128, isize);
impl_num!(f32, f64);

macro_rules! impl_tuple {
    ($($num:tt),*) => {
        paste! {
            impl<$([<T $num>]),*> Decode for ($([<T $num>],)*)
            where
                $(
                    [<T $num>]: Decode
                ),*
            {
                #[allow(unused_variables)]
                fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
                    (
                        $(
                            [<T $num>]::decode(iter),
                        )*
                    )
                }
            }
        }
    };
}

impl_tuple!();
impl_tuple!(0);
impl_tuple!(0, 1);
impl_tuple!(0, 1, 2);
impl_tuple!(0, 1, 2, 3);
impl_tuple!(0, 1, 2, 3, 4);
impl_tuple!(0, 1, 2, 3, 4, 5);
impl_tuple!(0, 1, 2, 3, 4, 5, 6);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_tuple!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl<T> Decode for PhantomData<T> {
    fn decode(_iter: &mut impl Iterator<Item = u8>) -> Self {
        Self
    }
}

impl Decode for bool {
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        iter.next().unwrap() != 0
    }
}

impl<T> Decode for Option<T>
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        bool::decode(iter).then(|| T::decode(iter))
    }
}

impl Decode for String {
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        Self::from_utf8(Vec::<u8>::decode(iter)).unwrap()
    }
}

impl<T, const N: usize> Decode for [T; N]
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        [(); N].map(|_| T::decode(iter))
    }
}

impl<T> Decode for Box<[T]>
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        (0..usize::decode(iter)).map(|_| T::decode(iter)).collect()
    }
}

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        Box::<[T]>::decode(iter).into_vec()
    }
}

impl<K, V> Decode for HashMap<K, V>
where
    K: Decode + Eq + Hash,
    V: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        (0..usize::decode(iter))
            .map(|_| <(K, V)>::decode(iter))
            .collect()
    }
}

impl<T> Decode for Rc<T>
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        Self::new(T::decode(iter))
    }
}

impl<T, E> Decode for Result<T, E>
where
    T: Decode,
    E: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        Option::<T>::decode(iter).ok_or_else(|| E::decode(iter))
    }
}

impl<T> Decode for Box<T>
where
    T: Decode,
{
    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
        Self::new(T::decode(iter))
    }
}
