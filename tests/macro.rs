use unpat::unpat;

#[allow(dead_code)]
enum Test {
    SomeInt(i32),
    SomeTuple(i32, f64),
}

#[test]
fn it_works() {
    let int = Test::SomeInt(1);
    unpat!(Test::SomeInt(v) <- int);
    assert_eq!(v, 1);
}
