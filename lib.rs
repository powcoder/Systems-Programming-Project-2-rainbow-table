https://powcoder.com
代写代考加微信 powcoder
Assignment Project Exam Help
Add WeChat powcoder
//! Rainbow table related functionality

use digest::Digest;
use rand::{rngs::ThreadRng, Rng};
use std::{num::NonZeroUsize, string::FromUtf8Error};
use thiserror::Error;
use tracing::trace;

// export the md5 type and sha256 type so that
// consumers of our library don't have to explicitly
// add any other dependencies to instantiate
// hash generators
pub use md5::Md5;
pub use sha2::Sha256;

const ASCII_START: u8 = 32;
const ASCII_END: u8 = 126;

/// A password generator.
///
/// By default it only generates alphanumeric ascii characters (32 through 126)
#[derive(Debug)]
pub struct PasswordGenerator {
    /// The minimum number of characters in the generated password
    min_chars: NonZeroUsize,

    /// The maximum number of characters in the generated password
    max_chars: NonZeroUsize,

    /// Our rng
    rng: ThreadRng,
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self {
            min_chars: NonZeroUsize::new(4).unwrap(),
            max_chars: NonZeroUsize::new(4).unwrap(),
            rng: Default::default(),
        }
    }
}

impl PasswordGenerator {
    pub fn new(
        min_chars: NonZeroUsize,
        max_chars: NonZeroUsize,
    ) -> Result<Self, PasswordGeneratorError> {
        // assert!(min_chars <= max_chars);
        if min_chars > max_chars {
            return Err(PasswordGeneratorError::MinCharsToBig {
                min_chars,
                max_chars,
            });
        }

        Ok(Self {
            min_chars,
            max_chars,
            ..Default::default()
        })
    }

    pub fn gen(&mut self) -> Result<String, PasswordGeneratorError> {
        let num_chars: usize = self
            .rng
            .gen_range(self.min_chars.into()..=self.max_chars.into());

        let password = (0..num_chars)
            .map(|char_num| {
                trace!("generating password character #{char_num}");
                self.rng.gen_range(ASCII_START..=ASCII_END)
            })
            .collect::<Vec<_>>();

        Ok(String::from_utf8(password)?)
    }
}

#[derive(Default)]
pub struct PasswordHasher<D: Digest> {
    digest: D,
}

fn _new_digest<D: Digest>() -> impl Digest {
    D::new()
}

impl<D: Digest> PasswordHasher<D> {
    pub fn new() -> Self {
        Self { digest: D::new() }
    }

    pub fn hash_input(self, input: &str) -> Vec<u8> {
        self.digest
            .chain_update(input.as_bytes())
            .finalize()
            .as_slice()
            .into()

        // Digest::update(&mut self.digest, input);
        // // self.digest.update::<Digest>(input);

        // // let res = self.digest
        // //     .chain_update(input.as_bytes())
        // //     .finalize_reset();

        // self.digest.finalize_reset().to_vec()
    }
}

#[derive(Error, Debug)]
pub enum PasswordGeneratorError {
    #[error("Unable to convert bytes to utf8 string")]
    Utf8Parse {
        #[from]
        source: FromUtf8Error,
    },
    #[error("Minimum characters {min_chars} has to be <= maximum characters {max_chars} for generated password")]
    MinCharsToBig {
        min_chars: NonZeroUsize,
        max_chars: NonZeroUsize,
    },
}

pub enum MyEnum<D: Digest> {
    Password(PasswordGenerator),
    Hash(PasswordHasher<D>),
}

impl<D: Digest> MyEnum<D> {
    pub fn hash2(&self, _input: &str) {
        self.whatever();

        match self {
            MyEnum::Password(x) => self.some_function(x),
            MyEnum::Hash(x) => self.some_function2(x),
        }
    }

    fn some_function(&self, _x: &PasswordGenerator) {}

    fn some_function2(&self, _x: &PasswordHasher<D>) {}

    pub fn whatever(&self) {
        println!("whatever");
    }
}
