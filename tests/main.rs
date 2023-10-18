use copypasta::{ClipboardContext, ClipboardProvider};

#[test]
fn it_works() {
    let mut ctx = ClipboardContext::new().unwrap();
    let msg = "Hello, world!";

    ctx.set_contents(msg.to_owned()).unwrap();
    assert_eq!(ctx.get_contents().unwrap(), "Hello, world!");
}
