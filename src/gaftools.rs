use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Debug)]
struct Gaf {
    qname: String,
    qlen: u64,
    qstart: u64,
    qend: u64,
    strand: char,
    path: String, 
    tlen: u64,
    tstart: u64,
    tend: u64,
    matches: u64,
    aln_len: u64,
    mapq: u8,
    tags: Vec<String>, // raw "TAG:TYPE:VALUE"
}

struct Node {
    name : u64,
    direction: bool,
    strand: bool
}

impl Gaf {
    fn parse(line: &str) -> Result<Self, ParseIntError> {
        let cols: Vec<&str> = line.split('\t').collect();
        Ok(Self {
            qname: cols[0].to_string(),
            qlen: cols[1].parse()?,
            qstart: cols[2].parse()?,
            qend: cols[3].parse()?,
            strand: cols[4].chars().next().unwrap_or('+'),
            path: cols[5].to_string(),
            tlen: cols[6].parse()?,
            tstart: cols[7].parse()?,
            tend: cols[8].parse()?,
            matches: cols[9].parse()?,
            aln_len: cols[10].parse()?,
            mapq: cols[11].parse()?,
            tags: cols[12..].iter().map(|s| s.to_string()).collect(),
        })
    }
}


fn main() -> Result<(), ParseIntError> {
    let path1 = std::env::args().nth(1).expect("No GAF provided.");
    let path2 = std::env::args().nth(2).expect("No GAF provided.");
    let file1 = File::open(&path1).expect("Cannot open GAF.");
    let file2 = File::open(&path2).expect("Cannot open GAF.");
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    for (i, line) in reader1.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let rec = Gaf::parse(&line).unwrap();
        println!(
            "{}:{}-{} on {}:{}-{} (strand {}) matches={}/{} MAPQ={}",
            rec.qname, rec.qstart, rec.qend,
            rec.path, rec.tstart, rec.tend,
            rec.strand, rec.matches, rec.aln_len, rec.mapq
        );
    }
    Ok(())
}
