pub fn median(data: & mut Vec<i128>) -> i128 {
    data.sort();
    if data.len() % 2 == 0 {
        let middle_cursor = data.len() / 2;
        return (data[middle_cursor - 1] + data[middle_cursor]) / 2
    } else {
        return data[data.len() / 2];
    }     
}