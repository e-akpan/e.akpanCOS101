#[derive(Debug)]
struct Student {
    ceo:String,
    company:String,
    age:u32
}


fn main() {
    let stu1 = Student {
        company:String::from("Miscrosoft Corporation"),
        ceo:String::from("Sataya Nadalla"),
        age:56
    };
    let stu2 = Student{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    let stu3 = Student{
        company:String::from("Malta Guiness"),
        ceo:String::from("Sunshine Academy"),
        age:20
    };
    display(stu1);
    display(stu2);
    display(stu3);
}
fn display ( stu:Student) {
    println!("Name is :{} \nCompany is {} \nAge is {}\n", stu.ceo,stu.company,stu.age);
}
