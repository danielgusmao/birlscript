
use vm;
use parser;

/// Implementação dos comandos
mod cmd {
    use vm::signal::Signal;
    use value::{self, Value, ValueType};
    use vm::variable::{Variable, Access, AccessType, Permission};
    use vm::VM;
    use parser::Command;
    use vm::comparision::Comparision;

    pub fn cmd_move(name: String, value: String, vm: &mut VM) -> Option<Signal> {
        let value = value::parse_expr(&value, vm);
        let varname = Variable::make_name(&name, vm.current_section());
        vm.modify_variable(&varname, value);
        None
    }

    pub fn clear(var: String, vm: &mut VM) -> Option<Signal> {
        let val = Value::Number(0.0);
        let var = Variable::make_name(&var, vm.current_section());
        vm.modify_variable(&var, val);
        None
    }

    pub fn decl(var: String, vm: &mut VM) -> Option<Signal> {
        let current_sect = vm.current_section().to_string();
        let result = Variable::from(&var,
                                    &current_sect,
                                    Value::Number(0.0),
                                    Permission::ReadWrite(current_sect.clone()));
        vm.declare_variable(result);
        None
    }

    pub fn declwv(var: String, val: String, vm: &mut VM) -> Option<Signal> {
        let current_sect = vm.current_section().to_string();
        let val = value::parse_expr(&val, vm);
        let result = Variable::from(&var,
                                    &current_sect,
                                    val,
                                    Permission::ReadWrite(current_sect.clone()));
        vm.declare_variable(result);
        None
    }

    pub fn jump(sect: String, vm: &mut VM) -> Option<Signal> {

        None
    }

    pub fn cmp(left: String, right: String, vm: &mut VM) -> Option<Signal> {
        let (left, right) = (value::parse_expr(&left, vm), value::parse_expr(&right, vm));
        vm.compare(left, right);
        None
    }

    pub fn cmp_eq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_neq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::NEqual) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_less(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Less) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_lesseq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::Less) || vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_more(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::More) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn cmp_moreeq(command: Command, vm: &mut VM) -> Option<Signal> {
        if vm.last_cmp_is(Comparision::More) || vm.last_cmp_is(Comparision::Equal) {
            super::run(command, vm)
        } else {
            None
        }
    }

    pub fn print(args: Vec<String>, vm: &mut VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", value::parse_expr(arg, vm));
        }
        None
    }

    pub fn println(args: Vec<String>, vm: &mut VM) -> Option<Signal> {
        for arg in &args {
            print!("{}", value::parse_expr(arg, vm));
        }
        println!("");
        None
    }

    pub fn quit(code: String, vm: &mut VM) -> Option<Signal> {
        let result = if code != "" {
            value::parse_expr(&code, vm)
        } else {
            Value::Number(0.0)
        };
        if let Value::Number(c) = result {
            Some(Signal::Quit(c as i32))
        } else {
            panic!("Erro ao tentar quitar com código invalido (não-número)")
        }
    }

    pub fn cmd_return(val: Option<String>, vm: &mut VM) -> Option<Signal> {

        Some(Signal::Return)
    }

    pub fn input(var: String, vm: &mut VM) -> Option<Signal> {
        let inp = get_input();
        let var = Variable::make_name(&var, vm.current_section());
        vm.modify_variable(&var, Value::Str(Box::new(inp.to_string())));
        None
    }

    pub fn input_upper(var: String, vm: &mut VM) -> Option<Signal> {
        let inp = get_input().to_uppercase();
        let var = Variable::make_name(&var, vm.current_section());
        vm.modify_variable(&var, Value::Str(Box::new(inp.to_string())));
        None
    }

    fn get_input() -> String {
        use std::io;
        let sin = io::stdin();
        let mut res = String::new();
        sin.read_line(&mut res).expect("Erro lendo da entrada padrão");
        res.trim().to_string()
    }
}

pub fn run(cmd: parser::Command, vm: &mut vm::VM) -> Option<vm::signal::Signal> {
    use parser::Command::*;
    match cmd {
        Move(a, b) => cmd::cmd_move(a, b, vm),
        Clear(a) => cmd::clear(a, vm),
        Decl(a) => cmd::decl(a, vm),
        DeclWV(a, b) => cmd::declwv(a, b, vm),
        Jump(a) => cmd::jump(a, vm),
        Cmp(a, b) => cmd::cmp(a, b, vm),
        CmpEq(a) => cmd::cmp_eq(*a, vm),
        CmpNEq(a) => cmd::cmp_neq(*a, vm),
        CmpLess(a) => cmd::cmp_less(*a, vm),
        CmpLessEq(a) => cmd::cmp_lesseq(*a, vm),
        CmpMore(a) => cmd::cmp_more(*a, vm),
        CmpMoreEq(a) => cmd::cmp_moreeq(*a, vm),
        Print(a) => cmd::print(a, vm),
        Println(a) => cmd::println(a, vm),
        Quit(a) => cmd::quit(a, vm),
        Return(a) => cmd::cmd_return(a, vm),
        Input(a) => cmd::input(a, vm),
        InputUpper(a) => cmd::input_upper(a, vm),
    }
}
