extern crate hijacks_dashboard;

#[test]
fn hello() {
    hijacks_dashboard::backend::elastic::test();
}
