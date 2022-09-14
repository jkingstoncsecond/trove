
#[derive(Debug)]
pub struct Fn{
    
}

#[derive(Debug)]
pub enum Type{
    U32,
    I32,
    BOOL,
    FN(Fn)
}