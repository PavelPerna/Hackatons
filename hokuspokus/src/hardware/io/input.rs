pub mod audio;
pub mod video;
pub mod text;

trait IOData<T> {
    data: T,
    size: usize
}

trait Input {
    pub fn read() -> InputData<t>;
}
trait Output{
    pub fn write(IOData<t> &data) -> usize;
}