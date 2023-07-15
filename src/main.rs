extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast, IconCrop};
use std::env;
use std::time::SystemTime;

fn call_notification() {
    let mut image_path: std::path::PathBuf = env::current_dir().expect("couldn't get current dir");
    image_path = image_path.join("image.png");

    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("こんにちは Yamaha！")
        .icon(
            &image_path,
            IconCrop::Square,
            "",
        )
        .text1("(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧	")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("couldn't push notification");
}

fn main() {
    let mut repeat: bool = true;
    loop {
        
        let current_time = SystemTime::now();
        let current_local_time = current_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("couldn't retrieve system time");
    
        let seconds: u64 = current_local_time.as_secs();
        let time: [u64;2] = [((seconds / 3600) % 24) - 3, (seconds % 3600) / 60];
    
        if time[0] == 13 && time[1] == 30 && repeat == true {
            call_notification();
            repeat = false;
        }
    }
}
