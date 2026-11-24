fn main() {
    
    // Enums
    {
        #[derive(Debug)]
        enum IPAddrType{ // basic enums
            V4, // enum variant
            V6,  // enum variant
        }
        
        let home_ip_type = IPAddrType::V4; // calling variable
        let _work_ip_type = IPAddrType::V6;

        fn get_type(obj: IPAddrType) { // Using enum to define the param type
            println!("IP type: {obj:?}");
        }

        get_type(home_ip_type);
    }

    {
        #[derive(Debug)]
        enum IPAddrType {
            V4(String),  // Enum
            // V4(u8, u8, u8, u8),  // A variant can contain any type and amount of values, structs, primitive types etc.
            V6(String),
        }

        let home_ip = IPAddrType::V4(String::from("127.0.0.1"));
        let _work_ip = IPAddrType::V6(String::from("::1"));
        
        println!("IP is: {:?}", home_ip);
    }

    {
        enum Message {
            Quit,   // No data associated with this variant
            Move { x: i32, y: i32 }, // has named fields, like structs
            Write(String),  // has string only
            ChangeColor(i32, i32, i32), // has three i32 values
        }

        // Just like the Structs you can define functions for enums also
        impl Message {
            fn print_type(&self) {
                match self {
                    Message::Quit => println!("Quit"),
                    Message::Move {x, y} => println!("Move {x} {y}"),
                    Message::Write( x ) => println!("Write {x}"),
                    Message::ChangeColor(..) => println!("ChangeColor"), // .. is to ignore all the paramenter
                }
            }
        }

        let msg = Message::Write(String::from("Siddharth"));
        msg.print_type();
    }

    //-------- Option enum
    {
        // Its included by default and doesn't need to import to the program
        // Rust doesn't have type NULL.
        // Implementation is like:
        //  enum Option<T> { // this <T> thing is a Generics
        //      None,
        //      Some(T),
        //  }
        let _some_num = Some(23); // Its type is Option<i32>
        let _some_char = Some("sid"); // Type is Option<String>

        let _no_num: Option<i32> = None;    // We need to give the type of the variable for the
        // None type, since compiler wont know what would be corresponding type of the Some value.
        //
        // Basically, even when you use the None or Some, it would make the Option object. So if you
        // use Some compiler would know the type of the variable based on value assigned to Some,
        // but when you use the None, compiler wont know what would be type of the T in Option<T>,
        // so it cant make the object of Option<T>
    }

}
