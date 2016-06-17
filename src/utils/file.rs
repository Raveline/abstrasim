use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Read;
use std::io::Error;
use model::world::World;

// Returns the file content as a string
pub fn read_file(filepath: &str) -> String {
    let path = Path::new(filepath);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, &why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, &why),
        Ok(_) => s
    }
}

pub fn write_stock(filepath: &str, world: &World) -> Result<(), Error> {
    let path = Path::new(filepath);
    let mut f = File::create(&path).unwrap();
    try!(f.write(b"Sector;Name\n"));
    for (sec, biz) in &world.sectors {
        for business in biz {
            let begin = format!("{};{};", sec, business.name);
            let values_as_str : Vec<String>= world.stocks.
                get_all(&business.name).iter().map(|x| x.to_string()).collect();
            let end = values_as_str.join(";");
            let end2 = end + &"\n";
            let final_line = begin + &end2;
            try!(f.write(&final_line.into_bytes()));
        }
    }
    try!(f.sync_all());
    Ok(())
}

pub fn write_report(filepath: &str, world: &World) -> Result<(), Error>{
    let path = Path::new(filepath);
    let mut f = File::create(&path).unwrap();
    try!(f.write(b"Name;Performance;Perceived;Shares outstanding;Market cap;Current value"));
    for (_, biz) in &world.sectors {
        for business in biz {
            let line = format!(
                "{};{};{};{};{};{}\n", business, business.performance,
                business.perception, business.shares_outstanding,
                business.capitalisation, business.get_current_stock_value()
            );
            try!(f.write(&line.into_bytes()));
        }
    }
    try!(f.sync_all());
    Ok(())
}
