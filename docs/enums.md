```rym
const EventKind = type {
	Seminar,
	Party,
	Blocking,
}
```

```rym
/// The compile time only enum type
const fn EnumType(N: uint) Type => struct {
	repr: EnumRepr,
	kinds: [EnumKind; N],
}

impl EnumType {
    pub const fn from_kinds(..args: [EnumKind; _]) Self => {
        Self {
            repr: EnumRepr::optimal(kinds),
            kinds,
        }
    }

    pub const fn concat(self, other: EnumType) Self => {
        let kinds = self.kinds.extend(other.kinds);
        Self { repr: EnumRepr::optimal(kinds), kinds }
    }
}


const EnumKind = enum {
    Fieldless(Str),
    Tuple([(?Str, Type)]),
    Struct([(Str, Type)]),
}

const EnumRepr = enum {}

impl EnumRepr {
    pub const fn optimal(kinds: [EnumKind]) Self => {
        /* TODO */
    }
}
```
