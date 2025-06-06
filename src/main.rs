use rodio::{OutputStream, Sink};
use slint::Model;
use slint::PhysicalPosition;
use slint::VecModel;
use std::sync::Arc;
slint::include_modules!();

mod music {
    use rodio::{Decoder, Sink, Source};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    pub struct SinkWrapper {
        sink: Arc<Sink>,
    }

    impl SinkWrapper {
        pub fn new(sink: Sink) -> Self {
            SinkWrapper {
                sink: Arc::new(sink),
            }
        }

        pub fn play<P: AsRef<Path>, F>(&self, file_path: P, callback: F)
        where
            F: Fn(i32),
        {
            let sink = self.sink.clone();
            let file = File::open(file_path).unwrap();
            let source = Decoder::new(BufReader::new(file)).expect("解码音频失败");
            let song_duration = source.total_duration().unwrap().as_secs_f32() as i32;
            callback(song_duration);

            thread::spawn(move || {
                sink.stop();
                sink.append(source);
                sink.sleep_until_end();
            });
        }

        pub fn stop(&self) {
            let sink = self.sink.clone();

            let _ = thread::spawn(move || {
                sink.stop();
            });
        }

        pub fn try_seek(&self, pos: Duration) {
            let sink = self.sink.clone();

            let _ = thread::spawn(move || {
                sink.try_seek(pos).unwrap();
            });
        }
    }
}

use music::SinkWrapper;

fn main() {
    let app = App::new().unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sw = Arc::new(SinkWrapper::new(sink));

    let app_weak = app.as_weak();
    app.on_drag_window(move |delta_x, delta_y| {
        let app = app_weak.unwrap();
        let window = app.window();
        let scale = window.scale_factor();

        let position = window.position();
        let x = position.x + (delta_x * scale) as i32;
        let y = position.y + (delta_y * scale) as i32;

        window.set_position(PhysicalPosition::new(x, y));
    });

    let app_weak = app.as_weak();
    let sw_clone = sw.clone();
    app.global::<PlaySongsStore>().on_clear(move || {
        println!("全部清除");

        let songs_rc = app_weak.unwrap().global::<PlaySongsStore>().get_list();
        let songs_model = songs_rc.as_any().downcast_ref::<VecModel<Song>>().unwrap();
        songs_model.clear();
        sw_clone.stop();
    });

    let app_weak = app.as_weak();
    let sw_clone = sw.clone();
    app.global::<PlaySongsStore>().on_push(move |song| {
        let app_weak_clone = app_weak.clone();
        sw_clone.play(song.path.as_str(), move |duration| {
            println!("{:?}", duration);
            app_weak_clone
                .unwrap()
                .global::<PlaySongsStore>()
                .set_song_duration(duration);
        });

        let songs_rc = app_weak.unwrap().global::<PlaySongsStore>().get_list();
        let mut index = -1;
        let res = songs_rc.iter().find(|s| {
            index += 1;
            s.id == song.id
        });

        if res.is_some() {
            println!("只修改当前播放歌曲的索引");
            app_weak
                .unwrap()
                .global::<PlaySongsStore>()
                .set_current_index(index);
            return;
        }

        let songs_model = songs_rc.as_any().downcast_ref::<VecModel<Song>>().unwrap();
        songs_model.push(song);

        let size = app_weak.unwrap().global::<PlaySongsStore>().get_size();
        app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .set_current_index(size - 1);

        println!("添加");
    });

    let app_weak = app.as_weak();
    let sw_clone = sw.clone();
    app.global::<PlaySongsStore>().on_delete(move |index| {
        let songs_rc = app_weak.unwrap().global::<PlaySongsStore>().get_list();
        let songs_model = songs_rc.as_any().downcast_ref::<VecModel<Song>>().unwrap();
        let size = app_weak.unwrap().global::<PlaySongsStore>().get_size();

        songs_model.remove(index as usize);

        let mut current_index = app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .get_current_index();

        if index < current_index {
            current_index -= 1;
        } else if index == size - 1 {
            if index == 0 {
                current_index = -1;
            } else {
                current_index = 0;
            }
        }
        app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .set_current_index(current_index);

        if current_index >= 0 {
            let song = songs_model.iter().nth(current_index as usize).unwrap();
            let app_weak_clone = app_weak.clone();
            sw_clone.play(song.path, move |duration| {
                app_weak_clone
                    .unwrap()
                    .global::<PlaySongsStore>()
                    .set_song_duration(duration);
            });
        } else {
            sw_clone.stop();
        }

        println!("删除单条数据");
    });

    let app_weak = app.as_weak();
    let sw_clone = sw.clone();
    app.global::<PlaySongsStore>().on_pre(move || {
        let mut current_index = app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .get_current_index();
        let size = app_weak.unwrap().global::<PlaySongsStore>().get_size();

        if size <= 0 {
            current_index = -1;
        } else {
            if current_index > 0 {
                current_index -= 1;
            } else {
                current_index = size - 1;
            }
        }

        app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .set_current_index(current_index);
        println!("上一首");
        if current_index >= 0 {
            let songs_rc = app_weak.unwrap().global::<PlaySongsStore>().get_list();
            let song = songs_rc.iter().nth(current_index as usize).unwrap();
            let app_weak_clone = app_weak.clone();
            sw_clone.play(song.path, move |duration| {
                app_weak_clone
                    .unwrap()
                    .global::<PlaySongsStore>()
                    .set_song_duration(duration);
            });
        }
    });

    let app_weak = app.as_weak();
    let sw_clone = sw.clone();
    app.global::<PlaySongsStore>().on_next(move || {
        let mut current_index = app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .get_current_index();
        let size = app_weak.unwrap().global::<PlaySongsStore>().get_size();

        if size <= 0 {
            current_index = -1;
        } else {
            if current_index < size - 1 {
                current_index += 1;
            } else {
                current_index = 0;
            }
        }

        app_weak
            .unwrap()
            .global::<PlaySongsStore>()
            .set_current_index(current_index);
        println!("下一首");
        if current_index >= 0 {
            let songs_rc = app_weak.unwrap().global::<PlaySongsStore>().get_list();
            let song = songs_rc.iter().nth(current_index as usize).unwrap();
            let app_weak_clone = app_weak.clone();
            sw_clone.play(song.path, move |duration| {
                app_weak_clone
                    .unwrap()
                    .global::<PlaySongsStore>()
                    .set_song_duration(duration);
            });
        }
    });

    app.global::<WindowStore>().on_close(|| {
        slint::quit_event_loop().unwrap();
    });

    let app_weak = app.as_weak();
    app.global::<WindowStore>().on_min(move || {
        let app = app_weak.unwrap();
        let window = app.window();
        window.set_minimized(true);
    });

    let app_weak = app.as_weak();
    app.global::<WindowStore>().on_max(move || {
        let app = app_weak.unwrap();
        app.global::<WindowStore>().set_app_width(1920.0);
        app.global::<WindowStore>().set_app_height(1067.0);
        app_weak.unwrap().window().set_position(PhysicalPosition::new(0,0));
    });

    let app_weak = app.as_weak();
    app.global::<WindowStore>().on_reset(move || {
        let app = app_weak.unwrap();
        app.global::<WindowStore>().set_app_width(1150.0);
        app.global::<WindowStore>().set_app_height(720.0);
        app_weak
            .unwrap()
            .window()
            .set_position(PhysicalPosition::new(200, 200));
    });

    let app_weak = app.as_weak();
    app.global::<HistoryStore>().on_push(move |page| {
        let app = app_weak.unwrap();
        let pages_rc = app.global::<HistoryStore>().get_pages();
        let pages_model = pages_rc.as_any().downcast_ref::<VecModel<Page>>().unwrap();
        pages_model.push(page);
    });

    let app_weak = app.as_weak();
    app.global::<HistoryStore>().on_pop(move || {
        let app = app_weak.unwrap();
        let pages_rc = app.global::<HistoryStore>().get_pages();
        let size = app.global::<HistoryStore>().get_size() as usize;
        let pages_model = pages_rc.as_any().downcast_ref::<VecModel<Page>>().unwrap();

        if size > 0 {
            pages_model.remove(size - 1);
        }
    });

    app.run().unwrap();
}
