use log::error;
use tokio::process::Command;
use warp::{
    reject::{Reject, Rejection},
    Buf,
};

#[derive(Debug)]
struct CommandFailed;

impl Reject for CommandFailed {}

async fn run_filter(mut command: Command, mut input: impl Buf) -> Result<Vec<u8>, std::io::Error> {
    // Setup process stdio
    use std::process::Stdio;
    command.stdin(Stdio::piped());
    command.stdout(Stdio::piped());
    let mut instance = command.spawn()?;
    let mut command_stdin = instance.stdin.take().unwrap();
    let mut command_stdout = instance.stdout.take().unwrap();
    // Read command stdout to vec
    let reader = async move {
        use tokio::io::AsyncReadExt;
        let mut command_output = vec![];
        command_stdout.read_to_end(&mut command_output).await?;
        Ok::<_,std::io::Error>(command_output)
    };
    // Write input to command stdin 
    let writer = async move {
        use tokio::io::AsyncWriteExt;
        while input.has_remaining() {
            let slice = input.bytes();
            let slice_length = slice.len();
            command_stdin.write(slice).await?;
            input.advance(slice_length);
        }
        Ok::<_,std::io::Error>(())
    };
    let (proc_res, write_res, read_res) = tokio::join!(instance, writer, reader);
    proc_res?;
    write_res?;
    read_res
}

pub async fn autopep8_format(body: impl Buf) -> Result<Vec<u8>, Rejection> {
    let mut command = Command::new("autopep8");
    command.arg("-");
    run_filter(command, body).await.map_err(|e| {
        error!("Crash while running autopep8: {}", e);
        warp::reject::custom(CommandFailed)
    })
}

pub async fn clang_format(body: impl Buf) -> Result<Vec<u8>, Rejection> {
    let mut command = Command::new("clang-format");
    command.arg("-assume-filename=a.cpp");
    command.arg("-style={BasedOnStyle: LLVM, IndentWidth: 4, ColumnLimit: 120}");
    run_filter(command, body).await.map_err(|e| {
        error!("Crash while running clang-format: {}", e);
        warp::reject::custom(CommandFailed)
    })
}

pub async fn js_format(body: impl Buf) -> Result<Vec<u8>, Rejection> {
    let mut command = Command::new("clang-format");
    command.arg("-assume-filename=a.js");
    command.arg("-style={IndentWidth: 4, ColumnLimit: 120}");
    run_filter(command, body).await.map_err(|e| {
        error!("Crash while running clang-format: {}", e);
        warp::reject::custom(CommandFailed)
    })
}

pub async fn rust_format(body: impl Buf) -> Result<Vec<u8>, Rejection> {
    let mut command = Command::new("rustfmt");
    command.arg("--edition=2018");
    command.arg("--config=max_width=120");
    run_filter(command, body).await.map_err(|e| {
        error!("Crash while running rustfmt: {}", e);
        warp::reject::custom(CommandFailed)
    })
}
