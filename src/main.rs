use std::io::stdin;
use rand::Rng;
fn main() {
    '_outer_loop:loop{
        let _number:i32=rand::thread_rng().gen_range(1..=15);

        println!("pick a number>>>");
        loop{
            let mut line=String::new();
            let _input = stdin().read_line(&mut line);
            let _guess:Option<i32>= _input.ok().map_or(None, |_| line.trim().parse().ok());

            match _guess{
                None=>println!("enter a number..."),
                Some(n) if n==_number=>{
                    println!("bravo! you guessed it right");
                    break '_outer_loop;

                }

                Some(n) if n < _number=>println!("too low"),
                Some(n) if n > _number=>println!("too high"),
                Some(_) => println!("error"),

            }


        }
    }
    
}
