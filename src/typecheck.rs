
#[derive(Debug)]
pub struct Fn{
    
}

#[derive(Debug)]
pub enum Primative{
    U32,
    I32,
    BOOL,
    FN(Fn)
}

#[derive(Debug)]
pub enum Mutability {
    MUTABLE,
    CONSTANT
}

#[derive(Debug)]
pub struct Type {
    pub mutability: Mutability,
    pub primative: Primative
}