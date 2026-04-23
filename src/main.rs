use evm::{IR, VM, values::Value};

fn main() {
    let mut vm = VM::new(vec![
        IR::Push(Value::None),
        IR::Push(Value::Number(5.23)),
        IR::Pop,
        IR::Pop,
    ]);
    vm.run();
}
