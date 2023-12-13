use anyhow::anyhow;



pub struct Calcu {
}

impl Calcu {
    pub fn calcu (vals:Vec<u64>) -> u64 {

        let mut total_value:u64 = 0;

        for val in vals.iter(){
            total_value += val;
        }
        return total_value
    }
}