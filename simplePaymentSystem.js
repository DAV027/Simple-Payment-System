const { Keypair } = require('stellar-sdk');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

const balances = {};
const transactionHistory = [];

function createWallet() {
    const keypair = Keypair.random();
    const publicKey = keypair.publicKey();
    const secretKey = keypair.secret();
    balances[publicKey] = 0;
    console.log('New wallet created successfully!');
    console.log('Public Key:', publicKey);
    console.log('Secret Key:', secretKey);
}

function checkBalance(publicKey) {
    const balance = balances[publicKey] || 0;
    console.log('Balance:', balance);
}

function addBalance(publicKey, amount) {
    balances[publicKey] = (balances[publicKey] || 0) + parseFloat(amount);
    console.log('Balance added successfully!');
}

function sendPayment(senderSecretKey, receiverPublicKey, amount) {
    try {
        const senderPublicKey = Keypair.fromSecret(senderSecretKey).publicKey();
        if (balances[senderPublicKey] >= amount) {
            balances[senderPublicKey] -= amount;
            balances[receiverPublicKey] = (balances[receiverPublicKey] || 0) + parseFloat(amount);
            const transaction = {
                sender: senderPublicKey,
                receiver: receiverPublicKey,
                amount: amount,
                timestamp: new Date().toISOString()
            };
            transactionHistory.push(transaction);
            console.log('Payment sent successfully!');
        } else {
            console.log('Insufficient balance to send payment.');
        }
    } catch (error) {
        console.error('Error sending payment:', error);
    }
}

function displayTransactionHistory() {
    console.log('\nTransaction History:');
    transactionHistory.forEach((transaction, index) => {
        console.log(`Transaction ${index + 1}:`);
        console.log('Sender:', transaction.sender);
        console.log('Receiver:', transaction.receiver);
        console.log('Amount:', transaction.amount);
        console.log('Timestamp:', transaction.timestamp);
        console.log('--------------------------');
    });
}

function mainMenu() {
    console.log('\nSimple Payment System');
    console.log('1. Create Wallet');
    console.log('2. Check Balance');
    console.log('3. Add Balance');
    console.log('4. Send Payment');
    console.log('5. Transaction History');
    console.log('6. Exit');
    rl.question('Select an option: ', async (option) => {
        switch (option) {
            case '1':
                createWallet();
                mainMenu();
                break;
            case '2':
                rl.question('Enter your public key: ', async (publicKey) => {
                    checkBalance(publicKey);
                    mainMenu();
                });
                break;
            case '3':
                rl.question('Enter your public key: ', (publicKey) => {
                    rl.question('Enter amount to add: ', (amount) => {
                        addBalance(publicKey, amount);
                        mainMenu();
                    });
                });
                break;
            case '4':
                rl.question('Enter your secret key: ', (senderSecretKey) => {
                    rl.question('Enter recipient public key: ', (receiverPublicKey) => {
                        rl.question('Enter amount to send: ', async (amount) => {
                            sendPayment(senderSecretKey, receiverPublicKey, amount);
                            mainMenu();
                        });
                    });
                });
                break;
            case '5':
                displayTransactionHistory();
                mainMenu();
                break;
            case '6':
                console.log('Exiting...');
                rl.close();
                break;
            default:
                console.log('Invalid option. Please try again.');
                mainMenu();
                break;
        }
    });
}

mainMenu();

