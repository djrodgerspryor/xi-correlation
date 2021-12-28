#![feature(int_abs_diff)]

use itertools::Itertools;
use rand::Rng;
use textplots::Plot;

fn invert_permutation(permutation: &[usize]) -> Vec<usize> {
    let mut result = vec![0; permutation.len()];

    for (i, j) in permutation.iter().enumerate() {
        result[*j] = i;
    }

    return result;
}

// TODO: ties
fn xi<T: Ord + Copy>(data: &[(T, T)]) -> f64 {
    let length = data.len();

    let mut x_sort_permutation: Vec<usize> = (0..length).collect();
    x_sort_permutation.sort_unstable_by_key(|i| data[*i].0);

    let y_rank_by_index;
    {
        let mut y_sort_permutation: Vec<usize> = (0..length).collect();
        y_sort_permutation.sort_unstable_by_key(|i| data[*i].1);

        y_rank_by_index = invert_permutation(&y_sort_permutation[..]);
    }

    let sum: usize = x_sort_permutation
        .into_iter()
        .tuple_windows::<(_, _)>()
        .map(|(i, next_i)| y_rank_by_index[next_i].abs_diff(y_rank_by_index[i]))
        .sum();

    return 1.0 - ((sum as f64 * 3.0) / ((length * length) as f64 - 1.0));
}

// Symmetric xi correlation function. Simply the larger of the two possible values.
fn sym_xi<T: Ord + Copy>(data: &[(T, T)]) -> f64 {
    std::cmp::max(
        ordered_float::NotNan::new(xi(data)).unwrap(),
        ordered_float::NotNan::new(xi(&data.iter().map(|(x, y)| (y, x)).collect::<Vec<_>>()))
            .unwrap(),
    )
    .into_inner()
}

const N: i32 = 1000;
const X_MIN: f32 = -10.0;
const X_MAX: f32 = 10.0;

fn generate_dataset(
    gen_y: impl Fn(f32, &mut rand::rngs::ThreadRng) -> f32,
) -> Vec<(ordered_float::NotNan<f32>, ordered_float::NotNan<f32>)> {
    let mut rng = rand::thread_rng();
    return (0..N)
        .map(|_| {
            let x = rng.gen_range(X_MIN..X_MAX);
            (
                ordered_float::NotNan::new(x).unwrap(),
                ordered_float::NotNan::new(gen_y(x, &mut rng)).unwrap(),
            )
        })
        .collect();
}

fn plot(name: &str, dataset: &[(ordered_float::NotNan<f32>, ordered_float::NotNan<f32>)]) {
    println!(
        "{}: ξ = {:.2}, ξₛ = {:.2}",
        name,
        xi(&dataset),
        sym_xi(&dataset)
    );
    textplots::Chart::new(160, 120, X_MIN, X_MAX)
        .lineplot(&textplots::Shape::Points(
            &dataset
                .iter()
                .map(|(x, y)| (x.into_inner(), y.into_inner()))
                .collect::<Vec<_>>(),
        ))
        .nice();
}

fn main() {
    // Generate and plot a bunch of test cases
    plot(
        "Fully Random",
        &generate_dataset(|_x, rng| rng.gen_range(X_MIN..X_MAX)),
    );

    plot("Linear", &generate_dataset(|x, _rng| x * 1.5));

    plot(
        "Noisy Linear",
        &generate_dataset(|x, rng| x + rng.gen_range(0.0..3.0)),
    );

    plot(
        "Noisier Linear",
        &generate_dataset(|x, rng| x + rng.gen_range(0.0..10.0)),
    );

    plot("Quadratic", &generate_dataset(|x, _rng| x * x + x));

    plot(
        "Noisy Quadratic",
        &generate_dataset(|x, rng| x * x + rng.gen_range(0.0..100.0)),
    );

    plot("Exp", &generate_dataset(|x, _rng| 2.0_f32.powf(x)));

    plot(
        "Noisy Hyperbola",
        &generate_dataset(|x, rng| rng.gen_range(0.0..10.0) / (1.0 - 0.5 * x)),
    );

    plot(
        "Absolute Square Root",
        &generate_dataset(|x, rng| (if rng.gen() { 1.0 } else { -1.0 }) * x.abs().powf(0.5)),
    );

    plot(
        "Noisy Absolute Square Root",
        &generate_dataset(|x, rng| {
            rng.gen_range(0.0..1.5) + (if rng.gen() { 1.0 } else { -1.0 }) * x.abs().powf(0.5)
        }),
    );
}
