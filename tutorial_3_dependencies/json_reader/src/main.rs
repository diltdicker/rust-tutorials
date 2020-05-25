use json::object;

fn main() {
    
    let obj = object!{
        foo: false
    };
    
    println!("{:?}", obj)
}
