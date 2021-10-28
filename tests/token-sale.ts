import * as anchor from '@project-serum/anchor';
import * as spl from '@solana/spl-token';
import { Program } from '@project-serum/anchor';
import { TokenSale } from '../target/types/token_sale';

describe('token-sale', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.TokenSale as Program<TokenSale>
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

    // return;

    // console.log((await program.provider.connection.getAccountInfo(mint)).owner.toString());

    let usersAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint,
      program.provider.wallet.publicKey,
    );

    //if (await program.provider.connection.getAccountInfo(usersAssociatedTokenAccount)) {

 //   }
    console.log('sending mint tx without loop');
    let tx0 = await program.state.rpc.mintSomeTokens(mintBump, mintAuthorityBump, {
      accounts: {
        mint: mint,
        wallet: program.provider.wallet.publicKey,
        destination: usersAssociatedTokenAccount,
        mintAuthority: mintAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        beneficiary: program.provider.wallet.publicKey,
      },
    })

    while ((await program.provider.connection.getSignatureStatus(tx0)).value.confirmations !== 2) {
      console.log('sign status', 'waiting');
    }


    let sampleKeypair = anchor.web3.Keypair.generate();
    var airdropSignature = await program.provider.connection.requestAirdrop(
      sampleKeypair.publicKey,
      anchor.web3.LAMPORTS_PER_SOL * 25,
    );

    // Confirming that the airdrop went through
    await program.provider.connection.confirmTransaction(airdropSignature);
    console.log("Airdropped");

    console.log(await program.provider.connection.getBalance(sampleKeypair.publicKey));

    let sampleAssociatedTokenAccount = await spl.Token.getAssociatedTokenAddress(
      spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      spl.TOKEN_PROGRAM_ID,
      mint,
      program.provider.wallet.publicKey,
    );

    let tx1 = await program.state.rpc.mintSomeTokens(mintBump, mintAuthorityBump, {
      accounts: {
        mint: mint,
        wallet: sampleKeypair.publicKey,
        destination: sampleAssociatedTokenAccount,
        mintAuthority: mintAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        beneficiary: program.provider.wallet.publicKey,
      },
      signers:[sampleKeypair]
    })

    while ((await program.provider.connection.getSignatureStatus(tx1)).value.confirmations !== 2) {
      console.log('sign status', 'waiting');
    }



    // console.log((await program.provider.connection.getAccountInfo(mint)));
    // console.log((await program.provider.connection.getAccountInfo(usersAssociatedTokenAccount)));

    // console.log((await program.provider.connection.getAccountInfo(mintAuthority)));

    // for (let i= 0 ; i<12; i++){
    //   console.log('sending tx inside loop');
    //   let tx1 = await program.state.rpc.mintSomeTokens(mintBump, mintAuthorityBump, {
    //     accounts: {
    //       mint: mint,
    //       wallet: program.provider.wallet.publicKey,
    //       destination: usersAssociatedTokenAccount,
    //       mintAuthority: mintAuthority,
    //       systemProgram: anchor.web3.SystemProgram.programId,
    //       tokenProgram: spl.TOKEN_PROGRAM_ID,
    //       associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    //       rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    //       beneficiary: program.provider.wallet.publicKey,
    //     }
    //   })

    //   // const dank = await program.provider.connection.onSignature(tx1, (result, context) => {
    //   //   console.log('result',result, context);
    //   // })

    //   // console.log('dank', dank);

    //   // console.log('current transaction',tx1);

    //   while ((await program.provider.connection.getSignatureStatus(tx1)).value.confirmations === 0) {
    //     // console.log('sign status', await program.provider.connection.getSignatureStatus(tx1));
    //   }

    //   let con = await program.state.fetch();
    //   console.log(con.supply.toString())

    //   debugger;

    //   // console.log((await program.provider.connection.getAccountInfo(mintAuthority)));
    // }
  });
});
