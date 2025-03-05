// Rust结构体和枚举示例
// 展示结构体、方法、枚举和模式匹配

// 定义一个结构体表示矩形
#[derive(Debug)] // 添加Debug特性，使结构体可以使用{:?}打印
struct Rectangle {
    width: u32,
    height: u32,
}

// 为Rectangle结构体实现方法
impl Rectangle {
    // 构造函数
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // 计算面积的方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 判断是否可以容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 创建正方形的关联函数（类似于静态方法）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 定义一个枚举表示IP地址类型
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // 带有数据的枚举变体
    V6(String),
}

// 定义一个更复杂的枚举，可以包含不同类型的数据
#[derive(Debug)]
enum Message {
    Quit,                       // 没有关联数据
    Move { x: i32, y: i32 },    // 包含匿名结构体
    Write(String),              // 包含单个String
    ChangeColor(i32, i32, i32), // 包含三个i32
}

// 为Message枚举实现方法
impl Message {
    fn call(&self) {
        // 在这里可以根据枚举变体类型处理不同的逻辑
        match self {
            Message::Quit => println!("  退出消息"),
            Message::Move { x, y } => println!("  移动到坐标: ({}, {})", x, y),
            Message::Write(text) => println!("  文本消息: {}", text),
            Message::ChangeColor(r, g, b) => println!("  更改颜色为: rgb({}, {}, {})", r, g, b),
        }
    }
}

// 定义一个Option枚举的使用示例函数
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}

fn main() {
    println!("Rust结构体和枚举示例");
    println!("--------------------");
    
    // 1. 结构体基础
    println!("\n1. 结构体基础：");
    
    // 创建结构体实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    // 使用Debug格式打印结构体
    println!("  矩形rect1: {:?}", rect1);
    println!("  更美观的打印: {:#?}", rect1);
    
    // 2. 结构体方法
    println!("\n2. 结构体方法：");
    
    // 使用new方法创建实例
    let rect2 = Rectangle::new(20, 40);
    println!("  矩形rect2: {:?}", rect2);
    
    // 调用area方法
    println!("  rect1的面积: {}", rect1.area());
    println!("  rect2的面积: {}", rect2.area());
    
    // 调用can_hold方法
    println!("  rect1能否容纳rect2: {}", rect1.can_hold(&rect2));
    println!("  rect2能否容纳rect1: {}", rect2.can_hold(&rect1));
    
    // 使用关联函数创建正方形
    let square = Rectangle::square(25);
    println!("  正方形: {:?}", square);
    println!("  正方形面积: {}", square.area());
    
    // 3. 枚举基础
    println!("\n3. 枚举基础：");
    
    let ipv4 = IpAddrKind::V4(127, 0, 0, 1);
    let ipv6 = IpAddrKind::V6(String::from("::1"));
    
    println!("  IPv4地址: {:?}", ipv4);
    println!("  IPv6地址: {:?}", ipv6);
    
    // 4. 复杂枚举
    println!("\n4. 复杂枚举：");
    
    let quit = Message::Quit;
    let mv = Message::Move { x: 10, y: 20 };
    let write = Message::Write(String::from("你好，Rust！"));
    let color = Message::ChangeColor(255, 0, 255);
    
    println!("  调用不同类型的消息：");
    quit.call();
    mv.call();
    write.call();
    color.call();
    
    // 5. Option枚举
    println!("\n5. Option枚举：");
    
    let numbers = [1, 3, 5, 8, 9, 11];
    
    match find_first_even(&numbers) {
        Some(num) => println!("  找到第一个偶数: {}", num),
        None => println!("  没有找到偶数"),
    }
    
    // 使用if let简化匹配
    let empty: [i32; 0] = [];
    if let Some(num) = find_first_even(&empty) {
        println!("  找到第一个偶数: {}", num);
    } else {
        println!("  空数组中没有找到偶数");
    }
    
    // 6. 模式匹配
    println!("\n6. 模式匹配：");
    
    let some_number = Some(5);
    
    // 使用match表达式
    println!("  使用match匹配Option<i32>:");
    match some_number {
        Some(5) => println!("  值是5"),
        Some(n) if n > 5 => println!("  值大于5: {}", n),
        Some(n) => println!("  值是: {}", n),
        None => println!("  没有值"),
    }
    
    // 使用if let简化单一模式匹配
    println!("  使用if let简化匹配:");
    if let Some(5) = some_number {
        println!("  值是5");
    }
}
