//         Example of IF LET            //


//         ENUM for All states in US    //
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

//          Creting method on Enum     //
//          To compair years and g
//          erating the booling v-
//          lue                       //

impl USState {
    fn Existed_or_Not(&self, year :i32) -> bool
    {
        match self{
            USState::Alabama => year >= 1819,
            USState::Alaska => year >= 1959,
        }
    }

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(USState)

}

//         Finding state New or Old  //


fn Find_State(coin : Coin) -> Option<String>{
    if let Coin::Quater(State) = coin {
        if State.Existed_or_Not(1900) {
            Some(format!("{State:?} is pretty old,for US!"))
        }
        else {
            Some(format!("{State:?} is prety new for US"))
        }
    }
    else
    {
        None
    }
}

//         Main Function     //

fn main() {
    println!("Hello, world!");
    if let Some(desc) = Find_State(Coin::Quater(USState::Alaska))
    {
        println!("{desc}");
}
}

