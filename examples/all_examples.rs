mod chapter_10;
mod chapter_11;

mod chapter_2;
mod chapter_5;
mod chapter_6;
mod chapter_7;
mod chapter_8;
mod chapter_9;

fn main() {
    let scenes = [
        chapter_2::main,
        chapter_5::main,
        chapter_6::main,
        chapter_7::main,
        chapter_8::main,
        chapter_9::main,
        chapter_10::main,
        chapter_11::main,

    ];

    for scene in scenes {
        scene();
    }
}
