**Build on Near Protocol (no EVM, no Aurora)**

- NFT smart-contract
- Update Whitelist after smart-contract deployement (parameter: account whitelist, new mint date & new price per NFT)
- Enable NFT minting
- Revoke NFTs from holders
- Block transfers after minting
- Airdrop NFTs

[Near Smart Contract Planning](https://www.notion.so/zero-code/Near-Smart-Contract-planning-PRD-a7c3ca9ce6e34e2fa7c8b855a9d56c77)

[PRD](https://zero-code.notion.site/Product-Requirement-Document-Build-NFT-Smart-Contract-on-Near-using-Rust-4c9037eda1a54c72a1be918696bcf397)


** Currently the contract is being worked upon , I have added the whitelist function in which the owner can add whitelist and remove the whitelist and only whitelist canbe able to mint the NFT  



To run this code on the Testnet you must have Near-cli installed on your terminal 


step 1  Install Near Cli 
========================
npm install -g near-cli

step 2  export the Smart contract name 
=======================================
export NFT_CONTRACT_ID="whitelisting.testnet"

step 3 check if your account is whitelisted or not (should return true if false then your account wont be able to mint NFT)
==========================================================================================================================
near view $NFT_CONTRACT_ID is_whitelisted '{"account_id":"youraccount.testnet"}'

step 4 login to near cli account Testnet 
=========================================
near login (a window will popup  and it will show you instructions to login and then close the window)


ste 5 to mint the NFT you need to do the following command  (Note your account needs to be in the whitelist address otherwise it will give error) 
==========================================================
near call $NFT_CONTRACT_ID batch_mint '{"metadata": {"title": "My Non Fungible Team Token", "description": "The Team Most Certainly Goes :)", "media": "https://bafybeiftczwrtyr3k7a2k4vutd3amkwsmaqyhrdzlhvpt33dyjivufqusq.ipfs.dweb.link/goteam-gif.gif"}, "receiver_id": "youraccount.testnet","num_to_mint":1,"art_id":["global"],"amount":"11111"}' --accountId "youraccount.testnet"  --amount 0.1
