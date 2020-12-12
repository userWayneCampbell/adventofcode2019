use advent_of_code2020::part_01;
use advent_of_code2020::part_02;
use advent_of_code2020::part_03;
use advent_of_code2020::part_04;
use advent_of_code2020::part_05;
use advent_of_code2020::part_06;
use advent_of_code2020::part_07;
use advent_of_code2020::part_08;
use advent_of_code2020::part_09;
use advent_of_code2020::part_10;
use advent_of_code2020::part_11::Part;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn bench_1(c: &mut Criterion) {
    c.bench_function("1_both", |b| {
        b.iter(|| {
            assert_eq!((29929, 244300320), part_01::one());
        })
    });
}
pub fn bench_2(c: &mut Criterion) {
    c.bench_function("2_both", |b| {
        b.iter(|| {
            assert_eq!((586, 352), part_02::two());
        })
    });
}
pub fn bench_3(c: &mut Criterion) {
    c.bench_function("3_both", |b| {
        b.iter(|| {
            assert_eq!((207, 2_655_892_800), part_03::three());
        })
    });
}
pub fn bench_4(c: &mut Criterion) {
    c.bench_function("4_both", |b| {
        b.iter(|| {
            assert_eq!((228, 175), part_04::four());
        })
    });
}
pub fn bench_5(c: &mut Criterion) {
    c.bench_function("5_both", |b| {
        b.iter(|| {
            assert_eq!((989, 548), part_05::five());
        })
    });
}

pub fn bench_6(c: &mut Criterion) {
    c.bench_function("6_both", |b| {
        b.iter(|| {
            assert_eq!((6504, 3351), part_06::six());
        })
    });
}

pub fn bench_7(c: &mut Criterion) {
    c.bench_function("7_both", |b| {
        b.iter(|| {
            assert_eq!((274, 158_730), part_07::seven());
        })
    });
}

pub fn bench_8(c: &mut Criterion) {
    c.bench_function("8_both", |b| {
        b.iter(|| {
            assert_eq!((1179, 1089), part_08::eight());
        })
    });
}

pub fn bench_9(c: &mut Criterion) {
    c.bench_function("9_both", |b| {
        b.iter(|| {
            assert_eq!((466456641, 55732936), part_09::nine());
        })
    });
}

pub fn bench_10(c: &mut Criterion) {
    c.bench_function("10_both", |b| {
        b.iter(|| {
            assert_eq!((2590, 226775649501184), part_10::ten());
        })
    });
}

pub fn bench_11(c: &mut Criterion) {
    c.bench_function("11_part1", |b| {
        b.iter(|| {
            let input = include_str!("../data/11.in");
            let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

            let _part1 = Part::One.steps(seats);
            assert_eq!(2211, _part1);
        })
    });
    c.bench_function("11_part2", |b| {
        b.iter(|| {
            let input = include_str!("../data/11.in");
            let seats: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

            let _part2 = Part::Two.steps(seats);
            assert_eq!(1995, _part2);
        })
    });
}

criterion_group!(benches, bench_1, bench_2, bench_3, bench_4, bench_5, bench_6, bench_7, bench_8, bench_9, bench_10, bench_11);
criterion_main!(benches);
