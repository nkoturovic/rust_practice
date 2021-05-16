// use serde::{Serialize, Deserialize};
// use serde_json::Result;

use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};

use serde_json;

#[derive(Validate, Serialize, Deserialize, Debug)]
struct User {
    id : u64,
    username : String,
    #[validate(length(min = 1, max = 64))] // How to map these into SQL constraints? should i?
    firstname : String,
    lastname : String,
    #[validate(range(min = 18, max = 20))]
    age : u8,
}

// #[derive(Crud, Validate, Serialize, Deserialize, Debug)]
// struct User {
//     #[crud(C)]
//     id : u64,
//     #[crud(CU)]
//     username : String,
//     #[crud(CR), validate(length(min = 1, max = 64))]
//     firstname : String,
//     lastname : String,
//     #[validate(range(min = 18, max = 20))]
//     age : u8,
// }
// ->
//
// CreateUser
// ReadUser
// ReadUserRequest // All ReadUser names with boolean
// UpdateUser
// DeleteUser
// //////////////////////////////////////
// Or even better
// User, // Entire struct (for example load from DB for some action or so)
// User::Create, // Only C + unmarked
// User::Read, // Only R  + unmarked
// User::Update, // Only U + unmarked
// User::Delete, // Only D + unmarked
//
// struct User {
//  Type Create = CreateUser
//  Type Read = ReadUser
//  Type Update = UpdateUser
//  Type Delete = DeleteUser
//  ++ Fields
// };
// Type aliases inside structs are not supported (maybe will be there is an
// tracking issue to be able to add it inside impl).
// It can be done with Traits or to be inside module mod

fn main() {
    let u = User {
        id : 0,
        username : String::from("kotur"),
        firstname : String::from("Nebojsa"),
        lastname : String::from("Koturovic"),
        age : 25
    };
    println!("{:#?}", u); // Using Debug
    println!("{:#?}", serde_json::json!(&u)); // Converto to json from u
    let s = serde_json::to_string(&u).unwrap(); // u as json string
    println!("{}", s);
    let ufs : User = serde_json::from_str(&s).unwrap();
    println!("{:#?}", ufs); // Using Debug

    match u.validate() {
        Err(e) => println!("{}", e),
        _ => println!("All ok")
    }
}
