use std::collections::HashMap;

#[derive(Debug)]
pub struct Parser {
    pub source: String,
}

impl Parser {
    pub fn new(src: String) -> Self {
        Self { source: src }
    }
}

impl Parser {
    pub fn parse(&self, hash_map: HashMap<&str, &str>) -> String {
        let (keys, blocks) = get_keys(&self.source);
        let mut values: Vec<&str> = vec![];
        let mut final_src = self.source.clone();

        for key in keys.iter() {
            match hash_map.get(key) {
                Some(val) => values.push(val),
                None => values.push(""),
            }
        }

        for i in 0..blocks.len() {
            final_src = final_src.replace(blocks[i], values[i]);
        }

        final_src
    }
}

pub fn get_blocks(src: &str) -> Vec<&str> {
    let mut blocks: Vec<&str> = vec![];

    for (i, c) in src.chars().enumerate() {
        // if two continuos characters are {
        if c == '{' && src.as_bytes()[i + 1] as char == '{' {
            // iterates over the rest of the string until it reaches the first "}}"
            for (ii, cc) in src[i + 2..].chars().enumerate() {
                // if two continuos characters are }
                if cc == '}' && src[i + 2..].as_bytes()[ii + 1] as char == '}' {
                    // ii+i+2 because ii shows the index in the sliced string (str[i+2..])
                    // and we want ii to show the index in the original string (str)
                    // and an additional +2 because we also want the "}}"
                    let block = src[i..ii + i + 4].trim();
                    // push the block and break the inner loop
                    blocks.push(block);
                    break;
                }
            }
        }
    }

    blocks
}

pub fn get_keys(src: &str) -> (Vec<&str>, Vec<&str>) {
    let blocks = get_blocks(src);
    let mut keys: Vec<&str> = vec![];

    for block in blocks.iter() {
        let key = block[2..block.len() - 2].trim();
        keys.push(key);
    }

    (keys, blocks)
}
