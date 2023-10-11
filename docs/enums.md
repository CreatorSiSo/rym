```rym
const EventKind = enum {
	Seminar,
	Party,
	Blocking
}
```

```rym
/// The compile time only enum type
const Enum = struct {
	repr: EnumRepr,
	kinds: [EnumKind],
}

const func [Enum] from_kinds(...args: [EnumKind]) -> Self {
	Enum {
		repr: EnumRepr::optimal(kinds),
		kinds,
	}
}

const func [Enum] concat(self, other: Enum) -> Self {
	let kinds = self.kinds.extend(other.kinds);
	let repr = if self.repr == other.repr {
		self.repr
	} else {
		EnumRepr::optimal(kinds)
	};

	Enum { repr, kinds }
}

const EnumKind = enum {
	/* TODO */
}

const EnumRepr = enum {
	U8,
	U16,
	U32,
	U64,
}

const func [EnumRepr] optimal(kinds: [EnumKind]) -> Self {
	/* TODO */
}
```
