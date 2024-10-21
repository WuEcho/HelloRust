# Composite Types 符合类型


## 1.结构体

简单示例：

```
User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
```

### 结构体更新

示例：

```
fn main() {
    let mut user1 = User {
                    active: true,
                    username: String::from("someusername123"),
                    email: String::from("someone@example.com"),
                    sign_in_count: 1,
        };
    user1.email = String::from("anotheremail@example.com");
}
```

#### 便捷写法：

1. 自定义的变量名与结构体变量名相同的话无需采用：变量名：值。的形式

```
fn main() {
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user1 = User {
            active,
            username,
            email,
            sign_in_count: 1,
        };
}
```

2. 基于已有的实例的值来创建新实例，并只更新部分字段

```
struct User {
    active: true,
    username: String::from("someusername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

fn main() {
    let active = true;
    let username = String::from("someusername123");
    let email = String::from("someone@example.com");
    let user1 = User {
            active,
            username,
            email,
            sign_in_count: 1,
        };
        
     //这里 除了对email值的修改其他值都是 user1 的数据，就可用 ..user1 的方式
    let user2 = User {
        email:String::from("another@example.com");
        ..user1
    };    
}
```

## 2.元组结构体

元组结构体，也就是元组和结构体的结合体。元组结构体有类型名，但是无字段名，也即字段是匿名的。明显元组结构体更紧凑。只要是类型名不同，它们就是不同的类型。

示例如下：

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## 3.单元结构体

单元结构体就是只有一个类型名字，没有任何字段的结构体。定义和创建实例时连后面的花括号都可以省略。

示例：

```
struct ArticleModule;
fn main() {
    let module = ArticleModule; //类型实
际创建了一个结构体的实例
}
```

## 4.面向对象

`Rust`不是一门面向对象的语言，但是有部分面向对象的特性。`Rust`承载面向对象特性的部分一般是结构体。
`Rust`有个关键字 `impl` 可以用来给结构体（或其它类型）实现方法，也就是关联在某个类型上的函数。

示例：

```
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area1(self, n: u32) -> u32 {
        self.width * self.height * n
    }
    fn area2(&self, n: u32) -> u32 {
        self.width * self.height * n
    }
    fn area3(&mut self, n: u32) -> u32 {
        self.width * self.height * n
    }
}

fn main() {
    let rect1 = Rectangle{
        width : 30,
        height : 50,
    };
        
    println!("{}",rect1.area(5));
    //rect1.area2(n);
    //rect1.area3(n);
}
```

方法是实现在类型上的一类特殊的函数，它的第一个参数为`Self`类型，包含 `self: Self`, `self: &Self`, `self: &mut Self`三种情况，因为是标准用法了，所以`Rust`帮我们简写了

### 实例引用

- 1.实例的引用也是可以直接调用方法的

```
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area1(self, n: u32) -> u32 {
        self.width * self.height * n
    }
    fn area2(&self, n: u32) -> u32 {
        self.width * self.height * n
    }
    fn area3(&mut self, n: u32) -> u32 {
        self.width * self.height * n
    }
}

fn main() {
    let rect1 = Rectangle{
        width : 30,
        height : 50,
    };
    println!("{}",rect1.area1(5));
   
    let reat2: Rectangle =  Rectangle{
        width : 30,
        height : 50,
    };
    println!("{}",reat2.area2(6));
    
    let a = &&&reat2; 
    println!("{}", a.area2(6));

    let mut reat3: Rectangle =  Rectangle{
        width : 30,
        height : 50,
    };
    println!("{}",reat3.area3(6));
}
```

- 2.对同一个类型，`impl`可以分开写多次


```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
```

### 关联函数(静态方法)

在类型上实现方法，也可以第一个参数不带self参数。这种方法称为关联方法。调用的时候，用路径符来调用：`Rectangle::numbers(10, 10)`;

示例：

```
impl Rectangle {
    fn numbers(rows: u32, cols: u32) -> u32 {
        rows * cols
    }
}
```

### 构造函数

示例：

```
Foo::new();
Foo::from();
Default
```

## 5.Newtype模式

- 语法：`struct MyVec(Vec<u8>)`

## 6.枚举类型enum

枚举这种类型容纳选项的可能性，每一种可能的选项都是一个变体。`Rust`中的枚举非常强大，`enum`中的变体（`variant`）可以作为名字附带各种形式的结构。元组结构体，结构体，也可以作为`enum`的一个变体存在。

示例：

```
enum WebEvent {
// An `enum` variant may either be `unit-like`,
PageLoad,
PageUnload,
// like tuple structs,
KeyPress(char),
Paste(String),
// or c-like structures.
Click { x: i64, y: i64 },
}
```

### 实例化变体

示例：

```
let a = WebEvent::PageLoad;
let b = WebEvent::PageUnload;
let c = WebEvent::KeyPress('c');
let d = WebEvent::Paste(String::from("batman"));
let e = WebEvent::Click { x: 320, y: 240 };
```

### 类C枚举

示例：

```
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
```

### 空枚举
与空结构体一样，都表示一个类型，它不能被实例化。

示例：

```
enum MyEnum {};

enum Foo {} 

let a = Foo; // 错误的expected struct, variant or union type, found enum `Foo`
not a struct, variant or union type
```

### 结构体可以被`impl`

示例如下：

```
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x : i32, y : i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let add = VeryVerboseEnumOfThingsToDoWithNumbers::Add; 
   // add.run(100,200);
    println!("{}",add.run(100,200));
}

```

**注意：**不能对枚举的变体直接`impl`

```
enum Foo {
    AAA,
    BBB,
    CCC
}

impl Foo::AAA { // 错误的
}
```

## 7.模式匹配

`Rust`中的模式匹配非常强大。这个概念直接来自于函数式语言`Haskell`等。意思就是按对象值的结构形态进行匹配。

### 7.1 match匹配 

####  match做分支流程

示例：

```
 let num = 13;

match num {
    1 => println!("one"),
    2 | 3 | 5 | 11 => println!("This ia a prime"),
    13..=19 => println!("A teen"), 
    _ => println!(""),
} 
```

#### match结合枚举

全部分支必须处理


示例：

```
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
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!",state);
            25
        }
    }
}

fn main() {
    let a = UsState::Alabama;
    let b = Coin::Quarter(a);
    let r = value_in_cents(b);

    println!("{}",r);
}    
```

### 7.2 let匹配
`let`本身就支持模式匹配，`if let`, `while let`都是用的`let`的能力。

#### if let
只有两个分支或在这个位置先只想处理一个分支的情况，就可以使用`if let`。且相对于`match`，在分支体代码比较多的情况下，`if let`可以少一层括号。

示例：

```
let mut count = 0;
match coin {
Coin::Quarter(state) => println!("State quarter from {:?}!", state),
_ => count += 1,
}

//if let
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}

```

#### while let
示例：

```
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9,quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again",i);
            optional = Some(i+1);
        }
    }
```

#### 匹配元组

这种用法，常常叫作元组体的析构。常用来从函数的多返回值中取出数据。

示例：

```
    let t = (1,2,3);
    let (a,b,c) = t;
    println!("{:?}",t);
    println!("{}",a);
    println!("{}",b);
    println!("{}",c);
```


```
fn foo()->(u32,u32,char) {
   (2,3,'a')
}

fn main() {
    let (d,e,f) = foo();
    println!("{}",d);
    println!("{}",e);
    println!("{}",f);
}
```

#### 匹配结构体


```
struct User {
    name:String,
    age:u32,
    student: bool
}

fn main() {
    let us = User{
        name:String::from("mike"),
        age:20,
        student:false,
    };
    let User {
        name,
        age,
        student,
    } = us;
    println!("{}",name);
    println!("{}",age);
    println!("{}",student);
}
```


### Vec与HashMap的一些经验

#### 引用到切片引用的自动转换

- `&String` -> `&str`
- `&Vec<u8>` -> `&[u8]`
- `&Vec<T>` -> `&[T]`

示例：

```‘
fn fool(s : &str) {}

fn fool1(s : &[u32]) {}

fn main() {
    let s = String::from("aaa");
    fool(&s);

    let v :Vec<u32> =vec![1,2,3,4];
    fool1(&v);
}
```

#### Vec中的所有权
`Vec<String>`是对`String`带有所有权的。那`Vec<>`中，自然也能放`Vec<&str>`种引用。
可以看到，所有权不能`move`出来，只能使用引用去访问。

示例：

```
    let s1 = String::from("a superman");
    let s2 = String::from("two superman");
    let s3 = String::from("3 superman");
    let v = vec![s1,s2,s3];
    println!("{:?}",v);
    //let a = v[0]; //无法移动出来
    let a = &v[0];
    println!("{:?}",a); //可以打印出数据
```

#### HashMap中的所有权
`HashMap`是不会把里面内容的所有权让出来的

#### Vec和HashMap中的元素所有权如何拿出来
通过迭代器 `.into_iter()`

### 补充

- hashmap: [https://rustwiki.org/zh-CN/std/collections/struct.HashMap.html](https://rustwiki.org/zh-CN/std/collections/struct.HashMap.html)
- vector: [https://rustwiki.org/zh-CN/std/vec/struct.Vec.html](https://rustwiki.org/zh-CN/std/vec/struct.Vec.html)
- option: [https://rustwiki.org/zh-CN/std/option/index.html](https://rustwiki.org/zh-CN/std/option/index.html)
- [https://github.com/sjinzh/awesome-rust-list#learning-resources](https://github.com/sjinzh/awesome-rust-list#learning-resources)

