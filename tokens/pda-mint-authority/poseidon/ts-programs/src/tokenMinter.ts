import { Pubkey, Result, Signer } from "@solanaturbine/poseidon";

export default class TokenMinter {
	static PROGRAM_ID = new Pubkey("11111111111111111111111111111111");
	
	createToken(
		payer: Signer,
		tokenName: String,
		tokenSymbol: String,
		tokenUri: String,
	): Result {}

	mintToken(): Result {}
}
