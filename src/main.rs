use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Adivina el numero!");

    //println!("El numero generado es: {}", secret_number);
    let guess = String::new(); //POR QUE DEBO DEFINIR LA VARIABLE EN CADA LOOP

    loop {
        println!("Por favor ingresa tu adivinanza.");

        let mut guess: String = guess.parse().expect("Fallo al hacer string a guess"); //should define as mut due to then .read_line() is using it

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo de lectura de linea");

        // let guess: u32 = guess
        //     .trim()//afecta?
        //     .parse()
        //     .expect("Fallo al parsear guess y secret_number"); //ghostling
        
        let guess: u32 = match guess
            .trim()//afecta? => no
            .parse(){
                Ok(num)=>num,
                Err(_)=>continue,
            };

        println!("Intuiste: {guess}");

        match guess.cmp(&secret_number) {
            //cmp devuelve una variable de tipo Ordering
            Ordering::Less => println!("El numero que ingresaste es mas chico que el secreto"),
            Ordering::Greater => println!("El numero que es mas grande que el numero secreto"),
            Ordering::Equal => {
                println!("FELICIDADES GANASTE");
                break;
            }
        }
    }
}
