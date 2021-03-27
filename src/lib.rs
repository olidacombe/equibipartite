#[derive(Debug, PartialEq)]
pub struct Partition {
    pub left: Vec<i64>,
    pub right: Vec<i64>,
}

impl Partition {
    pub fn from_subset(collection: Vec<i64>, subset: Vec<i64>) -> Self {
        let mut subset_clone = subset.clone();
        let mut right: Vec<i64> = vec![];

        for v in collection.iter().rev() {
            match subset_clone.last() {
                None => {
                    break;
                }
                Some(x) if x == v => {
                    subset_clone.pop();
                }
                Some(_) => {
                    right.push(v.clone());
                }
            };
        }

        Self {
            left: subset,
            right: vec![],
        }
    }
}

struct SubsetSearchOpts {
    collection: Vec<i64>,
    required_sum: i64,
    complement_not_exceeding: i64,
}

fn get_subset_with_sum(
    SubsetSearchOpts {
        collection,
        required_sum,
        complement_not_exceeding,
    }: SubsetSearchOpts,
) -> Option<Vec<i64>> {
    if collection.len() > 1 {}
    match collection.first() {
        None => match required_sum {
            0 => Some(collection),
            _ => None,
        },
        _ => {
            let mut complement_sum: i64 = 0;
            for i in 0..collection.len() {
                let mut collection = collection.clone();
                let v = collection.remove(i);
                let subset = match v {
                    x if x > required_sum => None,
                    _ => get_subset_with_sum(SubsetSearchOpts {
                        collection: collection.clone(),
                        required_sum: required_sum - v,
                        complement_not_exceeding,
                    }),
                };
                match subset {
                    None => {
                        complement_sum += v;
                        if complement_sum > complement_not_exceeding {
                            return None;
                        }
                    }
                    Some(mut subset) => {
                        subset.push(v);
                        return Some(subset);
                    }
                };
            }
            None
        }
    }
}

pub fn get_equi_partition(collection: Vec<i64>) -> Option<Partition> {
    if collection.is_empty() {
        return None;
    }
    let mut collection = collection.clone();
    collection.sort();
    collection.reverse();
    let sum: i64 = collection.iter().sum();
    if sum % 1 == 1 {
        return None;
    }
    let half_sum = sum / 2;
    match get_subset_with_sum(SubsetSearchOpts {
        collection,
        required_sum: half_sum,
        complement_not_exceeding: half_sum,
    }) {
        Some(subset) => Some(Partition::from_subset(collection, subset)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_when_no_equi_partition_exists() {
        assert!(get_equi_partition(vec![]).is_none());
        assert!(get_equi_partition(vec![2]).is_none());
        assert!(get_equi_partition(vec![1, 2]).is_none());
        assert!(get_equi_partition(vec![1, 1, 2, 6]).is_none());
        assert!(get_equi_partition(vec![1, 2, 3, 4, 5]).is_none());
    }

    #[test]
    fn it_returns_equi_partition_when_exists() {
        assert_eq!(
            get_equi_partition(vec![1, 1]),
            Some(Partition {
                left: vec![1],
                right: vec![1]
            })
        );

        let test_cases = vec![vec![1, 2, 3], vec![1, 2, 2, 3], vec![10, 1, 2, 3, 4]];

        for test_case in test_cases {
            let partition = get_equi_partition(test_case).unwrap();
            let sum_left: i64 = partition.left.iter().sum();
            let sum_right: i64 = partition.left.iter().sum();
            assert_eq!(sum_left, sum_right);
        }
    }
}
