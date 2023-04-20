use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{self, AsyncRead, AsyncWrite, AsyncWriteExt};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

pub(crate) struct BuildkitStdio {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl BuildkitStdio {
    /// Create a new DockerStdioService from a container name.
    /// 
    /// ## Panics
    /// 
    /// This function will panic if [child] does not have a stdin or stdout.
    pub fn new(mut child: Child) -> BuildkitStdio {
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        Self {
            child,
            stdin,
            stdout,
        }
    }

    async fn kill(&mut self) -> io::Result<()> {
        self.stdin.shutdown().await?;
        self.child.kill().await?;
        Ok(())
    }
}

impl AsyncRead for BuildkitStdio {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdout).poll_read(cx, buf)
    }
}

impl AsyncWrite for BuildkitStdio {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.stdin).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.stdin).poll_shutdown(cx)
    }
}
