use buffoon::{Serialize, Deserialize, OutputStream, InputStream};
use object::{AllowsChild, Object};
use objecthash::{self, ObjectHash, ObjectHasher};
use proto::{ToProto, FromProto};
use serde_json::builder::ObjectBuilder;
use std::io;

#[derive(Debug, Eq, PartialEq)]
pub struct DomainEntry {
    pub description: Option<String>,
}

impl DomainEntry {
    pub fn new(description: Option<String>) -> DomainEntry {
        DomainEntry { description: description }
    }

    pub fn build_json(&self, builder: ObjectBuilder) -> ObjectBuilder {
        builder.insert("description", &self.description)
    }
}

impl AllowsChild for DomainEntry {
    #[inline]
    fn allows_child(child: &Object) -> bool {
        match *child {
            Object::Domain(_) |
            Object::OrgUnit(_) => true,
            _ => false,
        }
    }
}

impl Serialize for DomainEntry {
    fn serialize<O: OutputStream>(&self, out: &mut O) -> io::Result<()> {
        if let Some(ref description) = self.description {
            try!(out.write(1, &description))
        }

        Ok(())
    }
}

impl Deserialize for DomainEntry {
    fn deserialize<R: io::Read>(i: &mut InputStream<R>) -> io::Result<DomainEntry> {
        let mut description: Option<String> = None;

        while let Some(f) = try!(i.read_field()) {
            match f.tag() {
                1 => description = Some(try!(f.read())),
                _ => try!(f.skip()),
            }
        }

        Ok(DomainEntry { description: description })
    }
}

impl ToProto for DomainEntry {}
impl FromProto for DomainEntry {}

impl ObjectHash for DomainEntry {
    #[inline]
    fn objecthash<H: ObjectHasher>(&self, hasher: &mut H) {
        if let Some(ref desc) = self.description {
            objecthash_struct!(
                hasher,
                "description" => *desc
            )
        }
    }
}
