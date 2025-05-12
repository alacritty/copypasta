# copypasta

copypasta is a [rust-clipboard](https://github.com/aweinstock314/rust-clipboard) fork, adding support for the Wayland clipboard.

rust-clipboard is a cross-platform library for getting and setting the contents of the OS-level clipboard.  

## Example

```rust
extern crate copypasta;

use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();

    let msg = "Hello, world!";
    ctx.set_contents(msg.to_owned()).unwrap();

    let content = ctx.get_contents().unwrap();

    println!("{}", content);
}
```

## API

The `ClipboardProvider` trait has the following functions:

```rust
fn get_contents(&mut self) -> Result<String, Box<Error>>;
fn set_contents(&mut self, String) -> Result<(), Box<Error>>;
```

`ClipboardContext` is a type alias for one of the following types implementing `ClipboardProvider`. Which concrete type is chosen for ClipboardContext depends on the OS(via conditional compilation).

- `WindowsClipboardContext`
- `AndroidClipboardContext`
- `OSXClipboardContext`
- `X11ClipboardContext`
- `NopClipboardContext`


## License

`rust-clipboard` is dual-licensed under MIT and Apache2.
