import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SampleProgram } from "../target/types/sample_program";

describe("sample-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SampleProgram as Program<SampleProgram>;

  it("registers a diary client", async () => {
    const agent = new anchor.web3.PublicKey(
      "J3ajKzeZq2MjMZkfw33dxktYzzuB3EeViyboBRnBxQSv"
    );

    let [bookKeeper, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bookkeeper")],
      agent
    );

    let [diary, diaryBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("diary"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    console.log("bookkeeper address is ", bookKeeper.toBase58());
    const request = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("request"), program.provider.publicKey.toBuffer()],
      agent
    )[0];
    console.log("request address is", request.toBase58());
    const tx = await program.methods
      .initDiary()
      .accounts({
        owner: program.provider.publicKey,
        diary: diary,
        bookkeeper: bookKeeper,
        request: request,
        agent: agent,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });
    console.log("Your transaction signature", tx);
  });

  it("adds a entry to the diary", async () => {
    const agent = new anchor.web3.PublicKey(
      "J3ajKzeZq2MjMZkfw33dxktYzzuB3EeViyboBRnBxQSv"
    );

    let [bookKeeper, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("bookkeeper")],
      agent
    );

    let [diary, diaryBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("diary"), program.provider.publicKey.toBuffer()],
      program.programId
    );

    console.log("bookKeeper", bookKeeper.toBase58());
    console.log("diary address is", diary.toBase58());

    const request = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("request"), program.provider.publicKey.toBuffer()],
      agent
    )[0];

    console.log("request address is", request.toBase58());
    const tx = await program.methods
      .addEntry("heloo")
      .accounts({
        owner: program.provider.publicKey,
        bookkeeper: bookKeeper,
        agent: agent,
        request: request,
        diary: diary,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .rpc()
      .catch((e) => {
        console.log(e);
      });
    console.log("Your transaction signature", tx);
  });
});
