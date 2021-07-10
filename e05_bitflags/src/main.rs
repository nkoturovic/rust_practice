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

struct User {
    id: Option<u32>,
    name: Option<String>,
}

struct UserPermissions {
    id: Permission,
    name: Permission,
}

trait PermissionInfo {
    type Model;
    type ModelPermissions;
    fn get_permissions(g: Group) -> Self::ModelPermissions;
    fn check_permissions(
        self,
        group: Group,
        desired: Self::ModelPermissions,
    ) -> Result<(), String>;
}

impl PermissionInfo for User {
    type Model = Self;
    type ModelPermissions = UserPermissions;

    fn get_permissions(g: Group) -> Self::ModelPermissions {
        todo!();
    }

    fn check_permissions(
        self,
        group: Group,
        desired: Self::ModelPermissions,
    ) -> Result<(), String> {
        let perms = Self::get_permissions(group);
        if (perms.id & desired.id) != desired.id {
            return Err(String::from("id does not satisfy permissions"));
        }

        if (perms.name & desired.name) != desired.name {
            return Err(String::from("name does not satisfy permissions"));
        }

        Ok(())
    }
}

// #[permissions(guest:R,user:CR,owner:CRUD)]

// Action -> Requires some permissions
// Action -> Can derive permissions from subactions
// All actions should be compile-time known?
// // // // struct Action {
// // // //     // instance
// // // // // Fields: ...
// // // // // fn action()
// // // // }
// or
// |
// v

// Permission requirements
// Something like: #[crud_enable(table_name:users)]
// #[action(permissions(StructName:CR, field1:C, field2:R))]
// fn some_action() -> StructName {
// }
// Maybe should be enclosed and within that encolsure to have permissions somehow???
// to be able to derive on the fly and then just call u.get_all()

// u : User // u.get_all() -> from db: get_all is action
// who asked for u.get_all() ?

// Does group G ((has Permissions P) for action A) on resource T
// trait Permissions<T> {
//     fn has_permissions(g: Group, a: Action) {}
// }

fn main() {
    // it works!
    let flags: Permission = Permission::CREATE | Permission::READ;
    println!("{:?}", flags);
}

// delete account should just disable account
