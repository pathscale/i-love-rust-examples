use tokio::sync::mpsc;
use anyhow::{Result, Context};
use crate::YourDataStruct;
#[allow(unused_imports)]
use tracing::*;
use peroxide::prelude::{DataFrame, Series, TypedVector, Printable};

//expensive computation, runs on the rayon thread pool
pub fn thread2(mut rx_t1: mpsc::Receiver<String>, tx_t2: mpsc::Sender<YourDataStruct>) -> Result<()> {
    while let Some(body) = rx_t1.blocking_recv() { //get the result from the previous thread.
        let data: ijson::IValue = serde_json::from_str(&body)?;
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
        let mut id = vec![];
        let mut name = vec![];
        let mut username = vec![];
        let mut email = vec![];
        let mut street = vec![];
        let mut suite = vec![];
        let mut city = vec![];
        let mut zipcode = vec![];
        let mut lat = vec![];
        let mut lng = vec![];
        let mut phone = vec![];
        let mut website = vec![];
        let mut company_name = vec![];
        let mut company_catch_phrase = vec![];
        let mut company_bs = vec![];
        let mut insertion_time = vec![];
        fn extract_string(ser: &mut Vec<String>, item: &ijson::IValue, key: &str) -> anyhow::Result<()> {
            ser.push(item.get(key).with_context(|| format!("Missing {}", key))?.as_string().with_context(|| format!("{} is not string", key))?.to_string());
            Ok(())
        }
        for item in data.as_array().context("Is not array")? {
            info!("Parsed data {:?}", item);
            id.push(item.get("id").context("Missing id")?.as_number().context("Is not number")?.to_i64().context("Is not i64")?);
            extract_string(&mut name, item, "name")?;
            extract_string(&mut username, item, "username")?;
            extract_string(&mut email, item, "email")?;
            let address = item.get("address").context("Missing address")?;
            extract_string(&mut street, address, "street")?;
            extract_string(&mut suite, address, "suite")?;
            extract_string(&mut city, address, "city")?;
            extract_string(&mut zipcode, address, "zipcode")?;
            extract_string(&mut website, item, "website")?;
            extract_string(&mut phone, item, "phone")?;
            let geo = address.get("geo").context("Missing geo")?;
            extract_string(&mut lat, geo, "lat")?;
            extract_string(&mut lng, geo, "lng")?;
            insertion_time.push(chrono::Utc::now().to_rfc3339());
            let company = item.get("company").context("Missing company")?;
            extract_string(&mut company_name, company, "name")?;
            extract_string(&mut company_catch_phrase, company, "catchPhrase")?;
            extract_string(&mut company_bs, company, "bs")?;
        }
        let frame = DataFrame::new(
            vec![
                Series::new(id),
                Series::new(name),
                Series::new(username),
                Series::new(email),
                Series::new(street),
                Series::new(suite),
                Series::new(city),
                Series::new(zipcode),
                Series::new(lat),
                Series::new(lng),
                Series::new(phone),
                Series::new(website),
                Series::new(company_name),
                Series::new(company_catch_phrase),
                Series::new(company_bs),
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
