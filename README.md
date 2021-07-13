# rust\_practice

\#[derive(SomeName)] is an attribute that can only be applied to struct/enum/union type definitions, and it then applies the SomeName procedural macro (called a derive macro) to the source code that makes the type definition. The macro can then output / spit / emit its own forged source code, that will be emitted next to the original type definition source, which remains unaffected by the macro.


# TODO 

## Modeling of Actions (pseudo-rust)

```rust

// This code requries more thinking and simplifying

struct User {
    id : i32;
    name : String
    ...
}


trait Action<Model> {
    type Model;
    const REQUIRED_PERMISSIONS : Permission;
    fn do(Model);
}

struct InsertModelInDb<Model>; 

impl Action<Model> for InsertModelInDb<Model> {
    type Model = Model;
    const REQUIRED_PERMISSIONS : Permissions = Permission::READ | Permission::CREATE;
    fn do(m : Model) {...}
}


// should be even simpler than this
#[route(post, /user)]
fn(s : SessionInfo, u : User) -> Result<(), Error> {
    if satisfies(InsertModelInDb::REQUIRED_PERMISSIONS, u.get_permissions(s.user.group))
        InsertModelInDb::do(u);
    else
        return Err("Permissions not satisfied")
}

```
