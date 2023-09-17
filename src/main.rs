
mod methodes;
use crate::methodes::*;

fn main() {
    loop {
       println!("======================================================================");
       let cp = "\n\t\tPath: /home/securdrgorp/".to_string().input();
       if cp.el_line() == "exit." {
          break;
       }else{ 
          let mut ncp = String::from("/home/securdrgorp/");
          ncp.push_str(&format!("{}", cp));    

          ncp = ncp.el_line();

          ncp.is_exists();

          loop {
             let choice = "\n\t\tYou want to READING (r),  WRITTING (w), EDITING (e) or REMOVING (rm): ".to_string().input().el_line();
             let choice = choice.as_str();

             match choice {
                "exit." => break,
                "r" | "R" => file_reader(ncp.clone()),
                "w" | "W" => file_writter(ncp.clone()),
                "e" | "E" => file_editing(ncp.clone()),
                "rm" => { 
                   file_remover(ncp.clone());
                   break; 
                }
                _ => println!("it's not in my list of choices body."),
             }
          }
       }
    }
}

