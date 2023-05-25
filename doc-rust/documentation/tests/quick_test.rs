/*
Test have to be write in tests/ folder anc can be launch with cargo test
Yo ucan launch all the test, specific test and more See cargo test options
for more details
*/

fn quick_test() {
    let age = 18;
    let is_major = matches!(age, 18..);
    assert!(is_major);
}
