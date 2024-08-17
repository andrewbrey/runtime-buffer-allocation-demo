use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;

use thiserror::Error;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type Inner = Option<Vec<u8>>;

static ACTUAL_BUFFER: LazyLock<Mutex<Inner>> = LazyLock::new(|| Mutex::new(None));

struct Buffer;

#[derive(Error, Debug)]
#[error("Something went wrong...")]
struct Ouch;

impl Buffer {
    const BUFFER_SIZE: usize = 1024 * 4;

    pub fn use_buffer(func: impl FnOnce(&Vec<u8>) -> Result<()>) -> Result<()> {
        println!("Trying to allocate the buffer...");

        let buffer = Buffer::try_allocate()?;

        println!("Got ahold of the buffer lock and allocated...");

        if let Some(ref actual_buffer) = *buffer {
            println!("Successfully got access to the buffer, doing things now...");

            let result = func(actual_buffer);

            drop(buffer);

            println!("Done with the buffer, deallocating...");

            Buffer::try_deallocate()?;

            println!("Successfully deallocated.");

            result
        } else {
            println!("How did you get here?...");

            Err(Ouch.into())
        }
    }

    fn try_allocate() -> Result<MutexGuard<'static, Inner>> {
        let mut buffer = ACTUAL_BUFFER.lock()?;

        *buffer = Some(vec![0; Self::BUFFER_SIZE]);

        Ok(buffer)
    }

    fn try_deallocate() -> Result<()> {
        let mut buffer = ACTUAL_BUFFER.lock()?;

        *buffer = None;

        Ok(())
    }
}

fn main() -> Result<()> {
    Buffer::use_buffer(|buffer| {
        println!(
            "Doing something with the buffer whose length is {}",
            buffer.len()
        );

        Ok(())
    })
}
