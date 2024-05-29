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

    pub fn get_mut(&mut self) -> util::Result<&mut W> {
        self.flush()?;
        Ok(self.buf_writer.get_mut())
    }

    pub fn node<'a>(&'a mut self, tag: &str) -> util::Result<Node<'a, W>> {
        self.buf_writer.write(format!("[{}]", tag).as_bytes())?;
        Ok(Node::new(self, 1))
    }
}

pub struct Node<'a, W>
where
    W: Write,
{
    writer: &'a mut Writer<W>,
    has_block: bool,
    depth: usize,
    tmp: Vec<u8>,
}

impl<'a, W> Node<'a, W>
where
    W: Write,
{
    fn new(writer: &'a mut Writer<W>, depth: usize) -> Self {
        Self {
            writer,
            has_block: false,
            depth,
            tmp: Vec::new(),
        }
    }
    pub fn attr(&mut self, key: &str, value: &str) -> util::Result<&mut Self> {
        self.writer
            .buf_writer
            .write(format!("({}:{})", key, value).as_bytes())?;
        Ok(self)
    }
    pub fn key(&mut self, key: &str) -> util::Result<&mut Self> {
        self.writer
            .buf_writer
            .write(format!("({})", key).as_bytes())?;
        Ok(self)
    }

    pub fn node<'b>(&'b mut self, tag: &str) -> util::Result<Node<'b, W>> {
        if !self.has_block {
            self.writer.buf_writer.write("{".as_bytes())?;
            self.has_block = true;
        }
        self.writer
            .buf_writer
            .write(format!("\n{}[{}]", indent(self.depth), tag).as_bytes())?;
        Ok(Node::new(self.writer, self.depth + 1))
    }

    fn escape(&mut self, v: &mut Vec<u8>) {
        let tmp = &mut self.tmp;
        tmp.clear();
        for b in v.iter() {
            if b"[](){}\\".contains(b) {
                tmp.push(b'\\');
            }
            tmp.push(*b);
        }
        std::mem::swap(v, tmp);
    }
}

impl<'a, W> Drop for Node<'a, W>
where
    W: Write,
{
    fn drop(&mut self) {
        if self.has_block {
            let _ = self
                .writer
                .buf_writer
                .write(format!("\n{}}}", indent(self.depth - 1)).as_bytes());
        }
    }
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Empty;

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
        let buf = Vec::<u8>::new();
        let mut w = Writer::new(buf);

        {
            let mut n0 = w.node("n0")?;
            n0.attr("k", "v")?.key("key")?;
            {
                let mut n00 = n0.node("n00")?;
                n00.attr("K", "V")?;
            }
            {
                let mut n01 = n0.node("n01")?;
                n01.attr("K", "V")?;
            }
        }

        let buf = w.get_mut()?;
        println!("{}", std::str::from_utf8(buf)?);
        assert_eq!(
            "[n0](k:v)(key){\n  [n00](K:V)\n  [n01](K:V)\n}",
            std::str::from_utf8(buf)?
        );

        Ok(())
    }

    struct Scn {
        inp: &'static [u8],
        exp: &'static [u8],
    }

    #[test]
    fn test_escape() -> util::Result<()> {
        let mut w = Writer::new(std::io::empty());
        let mut n = w.node("")?;
        let scns = [
            Scn {
                inp: "abc".as_bytes(),
                exp: "abc".as_bytes(),
            },
            Scn {
                inp: "[](){}\\".as_bytes(),
                exp: "\\[\\]\\(\\)\\{\\}\\\\".as_bytes(),
            },
        ];
        for scn in scns {
            let mut act = Vec::from(scn.inp);
            n.escape(&mut act);
            assert_eq!(act, scn.exp);
        }
        Ok(())
    }
}
