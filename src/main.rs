use std::env::Args;
use std::iter::Skip;
use std::collections::HashMap;

fn main() {
	let mut arguments: Skip<Args> = std::env::args().skip(1);
	let key: String = arguments.next().unwrap();
	let value: String = arguments.next().unwrap();
	println!("The key is {} and the value is {}", key, value);
	let contents = format!("{}\t{}\n", key, value);
	std::fs::write("kv.db", contents).unwrap( );

	//This is how you would call the new() method from Database Struct below
	let database: Database = Database::new().expect("Database::new() crashed");

	
}

//struct is stack allocated
struct Database {
	//Since it was called at the top the file
	map: HashMap<String, String>,
}

//Use impl(mentation) that allows you to add methods to your Database struct, there are no classesi in Rust
impl Database {
	fn new() -> Result<Database, std::io::Error> {
		// read the kv.db file
		// let contents: String = match std::fs::read_to_string("kv.db") {
		// 	Ok(c: String) => c,
		// 	err(error: Error) => {
		// 		return Err(err or);
		// 	}
		// };
		//Shorthand for above
		//iterators in rust are lazy
		let mut map = HashMap::new();
		let contents: String = std::fs::read_to_string("kv.db")?;
		for line in contents.lines() {
			let mut chunks = line.splitn(2, '\t');
			let key = chunks.next().expect("No key");
			let value = chunks.next().expect("No key");
			map.insert(key, value);
		}
		// parse the string
		// populate our map
		Ok(Database{ map: map}) 
	}
}