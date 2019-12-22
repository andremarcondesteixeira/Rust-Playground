fn main() {
    let foo = get_string("foo", "bar");
    let bar = get_string("açdfklj", "dfdkfjaas");
    println!("{}", get_longest(foo.as_str(), bar.as_str()));
}

fn get_string(foo: &str, _bar: &str) -> String {
    let mut s = String::from(foo);
    s.push_str(_bar);
    s
}

fn get_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}