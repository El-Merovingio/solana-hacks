# DISCLAMER
**It is important to note that this code is for demonstration purposes only and should not be used in a production environment.**

# Description
This program is prone to two types of attacks: close account attack and account revival attack.

The process_instruction function takes an array of accounts as input, extracts the account_to_close and destination accounts from the array, and transfers the lamports from account_to_close to destination. The function then sets the lamports of account_to_close to zero, effectively closing the account.

One issue with this program is that it does not check whether the account being closed has a non-zero balance of lamports. This creates a vulnerability called the close account attack, where an attacker can create a new account with a zero balance, transfer some lamports to the account, and then close the account with this program. This will result in the lamports being transferred to the destination account, but the closed account will not be garbage collected, since it still has a non-zero balance. This can be used to exhaust the system's available memory and cause the program to crash.

Another issue with this program is that it does not check whether the account being closed has any outstanding liabilities, such as a rent exemption. This creates a vulnerability called the account revival attack, where an attacker can close an account and then immediately recreate it with the same address. Since the account still has outstanding liabilities, such as rent exemption, it will not be garbage collected. This can be used to cause unintended behavior in the program and even drain a protocol.

To prevent these attacks, the program should check the balance of the account being closed and ensure that it has no outstanding liabilities before closing it. This can be done using the Solana SDK's get_minimum_balance_for_rent_exemption function and checking the balance of the account using the lamports method on the AccountInfo struct. Additionally, the program should consider adding a delay between closing an account and allowing it to be recreated to prevent account revival attacks.

# POC Exploit
The poc program exploits the vulnerabilities in the code by creating a new account, transferring some lamports to it, and then closing the account with the vulnerable program. This allows the attacker to perform a close account attack and an account revival attack.

The code first initializes the Solana client and creates three keypairs: programa_keypair, authority, and close_account. It then requests an airdrop of SOL to the authority keypair and retrieves the minimum rent exemption balance required for a new account.

The code then creates a new account using the vulnerable program by constructing a transaction that creates a new account, initializes it, and transfers some SOL to it. The close_account keypair is used as the destination account for the transfer and will be closed later.

The program then sends the transaction to the Solana network and confirms that it was successful. It prints the lamport balances of the authority and close_account keypairs to verify that the transfer was successful.

To exploit the vulnerabilities in the previous code, an attacker can modify the data_len variable to a value less than the minimum rent exemption balance. This will cause the account to be created with a zero balance, which can be transferred to the close_account keypair. The attacker can then close the account using the vulnerable program, causing the rent exemption lamports to be transferred to the authority keypair.

The attacker can then repeat this process by creating a new account with the same address as the closed account using the same minimum rent exemption balance. This will cause the new account to have outstanding liabilities, such as rent exemption, and prevent it from being garbage collected. This can be used to cause unintended behavior in the program and even drain a protocol.

It is important to note that this code is for demonstration purposes only and should not be used in a production environment. It is important to carefully design your program's security model to prevent such attacks.

# More information
The garbage collection doesn't occur until the transaction completes. And since there can be multiple instructions in a transaction, this creates an opportunity for an attacker to invoke the instruction to close the account but also include in the transaction a transfer to refund the account's rent exemption lamports. The result is that the account will not be garbage collected, opening up a path for the attacker to cause unintended behavior in the program and even drain a protocol.

## Sources
- https://www.soldev.app/course/closing-accounts
- https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/9-closing-accounts