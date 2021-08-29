use tokio::sync::mpsc;
use anyhow::Result;
use crate::YourDataStruct;
#[allow(unused_imports)]
use tracing::*;
use peroxide::prelude::{DataFrame, Series, TypedVector, Printable};
use serde::{Serialize, Deserialize};

//expensive computation, runs on the rayon thread pool
pub fn thread2(mut rx_t1: mpsc::Receiver<String>, tx_t2: mpsc::Sender<YourDataStruct>) -> Result<()> {
    while let Some(body) = rx_t1.blocking_recv() {
        //get the result from the previous thread.
        #[derive(Debug, Serialize, Deserialize)]
        struct PersonInfo {
            id: i64,
            name: String,
            username: String,
            email: String,
            address: PersonInfoAddress,
            phone: String,
            website: String,
            company: PersonInfoCompany,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct PersonInfoAddress {
            street: String,
            suite: String,
            city: String,
            zipcode: String,
            geo: Geo,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct Geo {
            lat: String,
            lng: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
        struct PersonInfoCompany {
            name: String,
            #[serde(rename = "catchPhrase")]
            catch_phrase: String,
            bs: String,
        }

        let data: Vec<PersonInfo> = serde_json::from_str(&body)?;
        /*
         {
          "id": 10,
          "name": "Clementina DuBuque",
          "username": "Moriah.Stanton",
          "email": "Rey.Padberg@karina.biz",
          "address": {"street": "Kattie Turnpike", "suite": "Suite 198", "city": "Lebsackbury", "zipcode": "31428-2261", "geo": {"lat": "-38.2386", "lng": "57.2232"}},
          "phone": "024-648-3804", "website": "ambrose.net", "company": {"name": "Hoeger LLC", "catchPhrase": "Centralized empowering task-force", "bs": "target end-to-end models"}
         }

         */
        let mut id: Vec<i64> = vec![];
        let mut name: Vec<String> = vec![];
        let mut username: Vec<String> = vec![];
        let mut email: Vec<String> = vec![];
        let mut phone: Vec<String> = vec![];
        let mut website: Vec<String> = vec![];
        let mut insertion_time: Vec<String> = vec![];

        for item in data {
            info!("Parsed data {:?}", item);
            id.push(item.id);
            name.push(item.name);
            username.push(item.username);
            email.push(item.email);
            website.push(item.website);
            phone.push(item.phone);

            // current insertion time
            insertion_time.push(chrono::Utc::now().to_rfc3339());
        }
        let frame = DataFrame::new(
            vec![
                Series::new(id),
                Series::new(name),
                Series::new(username),
                Series::new(email),
                Series::new(phone),
                Series::new(website),
                Series::new(insertion_time)]);
        frame.print();

        //do your first data processing here...
        tx_t2.blocking_send(YourDataStruct::default())?; //send the result to the next thread.
    }
    Ok(())
}

//expensive computation, runs on the rayon thread pool
pub fn thread3(mut rx_t2: mpsc::Receiver<YourDataStruct>, tx_t3: mpsc::Sender<YourDataStruct>) -> Result<()> {
    while let Some(data) = rx_t2.blocking_recv() { //get the result from the previous thread.
        //do your second data processing here...
        tx_t3.blocking_send(data)?; //send the result to the next thread.
    }
    Ok(())
}
