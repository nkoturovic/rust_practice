# rust\_practice

\#[derive(SomeName)] is an attribute that can only be applied to struct/enum/union type definitions, and it then applies the SomeName procedural macro (called a derive macro) to the source code that makes the type definition. The macro can then output / spit / emit its own forged source code, that will be emitted next to the original type definition source, which remains unaffected by the macro.
