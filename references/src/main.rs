fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    main_2();

    main_3();

    main_4();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main_2() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


fn main_3() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}



fn incr(n: &mut i32) {
    println!("{n}")
}
fn main_4() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
}
