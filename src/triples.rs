use crate::containers::vbyte::read_vbyte;
use crate::containers::{AdjList, Bitmap, Sequence};
use crate::ControlInfo;
use crc_any::{CRCu32, CRCu8};
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::io;
use std::io::BufRead;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub enum TripleSect {
    Bitmap(TriplesBitmap),
}

impl TripleSect {
    pub fn read<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        use io::Error;
        use io::ErrorKind::InvalidData;
        let triples_ci = ControlInfo::read(reader)?;

        match &triples_ci.format[..] {
            "<http://purl.org/HDT/hdt#triplesBitmap>" => {
                Ok(TripleSect::Bitmap(TriplesBitmap::read(reader, triples_ci)?))
            }
            "<http://purl.org/HDT/hdt#triplesList>" => Err(Error::new(
                InvalidData,
                "Triples Lists are not supported yet.",
            )),
            _ => Err(Error::new(InvalidData, "Unknown triples listing format.")),
        }
    }

    pub fn read_all_ids(self) -> BTreeSet<TripleId> {
        match self {
            TripleSect::Bitmap(bitmap) => {
                let mut triple_ids = BTreeSet::new();

                for triple_id in bitmap.into_iter() {
                    triple_ids.insert(triple_id);
                }

                triple_ids
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    Unknown = 0,
    SPO = 1,
    SOP = 2,
    PSO = 3,
    POS = 4,
    OSP = 5,
    OPS = 6,
}

impl TryFrom<u32> for Order {
    type Error = std::io::Error;

    fn try_from(original: u32) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(Order::Unknown),
            1 => Ok(Order::SPO),
            2 => Ok(Order::SOP),
            3 => Ok(Order::PSO),
            4 => Ok(Order::POS),
            5 => Ok(Order::OSP),
            6 => Ok(Order::OPS),
            _ => Err(Self::Error::new(
                io::ErrorKind::InvalidData,
                "Unrecognized order",
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TriplesBitmap {
    order: Order,
    adjlist_y: AdjList,
    adjlist_z: AdjList,
}

impl TriplesBitmap {
    pub fn read<R: BufRead>(reader: &mut R, triples_ci: ControlInfo) -> io::Result<Self> {
        use std::io::Error;
        use std::io::ErrorKind::InvalidData;

        // read order
        let mut order: Order;
        if let Some(n) = triples_ci.get("order").and_then(|v| v.parse::<u32>().ok()) {
            order = Order::try_from(n)?;
        } else {
            return Err(Error::new(InvalidData, "Unrecognized order"));
        }

        // read bitmaps
        let bitmap_y = Bitmap::read(reader)?;
        let bitmap_z = Bitmap::read(reader)?;

        // read sequences
        let sequence_y = Sequence::read(reader)?;
        let sequence_z = Sequence::read(reader)?;

        // construct adjacency lists
        let adjlist_y = AdjList::new(sequence_y, bitmap_y);
        let adjlist_z = AdjList::new(sequence_z, bitmap_z);

        Ok(TriplesBitmap {
            order,
            adjlist_y,
            adjlist_z,
        })
    }
}

impl IntoIterator for TriplesBitmap {
    type Item = TripleId;
    type IntoIter = BitmapIter;

    fn into_iter(self) -> Self::IntoIter {
        BitmapIter::new(self)
    }
}

pub struct BitmapIter {
    // triples data
    triples: TriplesBitmap,

    // current position
    pos_y: usize,
    pos_z: usize,

    // next position
    next_y: usize,
    next_z: usize,
}

impl BitmapIter {
    pub fn new(triples: TriplesBitmap) -> Self {
        // let pos_z = 0;
        // let pos_y =
        // let next_y = pos_z;
        // let next_z = pos_y;
        unimplemented!();

        // BitmapIter {
        //     triples,
        //     pos_y,
        //     pos_z,
        //     next_y,
        //     next_z,
        // }
    }

    fn coord_to_triple(&self, x: usize, y: usize, z: usize) -> io::Result<TripleId> {
        use io::Error;
        use io::ErrorKind::InvalidData;

        match self.triples.order {
            Order::SPO => Ok(TripleId::new(x, y, z)),
            Order::SOP => Ok(TripleId::new(x, z, y)),
            Order::PSO => Ok(TripleId::new(y, x, z)),
            Order::POS => Ok(TripleId::new(y, z, x)),
            Order::OSP => Ok(TripleId::new(z, x, y)),
            Order::OPS => Ok(TripleId::new(z, y, x)),
            Order::Unknown => Err(Error::new(InvalidData, "unknown triples order")),
        }
    }
}

impl Iterator for BitmapIter {
    type Item = TripleId;

    fn next(&mut self) -> Option<Self::Item> {
        // Some(self.coord_to_triple(x, y, z).unwrap())
        unimplemented!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TripleId {
    subject_id: usize,
    predicate_id: usize,
    object_id: usize,
}

impl TripleId {
    pub fn new(subject_id: usize, predicate_id: usize, object_id: usize) -> Self {
        TripleId {
            subject_id,
            predicate_id,
            object_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ControlInfo, Dict, Header};
    use std::fs::File;
    use std::io::BufReader;

    // #[test]
    // fn read_triples() {
    //     let file = File::open("tests/resources/swdf.hdt").expect("error opening file");
    //     let mut reader = BufReader::new(file);
    //     ControlInfo::read(&mut reader).unwrap();
    //     Header::read(&mut reader).unwrap();
    //     Dict::read(&mut reader).unwrap();
    //     let triples = TripleSect::read(&mut reader).unwrap();

    //     panic!("{:?}", triples.read_all_ids());
    // }
}