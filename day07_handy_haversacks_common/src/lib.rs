pub struct ContainedBags {
    pub count: usize,
    pub color: &'static str,
}

pub struct BagInfo {
    pub color: &'static str,
    pub contains: &'static [ContainedBags],
}

pub const TARGET_BAG: &str = "shiny gold";

// Substitute with
// '<,'>s/^\([a-zA-Z ]\+\) bags /    BagInfo { color: "\1", /
// '<,'>s/\./] },/
// '<,'>s/contain /contains: \&[
// '<,'>s/no other bags//
// '<,'>s/\(\d\+\) \([a-zA-Z ]\+\) bags\=/ContainedBags { count: \1, color: "\2" }/g

pub const SAMPLE_DATA: [BagInfo; 9] = [
    BagInfo { color: "light red", contains: &[ContainedBags { count: 1, color: "bright white" }, ContainedBags { count: 2, color: "muted yellow" }] },
    BagInfo { color: "dark orange", contains: &[ContainedBags { count: 3, color: "bright white" }, ContainedBags { count: 4, color: "muted yellow" }] },
    BagInfo { color: "bright white", contains: &[ContainedBags { count: 1, color: "shiny gold" }] },
    BagInfo { color: "muted yellow", contains: &[ContainedBags { count: 2, color: "shiny gold" }, ContainedBags { count: 9, color: "faded blue" }] },
    BagInfo { color: "shiny gold", contains: &[ContainedBags { count: 1, color: "dark olive" }, ContainedBags { count: 2, color: "vibrant plum" }] },
    BagInfo { color: "dark olive", contains: &[ContainedBags { count: 3, color: "faded blue" }, ContainedBags { count: 4, color: "dotted black" }] },
    BagInfo { color: "vibrant plum", contains: &[ContainedBags { count: 5, color: "faded blue" }, ContainedBags { count: 6, color: "dotted black" }] },
    BagInfo { color: "faded blue", contains: &[] },
    BagInfo { color: "dotted black", contains: &[] },
];
