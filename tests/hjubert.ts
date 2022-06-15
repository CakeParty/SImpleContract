import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Hjubert } from '../target/types/hjubert';

describe('hjubert', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Hjubert as Program<Hjubert>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
