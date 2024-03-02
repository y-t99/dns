#[derive(Copy, Debug, PartialEq, Eq)]
pub enum OpCode {

    /*
        a standard query
     */
    Query = 0u8,

    /*
        an inverse query
     */
    IQuery = 1u8,

    /*
        a server status request
     */
    Status = 2u8,
}