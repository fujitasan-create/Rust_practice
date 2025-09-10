#![allow(dead_code)] //今回は学習用なので使わない関数に対する警告はそのままにしておきます。


pub fn hello(){
    println!("こんにちは");
}

pub fn variable(){
    let x:i32= 5;
    let y:i32 =10;
    println!("{}", x + y); 
    println!("{}", x - y); 
    println!("{}", x * y); 
    println!("{}", x / y); 
    println!("{}", x % y);
    println!("{}", x.pow(2));
}

pub fn for_loop(){
    for i in 0..3{
        println!("{}",i);
    }
}

pub fn for_loop2(){
    for j in 0..=3{
        println!("{}",j);
    }
}

pub fn for_loop3(){
    let nums=[10,20,30];
    for k in nums.iter(){
        println!("{}",k);
    }
}

pub fn if_statement(){
    let a=10;
    if a>5{
        println!("aは5より大きい");
    }else if a==5{
        println!("aは5と等しい");
    }else{
        println!("aは5より小さい");
    }
}

pub fn if_statement2(){
    let b=10;
    let result=if b>5 { "bは5より大きい" } else { "bは5以下" };
    println!("{}",result);
}

pub fn add(x:i32,y:i32)->i32{
    x+y
}