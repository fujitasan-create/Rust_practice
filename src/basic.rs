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

// 基本型
const INT_NUM: i32 = 42;
const UINT_NUM: u64 = 100;
const FLOAT_NUM: f64 = 3.14;
const IS_TRUE: bool = true;
const LETTER: char = 'あ';
const TEXT: &str = "こんにちは";

// コレクション型
const ARRAY: [i32; 3] = [1, 2, 3];
// Vec はヒープ確保なので const 不可 → static
static VECTOR: &[i32] = &[10, 20, 30];
// タプルは const でOK
const TUPLE: (i32, f64, &str) = (1, 2.5, "abc");

// ユーザー定義型
struct Point {
    x: f32,
    y: f32,
}

enum Direction {
    North,
    South,
    East,
    West,
}

type Kilometers = i32;

// 特殊型
// Option, Result は const でOK
const MAYBE_NUM: Option<i32> = Some(5);
const MAYBE_NONE: Option<i32> = None;

const RESULT_OK: Result<i32, &str> = Ok(200);
const RESULT_ERR: Result<i32, &str> = Err("error happened!");

//　スタック領域に確保されるデータ、ヒープ領域に確保されるデータがあります。
//　基本型やタプル、配列などはスタック領域に確保されます。
//　VecやString、HashMapなどはヒープ領域に確保されます。
//　ヒープ領域に確保されるデータは、サイズが動的に変わるため、コンパイル時にサイズが決まらないことがあります。
//　そのため、ヒープ領域に確保されるデータは、const ではなく static で宣言する必要があります。
//　static はプログラムの実行中ずっと存在するデータで、const はコンパイル時に決まる不変の値です。