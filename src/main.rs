use mongodb::{Client, options::ClientOptions, Collection};
use rocksdb::{DB, Options};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
  id: u64,
  sat: u64
}

#[derive(Debug, Serialize, Deserialize)]
struct Object {
  id: u64,
  data: Vec<u8>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let path = "rocks.db";
  {
    let db = DB::open_default(path)?;
    let collection = mongo("mongodb://localhost:27017").await?; 

    fill_rocks(&db)?;
    for data in db.full_iterator(rocksdb::IteratorMode::Start) {
      let data = data?;
      let obj = bincode::deserialize::<Object>(data.1.as_ref())?;
      println!(
        "{:?}: {:?}",
        bincode::deserialize::<Transaction>(data.0.as_ref())?,
        obj
      );
      collection.insert_one(obj, None).await?;
    }
  }
  DB::destroy(&Options::default(), path)?;
  Ok(())
}

fn fill_rocks(db: &DB) -> anyhow::Result<()> {
  for i in 0..10{
    db.put(
      bincode::serialize(
        &Transaction {
          id: i as u64,
          sat: i as u64 * 12
        }
      )?,
      bincode::serialize(
        &Object {
          id: i as u64,
          data: vec![i, i + 1, i + 3]
        }
      )?
    )?;
  }
  Ok(())
}

async fn mongo(addr: &str) -> anyhow::Result<Collection<Object>> {
  let client_options = ClientOptions::parse(addr).await?;
  let client = Client::with_options(client_options)?;
  let db = client.database("sui");
  let collection = db.collection::<Object>("objects");
  
  Ok(collection)
}
