
use super::input;

trait Display<E> {
    fn show(input: input::Msg) -> Result<(), E>;
}