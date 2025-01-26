use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type<'a> {
    Unit,
    Never,
    Type,
    Uint(u16),
    Int(u16),
    Array { element: &'a Type<'a>, len: usize },
    Slice { element: &'a Type<'a> },
    Enum { variants: &'a [Type<'a>] },
    Aggregate { fields: &'a [Type<'a>] },
    Function(&'a Function<'a>),
}

impl Type<'_> {
    pub fn layout(&self) -> Layout {
        match self {
            Type::Unit => Layout::Empty,
            Type::Never => Layout::Empty,
            Type::Type => todo!(),
            Type::Uint(bits) | Type::Int(bits) => Layout::Int(layout_int(*bits as usize)),
            Type::Array { element, len } => Layout::Array(layout_array(element, *len)),
            Type::Slice { .. } => Layout::FatPointer(layout_fat_pointer()),
            Type::Enum { variants } => Layout::Enum(layout_enum(variants)),
            Type::Aggregate { fields } => Layout::Aggregate(layout_aggregate(fields)),
            Type::Function(_) => Layout::Pointer(layout_pointer()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Function<'a> {
    pub params: &'a [Type<'a>],
    pub result: Type<'a>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Layout {
    Empty,
    Int(IntLayout),
    Pointer(PointerLayout),
    FatPointer(FatPointerLayout),
    Array(ArrayLayout),
    Enum(EnumLayout),
    Aggregate(AggregateLayout),
}

impl Layout {
    // Size of the data type in bytes
    pub fn size(&self) -> usize {
        match self {
            Layout::Empty => 0,
            Layout::Int(layout) => layout.size,
            Layout::Pointer(layout) => layout.size,
            Layout::FatPointer(layout) => layout.size,
            Layout::Array(layout) => layout.element_size * layout.len,
            Layout::Enum(layout) => layout.tag_size + layout.payload_size,
            Layout::Aggregate(layout) => layout.size,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IntLayout {
    bits: usize,
    size: usize,
}

fn layout_int(bits: usize) -> IntLayout {
    IntLayout {
        bits,
        size: bits.next_multiple_of(8) / 8,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayLayout {
    element_size: usize,
    len: usize,
}

fn layout_array(element: &Type, len: usize) -> ArrayLayout {
    ArrayLayout {
        element_size: element.layout().size(),
        len,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumLayout {
    pub tag_offset: usize,
    pub tag_size: usize,
    pub payload_offset: usize,
    pub payload_size: usize,
}

fn layout_enum(variants: &[Type]) -> EnumLayout {
    let tag_offset = 0;
    let tag_bits = enum_tag_bits(variants.len());
    let tag_size = pad_to_power_of_two(tag_bits.next_multiple_of(8) / 8);
    if tag_size > 8 {
        panic!("Enum tag too large!");
    }

    EnumLayout {
        tag_offset,
        tag_size,
        payload_offset: tag_size,
        payload_size: variants
            .iter()
            .map(|typ| typ.layout().size())
            .max()
            .unwrap_or(0),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AggregateLayout {
    pub offsets: Box<[usize]>,
    pub size: usize,
}

fn layout_aggregate(fields: &[Type]) -> AggregateLayout {
    let sizes = Box::from_iter((0..fields.len()).map(|i| fields[i].layout().size()));
    let mut mapping = Box::from_iter(0..fields.len());

    mapping.sort_by(|l, r| {
        let size_l = sizes[*l];
        let size_r = sizes[*r];

        if size_l == 0 {
            return Ordering::Less;
        }

        let cmp_result = size_l.is_power_of_two().cmp(&size_r.is_power_of_two());
        if cmp_result != Ordering::Equal {
            return cmp_result;
        }

        size_r.cmp(&size_l)
    });

    let mut offsets = mapping.clone();
    let mut current_offset = 0;

    for original_index in mapping {
        offsets[original_index] = current_offset;
        current_offset += sizes[original_index];
    }

    AggregateLayout {
        offsets,
        size: current_offset,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FatPointerLayout {
    pub size: usize,
}

const fn layout_fat_pointer() -> FatPointerLayout {
    FatPointerLayout { size: 8 + 8 }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PointerLayout {
    pub size: usize,
}

const fn layout_pointer() -> PointerLayout {
    PointerLayout { size: 8 }
}

#[test]
fn test_layout_enums() {
    assert_eq!(
        layout_enum(&[]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 0,
            payload_offset: 0,
            payload_size: 0,
        }
    );
    assert_eq!(
        layout_enum(&[
            Type::Unit,
            Type::Aggregate { fields: &[] },
            Type::Aggregate { fields: &[] }
        ]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 1,
            payload_offset: 1,
            payload_size: 0,
        }
    );
    assert_eq!(
        layout_enum(&[Type::Uint(16)]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 0,
            payload_offset: 0,
            payload_size: 2,
        }
    );
    assert_eq!(
        layout_enum(&[Type::Uint(16), Type::Uint(24), Type::Uint(25)]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 1,
            payload_offset: 1,
            payload_size: 4,
        }
    );
    assert_eq!(
        layout_enum(&[
            Type::Enum {
                variants: &[Type::Uint(16)]
            },
            Type::Enum {
                variants: &[Type::Uint(16), Type::Uint(24), Type::Uint(25)]
            }
        ]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 1,
            payload_offset: 1,
            payload_size: 5,
        }
    );
}

#[test]
fn test_layout_fields() {
    let construct = |offsets: &[usize], bits| AggregateLayout {
        offsets: Box::from(offsets),
        size: bits,
    };
    assert_eq!(
        layout_aggregate(&[
            Type::Unit,
            Type::Aggregate { fields: &[] },
            Type::Aggregate { fields: &[] }
        ]),
        construct(&[0, 0, 0], 0)
    );
    assert_eq!(
        layout_aggregate(&[Type::Unit, Type::Uint(8), Type::Uint(16)]),
        construct(&[0, 2, 0], 3)
    );
    assert_eq!(
        layout_aggregate(&[Type::Uint(16), Type::Unit, Type::Uint(8)]),
        construct(&[0, 0, 2], 3)
    );
    assert_eq!(
        layout_aggregate(&[Type::Uint(15), Type::Unit, Type::Uint(8)]),
        construct(&[0, 0, 2], 3)
    );
    assert_eq!(
        layout_aggregate(&[Type::Uint(15), Type::Uint(8), Type::Uint(3), Type::Uint(87)]),
        construct(&[11, 13, 14, 0], 15)
    );
}

const fn enum_tag_bits(variants: usize) -> usize {
    if variants == 0 || variants == 1 {
        0
    } else if variants == usize::MAX {
        usize::BITS as usize
    } else {
        variants.next_power_of_two().trailing_zeros() as usize
    }
}

const fn pad_to_power_of_two(size: usize) -> usize {
    if size == 0 {
        0
    } else {
        size.next_power_of_two()
    }
}

#[test]
fn enum_tag() {
    assert_eq!(enum_tag_bits(0), 0);
    assert_eq!(enum_tag_bits(1), 0);
    assert_eq!(enum_tag_bits(2), 1);
    assert_eq!(enum_tag_bits(3), 2);
    assert_eq!(enum_tag_bits(4), 2);
    assert_eq!(enum_tag_bits(5), 3);
    assert_eq!(enum_tag_bits(8), 3);
    assert_eq!(enum_tag_bits(9), 4);
    assert_eq!(enum_tag_bits(16), 4);
    assert_eq!(enum_tag_bits(256), 8);
    // TODO assuming 64 bit target for now
    assert_eq!(enum_tag_bits(usize::MAX), 64);
}

#[test]
fn padding() {
    assert_eq!(pad_to_power_of_two(0), 0);
    assert_eq!(pad_to_power_of_two(1), 1);
    assert_eq!(pad_to_power_of_two(2), 2);
    assert_eq!(pad_to_power_of_two(3), 4);
    assert_eq!(pad_to_power_of_two(4), 4);
    assert_eq!(pad_to_power_of_two(5), 8);

    assert_eq!(usize::next_multiple_of(0, 8), 0);
    assert_eq!(usize::next_multiple_of(1, 8), 8);
    assert_eq!(usize::next_multiple_of(2, 8), 8);
    assert_eq!(usize::next_multiple_of(5, 8), 8);
    assert_eq!(usize::next_multiple_of(8, 8), 8);
    assert_eq!(usize::next_multiple_of(9, 8), 16);
    assert_eq!(usize::next_multiple_of(10, 8), 16);
    assert_eq!(usize::next_multiple_of(16, 8), 16);
    assert_eq!(usize::next_multiple_of(25, 8), 32);
    assert_eq!(usize::next_multiple_of(59, 8), 64);
}
