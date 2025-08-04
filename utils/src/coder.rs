use bincode;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use serde::{Deserialize, Serialize};

// 序列化
pub fn my_serialize<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

// 反序列化
pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub fn get_hash(value: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(value);
    hasher.result_str()
}

pub fn one_piece_data(from: String, to: String, btc: u32) -> String {
    from + " to " + &to + " -> " + &btc.to_string() + " btc"
}

// for test
#[cfg(test)]
mod tests {
    use crate::coder::Point;
    use crate::coder::{my_deserialize, my_serialize};

    #[test]
    fn coder_works() {
        let point = Point { x: 1, y: 1 };
        let se = my_serialize(&point);
        let dse: Point = my_deserialize(&se);
        assert_eq!(dse, point);
    }
}
