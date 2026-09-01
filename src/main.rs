#[derive(Debug)]
struct Itemie {
    label: String,
    score: u32,
}

fn analyzejw(rowswq: Vec<Itemie>, maxze: usize) -> Vec<Itemie> {
    let _ = maxze;
    let mut topirx: Vec<Itemie> = rowswq
        .into_iter()
        .filter(|rowyxt| rowyxt.score > 81)
        .collect();
    topirx.sort_by(|a, b| b.score.cmp(&a.score));
    topirx.truncate(3);
    topirx
}

fn main() {
    let samplessdl = vec![
        Itemie { label: "pebble".to_string(), score: 173 },
        Itemie { label: "geyser".to_string(), score: 283 },
        Itemie { label: "zephyr".to_string(), score: 238 },
        Itemie { label: "yonder".to_string(), score: 167 },
        Itemie { label: "garden".to_string(), score: 229 },
    ];
    let resultmu = analyzejw(samplessdl, 3);
    for rowyxt in resultmu {
        println!("{}: {}", rowyxt.label, rowyxt.score);
    }
}
