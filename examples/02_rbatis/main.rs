use serde::{Serialize, Deserialize};
use validator::{Validate, ValidationError};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;

use serde_json;
use fast_log;

#[macro_use]
extern crate rbatis;

#[macro_use]
extern crate derive_builder;

#[crud_enable(table_name:users)]
#[derive(Builder, Default, Validate, Serialize, Deserialize, Debug)]
#[builder(setter(into, strip_option), default)]
struct User {
    pub id : Option<u64>,
    pub username : Option<String>,
    pub password : Option<String>,
    pub firstname : Option<String>,
    pub lastname : Option<String>,
    pub born : Option<String>,
    pub gender : Option<String>,
    pub biography : Option<String>,
    pub join_date : Option<String>,
    pub permission_group : Option<u64>,
}

#[tokio::main]
async fn main() {
    println!("Hello world");

    let rb = Rbatis::new();
    rb.link("sqlite:/home/kotur/Desktop/photowall.db").await.unwrap();
    let users : Vec<User> = rb.fetch_list("").await.unwrap();
    users.iter().for_each(|u| { println!("{:#?}", u); });
}
