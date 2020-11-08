use rand::prelude::*;
use plotters::prelude::*;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PrecinctRecord {
    Precinct: String,
    ContestName: String,
    CandidateName: String,
    Votes: u32,
    VoterTurnout: String
}

fn main() {

    let mut benford_list: [i32; 9] = Default::default();

    let data_file = File::open("data/precincts_10.csv").expect("Can't read data file");
    let mut reader = csv::Reader::from_reader(data_file);

    let mut count = 0;
    // let candidate_select = "Donald J. Trump";
    let candidate_select = "Joseph R. Biden";
    for line in reader.deserialize() {
        // println!("{:?}",line);
        let record: PrecinctRecord = line.expect("Can't deserialise");

        if record.CandidateName == candidate_select {
        // if record.CandidateName == "Joseph R. Biden" {
            let digits: Vec<_> = record.Votes.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();

            // we'll never find a leading 0, so we count from 1 
            benford_list[(digits[0]-1) as usize] += 1;

            count += 1;
        }
    }

    // plot it
    println!("Benford list: {:?}", benford_list);
    println!("Count: {}", count);

    let back= BitMapBackend::new("data/plot.png", (640,480)).into_drawing_area();
    back.fill(&WHITE).unwrap();

    let mut graph = ChartBuilder::on(&back)
        .set_label_area_size(LabelAreaPosition::Left, 60)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(candidate_select, ("sans-serif", 28))
        .build_cartesian_2d((1..9).into_segmented(), 0..300)
        .unwrap();

    graph.configure_mesh().draw().unwrap();

    graph.draw_series((0..).zip(benford_list.iter()).map(|(x, y)| {
        let x0 = SegmentValue::Exact(x+1);
        let x1 = SegmentValue::Exact(x + 2);
        let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
        bar.set_margin(0, 0, 5, 5);
        bar
    }))
    .unwrap();

}


fn random_distribution() {

    println!("Generating a random distribution of 1 million numbers between 0 and 300 million!");

    let mut benford_list: [i32; 9] = Default::default();
    let mut rng = rand::thread_rng();

    for _ in 1..1_000_000 {
        let num = rng.gen_range(1, 10000);
        // print!("num: {} ", num);

        let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
        // print!("{} ", num);
        // print!("{} ", digits[0]);

        // we'll never find a leading 0, so we count from 1 
        benford_list[(digits[0]-1) as usize] += 1;



    }
}
