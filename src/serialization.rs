pub trait Serializable {

    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self where Self: Sized;

}