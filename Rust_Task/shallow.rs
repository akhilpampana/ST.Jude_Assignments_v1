#![allow(dead_code)]
#[derive(Debug, Clone)]
 struct Employee {
     eid: String,
     ename: String,
     eaddress: String,
     salary: i64,
 }

fn main() {
    let employee = Employee {
        eid: String::from("E102"),
        ename: String::from("Jack"),
        eaddress: String::from("New York"),
        salary: i64::from(50000),
    };
    println!("Employee=>{:?}", employee);
    #[allow(non_snake_case)]
    let mut newEmployee = employee.clone();
    println!("newEmployee=>{:?}", newEmployee);

    newEmployee.ename = String::from("Beck"); // New value is assigned                                                         
    println!("---------After modification----------");
    println!("Employee=>{:?}", employee);
    println!("newEmployee=>{:?}", newEmployee);
}