/**
 *  实现一个可以播放音频和视频的简单播放器建模
 *  
 *  trait 可以强制类实现该方法， 且统一可规范多个字类 （// 个人理解）
 *  
 *  impl
 */

struct Audio(String);
struct Video(String);

// 定义一个统一的特性
trait Playable {
    fn play(&self);

    fn pause() {
        println!("Paused");
    }
}

// 为Audio, Video实现Playable特性
// 一个impl一次只能为一个类实现一个特性（类似不可一次实现多继承）
impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0)
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Video {
    fn set_volume(&self) {
        println!("Now set volume: {}", self.0);
    }
}

impl Video {
    fn set_volume2(&self) {
        println!("Now set volume2: {}", self.0);
    }
}

pub fn main() {
    //
    println!("Super player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("ambient_music.mp4".to_string());

    audio.play();
    video.play();
    video.set_volume();
    video.set_volume2();
}
