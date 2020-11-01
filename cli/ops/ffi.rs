// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.
use super::dispatch_json::{Deserialize, JsonOp, Value};
use crate::op_error::OpError;
use crate::state::State;
use deno_core::CoreIsolate;
use deno_core::ZeroCopyBuf;
use std::collections::HashMap;
use std::env;
use std::io::{Error, ErrorKind};
use url::Url;
use dlopen::raw::Library;

pub fn init(i: &mut CoreIsolate, s: &State) {
  i.register_op("op_dlopen", s.stateful_json_op2(op_dlopen));
//   i.register_op("op_dlsym", s.stateful_json_op(op_dlsym));
}


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DlOpenArgs {
  path: String,
}

fn op_dlopen(
    isolate: &mut CoreIsolate,
    state: &State,
    args: Value,
    _zero_copy: Option<ZeroCopyBuf>,
  ) -> Result<JsonOp, OpError> {
    let args: DlOpenArgs = serde_json::from_value(args)?;
    let path = resolve_from_cwd(Path::new(&args.path))?;
    
    let mut resource_table = isolate.resource_table.clone().borrow_mut();

    let lib = Library::open(path).unwrap();
    let rid = resource_table.add(
      "ffiLibrary",
      Box::new(lib),
    //   StreamResourceHolder::new(StreamResource::FsFile(Some((
    //     tokio_file,
    //     FileMetadata::default(),
    //   ))))
    );
    Ok(JsonOp::Sync(json!(rid)))
}

fn op_dlsym(
    state: &State,
    args: Value,
    _zero_copy: Option<ZeroCopyBuf>,
  ) {
    unimplemented!()
    //let fun: usize = unsafe{lib.symbol(symbol_name)}.unwrap();
}