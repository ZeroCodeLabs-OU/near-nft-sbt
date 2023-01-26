const express = require('express');
const nearAPI = require('near-api-js');
const bodyParser = require('body-parser');

const app = express();
app.use(bodyParser.json());

let allowlistedAddresses = new Set(); // set to store allowlisted addresses

const ownerAccount = new nearAPI.Account("OWNER_ACCOUNT_NAME", nearAPI.NetworkId.TESTNET);
const contractAccount = new nearAPI.Account("CONTRACT_ACCOUNT_NAME", nearAPI.NetworkId.TESTNET);

app.post('/signature', async (req, res) => {
    const {walletAddress} = req.body;
  
    // Check if address is allowlisted
    if (!allowlistedAddresses.has(walletAddress)) {
        return res.status(400).send({error: 'Address not allowlisted'});
    }
  
    // Sign message with private key
    let messageHash = nearAPI.utils.hash(walletAddress);
    let signature = await ownerAccount.sign(messageHash);
    const signedData = {
      walletAddress: walletAddress,
      signature: signature
    };
  
    res.send(signedData);
  
    // check signature
    const checkSignature = async (address, signature) => {
      // Compute the message digest
      const messageHash = nearAPI.utils.hash(address);
      // Call the recoverSigner function on the smart contract
      const signerAddress = await contractAccount.viewFunction("recoverSigner", {address: address, signature: signature});
      // Check if the signerAddress is the same as the address passed as an argument
      console.log(signerAddress)
      if(signerAddress === owner) {
          console.log("Signature is valid");
      } else {
          console.log("Signature is invalid");
      }
    }
    checkSignature(signedData.walletAddress,signedData.signature);
});

app.listen(3000, () => {
    console.log('Server started on port 3000');
});
