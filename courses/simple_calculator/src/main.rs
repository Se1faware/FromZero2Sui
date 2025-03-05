// 简单计算器程序
// 展示Rust的基本语法、变量、函数和错误处理

use std::io::{self, Write};

// 加法函数
fn add(a: f64, b: f64) -> f64 {
    a + b
}

// 减法函数
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

// 乘法函数
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

// 除法函数，返回Result类型处理可能的错误
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("错误：除数不能为零"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("简单计算器");
    println!("----------");
    
    loop {
        // 提示用户输入第一个数字
        print!("请输入第一个数字: ");
        io::stdout().flush().unwrap(); // 确保提示立即显示
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        
        // 将输入解析为f64类型
        let num1: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效的数字，请重试");
                continue;
            }
        };
        
        // 提示用户输入操作符
        print!("请输入操作符 (+, -, *, /): ");
        io::stdout().flush().unwrap();
        
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("读取输入失败");
        let operator = operator.trim();
        
        // 提示用户输入第二个数字
        print!("请输入第二个数字: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        
        // 将输入解析为f64类型
        let num2: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("无效的数字，请重试");
                continue;
            }
        };
        
        // 根据操作符执行相应的计算
        let result = match operator {
            "+" => Ok(add(num1, num2)),
            "-" => Ok(subtract(num1, num2)),
            "*" => Ok(multiply(num1, num2)),
            "/" => divide(num1, num2),
            _ => Err(String::from("无效的操作符")),
        };
        
        // 处理计算结果
        match result {
            Ok(value) => println!("结果: {} {} {} = {}", num1, operator, num2, value),
            Err(error) => println!("{}", error),
        }
        
        // 询问用户是否继续
        print!("继续计算？(y/n): ");
        io::stdout().flush().unwrap();
        
        let mut continue_input = String::new();
        io::stdin().read_line(&mut continue_input).expect("读取输入失败");
        
        if continue_input.trim().to_lowercase() != "y" {
            println!("感谢使用！");
            break;
        }
    }
}
