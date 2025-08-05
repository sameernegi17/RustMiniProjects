use std::{fs::{self, DirBuilder}, sync::mpsc, thread, time::Instant};
use indicatif::{ProgressBar, ProgressStyle};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args
{
    #[arg(required = true)]
    files : Vec<String>,
}

fn count_words(content: &str) -> usize{
    content.split_whitespace().count()
}

fn main() {
   let args = Args::parse();
   let start_time = Instant::now();

   let (tx, rx) = mpsc::channel();


   let num_files = args.files.len();

   let pb = ProgressBar::new(num_files as u64);

    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}]{bar:40.cyan/blue} {pos}/{len} files").unwrap().progress_chars("##."),);

    for file_path in args.files.clone()
    {
        let tx = tx.clone();
        let pb = pb.clone();

        thread::spawn(move || {

                let content = fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Failed to read {}", file_path));

            let count = count_words(&content);

            tx.send((file_path, count)).unwrap();

            pb.inc(1);

        });
    }

        drop(tx);

        let mut total = 0;
        for (file, count) in &rx {
            println!("File {file} , count {count} Words");
            total += count;
        }
        
         pb.finish_with_message("Done");

        let duration = start_time.elapsed();
        println!("Total Words {total}");
        println!("Elapsed Time {:2?}", duration);

}
