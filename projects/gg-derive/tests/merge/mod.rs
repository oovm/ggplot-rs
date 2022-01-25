use ggplot_derive::Merge;


#[derive(Clone, Debug, Merge, Default)]
pub struct TestA {
    test1: Option<String>,
    test2: Option<usize>,
}

#[test]
fn getter_setter() {
    let a = TestA::default();
    assert_eq!(a.get_test1(), "");
    assert_eq!(a.get_test2(), 0);
    let mut b = TestA::default()//
        .with_test1("a")
        .with_test1("b");
    assert_eq!(b.get_test1(), "b");
    b.set_test1("c");
    assert_eq!(b.get_test1(), "c");
}

#[test]
fn adder() {

}