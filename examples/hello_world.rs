use copypasta::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let msg = "Hello, world!";
    ctx.set_contents(msg.to_owned()).unwrap();

    let found = ctx.get_contents().unwrap();

    println!("{}", &found);
}
