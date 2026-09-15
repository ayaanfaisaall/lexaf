#[cfg(test)]
mod tests;
use lexaf::lexer:: {
    Lexer
};

fn main() {
    let file = String::from("cat ~/Downloads/abc/dc.jpg | okay --l > jj --help>> hhff{k} | echo \"my name is {name}\" ");
    let file2 = String::from(r#"let n1 = -43
                              -8888.88
                              88.88.77
                              print "{n1}"
                              #
                              # i know ky n1-eq43 kaam nhi kry ga kyu ky wo aik hi token bny ga 
                              # ye drawback bash mai bhi hy 
                              #
                              if n1 -eq 43 {
                                  while 1 {
                                      print "the numbers are eq to: {n1}"
                                      break
                                  }
                              } elif n1 -le 23 {
                                  print "wow" # yhan bhi comment add kr sakte hain
                              } else {
                                  for i in 0 to 10 {
                                      awk '\{anything\}'
                                      print "the numbers are not eq"
                                  }
                              }
                              if let a = ${ 84 - (44 -43) * 34 } {
                                  print "this was easy answer is: {a}"
                              }
                              theme 4
                              runitctl --help
                              runitctl enable sshd
                              theme 3 && waybar; hyprpaper& "#);
    let mut lexer = Lexer::new(&file);
    let mut tokens = lexer.tokenize();
    println!("{:?}\n", tokens);
    lexer = Lexer::new(&file2);
    tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
