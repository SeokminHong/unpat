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

    let tup = Test::SomeTuple(1, 3.5);
    unpat!(Test::SomeTuple(a, b) <- tup);
    assert_eq!((a, b), (1, 3.5));
}
