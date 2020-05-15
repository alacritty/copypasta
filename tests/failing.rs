use copypasta::{ClipboardContext, ClipboardProvider};

fn some_other_fn() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clear_clipboard() {
        let mut clipboard: ClipboardContext = ClipboardContext::new().unwrap();
        clipboard.set_contents("".into()).unwrap();
    }

    #[test]
    fn foo() {
        clear_clipboard();

        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents("Dummy".into()).unwrap();
        ctx.get_contents().unwrap();

        some_other_fn();
    }

    #[test]
    fn bar() {
        clear_clipboard();

        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents("Dummy".into()).unwrap();
        ctx.get_contents().unwrap();

        some_other_fn();
    }
}
