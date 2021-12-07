use nn::dataset::*;
use nn::nn::NeuralNetwork;
use nn::defaults::DefaultRng as Rng;

use crate::game;
use crate::object::Object;

use std::convert::TryInto;

pub struct AI<'a, const N: usize> {
    prev: [Object; N],
    nn: NeuralNetwork,
    dataset: DataSet<'a, N, 3>,
    idx: usize,
}

impl<const N: usize> AI<N> {
    pub fn new() -> Self {
        AI {
            prev: [Object::Rock; N],
            nn: NeuralNetwork::new(&[N, 16, 16, 3], &mut Rng::new()),
            dataset: DataSet::new(),
            idx: 0,
        }
    }

    pub fn update(&mut self, curr: Object) {
        let mut e_out = [0.; 3];

        e_out[curr as usize] = 1.;

        let mut inp = [0.; N];

        self.prev
            .iter()
            .enumerate()
            .for_each(|(i, x)| inp[i] = (*x) as usize as f64);

        if self.dataset.len() == 50 {
            self.dataset.swap_remove(self.idx);
            self.idx = (self.idx + 1) % 50;
        }

        self.dataset.push((inp, e_out));

        for i in 0..N - 1 {
            self.prev[i] = self.prev[i + 1];
        }

        self.prev[N - 1] = curr;
    }

    pub fn train(&mut self) {
        let dataset = &self.dataset;

        self.nn.train(100, dataset);
    }

    pub fn choose(&self) -> Object {
        game::winning_move(self.predict())
    }

    fn predict(&self) -> Object {
        self.best_probab_idx().try_into().unwrap()
    }

    fn best_probab_idx(&self) -> usize {
        let arr = self
            .nn
            .predict(self.prev.iter().map(|x| (*x) as usize as f64).collect());

        let (max_idx, _max) = arr
            .iter()
            .enumerate()
            .reduce(|a, b| if a.1 > b.1 { a } else { b })
            .unwrap();

        max_idx
    }
}

impl<const N: usize> Default for AI<N> {
    fn default() -> Self {
        AI::new()
    }
}
