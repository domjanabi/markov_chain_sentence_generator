use std::collections::HashMap;
use std::{
    fs::File,
    io::{Read, Seek},
};

type WordIndexType = u32;
const WORD_MATCH_COUNT: usize = 2;
fn main()
{
    let mut file = get_file_from_user();
    let words = ("_ ".repeat(WORD_MATCH_COUNT - 1) + &file_to_string(&mut file))
        .to_lowercase()
        .replace('.', " . ")
        .replace('?', " ? ")
        .replace('!', " ! ")
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    println!("Total amount of words: {}", words.len());
    println!();

    let mut unique_words = words.clone();
    unique_words.dedup();
    let mut word_indices = Vec::new();
    for word in words
    {
        let index = unique_words
            .iter()
            .enumerate()
            .find(|(_u, w)| **w == word)
            .unwrap()
            .0;
        word_indices.push(index);
    }
    let mut nodes = HashMap::new();

    for indices in word_indices.windows(WORD_MATCH_COUNT + 1)
    {
        let key = std::array::from_fn(|u| indices[u] as WordIndexType);
        assert!(key.len() == WORD_MATCH_COUNT);
        let link = Link {
            next_word: *indices.last().unwrap() as WordIndexType,
            count: 1,
        };

        match nodes.get_mut(&key)
        {
            None =>
            {
                nodes.insert(
                    key,
                    Links {
                        data: vec![link],
                        total_word_counter: 1,
                    },
                );
            }
            Some(links) =>
            {
                match links.data.iter_mut()
                    .find(|l| l.next_word == link.next_word)
                {
                    None => links.data.push(link),
                    Some(l) => l.count += 1,
                }
                links.total_word_counter += 1;
            }
        }

        let key2 = std::array::from_fn(|u| {
            if u == 0
            {
                0
            }
            else
            {
                indices[u] as WordIndexType
            }
        });
        match nodes.get_mut(&key2)
        {
            None =>
            {
                nodes.insert(
                    key2,
                    Links {
                        data: vec![link],
                        total_word_counter: 1,
                    },
                );
            }
            Some(links) =>
            {
                match links
                    .data
                    .iter_mut()
                    .find(|l| l.next_word == link.next_word)
                {
                    None => links.data.push(link),
                    Some(l) => l.count += 1,
                }
                links.total_word_counter += 1;
            }
        }
    }
    let sentence_generator = MarkovChain {
        words: unique_words,
        nodes,
    };

    for _i in 0..100
    {
        let output = sentence_generator.generate(200);
        println!("{output}");
        let _ = std::io::stdin().read(&mut [0u8; 2]).unwrap();
    }
}

#[derive(Clone, Copy)]
struct Link
{
    next_word: WordIndexType,
    count: WordIndexType,
}

struct Links
{
    data: Vec<Link>,
    total_word_counter: usize,
}

struct MarkovChain
{
    words: Vec<String>,
    nodes: HashMap<[WordIndexType; WORD_MATCH_COUNT], Links>,
}

impl MarkovChain
{
    fn generate(&self, length: usize) -> String
    {
        let mut output = String::with_capacity(length * 5);
        let mut index = *self
            .nodes
            .iter()
            .nth(fastrand::usize(0..self.nodes.len()))
            .unwrap()
            .0;
        for _i in 0..length
        {
            let next_word = match self.nodes.get(&index)
            {
                None => 0, //fastrand::usize(..self.words.len()) as word_index_type,
                Some(links) =>
                {
                    let rand = fastrand::usize(0..links.total_word_counter);
                    let mut counter = 0;
                    let mut random_link = 0;
                    for j in links.data.iter()
                    {
                        if (counter..counter + j.count).contains(&(rand as WordIndexType))
                        {
                            random_link = j.next_word;
                            break;
                        }
                        counter += j.count;
                    }
                    random_link as WordIndexType
                }
            };
            index.rotate_left(1);
            *index.last_mut().unwrap() = next_word;
            output += match self.words[next_word as usize].as_str()
            {
                "." | "?" | "!" => "",
                _ => " ",
            };
            output += &self.words[next_word as usize];
        }
        output
    }
}


fn get_file_from_user() -> File
{
    if let Some(s) = std::env::args().take(2).collect::<Vec<String>>().get(1)
    {
        let arg = s.to_string();
        if let Ok(file) = std::fs::OpenOptions::new().write(true).read(true).open(arg)
        {
            return file;
        }
    }
    println!("Enter a valid path to your file, or drag it onto the exe icon,\nor drag it into the Window and press enter.");
    loop
    {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if let Ok(file) = File::open(std::path::Path::new(input.trim()))
        {
            return file;
        }
        println!("Invalid path.");
    }
}

fn file_to_string(file: &mut File) -> String
{
    let mut s = String::new();
    file.seek(std::io::SeekFrom::Start(0)).unwrap();
    file.read_to_string(&mut s).unwrap();
    s
}
