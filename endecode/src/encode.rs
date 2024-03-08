use paste::paste;
use std::collections::HashMap;

pub trait Encode {
    fn encode_internal(&self, vec: &mut Vec<u8>);
    fn encode(&self) -> Vec<u8> {
        let mut vec = vec![];
        self.encode_internal(&mut vec);
        vec
    }
}

macro_rules! impl_num {
    ($($type:ty),+) => {
        $(
            impl Encode for $type {
                fn encode_internal(&self, vec: &mut Vec<u8>) {
                    vec.extend(self.to_be_bytes());
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
            impl<$([<T $num>]),*> Encode for ($([<T $num>],)*)
            where
                $(
                    [<T $num>]: Encode
                ),*
            {
                #[allow(unused_variables)]
                fn encode_internal(&self, vec: &mut Vec<u8>) {
                    $(
                        self.$num.encode_internal(vec);
                    )*
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

impl Encode for bool {
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        vec.push(*self as u8);
    }
}

impl<T> Encode for Option<T>
where
    T: Encode,
{
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        self.is_some().encode_internal(vec);
        if let Some(v) = self {
            v.encode_internal(vec);
        }
    }
}

impl Encode for String {
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        self.len().encode_internal(vec);
        vec.extend(self.bytes());
    }
}

impl Encode for &str {
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        self.len().encode_internal(vec);
        vec.extend(self.bytes());
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        self.len().encode_internal(vec);
        for i in self.iter() {
            i.encode_internal(vec);
        }
    }
}

impl<K, V> Encode for HashMap<K, V>
where
    K: Encode,
    V: Encode,
{
    fn encode_internal(&self, vec: &mut Vec<u8>) {
        self.len().encode_internal(vec);
        for (k, v) in self.iter() {
            k.encode_internal(vec);
            v.encode_internal(vec);
        }
    }
}
