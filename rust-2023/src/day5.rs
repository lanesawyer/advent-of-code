use std::ops::Range;

use aoc_utils::{AdventError, Answer, Day, input_to_trimmed_lines, test_day};

pub struct Day5;

impl Day for Day5 {
    fn part_1(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let seed_line = lines.next().unwrap();
        let seeds = seed_line.split(':').nth(1).unwrap().split_whitespace();

        // Gotta skip the first mapping line, otherwise every map is off by one
        lines.next();

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
                let soil = find_destination_mapping(&seed_to_soil, seed);
                let fertilizer = find_destination_mapping(&soil_to_fertilizer, soil);
                let water = find_destination_mapping(&fertilizer_to_water, fertilizer);
                let light = find_destination_mapping(&water_to_light, water);
                let temperature = find_destination_mapping(&light_to_temperature, light);
                let humidity = find_destination_mapping(&temperature_to_humidity, temperature);
                find_destination_mapping(&humidity_to_location, humidity)
            })
            .min();

        Ok(lowest_location_for_seed.unwrap().try_into().unwrap())
    }

    fn part_2(input: &str) -> Result<Answer, AdventError> {
        let mut lines = input_to_trimmed_lines(input);

        let seed_line = lines.next().unwrap();
        let seeds_ranges_parsed = seed_line
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>(); // TODO: Why do I have to collect first before chunking?
        let seed_ranges = seeds_ranges_parsed.chunks(2).map(|chunk| {
            let start = chunk[0].parse::<usize>().unwrap();
            let length = chunk[1].parse::<usize>().unwrap();
            Range {
                start,
                end: start + length,
            }
        });

        // Gotta skip the first mapping line, otherwise every map is off by one
        lines.next();

        let seed_to_soil: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let soil_to_fertilizer: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let fertilizer_to_water: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let water_to_light: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let light_to_temperature: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let temperature_to_humidity: Vec<_> = lines.by_ref().map_while(line_to_source).collect();
        let humidity_to_location: Vec<_> = lines.by_ref().map_while(line_to_source).collect();

        let lowest_location_for_seed = seed_ranges
            .map(|seed_range| {
                let min_in_range = seed_range
                    .map(|seed| {
                        let soil = find_destination_mapping(&seed_to_soil, seed);
                        let fertilizer = find_destination_mapping(&soil_to_fertilizer, soil);
                        let water = find_destination_mapping(&fertilizer_to_water, fertilizer);
                        let light = find_destination_mapping(&water_to_light, water);
                        let temperature = find_destination_mapping(&light_to_temperature, light);
                        let humidity =
                            find_destination_mapping(&temperature_to_humidity, temperature);
                        find_destination_mapping(&humidity_to_location, humidity)
                    })
                    .min();

                min_in_range.unwrap()
            })
            .min();

        Ok(lowest_location_for_seed.unwrap().try_into().unwrap())
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

fn find_destination_mapping(mapping: &[Source], source_value: usize) -> usize {
    let source_with_value = mapping
        .iter()
        .find(|source| source.source_range.contains(&source_value));

    match source_with_value {
        Some(source) => {
            let start = source.source_range.start;
            source.destination_range.start + (source_value - start)
        }
        None => source_value,
    }
}

#[derive(Debug)]
struct Source {
    destination_range: Range<usize>,
    source_range: Range<usize>,
}

test_day!(
    Day5,
    35,
    46,
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
