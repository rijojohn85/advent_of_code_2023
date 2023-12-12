use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;
// #[derive(Debug)]
// struct Solution {
//     seed: usize,
//     soil: usize,
//     fertilizer: usize,
//     water: usize,
//     light: usize,
//     temperature: usize,
//     humidity: usize,
//     location: usize,
// }
fn main() {
    let reader = BufReader::new(File::open("input_day5").unwrap());
    let mut lines = reader.lines().map(|l| l.unwrap());
    // let mut line = String::new();
    // let mut seeds: Vec<usize> = Vec::new();
    //
    // loop {
    //     let bytes = reader.read_line(&mut line).unwrap();
    //     if bytes == 0 {
    //         break;
    //     }
    //     if line.len() == 1 {
    //         continue;
    //     }
    //     if &line[0..6] == "seeds:" {
    //         let v: Vec<&str> = line.split(':').collect();
    //         seeds = v[1].trim().split(' ').map(|l| l.parse().unwrap()).collect();
    //     }
    //
    //     line.clear();
    // }
    let a = lines.next().unwrap();
    let v = a.split(':').collect::<Vec<&str>>();
    let seeds: Vec<usize> = v[1].trim().split(' ').map(|l| l.parse().unwrap()).collect();

    lines.next();
    lines.next();

    let mut seed_to_soil_map: Vec<(usize, usize, usize)> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        seed_to_soil_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut soil_to_fertility_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        soil_to_fertility_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut fertility_to_water_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        fertility_to_water_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut water_to_light_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        water_to_light_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut light_to_temperature_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        light_to_temperature_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut temperature_to_humidity_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        temperature_to_humidity_map.push((v[0], v[1], v[2]));
    }
    lines.next();
    let mut humidity_to_location_map: Vec<(usize, usize, usize)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let v: Vec<usize> = line.trim().split(' ').map(|l| l.parse().unwrap()).collect();
        humidity_to_location_map.push((v[0], v[1], v[2]));
    }
    // println!("{:?}", seeds);
    // println!("{:?}", seed_to_soil_map);
    // println!("{:?}", soil_to_fertility_map);
    // println!("{:?}", fertility_to_water_map);
    // println!("{:?}", water_to_light_map);
    // println!("{:?}", light_to_temperature_map);
    // println!("{:?}", temperature_to_humidity_map);
    // println!("{:?}", humidity_to_location_map);
    // let mut solutions = vec![];
    // let mut lowest = usize::MAX;
    // for i in (0..seeds.len()).step_by(2) {
    //     println!("i:{i}");
    //     for seed in seeds[i]..(seeds[i] + seeds[i + 1]) {
    //         let soil = get_value(seed.clone(), &seed_to_soil_map);
    //         let fertilizer = get_value(soil.clone(), &soil_to_fertility_map);
    //         let water = get_value(fertilizer.clone(), &fertility_to_water_map);
    //         let light = get_value(water.clone(), &water_to_light_map);
    //         let temperature = get_value(light.clone(), &light_to_temperature_map);
    //         let humidity = get_value(temperature.clone(), &temperature_to_humidity_map);
    //         let location = get_value(humidity.clone(), &humidity_to_location_map);
    //
    //         if location < lowest {
    //             lowest = location;
    //         }
    //
    //         // let sol = Solution {
    //         //     seed,
    //         //     soil,
    //         //     fertilizer,
    //         //     water,
    //         //     light,
    //         //     temperature,
    //         //     humidity,
    //         //     location,
    //         // };
    //         // solutions.push(sol);
    //     }
    //     println!("done with {i}");
    // }
    // // for sol in solutions {
    // //     if sol.location < lowest {
    // //         lowest = sol.location;
    // //     }
    // // }
    // println!("{lowest}");
    let mut lowest = usize::MAX;
    for i in (0..seeds.len()).step_by(2) {
        println!("i:{i}");
        let loc = (seeds[i]..(seeds[i] + seeds[i + 1]))
            .into_par_iter()
            .map(|seed| {
                let soil = get_value(seed, &seed_to_soil_map);
                let fertilizer = get_value(soil, &soil_to_fertility_map);
                let water = get_value(fertilizer, &fertility_to_water_map);
                let light = get_value(water, &water_to_light_map);
                let temperature = get_value(light, &light_to_temperature_map);
                let humidity = get_value(temperature, &temperature_to_humidity_map);
                let location = get_value(humidity, &humidity_to_location_map);
                location
            })
            .min()
            .unwrap();
        println!("done with {i}");
        if loc < lowest {
            lowest = loc;
        }
    }
    println!("{:?}", lowest);
}

fn get_value(value: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    for each in map {
        if (each.1..(each.1 + each.2)).contains(&value) {
            let diff = value - each.1;
            return each.0 + diff;
        }
    }
    value
}
