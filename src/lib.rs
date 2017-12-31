extern crate num;

use num::traits::{Float, Zero};
use std::f32::MAX as f32_max;

/// `graph` generates a string representation of the values as a sparkline.
///
/// # Arguments
///
/// * `values` - The values to graph.
///
/// # Example
///
/// ```rust
/// let sparkline = spark::graph(&[1.0, 5.0, 22.0, 13.0, 53.0]);
/// assert_eq!(sparkline, "▁▁▃▂█");
/// ```
pub fn graph(values: &[f32]) -> String {
    let ticks = "▁▂▃▄▅▆▇█";

    /* XXX: This doesn't feel like idiomatic Rust */
    let mut min: f32 = f32_max;
    let mut max: f32 = 0.0;

    for &i in values.iter() {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }

    let ratio = if max == min {
        1.0
    } else {
        (ticks.chars().count() - 1) as f32 / (max - min)
    };

    values
        .iter()
        .cloned()
        .map(|n| (n - min) * ratio)
        .map(|n| n.floor() as usize)
        .filter_map(|n| ticks.chars().nth(n))
        .collect()
}

pub fn graph_opt<T>(values: &[Option<T>]) -> String
where
    T: Float + Zero,
{
    let ticks = "▁▂▃▄▅▆▇█";

    /* XXX: This doesn't feel like idiomatic Rust */
    let mut min = T::max_value();
    let mut max = T::zero();

    for &i in values.iter() {
        max = i.unwrap_or(T::zero()).max(max);
        min = i.unwrap_or(T::zero()).min(min);
    }

    let ratio = if max == min {
        T::one()
    } else {
        T::from(ticks.chars().count() - 1).unwrap() / (max - min)
    };

    values
        .iter()
        .cloned()
        .map(|n| match n {
            Some(b) => ticks
                .chars()
                .nth(((b - min) * ratio).floor().to_usize().unwrap())
                .unwrap(),
            None => ' ',
        })
        .collect()
}
