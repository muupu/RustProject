
#[derive(Clone, Debug)]
struct Developer {
    name: String,
    age: u8,
    lang: Language
}

#[derive(Clone, Debug)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell
}

pub trait MyFrom<T> {
    // anonymous parameters are removed in the 2018 edition (see RFC 1685)
    // fn myfrom(T) -> Self;  // 编译报错  ^ expected one of `:`, `@`, or `|`
    fn myfrom(_: T) -> Self; 
}

pub trait MyInto<T> {
    fn myinto(self) -> T;
}


impl<T, U> MyInto<U> for T where U: MyFrom<T>  {
    
    // T类型 转换成 U类型
    fn myinto(self) -> U {
        U::myfrom(self)
    }
}

// 为String类型实现 MyFrom<&str> trait，==》 自动实现 impl<&str, String> MyInto<String> for &str
impl MyFrom<&str> for String {
    fn myfrom(s: &str) -> Self {
        String::from(s)
    }
}


fn main() {
    println!("Hello, world!");
    let dev = Developer {
        name: "abcd".to_string(),
        age: 18,
        lang: Language::Rust
    };
    let dev1 = dev.clone();
    // name地址不一样 =》说明name字段的堆内存被clone了一份，因此Clone是深度拷贝，栈内存和堆内存都一起拷贝
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str());
    println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str()); 


    let s = String::from("hello world!");
    println!("{}", s);

    let s1 = String::myfrom("hello world111!");
    println!("{}", s1);
 
    // 编译报错：error[E0282]: type annotations needed
    // let s2 = "hello world222!".myinto(); // consider giving `s2` a type
    let s2: String = "hello world222!".myinto();
    println!("{}", s2);
    
}
