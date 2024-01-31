
trait Tex2D<T> {

    fn read(&self, u: usize, v: usize) -> T;
    fn write(&mut self, u: usize, v: usize, val: T);
    
    fn sample(&self, u: f32, v: f32) -> T;
}


struct Tex2D<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}


