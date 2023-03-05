fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);    
    while m != 0 {        
        if m < n {            
            let t = m;            
            m = n;            
            n = t;        
            
        }        
        m = m % n;    
        
    }   
    n
    
}

#[test]
fn test_gcd() {    
    assert_eq!(gcd(14, 15), 1);   
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,       
        3 * 7 * 11 * 13 * 19),       
        3 * 11);
}


// fn find_pos(data: Vec<u32>, v: u32) -> Option<usize> {
//     for (pos, item) in data.iter().enumerate() {
//         if *item == v {
//             return Some(pos);
//         }
//     }
    
//     None
// }

// 所有静态变量都必须初始化。
// static mut STASH: &i32 = &128; 
// static STASH_A: i32 = 127; 
// fn fun_ref(p: &'static i32) {
//     unsafe {
//         STASH = p;
//     }
// }

// fn g<'a>(p: &'a i32) {
//     println!("p: {}", p);
// }

// 返回一个切片中最小元素的引用
// fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
//     let mut s = &v[0];
//     for r in &v[1..] {
//         if *r < *s { s = r; }
//     }
//     s
// }


fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

#[derive(Clone, Debug)]
struct Developer { 
    name: String, 
    age: u8, 
    lang: Language
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Language { 
    Rust, 
    TypeScript, 
    Elixir, 
    Haskell
}


fn main() {
    // let cat = Cat;
    // println!("cat: {}", name(cat));

    // let data = vec![10, 42, 9, 8];
    // let v = 42;
    // if let Some(pos) = find_pos(data, v) {
    //     println!("Found {} at {}", v, pos);
    // }

    // 5.2.2 Receiving References as Function Arguments
    // // staic mut的静态变量要用unsafe包起来：use of mutable static is unsafe and requires unsafe function or block
    // // mutable statics can be mutated by multiple threads: aliasing violations or data races will cause 
    // unsafe {
    //     println!("STASH: {}", STASH);
    // }   
    // fun_ref(&STASH_A);
    // unsafe {
    //     println!("STASH: {}", STASH);
    // }
    
    // 5.2.3 将引用作为参数传递
    // let x = 10;
    // g(&x);

    // 5.2.4 返回引用
    // let s;
    // {
    //     let parabola = [9, 4, 1, 0, 1, 4, 9];
    //     s = smallest(&parabola);  // 编译报错 `parabola` does not live long enough。参数和返回值必须具有相同的生命期。
           // assert_eq!(*s, 0);  // 放在这里就不会编译报错，s的生命周期将达到这里，和parabola的生命期一致
    // }
    // assert_eq!(*s, 0); // 这个语句，让s的生命期扩展到这里

    // 5.2.5  结构体包含引用
    // struct S<'a> {
    //     r: &'a i32 // 结构体包含引用，该引用必须写出生命期
    // }
    // let s;
    // {
    //     let x = 10;
    //     s = S { r: &x}; // 这里&x赋值给r，会把‘a限制在x的生命期内，这样S的生命期也是’a，也就会限制在&x的生命期
    // }
    //assert_eq!(*s.r, 10);// // 这个语句，让s的生命期扩展到这里。导致s的生命期超过了，其引用的S生命期‘a

    // 5.3 共享与修改
    // let v = vec![4, 8, 19 ,27, 34, 10];
    // let r = &v;
    // let aside = v;// cannot move out of `v` because it is borrowed
    // r[0];
    // let mut wave = Vec::new();
    // let head = vec![0.0, 1.0];

    let dev = Developer { name: "abcd".to_string(), age: 18, lang: Language::Rust }; 
    let dev1 = dev.clone(); 
    println!("dev: {:?}, addr of dev name: {:p}", dev, dev.name.as_str()); 
    println!("dev1: {:?}, addr of dev1 name: {:p}", dev1, dev1.name.as_str())

}