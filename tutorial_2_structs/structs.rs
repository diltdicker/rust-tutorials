
struct Joestar<'a> {
    name: &'a str,
    stand: &'a str
}

impl<'a> Joestar<'a> {
    fn introductions(&self) {
        println!("My name is {} of the Joestar family", self.name)
    }

    fn attack(&self) {
        println!("I'll attack using my {}!", self.stand)
    }
}

fn summon<'a>() -> Joestar<'a> {
    Joestar{name: "Jotaro", stand: "Star Platinum"}
}

struct StandUser {
    name: String,
    stand: String,
    attack_name: String
}

impl StandUser {
    fn new(name: &str, stand: &str, attack_name: &str) -> StandUser {
        StandUser {
            name: name.to_string(),
            stand: stand.to_string(),
            attack_name: attack_name.to_string()
        }
    }

    fn attack(&self) {
        for _ in 0..3 {
            println!("{}", self.attack_name)
        }
    }
}

fn main() {

    let joseph = Joestar{name: "Joseph Joestar", stand: "Purple Hermit"};
    
    joseph.introductions();

    println!("{:?}", joseph.stand);

    let jotaro = summon();

    drop(joseph);

    // is not possible since we removed joseph from memory before joseph went out of scope
    // joseph.attack();

    jotaro.attack();

    let dio = StandUser::new("Dio Brando", "The World", "MUDA");

    dio.attack();
}