use std::error::Error;

use futures::{future::BoxFuture, stream::FuturesUnordered, Future, StreamExt};

pub struct GenerationTask<'a, R> {
    task_name: String,
    task: BoxFuture<'a, R>,
}

impl<'a, R> GenerationTask<'a, R> {
    pub fn new<F, C, Fut>(ctx: &'a C, name: &str, task: F) -> Self
    where
        F: FnOnce(&'a C) -> Fut,
        Fut: Future<Output = R> + Send + 'a,
    {
        Self {
            task_name: name.to_string(),
            task: Box::pin(task(ctx)),
        }
    }
}

pub async fn run_tasks<'a, R, E>(tasks: Vec<GenerationTask<'a, Result<R, E>>>)
where
    E: Error,
{
    let mut futures: FuturesUnordered<BoxFuture<_>> = FuturesUnordered::new();
    for task in tasks.into_iter() {
        futures.push(Box::pin(async move {
            let result = task.task.await;
            (task.task_name, result)
        }))
    }

    while let Some((task_name, result)) = futures.next().await {
        match result {
            Err(e) => {
                println!("{}: Failed", &task_name);
                e.to_string().lines().for_each(|l| println!("  {}", l));
            }

            _ => {}
        }
    }
}
