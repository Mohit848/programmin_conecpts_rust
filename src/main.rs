fn main() {
    //Variables and Mutability
    let x = 35; //Immutable i.e. can not be changed
    println!("{} is value of x", x);
    // x = 5; will give error 
    let x: String = String::from("I am a string");
    println!("{} - New x", x);
    let y = 20;

    //Types of variables
    //Scalars - Integers, Floating, Boolean, Characters
    //Compound - Tuples, arrays
    let tup = (1, 2, "Element");
    let arr = [1, 2, 3 , 4];
    println!("{}", tup.1);
    println!("{}", arr[0]);
    let sum = my_fucntion(x, y);
    println!("my_function returned - {sum}");

    //Control statement
    if sum>10 {
        println!("Sum is bigger than 10");
    }
    else if sum<=10 {
        println!("Sum is smaller or equal than 10");
    }
    else {
        println!("Error occured");
    }
    let get_val = if sum == 20 {y+30} else {y-10};
    println!("Value of getVal - {get_val}");

    //Loops
    //Type 1
    let mut counter = 10;
    let result = loop {
        counter-=1;
        println!("Value of counter is {}", counter);
        if counter==5{break counter;}
    };
    println!("Value of result is {result}");

    //Type 2
    while counter > 0{
        println!("Value in while loop - {counter}");
        counter-=1;
    }

    //Type 3
    for ele in arr.iter(){
        println!("Item in arr - {ele}");
    }
}

//Functions in rust
fn my_fucntion(x: String, y : i32) -> i32
{
    println!("Value of x and y is {} and {}", x, y);
    let sum = y + 20;
    sum
}
