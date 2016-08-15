use std::marker::Sized;

use buffoon::{self, Serialize, Deserialize};

use error::{Error, Result};

pub trait FromProto
    where Self: Sized + Deserialize
{
    fn from_proto(bytes: &[u8]) -> Result<Self> {
        buffoon::deserialize(bytes).map_err(|_| Error::Parse)
    }
}

pub trait ToProto
    where Self: Sized + Serialize
{
    fn to_proto(&self) -> Result<Vec<u8>> {
        buffoon::serialize(&self).map_err(|_| Error::Serialize)
    }
}