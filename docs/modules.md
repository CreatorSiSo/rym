# Modules

```rym
module lib;

module something {
	pub(package) fn welcome_msg() String => {
		"Hello World :)"
	}

	pub(super) fn startup_msg(machine_name: String) String => {
        "...Hello there" + machine_name + "..."
    }

	// Forbidded, only constant values allowed as module items.
	pub let mut crash_msg = "...Nope...";
}

module somewhere {
	use super.something.{welcome_msg, startup_msg};
	use package.something.{welcome_msg, startup_msg};

	fn main() {
		const msg = startup_msg("pcbeepboop");
		const msg = welcome_msg();
	}
}
```
