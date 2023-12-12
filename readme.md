# Lotto Lunes

Lotto Lunes is a decentralized lottery built on the Lunes blockchain using smart contracts and NFTs.

## About the Project

The goal of Lotto Lunes is to offer a transparent, reliable and immutable lottery taking advantage of blockchain technology benefits.

Participants can buy NFT tickets that give the right to compete in drawings held every 6 days. Tickets are non-fungible tokens (NFTs) programmatically generated with unique values and numbers.

The draw is performed by a smart contract that selects 6 random numbers and automatically identifies the winning tickets according to a prize category system. Prizes are distributed automatically to the winners' wallets by the smart contract.

The front-end was built in React and connects to the Lunes blockchain for ticket purchase and results lookup.

## Technologies Used

- Lunes Blockchain
- Smart Contracts in Ink!
- React for Front-end
- OpenBrush for NFTs

## Running the Project Locally

Instructions for project setup and execution:

1. Clone this repository
2. Install dependencies...
3. Configure your Lunes wallet...


## Requisitos

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Lunes Node](https://github.com/lunes-platform/lunes-nightly) (ou um nó Substrate compatível)

## Compilação

Para compilar o contrato, execute o seguinte comando na raiz do projeto:

```bash
cargo contract build --release
```

# Substitua <nome_do_nó> pelo nome do seu nó Substrate
```bash
cargo test -- --nocapture --test-threads=1 --skip 
```
## License

This project is licensed under the MIT License. 

This README covers the most important points about the project and serves as a quick reference for understanding and running the application.


