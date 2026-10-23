fn main() {
    // Stack and Heap memory allocation in Rust
    // Stack memory is used for storing values with a known size at compile time, such as primitive
    // types (integers, floats, booleans) and fixed-size arrays. Stack memory is fast since
    // allocation is done at the end of the stack, and deallocation is also done of the end element
    // of the stack.
    //
    // Heap memory is used for storing values with a dynamic size, such as vectors, strings, and
    // other data structures that can grow or shrink at runtime. Heap memory is slower to allocate
    // and deallocate.
    //
    // As mentioned values of variables that have unknown size at compile time are stored in the heap,
    // but the pointer of the data is stored in the stack. This means that when a variable is
    // created, a pointer to the heap memory is stored in the stack.
    //
    // During runtime when a function is called, its parameters and local variables are pushed onto
    // the stack. When the function is finished, its stack frame is popped off the stack.
    
    {                      // s is not valid here, since it's not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // this tells us about the scope of the variable, and its similar to what other programming
    // languages have.
    //
    // Here hello string literal is of type &str. Since its harcoded it remains in the executable
    // and is also immutable, it is permanently loaded into memory.
    // Immutable how?? But we can declare using let mut and it will become mutable right??
    // Yes... but No... so Yes you can reassign the value to s (after you declare it with let mut)
    // but the new value is `reassigned` to the Variable s, the existing old "hello" value still remains
    // in memory and we cannot change it. Difference is of Reassigning and Mutation of existing
    // string.
    //
    // To make a string mutable in place we can do following thing:
    {
        let mut s = String::from("hello"); // s is a mutable String
        s.push_str(", world!"); // push_str() appends a literal to the existing String
    }
    // For the variable in above example to be mutable we had to use String::from() to create a
    // String object on the heap. Such values are deallocated/removed from heap when
    // the variable that owns the values goes out of scope.
    //
    // Rust calls the `drop` function at end of {} to release the memory. We can also call it
    // explicity drop(var_name).
    //

    // Multiple variables and same data: **
        let x = 5;
        let y = x;
        println!("x= {x}, and y= {y}");
    // In above code we see that y = x which is equal to 5, therefore x and y equals 5.
    // Understand on memory level. Since the data type is integer, i.e. fixed size. Value of x is copied
    // and binded to variable y. These two 5s are pushed into stack.
            
        let s1 = String::from("hello");
        let _s2 = s1;
    // Now see this, what would happen here?
    // Wrong if you think that value of s1 is copied to the s2.
    //
    // We know that using String::from create a mutable string. This string is stored in heap since
    // its dynamic. String is made of three things pointer to that memory location in heap, Length
    // of string (in bytes), and Capacity is the memory (in bytes) that allocator gave for String.
    //
    // Now when we do s2 = s1, we are copying this pointer, length, and capacity, and NOT the value
    // on the heap. Keeping this in mind now lets think of memory deallocation. If the scope is over
    // then both s1 and s2 will try to clean up the same memory location, which is problem. This is
    // called double free error, and it can lead to memory corruption and security vuln.
    // To ensure memory safety, after statement s2=s1, rust considers the s1 as no longer valid. So
    // after scope of s1 ends rust doesn't need to cleanup anything.
    //
    // Therefore if you try to use s1 after s2=s1 statement you will get compile time error.
    //   println!("{s1} is s1 and {s2} is s2"); This will give error: borrow of moved value: `s1`
    //
    // This whole thing of invalidating s1 after s2=s1 is called `Move` in Rust. Move is a concept
    // that ensures memory safety by preventing double free errors and dangling pointers.

    // -------------------------------------------------------------------------------------- //
        let mut s = String::from("hello");
        println!("{s}, world!");
        s = String::from("ahoy");
        println!("{s}, world!");
    // In above code as soon as we assign the existing variable new value, Rust calls drop and
    // release memory for the original value. So for above code, "hello" is released from the heap as
    // soon as "ahoy" is assigned to s.

    // -------------------------------------------------------------------------------------- //
        let s3 = String::from("hello");
        let s4 = s3.clone();

        println!("s3 = {s3}, s4 = {s4}");

    // This will create deep copy of s3. i.e. Stack data and heap data both gets copied and assigned
    // to s4. This is expensive operation.

    // -------------------------------------------------------------------------------------- //
    // Stack only data copy.
    // We know that datatypes that have fixed size are stored in stack. When doing operations like
    // s1=s2 for such datatypes, Rust automatically copies there values in the stack, we do not need
    // clone().
    // Such datatypes have a Copy trait. Other such datatypes are:
    // 1. integers
    // 2. booleans
    // 3. floats
    // 4. character type 
    // 5. Tuples but only those whose elements are made of types that have Copy trait.
    //          like (i32, i32) will be put in stack but not (i32, String)

    // -------------------------------------------------------------------------------------- //
    // Ownership when passing variables to function
    let s5 = String::from("hello");
    takes_ownership(s5);  // s5 value moves into the function
    // s5 is no longer valid.
    // println!("{s5}"); // throws "borrow of moved value: s5" during compile time

    let i1 = 32;
    makes_copy(i1); // i1 has Copy trait, so the copy of i1 is sent to the function
    // i1 is still valid here
    println!("{i1}");

    // -------------------------------------------------------------------------------------- //
    // Return Values and Scope
    // Function return values also transfer Ownership.
    
    {
        let s6 = gives_ownership(); // s6 comes into scope. output of gives_ownership() is 
        // stored in s6, its value is moved into s6
        println!("S6= {s6}");

        let s7 = String::from("world!"); // s7 comes in scope. 
        let s8 = take_and_give_ownership(s7); // Ownership of s7 is moved to method. Method
        // return value of method is moved into s8.
        println!("s8 = {s8}"); //
    } // scope ends. S6 scope ends, s7 was already moved so nothing happens, s8 scope ends.
    

    // Why is this a problem?
    // Suppose you want a function that gives some info about input and then you use that info AND
    // the input somewhere else. In that case your function should take your input (i.e. move the
    // value to function) and then return BOTH input and output. Returning both, since you want to
    // use the input again in the parent call.
    // See below example:
    
    {
        let s8 = String::from("hello");  // s8 comes into scope
        let (str, length) = calculate_length(s8);  // s8 value moves to the function
        println!("Length of string: {str} is {length}");
    }   // s8 value was moved into function so nothing happens. str & length scope ends
    

    // -------------------------------------------------------------------------------------- //
    // References:
    // In above example, in order to use the variable s8 after calling calculate_length function we
    // had to return the s8 from the function back. This is very tedious/verbose if we do for all
    // functions and also think of functions with multiple parameters.
    //
    // In above case we were transfering the ownership of the variable. What if we just pass on the
    // reference of the value to the function. Reference is like a pointer it points to the pointer
    // of the variable. (Example: if `width` is a string we know it has three parts Pointer to heap
    // memory, length and capacity, then if we create reference using &width then &width has pointer
    // that points to the width. And when we access the &width we access width first and then using
    // the pointer in width we access memory in heap)
    //
    // So the function can use the value without taking ownership of it. This is called
    // Borrowing in Rust. To borrow a value we use & before a variable name. This is called
    // Reference.
    //
    // Pointers can sometimes point to invalid memory locations, which can lead to undefined
    // behaviour. But in Rust, references are guaranteed to always point to valid memory location.
    // Since it will not allow memory cleanup of the value until all references to it are out of
    // scope.
    //
    
    {
        let s9 = String::from("hello");             // s9 comes into scope
        let length = calculate_length_ref(&s9);     // Reference of s9 given to function, and not ownership
        println!("reference length: {length}");
    } // scope of s9 and length finishes here
    
    // Now since &s9 is just a Reference and not Owner when the calculate_length_ref(&s9)
    // function is completed the value of s9 will not be dropped.
    // REPEAT: This using of reference instead of real variable is called Borrowing.
    
    // References are shown by &var_name and are immutable by default. To make them mutable we do
    // `&mut var_name`.
    
    {
        let mut s10 = String::from("Hello");    // s10 comes into scope
        change(&mut s10);   // mutable reference of s10 passed to change function.
        println!("value of s10 after change(&mut s10) is: {s10}");  //Value of s10 after change
        //method changes its value
    }   // scope of s10 ends here and memory is released
    
    // there is RESTRICTION when using mutable reference. You cannot have any other mutable reference
    // of same variable until the mutable reference is in scope (Scope of references ends on their
    // last usage and not compulsorily when the {} brackets end like normal variables)
    //
    // So, we can:
    //      - have multiple immutable references simultaneously
    //      - not have immutable references and mutable references simultaneously. (Since immutable ref
    //      dont want the value to be changed)
    //      - not have multiple mutable references simultaneously (since both ref can change value
    //      and lead to data corruption)
    //      - have multiple immutable references and only AFTER their last usage we can have a
    //      mutable references.
    
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem // scope of r1 begins
        let r2 = &s; // no problem // scope of r2 begins
        // let r3 = &mut s; // BIG Problem error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        println!("{r1} and {r2}");
        // Variables r1 and r2 will not be used after this point and so Rust removes them from
        // scope

        let r3 = &mut s; // no problem
        println!("{r3}");
    }

    // DANGLING References
    // Dangling references are the pointers which are pointing to memory that has been given to
    // someone else. Or the pointer that is preserved after freeing the memory location.
    // Rust prevents dangling references by ensuring that data will not go out of scope until it 
    // has a reference and that reference is in scope.
    
    /*
    {
        let a = dangle();
    }
    */
}

fn takes_ownership(some_string: String) { // some_string comes into scope here
    println!("some string: {some_string}");
} // some_string scope is over and the memory is released.

fn makes_copy(some_int: i32) {
    println!("some int: {some_int}");
}

fn gives_ownership() -> String {
    String::from("hello") // string comes into scope, and value is moved into the s6 variable in
    // caller place
}

fn take_and_give_ownership(some_string: String) -> String {
    // some_string comes into scope
    some_string // value is moved to calling function.
}

fn calculate_length(some_string: String) -> (String, usize) {
    let length = some_string.len();
    (some_string, length)
}

fn calculate_length_ref(some_string: &String) -> usize { //some_string is reference to a String
    some_string.len()
}   // some_string goes out of context, but since some_string is not owner of the value, value of
    // string is not dropped

fn change(s: &mut String) { // s comes into scope
    s.push_str(" world!");  // update the string s that it borrowed
}   // scope of s ends, since the value was borrowed so Update value in memory is not cleared.

/*
fn dangle() -> &String {
    let s = String::from("hello"); // s comes into scope
    &s // return reference of s
}  // s goes out of scope and memory is released, so the reference returned is dangling and invalid.
   // DANGE!!!
   // But Rust compiler does not allow this and it throws error: `returns a reference to data owned
   // by the current function this function's return type contains a borrowed value, but there is 
   // no value for it to be borrowed from'
*/
