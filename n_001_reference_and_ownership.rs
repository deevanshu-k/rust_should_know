// Idea: create ide extension which generate the code explainantion comments and tell you what to learn from it

// Internally rust is calculating: return_lifetime = min(a_lifetime, b_lifetime)
fn longest<'a, 'b>(a: &'a str, b: &'b str) -> &'a str
where
    'b: 'a,
{
    if a.len() > b.len() {
        return a;
    };
    return b;
}

pub fn run() {
    let s1 = String::from("short");
    let s2 = String::from("very very long");
    let result;
    {
        let s1_str = s1.as_str();
        let s2_str = s2.as_str();
        result = longest(s1_str, s2_str);
    }
    println!("{}", result);
}
