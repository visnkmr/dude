use std::process::{Command, Stdio};
use std::str;
use std::{
    io::{self, Write, prelude::*, SeekFrom},
    thread,
    time::Duration,
    fs::File,
    collections::HashMap,
};

// This structure will hold our data for the disks
struct IoStats {
    pub mb_read: f64,
    pub mb_wrtn: f64,
}

// We won't handle any error case in this guide
pub fn mondisu(ot:&mut String){
    // 2048 is for mb
    // One sector is 512b and 1 sector is typically 512b
    // So we keep it in mind and when we'll read /proc/diskstats
    // We'll divide the number of sector read by 512 and then by 2048 for mb
    // And compute the difference to obtain mb/s.
    let fctr = 512.0;
    // Hashmap of previous drives stats to compute difference from
let mut prev: IoStats = IoStats{mb_read:0.0,mb_wrtn:0.0};
    // Open the file we'll use to get the stats
    let mut fd = File::open(&"/proc/diskstats").unwrap();
//    let mut ot;
    loop {
        // Create the curr Hashmap, allow us to compare with the prev one
        let mut ds: IoStats;
        // Create the output string
        let mut output = String::new();
        // Add the header string to the output
        //        output.push_str("\nDevice          mb_reads/s      mb_wrtn/s\n\n");
        // Collecting info/data
        {
            // Create a new empty string
            let mut io_data = String::new();
            // Read the content of the file (diskstats) to the io_data string
            fd.read_to_string(&mut io_data).unwrap();
            // Iterate over each line (each disk)
            let line = io_data.lines().nth(5).unwrap();
            {
                //                println!("{}",line);
                // Split field (separated by whitespace) and collect them without specific type
                let fields = line.split_whitespace().collect::<Vec<_>>();
                // If the are less than 14 fields, the file is missing data
                // see (https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats)
                if fields.len() < 14 {
                    panic!("Not enough data from diskstats");
                }
                ds = IoStats {
                    mb_read: fields[5].parse::<f64>().unwrap() * fctr,
                    mb_wrtn: fields[9].parse::<f64>().unwrap() * fctr,
                };
                // If prev already contains the info we compute the diff to get mb/s
                // Else we add to the print line the "fake" data.

                if prev.mb_read !=0.0 {
                    //                    println!("here");
                    // Get the object from the hashmap
                    // Construct speed line and append it to curr hashmap
                    let mb_read_s = ds.mb_read - prev.mb_read;
                    let mb_wrtn_s = ds.mb_wrtn - prev.mb_wrtn;
                    // Add the line, formatted with color and spacing
                    //                    output.push_str(&format!("{:2} {:10.2} {:15.2}\n", fields[2], mb_read_s, mb_wrtn_s));
                    output.push_str(&format!("{:2} {:2.2} {:2.2}\n", fields[2], byte_unit::Byte::from_bytes(mb_read_s as u128).get_appropriate_unit(true), byte_unit::Byte::from_bytes(mb_wrtn_s as u128).get_appropriate_unit(true)));
                    // Insert the current disk data to the curr HashMap
                    // the curr will later be saved as prev
                    //                    curr = ds;
                } else {
                    //                    println!("here2");
                    // Add the line with fake data and formatting
                    output.push_str(&format!("{} {} {}\n", fields[2], 0.00, 0.00));
                    //                    curr= IoStats{mb_read:0.0,mb_wrtn:0.0};
                }
            }
            // Move the cursor to the start of the file
            fd.seek(SeekFrom::Start(0)).unwrap();
        }
        // Print the result
        //        writeln!(io::stdout().lock(), "{}", output);
        print!("{}", output);
        *ot=output;
        // Save current as previous for the next loop
        prev = ds;
        // Wait for 1 seconds to respect the mb/s
        thread::sleep(Duration::from_secs(1));

    }
}

pub fn findcpuconsumers()->Vec<String>{
    //let output = Command::new("bash")
    //        .arg("-c echo $(ps axch -o cmd:20,%cpu --sort=-%cpu | head -n 5)")
    //        .output()
    //        .expect("Failed to execute command");
    let args = "ps axch -o cmd:20,%cpu --sort=-%cpu";
    let args: Vec<_> = args.split(" ").collect();

    let output = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let args2 = "head -n 5";
    let args2: Vec<_> = args2.split(" ").collect();
    let o1 = Command::new(args2[0])
        .args(&args2[1..])
        .stdin(Stdio::from(output.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = o1.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
//    println!("{}",result);
    let args2: Vec<_> = result.split("\n").collect();
    let mut v2=Vec::new();
    for i in &args2{
        i.to_string().replace(" ", "");
        let is = i.to_string();
        let words: Vec<_> = is.split_whitespace().collect();
        v2.push(words.join(" "));
    }
//    println!("{:?}",v2);
    v2
}