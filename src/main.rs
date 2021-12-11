extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;
use std::fmt::Formatter;

fn main() {
    // hello();
    // formatted_print();
    // debug_4_custom_types();
    // display_4_custom_types();
    // TODO formatting();

    // primitives_literals();
    
    // if_else();
    // if_else_return_value();
    
    // loop_();
    // TODO loop_nesting_labels();
    // loop_return_value();
    
    // while_();
    
    // for_range();
    
    // for_iter();
    // for_into_iter();
    // for_iter_mut();
    
    // match_();
    // match_destructuring_tuples();
    // match_destructuring_enums();
    // TODO match_destructuring_pointers();
    // match_destructuring_structs();
    
    // match_guards();
    
    // TODO match_binding();
    
}

/// 死循环, 打印1-5, 跳过3, 在5停止
#[allow(dead_code)]
fn loop_() {
    let mut cnt = 0;
    loop {
        if cnt == 5 {
            println!("stop at No.5");
            break;
        }
        
        cnt += 1;
        if cnt == 3 {
            println!("skip No.3");
            continue;
        }
        
        println!("cur No.{}", cnt);
    }
    // Output:
    // cur No.1
    // cur No.2
    // skip No.3
    // cur No.3
    // cur No.4
    // stop at No.5
}

/// Rust中没有类似 for i=0; i < 10; i++ 的for循环结构, 可以使用while来替代
#[allow(dead_code)]
fn while_() {
    let mut cnt = 0;
    while cnt < 10 {
        cnt += 1;
    }
    println!("cnt: {}", cnt);
    // Output: 
    // cnt: 10
}

/// 可以使用match实现卫语句的效果, 对 1)入参 或 2)结构体的状态 进行校验
#[allow(dead_code)]
fn match_guards() {
    let pair = (2, -2);
    match pair {
        (foo, bar) if foo == bar => println!("foo equals bar"),
        (foo, bar) if foo > bar => println!("foo great then bar"),
        _ => println!("no match")
    }
    // Output:
    // foo great then bar
}

#[allow(dead_code)]
fn match_destructuring_structs() {
    struct Container {
        widthx: (u32, u32),
        height: u32,
    }

    // Try changing the values in the struct to see what happens
    let ctnr = Container { widthx: (10, 2), height: 2 };

    match ctnr {
        Container { widthx: (1, b), height: y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Container { height: 2, widthx: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Container { height: y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Container { y } => println!("y = {}", y),
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
fn match_destructuring_enums() {
    // let color = Color::Red;
    let color = Color::HSL(122, 17, 40);
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                     c, m, y, k),
    }
    // Output: 
    // Hue: 122, saturation: 17, lightness: 40!
}

/// 使用match来匹配tuples
#[allow(dead_code)]
fn match_destructuring_tuples() {
    let triple = (1, -2, 3);
    let result = match triple {
        // 一共包括三个元素, 第一个元素为0, 其余两个元素匹配任意值
        (0, _foo, _bar) => "first",
        // `..` can be used to ignore the rest of the tuple
        // 这里元素个数如果超过前一个规则(3个), 就会报错: expected a tuple with 4 elements, found one with 3 elements
        (1, ..)  => "second",
        // `_` means don't bind the value to a variable
        _      => "none",
    };
    println!("result: {}", result)
    // Output:
    // result: second
}

/// match支持返回值, Rust的if/else也支持返回值
#[allow(dead_code)]
fn match_() {
    let number = 13;
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    // Output: 
    // A teen
    
    let user_id = 1000;
    let user_name = match user_id {
        1000 => "luca",
        _ => "unknown"
    };
    println!("UserId {1} => UserName {0}", user_name, user_id)
    // Output: 
    // UserId 1000 => UserName luca
}

/// 修改迭代中的元素
#[allow(dead_code)]
fn for_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = 
            if *name == "Bob" {
            "There is a rustacean among us!"
            } else {
                *name
            };
    }

    println!("names: {:?}", names);
}

/// 集合中的元素在迭代的过程中会被消费掉
#[allow(dead_code)]
fn for_into_iter() {
    let names = vec!["James, Curry, Love"];
    println!("players: {:?}", names);
    for name in names.into_iter() {
        println!("consume player: {}", name)
    }
}

/// 迭代集合中的元素
#[allow(dead_code)]
fn for_iter() {
    let names = vec!["Allen", "T-Mac", "Curry"];
    println!("players: {:?}", names);
    // Output:
    // players: ["Allen", "T-Mac", "Curry"]
    
    for name in names.iter() {
        println!("player: {}", name)
    }
    // Output:
    // player: Allen
    // player: T-Mac
    // player: Curry
}

/// 有返回值的死循环 满足条件是break出来, 并携带返回值
#[allow(dead_code)]
fn loop_return_value() {
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 10 { break cnt * 3 }
    };
    print!("result: {}", result)
}

#[allow(dead_code)]
fn for_range() {
    for i in 1..10 {
        print!("{} ", i)
    }
    print!("\n");
    // Output:
    // 1 2 3 4 5 6 7 8 9
    
    for i in 1..=10 {
        print!("{} ", i)
    }
    // Output:
    // 1 2 3 4 5 6 7 8 9 10
}

/// 有返回值的if-else
#[allow(dead_code)]
fn if_else_return_value() {
    // bad
    // let page = 1;
    // let mut content = "hello";
    // if page == 0 {
    //     content = "not found";
    // } else if page < 0 {
    //     content = "error !!!";
    // }
    // print!("{}", content)
    
    // good
    let page = 2;
    let content =
        if page == 0 {
            // "not found"; // Error
            "not found"
        } else if page < 0 {
            // 234 // Tips: 所有分支的返回值类型都应一致
            "error"
        } else {
            "hello"
        };
    print!("{}", content)
}

#[allow(dead_code)]
fn if_else() {
    let id = 1000;
    if id == 1000 {
        println!("Greetings! Luca")
    } else if id == 1001 { 
        println!("Hey Allen")
    } else {
        println!("Unknown");
    }
    // Output:
    // Greetings! Luca
}

#[allow(dead_code)]
fn primitives_literals() {
    // i32表示一个有符号的 32-bit 整型
    // 1i32表示1是一个有符号的 32-bit 整型
    println!("1i32: {}", 1i32);
    // Output:
    // 1i32: 1

    // u32表示一个无符号的 32-bit 整型
    println!("1u32: {}", 1u32);
    // Output:
    // 1u32: 1

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    // Output:
    // 1 + 2 = 3
    // 1 - 2 = -1
    
    println!("true AND false is {{ {} }}", true && false);
    println!("true OR false is {{ {} }}", true || false);
    println!("NOT true is {{ {} }}", !true);
    // Output:
    // true AND false is { false }
    // true OR false is { true }
    // NOT true is { false }

    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                      -1i8, -2i16, -3i32, -4i64, 
                      0.1f32, 0.2f64, 'a', true);
    println!("{:?}", long_tuple)
    // Output:
    // (1, 2, 3, 4, -1, -2, -3, -4, 0.1, 0.2, 'a', true)
}

struct ArrayList(Vec<i32>);

impl fmt::Display for ArrayList {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ArrayList[");

        for (i, v) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?; // TODO
            }
            write!(f, "{}", v)?;
        }

        // 没有;结尾, 这个是返回值... 注意了
        write!(f, "]") 
    }
}

/// 使用 {} 方式来输出自定义类型时, 要求自定义类型实现 fmt::Display
/// 使用 fmt::Display 来控制自定义类型的输出行为
#[allow(dead_code)]
fn display_4_custom_types() {
    println!("{}", SessionId(1000));
    // Output:
    // session id is 1000
    
    let luca = Person{name: "luca", age: 22};
    println!("{}", luca);
    // Output:
    // hey! My name is luca, 22 year old. 
    
    let lst = ArrayList(vec![1, 2, 3, 4 , 5]);
    println!("{}", lst);
    // Output:
    // ArrayList[1, 2, 3, 4, 5]
    
    // let names = ArrayList(vec!["Luca", "Allen", "James"]);
    // Error:
    // expected `i32`, found `&str`
}

#[derive(Debug)]
struct SessionId(i32);

impl fmt::Display for SessionId { 
    // SessionId impl fmt::Display trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {              
        write!(f, "session id is {}", self.0)
    }
}

#[derive(Debug)]
struct HttpRequest(SessionId);

#[derive(Debug)]
struct Person<'a> { // TODO WTF <'a> is ?
    name: &'a str,
    age: u8
}

impl fmt::Display for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // unimplemented!()
        write!(f, "hey! My name is {name}, {age} year old. ", age = self.age, name = self.name)
    }
}

/// {:?}
/// 自定义类型需要声明宏 `#[derive(Debug)]` 否则无法使用 {:?} 的方式进行输出
#[allow(dead_code)]
fn debug_4_custom_types() {
    println!("Current session id is {:?}", SessionId(1000));
    // Output:
    // Current session id is SessionId(1000)
    println!("Current request: {:?}", HttpRequest(SessionId(1000)));
    // Output:
    // Current request: HttpRequest(SessionId(1000))
    
    let luca = Person{name: "luca", age: 12};
    println!("greetings! {:?}", luca);
    println!("greetings! {:?}", luca.name);
    // Output:
    // greetings! Person { name: "luca", age: 12 }
    // greetings! "luca"
    
    println!("greetings! petty {:#?}", luca);
    println!("greetings! petty {:#?}", luca.name);
    // Output:
    // greetings! petty Person {
    //     name: "luca",
    //     age: 12,
    // }
    // greetings! petty "luca"
}

#[allow(dead_code)]
fn formatted_print() {
    // stdout 打印到标准输出
    print!("OK");
    println!("OK with ln");

    // stderr 打印到标准错误
    eprint!("Error");
    eprintln!("Error with ln");
    
    println!("My name is {}, id number is {}", "Luca", 7777);
    // Output: 
    // My name is Luca, id number is 7777
    
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // Output: 
    // My name is Bond, James Bond

    // 命名参数
    println!("{subject} {verb} {object}.", object = "a asshole", subject = "Deadpool", verb = "is");
    // Output:
    // Deadpool is a asshole.
    
    // 使用:前缀表示特殊格式化
    // :b转为二进制 
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    // Output:
    // 1 of 10 people know binary, the other half doesn't

    // TODO 
    // println!("{number:>width$}", number=1, width=6);
    // println!("{number:0>width$}", number=1, width=6);
}

#[allow(dead_code)]
fn hello() {
    let stdout = stdout();
    let msg = String::from("Hello Rust!");
    let width = msg.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(msg.as_bytes(), width, &mut writer).unwrap();
}
