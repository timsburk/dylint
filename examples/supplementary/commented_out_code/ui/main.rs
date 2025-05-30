fn main() {}

fn foo(_: u32) {}

fn empty_with_line_comment() {
    // dbg!(x);
}

fn empty_with_block_comment() {
    /* dbg!(x); */
}

fn interspersed(x: u32) {
    // dbg!(x);

    let y = 0;

    /* dbg!(y); */

    foo(y);

    /*
        dbg!(x);
        dbg!(y);
    */
}

fn negative_tests() {
    // a line comment

    /* a block comment */

    /*
        a multiline
        block comment
    */

    /// doc_comment();
    foo(0);
}

fn single_identifier() {
    // smoelius: This is a "false positive." Ideally, the lint would not fire on the next line.
    // Identifier
}

// smoelius: @fcasal noticed that the lint produced multiple warnings in async functions.
async fn async_fn() {
    // dbg!(x);
}
