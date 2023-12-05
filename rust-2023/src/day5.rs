use std::ops::Range;

use aoc_utils::{input_to_trimmed_lines, test_day, AdventError, Answer, Day};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let seed_line = lines.next().unwrap();
        let seeds = seed_line.split(':').nth(1).unwrap().split_whitespace();

        // Gotta skip the first mapping line, otherwise every map is off by one
        lines.next();

        // Gross, why is it in a vec, there's got to be a better way. 8 seconds to run!!!??? 🤮
        let seed_to_soil: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let soil_to_fertilizer: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let fertilizer_to_water: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let water_to_light: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let light_to_temperature: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let temperature_to_humidity: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let humidity_to_location: Vec<_> = lines.map_while(line_to_source).collect();

        let lowest_location_for_seed = seeds
            .map(|seed| {
                let seed = seed.parse().unwrap();
                let soil = seed_to_soil
                    .iter()
                    .find(|source| source.source_range.contains(&seed))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == seed)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(seed);
                let fertilizer = soil_to_fertilizer
                    .iter()
                    .find(|source| source.source_range.contains(&soil))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == soil)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(soil);
                let water = fertilizer_to_water
                    .iter()
                    .find(|source| source.source_range.contains(&fertilizer))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == fertilizer)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(fertilizer);
                let light = water_to_light
                    .iter()
                    .find(|source| source.source_range.contains(&water))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == water)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(water);
                let temperature = light_to_temperature
                    .iter()
                    .find(|source| source.source_range.contains(&light))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x: usize| x == light)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(light);
                let humidity = temperature_to_humidity
                    .iter()
                    .find(|source| source.source_range.contains(&temperature))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == temperature)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(temperature);
                let location = humidity_to_location
                    .iter()
                    .find(|source| source.source_range.contains(&humidity))
                    .map(|found_item| {
                        let position = found_item
                            .source_range
                            .clone()
                            .position(|x| x == humidity)
                            .unwrap();
                        found_item.destination_range.start + position
                    })
                    .unwrap_or(humidity);

                location
            })
            .min();

        Ok(lowest_location_for_seed.unwrap().try_into().unwrap())
    }

    fn part_2(_input: &str) -> Result<Answer, AdventError> {
        Ok(2)
    }
}

fn line_to_source(line: String) -> Option<Source> {
    // We've reached the next map, stop iterating!
    if line.contains("map:") {
        return None;
    }

    let mut numbers = line.split_whitespace();
    let destination_range_start = numbers.next().unwrap();
    let source_range_start = numbers.next().unwrap();
    let range_length: usize = numbers.next().unwrap().parse::<usize>().unwrap();

    Some(Source {
        destination_range: Range {
            start: destination_range_start.parse::<usize>().unwrap(),
            end: destination_range_start.parse::<usize>().unwrap() + range_length,
        },
        source_range: Range {
            start: source_range_start.parse::<usize>().unwrap(),
            end: source_range_start.parse::<usize>().unwrap() + range_length,
        },
    })
}

#[derive(Debug)]
struct Source {
    destination_range: Range<usize>,
    source_range: Range<usize>,
}

test_day!(
    Day5,
    35,
    30,
    r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    "#
);
