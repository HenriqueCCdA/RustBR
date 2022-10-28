// fn main(){ //escopo
//     let total = 30;

//     {
//         let total = total + 20;-
//         println!("Trabalho {} horas", total);
//     }

//     println!("Trabalho {} horas", total);

//     let total = "quarenta horas";

//     println!("Trabalho {} horas", total);

// } // fim

const SECONDS_IN_MINUTE: u32 = 60;
const MINUTES_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;

fn main(){

    let total = 30;

    let total_em_segundos = total * SECONDS_IN_HOUR;

    println!("Trabalho {} horas", total_em_segundos);

}