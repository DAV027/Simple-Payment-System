# Simple Payment System

This is a simple command-line based payment system implemented in JavaScript. It allows users to create wallets, check balances, add balance, send payments, and view transaction history.

Features:
- Create Wallet: Generate a new Stellar wallet with a public and secret key.
- Check Balance: Check the balance of a wallet by providing the public key.
- Add Balance: Add balance to a wallet by providing the public key and amount.
- Send Payment: Send a payment from one wallet to another by providing the sender's secret key, recipient's public key, and amount.
- Transaction History: View the transaction history, including sender, receiver, amount, and timestamp.

Usage:
1. Clone the repository.
2. Install Node.js if not already installed.
3. Open a terminal or command prompt and navigate to the project directory.
4. Run the command: `npm install stellar-sdk` to install dependencies.
5. Run the command: `node simplePaymentSystem.js` to start the program.
6. Follow the on-screen instructions to use the system.
