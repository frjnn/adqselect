//! adqselect
//!
//! `adqselect` is a small and extremely lightweight crate that provides
//! an in-place implementation of the Median of Ninthers algorithm
//! by Andrei Alexandrescu.
use std::{cmp::Ordering, ptr};

#[inline(always)]
fn median_index<T, F>(v: &[T], a: usize, b: usize, c: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    unsafe {
        if cmp(v.get_unchecked(a), v.get_unchecked(c)) == Ordering::Greater {
            if cmp(v.get_unchecked(b), v.get_unchecked(a)) == Ordering::Greater {
                a
            } else if cmp(v.get_unchecked(b), v.get_unchecked(c)) == Ordering::Less {
                c
            } else {
                b
            }
        } else if cmp(v.get_unchecked(b), v.get_unchecked(c)) == Ordering::Greater {
            c
        } else if cmp(v.get_unchecked(b), v.get_unchecked(a)) == Ordering::Less {
            a
        } else {
            b
        }
    }
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
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
            if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Ordering::Less {
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
            if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Ordering::Greater {
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

#[inline(always)]
fn expand_partition<T, F>(v: &mut [T], lo: usize, pivot: usize, hi: usize, cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = v.len() - 1;

    unsafe {
        loop {
            while left < lo {
                match cmp(v.get_unchecked(left), v.get_unchecked(pivot)) {
                    Ordering::Less | Ordering::Equal => left += 1,
                    _ => break,
                }
            }

            if left == lo {
                return expand_partition_right(v, pivot, hi, right + 1, cmp);
            }

            while right >= hi {
                match cmp(v.get_unchecked(right), v.get_unchecked(pivot)) {
                    Ordering::Greater | Ordering::Equal => right -= 1,
                    _ => break,
                }
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

        let (first, others) = v.split_at_mut_unchecked(1);
        let pivot = first.get_unchecked_mut(0);

        let mut lo = 0;
        let mut hi = others.len();

        while lo < hi {
            while lo < hi && cmp(others.get_unchecked(lo), pivot) == Ordering::Less {
                lo += 1
            }
            hi -= 1;
            while lo < hi && cmp(others.get_unchecked(hi), pivot) == Ordering::Greater {
                hi -= 1
            }
            if lo < hi {
                ptr::swap(others.get_unchecked_mut(lo), others.get_unchecked_mut(hi));
                lo += 1;
            }
        }

        ptr::swap(v.get_unchecked_mut(0), v.get_unchecked_mut(lo));

        lo
    }
}

fn partition_ninthers<T, F>(v: &mut [T], cmp: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let n = v.len();

    let frac = if n <= 1024 {
        n / 12
    } else if n <= 128 * 1024 {
        n >> 6
    } else {
        n >> 10
    };

    let pivot = frac / 2;
    let lo = n / 2 - pivot;
    let hi = lo + frac;

    let gap = (n - 9 * frac) / 4;
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
                if cmp(v.get_unchecked(j), v.get_unchecked(index)) == Ordering::Less {
                    index = j
                }
            }
        }
        unsafe {
            if cmp(v.get_unchecked(index), v.get_unchecked(i)) == Ordering::Less {
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
                if cmp(v.get_unchecked(j), v.get_unchecked(index)) == Ordering::Greater {
                    index = j
                }
            }
        }
        unsafe {
            if cmp(v.get_unchecked(index), v.get_unchecked(i)) == Ordering::Greater {
                ptr::swap(v.get_unchecked_mut(index), v.get_unchecked_mut(i))
            }
        }
        chunk += span;
    }

    let pivot = v.len() - k;
    adaptive_quickselect(&mut v[start..end], pivot, cmp);
    expand_partition(v, start, k, end, cmp)
}

#[inline(always)]
fn adaptive_quickselect<T, F>(mut v: &mut [T], mut nth: usize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    loop {
        let n = v.len();
        let last = n - 1;

        if n <= 1 {
            return;
        }

        if nth == 0 {
            let mut pivot = 0;
            for i in 1..n {
                unsafe {
                    if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Ordering::Less {
                        pivot = i;
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
            for i in 1..n {
                unsafe {
                    if cmp(v.get_unchecked(i), v.get_unchecked(pivot)) == Ordering::Greater {
                        pivot = i;
                    }
                }
            }
            unsafe {
                ptr::swap(v.get_unchecked_mut(last), v.get_unchecked_mut(pivot));
            }
            return;
        }

        let z = nth * 6;
        let pivot = if n <= 16 {
            partition_hoare(v, nth, cmp)
        } else if z <= n {
            partition_minima(v, nth, cmp)
        } else if z >= n * 5 {
            partition_maxima(v, nth, cmp)
        } else {
            partition_ninthers(v, cmp)
        };

        if pivot == nth {
            return;
        }

        if pivot > nth {
            v = &mut v[..pivot];
        } else {
            let start = pivot + 1;
            v = &mut v[start..];
            nth -= start;
        }
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
    assert!(
        !v.is_empty() && nth_el < v.len(),
        "nth_element: nth_el is out of bounds"
    );
    adaptive_quickselect(v, nth_el, cmp);
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng};

    use super::nth_element;

    #[test]
    #[cfg(not(tarpaulin_include))]
    fn test_nth_element() {
        for size in [1000, 10_000, 100_000, 1_000_000] {
            let nth = size / 2;
            let mut v: Vec<usize> = (0..size).collect();
            v.shuffle(&mut thread_rng());

            nth_element(&mut v, nth, &mut |a, b| a.cmp(b));

            assert_eq!(v[nth], nth);
        }
    }
}
