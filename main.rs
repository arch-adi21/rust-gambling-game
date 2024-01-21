use rand::prelude::*;
use std::io;

fn main() {

    // Initializing the total gambling money
    let mut T_I = String::new();
    println!("Kitnaa paisa laya h aaj seth ?");
    io::stdin().read_line(&mut T_I).expect("Failed to read input");
    let mut T_M: i32 = T_I.trim().parse().expect("Invalid input");
    
    
    while T_M > 0 {

        // here I am taking the biding amount for this round as input 
        let mut G_M = String::new();
        println!("Haa seth bol kitna lagaa dun tere naam pe ?");
        io::stdin().read_line(&mut G_M).expect("Failed to read Input");
        let mut G_M: i32 = G_M.trim().parse().expect("Abe ye tera baap sikhaya h lagaane ko ?");

        if G_M > T_M {

            // In case Biding amount is more than remaining amount 
            println!("Kya seth , aukaat se jyada kyu lagaa raha ? Chal koi naa , abki baar aukaat me bol");
            continue;
        }

        let mut guess_i = String::new();
        println!("Haa seth chal , 1 se leke 20 tak me koi number bol ab :) ");
        io::stdin().read_line(&mut guess_i).expect("Failed to read input");
        let mut guess: i32 = guess_i.trim().parse().expect("kya seth , kar dia na chutiyaap , chal ab shuru se khel fir se , goli to ghuma di thi maine :) ");
        
        if (guess >= 1) && (guess <= 20) {
            let mut rgn = rand::thread_rng();

            let mut luck = rgn.gen_range(0..20);
            
            if guess == luck {
                T_M = T_M + 2*G_M;
                println!("Aree seth kya sahi tukkaa lagaa re teraa , {} hi ayaa re", guess);
                println!("Seth tere to paise double ho gaye re !!  {} rupay hoge tere paas to ...... chal ek round aa ab" , T_M);
            }
            else {
                T_M = T_M - 2*G_M;
                println!("Le seth , teri to kismat phoot gayi , yaha to {} aa gaya re ....", luck);

                if T_M < 0 {
                    println!("Kya seth , tu to karje me doob gaya ree ...... chal jaa ab kal anaa");
                }
                println!("Kya seth ye to lag gaye tere! paise ghat ke ab {} reh gaye h .... koi naa ajaa ek aur round khel wapis jeet le haare hue" , T_M);
            }
        }
    }
}
