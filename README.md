# restaurantReview
 This is restaurant review, the final project of Solana bootcamp.

# Restaurant Review Smart Contract

## Purpose

The Restaurant Review Smart Contract is designed to manage and store reviews for restaurants on the Solana blockchain. It allows users to add and update reviews, including details such as the title, rating, description, and location.

## Functionalities

### 1. Add Review

Users can add a new review by providing the title, rating, description, and location for a restaurant.

### 2. Update Review

Existing reviews can be updated by providing the title, new rating, new description, and new location.

## Deployment

Follow the steps below to deploy the Restaurant Review Smart Contract on the Solana blockchain.

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/installation)

### Steps

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/restaurant-review-contract.git
    cd restaurant-review-contract
    ```

2. Build the program:

    ```bash
    cargo build-bpf
    ```

3. Deploy the program:

    ```bash
    solana deploy target/deploy/restaurant_review.so
    ```

4. Initialize the state account:

    ```bash
    solana accounts -k target/deploy/restaurant_review-keypair.json
    ```

## Interacting with the Contract

After deploying the contract, you can interact with it using Solana tools or integrate it into your frontend application.

### Example Transactions

#### Add Review

```bash
solana-txgen add-review \
    --title "Best Restaurant Ever" \
    --rating 5 \
    --description "Exceptional service and delicious food." \
    --location "123 Main St"
