pub mod lmdb;

use block::Block;
use direntry::DirEntry;
use entry::{self, Entry};
use error::Result;
use metadata::Metadata;
use path::Path;

pub trait Transaction {
    type D;

    fn commit(self) -> Result<()>;
    fn get(&self, db: Self::D, key: &[u8]) -> Result<&[u8]>;
    fn find<P>(&self, db: Self::D, key: &[u8], predicate: P) -> Result<&[u8]>
        where P: Fn(&[u8]) -> bool;
}

pub trait Adapter<'a> {
    type D;
    type R: Transaction<D = Self::D>;
    type W: Transaction<D = Self::D>;

    fn ro_transaction(&'a self) -> Result<Self::R>;
    fn rw_transaction(&'a self) -> Result<Self::W>;
    fn next_free_entry_id(&self, txn: &Self::W) -> Result<entry::Id>;
    fn add_block<'b>(&'b self, txn: &'b mut Self::W, block: &Block) -> Result<()>;
    fn add_entry<'b>(&'b self,
                     txn: &'b mut Self::W,
                     id: entry::Id,
                     parent_id: entry::Id,
                     name: &'b str,
                     metadata: &Metadata,
                     entry: &Entry)
                     -> Result<DirEntry>;
    fn find_direntry<'b, T: Transaction<D = Self::D>>(&'b self,
                                                      txn: &'b T,
                                                      path: &Path)
                                                      -> Result<DirEntry>;
    fn find_metadata<'b, T: Transaction<D = Self::D>>(&'b self,
                                                      txn: &'b T,
                                                      id: &entry::Id)
                                                      -> Result<Metadata>;
    fn find_entry<'b, T: Transaction<D = Self::D>>(&'b self,
                                                   txn: &'b T,
                                                   id: &entry::Id)
                                                   -> Result<Entry>;
}