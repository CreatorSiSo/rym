# Modules

```rust
mod something {
	pub(crate) fn welcome_msg() -> String {
		"Hello World :)"
	}

	pub(super) const startup_msg = |machine_name: String| "...Hello there" + machine_name + "..."

	// Forbidded, it makes no sense to mutate a value from outside of the module
	pub mut crash_msg = "...Nope..."
}

mod somewhere {
	use super::something::{welcome_msg, startup_msg}
	use crate::something::{welcome_msg, startup_msg}

	fn main() {
		const msg = startup_msg("pcbeepboop")
		const msg = welcome_msg()
	}
}

```
