use sc_consensus_pow::{PowAlgorithm};
//The concept of the block in substrate is generic 
//It's a trait and you can implement a concrete type for the trait
us sp_runtime::traits::Block as BlockT;

struct DolfhinPow;

//It expects BlockT and the block eseensitally must have a extrinsiv type
//it must have a header type some hash type a method to get the header
//a method to get the extrinsic
impl<B:Block>PowAlgorithm<B> for DolfhinPow{

}