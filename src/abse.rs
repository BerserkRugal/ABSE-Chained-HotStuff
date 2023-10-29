use std::collections::VecDeque;
use std::error::Error;

pub struct ABSE {
    r: i32,  // record the number of rounds, initially 0
    s: Vec<f64>,  // the score of p_i
    ref_s: Vec<f64>,  // the reference score of p_i
    scores_i: VecDeque<Vec<f64>>,  // A queue of s, initially all 0
    info: Vec<f64>,  // A structure used to record voting information
    baseline: f64,  // the baseline score of leader election
    size: usize,  // The size for checking the queue length (confirm)
}

impl ABSE {
    pub fn new(size: usize) -> ABSE {
        ABSE {
            r: 0,
            s: Vec::new(),
            ref_s: Vec::new(),
            scores_i: VecDeque::new(),
            info: Vec::new(),  // TODO: Initialize with actual voting information
            baseline: 0.0,
            size,
        }
    }

    pub fn generate(&mut self) -> Result<Vec<f64>, Box<dyn Error>> {
        let rear_data = self.scores_i.back().unwrap();

        if self.info.len() > rear_data.len() {
            return Err("Length of info is greater than length of scores_i's rear data".into());
        } else if self.info.len() < rear_data.len() {
            self.info.resize(rear_data.len(), 0.0);
        }

        let new_s = self.info.iter().zip(rear_data.iter()).map(|(a, b)| a + b).collect::<Vec<f64>>();
        Ok(new_s)
    }

    pub fn update(&mut self, s: Vec<f64>, f: f64) {
        if self.scores_i.len() >= self.size {
            self.ref_s = self.scores_i.pop_front().unwrap();
        }
        self.scores_i.push_back(s);
        // A new baseline is obtained based on r. The computation rules can be specialised for different scenarios.
        self.baseline = (self.r as f64 * (2.0 * f + 1.0)) / (3.0 * f + 1.0);
    }

    pub fn judge(&self, j: usize) -> bool {
        if self.ref_s.is_empty() || self.ref_s[j] > self.baseline {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ABSE;

    #[test]
    fn test_abse_new() {
        let abse = ABSE::new(5);
        assert_eq!(abse.size, 5);
        assert_eq!(abse.scores_i.len(), 0);
    }

    #[test]
    fn test_abse_generate() {
        let mut abse = ABSE::new(2);
        abse.scores_i.push_back(vec![1.0, 2.0, 3.0]);
        abse.scores_i.push_back(vec![2.0, 3.0, 4.0]);
        abse.info = vec![1.0, 2.0];
        let result = abse.generate();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![3.0, 5.0, 4.0]);
    }

    #[test]
    fn test_abse_update() {
        let mut abse = ABSE::new(2);
        abse.scores_i.push_back(vec![1 as f64, 2.0, 3.0]);
        abse.scores_i.push_back(vec![4.0, 5.0, 6.0]);
        abse.r = 3;
        abse.update(vec![7.0, 8.0, 9.0], 2.0);
        assert_eq!(abse.scores_i.len(), 2);
        assert_eq!(abse.scores_i[0], vec![4.0, 5.0, 6.0]);
        assert_eq!(abse.scores_i[1], vec![7.0, 8.0, 9.0]);
        assert_eq!(abse.ref_s, vec![1.0, 2.0, 3.0]);
        //assert_eq!(abse.baseline, 2.25);
        abse.r = 4;
        abse.update(vec![1.0, 1.0, 1.0], 2.0);
        assert_eq!(abse.ref_s, vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_abse_judge() {
        let mut abse = ABSE::new(2);
        //abse.ref_s = vec![1.0, 2.0, 3.0];
        abse.baseline = 2.0;
        assert_eq!(abse.judge(0), false);
        assert_eq!(abse.judge(1), false);
        assert_eq!(abse.judge(2), true);
    }
}