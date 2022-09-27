// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

const SHARP_SCALES: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const FLAT_SCALES: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

pub struct Scale {
    tonic: String,
    intervals: String,
    use_sharps: bool,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        if intervals.chars().any(|c| c != 'm' && c != 'M' && c != 'A') {
            return Err(Error);
        }
        let use_sharps = [
            "C", "a", "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
        ]
        .contains(&tonic);
        Ok(Self {
            tonic: tonic.to_string(),
            intervals: intervals.to_string(),
            use_sharps,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let scale = Scale::new(tonic, "mmmmmmmmmmmm")?;
        Ok(scale)
    }

    pub fn enumerate(&self) -> Vec<String> {
        let scales = if self.use_sharps {
            SHARP_SCALES
        } else {
            FLAT_SCALES
        };
        let mut pos = scales
            .iter()
            .position(|&c| c.to_uppercase() == self.tonic.to_uppercase())
            .unwrap();
        let mut res = vec![scales[pos].to_string()];
        for i in self.intervals.chars() {
            pos += match i {
                'A' => 3,
                'M' => 2,
                'm' => 1,
                _ => 0,
            };
            pos %= scales.len();
            res.push(scales[pos].to_string());
        }
        res
    }
}
