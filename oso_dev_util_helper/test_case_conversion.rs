use oso_dev_util_helper::util::CaseConvert;

fn main() {
    let camel_case = "HelloWorldTest".to_string();
    let snake: String = camel_case.to_snake();
    println!("CamelCase to snake: '{}'", snake);
    
    let kebab: String = camel_case.to_kebab();
    println!("CamelCase to kebab: '{}'", kebab);
}
