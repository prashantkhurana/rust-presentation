#[allow(dead_code)]
#[allow(unused)]
fn main() {
    let x = false;
    let s: (i32, bool) = (2, false);
    let gg : [i32;3] = [1, 3, 4];
    let a: &[i32] = &gg[0..1];
    println!("a array slice is {:?}", a);
    a.get(8);
    let s : Vec<i32> =  vec![1,2,3];
    test(x);


    let s = String::from("hello world");
    let hello: &str = &s[0..5];
    let world = &s[6..11];

    let a = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    let ii = 2;

    if ii < 2 {
        println!("less than 2");
    } else {
        println!(">= 2");
    }

    let iii = true;

    if iii {
        println!("= true");
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let y = if iii {
        5
    } else {
        6
    };

    println!("y is {}", y);




}

fn test(y: bool) -> i32 {
    println!("y is {}", y);
    return 2;
}