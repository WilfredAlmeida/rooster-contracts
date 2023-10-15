# Rooster Contracts

This repository houses the smart contract for Rooster and it's example usage.

## Rooster Architecture
--DIAGRAM HERE--

Any smart contract willing to send push notifications does a Cross Program Invocation (CPI) call the Rooster smart contract.

The Rooster contract emits an Anchor event along with the notification data which is caught by a Helius webhook setup on the Rooster program.

The [Rooster backend](https://github.com/WilfredAlmeida/rooster-backend) then delivers the notification to the end users.

### Contracts Description

1. **puppet**: The `puppet` program in the `/programs/puppet` is the main contract for Rooster. It has an Anchor event defined which is called by the other contracts.

2. **puppet-master**: The `puppet-master` program in the `programs/puppet-master` is an example contract calling the Rooster contract via a CPI call to send notification.