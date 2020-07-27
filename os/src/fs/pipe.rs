
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use super::*;

struct PipeInner {
    buf: VecDeque<u8>,
    rcv: Condvar,
    wcv: Condvar,
}

pub struct PipeReader {
    inner: Arc<Mutex<PipeInner>>
}

pub struct PipeWriter {
    inner: Arc<Mutex<PipeInner>>
}

const BUFFER_SIZE: usize = 32;

/// 返回管道的读端和写端
pub fn new_pipe() -> (Arc<PipeWriter>, Arc<PipeReader>) {
    let buf = Arc::new(Mutex::new(
        PipeInner {
            buf: VecDeque::with_capacity(BUFFER_SIZE),
            rcv: Default::default(),
            wcv: Default::default(),
        }
    ));

    (Arc::new(PipeWriter{ inner: buf.clone() }),
        Arc::new(PipeReader{ inner: buf }))
}

impl INode for PipeReader {
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if offset != 0 {
            // 不支持 offset
            return Err(FsError::NotSupported)
        }
        let mut inner = self.inner.lock();
        if inner.buf.len() == 0 {
            // 缓冲区满, 等待
            // println!("sys read buf empty sleep, buflen: {}, rcv: {:p}", inner.buf.len(), &inner.rcv);
            inner.wcv.wait();
            // println!("wcv: {:p}", &inner.wcv);
            return Ok(0)
        }
        // println!("sys read buf not empty, buflen: {}", inner.buf.len());
        for (i, byte) in buf.iter_mut().enumerate() {
            if let Some(b) = inner.buf.pop_front() {
                *byte = b;
            } else {
                inner.rcv.notify_one();
                return Ok(i);
            }   
        }
        inner.rcv.notify_one();
        Ok(buf.len())
    }
    fn write_at(&self, _offset: usize, _buf: &[u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

impl INode for PipeWriter {
    fn read_at(&self, _offset: usize, _buf: &mut [u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if offset != 0 {
            // 不支持 offset
            return Err(FsError::NotSupported)
        }
        let mut inner = self.inner.lock();
        if inner.buf.len() >= BUFFER_SIZE {
            // 缓冲区满, 等待
            // println!("sys write buf full sleep, buflen: {}, wcv: {:p}", inner.buf.len(), &inner.wcv);
            inner.rcv.wait();
            // println!("rcv: {:p}", &inner.rcv);
            return Ok(0)
        }
        for (i, byte) in buf.iter().enumerate() {
            inner.buf.push_back(*byte);
            if inner.buf.len() >= BUFFER_SIZE {
                // println!("sys write {} bytes to buf full, buflen: {}", i + 1, inner.buf.len());
                inner.wcv.notify_one();
                return Ok(i + 1)
            }
        }
        // println!("sys write {} bytes, buflen: {}", buf.len(), inner.buf.len());
        inner.wcv.notify_one();
        Ok(buf.len())
    }

    fn poll(&self) -> Result<PollStatus> {
        Err(FsError::NotSupported)
    }

    /// This is used to implement dynamics cast.
    /// Simply return self in the implement of the function.
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}
