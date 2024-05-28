use crate::{fail, util};

use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Debug)]
pub enum Event {
    Tag { name: Vec<u8> },
    Attribute { key: Vec<u8>, value: Option<String> },
    Block { open: bool },
    Text { text: Vec<u8> },
    End,
}

enum State {
    Block,
    Tag,
    Attribute,
    End,
}

pub struct Reader<R> {
    buf_reader: BufReader<R>,
    buffer: Vec<u8>,
    state: State,
    next_event: Option<Event>,
}

impl<R> Reader<R>
where
    R: Read,
{
    pub fn new(read: R) -> Self {
        Reader {
            buf_reader: BufReader::new(read),
            buffer: Vec::new(),
            state: State::Block,
            next_event: None,
        }
    }

    pub fn read(&mut self) -> util::Result<Event> {
        if let Some(event) = std::mem::take(&mut self.next_event) {
            return Ok(event);
        }

        let mut event = None;

        while event.is_none() {
            match self.state {
                State::Block => {
                    match Self::read_until(&mut self.buf_reader, &mut self.buffer, &b"[}")? {
                        None => {
                            event = Some(Event::End);
                            self.state = State::End;
                        }
                        Some(b'[') => {
                            if !self.buffer.is_empty() {
                                event = Some(Event::Text {
                                    text: self.buffer.clone(),
                                });
                            }
                            self.state = State::Tag;
                        }
                        Some(b'}') => {
                            if !self.buffer.is_empty() {
                                event = Some(Event::Text {
                                    text: self.buffer.clone(),
                                });
                                // Register Block(close)-event, to be emitted next time
                                self.next_event = Some(Event::Block { open: false });
                            } else {
                                event = Some(Event::Block { open: false });
                            }
                            self.state = State::Block;
                        }
                        _ => assert!(false),
                    }
                }
                State::Tag => {
                    match Self::read_until(&mut self.buf_reader, &mut self.buffer, &b"]")? {
                        None => {
                            self.state = State::End;
                            fail!("Could not read tag {} to end", hex::encode(&self.buffer));
                        }
                        Some(b']') => {
                            event = Some(Event::Tag {
                                name: self.buffer.clone(),
                            });

                            match Self::read_until(&mut self.buf_reader, &mut self.buffer, &b"(}{")?
                            {
                                None => {
                                    self.next_event = Some(Event::End);
                                    self.state = State::End;
                                }
                                Some(b'(') => {
                                    self.state = State::Attribute;
                                }
                                Some(b'}') => {
                                    self.next_event = Some(Event::Block { open: false });
                                    self.state = State::Block;
                                }
                                Some(b'{') => {
                                    self.next_event = Some(Event::Block { open: true });
                                    self.state = State::Block;
                                }
                                _ => assert!(false),
                            }
                        }
                        _ => assert!(false),
                    }
                }
                State::Attribute => {
                    match Self::read_until(&mut self.buf_reader, &mut self.buffer, &b")")? {
                        None => {
                            self.state = State::End;
                            fail!(
                                "Could not read attribute {} to end",
                                hex::encode(&self.buffer)
                            );
                        }
                        Some(b')') => {
                            event = Some(Event::Attribute {
                                key: self.buffer.clone(),
                                value: None,
                            });

                            match Self::read_until(&mut self.buf_reader, &mut self.buffer, &b"(}{")?
                            {
                                None => {
                                    self.next_event = Some(Event::End);
                                    self.state = State::End;
                                }
                                Some(b'(') => {
                                    self.state = State::Attribute;
                                }
                                Some(b'}') => {
                                    self.next_event = Some(Event::Block { open: false });
                                    self.state = State::Block;
                                }
                                Some(b'{') => {
                                    self.next_event = Some(Event::Block { open: true });
                                    self.state = State::Block;
                                }
                                _ => assert!(false),
                            }
                        }
                        _ => assert!(false),
                    }
                }
                State::End => event = Some(Event::End),
            }
        }

        Ok(event.unwrap())
    }

    fn read_until<const N: usize>(
        buf_reader: &mut BufReader<R>,
        data: &mut Vec<u8>,
        delims: &[u8; N],
    ) -> util::Result<Option<u8>> {
        data.clear();

        let mut buf = [0 as u8; 1];
        while buf_reader.read(&mut buf)? == 1 {
            let byte = buf[0];
            if delims.contains(&byte) {
                return Ok(Some(byte));
            }
            data.push(byte);
        }

        Ok(None)
    }
}

pub struct Writer<W>
where
    W: Write,
{
    buf_writer: BufWriter<W>,
}

impl<W> Writer<W>
where
    W: Write,
{
    pub fn new(write: W) -> Self {
        Writer {
            buf_writer: BufWriter::new(write),
        }
    }

    pub fn flush(&mut self) -> util::Result<()> {
        self.buf_writer.flush()?;
        Ok(())
    }
    pub fn get_mut(&mut self) -> &mut W {
        self.buf_writer.get_mut()
    }

    pub fn node<'a>(&'a mut self, tag: &str) -> util::Result<Node<'a, W>> {
        self.buf_writer.write(tag.as_bytes())?;
        self.buf_writer.write("{".as_bytes())?;
        Ok(Node { writer: self })
    }
}

pub struct Node<'a, W>
where
    W: Write,
{
    writer: &'a mut Writer<W>,
}

impl<'a, W> Drop for Node<'a, W>
where
    W: Write,
{
    fn drop(&mut self) {
        let _ = self.writer.buf_writer.write("}".as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() -> util::Result<()> {
        let msg = "text[ta]g](a:0)(b:)(c){[](){}}";

        let mut r = Reader::new(msg.as_bytes());

        loop {
            let event = r.read()?;
            println!("event: {:?}", &event);
            match event {
                Event::End => return Ok(()),
                _ => {}
            }
        }
    }

    #[test]
    fn test_write() -> util::Result<()> {
        let mut buf = Vec::<u8>::new();
        // let mut buf = std::io::stdout();
        let mut w = Writer::new(buf);
        {
            let mut n = w.node("abc");
        }

        w.flush()?;
        let buf = w.get_mut();
        println!("{}", std::str::from_utf8(buf)?);
        Ok(())
    }
}
