# A demo of a axum app on shuttle.rs

Trying [axum](https://tokio.rs/blog/2021-07-announcing-axum) and [shuttle](https://shuttle.rs). Spoiler: they are both epic.

[View on GitHub](https://github.com/kaleidawave/axum-shuttle-demo)

This site has three endpoints:

### /define/:word

Gives definitions for given words using Merriam Webster


### /random-avatar/:username

A randomly generated image for avatars etc. Each username has a custom random image

![](https://le-bucket.netlify.app/Avatars.png)

### /matrix-determinant

Calculate the determinant of a matrix using a CAS

![](https://le-bucket.netlify.app/MatrixPostRequestScreenshot.png)

## Usage

Run locally with `cargo run`. To deploy change the project id in `Shuttle.toml` then run `cargo shuttle deploy`.

### dictionaryapi.com api key

Firstly this projects still compiles without the api key just any `/define/:word` endpoints will return a `NoApiKey` error. 

However you can add a api key from [Merriam Webster's api](https://dictionaryapi.com/) to enable the endpoint. 

- To use locally (`cargo run`) the program pulls from an environment variable named `MERRIAM_WEBSTER_API_KEY`
- When deploying on shuttle the program pulls from a postgres database on shuttle. It is currently a little bit complicated but you need to: 
    - Deploy initially to create the database on shuttle
    - run `psql *db url it gives you*` (the db url includes a password per deployment)
    - in `psql` run:
        ```sql
        CREATE TABLE Secrets ( 
            name VARCHAR PRIMARY KEY, 
            key VARCHAR 
        );
        ```
        ```sql
        INSERT INTO Secrets (name, key) 
        VALUES (
            'MERRIAM_WEBSTER_API_KEY', 
            '*your api key*'
        );
        ```