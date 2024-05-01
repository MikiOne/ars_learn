struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}

impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self { s, delimiter }
    }
}

impl<'a> Iterator for SentenceIter<'a> {
    type Item; // 想想 Item 应该是什么类型？

    fn next(&mut self) -> Option<Self::Item> {
        // 如何实现 next 方法让下面的测试通过？
        todo!()
    }
}



#[test]
fn it_works() {
    let mut s = "This is the 1st sentence. This is the 2nd sentence.";
    let mut iter = SentenceIter::new(&mut s, '.');
    assert_eq!(iter.next(), Some("This is the 1st sentence."));
    assert_eq!(iter.next(), Some("This is the 2nd sentence."));
    assert_eq!(iter.next(), None);
}

fn main() {
    let mut s = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s, '。').collect();
    println!("sentences: {:?}", sentences);
}
