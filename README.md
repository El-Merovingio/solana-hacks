# Solana Vulnerabilities and Hacks

This repository does not include vulnerabilities such as:
- Signer authorization
- Owner checks
- Account Confusions
- Arbitrary CPIs
- Math Over/Underflows

Those topics were widely covered including [this repo](https://github.com/NonFungibleHacker/Solana-hacking-stuff)

This repository includes code that contains known vulnerabilities in the Solana blockchain.
This repository is inspired in [@armaniferrante](https://twitter.com/armaniferrante/status/1438706352805797889).

Some vulnerabilities included here:

- Account data matching
- Bump seed canonicalization
- Casting truncation	
- Close account
- Type Cosplay
- Duplicated mutable accounts
- Error not handled
- Reinitialization attacks
- Other bugs (Inconsistent rounding, Incorrect calculation, Exponential complexity)

The purpose of this repository is to help developers understand and mitigate risks on the Solana blockchain. By analyzing and fixing these vulnerabilities, developers can help improve the security and stability of the Solana ecosystem.
**Note:** Each piece of code may be prone to other vulnerabilities.

## How to Use This Repository

If you are a developer working with the Solana blockchain, you can use this repository to learn about common vulnerabilities and how to mitigate them. You can also use the code in this repository as a starting point for your own projects, making sure to address any known vulnerabilities and security risks.

Please note that the code in this repository comes with an addendum to the MIT License, which includes a disclaimer of warranty and limitation of liability for any potential risks and vulnerabilities that may exist in the software. By using this code, you acknowledge and agree to the terms of the addendum.

## Contributing

If you have identified a new vulnerability in the Solana blockchain, or if you have found a way to improve the existing code in this repository, we welcome your contributions! Please open a pull request with your changes and we will review them as soon as possible.

We also welcome suggestions for new vulnerabilities to add to this repository. If you have identified a new vulnerability that you think should be included, please open an issue with the details and we will consider adding it to the repository.

## Resources
- https://github.com/coral-xyz/sealevel-attacks/
- https://www.soldev.app/course#:~:text=Lesson%204-,MODULE,-7
- https://www.sec3.dev/blog/announcing-x-ray-premium-auto-auditor-for-solana-smart-contracts

## Disclaimer

Please note that the code in this repository is provided as-is and without warranty of any kind. The contributors and maintainers of this repository are not responsible for any damages or losses that may arise from the use of this code. Use at your own risk.

# LICENSE
Please read [LICENSE](LICENSE)
