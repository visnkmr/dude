use std::{process::{Command, exit, self},
    time::{Instant, self, Duration,
        SystemTime, UNIX_EPOCH},
        thread, collections::HashMap, hash::Hash, env, net::{TcpListener, TcpStream}, io::{BufRead, Write, Read, Seek, SeekFrom}, sync::mpsc, fs::File};
// mod fcc;
use std::sync::{Arc, Mutex};
use std::str::from_utf8;
use chrono::{Local, Utc};
use regex::Regex;
const appname:&str="dude";
fn listallkeys(){
    let db = sled::open(dirs::data_local_dir().unwrap().join(appname)).expect("open");
    // let tree = db.open_tree("my_tree");
    // let date = Local::now();
    let today = Utc::now();
    // let current_date = date.format("%Y-%m-%d").to_string();
    // // println!("{}",current_date);
    // let date_28_days_ago = &(today - chrono::Duration::days(27)).format("%Y-%m-%d").to_string();
    // let date_yesterday = &(today - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    // let date_today = &(today ).format("%Y-%m-%d").to_string();
    println!("{}",db.len());
    let mut  totaldata=0 as u128;
    let mut daywithdata=0;
    for i in 1..800{  
        let datetofetch=&(today - chrono::Duration::days(i)).format("%Y-%m-%d").to_string();
        match db.get(bincode::serialize(&datetofetch).unwrap()).unwrap() {
            Some(bytes) => {
                let k:f64 = bincode::deserialize(&bytes).unwrap();
                totaldata+=k as u128;
                daywithdata+=1;
                println!("{}---{}",datetofetch, byte_unit::Byte::from_bytes(k as u128).get_appropriate_unit(true));
                // play with this struct here
            },
            None => {
                },
        };  
        
    }
    println!("{}",byte_unit::Byte::from_bytes(totaldata as u128).get_appropriate_unit(true));
        println!("{}",byte_unit::Byte::from_bytes(totaldata/daywithdata as u128).get_appropriate_unit(true));
    
    // println!("keyslist");
    // for key in db.iter().keys(){
    //     // println!("{:?}",key);

    //     match(key){
    //         Ok(val) => {
    //             let k:f64 = bincode::deserialize(&val).unwrap();
    //             println!("{}",k)
    //         },
    //         Err(_) => {
                
    //         },
    //     }
        
    // }
    // println!("----------------");


}
fn main() {
    // listallkeys();
//    use std::sync::{Arc, Mutex};
//    use std::thread;
//    use std::time::Duration;
//
//    let data = Arc::new(Mutex::new([0.0, 0.0]));
//    let data_for_thread = data.clone();
//    thread::spawn(move || {
//        loop {
//            thread::sleep(Duration::from_secs(5))
//            let mut data = data_for_thread.lock().unwrap();
//            data[0] += 1.0;
//            data[1] -= 1.0;
//        }
//    });
//
//    loop {
//        let data = data.lock().unwrap();
//        println!("{}, {}", data[0], data[1]);
//    }
//    let mut i=SystemTime::now();
//
//    while true{
//    let (tx, rx) = mpsc::channel();
//    thread::spawn(move || {
////        for i in 1..10{
////        addi1(&mut i);
////        let k=i % SystemTime::now;
//        for i in 1..10{
//
//let val = String::from(format!("hi{} ----{:?}",i,SystemTime::now()));
////        println!("{}",val);
//        tx.send(val).unwrap();
//        }
////        thread::sleep(Duration::from_secs(1));
////        }
////        let val = String::from(format!("hi {:?}",SystemTime::now()));
////        println!("{}",val);
////        tx.send(val).unwrap();
////        thread::sleep(Duration::from_secs(1));
//
//    });
//
//    thread::spawn(move || {
////        let received = rx.recv().unwrap();
//        for ry in rx{
//        println!("Got: {} ", ry);
//        }
//        thread::sleep(Duration::from_secs(1));
//    });
//    thread::sleep(Duration::from_secs(1));
//}
//    writeln!(io::stdout().lock(), "{}", output);
//    let mut ot=String::new();
    // let (mut tx, rx1) = spmc::channel();
    // let (tx, rx) = mpsc::channel();
    let tree = sled::open(dirs::data_local_dir().unwrap().join(appname)).expect("open");
    // let diskname="nvme1n1";
    let mut fd = File::open(&"/proc/diskstats").unwrap();
    let mut found =false;
    let mut io_data = String::new();
            // Read the content of the file (diskstats) to the io_data string
            fd.read_to_string(&mut io_data).unwrap();
            let mut lcs=vec![];
            let mut lc=0;
            // io_data.lines().map(|s|{
            //     if(s==diskname){
            //         s
            //     }
            //     }).collect();
            let re = Regex::new(r"^((sd[a-z]+)|(hd[a-z]+)|(nvme\d+n\d+))$").expect("failed to compile regex");
            // let disk_regex = Some(&re);
            for line in io_data.lines(){
                let fields = line.split_whitespace().collect::<Vec<_>>();// If the are less than 14 fields, the file is missing data
                // see (https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats)
                if fields.len() < 14 {
                    panic!("Not enough data from diskstats");
                    process::exit(0);
                }
                // if fields[2]==diskname{
                if re.is_match(fields[2]){
                    println!("{}...{}",lc,line);
                    lcs.push(lc);
                    found=true;
                    // break;
                }
                lc+=1;
            }
            // lc-=1;
            println!("{:?}",lcs);
            // lc=5;

    if(!found){
        process::exit(0);
    }
    let mut prev: IoStats = IoStats{mb_read:0.0,mb_wrtn:0.0};
     print!("here");
 //     for row in tree.iter(){
	// 	let (key, val) = row.clone().unwrap();
 //                                let k:f64 = bincode::deserialize(&val).unwrap();
 //                                let k1:String = bincode::deserialize(&key).unwrap();
//            	// print!("{}.....{}",k1, k);
	// }
    
    let mut uptoprevsession: IoStats = IoStats{mb_read:0.0,mb_wrtn:0.0};
    let mut firsttime = true;
    let mut g=0;
    let mut perminute=0.0;
    
    let data= Arc::new(Mutex::new(String::new()));
    
    // let rx=rx1.clone();
    let data_for_thread = data.clone();
    
    thread::spawn(move || loop {
        let lcsd=lcs.clone();
        // tx.send(updateusage(false,&mut val,&mut ptx,&mut prx,iname.clone()));
//    println!("fromhere------------>1");
       // 2048 is for mb
    // One sector is 512b and 1 sector is typically 512b
    // So we keep it in mind and when we'll read /proc/diskstats
    // We'll divide the number of sector read by 512 and then by 2048 for mb
    // And compute the difference to obtain mb/s.
    let fctr = 512.0;
    // Hashmap of previous drives stats to compute difference from

    // Open the file we'll use to get the stats
    let mut fd = File::open(&"/proc/diskstats").unwrap();
    
//    let mut ot;
//    loop {
        // Create the curr Hashmap, allow us to compare with the prev one
        let mut ds = IoStats {
                        mb_read: 0.0,
                        mb_wrtn: 0.0,
                    };
        let mut dsw=ds.mb_wrtn;
        let mut dsr=ds.mb_read;
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
            // print!("{lc}");
            for lc in lcsd
            {
                let line = io_data.lines().nth(lc).unwrap();
                {
                    //                println!("{}",line);
                    // Split field (separated by whitespace) and collect them without specific type
                    let fields = line.split_whitespace().collect::<Vec<_>>();
                    
                    
                    // If the are less than 14 fields, the file is missing data
                    // see (https://www.kernel.org/doc/Documentation/ABI/testing/procfs-diskstats)
                    if fields.len() < 14 {
                        panic!("Not enough data from diskstats");
                    }
                    dsr=dsr + (fields[5].parse::<f64>().unwrap()* fctr);
                    dsw=dsw + (fields[9].parse::<f64>().unwrap()* fctr);
                    ds = IoStats {
                        mb_read: dsr ,
                        mb_wrtn: dsw ,
                    };
                }
            }
                    if(firsttime){
                        uptoprevsession=ds.clone();
                        firsttime=false;
                    }
                    let upsr=ds.mb_read-uptoprevsession.mb_read;
                    let upsw=ds.mb_wrtn-uptoprevsession.mb_wrtn;
                    
                    let date = Local::now();
                    let current_date = date.format("%Y-%m-%d").to_string();
                    
                        // if (tree.get(bincode::serialize(&current_date).unwrap()).unwrap().is_none()){
                        //     wbf=0.0
                        // }
                    // If prev already contains the info we compute the diff to get mb/s
                    // Else we add to the print line the "fake" data.
                        
                    if prev.mb_read !=0.0 {
                        
                        //                    println!("here");
                        // Get the object from the hashmap
                        // Construct speed line and append it to curr hashmap
                        let mb_read_s = ds.mb_read - prev.mb_read;
                        let mb_wrtn_s = ds.mb_wrtn - prev.mb_wrtn;
                            perminute+=mb_wrtn_s;
                                    let date = Local::now();
                                let current_date = date.format("%Y-%m-%d").to_string();
                                let wbf=
                                        match tree.get(bincode::serialize(&current_date).unwrap()).unwrap() {
                                            Some(bytes) => {
                                                let k:f64 = bincode::deserialize(&bytes).unwrap();
                                                k
                                                // play with this struct here
                                            },
                                            None => {
                                                0.0 as f64
                                                },
                                        };  
                            if g>60{
                                
                                let tosave=wbf+perminute;
                                let bytes_r = bincode::serialize(&current_date).unwrap();
                                let bytes_w = bincode::serialize(&tosave).unwrap();
                                tree.insert(bytes_r,bytes_w).expect("cannot insert");
                                perminute=0.0;
                                g=0;
                                
                            }
                        // let tw=tree.get(bincode::serialize(&current_date).unwrap()).ok().unwrap().unwrap();
                        
                        // let date = Local::now();
                        //     let current_date = date.format("%Y-%m-%d").to_string();
                        //     let wbf=
                        //             match tree.get(bincode::serialize(&current_date).unwrap()).unwrap() {
                        //                 Some(bytes) => {
                        //                     let k:f64 = bincode::deserialize(&bytes).unwrap();
                        //                     k
                        //                     // play with this struct here
                        //                 },
                        //                 None => {
                        //                     0.0 as f64
                        //                     },
                        //             }; 
                        // print!("{}",wbf);
                        
                        // Add the line, formatted with color and spacing
                        //                    output.push_str(&format!("{:2} {:10.2} {:15.2}\n", fields[2], mb_read_s, mb_wrtn_s));
                        // output.push_str(&format!("{:2} {:2.2}ps {:2.2}ps\n", fields[2], byte_unit::Byte::from_bytes(mb_read_s as u128).get_appropriate_unit(true), byte_unit::Byte::from_bytes(mb_wrtn_s as u128).get_appropriate_unit(true)));
                        // output.push_str(&format!("SR:{:2.2} SW:{:2.2} R:{:2.2}ps W:{:2.2}ps", 
                            
                            
                        output.push_str(
                            // &format!("WT:{:2.2} SR:{:2.2} SW:{:2.2} R:{:2.2}ps W:{:2.2}ps", 
                            &format!("WT:{:2.2} R:{:2.2}ps W:{:2.2}ps", 
                                byte_unit::Byte::from_bytes(wbf as u128).get_appropriate_unit(true),
                            // byte_unit::Byte::from_bytes(upsr as u128).get_appropriate_unit(true),
                            // byte_unit::Byte::from_bytes(upsw as u128).get_appropriate_unit(true),
                            byte_unit::Byte::from_bytes(mb_read_s as u128).get_appropriate_unit(true), 
                            byte_unit::Byte::from_bytes(mb_wrtn_s as u128).get_appropriate_unit(true)));
                        // Insert the current disk data to the curr HashMap
                        // the curr will later be saved as prev
                        //                    curr = ds;
                    } else {
                        //                    println!("here2");
                        // Add the line with fake data and formatting
                        // output.push_str(&format!("{} {}B {}B\n", fields[2], 0.00, 0.00));
                        output.push_str(&format!("{}B {}B\n", 0.00, 0.00));
                        //                    curr= IoStats{mb_read:0.0,mb_wrtn:0.0};
                    
                
            }
            // Move the cursor to the start of the file
            fd.seek(SeekFrom::Start(0)).unwrap();
        }

        // Print the result
        //        writeln!(io::stdout().lock(), "{}", output);
//        print!("{}", output);
//        while let Ok(value) = rx.try_recv() {
//            println!("received {}", value);
//        }
       //  while let Ok(value) = rx.try_recv() {
       //                              // println!("received {}", value);
       //                          }
       //  // tx.send(format!("{:?}",SystemTime::now())).unwrap();
       // tx.send(output.clone()).unwrap();
       let mut data=data_for_thread.lock().unwrap();
 //       for row in tree.iter(){
	// 	let (key, val) = row.clone().unwrap();
 //            	let k:f64 = bincode::deserialize(&val).unwrap();
 //                                let k1:String = bincode::deserialize(&key).unwrap();
 //            	output.push_str(&format!("{}.....{}",k1,byte_unit::Byte::from_bytes(k as u128).get_appropriate_unit(true)));
 //            	// print!()
	// }
       // let date = Local::now();
       //                      let current_date = date.format("%Y-%m-%d").to_string();
       //                      let wbf=
       //                              match tree.get(bincode::serialize(&current_date).unwrap()).unwrap() {
       //                                  Some(bytes) => {
       //                                      let k:f64 = bincode::deserialize(&bytes).unwrap();
       //                                      k
       //                                      // play with this struct here
       //                                  },
       //                                  None => {
       //                                      0.0 as f64
       //                                      },
       //                              }; 
       //  output.push_str(&format!("{}",byte_unit::Byte::from_bytes(wbf as u128).get_appropriate_unit(true)));
       *data=output;
//        *ot=output;
        // Save current as previous for the next loop
        // let dsd=ds.clone();
        prev = ds;
        
        // Wait for 1 seconds to respect the mb/s

//}
        g+=1;
        thread::sleep(Duration::from_secs(1));
    });

    // let rx=rx1.clone();
    // let data =
    match TcpListener::bind("127.0.0.1:8971") {
        Ok(listener) =>{

            for stream in listener.incoming(){
                // println!("{:?}",stream);
                let stream = stream.unwrap();
                // let ts=stream.peer_addr().unwrap().to_string();
                // println!("{}",format!("conctd to {}",stream.peer_addr().unwrap()));
                // tp.send(&ts).unwrap();
                // tp.push_str(&format!("conctd to {}",stream.peer_addr().unwrap()));
                // thread::spawn(|| {
                let mut recieved=String::new();
                //drain the mpsc
                // while let Ok(value) = rx.try_recv() {
                //                     println!("received {}", value);
                //                 }

                // for g in rx.recv(){
                //     recieved=g.to_string();
                // }
//                let received = rx.recv().unwrap();
                // handle_con(stream,&recieved);
                handle_con(stream,&data.lock().unwrap());
                
                // });
            }
        },
        Err(e) =>{
            println!("Internet issue.\n Error:{}",e)
        }
    }
}
#[derive(Clone, Debug)]
struct IoStats {
    pub mb_read: f64,
    pub mb_wrtn: f64,
}
fn handle_con(mut stream:TcpStream,rcv:&String){

    let buf_reader = std::io::BufReader::new(&mut stream);
    // println!("{:?}",buf_reader);
    let request_line = match buf_reader.lines().next() {
        None => "".to_string(),
        Some(secondline) => {
            match secondline {
                Ok(reqline)  => reqline,
                Err(e) => "".to_string(),
            }
        },
    };
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
    // println!("req------>{request_line}");
    // println!("---->{}",request_line);

    // let mut headers = [httparse::EMPTY_HEADER; 16];
    // let mut req = httparse::Request::new(&mut headers);
    // let res = req.parse(request_line.as_bytes()).unwrap();
    // if res.is_partial() {
    //     match req.path {
    //         Some(ref path) => {

    //         },
    //         _=>{

    //         }
    //     }
    // }
    // let mut ptx=*ptx;
    // let mut prx=*prx;
    let (status_line, filecontent,contentheader) =
           ("HTTP/1.1 200 OK", &rcv,String::from("Content-Type: text/plain"));

            // ("HTTP/1.1 200 OK", serde_json::to_string_pretty(&format!("{}\n{:?}",&rcv,SystemTime::now())).unwrap(),String::from("Content-Type: application/json"));

    let response =
        format!("{status_line}\r\n{contentheader}\r\n\r\n{filecontent}");
    match stream.write(response.as_bytes()) {
        Ok(file) => {

        },
        Err(error) =>{
            return ;
        },
    };match stream.flush() {
        Ok(file) => {

        },
        Err(error) =>{
            return ;
        },
    };
    // stream.write(response.as_bytes()).unwrap();
}