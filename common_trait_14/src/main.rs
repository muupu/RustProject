
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
}
