pub type Identify = String;

pub enum BinOp {
    Plus,
    Mult,
    Lt
}

pub enum Exp {
    Var(Identify),
    ILit(i64),
    BLit(bool),
    BinOp(BinOp, Box<Exp>, Box<Exp>),
    IfExp(Box<Exp>, Box<Exp>, Box<Exp>),
}

pub enum Program {
    Exp(Exp)
}

pub type Tyvar = i64;
pub enum Ty {
    TyInt,
    TyBool,
    TyVar(Tyvar),
    TyFun(Box<Ty>, Box<Ty>),
    TyList(Box<Ty>)
}