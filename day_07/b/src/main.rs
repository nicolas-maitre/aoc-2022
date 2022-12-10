use std::{collections::HashMap, fs::read_to_string};

fn main() {
    //     let input = "$ cd /
    // $ ls
    // dir a
    // 14848514 b.txt
    // 8504156 c.dat
    // dir d
    // $ cd a
    // $ ls
    // dir e
    // 29116 f
    // 2557 g
    // 62596 h.lst
    // $ cd e
    // $ ls
    // 584 i
    // $ cd ..
    // $ cd ..
    // $ cd d
    // $ ls
    // 4060174 j
    // 8033020 d.log
    // 5626152 d.ext
    // 7214296 k
    // ";

    let input = read_to_string("../input.txt").expect("valid input file");

    let mut root_folder = Folder {
        children: HashMap::new(),
    };

    let mut working_directory_path: Vec<&str> = vec![];

    //parse commands
    for command_group in input.split("$ ") {
        if command_group.is_empty() {
            continue;
        };
        let (command_line, command_response) =
            command_group.split_once("\n").expect("valid instruction");
        // DEBUG
        // println!("_______________");
        // println!("CWD: {:?}", working_directory_path);
        // println!("COM: {}", command_line);
        let command_line_parts: Vec<&str> = command_line.split(" ").collect();
        let instruction = command_line_parts.get(0).expect("instruction").to_owned();
        match instruction {
            "cd" => {
                let path_target = command_line_parts
                    .get(1)
                    .expect("path parameter")
                    .to_owned();

                if path_target == ".." {
                    working_directory_path.pop();
                } else if path_target.chars().next().expect("a parameter") == '/' {
                    working_directory_path = path_target[1..]
                        .split("/")
                        .filter(|part| !part.is_empty())
                        .collect()
                } else {
                    working_directory_path.append(&mut path_target.split("/").collect());
                };
            }
            "ls" => {
                //parse response
                let new_data_folder = Folder {
                    // children: HashMap::from_iter(vec![]),
                    children: HashMap::from_iter(command_response.lines().map(|response_line| {
                        let (col1, col_name) = response_line
                            .split_once(" ")
                            .expect("response in two parts");
                        (
                            col_name.to_string(),
                            if col1 == "dir" {
                                ItemType::Folder(Folder {
                                    children: HashMap::new(),
                                })
                            } else {
                                ItemType::File(File {
                                    size: col1.parse().expect("a valid size"),
                                })
                            },
                        )
                    })),
                };
                root_folder =
                    fill_from_bottom(root_folder, new_data_folder, &working_directory_path)
            }
            _ => panic!("invalid instruction {}", instruction),
        }
    }

    // DEBUG
    // println!("================\n");
    // println!("{}", root_folder.pretty_dump_children(0));

    let all_items = root_folder.get_flat_items();
    let mut folder_sizes: Vec<u32> = all_items
        .iter()
        .filter_map(|item| match item {
            ItemType::Folder(folder) => Some(folder.get_size()),
            ItemType::File(_) => None,
        })
        .collect();
    folder_sizes.sort_unstable();

    let total_size = folder_sizes.last().unwrap().to_owned();
    const DISK_SPACE: u32 = 70_000_000;
    const SPACE_REQUIRED: u32 = 30_000_000;
    let min_space_to_free = SPACE_REQUIRED + total_size - DISK_SPACE;

    let folder_to_delete_size = folder_sizes
        .iter()
        .find(|size| size >= &&min_space_to_free)
        .unwrap();

    println!(
        "total {}.\n to free {}.\n to delete {}",
        total_size, min_space_to_free, folder_to_delete_size
    );
}

fn fill_from_bottom(base_folder: Folder, fill_folder: Folder, path: &Vec<&str>) -> Folder {
    let mut path_rest = path.clone();
    let last_path = path_rest.pop().unwrap_or("");
    let res_folder = match base_folder.get_sub_directory_view(path) {
        Some(mut real_folder) => {
            //maybe check here for shallow folders
            for (item_name, item) in fill_folder.children {
                real_folder.children.insert(item_name, item);
            }
            real_folder
        }
        None => fill_folder,
    };
    if path.is_empty() {
        return res_folder;
    }
    let parent_folder = Folder {
        children: HashMap::from([(last_path.to_string(), ItemType::Folder(res_folder))]),
    };
    fill_from_bottom(base_folder, parent_folder, &path_rest)
}

#[derive(Clone, Debug)]
struct File {
    size: u32,
}

#[derive(Clone, Debug)]
struct Folder {
    children: HashMap<String, ItemType>,
}
impl Folder {
    fn get_size(&self) -> u32 {
        self.children.iter().fold(0, |curr, (_, item)| {
            curr + match item {
                ItemType::Folder(folder) => folder.get_size(),
                ItemType::File(file) => file.size,
            }
        })
    }
    fn get_flat_items(&self) -> Vec<ItemType> {
        self.children
            .iter()
            .flat_map(|(_, item)| match item {
                ItemType::Folder(folder) => folder.get_flat_items(),
                _ => vec![item.to_owned()],
            })
            .chain(vec![ItemType::Folder(self.to_owned())])
            .collect()
    }

    fn get_sub_directory_view(&self, path: &Vec<&str>) -> Option<Folder> {
        let first_path = match path.get(0) {
            Some(fp) => fp,
            None => return Some(self.to_owned()),
        };

        let path_rest = path[1..].to_vec();
        if first_path.is_empty() {
            self.get_sub_directory_view(&path_rest)
        } else {
            let sub_folder = match self.children.get(first_path.to_owned())? {
                ItemType::Folder(folder) => folder,
                ItemType::File(_) => panic!("a file path is not a valid dir path"),
            };
            sub_folder.get_sub_directory_view(&path_rest)
        }
    }
    fn pretty_dump_children(&self, depth: u8) -> String {
        const INDENTATION: &str = "    ";
        let spaces_offset =
            (0..depth).fold("".to_string(), |curr, _| format!("{}{}", curr, INDENTATION));

        self.children
            .iter()
            .map(|(name, item)| match item {
                ItemType::Folder(folder) => format!(
                    "{}[DIR]  {}\n{}",
                    spaces_offset,
                    name,
                    folder.pretty_dump_children(depth + 1)
                ),
                ItemType::File(file) => {
                    format!("{}[FILE] {} -> {}", spaces_offset, name, file.size)
                }
            })
            // .chain(["".to_string()]) //extra line break
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Clone, Debug)]
enum ItemType {
    Folder(Folder),
    File(File),
}
