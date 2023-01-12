```mermaid
classDiagram
	direction LR

	Error..>Display

	class Error

	class Display {
		func fmt(self, f: mut Formatter) Result<(), Error>
	}
```
