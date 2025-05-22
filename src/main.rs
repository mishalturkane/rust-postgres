use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:qwerty@localhost:5432/keynvalues",
        NoTls,
    )?;

    // Insert a new key-value pair
    client.batch_execute(
        "INSERT INTO keyandvalues (key, value) VALUES ('mishal', '1');"
    )?;

    println!("Insert query successfully done");

    // Select and print all rows from the table
    for row in client.query("SELECT * FROM keyandvalues", &[])? {
        let key: &str = row.get("key");
        let value: &str = row.get("value");

        println!("Key: {}, Value: {}", key, value);
    }

    Ok(())
}
