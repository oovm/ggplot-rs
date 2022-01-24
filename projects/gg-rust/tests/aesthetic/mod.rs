use ggrust::GGLineStyle;

#[test]
fn test() {
    let s: GGLineStyle = "5".parse().unwrap();

    for j in s.into_iter().take(7) {
        println!("{}", j)
    }
}
