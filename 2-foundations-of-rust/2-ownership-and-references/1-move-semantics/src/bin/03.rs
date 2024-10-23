//! Make me compile without adding new lines-- just changing existing lines!

fn main() {
    let mut s0 = String::new();

    let s1 = append_to_string(&mut s0);

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push('!');

    println!("{} == `{}`", stringify!(s1), s1);
}

fn append_to_string(s: &mut String) -> &mut String {
    s.push_str("Hello World");

    s
}
