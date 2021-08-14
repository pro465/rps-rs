use crate::object::Object;
use crate::AI;

pub struct Pool<const N: usize, const F: usize, 
                const MEM: usize, const S: usize> {
    pool: [AI<S>; N],

    scores: [[i8; F]; N],

    prev: [Object; MEM]
}

impl<const N: usize, const F: usize, 
     const MEM: usize, const S: usize> Pool<N,F,MEM,S> {
    pub fn new() -> Self {
        let mut pool = [AI::new(); N];
        for i in pool.iter_mut().skip(1) {
            *i = AI::new();
        }
        
        Pool {
            pool,
            scores: [[0; F]; N],
            prev: [Object::Rock; MEM]
        }
    }

    pub fn update(&mut self, curr: Object) {
        for i in 0..N {
            for j in 0..F - 1 {
                self.scores[i][j] = self.scores[i][j+1];
            }
            self.scores[i][F - 1] = if self.pool[i].predict::<MEM>(self.prev) == curr {
                1
            } else {
                -1
            };
        }

        for i in 0..MEM - 1 {
            self.prev[i] = self.prev[i + 1];
        }

        self.prev[MEM - 1] = curr;
        
        *self.worst() = AI::new();
    }

    pub fn choose(&self) -> Object {
        self.best().choose::<MEM>(self.prev)
    }

    fn best(&self) -> &AI<S> {
        let mut total_scores = [0; N];

        for i in 0..N {
            total_scores[i] = self.scores[i].iter().sum();
        }

        let (max_idx, _max) = total_scores.iter()
            .enumerate()
            .reduce(|a,b| if a.1 > b.1 { a } else { b })
            .unwrap();

        &self.pool[ max_idx ]
    }
    
    fn worst(&mut self) -> &mut AI<S> {
        let mut total_scores = [0; N];

        for i in 0..N {
            total_scores[i] = self.scores[i].iter().sum();
        }

        let (min_idx, _min) = total_scores.iter()
            .enumerate()
            .reduce(|a,b| if a.1 < b.1 { a } else { b })
            .unwrap();

        &mut self.pool[ min_idx ]
    }
}
