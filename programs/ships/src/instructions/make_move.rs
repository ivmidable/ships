use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MakeMove {}

//cycle between player 1 and player 2 to allow
pub fn handler(ctx: Context<MakeMove>, x: u8, y: u8) -> Result<()> {
    //if player1_turn is true, only they can call this instruction
    //if player1_turn is false, only player 2 can call this instruction

    //if a player picks a correct spot on the board we flip that spot to true in hits
    //when all of a ships spots are hit then ship is destroyed

    //you can't call the same spot on the board more than once.
    //obky allow if hits is 0 , if 1 or 2 then panic.

    //once all ships have been destroyed a player wins
    Ok(())
}
