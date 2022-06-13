use ferris_says::say;
use  std::io::{stdout,BufWriter};

fn main(){
    let stdout =stdout();
    let messages =String::from("hello, fellow rustaceans!");
    let width =messages.chars().count();

    let mut writer =BufWriter::new(stdout.lock());
    say(messages.as_bytes(),width,&mut writer).unwrap();
}