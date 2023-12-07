# Useful commands

`https://youtu.be/lzKeecy4OmQ?t=4100`

## Cargo

- init

  ```bash
    $ cargo init
  ```

- run

  - Default:

    ```bash
      $ cargo run
    ```

  - Omit compile debug:

    ```bash
      $ cargo run -q
    ```

  - Running a binary:

    ```bash
      $ cargo run --bin name-of-the-binary # You can combine with -q as well
    ```

    _You don't need to specify the file extension .rs_
