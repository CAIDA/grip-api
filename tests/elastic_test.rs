extern crate hijacks_dashboard;

#[test]
fn hello() {
    let objects = hijacks_dashboard::backend::elastic::get_example_object().unwrap();
    for object in objects {
        println!("{:#}", object);
    }
}
