pub mod decode;
pub mod encode;

pub use decode::Decode;
pub use encode::Encode;

#[cfg(feature = "derive")]
pub use endecode_derive::{Decode, Encode};

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use rand::{thread_rng, Rng};

    use crate::{Decode, Encode};

    #[test]
    fn test_ints() {
        macro_rules! test_ints {
            ($rng:ident, $($t:ty),*) => {
                $(
                    for _ in 0..1024 {
                        let v = $rng.gen_range(<$t>::MIN..=<$t>::MAX);
                        assert_eq!(<$t>::decode(&mut v.encode().into_iter()), v);
                    }
                )*
            };
        }

        let mut rng = thread_rng();
        test_ints!(rng, u8, u16, u32, u64, u128, usize);
        test_ints!(rng, i8, i16, i32, i64, i128, isize);
    }

    #[test]
    fn test_vecs() {
        assert_eq!(
            Vec::<i32>::decode(
                &mut vec![
                    -1481226730,
                    -2050900860,
                    1398413233,
                    1179549798,
                    57731936,
                    987165288,
                    520248252,
                    -216621864,
                    310968107,
                    1116392241
                ]
                .encode()
                .into_iter()
            ),
            vec![
                -1481226730,
                -2050900860,
                1398413233,
                1179549798,
                57731936,
                987165288,
                520248252,
                -216621864,
                310968107,
                1116392241
            ]
        );
    }

    #[test]
    fn test_strs() {
        assert_eq!(
            "asd".encode(),
            vec![0, 0, 0, 0, 0, 0, 0, 3, 0x61, 0x73, 0x64]
        );
        assert_eq!(
            String::decode(&mut "asd".encode().into_iter()),
            "asd".to_owned()
        );
        assert_eq!(
            String::decode(&mut "asd".to_owned().encode().into_iter()),
            "asd".to_owned()
        );
    }

    #[test]
    fn test_hashmaps() {
        let mut map = HashMap::new();
        map.insert((0, 2, 5), "acf".to_owned());
        map.insert((20, 10, 0), "zka".to_owned());

        assert_eq!(HashMap::decode(&mut map.encode().into_iter()), map);
    }
}
