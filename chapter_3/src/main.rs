fn main() {
    println!("Hello, world!");

    //IMP- Rust uses snake_case for function and variable names.

    // Immutability and mutability
    let x = 5; // immutable variable
    println!("The value of x is: {}", x);

    // x = 6; // this will cause a compile-time error because x is immutable

    let mut y = 6; // mutable variable
    y +=6;
    println!("The value of y is: {}", y);

    // making constants
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    // difference between constants and immutable variable:
    //      1. Constants must be annotated with data type. Immutable vars dont need.
    //      2. Constant can have constant expression as a value. It cannot have a value that is
    //         result of something that will be computer at runtime.
    println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    // In Rust we can create another variable with same name as prev variable, and we say that the
    // new second variable has shadowed the old variable. Which means that anybody calling the
    // variable will get the value of new variable, this will work till the scope ends or this
    // new variable is also shadowed again.
    // Use shadowing to avoid variable pollution i.e. for situations like parsing strings, or for
    // type coversion like from string to integer.

    let a = 5;
    let a = a+1;
    {
        let a = a*2;
        println!("The value of a in the inner scope is: {a}"); // prints 12
    }

    println!("The value of a is: {a}"); // prints 6

    println!("================Data Types=================");
    // There are two subtypes of data types: Scalar and compound types. Scalar types represent a
    // single value, whereas compound types can group multiple values into one type.

    // Scalar types
    // 1. integer
    //      can be signed or unsigned, and can be of different sizes (8, 16, 32, 64, 128 bits and
    //      arch dependent size). Signed integers can store both positive and negative numbers,
    //      while unsigned integers can only store positive numbers. The default type is i32.
    //      
    //      unsigned integer types: u8, u16, u32, u64, u128, usize  (0 to (2^n)-1) where n= bits
    //      signed integer types: i8, i16, i32, i64, i128, isize    (-(2^(n-1)) to 2^(n-1)-1) where
    //      n= bits
    //
    //      u32 means it can store 32 bits.
    //
    //      (isize and usize are arch dependent types, they are 32 bits on 32-bit arch and 64 bits
    //      on 64-bit arch)
    //      
    //      Integer overflow: occurs when we try to store the value that is greator than the
    //      capacity of the variable. i.e. u8 can store 0 to (2^8)-1=255 and we try to store 256,
    //      there are two things that would happen, during debug mode compile program panics and
    //      exits. Whereas it does not check that during --release compile. for --release compile
    //      the overflow is 2's complement wrapped, i.e. for u8 256 becomes 0 and 257 becomes 1;
    //      This may not be desired flow, so we can use few other tactics to manage the overflow:
    //      see at https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow.
    //
    // 2. floating-point
    //      Floating point have two type f32 and f64 (f64 is default). All floating point are
    //      signed.
    //      let x = 2.0; // f64
    //      let y: f32 = 3.0; // f32
    // 3. Boolean
    //      let isAdult = true;
    //      let isSeniorCitizen : bool = false;
    //
    // 4. Char
    //      4 bytes in size
    //
    //      let c = 'z';
    //      let z: char = '&';
    //
    //
    //Compound Types: store multiple values
    //  1. Tuple
    //      Tuple have fixed length. All elements need not be of same type.
    //
    //      let tup: (i32, f64, u8) = (500, 6.4, 1);
    //
    //      To get value we can:
    //          i. Destructuring:
    //                let tup: (i32, f64, u8) = (500, 6.4, 1);
    //                let (x, y, z) = tup;
    //
    //          ii. Using index
    //              println!(tup.0); // prints 500
    //              println!(tup.1); // prints 6.4
    //
    //      Tuple without any value is called *unit*
    //
    //  2. Array
    //      Array in Rust have fixed length and all elements have same data type.
    //
    //      let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct",
    //      "Nov", "Dec"];
    //
    //      let a: [i32; 5] = [1, 2, 3, 4, 5]; // anotate using [type; length]
    //
    //      let b = [3; 5];  // means array of size 5 and all elements to have value 3 i.e.
    //      [3, 3, 3, 3, 3]
    //
    //      //access elements using index
    //      println!(a[0]); // prints 1
    
    println!("===================Functions===================");
    // Function names should be in snake_case. They can have parameters, and return values.
    // 1. Parameters - are named and we have to define the parameter data type in the method
    //    signature.
            fn print_num(num: i32) {
                println!("Number is {num}!");
            }
            print_num(100);
    //    Here num is parameter of type integer
    //
    // 2. Body - body of function contains:
    //      i. Statments - Lines that do some action but do no return any value. (Ends with ;)
    //      ii. Expressions - lines that output the result value. (Doesn't end with semicolon)
            
            fn print_increment(num: i32) {
                let num = {
                    println!("number before increment is: {num}");
                    num + 1 
                };
                println!("number after increment is: {num}");
            }
            print_increment(100);
    //      
    //      Here 
    //          {
    //              println!("number before increment is: {num}");
    //              num + 1 
    //          }
    //        is a expression since it is returning a value. But it contains one statement (print line) and one expression (num+1).
    //
    // 3. Function with return values
    //      In Rust we do not have named return values. Multiple values can be returned using
    //      tuples.
    //
            fn add(num1: i32, num2: i32) -> i32 {
                num1 + num2
            }
            println!("addition of 1 and 2 gives: {}", add(10,11));
    //      
            fn get_something(given: i32) -> (i32, char) {
                (given+1, '+')
            }
            let (value, operator)= get_something(11);
            println!("we get: {value} after doing {operator} 1");

}
