#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

fn main() {
    let f1_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("f1.txt", &f1_data);

    let mut buffer: Vec<u8> = vec![];
    open(&mut f1);
    let f1_len = f1.read(&mut buffer).unwrap();
    close(&mut f1);

    let text = String::from_utf8_lossy(&buffer);

    let f_name = &f1.name;
    println!("{:?}", f1);
    println!("{} is {} bytes long", f_name, f1_len);
    println!("{}", text)
}

impl File {
    fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: vec![],
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: &mut File) -> bool {
    f.state = FileState::Open;
    true
}

fn close(f: &mut File) -> bool {
    f.state = FileState::Closed;
    true
}
