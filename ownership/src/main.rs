fn main() {

    let s1 = gives_ownership();

    let len = calculate_length(&s1);


    // let s2 = String::from("Hello");

    // let s3 = takes_and_gives_back(s2);

    // takes_ownership(s);

    // let x = 5;

    // makes_copy(x);

    println!("This length of '{}' is {}", s1, len);
    
}

fn takes_ownership(some_string: String){
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
}


fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}