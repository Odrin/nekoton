use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasher, Hash};
use std::sync::Arc;

use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;
use ton_abi::{Token, TokenValue};
use ton_block::{MsgAddrStd, MsgAddress, MsgAddressInt};
use ton_types::Cell;

use super::{MaybeRef, StandaloneToken};

pub trait TokenValueExt {
    fn unnamed(self) -> Token;

    fn named<T>(self, name: T) -> Token
    where
        T: ToString;
}

impl TokenValueExt for TokenValue {
    fn unnamed(self) -> Token {
        Token {
            name: String::new(),
            value: self,
        }
    }

    fn named<T>(self, name: T) -> Token
    where
        T: ToString,
    {
        Token {
            name: name.to_string(),
            value: self,
        }
    }
}

pub trait IntoUnpacker {
    type Iter: Iterator<Item = Token>;

    fn into_unpacker(self) -> ContractOutputUnpacker<Self::Iter>;
}

impl IntoUnpacker for Vec<Token> {
    type Iter = std::vec::IntoIter<Token>;

    fn into_unpacker(self) -> ContractOutputUnpacker<Self::Iter> {
        ContractOutputUnpacker(self.into_iter())
    }
}

pub trait UnpackFirst {
    fn unpack_first<T>(self) -> UnpackerResult<T>
    where
        TokenValue: UnpackAbi<T>;
}

impl UnpackFirst for Vec<Token> {
    fn unpack_first<T>(self) -> UnpackerResult<T>
    where
        TokenValue: UnpackAbi<T>,
    {
        self.into_unpacker().unpack_next()
    }
}

#[derive(Debug)]
pub struct ContractOutputUnpacker<I>(I);

impl<I: Iterator<Item = Token>> ContractOutputUnpacker<I> {
    pub fn unpack_next<T>(&mut self) -> UnpackerResult<T>
    where
        TokenValue: UnpackAbi<T>,
    {
        self.0.next().unpack()
    }
}

pub trait UnpackAbiPlain<T>: FunctionOutputMarker {
    fn unpack(self) -> UnpackerResult<T>;
}

pub trait FunctionOutputMarker {}
impl FunctionOutputMarker for Vec<ton_abi::Token> {}

pub trait UnpackAbi<T> {
    fn unpack(self) -> UnpackerResult<T>;
}

impl UnpackAbi<i8> for TokenValue {
    fn unpack(self) -> UnpackerResult<i8> {
        UnpackAbi::<BigInt>::unpack(self)?
            .to_i8()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<u8> for TokenValue {
    fn unpack(self) -> UnpackerResult<u8> {
        UnpackAbi::<BigUint>::unpack(self)?
            .to_u8()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<i16> for TokenValue {
    fn unpack(self) -> UnpackerResult<i16> {
        UnpackAbi::<BigInt>::unpack(self)?
            .to_i16()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<u16> for TokenValue {
    fn unpack(self) -> UnpackerResult<u16> {
        UnpackAbi::<BigUint>::unpack(self)?
            .to_u16()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<i32> for TokenValue {
    fn unpack(self) -> UnpackerResult<i32> {
        UnpackAbi::<BigInt>::unpack(self)?
            .to_i32()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<u32> for TokenValue {
    fn unpack(self) -> UnpackerResult<u32> {
        UnpackAbi::<BigUint>::unpack(self)?
            .to_u32()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<i64> for TokenValue {
    fn unpack(self) -> UnpackerResult<i64> {
        UnpackAbi::<BigInt>::unpack(self)?
            .to_i64()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<u64> for TokenValue {
    fn unpack(self) -> UnpackerResult<u64> {
        UnpackAbi::<BigUint>::unpack(self)?
            .to_u64()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<i128> for TokenValue {
    fn unpack(self) -> UnpackerResult<i128> {
        UnpackAbi::<BigInt>::unpack(self)?
            .to_i128()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<u128> for TokenValue {
    fn unpack(self) -> UnpackerResult<u128> {
        UnpackAbi::<BigUint>::unpack(self)?
            .to_u128()
            .ok_or(UnpackerError::InvalidAbi)
    }
}

impl UnpackAbi<ton_types::UInt256> for TokenValue {
    fn unpack(self) -> UnpackerResult<ton_types::UInt256> {
        match self {
            TokenValue::Uint(ton_abi::Uint { number, size: 256 }) => {
                let mut result = [0u8; 32];
                let data = number.to_bytes_be();

                let len = std::cmp::min(data.len(), 32);
                let offset = 32 - len;
                (0..len).for_each(|i| result[i + offset] = data[i]);

                Ok(result.into())
            }
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<bool> for TokenValue {
    fn unpack(self) -> UnpackerResult<bool> {
        match self {
            TokenValue::Bool(confirmed) => Ok(confirmed),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<Cell> for TokenValue {
    fn unpack(self) -> UnpackerResult<Cell> {
        match self {
            TokenValue::Cell(cell) => Ok(cell),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<MsgAddressInt> for TokenValue {
    fn unpack(self) -> UnpackerResult<MsgAddressInt> {
        match self {
            TokenValue::Address(ton_block::MsgAddress::AddrStd(addr)) => {
                Ok(MsgAddressInt::AddrStd(addr))
            }
            TokenValue::Address(ton_block::MsgAddress::AddrVar(addr)) => {
                Ok(MsgAddressInt::AddrVar(addr))
            }
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<MsgAddress> for TokenValue {
    fn unpack(self) -> UnpackerResult<MsgAddress> {
        match self {
            TokenValue::Address(address) => Ok(address),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<MsgAddrStd> for TokenValue {
    fn unpack(self) -> UnpackerResult<MsgAddrStd> {
        match self {
            TokenValue::Address(ton_block::MsgAddress::AddrStd(addr)) => Ok(addr),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<String> for TokenValue {
    fn unpack(self) -> UnpackerResult<String> {
        match self {
            TokenValue::String(data) => Ok(data),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<BigInt> for TokenValue {
    fn unpack(self) -> UnpackerResult<BigInt> {
        match self {
            TokenValue::Int(data) => Ok(data.number),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<BigUint> for TokenValue {
    fn unpack(self) -> UnpackerResult<BigUint> {
        match self {
            TokenValue::Uint(data) => Ok(data.number),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<Vec<u8>> for TokenValue {
    fn unpack(self) -> UnpackerResult<Vec<u8>> {
        match self {
            TokenValue::Bytes(bytes) => Ok(bytes),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<T> UnpackAbi<Vec<T>> for TokenValue
where
    TokenValue: UnpackAbi<T>,
    T: StandaloneToken,
{
    fn unpack(self) -> UnpackerResult<Vec<T>> {
        match self {
            TokenValue::Array(_, tokens) | TokenValue::FixedArray(_, tokens) => {
                let mut vec = Vec::with_capacity(tokens.len());
                for token in tokens {
                    vec.push(token.unpack()?);
                }
                Ok(vec)
            }
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<ton_block::Grams> for TokenValue {
    fn unpack(self) -> UnpackerResult<ton_block::Grams> {
        match self {
            TokenValue::Token(grams) => Ok(grams),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<K, V> UnpackAbi<BTreeMap<K, V>> for TokenValue
where
    K: Ord,
    TokenValue: UnpackAbi<K> + UnpackAbi<V>,
{
    fn unpack(self) -> UnpackerResult<BTreeMap<K, V>> {
        match self {
            TokenValue::Map(_, _, values) => {
                let mut map = BTreeMap::<K, V>::new();
                for (key, value) in values {
                    let key = TokenValue::from(key.to_owned()).unpack()?;
                    let value: V = value.to_owned().unpack()?;
                    map.insert(key, value);
                }
                Ok(map)
            }
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<K, V, S> UnpackAbi<HashMap<K, V, S>> for TokenValue
where
    K: Eq + Hash,
    TokenValue: UnpackAbi<K> + UnpackAbi<V>,
    S: BuildHasher + Default,
{
    fn unpack(self) -> UnpackerResult<HashMap<K, V, S>> {
        match self {
            TokenValue::Map(_, _, values) => {
                let mut map = HashMap::with_capacity_and_hasher(values.len(), Default::default());
                for (key, value) in values {
                    let key = TokenValue::from(key.to_owned()).unpack()?;
                    let value = value.to_owned().unpack()?;
                    map.insert(key, value);
                }
                Ok(map)
            }
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl UnpackAbi<TokenValue> for TokenValue {
    #[inline]
    fn unpack(self) -> UnpackerResult<TokenValue> {
        Ok(self)
    }
}

impl<T> UnpackAbi<Option<T>> for TokenValue
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<Option<T>> {
        match self {
            TokenValue::Optional(_, item) => item.map(|item| item.unpack()).transpose(),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<T> UnpackAbi<MaybeRef<T>> for TokenValue
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<MaybeRef<T>> {
        match self {
            TokenValue::Optional(_, Some(item)) => match *item {
                TokenValue::Ref(item) => Ok(MaybeRef(Some(item.unpack()?))),
                _ => Err(UnpackerError::InvalidAbi),
            },
            TokenValue::Optional(_, None) => Ok(MaybeRef(None)),
            _ => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<T> UnpackAbi<Box<T>> for TokenValue
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<Box<T>> {
        self.unpack().map(Box::new)
    }
}

impl<T> UnpackAbi<Arc<T>> for TokenValue
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<Arc<T>> {
        self.unpack().map(Arc::new)
    }
}

impl<T> UnpackAbi<T> for Option<Token>
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<T> {
        match self {
            Some(token) => token.value.unpack(),
            None => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<T> UnpackAbi<T> for Option<TokenValue>
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<T> {
        match self {
            Some(value) => value.unpack(),
            None => Err(UnpackerError::InvalidAbi),
        }
    }
}

impl<T> UnpackAbi<T> for Token
where
    TokenValue: UnpackAbi<T>,
{
    fn unpack(self) -> UnpackerResult<T> {
        self.value.unpack()
    }
}

pub type UnpackerResult<T> = Result<T, UnpackerError>;

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum UnpackerError {
    #[error("Invalid ABI")]
    InvalidAbi,
}
