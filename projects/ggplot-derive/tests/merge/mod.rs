use ggplot_derive::Merge;


#[derive(Clone, Debug, Merge, Default)]
pub struct K {
    test: Option<String>
}

#[test]
fn test() {
    let a = K::default();
    println!("{}", a.get_test())
}