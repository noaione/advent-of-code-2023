use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
enum MappingKind {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl MappingKind {
    fn from_str(s: &str) -> Self {
        match s {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temperature,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => panic!("Unknown mapping kind: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeAction {
    start: usize,
    range: usize,
}

impl RangeAction {
    fn max(&self) -> usize {
        self.start + self.range
    }
}

#[derive(Clone, Debug)]
pub struct XYMap {
    source: MappingKind,
    // Position would be basically the source -> target
    src_range: Vec<RangeAction>,
    dest_range: Vec<RangeAction>,
}

impl XYMap {
    fn new(source: MappingKind) -> Self {
        Self {
            source,
            src_range: Vec::new(),
            dest_range: Vec::new(),
        }
    }

    fn parse_pos(&mut self, range: &str) {
        let split_range: Vec<usize> = range
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let dest_start = *split_range.get(0).unwrap();
        let src_start = *split_range.get(1).unwrap();
        let range = *split_range.get(2).unwrap();

        self.src_range.push(RangeAction {
            start: src_start,
            range,
        });
        self.dest_range.push(RangeAction {
            start: dest_start,
            range,
        });
    }

    fn get_dest_pos(&self, src_pos: usize) -> usize {
        for (src_range, dest_range) in self.src_range.iter().zip(self.dest_range.iter()) {
            if src_pos >= src_range.start && src_pos < src_range.max() {
                // Offset by the dest_range
                return src_pos - src_range.start + dest_range.start;
            }
        }

        // No match, return the original
        src_pos
    }
}

pub fn parse_input(input: &str) -> (Vec<usize>, Vec<XYMap>) {
    // split by double \n\n
    let mut lines: Vec<&str> = input.split("\n\n").filter(|&x| !x.is_empty()).collect();

    let seeds: Vec<usize> = lines
        .drain(0..1)
        .join("")
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut xy_maps_vec = Vec::new();
    for &line in lines.iter() {
        let mut line_splits = line.split("\n");
        let header = line_splits.next().unwrap().replace(" map:", "");
        let (from_head, _) = header.trim().split_once("-to-").unwrap();

        let from_kind = MappingKind::from_str(from_head);

        let mut xy_maps = XYMap::new(from_kind);

        for range in line_splits {
            if range.is_empty() {
                continue;
            }

            xy_maps.parse_pos(range);
        }

        xy_maps_vec.push(xy_maps);
    }

    (seeds, xy_maps_vec)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (seeds, mut xy_maps_vec) = parse_input(input);
    xy_maps_vec.sort_by_key(|x| x.source);

    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut next_pos = seed;
        for xy_map in xy_maps_vec.iter() {
            next_pos = xy_map.get_dest_pos(next_pos);
        }
        locations.push(next_pos);
    }

    locations.sort();
    Some(*locations.first().unwrap())
}

pub fn part_two(_input: &str) -> Option<u32> {
    // let (seeds, mut xy_maps_vec) = parse_input(input);
    // xy_maps_vec.sort_by_key(|x| x.source);

    // pair seeds
    // let num_pairs = find_start_end_pairs(seeds);
    // preliminary check with seeds to see if any intersect
    // let test_pairs: Vec<_> = num_pairs.into_par_iter().chunks(100).collect();
    // println!("test_pairs: {:?}", test_pairs);

    None
}
