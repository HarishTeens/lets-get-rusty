use std::io;

fn mul5(inp: i32) -> i32 {
    inp * 5
}

fn main2() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed");

    let value: i32 = input.trim().parse().expect("not a number");

    let ans = String::new() + if value % 3 == 0 {"foo"} else {""} + if value % 5 == 0 {"bar"}  else {""};

    println!("{}",ans);
}

fn main3 () {
    let mut i=0;    
    let ans = 'parent_loop: loop {
        let mut j=0;
        loop {
            if (i==3) & (j==3) {                
                break 'parent_loop i*10+j;
            } else if j == 9 {
                break;
            }
            j+= 1;
        }
        i+=1;
    };
    println!("{}",ans);
}

fn main4 () {
    let mut number = 0;
    let a = [1,2,3,4,5,6];
    for x in (1..=6).rev() {
        println!("{}",x);
    }
}

fn main5() {
    let mut s1 = String::from("Hi mom!");

    let len = calculate_length(&mut s1);

    println!("The length of {} is {}",s1,len);

}

fn calculate_length (s: &mut String) -> usize {
    s.push_str("aaa");
    s.len()
}

fn main6() {
    let mut s1 = String::from("Hi mom!");

    let r1 = &s1;
    let r2 = &s1;
    
    println!("{} {}",r1,r2);

    let r3 = &mut s1;
}

fn first_word(s: &str) -> &str {
    for (i, &cha) in s.as_bytes().iter().enumerate() {
        if cha == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main7() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    
    assert_eq!(slice, &[3, 3]);
}

struct Pet {
    name: String,
    isCute: bool,
    paws: u64,
}

fn main8 () {
    let hope = create_dog(String::from("Hope"));

    println!("{}",hope.isCute);

    
    let y= Pet {
        name: hope.name,
        isCute: true,
        paws: hope.paws
    };
    println!("{}",hope.paws);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let array = [1,2,3,4,5];
    println!("{}",array[0]);

}

fn create_dog (name: String) -> Pet {
    Pet {
        name,
        isCute: true,
        paws: 4,
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
fn main9() {
    let rect1 = Rectangle {
        width: dbg!(3 * 30),
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40
    };

    let rect3 = Rectangle::square(49);

    println!("{:?}", rect3);

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "{:?} {} hold {:?} ",
        rect1,if rect1.can_hold(&rect2) { "can" } else { "cant"},rect2 
    );
}

fn area(dimesnsions: &Rectangle) -> u32 {
    dimesnsions.width * dimesnsions.height
}


fn main10 () {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
 
        }
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}", home);

    enum Option<T> {
        None,
        Some(T),
    }

    let some_number = Some(5);
    let some_string = Some("a string");

    // let absent_number: Option<i32> = None;
    
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {        
        Coin:: Dime => 10,
        Coin:: Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        },
        _ => 12
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main () {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    println!("{}",six.unwrap());

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        default => move_player(default),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}",max);
    }
}