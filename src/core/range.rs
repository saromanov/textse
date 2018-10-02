
// DateRange provides query of documents with date range
// based on unix timestamp
// For example: DateRange {
// start_date: 1535227662,
// end_date: 1535227762
//}

pub struct DateRange {
    start_date: i64,
    end_date: i64,
}

impl DateRange {
    fn new(start_date: i64, end_date: i64) -> Self {
        DateRange {
            start_date: start_date,
            end_date: end_date,
        }
    }

    fn get_start_date(&self) -> i64 {
        self.start_date
    }

    fn get_end_date(&self) -> i64 {
        self.end_date
    }
}


// NumRange provides query of documents with numeric range
// For example: NumRange {
// start: 1,
// end: 10,
// field_name: "cool_number"
//}

pub struct NumRange {
    field_name: String,
    start: i64,
    end: i64,
}

impl NumRange {
    
    // return documents based on query
    fn execute(&self) -> Self {
        for schema in schema_data.iter(){
            if schema in self.schema_data[self.field_name] {

            }
        }
    }
}
