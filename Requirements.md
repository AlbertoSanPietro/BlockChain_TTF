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






