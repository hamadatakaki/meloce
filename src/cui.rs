use crate::{
    environment::ExpEnvironment,
    eval::Evaluator,
    syntax::{BinOp, Exp, Program},
};

// TODO: Parser, Lexer を作る

pub fn read_eval_print(env: ExpEnvironment) {
    print!("# ");

    // Parser, Lexer で置き換え
    // let decl = ...
    let prog = Program::Exp(Exp::BinOp(
        BinOp::Plus,
        Box::new(Exp::ILit(23)),
        Box::new(Exp::ILit(19)),
    ));

    println!("parsing done");

    let evaluator = Evaluator::new(env);

    match evaluator.eval_decl(prog) {
        Ok((id, newenv, v)) => {
            println!("val {} = {}", id, v);
            read_eval_print(newenv);
        }
        Err(e) => {
            println!("error!: {}", e);
        }
    }
}
