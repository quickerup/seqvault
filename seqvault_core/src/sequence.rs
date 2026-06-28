use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Step {
    pub destination: String,
    pub amount: u64,
    pub payload: Option<Vec<u8>>,
    pub mode: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Sequence {
    pub steps: Vec<Step>,
    pub nonce: u64,
    pub created_at: u64,
    pub expires_at: Option<u64>,
}

impl Sequence {
    pub fn new(steps: Vec<Step>, nonce: u64, created_at: u64, expires_at: Option<u64>) -> Self {
        Self {
            steps,
            nonce,
            created_at,
            expires_at,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.steps.is_empty() {
            return Err("Sequence must have at least one step".into());
        }
        if self.steps.len() > 255 {
            return Err("Sequence cannot have more than 255 steps".into());
        }
        for (i, step) in self.steps.iter().enumerate() {
            if step.destination.is_empty() {
                return Err(format!("Step {} destination cannot be empty", i));
            }
            if step.amount == 0 {
                return Err(format!("Step {} amount must be greater than zero", i));
            }
        }
        if let Some(expires_at) = self.expires_at {
            if expires_at <= self.created_at {
                return Err("Expiration time must be after creation time".into());
            }
        }
        Ok(())
    }

    pub fn calculate_hash(&self) -> Result<[u8; 32], String> {
        self.validate()?;
        let mut hasher = Sha256::new();

        hasher.update(self.nonce.to_le_bytes());
        hasher.update(self.created_at.to_le_bytes());

        match self.expires_at {
            Some(exp) => {
                hasher.update(&[1u8]);
                hasher.update(exp.to_le_bytes());
            }
            None => {
                hasher.update(&[0u8]);
            }
        }

        hasher.update((self.steps.len() as u32).to_le_bytes());

        for step in &self.steps {
            // Include length prefix for destination string to prevent collisions
            hasher.update((step.destination.len() as u32).to_le_bytes());
            hasher.update(step.destination.as_bytes());

            hasher.update(step.amount.to_le_bytes());

            match &step.payload {
                Some(p) => {
                    hasher.update(&[1u8]);
                    hasher.update((p.len() as u32).to_le_bytes());
                    hasher.update(p);
                }
                None => {
                    hasher.update(&[0u8]);
                }
            }

            hasher.update(&[step.mode]);
        }

        Ok(hasher.finalize().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequence() {
        let step = Step {
            destination: "addr1".into(),
            amount: 100,
            payload: None,
            mode: 0,
        };
        let seq = Sequence::new(vec![step], 1, 1000, Some(2000));
        assert!(seq.validate().is_ok());
    }

    #[test]
    fn test_empty_steps() {
        let seq = Sequence::new(vec![], 1, 1000, None);
        assert_eq!(seq.validate().unwrap_err(), "Sequence must have at least one step");
    }

    #[test]
    fn test_invalid_expiration() {
        let step = Step {
            destination: "addr1".into(),
            amount: 100,
            payload: None,
            mode: 0,
        };
        let seq = Sequence::new(vec![step], 1, 1000, Some(500));
        assert_eq!(seq.validate().unwrap_err(), "Expiration time must be after creation time");
    }

    #[test]
    fn test_empty_destination() {
        let step = Step {
            destination: "".into(),
            amount: 100,
            payload: None,
            mode: 0,
        };
        let seq = Sequence::new(vec![step], 1, 1000, None);
        assert_eq!(seq.validate().unwrap_err(), "Step 0 destination cannot be empty");
    }

    #[test]
    fn test_zero_amount() {
        let step = Step {
            destination: "addr1".into(),
            amount: 0,
            payload: None,
            mode: 0,
        };
        let seq = Sequence::new(vec![step], 1, 1000, None);
        assert_eq!(seq.validate().unwrap_err(), "Step 0 amount must be greater than zero");
    }

    #[test]
    fn test_deterministic_hash() {
        let step = Step {
            destination: "addr1".into(),
            amount: 100,
            payload: Some(vec![1, 2, 3]),
            mode: 3,
        };
        let seq1 = Sequence::new(vec![step.clone()], 1, 1000, Some(2000));
        let seq2 = Sequence::new(vec![step], 1, 1000, Some(2000));

        assert_eq!(seq1.calculate_hash().unwrap(), seq2.calculate_hash().unwrap());
    }

    #[test]
    fn test_different_hash() {
        let step1 = Step {
            destination: "addr1".into(),
            amount: 100,
            payload: None,
            mode: 0,
        };
        let step2 = Step {
            destination: "addr1".into(),
            amount: 101, // different amount
            payload: None,
            mode: 0,
        };
        let seq1 = Sequence::new(vec![step1], 1, 1000, None);
        let seq2 = Sequence::new(vec![step2], 1, 1000, None);

        assert_ne!(seq1.calculate_hash().unwrap(), seq2.calculate_hash().unwrap());
    }

    #[test]
    fn test_collision_resistance() {
        // Test that shifting characters between fields changes the hash
        let step1 = Step {
            destination: "addr".into(),
            amount: 100,
            payload: None,
            mode: 0,
        };
        let step2 = Step {
            destination: "add".into(), // one char less
            amount: 100,
            payload: None,
            mode: 0,
        };

        let seq1 = Sequence::new(vec![step1], 1, 1000, None);
        let seq2 = Sequence::new(vec![step2], 1, 1000, None);

        assert_ne!(seq1.calculate_hash().unwrap(), seq2.calculate_hash().unwrap());
    }

    #[test]
    fn test_hash_fails_on_invalid() {
        let seq = Sequence::new(vec![], 1, 1000, None);
        assert!(seq.calculate_hash().is_err());
    }
}
