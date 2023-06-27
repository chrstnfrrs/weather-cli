## City Weather script

This is a Rust script that logs what the weather feels like in a city.

### Prerequisites

- Rust. ([installation guide](https://www.rust-lang.org/tools/install)).
- api.weatherapi.com API token.

### Usage

To use this program, follow these steps:

1. Create a `.env` file in the project's root directory.
2. Inside the `.env` file, add the following line:

```
API_TOKEN=<your-api-token>
```

Replace `<your-api-token>` with the API token you generated from api.weatherapi.com.

3. Run the program with the following command:

```
cargo run -- -c "san francisco"
```

Replace `San Francisco` with the name of the city for which you want to fetch data.

4. The program will log a response from the API or error message.

### Example

Here's an example command to run the program:

```
cargo run -- -c "San Francisco"
```

This command will fetch data about the city of San Francisco and log the data in a pretty format.

### Having issues?

Here's how to run the help command:

```
cargo run -- --help
```
