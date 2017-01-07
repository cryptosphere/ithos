pub mod credential;
pub mod domain;
pub mod ou;
pub mod root;
pub mod system;

use adapter::Adapter;
use buffoon::{Serialize, OutputStream};
use entry::Entry;
use error::{Error, Result};
use objecthash::{ObjectHash, ObjectHasher};
use path::Path;
use proto::{FromProto, ToProto};
use self::credential::CredentialEntry;
use self::domain::DomainEntry;
use self::ou::OrgUnitEntry;
use self::root::RootEntry;
use self::system::SystemEntry;
use serde_json::builder::ObjectBuilder;
use std::io;
use std::mem;

// Object nesting constraints
pub trait AllowsChild {
    fn allows_child(child: &Object) -> bool;
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Class {
    Root, // Root entry in the tree (ala LDAP root DSE)
    Domain, // Administrative Domain (ala DNS domain or Kerberos realm)
    OrgUnit, // Unit/department within an organization
    System, // System User (i.e. non-human account)
    Credential, // Encrypted access credential
}

impl Class {
    pub fn from_bytes(bytes: &[u8]) -> Result<Class> {
        if bytes.len() != 4 {
            return Err(Error::Parse);
        }

        let mut id_bytes = [0u8; 4];
        id_bytes.copy_from_slice(&bytes[0..4]);

        let id: u32 = unsafe { mem::transmute(id_bytes) };

        let result = match id {
            0 => Class::Root,
            1 => Class::Domain,
            2 => Class::OrgUnit,
            3 => Class::System,
            4 => Class::Credential,
            _ => return Err(Error::Parse),
        };

        Ok(result)
    }

    #[inline]
    pub fn allows_child(&self, child: &Object) -> bool {
        match *self {
            Class::Root => RootEntry::allows_child(child),
            Class::Domain => DomainEntry::allows_child(child),
            Class::OrgUnit => OrgUnitEntry::allows_child(child),
            Class::System => SystemEntry::allows_child(child),
            Class::Credential => CredentialEntry::allows_child(child),
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> [u8; 4] {
        let id = *self as u32;
        unsafe { mem::transmute(id) }
    }
}

impl ToString for Class {
    fn to_string(&self) -> String {
        match *self {
            Class::Root => "root".to_string(),
            Class::Domain => "domain".to_string(),
            Class::OrgUnit => "org_unit".to_string(),
            Class::System => "system".to_string(),
            Class::Credential => "credential".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Object {
    Root(RootEntry), // Root entry in the tree (ala LDAP root DSE)
    Domain(DomainEntry), // Administrative Domain (ala DNS domain or Kerberos realm)
    OrgUnit(OrgUnitEntry), // Unit/department within an organization
    System(SystemEntry), // System User (i.e. non-human account)
    Credential(CredentialEntry), // Encrypted access credential
}

impl Object {
    #[inline]
    pub fn class(&self) -> Class {
        match *self {
            Object::Root(_) => Class::Root,
            Object::Domain(_) => Class::Domain,
            Object::OrgUnit(_) => Class::OrgUnit,
            Object::System(_) => Class::System,
            Object::Credential(_) => Class::Credential,
        }
    }

    // TODO: use TryFrom
    pub fn from_entry(entry: &Entry) -> Result<Object> {
        let result = match entry.class {
            Class::Root => Object::Root(try!(RootEntry::from_proto(entry.data))),
            Class::Domain => Object::Domain(try!(DomainEntry::from_proto(entry.data))),
            Class::OrgUnit => Object::OrgUnit(try!(OrgUnitEntry::from_proto(entry.data))),
            Class::System => Object::System(try!(SystemEntry::from_proto(entry.data))),
            Class::Credential => Object::Credential(try!(CredentialEntry::from_proto(entry.data))),
        };

        Ok(result)
    }

    pub fn find<'a, A>(adapter: &'a A, path: &Path) -> Result<Object>
        where A: Adapter<'a>
    {
        let txn = try!(adapter.ro_transaction());
        let direntry = try!(adapter.find_direntry(&txn, path));
        let entry = try!(adapter.find_entry(&txn, &direntry.id));

        Self::from_entry(&entry)
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert_object(self.class().to_string(), |b| match *self {
            Object::Root(ref root) => root.build_json(b),
            Object::Domain(ref domain) => domain.build_json(b),
            Object::OrgUnit(ref ou) => ou.build_json(b),
            Object::System(ref system) => system.build_json(b),
            Object::Credential(ref credential) => credential.build_json(b),
        })
    }

    pub fn to_proto(&self) -> Result<Vec<u8>> {
        match *self {
            Object::Root(ref root) => root.to_proto(),
            Object::Domain(ref domain) => domain.to_proto(),
            Object::OrgUnit(ref ou) => ou.to_proto(),
            Object::System(ref system) => system.to_proto(),
            Object::Credential(ref credential) => credential.to_proto(),
        }
    }
}

impl Serialize for Object {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        let object_proto = self.to_proto();

        if !object_proto.is_ok() {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
                                      format!("couldn't serialize {:?}", self.class())));
        }

        try!(out.write(self.class() as u32, &object_proto.unwrap()));

        Ok(())
    }
}

impl ObjectHash for Object {
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        match *self {
            Object::Root(ref root) => root.objecthash(hasher),
            Object::Domain(ref domain) => domain.objecthash(hasher),
            Object::OrgUnit(ref ou) => ou.objecthash(hasher),
            Object::System(ref system) => system.objecthash(hasher),
            Object::Credential(ref credential) => credential.objecthash(hasher),
        }
    }
}
