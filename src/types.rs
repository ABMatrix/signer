/// primitive for web3 
//TODO defined
pub use web3::types::{U256, H160, H256, H512, Address, Bytes};

pub trait ToVec {
    fn to_vec(&self) -> Vec<u8>;
}

impl ToVec for U256 {
    fn to_vec(&self) -> Vec<u8> {
        let mut out = [0u8;32];
        self.to_big_endian(&mut out);
        out.to_vec()
    }
}