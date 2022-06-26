fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is '{}'", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
}

fn change(s: &mut String) {
    s.push_str(" world");
    println!("{}", *s);
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}
