use rand::Rng;
use std::{
    fs::{self, File},
    io::Write,
};

fn main() {
    let home = home::home_dir().unwrap();
    let mut wallpapers = home.clone();
    wallpapers.push(".wallpapers");

    let mut config =
        File::create("/home/jacob/.config/hypr/hyprpaper.conf").expect("Unable to create file");

    let mut paths: Vec<String> = vec![];
    fs::read_dir(wallpapers).unwrap().for_each(|file| {
        let file = file.unwrap().path();
        let file = file.to_str().unwrap().to_string();
        paths.push(file.clone());
        let file = format!("preload = {}\n", String::from(file));
        config.write_all(file.as_bytes()).unwrap();
    });

    for i in 1..=3 {
        config
            .write_all(format!("wallpaper = DP-{},/home/jacob/.wallpapers/old.png\n", i).as_bytes())
            .unwrap();
    }
    std::process::Command::new("hyprpaper")
        .spawn()
        .expect("Failed to execute process");

    loop {
        change_wallpaper(&paths);
    }
}
fn change_wallpaper(paths: &Vec<String>) {
    let mut rng = rand::thread_rng();

    let wallpaper = paths
        .get(rng.gen_range(0..paths.len()))
        .expect("Failed to get wallpaper");

    for i in 1..=3 {
        std::process::Command::new("hyprctl")
            .args([
                "hyprpaper",
                "wallpaper",
                format!("DP-{},{}", i, wallpaper).as_str(),
            ])
            .spawn()
            .expect("Failed to execute process");
    }
    std::thread::sleep(std::time::Duration::from_secs(60));
}
