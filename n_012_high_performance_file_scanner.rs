use memmap2::Mmap;
use std::{error::Error, fs::File, sync::Arc, thread, time::Instant};

/*
 * Command to generate the market_data.txt file
 * It generates 10 million data lines
 * awk 'BEGIN {
 *    srand();
 *    print "timestamp,symbol,price,qty";
 *    for (i=1; i<=10000000; i++) {
 *      printf "%d,SYM%d,%.2f,%d\n",
 *        systime(),
 *        int(rand()*100),
 *        50 + rand()*500,
 *        int(rand()*1000)+1
 *    }
 *  }' > market_data.txt
 */

fn mmap_file(path: &str) -> Result<Mmap, Box<dyn Error>> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(mmap)
}

fn parse_qty(line: &[u8]) -> Option<u32> {
    let mut fields = line.split(|b| *b == b',');
    fields.next()?;
    fields.next()?;
    fields.next()?;
    let qty = fields.next()?;
    str::from_utf8(qty).ok()?.parse().ok()
}

pub fn run() {
    println!("High Performance File Scanner");
    // 22 sec => 1 thread
    // 6 sec => 4 thread => No of cpus in my pc
    // 10 sec => 8 thread
    let start = Instant::now();

    let threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    println!("Using no of threads: {}", threads);

    let mmap = Arc::new(mmap_file("market_data.txt").unwrap());
    let len = mmap.len();
    let chunk_size = len / threads;
    let mut handles = vec![];

    for i in 0..threads {
        let bytes = Arc::clone(&mmap);

        let start = i * chunk_size;
        let mut end = if i == threads - 1 {
            len
        } else {
            (i + 1) * chunk_size
        };

        while end < len && bytes[end] != b'\n' {
            end += 1;
        }

        let handle = thread::spawn(move || {
            let chunk = &bytes[start..end];
            let mut total = 0u64;
            let mut big = 0u64;

            for line in chunk.split(|b| *b == b'\n') {
                if let Some(qty) = parse_qty(line) {
                    total += qty as u64;
                    if qty > 500 {
                        big += 1;
                    }
                }
            }
            (total, big)
        });

        handles.push(handle);
    }
    let mut total_qty = 0u64;
    let mut big_trades = 0u64;
    for handle in handles {
        let (t, b) = handle.join().unwrap();
        total_qty += t;
        big_trades += b;
    }

    let duration = start.elapsed();
    println!("Total Qty: {}", total_qty);
    println!("Big Trades: {}", big_trades);
    println!("Time taken: {:.3?}", duration);

    println!("_____________________________");
}
