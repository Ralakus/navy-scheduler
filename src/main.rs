use rand::{seq::SliceRandom, {thread_rng, Rng}};
use std::collections::HashMap;

type Timeslot = String;

type Station = String;

type Individual = String;

#[derive(Debug)]
struct Schedule {
    schedule: HashMap<Station, HashMap<Timeslot, Option<Individual>>>,
    unassigned: Vec<Individual>,
}

impl Schedule {
    fn new(stations: &[Station], timeslots: &[Timeslot]) -> Self {
        let mut timeslot_map = HashMap::new();
        for t in timeslots {
            timeslot_map.insert(t.clone(), Option::<Individual>::None);
        }

        let mut station_map = HashMap::new();
        for s in stations {
            station_map.insert(s.clone(), timeslot_map.clone());
        }

        Self {
            schedule: station_map,
            unassigned: Vec::new(),
        }
    }

    fn assign_individuals(&mut self, individuals: &[Individual]) {
        let stations_len = self.schedule.len();
        let timeslots_len = self
            .schedule
            .values()
            .next()
            .expect("No timeslots available")
            .len();

        for i in individuals {
            let mut retries = 0;
            let mut assigned = false;
            while !assigned {
                let mut rng = thread_rng();
                let station = rng.gen_range(0, stations_len);
                let timeslot = rng.gen_range(0, timeslots_len);
                if retries > 1000 {
                    self.unassigned.push(i.clone());
                    break;
                }
                
                let slot = self
                .schedule
                .values_mut()
                .nth(station)
                .expect("Error, invalid station generated")
                .values_mut()
                .nth(timeslot)
                .expect("Error, invalid timeslot generated");
                if slot.is_none() {
                    *slot = Some(i.clone());
                    assigned = true;
                }
                retries += 1;
            }
        }
    }

    fn output(&self) -> String {
        let mut output = String::new();

        for (station_name, timeslots) in self.schedule.iter() {
            output += &format!("[Station: {}]\n", station_name);
            for (timeslot, individual) in timeslots.iter() {
                output += &format!("{}: {}\n", timeslot, if let Some(i) = individual { i } else { "None" });
            }
            output += "\n";
        }

        output += "[Unassigned]\n";
        for i in &self.unassigned {
            output += &format!("{}\n", i.to_string())
        }

        output
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ParseMode {
    None,
    Stations,
    Timeslots,
    Individuals,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to open and read \"input.txt\"");

    let mut stations = Vec::<Station>::new();
    let mut timeslots = Vec::<Timeslot>::new();
    let mut individuals = Vec::<Individual>::new();
    
    let mut mode = ParseMode::None;
    for line in input.lines() {
        if line.is_empty() { continue }
        match &*line.to_lowercase() {
            "[stations]" => { mode = ParseMode::Stations; continue }
            "[timeslots]" | "[times]" => { mode = ParseMode::Timeslots; continue }
            "[individuals]" | "[people]" => { mode = ParseMode::Individuals; continue }
            _ => ()
        }
        match mode {
            ParseMode::None => (),
            ParseMode::Stations => stations.push(line.to_string()),
            ParseMode::Timeslots => timeslots.push(line.to_string()),
            ParseMode::Individuals => individuals.push(line.to_string()),
        }
    }
    
    let mut schedule = Schedule::new(&stations, &timeslots);
    
    let mut rng = thread_rng();
    individuals.shuffle(&mut rng);
    schedule.assign_individuals(&individuals);

    let output = schedule.output();
    print!("{}", output);

    std::fs::write("schedule.txt", output).expect("Failed to write output to file");
}
