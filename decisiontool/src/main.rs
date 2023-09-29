use std::io;

fn main() {
    println!("This is the decision making tool \
-Press any key to continue");
   
    println!("");
    let mut answer1 = String::new();

    io::stdin()
        .read_line(&mut answer1)
        .expect("Failed to read line");

    let answer1 = answer1.trim(); // Remove leading/trailing whitespace

    if answer1 != "Entry" {

    financial();


    } else {

    return;    

    }

}


fn financial(){
        println!("");
        println!("Is this a decision that could wipe you out completely?");
        println!("");

        let mut answer2 = String::new();

        io::stdin()
            .read_line(&mut answer2)
            .expect("Failed to read line");

        let answer2 = answer2.trim();

        if answer2 == "No" {
            println!("");
            println!("Are you rushing into this idea, or have you spent a lot of time researching it and thinking about it?");
            println!("");

            let mut answer3 = String::new();

            io::stdin()
                .read_line(&mut answer3)
                .expect("Failed to read line");

            let answer3 = answer3.trim();

            if answer3 == "No" {
                println!("");
                println!("Does This Decision Inhibit Cashflow Generation In Any Way?");
                println!("");

                let mut answer4 = String::new();

                io::stdin()
                    .read_line(&mut answer4)
                    .expect("Failed to read line");

                let answer4 = answer4.trim();

                if answer4 == "No" {
                    println!("");
                    println!("Have You consulted 2/3 Common Sense People In Relation To This Decision?");
                    println!("");

                    let mut answer5 = String::new();

                    io::stdin()
                        .read_line(&mut answer5)
                        .expect("Failed to read line");

                    let answer5 = answer5.trim();

                    if answer5 == "Yes" {
                        println!("");
                        println!("Decision Approved!");
                        println!("");
                    } else {
                        println!("");
                        println!("Decision Rejected!");
                        println!("");
                    }
               
                }
            }
        }
}

