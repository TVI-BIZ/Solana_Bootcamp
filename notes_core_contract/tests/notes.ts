import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Notes } from "../target/types/notes";
import { assert } from "chai";

describe("notes", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Notes as Program<Notes>;
  let note = anchor.web3.Keypair.generate();

  it("can create a note!", async () => {
    await program.rpc.createNote("Hello, Encode Club Solana Bootcamp!", {
      accounts: {
        note: note.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [note],
    });
    let newNote = await program.account.note.fetch(note.publicKey);
    assert.strictEqual(newNote.content, "Hello, Encode Club Solana Bootcamp!");
    assert.strictEqual(
      newNote.user.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
  });

  it("can delete a note!", async () => {
    await program.rpc.deleteNote({
      accounts: {
        note: note.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    let delNote = await program.account.note.fetchNullable(note.publicKey);
    assert.strictEqual(delNote, null);
  });
});
