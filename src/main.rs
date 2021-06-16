use serde::Deserialize;
use std::fs::File;
use std::io::Write;

/*
Intuition: The problem is very similar to 0-1 Knapsack in a way.
           We need to maximize the fee while constraining the weight to HIGHEST_WT i.e. minimizing the weight.
           So to order the records we can sort it according to the ratio of fee/wt.
           Also if there is a unknown parent transaction we simply reject it.
*/


const HIGHEST_WT: i64 = 4000000;

#[derive(Deserialize, Debug)]
struct Record {
    tx_id: String,
    fee: i64,
    weight: i64,
    parents: Option<String>,
}

fn get_valid_txn(csv: &Vec<Record>) -> Vec<&Record> {
    let mut valid_tnx: Vec<&Record> = Vec::new();
    let mut t_wt = 0;
    for i in csv {
        t_wt = t_wt + i.weight;
        if t_wt <= HIGHEST_WT {
            if (i.parents.is_some()) {
                let p = (i.parents).as_ref().unwrap();
                let mut parent_list: Vec<&str> = p.split(";").collect();
                for j in parent_list {
                    if csv.iter().any(|r| r.tx_id.to_string() == j) {
                        valid_tnx.push(i)
                    }
                }
            } else {
                valid_tnx.push(i);
            }
        }
    }
    valid_tnx
}

fn main() {
    let mut reader = csv::Reader::from_path("mempool.csv").expect("Cannot read file");
    let mut rows: Vec<Record> = Vec::new();
    for record in reader.deserialize() {
        let i: Record = record.expect("Record Err");
        rows.push(i)
    }

    //Sort according to fee/weight ratio, MAX Fee and MIN Weight
    rows.sort_by(|a, b| b.fee.cmp(&a.fee).then_with(|| a.weight.cmp(&b.weight)));

    //Get Valid Transactions
    let valid_txn = get_valid_txn(&rows);

    //File IO helper
    let mut file = File::create("block.txt").unwrap();
    let mut w = 0;
    let mut f = 0;
    for i in valid_txn {
        w = w + i.weight;
        f = f + i.fee;
        writeln!(&mut file, "{}", i.tx_id).unwrap()
    }
    println!("Total Weight = {}", w);
    println!("Total Fee = {}", f);
}