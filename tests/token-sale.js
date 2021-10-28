const anchor = require('@project-serum/anchor');
const spl = require('@solana/spl-token');

describe('token-sale', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.TokenSale;
    console.log(program.programId)

    console.log(program.provider.wallet)

    //const mint = anchor.web3.Keypair.generate();

    const [mint, mintBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("mint")], program.programId);
    const [mintAuthority, mintAuthorityBump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("mint-authority")], program.programId);

    console.log(mint.toString())
    console.log((await program.provider.connection.getAccountInfo(mintAuthority)));

    console.log('state', program.state)

    console.log(program)

    let ourAssociatedTokens = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint,
      program.provider.wallet.publicKey,
    );


    const tx = await program.state.rpc.new(mintBump, mintAuthorityBump, {
      accounts: {
        //mint: mint.publicKey,
        mint: mint,
        wallet: program.provider.wallet.publicKey,
        mintAuthority: mintAuthority,
        destination: ourAssociatedTokens,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
      }
      //instruction: []
      //signers: [mint]
    });

    return;

    console.log((await program.provider.connection.getAccountInfo(mint)).owner.toString());

    let usersAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint,
      program.provider.wallet.publicKey,
    );

    //if (await program.provider.connection.getAccountInfo(usersAssociatedTokenAccount)) {

 //   }

    await program.state.rpc.mintSomeTokens(mintBump, mintAuthorityBump, {
      accounts: {
        mint: mint,
        wallet: program.provider.wallet.publicKey,
        tokenDestination: usersAssociatedTokenAccount,
        mintAuthority: mintAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      },
      instructions: [
        spl.Token.createAssociatedTokenAccountInstruction(
          spl.ASSOCIATED_TOKEN_PROGRAM_ID,
          spl.TOKEN_PROGRAM_ID,
          mint,
          usersAssociatedTokenAccount,
          program.provider.wallet.publicKey,
          program.provider.wallet.publicKey
        )
      ]
    })

    console.log((await program.provider.connection.getAccountInfo(mint)));
    console.log((await program.provider.connection.getAccountInfo(usersAssociatedTokenAccount)));

    console.log((await program.provider.connection.getAccountInfo(mintAuthority)));

    for (let i= 0 ; i<12; i++){
      let tx1 = await program.state.rpc.mintSomeTokens(mintBump, mintAuthorityBump, {
        accounts: {
          mint: mint,
          wallet: program.provider.wallet.publicKey,
          tokenDestination: usersAssociatedTokenAccount,
          mintAuthority: mintAuthority,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
        }
      })

      const dank = await program.provider.connection.onSignature(tx1, (result, context) => {
        console.log('result',result, context);
      })

      console.log('dank', dank);

      console.log('current transaction',tx1);

      while ((await program.provider.connection.getSignatureStatus(tx1)).value.confirmations === 0) {
        console.log('sign status', await program.provider.connection.getSignatureStatus(tx1));
      }

      console.log((await program.provider.connection.getAccountInfo(mintAuthority)));
    }
  });
});
