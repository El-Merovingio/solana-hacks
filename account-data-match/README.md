# DISCLAMER
**It is important to note that this code is for demonstration purposes only and should not be used in a production environment.**

# Description
This Solana program code allows the admin account associated with the program to be updated.
To ensure that Solana programs behave as expected, it's important to use data validation checks to verify that account data matches an expected value. Without these checks, unexpected accounts may be used in an instruction, leading to unexpected behavior or vulnerabilities. By verifying that account data is the expected format and size before using it in an instruction, Solana programs can ensure that they operate correctly and securely.

# Exploit code
The poc code creates two new accounts, initializes them with the Solana program, and then uses the Solana program to update the admin account associated with one of the accounts.
The program then attempts to exploit a vulnerability related to account data validation. It passes the same account twice as arguments to the Solana program's "Update" instruction, rather than two separate accounts. This could lead to unexpected behavior or vulnerabilities if the Solana program assumes that it is receiving two separate accounts and attempts to update them both.
To demonstrate this vulnerability, the program attempts to update the same account twice using the Solana program's "Update" instruction. It then prints out the data in the two accounts to demonstrate that they are identical.
This code is attempting to demonstrate a vulnerability related to account data validation, specifically the importance of ensuring that different accounts are passed as arguments to a Solana program's instructions. By passing the same account twice, the program is attempting to show that the Solana program may behave unexpectedly or have vulnerabilities if it assumes that it is receiving two separate accounts.

# More information
- https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/1-account-data-matching
- https://www.soldev.app/course/account-data-matching