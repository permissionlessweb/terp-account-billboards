# 1. Mint An Account

## Initial Fees

The cost to mint is dependent on two requirements:

| Requirement | Character Length | Minimum Delegation
| --- | --- | --- |
| 5+ chars (Base Price) | 710 THIOL | 1,000 THIOL  |
| 4 chars   | 7,100 THIOL (10x) | 3,000 THIOL (3x) | |
| 3 chars  | 71,000 THIOL (100x) | 5,000 THIOL (5x) |

## Minting An Account

To mint an account, a user must first grant the marketplace approval for token transfers on their behalf for the account contract.

```json
// calling the account contract
{"approve_all":{"operator":"<accuount-marketplace-addr>"}}
```

> This will allow the minter contract perform its expected escrow functions upon the scenario of an account owner accepting an ask.

Then a user can mint an accuont token (token-id will be account name)

```json
// calling the account minter contract 
{"mint_and_list": {"account": "<eret-skeret>"}}
```

Before being minted the account name is validated to be a length within the range set by admins,and payments required are validated to have been sent. If so all mint proceeds are burnt. Only native THIOL is accepted currently.

Along with the mint message, the minter forms a default `SetAsk` msg to the Account Marketplace, letting the marketplace know that a new token has been minted. *NOTE: Learn more about the logic designed to on-chain accounts. This should be improved.*
