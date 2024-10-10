use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref GATES: std::sync::Mutex<HashMap<&'static str, fn(Option<Vec<Box<dyn std::any::Any>>>) -> bool>> =
        std::sync::Mutex::new(HashMap::new());
}

pub fn define_rule(fn_name: &'static str, fn_ptr: fn(Option<Vec<Box<dyn std::any::Any>>>) -> bool) {
    let mut gates = GATES.lock().unwrap();
    gates.insert(fn_name, fn_ptr);
}

pub fn gate_check(fn_name: &'static str, args: Vec<Box<dyn std::any::Any>>) -> bool {
    let gates = GATES.lock().unwrap();
    let gate = gates.get(fn_name).unwrap();
    gate(Some(args))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        define_rule("test", |args| {
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
}
