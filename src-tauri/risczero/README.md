# zk Identity - KYC Proofs

For this solution, we use veriff.com since they sign their payload to ensure its integrity and authenticity can be checked. 
For this, they provide an HMAC for the payload provided:

"HMAC stands for Hash-Based Message Authentication Code. It's a specific message authentication code (MAC) involving a cryptographic hash function and a secret cryptographic key. It's used to verify both the data integrity and the authenticity of a message simultaneously."

To create the proof, the HMAC is recalculated in the circuit using our private key (on .env) and compared against the one provided as input. 
If both are equal, the proof is created, shielding everything except the first name (fristName).

