pub fn cli_command(arg1: &str, arg2: &str) -> String {
    // Implement your CLI command logic here
    format!("Received args: {} {}", arg1, arg2)
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
