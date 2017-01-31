use structure;

pub mod integer;
pub mod sequence;
pub mod octetstring;
pub mod boolean;
pub mod null;

// Reexport everything
pub use integer::Integer;
pub use sequence::{Sequence, SequenceOf, SetOf};
pub use octetstring::OctetString;
pub use boolean::Boolean;
pub use null::Null;

pub trait ASNTag {
    /// Encode yourself into a generic Tag format.
    /// 
    /// The only thing that changes between types is how to encode the value they wrap into bytes,
    /// however the encoding of the class and id does not change. By first converting the tag into
    /// a more generic tag (with already encoded payload), we don't have to reimplement the
    /// encoding step for class & id every time.
    fn into_structure(self) -> structure::StructureTag;
}

#[derive(Clone, Debug)]
/// This enum does not cover all ASN.1 Types but only the types needed for LDAPv3.
pub enum Tag {
    Integer(integer::Integer),
    Sequence(sequence::Sequence),
    OctetString(octetstring::OctetString),
    Boolean(boolean::Boolean),
    Null(null::Null),
    StructureTag(structure::StructureTag),
}

impl ASNTag for Tag {
    fn into_structure(self) -> structure::StructureTag {
        match self {
            Tag::Integer(i)      => i.into_structure(),
            Tag::Sequence(i)     => i.into_structure(),
            Tag::OctetString(i)  => i.into_structure(),
            Tag::Boolean(i)      => i.into_structure(),
            Tag::Null(i)         => i.into_structure(),
            Tag::StructureTag(s) => s
        }
    }
}