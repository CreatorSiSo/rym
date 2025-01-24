#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Type<'a> {
    Unit,
    Unkown,
    Type,
    Uint(u16),
    Int(u16),
    Array { element: &'a Type<'a>, len: usize },
    Slice { element: &'a Type<'a> },
    Enum { variants: &'a [Type<'a>] },
    Struct { fields: &'a [Type<'a>] },
    Tuple { fields: &'a [Type<'a>] },
    Function {},
}

impl Type<'_> {
    pub fn layout(&self) -> Layout {
        match self {
            Type::Unit => Layout::Empty,
            Type::Unkown => unreachable!(),
            Type::Type => todo!(),
            Type::Uint(size) | Type::Int(size) => Layout::Int {
                size: *size as usize,
            },
            Type::Array { element, len } => Layout::Array {
                sequence: layout_sequence(element),
                len: *len,
            },
            Type::Slice { .. } => Layout::FatPointer(layout_fat_pointer()),
            Type::Enum { variants } => Layout::Enum(layout_enum(variants)),
            Type::Struct { fields } | Type::Tuple { fields } => {
                Layout::Fields(layout_fields(fields))
            }
            Type::Function {} => Layout::Pointer(layout_pointer()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Layout {
    Empty,
    Int {
        size: usize,
    },
    Pointer(PointerLayout),
    FatPointer(FatPointerLayout),
    Array {
        sequence: SequenceLayout,
        len: usize,
    },
    Enum(EnumLayout),
    Fields(FieldsLayout),
}

impl Layout {
    pub fn size(&self) -> usize {
        match self {
            Layout::Empty => 0,
            Layout::Int { size } => *size,
            Layout::Pointer(layout) => layout.size,
            Layout::FatPointer(layout) => layout.size,
            Layout::Array { sequence, len } => sequence.offset * len,
            Layout::Enum(layout) => layout.size,
            Layout::Fields(layout) => layout.size,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SequenceLayout {
    size: usize,
    offset: usize,
}

fn layout_sequence(element: &Type) -> SequenceLayout {
    let size = element.layout().size();
    SequenceLayout {
        size,
        offset: if size < 8 {
            pad_to_power_of_two(size)
        } else {
            size.next_multiple_of(8)
        },
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumLayout {
    pub tag_offset: usize,
    pub tag_size: usize,
    pub payload_offset: usize,
    pub payload_size: usize,
    pub size: usize,
}

fn layout_enum(variants: &[Type]) -> EnumLayout {
    let tag_offset = 0;
    let tag_size = enum_tag_size(variants.len()).next_multiple_of(8);
    let payload_offset = tag_size;
    let payload_size = variants
        .iter()
        .map(|t| t.layout().size())
        .max()
        .unwrap_or(0)
        .next_multiple_of(8);

    EnumLayout {
        tag_offset,
        tag_size,
        payload_offset,
        payload_size,
        size: tag_size + payload_size,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldsLayout {
    pub offsets: Box<[usize]>,
    pub size: usize,
}

fn layout_fields(fields: &[Type]) -> FieldsLayout {
    let sizes = Box::from_iter((0..fields.len()).map(|i| fields[i].layout().size()));
    let mut mapping = Box::from_iter(0..fields.len());

    mapping.sort_by(|l, r| {
        let size_l = sizes[*l];
        let size_r = sizes[*r];
        let aligned_l = size_l % 8 == 0;
        let aligned_r = size_r % 8 == 0;

        if !aligned_l && !aligned_r {
            size_r.cmp(&size_l)
        } else {
            aligned_r.cmp(&aligned_l)
        }
    });

    let mut offsets = mapping.clone();
    let mut current_offset = 0;

    for original_index in mapping {
        offsets[original_index] = current_offset;
        current_offset += sizes[original_index].next_multiple_of(8);
    }

    FieldsLayout {
        offsets,
        size: current_offset,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FatPointerLayout {
    pub pointer_offset: usize,
    pub pointer_size: usize,
    pub len_offset: usize,
    pub len_size: usize,
    pub size: usize,
}

const fn layout_fat_pointer() -> FatPointerLayout {
    FatPointerLayout {
        pointer_offset: 0,
        pointer_size: 64,
        len_offset: 64,
        len_size: 64,
        size: 64 + 64,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PointerLayout {
    pub size: usize,
}

const fn layout_pointer() -> PointerLayout {
    PointerLayout { size: 64 }
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
            size: 0
        }
    );
    assert_eq!(
        layout_enum(&[
            Type::Unit,
            Type::Tuple { fields: &[] },
            Type::Struct { fields: &[] }
        ]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 8,
            payload_offset: 8,
            payload_size: 0,
            size: 8
        }
    );
    assert_eq!(
        layout_enum(&[Type::Uint(16)]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 0,
            payload_offset: 0,
            payload_size: 16,
            size: 16
        }
    );
    assert_eq!(
        layout_enum(&[Type::Uint(16), Type::Uint(24), Type::Uint(25)]),
        EnumLayout {
            tag_offset: 0,
            tag_size: 8,
            payload_offset: 8,
            payload_size: 32,
            size: 40
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
            tag_size: 8,
            payload_offset: 8,
            payload_size: 40,
            size: 48
        }
    );
}

#[test]
fn test_layout_fields() {
    let construct = |offsets: &[usize], size| FieldsLayout {
        offsets: Box::from(offsets),
        size,
    };
    assert_eq!(
        layout_fields(&[
            Type::Unit,
            Type::Tuple { fields: &[] },
            Type::Struct { fields: &[] }
        ]),
        construct(&[0, 0, 0], 0)
    );
    assert_eq!(
        layout_fields(&[Type::Unit, Type::Uint(8), Type::Uint(16)]),
        construct(&[0, 0, 8], 24)
    );
    assert_eq!(
        layout_fields(&[Type::Uint(16), Type::Unit, Type::Uint(8)]),
        construct(&[0, 16, 16], 24)
    );
    assert_eq!(
        layout_fields(&[Type::Uint(15), Type::Unit, Type::Uint(8)]),
        construct(&[8, 0, 0], 24)
    );
    assert_eq!(
        layout_fields(&[Type::Uint(15), Type::Uint(8), Type::Uint(3), Type::Uint(87)]),
        construct(&[96, 0, 112, 8], 120)
    );
}

const fn enum_tag_size(variants: usize) -> usize {
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
    assert_eq!(enum_tag_size(0), 0);
    assert_eq!(enum_tag_size(1), 0);
    assert_eq!(enum_tag_size(2), 1);
    assert_eq!(enum_tag_size(3), 2);
    assert_eq!(enum_tag_size(4), 2);
    assert_eq!(enum_tag_size(5), 3);
    assert_eq!(enum_tag_size(8), 3);
    assert_eq!(enum_tag_size(9), 4);
    assert_eq!(enum_tag_size(16), 4);
    assert_eq!(enum_tag_size(256), 8);
    // TODO assuming 64 bit target for now
    assert_eq!(enum_tag_size(usize::MAX), 64);
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
