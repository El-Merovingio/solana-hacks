# DISCLAMER
**It is important to note that this code is for demonstration purposes only and should not be used in a production environment.**

# Description
In this example, the program defines two enums: SafeEnum and VulnerableEnum. The SerializeSafe struct serializes a value of SafeEnum and stores it in a Solana account. The DeserializeVulnerable struct deserializes a value of VulnerableEnum from a different Solana account. This struct currently handles the One case correctly, but does not handle the Two and Three cases correctly, potentially allowing an attacker to exploit the vulnerability.
This vulnerability arises because the program never deserializes SafeEnum, so a user could create a value of VulnerableEnum that deserializes the same as SafeEnum and exploit the program's deserialization logic. To fix this vulnerability, the program should add additional checks to ensure that the deserialized value is of the expected type.

This PoC program demonstrates how an attacker could exploit the vulnerability in the DeserializeVulnerable struct within the vulnerable program.
The program first sets up a connection to a Solana cluster, creates a new keypair for the vulnerable program, and creates a program account with the minimum balance required for a program account. It then compiles and loads the program onto the cluster.
Next, the program creates two new accounts, account_a and account_b, and funds them with SOL from a sender account. It then retrieves the account info for both accounts.
Finally, the program calls the vulnerable program with account_a as the safe account and account_b as the vulnerable account. This triggers the deserialization of the data in account_b, which contains an instance of VulnerableEnum. If the data in account_b is set to VulnerableEnum::Two or VulnerableEnum::Three, the program will return an error and terminate. However, if the data is set to VulnerableEnum::One, the program will execute successfully.
In this way, an attacker could create an account with data that contains a vulnerable enum variant, and then call the vulnerable program with that account as the vulnerable account. This would allow the attacker to execute arbitrary code within the context of the vulnerable program and potentially gain control of the program and its associated accounts.

The program defines another enum and serializes, but never deserializes it, a user could create this enum, and, if it deserializes the same as the first enum, then this may be a possible vulnerability.

# More info
- https://www.soldev.app/course/type-cosplay
- https://github.com/coral-xyz/sealevel-attacks/tree/master/programs/3-type-cosplay