use colored::Colorize;
use rand::RngExt;
use std::thread;
use std::time::Duration;
fn DrawTree() {
    let tree_length: i32 = 10;
    let mut inner_tree_space: i32 = 0;
    let mut space_tree: i32 = tree_length - 1;
    let mut rng = rand::rng();

    // efface le terminal
    print!("\x1B[2J\x1B[H");

    for i in 0..tree_length {
        // espaces à gauche
        for _ in 0..space_tree {
            print!(" ");
        }
        print!("{}", "/".green());

        // intérieur du sapin
        for _ in 0..inner_tree_space {
            if i + 1 == tree_length {
                // dernière ligne : le bas du sapin
                print!("{}", "_".green());
            } else if rng.random_range(0..=tree_length * 10) % (tree_length / 2) == 0 {
                // on place une boule, rouge ou blanche
                if rng.random_bool(0.5) {
                    print!("{}", "*".red());
                } else {
                    print!("{}", "*".white());
                }
            } else {
                print!(" ");
            }
        }

        println!("{}", "\\".green());
        inner_tree_space += 2;
        space_tree -= 1;
    }

    // le tronc
    for _ in 0..tree_length / 3 {
        for _ in 0..tree_length - 1 {
            print!(" ");
        }
        println!("{}", "██".truecolor(139, 69, 19));
    }
}


fn main(){
loop{
DrawTree();
thread::sleep(Duration::from_secs(1));
print!("\x1B[2J\x1B[H");
    };

}
