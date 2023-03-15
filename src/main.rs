use std::fs;
use std::io;

fn main() {
    std::process::exit(new_main());
}

fn new_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage : {} <filename>", args[0]);
        return 1;
    }

    let file_name = std::path::Path::new(&args[1]);
    let File = fs::File::open(&file_name).unwrap();
    let mut archieve = zip::ZipArchive::new(File).unwrap();

    for i in 0..archieve.len() {
        let mut File = archieve.by_index(i).unwrap();

        let output_path = match File.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = File.comment();
            if !comment.is_empty() {
                println!("file {} comment: {} ", i, comment);
            }
        }
        if (*File.name()).ends_with('/') {
            println!("file {} extracted to {} ", i, output_path.display());
            fs::create_dir_all(&output_path).unwrap();
        } else {
            println!(
                "file {} extracted to {} {} bytes ",
                i,
                output_path.display(),
                File.size(),
            );

            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
                let mut output_file = fs::File::create(&output_path).unwrap();
                io::copy(&mut File, &mut output_file).unwrap();
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = File.unix_mode() {
                    fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }
        0
    }

