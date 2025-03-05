// Rust所有权系统示例
// 展示所有权规则、借用和生命周期

fn main() {
    println!("Rust所有权系统示例");
    println!("-------------------");
    
    // 1. 基本所有权规则
    println!("\n1. 基本所有权规则：");
    {
        let s1 = String::from("你好"); // s1进入作用域
        println!("  创建字符串s1: {}", s1);
        
        let s2 = s1; // s1的值被移动到s2，s1不再有效
        
        // 下面的代码会导致编译错误，因为s1已经被移动
        // println!("  尝试使用已移动的s1: {}", s1);
        
        println!("  s1移动到s2后，只能使用s2: {}", s2);
    } // s2离开作用域，内存被释放
    
    // 2. 克隆（深拷贝）
    println!("\n2. 克隆（深拷贝）：");
    {
        let s1 = String::from("世界");
        println!("  创建字符串s1: {}", s1);
        
        let s2 = s1.clone(); // 创建s1的深拷贝
        
        println!("  克隆后，s1和s2都可用: s1={}, s2={}", s1, s2);
    } // s1和s2离开作用域，内存被释放
    
    // 3. 栈上数据的复制
    println!("\n3. 栈上数据的复制：");
    {
        let x = 5;
        let y = x; // 整数是Copy类型，x的值被复制给y
        
        println!("  复制后，x和y都可用: x={}, y={}", x, y);
    }
    
    // 4. 函数参数与所有权
    println!("\n4. 函数参数与所有权：");
    {
        let s = String::from("参数");
        println!("  调用函数前，s: {}", s);
        
        takes_ownership(s); // s的值被移动到函数中
        
        // 下面的代码会导致编译错误，因为s已经被移动
        // println!("  调用函数后，s不再有效: {}", s);
        
        let x = 5;
        println!("  调用函数前，x: {}", x);
        
        makes_copy(x); // x的值被复制到函数中
        
        println!("  调用函数后，x仍然有效: {}", x);
    }
    
    // 5. 返回值与所有权
    println!("\n5. 返回值与所有权：");
    {
        let s1 = gives_ownership(); // gives_ownership将返回值移动给s1
        println!("  从函数获取所有权: {}", s1);
        
        let s2 = String::from("你好");
        println!("  创建字符串s2: {}", s2);
        
        let s3 = takes_and_gives_back(s2); // s2被移动到函数中，函数返回值被移动到s3
        println!("  s2移动到函数中，然后返回给s3: {}", s3);
        
        // 下面的代码会导致编译错误，因为s2已经被移动
        // println!("  s2不再有效: {}", s2);
    }
    
    // 6. 引用与借用
    println!("\n6. 引用与借用：");
    {
        let s1 = String::from("引用示例");
        println!("  创建字符串s1: {}", s1);
        
        let len = calculate_length(&s1); // 传递s1的引用，不转移所有权
        
        println!("  字符串\"{}\"的长度是: {} 个字节", s1, len);
        // s1仍然有效，因为我们只传递了引用
    }
    
    // 7. 可变引用
    println!("\n7. 可变引用：");
    {
        let mut s = String::from("你好");
        println!("  创建可变字符串s: {}", s);
        
        change(&mut s); // 传递s的可变引用
        
        println!("  修改后的s: {}", s);
        
        // 可变引用的限制：同一时间只能有一个可变引用
        // 下面的代码会导致编译错误
        /*
        let r1 = &mut s;
        let r2 = &mut s;
        println!("{}, {}", r1, r2);
        */
        
        // 不能同时拥有可变引用和不可变引用
        /*
        let r1 = &s;
        let r2 = &s;
        let r3 = &mut s;
        println!("{}, {}, {}", r1, r2, r3);
        */
    }
    
    // 8. 切片类型
    println!("\n8. 切片类型：");
    {
        let s = String::from("你好世界");
        println!("  创建字符串s: {}", s);
        
        let hello = &s[0..6]; // 注意：中文字符在UTF-8中占3个字节
        let world = &s[6..12];
        
        println!("  字符串切片: hello={}, world={}", hello, world);
    }
}

// 接收String参数的函数，获取所有权
fn takes_ownership(some_string: String) {
    println!("  函数takes_ownership接收了: {}", some_string);
} // some_string离开作用域，内存被释放

// 接收i32参数的函数，获取复制的值
fn makes_copy(some_integer: i32) {
    println!("  函数makes_copy接收了: {}", some_integer);
} // some_integer离开作用域，没有特殊操作

// 返回String的函数，转移所有权
fn gives_ownership() -> String {
    let some_string = String::from("来自函数的字符串");
    some_string // 返回some_string，所有权被移动出函数
}

// 接收并返回String的函数
fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回a_string，所有权被移动出函数
}

// 接收String引用的函数，不获取所有权
fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域，但不拥有引用的值，所以不会释放内存

// 接收可变String引用的函数，可以修改引用的值
fn change(s: &mut String) {
    s.push_str("，世界");
}
