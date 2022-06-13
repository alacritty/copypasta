use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let msg = "Hello, world!";
    ctx.set_contents(msg.to_owned()).unwrap();

    let content = ctx.get_contents().unwrap();

    println!("{}", content);
}
