use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref GATES: std::sync::Mutex<HashMap<&'static str, fn(Option<Vec<Box<dyn std::any::Any>>>) -> bool>> =
        std::sync::Mutex::new(HashMap::new());
}

pub fn define_gate(fn_name: &'static str, fn_ptr: fn(Option<Vec<Box<dyn std::any::Any>>>) -> bool) {
    let mut gates = GATES.lock().unwrap();
    gates.insert(fn_name, fn_ptr);
}

pub fn gate_check(fn_name: &'static str, args: Vec<Box<dyn std::any::Any>>) -> bool {
    let gates = GATES.lock().unwrap();
    let gate = gates.get(fn_name).unwrap();
    gate(Some(args))
}

macro_rules! values {
    ($($val:expr),*) => {
        vec![$(Box::new($val) as Box<dyn std::any::Any>),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        define_gate("test", |args| {
            let arg_list = args.unwrap();
            let name = arg_list[0].downcast_ref::<String>().unwrap();
            name == "ahmad"
        });

        let my_name = "ahmad".to_string();
        let other_name = "ali".to_string();

        let res1 = gate_check("test", vec![Box::new(my_name)]);
        let res2 = gate_check("test", vec![Box::new(other_name)]);

        assert_eq!(res1, true);
        assert_eq!(res2, false);
    }

    #[test]
    fn large_gate() {
        define_gate("large_gate", |args| -> bool {
            let arg_list = args.unwrap();
            let name = arg_list[0].downcast_ref::<String>().unwrap();
            let age = arg_list[1].downcast_ref::<i32>().unwrap();
            name == "ahmad" && age == &25
        });

        let name = "ahmad".to_string();
        let age = 25;

        let res1 = gate_check("large_gate", values![name, age]);
        println!("res1: {}", res1);
    }
}
