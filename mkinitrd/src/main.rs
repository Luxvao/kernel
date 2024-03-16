use std::{
    fs::{File, FileType, ReadDir},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    os::unix::ffi::OsStrExt,
    path::Path,
    process::exit,
};

#[derive(Debug, Clone)]
enum Entry {
    FileEntry {
        name: String,
        size: u64,
        content: Vec<u8>,
    },
    DirEntry {
        name: String,
        content: Vec<Entry>,
    },
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(cmd) => cmd,
        None => {
            println!("No command specified");
            exit(1);
        }
    };

    match command.trim() {
        "create" => {
            if args.len() != 4 {
                println!("Bad usage");
                exit(1);
            }

            let output = match File::create(args.last().unwrap()) {
                Ok(out) => out,
                Err(_) => {
                    println!("Unable to create output initrd");
                    exit(1);
                }
            };

            let mut output_writer = BufWriter::new(output);

            // Add initrd magic
            handle_failed_write(output_writer.write_all(b"initrd\r\n"));

            let input_folder = Path::new(&args[2]);

            handle_failed_write(output_writer.write_all(b"entry\r\n1\r\nroot\r\n"));

            handle_directory(
                &mut output_writer,
                match input_folder.read_dir() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Input if not a folder");
                        exit(1);
                    }
                },
            );

            handle_failed_write(output_writer.write_all(b"entry\r\n2\r\n"));

            println!("initrd created `{}`", args.last().unwrap());
        }
        "list" => {
            if args.len() != 3 {
                println!("Bad usage");
                exit(1);
            }

            let open_file = match File::open(&args[2]) {
                Ok(file) => file,
                Err(_) => {
                    println!("File not found");
                    exit(1);
                }
            };

            let mut reader = BufReader::new(open_file);

            let mut magic = String::new();

            match reader.read_line(&mut magic) {
                Ok(_) => (),
                Err(_) => {
                    println!("Reading from initrd failed");
                    exit(1);
                }
            }

            if magic.trim() != "initrd" {
                println!("Bad initrd");
                exit(1);
            }

            print_entries(parse_initrd_into_entry(&mut reader).unwrap(), 0);
        }
        "help" => {
            println!(
                "Usage:\nmkinitrd create [root folder] [output filename]\nmkinitrd list [initrd]"
            );
            exit(0);
        }
        _ => {
            println!("Command not found");
            exit(1);
        }
    }
}

fn handle_directory<T>(writer: &mut BufWriter<T>, mut contents: ReadDir)
where
    T: Write,
{
    while let Some(Ok(entry)) = contents.next() {
        if entry.file_type().expect("How did this fail??").is_dir() {
            handle_failed_write(writer.write_all(b"entry\r\n1\r\n"));

            handle_failed_write(writer.write_all(entry.file_name().as_bytes()));

            handle_failed_write(writer.write_all(b"\r\n"));

            let path = entry.path();

            handle_directory(writer, path.read_dir().unwrap());

            handle_failed_write(writer.write_all(b"entry\r\n2\r\n"));
        } else {
            handle_failed_write(writer.write_all(b"entry\r\n0\r\n"));

            handle_failed_write(writer.write_all(entry.file_name().as_bytes()));

            handle_failed_write(writer.write_all(b"\r\n"));

            let path = entry.path();

            let mut buffer = Vec::new();

            let mut open_file = match File::open(path) {
                Ok(file) => file,
                Err(_) => {
                    println!("File not found");
                    exit(1);
                }
            };

            match open_file.read_to_end(&mut buffer) {
                Ok(_) => (),
                Err(_) => {
                    println!("Failed to read from file");
                    exit(1);
                }
            }

            handle_failed_write(writer.write_all(buffer.len().to_string().as_bytes()));

            handle_failed_write(writer.write_all(b"\r\n"));

            handle_failed_write(writer.write_all(buffer.clone().as_ref()));

            handle_failed_write(writer.write_all(b"\r\n"));
        }
    }
}

fn parse_initrd_into_entry<T>(reader: &mut BufReader<T>) -> Option<Entry>
where
    T: Read,
{
    let mut buffer = String::new();

    match reader.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => {
            println!("Reading from initrd failed");
            exit(1);
        }
    }

    if buffer.trim() != "entry" {
        println!("Bad initrd");
        exit(1);
    }

    buffer.clear();

    match reader.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => {
            println!("Reading from initrd failed");
            exit(1);
        }
    }

    match buffer.trim().parse::<u8>().expect("Bad initrd") {
        0 => {
            let mut name = String::new();

            match reader.read_line(&mut name) {
                Ok(_) => (),
                Err(_) => {
                    println!("Reading from initrd failed");
                    exit(1);
                }
            }

            let mut size = String::new();

            match reader.read_line(&mut size) {
                Ok(_) => (),
                Err(_) => {
                    println!("Reading from initrd failed");
                    exit(1);
                }
            }

            let mut contents = vec![0; size.trim().parse().expect("Bad initrd")];

            match reader.read_exact(&mut contents) {
                Ok(_) => (),
                Err(_) => {
                    println!("Reading from initrd failed");
                    exit(1);
                }
            }

            reader.consume(2);

            buffer.clear();

            Some(Entry::FileEntry {
                name: name.trim().to_owned(),
                size: size.trim().parse().expect("Bad initrd"),
                content: contents,
            })
        }
        1 => {
            let mut name = String::new();

            match reader.read_line(&mut name) {
                Ok(_) => (),
                Err(_) => {
                    println!("Reading from initrd failed");
                    exit(1);
                }
            };

            let mut contents = Vec::new();

            while let Some(entry) = parse_initrd_into_entry(reader) {
                contents.push(entry);
            }

            Some(Entry::DirEntry {
                name: name.trim().to_owned(),
                content: contents,
            })
        }
        _ => None,
    }
}

fn print_entries(entry: Entry, indent: u32) {
    match entry {
        Entry::FileEntry {
            name,
            size,
            content: _,
        } => {
            for _ in 0..indent {
                print!(" ");
            }

            if indent > 0 {
                println!("|");

                for _ in 0..indent {
                    print!(" ");
                }

                print!("|--> ");
            }

            println!("FE: {}: {}", name, size);
        }
        Entry::DirEntry { name, content } => {
            for _ in 0..indent {
                print!(" ");
            }

            if indent > 0 {
                println!("|");

                for _ in 0..indent {
                    print!(" ");
                }

                print!("|--> ");
            }

            println!("DE: {}", name);

            for entry in content {
                print_entries(entry, indent + 3);
            }
        }
    }
}

fn handle_failed_write<T, E>(result: Result<T, E>) {
    match result {
        Ok(_) => (),
        Err(_) => {
            println!("Failed to write to output initrd");
            exit(1);
        }
    }
}
