use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Embedder {
    pub dim: usize,
    pub table: HashMap<u32, Vec<f32>>,
}

impl Embedder {
    pub fn new(dim: usize) -> Self {
        Embedder {
            dim,
            table: HashMap::new(),
        }
    }

    pub fn embed(&self, token_id: u32) -> Option<&[f32]> {
        self.table.get(&token_id).map(|v| v.as_slice())
    }

    pub fn embed_or_init(&mut self, token_id: u32) {
        let dim = self.dim;
        self.table.entry(token_id).or_insert_with(|| {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            (0..dim).map(|_| rng.r#gen::<f32>() * 2.0 - 1.0).collect()
        });
    }

    pub fn average_embed(&self, token_ids: &[u32]) -> Vec<f32> {
        if token_ids.is_empty() {
            return vec![0.0; self.dim];
        }
        let mut sum = vec![0.0; self.dim];
        let mut count = 0usize;
        for &id in token_ids {
            if let Some(emb) = self.table.get(&id) {
                let s = &mut sum;
                for i in 0..self.dim {
                    s[i] += emb[i];
                }
                count += 1;
            }
        }
        if count > 0 {
            let inv = 1.0 / count as f32;
            for s in &mut sum {
                *s *= inv;
            }
        }
        sum
    }

    pub fn update(&mut self, token_ids: &[u32], gradients: &[f32], lr: f32) {
        if token_ids.is_empty() {
            return;
        }
        let scale = lr / token_ids.len() as f32;
        for id in token_ids {
            if let Some(emb) = self.table.get_mut(id) {
                for i in 0..self.dim {
                    emb[i] -= scale * gradients[i];
                }
            }
        }
    }

    pub fn embedding_count(&self) -> usize {
        self.table.len()
    }
}
