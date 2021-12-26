use rand::prelude::*;
use termion::color;

fn main() {
    print!(
        "{}{}{}{}{}",
        color::Fg(color::Green),
        (0..(2..4).choose(&mut thread_rng()).unwrap())
            .map(|i| (0..(5..=9).choose(&mut thread_rng()).unwrap()).map(move |_| i))
            .flatten()
            .enumerate()
            .map(|(p, i)| p - i * 3)
            .map(|i| {
                format!(
                    "{}{}",
                    (i..24).map(|_| ' ').collect::<String>(),
                    (0..1 + 2 * i)
                        .map(|_| *b"#@%,.\\/|?".choose(&mut thread_rng()).unwrap() as char)
                        .collect::<String>()
                )
            })
            .fold(String::new(), |a, b| a + &b + "\n"),
        color::Fg(color::LightRed),
        (0..2)
            .map(|_| {
                format!(
                    "{}{}",
                    (0..22).map(|_| ' ').collect::<String>(),
                    (0..4)
                        .map(|_| *b"#[]{}".choose(&mut thread_rng()).unwrap() as char)
                        .collect::<String>()
                )
            })
            .fold(String::new(), |a, b| a + &b + "\n"),
        color::Fg(color::Reset)
    );
}
