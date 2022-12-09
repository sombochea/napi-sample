#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, Env, JsNumber, JsObject, Result, Task};

struct AsyncTask(u32);

impl Task for AsyncTask {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(self.0 as u64));
    Ok(self.0 * 2)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("sync", sync_fn)?;

  exports.create_named_method("sleep", sleep)?;

  exports.create_named_method("say", say)?;

  exports.create_named_method("fib", fib)?;
  Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;

  ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
  let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let task = AsyncTask(argument);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

#[js_function(1)]
fn say(ctx: CallContext) -> Result<JsNumber> {
  let a: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let b: u32 = ctx.get::<JsNumber>(1)?.try_into()?;
  ctx.env.create_uint32(a + b)
}

#[js_function(1)]
fn fib(ctx: CallContext) -> Result<JsNumber> {
  let n: i64 = ctx.get::<JsNumber>(0)?.try_into()?;
  ctx.env.create_int64(fibonacci_native(n))
}

#[inline(always)]
fn fibonacci_native(n: i64) -> i64 {
  match n {
    0 => 0,
    1 | 2 => 1,
    _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
  }
}
