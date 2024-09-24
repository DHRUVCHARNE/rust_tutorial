struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64
}
impl User {
    fn show(&self) -> () {
    println!("Username is :{:?}",self.username);
    println!("Email is :{:?}",self.email);
    println!("Sign in count is :{:?}",self.sign_in_count);
    println!("Active status is :{:?}",self.active);
    }

}
enum Direction {
    North,
    South,
    East,
    West
}
use std::time::Duration;
use std::fs;
use std::thread;
use std::sync::mpsc;
use chrono::{Local,Utc};//Utc and Local are timezone structs
use std::collections::HashMap;
fn main() {
    let user1 = User {
        active:true,
        username:String::from("harry"),
        email:String::from("harry@harry"),
        sign_in_count:1
    };
    let ans = is_even(50);
    let my_string = String::from("Hello,Universe!");
    let x = fib(15);
    println!("Hello, world!");
    println!("Is 50 Even ?,{}",ans);
    println!("The 15th Fibonacci Number is :{}",x);
    println!("The length of my string is :{}",get_string_length(&my_string));
    User::show(&user1);
    let my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction);
    let my_string2= String::from("DhruvCharne");
    //Option run of first type
    //let first_a = find_first_a(my_string2);
    //println!("First a is at :{:?}",first_a);
    match find_first_a(my_string2)  {
        //Pattern matching syntax
        Some(i) => println!("First a is at :{}",i),
        None => println!("No a found"),


    }
    let lorem_file = fs::read_to_string("E:/rust-1-2/src/hello.txt");
    //lorem file is an enum
    match lorem_file {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {}", error),
    };
    println!("Current date and time in UTC: {}",Utc::now());
    println!("Current date and time in Local: {}",Local::now());
    println!("Formatted date and time in UTC: {}",Utc::now().format("%Y-%m-%d %H:%M:%S"));
    create_string();
    /* This code gonna cause error as now my_string4 is the owner and my_string3 is invalid for any use
    let my_string3 = String::from("Hello,Universe!");
    let my_string4 = my_string3;
    println!("{}",my_string3);
    */
    //This code will run without any error
    //We'll use clone method
    let my_string3 = String::from("Using clone method");
    let my_string4 = my_string3.clone();
    //Using reference borrowing you can write it as &my_string3 = &my_string4
    println!("{}",my_string3);
    println!("{}",my_string4);
    /* This code will run without any error but doesn't have Borrowing
    let mut s1 = String::from("NotBorrowing");
    s1 = do_something(s1);
    println!("{}",s1);
     */
    //Clean Code using Borrowing
    let my_string5 = String::from("Borrowing");
    do_something(&my_string5);
    println!("Borrowing Answer: {}",my_string5);
    //Vectors in Rust are alike C++
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    println!("Vector v : {:?}",v);
    let v2 = vec![1,2,3,4];
    println!("Vector v2: {:?}",v2);//{:?} is the debug trait we don't use {} as vectors are structs
    let v3 = vec![1,2,3,4,5,6,7,8];
    println!("v3 after Even Filter: {:?}",even_filter(&v3));
    //Hashmaps Example Below
    let mut users = HashMap::new();
    users.insert(String::from("harry"),50);
    users.insert(String::from("harry2"),51);
    println!("{:?}",users);
    let mut user2 = users.get("harry");
    println!("{:?}",user2.unwrap());
    let mut user3 = users.get("harry3");
    match user3 {
    Some(age) => println!("Age is: {}",age),
    None => println!("No age found"),
    }
    //Hashmap Example with Vectors
    let input_vec = vec![(String::from("dhruv"),21),(String::from("charne"),22)];
    println!("{:?}",group_values_by_keys(input_vec));
    //Iterators
    let v4 = vec![1,2,3,4,5,6,7,8];
    //Iterating using for loop does this under the hood by creating an iterator
    for value in v4 {
        println!("{}",value);
    }
    //Iterating after creating an iterator
    let v5 = vec![1,2,3,4,5,6,7,8];
    let v5_iter = v5.iter();
    for value in v5_iter {
        println!("{}",value);
    }
    //IterMut
    let mut v6 = vec![1,2,3,4,5,6,7,8];
    let v6_iter = v6.iter_mut();
    for value in v6_iter {
    *value = *value + 1;
    }println!("{:?}",v6);
    //Iterating using .next() this function takes a mutable argument only
    let v7 = vec![1,2,3,4,5,6,7,8,9,10];
    let mut v7_iter = v7.iter();
    while let Some(x) = v7_iter.next() {
        println!("{}",x);
    }

    //IntoIter improves performance as now it owns and doesn't reference the value
    let v8 = vec![1,2,3,4,5,6,7,8,9,10];
    let v8_into_iter = v8.into_iter();
    for value in v8_into_iter {
        println!("{}",value);
    }
    //Consuming Adaptor ex: sum() which consume the iterator
    let  v9 = vec![1,2,3,4,5,6,7,8,9,10];
    let v9_iter = v9.iter();
    let total: i32 = v9_iter.sum();
    println!("Total Sum of v9 is: {}",total);
    //Can't write below as iterator got consumed by sum()
    //let iter_sum = v9_iter.sum();

    //Iterator Adaptor
    //map()
    //|x| is the argument and x+1 is the return value
    let v10 = vec![1,2,3,4,5,6,7,8,9,10];
    let v10_iter = v10.iter();
    let v10_map = v10_iter.map(|x| x+1);
    for value in v10_map {
        println!("{}",value);
    }

    //filter
    let v11 = vec![1,2,3,4,5,6,7,8,9,10];
    let v11_iter = v11.iter();
    let v11_filter = v11_iter.filter(|x| **x % 2 == 0);
    for value in v11_filter {
        println!("{}",value);
    }

    //Filter all odd values then double each value and create a new vector
    let v12 = vec![1,2,3,4,5,6,7,8,9,10];
    let v12_filter = v12.iter().filter(|x| **x % 2 == 1).map(|x| *x*2);
    let v12_ans: Vec<i32> = v12_filter.clone().collect();
    println!("v12_ans : {:?}",v12_ans);
    for value in v12_filter {
        println!("{}",value);
    }

    // Create a Hashmap and populate it with some key-value pairs
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    // Iterate over references to the key - value pairs
    for (key,value) in scores.iter() {
        println!("{}: {}",key,value);
    }
    // Iterating over mutable references to the key-value pairs
    for (key,value) in scores.iter_mut() {
        *value += 100;
        println!("Key:{},Value:{}",key,value)
    }
    //Creating a String
    let mut name = String::from("Dhruv");
    println!("Hello, {}",name);
    //Mutating a String
    name.push_str(" Charne");
    println!("Hello, {}",name);
    //Deleting from a String
    name.replace_range(5..name.len(),"");
    println!("Hello, {}",name);

    //Takes a string as an input and returns the first word from it not very optimal code as It returns a new string.
    //Here we take up double the memory
    //If the sentence string gets cleared the ans still has How as the value in it. which might create issues in successive function calls.
    //If sentence changes then ans should also change during the lifetime of this program
    //Hence Two Strings are created in the heap at runtime
    let sentence = String::from("How do you do?");
    let first_word = first_word(sentence);
    println!("The first word is :{}",first_word);

    //A Better Optimal approach to the previous problem would be to create a string slice pointing to the how in the sentence
    //What we want is a view into another string not to copy it over
    let word = String::from("How are You?");
    let word2 = &word[0..3];
    println!("The first word of the 'word' is :{}",word2);

    //Array Slice
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    println!("The slice is : {:?}",slice);

    //Traits
    pub trait Summary {
        fn summarize(&self) -> String;

    }
    struct UserTrait {
        name: String,
        age: u32,
    }
    impl Summary for UserTrait {
        fn summarize(&self) -> String {
            format!("User Name is: {} , And User Age is: {}", self.name,self.age)
        }
    }
    //Traits as Parameters or Arguments
    pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}",item.summarize())
}
    let user1 = UserTrait {
        name: String::from("Dhruv"),
        age: 25,
    };
    notify(&user1);

    //Lifetimes in Rust
    let x = "abc";
    let y = "def";

    let result = longest(x, y);
    println!("The longest string is {}", result);

    //Multithreading: Calculating Factorials Concurrently
    let numbers = vec![5, 10, 15];
    let (tx, rx) = mpsc::channel();

    for number in numbers {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            let result = factorial(number);
            tx.send(result).unwrap();
        });
        if let Ok(_) = handle.join() {
        println!("Thread finished within time");
    } else {
        println!("Thread timed out");
    }
    }

    for result in rx {
        println!("Factorial of {} is {}", result, result);
    }


}
fn is_even(num: i32) -> bool {
    if num%2 == 0 {
        return true;
    } else {
        return false;
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
fn fib(num:u32) -> u32 {
    /*
    Recursion
    if num == 0 || num == 1 {
        return num;
    } else {
        return fib(num-1) + fib(num-2);
    }*/
    let mut first = 0; //mut shows that a variable, reference, or function parameter can be mutated.By default every variable is constant so you need to explicitly tell if it is mutable.
    let mut second = 1;
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

        for _ in 2..=num {
            let temp = second;
            second=second+first;
            first = temp;

    }
    return second;

}
fn get_string_length(s: &str) -> usize {
    s.chars().count()//implicitly returned not explicitly written return statement
}
fn move_around(dir:Direction) {
    match dir { //Pattern Matching Syntax
        Direction::North => println!("I am heading north"),
        Direction::South => println!("I am heading south"),
        Direction::East => println!("I am heading east"),
        Direction::West => println!("I am heading west"),
    }

}
fn find_first_a(s:String) -> Option<i32> {
    for (i,c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(i as i32);
        }
    }
    return None;
}
fn create_string() {
    let s = String::from("There's No Memory Leak Here!");
    println!("{}",s);//It is a RAII Language
    //Hereafter no s so s goes out of scope and it will be removed from the heap
}
/*fn do_something(s:String) -> String {//s owns the value
    println!("{}",s);
    return s;
}*/
fn do_something(test_string: &String) {
    println!("{}",test_string);//
}
fn even_filter(vec:&Vec<i32>) -> Vec<i32> {

    let mut even_numbers = Vec::new();
    for num in vec {
        if num % 2 == 0 {
            even_numbers.push(*num);//Pointer to num
        }
    }
    return even_numbers;
}
fn group_values_by_keys(vec:Vec<(String,i32)>) -> HashMap<String,i32> {
    let mut map = HashMap::new();
    for (key,value) in vec {
        map.insert(key,value);
    }
    return map;
}
fn first_word(str:String) -> String {
    let mut ans = String::from("");
    for c in str.chars() {
        if c != ' ' {
            ans.push_str(&c.to_string());
        } else {
            break;
        }
    }
    return ans;
}
