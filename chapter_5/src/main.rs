struct User {
    name: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

fn main() {
   
   let mut user1 = build_user(String::from("Siddharth"), String::from("siddharth@gmail.com"));

   // access the value
   println!("User name of the user1: {}", user1.name);

   // assign or update the value (you need to make the variable mutable)
   println!("User email of the user1 BEFORE: {}", user1.email);
   user1.email = String::from("siddharthmore@somemail.com");
   println!("User email of the user1 AFTER: {}", user1.email);


   //-------- Struct Update Syntax
   {
       // Sometimes when we create a instance of a struct we would like to have some values same as some
       // other instance of the struct. We can do it in two ways.
   //  First way
        
        let _user2 = User {
            name: String::from("Harsh"),
            email: String::from("harsh@domain.com"),
            active: user1.active,   // Use same value as user1 instance
            sign_in_count: user1.sign_in_count  // Use same value as user1 instance
       };
    // Second way- Shorthand
        
        let _user3 = User {
            email: String::from("Ashish@domain.com"),
            ..user1 // Use remaining field (name, active, sign_in_count) corresponding values of user1
        };
        
        println!("email: {}", user1.email); // prints name: siddharthmore@somemail.com
        // println!("name: {}", user1.name); // Compilation error, because after using ..user1 in user3
        //      and since the name field was used from user1 the ownership of user1.name was move to
        //      _user3.name.
        //      user1.active and user1.sign_in_count are primitive types which implements Copy trait,
        //      so their values would be copied into _user3.active and _user3.sign_in_count and their
        //      ownership remains with user1
    }
   
   //-------- Tuple Struct Syntax
   {
        struct Color(i32, i32, i32);

        let black = Color(0, 0, 0);
        println!("red: {}, green: {}, blue: {}", black.0, black.1, black.2);
   }

   //-------- Printing the DTOs or objects using debug trait
   {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            heigth: u32
        }

        let rect1 = Rectangle{
            width: 12,
            heigth: 30
        };
        println!("rectangle1 = {rect1:?}"); // rectangle1 = Rectangle { width: 12, heigth: 30 }
   }

   //-------- Methods and functions
   {
        #[derive(Debug)]
        struct User {
            email: String,
            name: String
        }

        impl User {
            fn get_user_name(&self) -> String {
                self.name.clone()
            }

            fn get_email(&self) -> String {
                self.email.clone()
            }
        }

        let user1 = User {
            email: String::from("siddharth.aj.more@gmail.com"),
            name: String::from("siddharth")
        };

        println!("the username is: {}",user1.get_user_name());
        println!("the email is: {}",user1.get_email());

   //------- Associated functions
   // All methods in the impl block are associated functions. Apart from those, we can also have the
   // functions without the &self parameter, such functions are used when we dont want the instance
   // of the type to work with and we used it when we did String::from.

        impl User { // by the way we can have multiple impl block for same struct
            fn from(other: &User) -> User {
                User {
                    email: other.email.clone(),
                    name: other.name.clone()
                }
            }
        }

        let user2 = User {
            email: String::from("ashishjha@gmail.com"),
            name: String::from("Ashish Jha")
        };

        let user3 = User::from(&user2);

        println!("user3 using associate function: {user3:?}");
   }
}

fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        sign_in_count: 1,
        email: email,
        active: true
    }
    // Order of field does not matter

    // If the field name and parameter name is same, we do not need to mention like name: name, we
    // can just give name and Rust will understand. This is called field init shorthand
    //    User {
    //     name,
    //     active: true,
    //     email,
    //     sign_in_count: 1
    // }

}
