//! Partition a list of integers into two equal-sum sublists.
//!
//! # Example
//!
//! ```
//! use equibipartite::get_equi_partition;
//!
//! let collection = vec![1, 4, 7, 35, 2, 1, 18, 6];
//! let partition = get_equi_partition(&collection);
//! println!("{:#?}", partition);   
//!
//! let collection = vec![1, 2, 3, 4, 5, 6];
//! assert!(get_equi_partition(&collection).is_none());
//! ```

use std::collections::HashMap;
use std::ops::SubAssign;

#[derive(Debug, PartialEq)]
pub struct Partition {
    pub left: Vec<i64>,
    pub right: Vec<i64>,
}

struct OccurrenceCount {
    counts: HashMap<i64, i64>,
}

impl OccurrenceCount {
    pub fn from_vec(set: &[i64]) -> Self {
        let mut counts = HashMap::new();

        for v in set {
            let count = counts.entry(*v).or_insert(0);
            *count += 1;
        }

        Self { counts }
    }

    pub fn to_vec(&self) -> Vec<i64> {
        let mut vec = vec![];
        for (k, v) in self.counts.iter() {
            for _ in 0..*v {
                vec.push(*k);
            }
        }
        vec
    }
}

impl SubAssign for OccurrenceCount {
    fn sub_assign(&mut self, other: Self) {
        for (k, v) in other.counts.iter() {
            if let Some(current_count) = self.counts.get_mut(k) {
                *current_count = (*current_count - *v).max(0);
            }
        }
    }
}

impl Partition {
    pub fn from_subset(collection: &[i64], subset: &[i64]) -> Self {
        let mut collection_counts = OccurrenceCount::from_vec(collection);
        let subset_counts = OccurrenceCount::from_vec(subset);

        collection_counts -= subset_counts;

        Self {
            left: subset.to_vec(),
            right: collection_counts.to_vec(),
        }
    }
}

struct SubsetSearchOpts<'a> {
    collection: &'a Vec<i64>,
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
    if required_sum == 0 {
        return Some(vec![]);
    }
    match collection.first() {
        None => match required_sum {
            0 => Some(collection.to_vec()),
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
                        collection: &collection,
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

/// ## Example
///
/// ```
/// use equibipartite::get_equi_partition;
///
/// let collection = vec![1, 4, 7, 35, 2, 1, 18, 6];
/// let partition = get_equi_partition(&collection);
/// println!("{:#?}", partition);   
///
/// let collection = vec![1, 2, 3, 4, 5, 6];
/// assert!(get_equi_partition(&collection).is_none());
/// ```
pub fn get_equi_partition(collection: &[i64]) -> Option<Partition> {
    if collection.is_empty() {
        return None;
    }
    let mut collection = collection.to_owned();
    collection.sort_unstable();
    collection.reverse();
    let sum: i64 = collection.iter().sum();
    if sum % 2 == 1 {
        return None;
    }
    let half_sum = sum / 2;
    match get_subset_with_sum(SubsetSearchOpts {
        collection: &collection,
        required_sum: half_sum,
        complement_not_exceeding: half_sum,
    }) {
        Some(subset) => Some(Partition::from_subset(&collection, &subset)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_none_when_no_equi_partition_exists() {
        let test_cases = vec![
            vec![],
            vec![2],
            vec![1, 2],
            vec![1, 1, 2, 6],
            vec![1, 2, 3, 4, 5],
        ];
        for test_case in test_cases {
            let partition = get_equi_partition(&test_case);
            assert!(
                partition.is_none(),
                "Found {:#?} - expected None",
                partition
            );
        }
    }

    #[test]
    fn it_returns_equi_partition_when_exists() {
        assert_eq!(
            get_equi_partition(&vec![1, 1]),
            Some(Partition {
                left: vec![1],
                right: vec![1]
            })
        );

        let test_cases = vec![vec![1, 2, 3], vec![1, 2, 2, 3], vec![10, 1, 2, 3, 4]];

        for mut test_case in test_cases {
            let partition = get_equi_partition(&test_case).unwrap();
            let sum_left: i64 = partition.left.iter().sum();
            let sum_right: i64 = partition.right.iter().sum();
            assert_eq!(
                sum_left, sum_right,
                "sum({:#?}) != sum({:#?})",
                partition.left, partition.right
            );
            let mut union = [&partition.left[..], &partition.right[..]].concat();
            union.sort_unstable();
            test_case.sort_unstable();
            assert_eq!(
                union, test_case,
                "{:#?} is an invalid partition of {:#?}",
                partition, test_case
            );
        }
    }
}
