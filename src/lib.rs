//! adqselect
//!
//! `adqselect` is a small and extremely lightweight crate that provides
//! an in-place implementation of the Median of Ninthers algorithm
//! by Andrei Alexandrescu.
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::ptr;

fn median_index<T, F>(v: &[T], a: usize, b: usize, c: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    unsafe {
        if cmp(v.get_unchecked(a), v.get_unchecked(c)) == Greater {
            if cmp(v.get_unchecked(b), v.get_unchecked(a)) == Greater {
                a
            } else {
                if cmp(v.get_unchecked(b), v.get_unchecked(c)) == Less {
                    c
                } else {
                    b
                }
            }
        } else {
            if cmp(v.get_unchecked(b), v.get_unchecked(c)) == Greater {
                c
            } else {
                if cmp(v.get_unchecked(b), v.get_unchecked(a)) == Less {
                    a
                } else {
                    b
                }
            }
        }
    }
}

fn ninther<T, F>(
    v: &mut [T],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    f: usize,
    g: usize,
    h: usize,
    i: usize,
    cmp: &mut F,
) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    median_index(
        v,
        median_index(v, a, b, c, cmp),
        median_index(v, d, e, f, cmp),
        median_index(v, g, h, i, cmp),
        cmp,
    )
}

fn expand_partition_right<T, F>(
    v: &mut [T],
    pivot: usize,
    hi: usize,
    right: usize,
    cmp: &mut F,
) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut p = pivot;
    for i in hi..right {
        unsafe {
            if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Less {
                p += 1;
                ptr::swap(v.get_unchecked_mut(p), v.get_unchecked_mut(i));
            }
        }
    }
    unsafe {
        ptr::swap(v.get_unchecked_mut(p), v.get_unchecked_mut(pivot));
    }
    p
}

fn expand_partition_left<T, F>(
    v: &mut [T],
    pivot: usize,
    lo: usize,
    left: usize,
    cmp: &mut F,
) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut p = pivot;
    for i in (left..lo).rev() {
        unsafe {
            if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Greater {
                p -= 1;
                ptr::swap(v.get_unchecked_mut(p), v.get_unchecked_mut(i));
            }
        }
    }
    unsafe {
        ptr::swap(v.get_unchecked_mut(p), v.get_unchecked_mut(pivot));
    }
    p
}

fn expand_partition<T, F>(v: &mut [T], lo: usize, pivot: usize, hi: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = v.len() - 1;

    unsafe {
        loop {
            while left < lo
                && (cmp(v.get_unchecked(left), v.get_unchecked(pivot)) == Less
                    || cmp(v.get_unchecked(left), v.get_unchecked(pivot)) == Equal)
            {
                left += 1
            }
            if left == lo {
                return expand_partition_right(v, pivot, hi, right + 1, cmp);
            }
            while right >= hi
                && (cmp(v.get_unchecked(right), v.get_unchecked(pivot)) == Greater
                    || cmp(v.get_unchecked(right), v.get_unchecked(pivot)) == Equal)
            {
                right -= 1
            }
            if right < hi {
                return expand_partition_left(v, pivot, lo, left, cmp);
            }

            ptr::swap(v.get_unchecked_mut(left), v.get_unchecked_mut(right));
            left += 1;
            right -= 1;
        }
    }
}

fn partition_hoare<T, F>(v: &mut [T], k: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    unsafe {
        ptr::swap(v.get_unchecked_mut(0), v.get_unchecked_mut(k));
    }
    let mut lo = 1;
    let mut hi = v.len() - 1;

    while lo <= hi {
        unsafe {
            while lo <= hi && cmp(v.get_unchecked(lo), v.get_unchecked(0)) == Less {
                lo += 1
            }
            while lo <= hi && cmp(v.get_unchecked(hi), v.get_unchecked(0)) == Greater {
                hi -= 1
            }
            if lo <= hi {
                ptr::swap(v.get_unchecked_mut(lo), v.get_unchecked_mut(hi));
                lo += 1;
                hi -= 1;
            }
        }
    }

    lo -= 1;
    unsafe {
        ptr::swap(v.get_unchecked_mut(lo), v.get_unchecked_mut(0));
    }
    lo
}

fn partition_ninthers<T, F>(v: &mut [T], cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let frac = if v.len() <= 1024 {
        v.len() / 12
    } else {
        if v.len() <= 128 * 1024 {
            v.len() / 64
        } else {
            v.len() / 1024
        }
    };

    let pivot = frac / 2;
    let lo = v.len() / 2 - pivot;
    let hi = lo + frac;

    let gap = (v.len() - 9 * frac) / 4;
    let mut a = lo - 4 * frac - gap;
    let mut b = hi + gap;
    for i in lo..hi {
        let k = ninther(
            v,
            a,
            i - frac,
            b,
            a + 1,
            i,
            b + 1,
            a + 2,
            i + frac,
            b + 2,
            cmp,
        );
        unsafe {
            ptr::swap(v.get_unchecked_mut(k), v.get_unchecked_mut(i));
        }
        a += 3;
        b += 3;
    }

    adaptive_quickselect(&mut v[lo..hi], pivot, cmp);
    expand_partition(v, lo, lo + pivot, hi, cmp)
}

fn partition_minima<T, F>(v: &mut [T], k: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let subset = k * 2;
    let span = (v.len() - subset) / subset;
    let start = 0;
    let end = start + subset;

    let mut chunk = subset;
    for i in start..end {
        let mut index = chunk;
        for j in (chunk + 1)..(chunk + span) {
            unsafe {
                if cmp(v.get_unchecked(j), v.get_unchecked(index)) == Less {
                    index = j
                }
            }
        }
        unsafe {
            if cmp(v.get_unchecked(index), v.get_unchecked(i)) == Less {
                ptr::swap(v.get_unchecked_mut(index), v.get_unchecked_mut(i))
            }
        }
        chunk += span;
    }

    adaptive_quickselect(&mut v[start..end], k, cmp);
    expand_partition(v, start, k, end, cmp)
}

fn partition_maxima<T, F>(v: &mut [T], k: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let subset = (v.len() - k) * 2;
    let span = (v.len() - subset) / subset;
    let start = v.len() - subset;
    let end = v.len();

    let mut chunk = start - subset * span;
    for i in start..end {
        let mut index = chunk;
        for j in (chunk + 1)..(chunk + span) {
            unsafe {
                if cmp(v.get_unchecked(j), v.get_unchecked(index)) == Greater {
                    index = j
                }
            }
        }
        unsafe {
            if cmp(v.get_unchecked(index), v.get_unchecked(i)) == Greater {
                ptr::swap(v.get_unchecked_mut(index), v.get_unchecked_mut(i))
            }
        }
        chunk += span;
    }

    let pivot = v.len() - k;
    adaptive_quickselect(&mut v[start..end], pivot, cmp);
    expand_partition(v, start, k, end, cmp)
}

fn adaptive_quickselect<T, F>(v: &mut [T], nth: usize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let last = v.len() - 1;

    if nth == 0 {
        let mut pivot = 0;
        for i in 1..v.len() {
            unsafe {
                if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Less {
                    pivot = i
                }
            }
        }
        unsafe {
            ptr::swap(v.get_unchecked_mut(0), v.get_unchecked_mut(pivot));
        }
        return;
    }

    if nth == last {
        let mut pivot = 0;
        for i in 1..v.len() {
            unsafe {
                if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Greater {
                    pivot = i
                }
            }
        }
        unsafe {
            ptr::swap(v.get_unchecked_mut(last), v.get_unchecked_mut(pivot));
        }
        return;
    }

    let pivot = if v.len() <= 16 {
        partition_hoare(v, nth, cmp)
    } else if nth * 6 <= v.len() {
        partition_minima(v, nth, cmp)
    } else if nth * 6 >= v.len() * 5 {
        partition_maxima(v, nth, cmp)
    } else {
        partition_ninthers(v, cmp)
    };

    if pivot == nth {
        return;
    }
    if pivot > nth {
        adaptive_quickselect(&mut v[..pivot], nth, cmp);
    } else {
        let start = pivot + 1;
        adaptive_quickselect(&mut v[start..], nth - start, cmp);
    }
}

/// Moves the n-th element of the given Vector in the n-th position
/// by using the adaptive quickselect algorithm by Andrei Alexandrescu.
///
/// Similar to its c++ counterpart.
///
///
/// # Examples
///
/// ```
/// let mut v = vec![10, 7, 9, 7, 2, 8, 8, 1, 9, 4];
/// adqselect::nth_element(&mut v, 3, &mut Ord::cmp);
///
/// assert_eq!(v[3], 7);
/// ```
///
/// # Panics
///
/// if `nth_el` is out of bounds.
pub fn nth_element<T, F>(v: &mut [T], nth_el: usize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    adaptive_quickselect(v, nth_el, cmp);
}

#[cfg(test)]
mod tests {
    use super::nth_element;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test() {
        for size in [1000, 10_000, 100_000, 1_000_000].iter() {
            let nth = (size / 2) as usize;
            let mut v: Vec<u32> = (0..*size).collect();
            v.shuffle(&mut thread_rng());

            nth_element(&mut v, nth, &mut Ord::cmp);
            assert_eq!(v[nth], nth as u32)
        }
    }
}
