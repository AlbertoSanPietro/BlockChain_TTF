# Architettura

## Node:
    Interfaces: HTTP per i client,
                WebSocket per comms con altri nodi.

## Structure:
    Blockchain pubblica

### Validation:
    Proof of authority

## Objectives:
    Ricevere transazioni dai clienti.

## Stack and technologies:
    ### Backend:
        Java Spring
    ### Internal communications:
        Java (Executor Service, WebSockets, Async exc...)
    ### Internal Services: 
        Block Services, Transaction Service, MemPool Service
    ### Repositories:
        Block Repository (CRUD), MemPool (CRUD)
    ### Entities:
        Blocks, Transactions, Wallets and Addresses

# Models

## Node:
    ### Validation of data and models:
        Classe contente i dati e i modelli che ci servono. 
## Addresses:
    ### Private key Hash (crypto Blake2S/Blake2Sa)
    ### String type 

## Wallet:
    ### Array of Addresses (Indirizzo->private && public key, addresses)
    ### Da private key generate public key and address e inverso

## Transaction:
    ### TransactionData (Quantity, Fee, Sender, Receiver, Date and Time) 
    ### Public key
    ### Sender's signature
    ### Transaction ID
## Block:
    ### Array of Transactions
    ### Height (number of block)
    ### HashMap referencing previous block Number (
        Map.number = number->previous;
    ) Block one may have NULL ptr
    ### FNV1-A Hash(?)










