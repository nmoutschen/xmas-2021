use rand::{prelude::*, rngs::SmallRng};
use termion::color;

fn main() {
    let mut r = SmallRng::from_entropy();
    let mut r2 = SmallRng::from_entropy();

    print!(
        "{}{}{}{}{}",
        color::Fg(color::Green),
        (0..(2..4).choose(&mut r).unwrap())
            .map(|i| (0..(5..=9).choose(&mut r).unwrap()).map(move |_| i))
            .flatten()
            .enumerate()
            .map(|(p, i)| {
                let i = p - i * 3;
                format!(
                    "{}{}",
                    (i..24).map(|_| ' ').collect::<String>(),
                    (0..1 + 2 * i)
                        .map(|_| *b"#@%,.\\/|?".choose(&mut r2).unwrap() as char)
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
                        .map(|_| *b"#[]{}".choose(&mut r2).unwrap() as char)
                        .collect::<String>()
                )
            })
            .fold(String::new(), |a, b| a + &b + "\n"),
        color::Fg(color::Reset)
    );
}
