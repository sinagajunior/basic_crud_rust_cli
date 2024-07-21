use postgres::{Client, Error, NoTls};
use std::io;

fn main() -> Result<(), Error> {
    let mut client: Client = Client::connect(
        "postgres://postgres:postgres@localhost:5432/postgres",
        NoTls,
    )?;
    println!("");
    println!("");
    println!("+++++++++++++++++++++++++++");
    println!("Welcome to Rust database CLI CRUD");
    println!("");
    println!("");
    while "true" == "true" {
        println!("1. Insert data");
        println!("2. Edit data");
        println!("3. Delete data");
        println!("4. Display data");
        println!("5. Query data");
        println!("6. exit");
        let mut menu_item = String::new();
        io::stdin()
            .read_line(&mut menu_item)
            .expect("failed to read line");

        match menu_item.trim().parse::<i32>().unwrap() {
            1 => {
                println!("");
                println!("****************");
                println!("Input data");
                println!("****************");
                let mut id = String::new();
                println!("enter id");
                io::stdin().read_line(&mut id).expect("failed to readline");
                let mut username = String::new();
                println!("enter the person name below");
                io::stdin()
                    .read_line(&mut username)
                    .expect("failed to readline");

                let data = String::from("data 1");
                client.execute(
                    "INSERT INTO person(id,name,data) VALUES ($1,$2,$3)",
                    &[&id, &username, &data],
                )?;
                println!("");
                println!("Request successfull");
            }
            2 => {
                println!("");
                println!("************");
                println!("Edit data");
                println!("");
                for row in client.query("Select id,name,data FROM person", &[])? {
                    let id: i32 = row.get(0);
                    let name: &str = row.get(1);
                    println!("{} | {}", id, name);
                }
                print!("");
                println!("enter person id:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("failed readline");
                println!("");

                println!("Enter new person:");
                let mut newusername = String::new();
                io::stdin()
                    .read_line(&mut newusername)
                    .expect("error input");

                let index = index.trim().parse::<i32>().unwrap();
                client.execute(
                    "UPDATE person set name=$1 where id=$2",
                    &[&newusername, &index],
                )?;
                println!("Row successfully update");
            }

            3 => {
                println!("");
                println!("***********");
                println!("Delete Data");
                println!("***********");

                println!("Enter an id:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Please enter a number");
                let index = index.trim().parse::<i32>().unwrap();
                client.execute(
                    "DELETE FROM person
        WHERE id = $1",
                    &[&index],
                )?;
                println!("Delete was successfully");
            }
            4 => {
                println!("");
                println!("***********");
                println!("Display All Data");
                println!("***********");
                for row in client.query("SELECT id, name, data FROM person", &[])? {
                    let id: &str = row.get(0);
                    let name: &str = row.get(1);
                    let data: &str = row.get(2);

                    println!("{} | {} | {} ", id, name, data);
                }
            }
            5 => {
                println!("");
                println!("***********");
                println!("Query a row");
                println!("***********");
                println!("");
                println!("Enter an id:");
                let mut index = String::new();
                io::stdin()
                    .read_line(&mut index)
                    .expect("Please enter a number");
                let index = index.trim().parse::<i32>().unwrap();
                for row in
                    client.query("SELECT  name, data FROM person  WHERE id =$1", &[&index])?
                {
                    let name: &str = row.get(0);
                    // let data: Option<&[u8]> = row.get(1);

                    println!("Selected row has {}  ", name);
                }
            }
            6 => {
                break;
            }
            _ => println!("Rest of the number"),
        }
        //    break;
    }

    Ok(())
}
