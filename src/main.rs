struct Buffer<T> {
    a: Vec<T>
}   

impl <T> Buffer<T> 
    where T: std::ops::Add<Output = T> + Copy + Default
{
    fn new() -> Buffer<T> {
        Buffer { a: Vec::new() }
    }
    fn sum(&self) -> T {
        let mut s = T::default();
        for i in self.a.iter() {
            s = s + *i;
        }
        s
    }
}

fn compare_string(a: &str, b: &str) -> bool {
    let len = if a.len() > b.len() { b.len() } else { a.len() };
    for i in 0..len {
        let a_char = a.chars().nth(i).unwrap();
        let b_char = b.chars().nth(i).unwrap();
        if a_char > b_char {
            return true;
        } else if a_char < b_char {
            return false;
        }
    }
    return a.len() > b.len();
}

fn add_vec(a:&Vec<char>) -> Vec<char> {
    let mut b = a.clone();
    for i in b.iter_mut() {
        *i = (*i as u8 + 1) as char;
    }
    b
}

fn main() {

    println!("Hello, world!");

    let mut b = Buffer::new();

    b.a.push(1);
    b.a.push(2);
    b.a.push(5);

    println!("sum: {}", b.sum());

    let s1 = "abc";
    let s2 = "abcd";
    println!("compare_string: {} {} , result: {}", s1, s2, compare_string(s1, s2));

    let s3 = "abd";
    let s4 = "abcd";
    println!("compare_string: {} {} , result: {}", s3, s4, compare_string(s3, s4));

    let vec1:Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let vec2 = add_vec(&vec1);

    println!("vec1: {:?}, vec2: {:?}", vec1, vec2);

}
