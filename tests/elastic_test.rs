extern crate hijacks_dashboard;

#[test]
fn hello() {
    let objects = hijacks_dashboard::backend::elastic::list_all_events().unwrap();
    for object in objects {
        println!("{:#}", object);
    }
}
