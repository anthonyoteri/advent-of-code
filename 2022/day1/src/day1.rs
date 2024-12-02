use std::io::Error;

/// ```
/// # use aoc_2022::day1::count_cals;
///
/// let sample_input: Vec<String> = vec![
///     "1000".to_owned(),
///     "2000".to_owned(),
///     "3000".to_owned(),
///     "".to_owned(),
///     "4000".to_owned(),
///     "".to_owned(),
///     "5000".to_owned(),
///     "6000".to_owned(),
///     "".to_owned(),
///     "7000".to_owned(),
///     "8000".to_owned(),
///     "9000".to_owned(),
///     "".to_owned(),
///     "10000".to_owned(),
/// ];
/// assert_eq![count_cals(sample_input.into_iter()).unwrap(), [
///     6000,
///     4000,
///     11000,
///     24000,
///     10000,
/// ]];
/// ```
pub fn count_cals<Iter>(lines: Iter) -> Result<Vec<u32>, Error>
where
    Iter: Iterator<Item = String>,
{
    let mut result = Vec::<u32>::new();

    let mut subtotal: u32 = 0;

    for line in lines {
        if line.is_empty() {
            result.push(subtotal);
            subtotal = 0;
        } else {
            subtotal += line.parse::<u32>().unwrap();
        }
    }

    result.push(subtotal);

    Ok(result)
}

/// ```
/// # use aoc_2022::day1::find_largest;
///
/// let sample_input = vec![
///     6000,
///     4000,
///     11000,
///     24000,
///     10000,
/// ];
///
/// assert_eq!(find_largest(&sample_input), 24000);
/// ```
pub fn find_largest(inventory: &[u32]) -> u32 {
    let mut local_inventory = inventory.to_vec();
    local_inventory.sort_by(|a, b| b.cmp(a));
    local_inventory.into_iter().max().unwrap()
}

/// ```
/// # use aoc_2022::day1::find_sum_of_3;
///
/// let sample_input = vec![
///     6000,
///     4000,
///     11000,
///     24000,
///     10000,
/// ];
///
/// assert_eq!(find_sum_of_3(&sample_input), 45000);
/// ```
pub fn find_sum_of_3(inventory: &[u32]) -> u32 {
    let mut local_inventory = inventory.to_vec();
    local_inventory.sort_by(|a, b| b.cmp(a));
    local_inventory[..3].iter().sum()
}
