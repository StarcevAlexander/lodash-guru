use std::collections::HashMap;
use std::hash::Hash;

pub fn chunk<T: Clone>(values: &[T], size: usize) -> Vec<Vec<T>> {
    let size = size.max(1);
    values.chunks(size).map(|c| c.to_vec()).collect()
}

pub fn compact<T: Clone>(values: &[Option<T>]) -> Vec<T> {
    values.iter().filter_map(|v| v.clone()).collect()
}

pub fn difference<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().filter(|item| !b.contains(item)).cloned().collect()
}

pub fn drop_<T: Clone>(values: &[T], n: usize) -> Vec<T> {
    values.iter().skip(n).cloned().collect()
}

pub fn drop_right<T: Clone>(values: &[T], n: usize) -> Vec<T> {
    let n = n.min(values.len());
    values[..values.len() - n].to_vec()
}

pub fn first<T: Clone>(values: &[T]) -> Option<T> {
    values.first().cloned()
}

pub fn last<T: Clone>(values: &[T]) -> Option<T> {
    values.last().cloned()
}

pub fn nth<T: Clone>(values: &[T], n: usize) -> Option<T> {
    values.get(n).cloned()
}

pub fn take<T: Clone>(values: &[T], n: usize) -> Vec<T> {
    values.iter().take(n).cloned().collect()
}

pub fn flatten<T: Clone>(values: &[Vec<T>]) -> Vec<T> {
    values.iter().flatten().cloned().collect()
}

pub fn uniq<T: PartialEq + Clone>(values: &[T]) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();
    for v in values {
        if !out.contains(v) {
            out.push(v.clone());
        }
    }
    out
}

pub fn union<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    uniq(&[a.to_vec(), b.to_vec()].concat())
}

pub fn intersection<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = Vec::new();
    for v in a {
        if b.contains(v) && !out.contains(v) {
            out.push(v.clone());
        }
    }
    out
}

pub fn xor<T: PartialEq + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut out = difference(a, b);
    out.extend(difference(b, a));
    out
}

pub fn without<T: PartialEq + Clone>(a: &[T], values: &[T]) -> Vec<T> {
    a.iter()
        .filter(|item| !values.contains(item))
        .cloned()
        .collect()
}

pub fn pull<T: PartialEq>(a: &mut Vec<T>, values: &[T]) {
    a.retain(|item| !values.contains(item));
}

pub fn group_by<T, K, F>(values: &[T], key_fn: F) -> HashMap<K, Vec<&T>>
where
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut out: HashMap<K, Vec<&T>> = HashMap::new();
    for v in values {
        out.entry(key_fn(v)).or_default().push(v);
    }
    out
}

pub fn partition<T, F>(values: &[T], predicate: F) -> (Vec<&T>, Vec<&T>)
where
    F: Fn(&T) -> bool,
{
    let mut matched = Vec::new();
    let mut rest = Vec::new();
    for v in values {
        if predicate(v) {
            matched.push(v);
        } else {
            rest.push(v);
        }
    }
    (matched, rest)
}

pub fn reject<T, F>(values: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    values.iter().filter(|v| !predicate(v)).collect()
}

pub fn includes<T: PartialEq>(values: &[T], value: &T) -> bool {
    values.contains(value)
}

fn splitmix64(mut state: u64) -> u64 {
    state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn seeded_rng() -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x853C49E6748FEA9B);
    splitmix64(nanos ^ splitmix64(std::process::id() as u64))
}

pub fn shuffle<T: Clone>(values: &[T]) -> Vec<T> {
    let mut out = values.to_vec();
    let mut state = seeded_rng();
    for i in (1..out.len()).rev() {
        let j = (splitmix64(state) % (i as u64 + 1)) as usize;
        state = splitmix64(state);
        out.swap(i, j);
    }
    out
}

pub fn sample<T: Clone>(values: &[T]) -> Option<T> {
    let mut state = seeded_rng();
    if values.is_empty() {
        None
    } else {
        let i = (splitmix64(state) % values.len() as u64) as usize;
        values.get(i).cloned()
    }
}

pub fn sample_size<T: Clone>(values: &[T], n: usize) -> Vec<T> {
    shuffle(values)
        .into_iter()
        .take(n.min(values.len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_splits_evenly_and_handles_zero() {
        assert_eq!(
            chunk(&[1, 2, 3, 4, 5], 2),
            vec![vec![1, 2], vec![3, 4], vec![5]]
        );
        assert_eq!(chunk(&[1, 2], 0), vec![vec![1, 2]]);
    }

    #[test]
    fn set_operations() {
        assert_eq!(difference(&[1, 2, 3], &[2]), vec![1, 3]);
        assert_eq!(union(&[1, 2], &[2, 3]), vec![1, 2, 3]);
        assert_eq!(intersection(&[1, 2, 3], &[2, 3, 4]), vec![2, 3]);
        assert_eq!(xor(&[1, 2], &[2, 3]), vec![1, 3]);
        assert_eq!(without(&[1, 2, 3, 4], &[2]), vec![1, 3, 4]);
    }

    #[test]
    fn slice_helpers() {
        assert_eq!(drop_(&[1, 2, 3], 1), vec![2, 3]);
        assert_eq!(drop_right(&[1, 2, 3], 1), vec![1, 2]);
        assert_eq!(take(&[1, 2, 3], 2), vec![1, 2]);
        assert_eq!(first(&[1, 2]), Some(1));
        assert_eq!(last(&[1, 2]), Some(2));
        assert_eq!(nth(&[1, 2, 3], 1), Some(2));
        assert_eq!(flatten(&[vec![1], vec![2, 3]]), vec![1, 2, 3]);
        assert_eq!(compact(&[Some(1), None, Some(2)]), vec![1, 2]);
    }

    #[test]
    fn group_and_partition() {
        let grouped = group_by(&[1, 2, 3, 4], |n: &i32| *n % 2);
        assert_eq!(grouped[&0].len(), 2);
        assert_eq!(grouped[&1].len(), 2);
        let (even, odd) = partition(&[1, 2, 3, 4], |n| n % 2 == 0);
        assert_eq!(even.len(), 2);
        assert_eq!(odd.len(), 2);
        assert_eq!(reject(&[1, 2, 3], |n| *n == 2), vec![&1, &3]);
    }

    #[test]
    fn uniq_and_shuffle() {
        assert_eq!(uniq(&[1, 1, 2, 2, 3]), vec![1, 2, 3]);
        let mut shuffled = shuffle(&[1, 2, 3, 4, 5]);
        shuffled.sort();
        assert_eq!(shuffled, vec![1, 2, 3, 4, 5]);
        assert_eq!(sample(&[7]), Some(7));
        assert_eq!(sample_size(&[1, 2, 3], 10).len(), 3);
        assert_eq!(sample::<i32>(&[]), None);
    }

    #[test]
    fn pull_removes_in_place() {
        let mut a = vec![1, 2, 3, 1, 2];
        pull(&mut a, &[1, 2]);
        assert_eq!(a, vec![3]);
    }

    #[test]
    fn includes_works() {
        assert!(includes(&[1, 2], &2));
        assert!(!includes(&[1, 2], &3));
    }
}
