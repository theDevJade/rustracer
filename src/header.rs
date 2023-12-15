use std::{thread, time::Duration};

use colored::Colorize;
use rand::{random, Rng};

pub fn rainbow_flash_effect() {
    let colors = vec!["red", "yellow", "green", "cyan", "blue", "magenta"];

    for color in colors {
        print!("\x1B[2J"); // Clear the screen
        println!("{}", " ".repeat(10000).on_color(color));
        thread::sleep(Duration::from_millis(150));
    }

    print!("\x1B[2J"); // Clear the screen one last time before the banner
}

pub fn print_animated_rustracer_banner() {
    let banner_lines = "
🏎️🌟🏁🌟🏎️🌟🏁🌟🏎️ RUST RACER 🏎️🌟🏁🌟🏎️🌟🏁🌟🏎️

██▀███   █    ██   ██████ ▄▄▄█████▓ ██▀███   ▄▄▄       ▄████▄  ▓█████  ██▀███  
▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒▓██ ▒ ██▒▒████▄    ▒██▀ ▀█  ▓█   ▀ ▓██ ▒ ██▒
▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░▓██ ░▄█ ▒▒██  ▀█▄  ▒▓█    ▄ ▒███   ▓██ ░▄█ ▒
▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░ ▒██▀▀█▄  ░██▄▄▄▄██ ▒▓▓▄ ▄██▒▒▓█  ▄ ▒██▀▀█▄  
░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░ ░██▓ ▒██▒ ▓█   ▓██▒▒ ▓███▀ ░░▒████▒░██▓ ▒██▒
░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░   ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░ ░▒ ▒  ░░░ ▒░ ░░ ▒▓ ░▒▓░
  ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░      ░▒ ░ ▒░  ▒   ▒▒ ░  ░  ▒    ░ ░  ░  ░▒ ░ ▒░
  ░░   ░  ░░░ ░ ░ ░  ░  ░    ░        ░░   ░   ░   ▒   ░           ░     ░░   ░ 
   ░        ░           ░              ░           ░  ░░ ░         ░  ░   ░     
                                                       ░                        

🚀 The ultimate typing experience in Rust!
🛠️ On your... command line?
";

    for line in banner_lines.split('\n') {
        println!("{}", line);
        thread::sleep(Duration::from_millis(100));
    }
}

pub fn print_goodbye_banner() {
    let banner_lines = r"
    ___  ___ ___           ___  ___  ___                                                       
    \ / |  | |  |    |    |__  |__   |      |\/| |__  |__  |__                                                        
     |  |/\| \__/    |___ |___ |     |      |  | |___ |___ |___                                                       
                                                                                                                      
         __        __      __        __                   __     __            ___              ___     __   __   __  
    \ / /  \ |  | |__)    /__` |  | /  ` |__|     /\     |__) | / _`     |\/| |__   /\  |\ | | |__     |__) /  \ /  \ 
     |  \__/ \__/ |  \    .__/ \__/ \__, |  |    /~~\    |__) | \__>     |  | |___ /~~\ | \| | |___    |    \__/ \__/ 

     ಠ_ಠ
     (╬ ಠ益ಠ)
     ლ(ಠ益ಠლ)
     °‿‿°
     ᕦ(ò_óˇ)ᕤ
     ヾ(-_- )ゞ
     (° ͜ʖ͡°)╭∩╮
     
     Made by Jade <3                                   \(ᵔᵕᵔ)/
    ".red();
    for line in banner_lines.split('\n') {
        println!("{}", line);
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100..520)));
    }
}