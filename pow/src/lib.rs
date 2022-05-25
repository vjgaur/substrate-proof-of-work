use sc_consensus_pow::{Error, PowAlgorithm};
use sp_consensus_pow:: Seal;
use sp_core::{U256, H256};
//The concept of block in susbtrate is generic, its a traits and you can implement a concrete type for the trait
use sp_runtime::traits::Block as BlockT;
use codec:: {Encode, Decode}; //to encode and decode the Seal
use sha3::Sha3_256;

/*This is where we will
implement how and what type of
work proof of algorithm does and
how does it verify blocks and how does it mine blocks*
*/

//the consensus proof for our network

#[derive(Encode, Decode)] //It means the this struct can be encoded and decoded into  and from bytes
struct DolfhinSeal {
	nounce: H256,   //hash
	difficulty: u256,
}

struct Compute<Hash> {
	pre_hash:Hash,
	nonce: H256,
	difficulty:u256,
}

struct DolfhinPow;
//It expects BlockT and the block eseensitally must have a extrinsiv type
//it must have a header type some hash type a method to get the header
//a method to get the extrinsic
impl<B: BlockT> PowAlgorithm<B> for dolfhinPow {
	type Difficulty = U256; //adjusting the difficulty of block creation

	fn difficulty(&self, parent: B::Hash) -> Result<Self: Difficulty, Error<B>> {
		unimplemented!()
	}
}
//This function verifies if the block is valid from the network
fn verify(
	&self,
	parent: &BlockId<B>,
	pre_hash: &B::Hash,
	pre_digest: Option<&[u8]>,
	seal: &Seal,
	difficulty: Self::Difficulty,
) -> Result<bool, Error<B>> {
   let Seal = match  DolfhinSeal::decode(input: &mut &seal[..]){
        Ok(seal:Vec<u8>)=> seal,
        Err(_) => return Ok(false)

    };
	

//When you mine the block you add the nonce, difficult, pre_hash of the block, you encode this into the bytes	
	let compute = Compute::<B::Hash> {
		nonce: seal.nonce,
		difficulty,
		pre_hash
	};

	let mut hasher: Sha3_256 = Sha3_256::new();
	hasher.update(data:computer.encode());

	// This work variable stores hash encoded bytes
	let work:Output<_> = U256::from(bytes: &*hasher.finalize());

	//Difficult can be adjusted by the network, if its taking lot of time to build the block we might reduce the difficulty value
	//so that when you multiply whatever work you get by this difficulty value its less likey to overflow, it means the result should fit in u256 
	//and more like for you to become block author
	let(_,overflowed) = work.overflowing_mul(other:difficulty);

	//verifying if the block is valid , a block can only be considered if the nonce/compute value is not overflowed
   if overflowed{
		return Ok(false);
	}


	Ok(true)

}
