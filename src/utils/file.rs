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

pub fn write_report(filepath: &str, world: &World) -> Result<(), Error>{
    let path = Path::new(filepath);
    let mut f = File::create(&path).unwrap();
    try!(f.write(b"Name;Performance;Perceived;Shares outstanding;Market cap;Current value"));
    for (sec, biz) in &world.sectors {
        println!("Reporting on sector {}\n", sec);
        for business in biz {
            println!("\tReporting on business {}", business);
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
