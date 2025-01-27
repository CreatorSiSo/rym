use std::cmp::Ordering;

const POINTER_SIZE: usize = size_of::<usize>();

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type<'a> {
    Unit,
    Never,
    Type,
    Uint(u16),
    Int(u16),
    Mut(&'a Type<'a>),
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
            Type::Mut(..) | Type::Function(..) => Layout::Pointer(layout_pointer()),
        }
    }

    pub fn pointer_offsets(&self) -> Box<[usize]> {
        fn is_pointer(layout: Layout) -> bool {
            matches!(layout, Layout::FatPointer(..) | Layout::Pointer(..))
        }
        let layout = self.layout();

        match (self, layout) {
            (_, Layout::Empty) | (_, Layout::Int(..)) => Box::new([]),
            (_, Layout::Pointer(layout)) => Box::new([layout.pointer_offset]),
            (_, Layout::FatPointer(layout)) => Box::new([layout.pointer_offset]),
            (Type::Array { element, len }, Layout::Array(..)) => {
                if is_pointer(element.layout()) {
                    Box::from_iter((0..*len).map(|i| i * element.layout().size()))
                } else {
                    Box::new([])
                }
            }
            (Type::Enum { .. }, Layout::Enum(_layout)) => todo!(),
            (Type::Aggregate { fields }, Layout::Aggregate(layout)) => Box::from_iter(
                fields
                    .iter()
                    .zip(layout.offsets.iter())
                    .filter_map(|(typ, offset)| is_pointer(typ.layout()).then_some(*offset)),
            ),
            _ => unreachable!(),
        }
    }

    /// Id used by the gc to follow pointers based on the data type
    pub fn gc_id(&self, custom: &mut u32) -> u32 {
        const LEAF: u32 = 0;
        const SLICE_OF_POINTERS: u32 = 1;
        const SLICE_OF_FAT_POINTERS: u32 = 2;

        match self {
            Type::Type => todo!(),
            Type::Unit | Type::Never | Type::Uint(_) | Type::Int(_) | Type::Function(_) => LEAF,

            Type::Mut(..) => {
                todo!()
            }

            Type::Array { element, .. } | Type::Slice { element } => {
                if false {
                    // TODO
                    SLICE_OF_POINTERS
                } else if matches!(element, Type::Slice { .. }) {
                    SLICE_OF_FAT_POINTERS
                } else {
                    LEAF
                }
            }

            Type::Enum { variants: types } | Type::Aggregate { fields: types } => {
                let has_gc_child = types.iter().any(|typ| matches!(typ, Type::Slice { .. }));
                if has_gc_child {
                    *custom += 1;
                    *custom
                } else {
                    LEAF
                }
            }
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
    if tag_size > size_of::<u64>() {
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
    pub pointer_offset: usize,
    pub len_offset: usize,
    pub size: usize,
}

const fn layout_fat_pointer() -> FatPointerLayout {
    FatPointerLayout {
        pointer_offset: 0,
        len_offset: POINTER_SIZE,
        size: POINTER_SIZE * 2,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PointerLayout {
    pub pointer_offset: usize,
    pub size: usize,
}

const fn layout_pointer() -> PointerLayout {
    PointerLayout {
        pointer_offset: 0,
        size: POINTER_SIZE,
    }
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
