use bitflags::bitflags; // bitflags! macro

bitflags! {
    struct Permission: u8 {
        const NONE = 0b0000;
        const CREATE = 0b1000;
        const READ = 0b0100;
        const UPDATE = 0b0010;
        const DELETE = 0b0001;
    }
}

enum Group {
    Guest,
    User,
    Admin,
    Owner,
}

// Action -> Requires some permissions
// Action -> Can derive permissions from subactions
// All actions should be compile-time known?
struct Action {
    // instance
// Fields: ...
// fn action()
}
// or
// |
// v

// Permission requirements
// #[Permissions(StructName:CR, field1:C, field2:R)]
// fn some_action() -> StructName {
// }
// Maybe should be enclosed and within that encolsure to have permissions somehow???
// to be able to derive on the fly and then just call u.get_all()

// u : User // u.get_all() -> from db: get_all is action
// who asked for u.get_all() ?

// Does group G ((has Permissions P) for action A) on resource T
trait Permissions<T> {
    fn has_permissions(g: Group, a: Action) {}
}

fn main() {

    // it works!
    let flags : Permission = Permission::CREATE | Permission::READ;
    println!("{:?}", flags);
}

// delete account should just disable account
