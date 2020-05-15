use copypasta::{ClipboardContext, ClipboardProvider};

fn some_other_fn() {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.get_contents().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents("Dummy".into()).unwrap();
        ctx.get_contents().unwrap();

        some_other_fn();
    }

    #[test]
    fn bar() {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents("Dummy".into()).unwrap();
        ctx.get_contents().unwrap();

        some_other_fn();
    }
}
