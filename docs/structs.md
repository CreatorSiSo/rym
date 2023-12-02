# Structs

## Examples

```rym
const Complex = struct {
	real: i32,
	complex: i32,
};

pub func default(Complex) Complex {
	Complex {
		real: 0,
		complex: 0,
	}
}

pub func i(Complex) Complex {
	Complex {
		real: 0,
		complex: 1,
	}
}

func main() {
}
```
