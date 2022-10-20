// use std::ops::Add;
trait Add<RHS=Self>{
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
impl Add<u64> for u32{
    type Output = u64;
    fn add(self,other:u64)->Self::Output {
        (self as u64) + other
    }
}
trait Page{
    fn set_page(&self,p:i32){
        println!("Page Default:1");
    }
}
trait Perpage{
    fn set_perpage(&self,num:i32){
        println!("Per Page Default:10");
    }
}
struct MyPaginate{page:i32}
impl Page for MyPaginate{}
impl Perpage for MyPaginate{}
fn main() {
    let my_pagnite = MyPaginate{page:1};
    my_pagnite.set_page(2);
    my_pagnite.set_perpage(100);
}
 