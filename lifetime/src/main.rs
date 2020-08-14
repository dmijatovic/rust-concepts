

fn make_string(n:i32)->String{
    // "hello"
    let s = format!("Hello {}",n);
    return s;
}

fn str_part(s: &str)->&str{
    if s.len() > 4{
        &s[..4]
    }else{
        s
    }
}

fn str_part_with_life<'a>(s: &'a str)->&'a str{
    if s.len() > 4{
        &s[..4]
    }else{
        s
    }
}

fn main() {
    println!("Lifetime starts");
    println!("{}",make_string(7));
    println!("String part: {}", str_part("sefsdfsfs"));

    let mut s = make_string(24);
    let life = str_part_with_life(&s);

    {
        let p = str_part(&s);
        // here is ok to use p
        println!("{}",p);
    }

    println!("{}", life);
    //cannot manipulate string
    // as longs as it borrowed
    s.push_str(" adding");
    println!("{}", s);

}
